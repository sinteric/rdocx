//! Structured template parsing and JSON evaluation.

use std::collections::{BTreeMap, HashMap, HashSet};

use quick_xml::Reader;
use quick_xml::events::Event;
use rdocx_oxml::content_control::{CT_Sdt, SdtContent};
use rdocx_oxml::namespace::matches_local_name;
use rdocx_oxml::placeholder;
use rdocx_oxml::table::{CT_Row, CT_Tbl, CT_Tc, CellContent};
use rdocx_oxml::text::{CT_P, RunContent};
use serde_json::Value;

use crate::document::Document;
use crate::error::{Error, Result};

/// The explicitly selected stable template grammar.
pub const TEMPLATE_GRAMMAR_VERSION: u32 = 1;

/// Original physical paragraph and expanded scalar occurrence supplied to a formatter.
pub struct TemplateFormatContext<'a> {
    pub location: &'a crate::ContentLocation,
    pub path: &'a str,
    pub occurrence: usize,
}

// Source kind, source part, old id, destination part, new id.
type NoteCopy = (crate::StoryKind, String, i32, String, i32);

// Part name, original bytes, rendered bytes.
type PartRewrite = (String, Vec<u8>, Vec<u8>);

// Prepared grammar state for one operation, derived by the existing physical
// story inventory. Ranges remain original source coordinates, never authority
// for a later document or for an imported item's changed byte offsets.
#[derive(Clone)]
pub(crate) struct TemplateProgram {
    pub story: crate::StoryId,
    pub part: String,
    pub ranges: Vec<std::ops::Range<usize>>,
    pub raw: Vec<u8>,
    pub scope: BTreeMap<String, String>,
    pub required_paragraph: bool,
    pub table_rows: bool,
    pub items: Vec<TemplateProgramItem>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TemplateItemKind {
    Source(crate::StoryItemKind),
    TableRow,
}

#[derive(Clone)]
pub(crate) struct TemplateProgramItem {
    pub location: crate::ContentLocation,
    pub range: std::ops::Range<usize>,
    pub kind: TemplateItemKind,
    pub field_owned: bool,
    pub marker_unsafe: bool,
    pub raw: Vec<u8>,
    pub scope: BTreeMap<String, String>,
    pub children: Vec<TemplateProgram>,
}

#[derive(Debug, Clone)]
enum PathStep {
    Key(String),
    Index(usize),
}

#[derive(Debug)]
enum Filter {
    Trim,
    Upper,
    Lower,
    Default(String),
    Join(String),
}

#[derive(Debug)]
struct Expression {
    spelling: String,
    path: Vec<PathStep>,
    filters: Vec<Filter>,
}

#[derive(Debug)]
enum ScalarValue {
    Expression(Expression),
    Literal(&'static str),
}

#[derive(Debug)]
struct ScalarToken {
    range: std::ops::Range<usize>,
    value: ScalarValue,
}

fn scalar_tokens(source: &str, versioned: bool) -> Result<Vec<ScalarToken>> {
    let mut tokens = Vec::new();
    let mut cursor = 0;
    while cursor < source.len() {
        if versioned {
            let escape = [
                ("{{{{", "{{"),
                ("{%{%", "{%"),
                ("}}}}", "}}"),
                ("%}%}", "%}"),
            ]
            .into_iter()
            .find(|(literal, _)| source[cursor..].starts_with(literal));
            if let Some((literal, value)) = escape {
                tokens.push(ScalarToken {
                    range: cursor..cursor + literal.len(),
                    value: ScalarValue::Literal(value),
                });
                cursor += literal.len();
                continue;
            }
        }
        if source[cursor..].starts_with("{{") {
            let start = cursor;
            cursor += 2;
            let inner_start = cursor;
            let mut quoted = false;
            let mut escaped = false;
            loop {
                let Some(character) = source[cursor..].chars().next() else {
                    return Err(template_error("unclosed scalar tag"));
                };
                if versioned && quoted {
                    if escaped {
                        escaped = false;
                    } else if character == '\\' {
                        escaped = true;
                    } else if character == '"' {
                        quoted = false;
                    }
                } else if versioned && character == '"' {
                    quoted = true;
                } else if source[cursor..].starts_with("}}") {
                    let inner = &source[inner_start..cursor];
                    let expression = if versioned {
                        parse_expression(inner)?
                    } else {
                        Expression {
                            spelling: inner.trim().into(),
                            path: parse_path(inner)?
                                .into_iter()
                                .map(|key| PathStep::Key(key.into()))
                                .collect(),
                            filters: Vec::new(),
                        }
                    };
                    cursor += 2;
                    tokens.push(ScalarToken {
                        range: start..cursor,
                        value: ScalarValue::Expression(expression),
                    });
                    break;
                } else if source[cursor..].starts_with("{{")
                    || source[cursor..].starts_with("{%")
                    || source[cursor..].starts_with("%}")
                {
                    return Err(template_error("nested or mismatched scalar tag"));
                }
                cursor += character.len_utf8();
            }
        } else if ["{%", "}}", "%}"]
            .iter()
            .any(|marker| source[cursor..].starts_with(marker))
        {
            return Err(template_error(
                "structural or unmatched delimiter in scalar text",
            ));
        } else {
            cursor += source[cursor..]
                .chars()
                .next()
                .expect("cursor is before end")
                .len_utf8();
        }
    }
    Ok(tokens)
}

struct ParsedProgram<'source> {
    source: &'source TemplateProgram,
    blocks: Vec<Block<usize>>,
    scalars: Vec<Vec<(usize, ScalarToken)>>,
    children: Vec<Vec<ParsedProgram<'source>>>,
}

#[derive(Clone)]
struct ProgramExecution {
    blocks: Vec<ExecutedBlock>,
}

#[derive(Clone)]
enum ExecutedBlock {
    Item {
        index: usize,
        scalars: Vec<(
            usize,
            std::ops::Range<usize>,
            String,
            Option<rdocx_oxml::properties::CT_RPr>,
        )>,
        children: Vec<ProgramExecution>,
    },
    Repeat {
        source_range: std::ops::Range<usize>,
        copies: Vec<Vec<ExecutedBlock>>,
    },
}

struct TemplateEvaluation<'data, 'formatter> {
    data: &'data Value,
    versioned: bool,
    occurrence: usize,
    // The callback lifetime stays on this field. A type alias would force 'static.
    #[allow(clippy::type_complexity)]
    formatter: Option<
        &'formatter mut dyn FnMut(
            &TemplateFormatContext<'_>,
            &mut crate::MailMergeFormattedText,
        ) -> Result<()>,
    >,
}

fn program_marker(sources: &[String], versioned: bool) -> Result<Option<Control>> {
    if !versioned {
        return marker_from_sources(sources);
    }
    let controls = sources
        .iter()
        .filter(|source| {
            source.trim_start().starts_with("{%") && !source.trim_start().starts_with("{%{%")
        })
        .collect::<Vec<_>>();
    if controls.is_empty() {
        return Ok(None);
    }
    if controls.len() != 1
        || sources
            .iter()
            .filter(|source| !source.trim().is_empty())
            .count()
            != 1
    {
        return Err(template_error(
            "a control marker must occupy one sibling paragraph or row",
        ));
    }
    let text = controls[0].trim();
    let inner = text
        .strip_prefix("{%")
        .and_then(|text| text.strip_suffix("%}"))
        .ok_or_else(|| template_error("malformed control marker"))?
        .trim();
    if inner == "endfor" {
        return Ok(Some(Control::End(EndKind::For)));
    }
    if inner == "endif" {
        return Ok(Some(Control::End(EndKind::If)));
    }
    let (name, path, is_loop) = if let Some(path) = inner.strip_prefix("if ") {
        ("", path, false)
    } else if let Some(rest) = inner.strip_prefix("for ") {
        let (name, path) = rest
            .split_once(" in ")
            .ok_or_else(|| template_error("for requires a variable and path"))?;
        validate_scope_name(name.trim())?;
        (name.trim(), path, true)
    } else {
        return Err(template_error("unsupported control marker"));
    };
    let expression = parse_expression(path)?;
    if !expression.filters.is_empty() {
        return Err(template_error(
            "structural controls require a path without scalar filters",
        ));
    }
    Ok(Some(if is_loop {
        Control::For {
            name: name.into(),
            path: vec![expression.spelling],
        }
    } else {
        Control::If {
            path: vec![expression.spelling],
        }
    }))
}

fn parse_program(
    source: &TemplateProgram,
    versioned: bool,
    inherited_field: bool,
) -> Result<ParsedProgram<'_>> {
    let mut markers = Vec::new();
    let mut scalars = Vec::new();
    let mut children = Vec::new();
    for item in &source.items {
        let mut tokens = Vec::new();
        let marker = if item.kind == TemplateItemKind::Source(crate::StoryItemKind::Paragraph) {
            let closed =
                crate::document::close_content_fragment_namespaces(&item.raw, &item.scope)?;
            let paragraph = CT_P::from_xml_fragment(&closed)?;
            let texts = placeholder::replaceable_texts(&paragraph);
            let marker = program_marker(&texts, versioned)?;
            if marker.is_some() && (item.field_owned || inherited_field) {
                return Err(template_error(
                    "structural marker overlaps a physical field owner",
                ));
            }
            if marker.is_none() {
                for (stretch, text) in texts.iter().enumerate() {
                    tokens.extend(
                        scalar_tokens(text, versioned)?
                            .into_iter()
                            .map(|token| (stretch, token)),
                    );
                }
            }
            marker
        } else if item.kind == TemplateItemKind::TableRow {
            let mut wrapped =
                format!(r#"<w:tbl xmlns:w="{}">"#, rdocx_oxml::namespace::W_NS).into_bytes();
            wrapped.extend(crate::document::close_content_fragment_namespaces(
                &item.raw,
                &item.scope,
            )?);
            wrapped.extend_from_slice(b"</w:tbl>");
            let table = CT_Tbl::from_xml_fragment_checked(&wrapped)?;
            let row = table
                .rows
                .first()
                .ok_or_else(|| template_error("prepared row source disappeared"))?;
            let mut direct = Vec::new();
            collect_row_marker_sources(row, &mut direct);
            let mut all = Vec::new();
            collect_row(row, &mut all);
            // A balanced paragraph program inside one cell remains that
            // cell's grammar. Only a whole-row dedicated marker owns rows.
            let marker = if direct.iter().filter(|text| !text.trim().is_empty()).count() == 1
                && program_marker(&direct, versioned)?.is_some()
            {
                program_marker(&all, versioned)?
            } else {
                None
            };
            // Start-tag attributes such as `w14:paraId` share `extra_xml`
            // with unmodelled children. They are not preserved siblings.
            let preserved_siblings = row
                .extra_xml
                .iter()
                .any(|(position, raw)| !CT_Row::raw_is_root_attributes(*position, raw));
            if marker.is_some()
                && (item.marker_unsafe
                    || item.field_owned
                    || inherited_field
                    || preserved_siblings
                    || !row.content_controls.is_empty())
            {
                return Err(template_error(
                    "a marker row cannot carry preserved siblings or ambiguous field ownership",
                ));
            }
            marker
        } else {
            None
        };
        markers.push(marker.clone());
        scalars.push(tokens);
        children.push(
            item.children
                .iter()
                .map(|child| {
                    if marker.is_some() && item.kind == TemplateItemKind::TableRow {
                        Ok(ParsedProgram {
                            source: child,
                            blocks: Vec::new(),
                            scalars: Vec::new(),
                            children: Vec::new(),
                        })
                    } else {
                        parse_program(child, versioned, inherited_field || item.field_owned)
                    }
                })
                .collect::<Result<Vec<_>>>()?,
        );
    }
    let indices = (0..source.items.len()).collect::<Vec<_>>();
    let blocks = parse_blocks(&indices, |index| Ok(markers[*index].clone()))?;
    Ok(ParsedProgram {
        source,
        blocks,
        scalars,
        children,
    })
}

fn deferred_expression(expression: &Expression, root: &Value, scopes: &[Scope]) -> bool {
    let Some(PathStep::Key(key)) = expression.path.first() else {
        return false;
    };
    if let Some(scope) = scopes.iter().rev().find(|scope| scope.name == *key) {
        return scope.deferred;
    }
    // An empty item cannot supply implicit properties yet. Known outer/root
    // properties must still be evaluated, including invalid intermediate kinds.
    scopes.iter().any(|scope| scope.deferred)
        && !root
            .as_object()
            .is_some_and(|value| value.contains_key(key))
        && !scopes.iter().any(|scope| {
            !scope.deferred
                && scope
                    .value
                    .as_object()
                    .is_some_and(|value| value.contains_key(key))
        })
}

fn program_path_value<'a>(
    path: &[String],
    evaluation: &'a TemplateEvaluation<'_, '_>,
    scopes: &'a [Scope],
) -> Result<ResolvedValue<'a>> {
    if !evaluation.versioned {
        return resolve_value(evaluation.data, scopes, path);
    }
    let expression = parse_expression(&path[0])?;
    if deferred_expression(&expression, evaluation.data, scopes) {
        return Ok(ResolvedValue::Deferred);
    }
    expression_value(&expression, evaluation.data, scopes)?
        .map(ResolvedValue::Value)
        .ok_or_else(|| template_error("missing structural template path"))
}

fn evaluate_program_blocks(
    program: &ParsedProgram<'_>,
    blocks: &[Block<usize>],
    scopes: &mut Vec<Scope>,
    evaluation: &mut TemplateEvaluation<'_, '_>,
    emit: bool,
) -> Result<Vec<ExecutedBlock>> {
    let mut output = Vec::new();
    for block in blocks {
        match block {
            Block::Item { value: index, .. } => {
                let mut scalars = Vec::new();
                for (stretch, token) in &program.scalars[*index] {
                    let (text, properties) = match &token.value {
                        ScalarValue::Literal(text) => ((*text).to_owned(), None),
                        ScalarValue::Expression(expression) => {
                            if deferred_expression(expression, evaluation.data, scopes) {
                                continue;
                            }
                            let text = if evaluation.versioned {
                                expression_text(expression, evaluation.data, scopes)?
                            } else {
                                let path = expression
                                    .path
                                    .iter()
                                    .map(|step| match step {
                                        PathStep::Key(key) => key.clone(),
                                        PathStep::Index(_) => {
                                            unreachable!("legacy path has only keys")
                                        }
                                    })
                                    .collect::<Vec<_>>();
                                match resolve_value(evaluation.data, scopes, &path)? {
                                    ResolvedValue::Value(value) => scalar_text(value, &path)?,
                                    ResolvedValue::Deferred => continue,
                                }
                            };
                            oxml_core::xml::reject_non_xml_characters(
                                "template replacement",
                                &text,
                            )?;
                            (text, None)
                        }
                    };
                    if emit {
                        scalars.push((*stretch, token.range.clone(), text, properties));
                    }
                }
                let children = program.children[*index]
                    .iter()
                    .map(|child| {
                        evaluate_program_blocks(child, &child.blocks, scopes, evaluation, emit)
                            .map(|blocks| ProgramExecution { blocks })
                    })
                    .collect::<Result<Vec<_>>>()?;
                if emit {
                    output.push(ExecutedBlock::Item {
                        index: *index,
                        scalars,
                        children,
                    });
                }
            }
            Block::For {
                name,
                path,
                source_range,
                body,
            } => {
                let values = match program_path_value(path, evaluation, scopes)? {
                    ResolvedValue::Value(Value::Array(values)) => values.clone(),
                    ResolvedValue::Deferred => Vec::new(),
                    _ => return Err(template_error("loop path must resolve to an array")),
                };
                if values.is_empty() {
                    scopes.push(Scope {
                        name: name.clone(),
                        value: Value::Null,
                        deferred: true,
                    });
                    evaluate_program_blocks(program, body, scopes, evaluation, false)?;
                    scopes.pop();
                }
                let mut copies = Vec::new();
                for value in values {
                    scopes.push(Scope {
                        name: name.clone(),
                        value,
                        deferred: false,
                    });
                    let blocks = evaluate_program_blocks(program, body, scopes, evaluation, emit)?;
                    scopes.pop();
                    if emit {
                        copies.push(blocks);
                    }
                }
                if emit {
                    output.push(ExecutedBlock::Repeat {
                        source_range: source_range.clone(),
                        copies,
                    });
                }
            }
            Block::If { path, body } => {
                let enabled = matches!(program_path_value(path, evaluation, scopes)?, ResolvedValue::Value(value) if is_truthy(value));
                let blocks =
                    evaluate_program_blocks(program, body, scopes, evaluation, emit && enabled)?;
                if emit && enabled {
                    output.extend(blocks);
                }
            }
        }
    }
    Ok(output)
}

fn validate_paragraph_numbering(
    paragraph: &CT_P,
    document: &Document,
    repeated: bool,
) -> Result<()> {
    if !repeated {
        return Ok(());
    }
    let Some(properties) = &paragraph.properties else {
        return Ok(());
    };
    let (num_id, level) = match (properties.num_id, properties.num_ilvl) {
        (Some(num_id), level) => (num_id, level.unwrap_or(0)),
        // A `w:ilvl` without a `w:numId` takes the instance of the paragraph
        // style chain, as Word does, and numbers nothing without one.
        (None, Some(level)) => match crate::style::resolve_paragraph_properties(
            properties.style_id.as_deref(),
            &document.styles,
        )
        .num_id
        {
            Some(num_id) if num_id != 0 => (num_id, level),
            _ => return Ok(()),
        },
        (None, None) => return Ok(()),
    };
    if !document.template_numbering_reference_exists(num_id, level) {
        return Err(template_error(&format!(
            "a repeated paragraph references missing numbering numId {num_id} level {level}"
        )));
    }
    Ok(())
}

fn validate_program_numbering(
    program: &ParsedProgram<'_>,
    blocks: &[Block<usize>],
    document: &Document,
    repeated: bool,
) -> Result<()> {
    for block in blocks {
        match block {
            Block::Item { value: index, .. } => {
                let item = &program.source.items[*index];
                if repeated
                    && item.kind == TemplateItemKind::Source(crate::StoryItemKind::Paragraph)
                {
                    let closed =
                        crate::document::close_content_fragment_namespaces(&item.raw, &item.scope)?;
                    validate_paragraph_numbering(
                        &CT_P::from_xml_fragment(&closed)?,
                        document,
                        true,
                    )?;
                }
                for child in &program.children[*index] {
                    validate_program_numbering(child, &child.blocks, document, repeated)?;
                }
            }
            Block::For { body, .. } => validate_program_numbering(program, body, document, true)?,
            Block::If { body, .. } => {
                validate_program_numbering(program, body, document, repeated)?
            }
        }
    }
    Ok(())
}

fn flattened_execution_items(blocks: &[ExecutedBlock]) -> Vec<&ExecutedBlock> {
    let mut items = Vec::new();
    for block in blocks {
        match block {
            ExecutedBlock::Item { .. } => items.push(block),
            ExecutedBlock::Repeat { copies, .. } => {
                for copy in copies {
                    items.extend(flattened_execution_items(copy));
                }
            }
        }
    }
    items
}

fn format_program_execution(
    program: &ParsedProgram<'_>,
    blocks: &mut [ExecutedBlock],
    current: &TemplateProgram,
    position: &mut usize,
    evaluation: &mut TemplateEvaluation<'_, '_>,
) -> Result<()> {
    for block in blocks {
        match block {
            ExecutedBlock::Repeat { copies, .. } => {
                for copy in copies {
                    format_program_execution(program, copy, current, position, evaluation)?;
                }
            }
            ExecutedBlock::Item {
                index,
                scalars,
                children,
            } => {
                let actual = current
                    .items
                    .get(*position)
                    .ok_or_else(|| template_error("expanded template source is missing"))?;
                *position += 1;
                if actual.kind != program.source.items[*index].kind
                    || scalars.len() != program.scalars[*index].len()
                    || children.len() != actual.children.len()
                {
                    return Err(template_error(
                        "expanded template source correspondence changed",
                    ));
                }
                for (prepared, (_, token)) in scalars.iter_mut().zip(&program.scalars[*index]) {
                    let ScalarValue::Expression(expression) = &token.value else {
                        continue;
                    };
                    let mut formatted = crate::MailMergeFormattedText {
                        text: prepared.2.clone(),
                        run_properties: prepared.3.clone(),
                    };
                    if let Some(callback) = evaluation.formatter.as_deref_mut() {
                        callback(
                            &TemplateFormatContext {
                                location: &program.source.items[*index].location,
                                path: &expression.spelling,
                                occurrence: evaluation.occurrence,
                            },
                            &mut formatted,
                        )?;
                    }
                    evaluation.occurrence += 1;
                    oxml_core::xml::reject_non_xml_characters(
                        "template replacement",
                        &formatted.text,
                    )?;
                    if formatted.run_properties.is_some() {
                        let mut paragraph = CT_P::new();
                        let mut run = rdocx_oxml::text::CT_R::new(&formatted.text);
                        run.properties = formatted.run_properties.clone();
                        paragraph.runs.push(run);
                        let mut writer = quick_xml::Writer::new(Vec::new());
                        paragraph.to_xml(&mut writer)?;
                        let closed = crate::document::close_content_fragment_namespaces(
                            &writer.into_inner(),
                            &program.source.items[*index].scope,
                        )?;
                        CT_P::from_xml_fragment_checked(&closed)?;
                    }
                    prepared.2 = formatted.text;
                    prepared.3 = formatted.run_properties;
                }
                for ((source, execution), child) in program.children[*index]
                    .iter()
                    .zip(children)
                    .zip(&actual.children)
                {
                    let mut child_position = 0;
                    format_program_execution(
                        source,
                        &mut execution.blocks,
                        child,
                        &mut child_position,
                        evaluation,
                    )?;
                    if child_position != child.items.len()
                        && !(child_position == 0 && child.required_paragraph)
                    {
                        return Err(template_error(
                            "expanded child template source has extra items",
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

fn replay_program_blocks(
    program: &ParsedProgram<'_>,
    blocks: &[ExecutedBlock],
    current: &TemplateProgram,
    offset: usize,
    candidate: &mut Document,
    mut note_copies: Option<&mut Vec<NoteCopy>>,
    pending_drawings: &mut HashSet<u32>,
) -> Result<Vec<u8>> {
    let mut output = Vec::new();
    let structural = note_copies.is_some();
    let mut position = 0;
    let blocks = if structural {
        blocks.iter().collect::<Vec<_>>()
    } else {
        flattened_execution_items(blocks)
    };
    for block in blocks {
        match block {
            ExecutedBlock::Item {
                index,
                scalars,
                children,
            } => {
                let local = if structural {
                    index.checked_sub(offset).ok_or_else(|| {
                        template_error("imported template item escaped its original range")
                    })?
                } else {
                    let index = position;
                    position += 1;
                    index
                };
                let item = current
                    .items
                    .get(local)
                    .ok_or_else(|| template_error("imported template item is missing"))?;
                if item.kind != program.source.items[*index].kind
                    || children.len() != program.children[*index].len()
                    || children.len() != item.children.len()
                {
                    return Err(template_error("import changed template source topology"));
                }
                let mut raw = item.raw.clone();
                let mut nested = Vec::new();
                for ((source, execution), actual) in program.children[*index]
                    .iter()
                    .zip(children)
                    .zip(&item.children)
                {
                    if source.source.story.kind() != actual.story.kind()
                        || source.source.ranges.len() != actual.ranges.len()
                    {
                        return Err(template_error("import changed template child ownership"));
                    }
                    for (alias, range) in actual.ranges.iter().enumerate() {
                        let relative =
                            range.start.checked_sub(item.range.start).ok_or_else(|| {
                                template_error("template child begins outside its item")
                            })?
                                ..range.end.checked_sub(item.range.start).ok_or_else(|| {
                                    template_error("template child ends outside its item")
                                })?;
                        let alias_source;
                        let original_alias;
                        let selected = if alias == 0 {
                            original_alias = None;
                            actual
                        } else {
                            let source_bytes = raw.get(relative.clone()).ok_or_else(|| {
                                template_error("paired template child source is stale")
                            })?;
                            original_alias = Some(source_bytes.to_vec());
                            alias_source = Document::template_fragment_program(
                                &current.part,
                                source_bytes,
                                &item.scope,
                                actual.story.kind(),
                            )?;
                            &alias_source
                        };
                        let mut replacement = replay_program(
                            source,
                            &execution.blocks,
                            selected,
                            0,
                            candidate,
                            note_copies.as_deref_mut(),
                            pending_drawings,
                            false,
                        )?;
                        if let Some(original_alias) = original_alias.as_deref() {
                            // Parsing the copy declares inherited namespaces on
                            // its root. The stored copy keeps the root it had.
                            replacement = restore_paired_copy_root(original_alias, &replacement)?;
                        }
                        nested.push((relative, replacement));
                    }
                }
                nested.sort_by_key(|(range, _)| range.start);
                if nested
                    .windows(2)
                    .any(|pair| pair[0].0.end > pair[1].0.start)
                {
                    return Err(template_error("template child sources overlap"));
                }
                for (range, replacement) in nested.into_iter().rev() {
                    raw.splice(range, replacement);
                }
                if !structural && !scalars.is_empty() {
                    let closed =
                        crate::document::close_content_fragment_namespaces(&raw, &item.scope)?;
                    let mut paragraph = CT_P::from_xml_fragment(&closed)?;
                    placeholder::replace_prepared_occurrences(&mut paragraph, scalars)?;
                    let mut writer = quick_xml::Writer::new(Vec::new());
                    paragraph.to_xml(&mut writer)?;
                    raw = writer.into_inner();
                }
                let gap_start = if local == 0 {
                    item.range.start
                } else {
                    current.items[local - 1].range.end
                };
                let owner_start = current.ranges[0].start;
                output.extend_from_slice(
                    current
                        .raw
                        .get(gap_start - owner_start..item.range.start - owner_start)
                        .ok_or_else(|| template_error("template source gap is stale"))?,
                );
                output.extend(raw);
            }
            ExecutedBlock::Repeat {
                source_range,
                copies,
            } => {
                if source_range.is_empty() {
                    continue;
                }
                let first = source_range
                    .start
                    .checked_sub(offset)
                    .ok_or_else(|| template_error("repeated source escaped imported range"))?;
                let end = source_range
                    .end
                    .checked_sub(offset)
                    .ok_or_else(|| template_error("repeated source escaped imported range"))?;
                let selected = current
                    .items
                    .get(first..end)
                    .ok_or_else(|| template_error("repeated source items are missing"))?;
                let mut fragment = Vec::new();
                for (position, item) in selected.iter().enumerate() {
                    if position > 0 {
                        let start = selected[position - 1].range.end - current.ranges[0].start;
                        let end = item.range.start - current.ranges[0].start;
                        fragment.extend_from_slice(&current.raw[start..end]);
                    }
                    fragment.extend(crate::document::close_content_fragment_namespaces(
                        &item.raw,
                        &item.scope,
                    )?);
                }
                for copy in copies {
                    if copy.is_empty() {
                        continue;
                    }
                    let imported = candidate.template_import_repeated_blocks(
                        current,
                        fragment.clone(),
                        note_copies.as_deref_mut().expect("structural import"),
                        pending_drawings,
                    )?;
                    if imported.items.len() != source_range.len()
                        || imported
                            .items
                            .iter()
                            .zip(selected)
                            .any(|(a, b)| a.kind != b.kind)
                    {
                        return Err(template_error(
                            "whole-range importer changed direct item correspondence",
                        ));
                    }
                    let rendered = replay_program(
                        program,
                        copy,
                        &imported,
                        source_range.start,
                        candidate,
                        note_copies.as_deref_mut(),
                        pending_drawings,
                        false,
                    )?;
                    if current.table_rows {
                        let first = imported
                            .items
                            .first()
                            .ok_or_else(|| template_error("imported row range is empty"))?;
                        let last = imported.items.last().unwrap();
                        let start = first.range.start - imported.ranges[0].start;
                        let suffix =
                            imported.raw.len() - (last.range.end - imported.ranges[0].start);
                        output.extend_from_slice(&rendered[start..rendered.len() - suffix]);
                        continue;
                    }
                    let mut wrapped =
                        format!(r#"<w:document xmlns:w="{}">"#, rdocx_oxml::namespace::W_NS)
                            .into_bytes();
                    wrapped.extend(rendered);
                    wrapped.extend_from_slice(b"</w:document>");
                    output.extend(crate::document::package_authoritative_body_fragment(
                        &wrapped,
                        false,
                        &BTreeMap::new(),
                    )?);
                }
            }
        }
    }
    Ok(output)
}

/// Drop namespace declarations added so a paired copy could be parsed alone.
/// The copy's original root start tag stays, and the rendered interior stays.
fn restore_paired_copy_root(original: &[u8], rendered: &[u8]) -> Result<Vec<u8>> {
    let original_end = fragment_root_start_tag_end(original)?;
    let rendered_end = fragment_root_start_tag_end(rendered)?;
    let mut output = Vec::with_capacity(original_end + rendered.len().saturating_sub(rendered_end));
    output.extend_from_slice(&original[..original_end]);
    output.extend_from_slice(&rendered[rendered_end..]);
    Ok(output)
}

fn fragment_root_start_tag_end(xml: &[u8]) -> Result<usize> {
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(false);
    let mut buffer = Vec::new();
    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Start(_) | Event::Empty(_)) => return Ok(reader.buffer_position() as usize),
            Ok(Event::Eof) => {
                return Err(template_error("paired template copy has no root element"));
            }
            Ok(_) => {}
            Err(error) => return Err(template_xml_error(error)),
        }
        buffer.clear();
    }
}

fn replay_program(
    program: &ParsedProgram<'_>,
    blocks: &[ExecutedBlock],
    current: &TemplateProgram,
    offset: usize,
    candidate: &mut Document,
    note_copies: Option<&mut Vec<NoteCopy>>,
    pending_drawings: &mut HashSet<u32>,
    refresh_root: bool,
) -> Result<Vec<u8>> {
    if current.items.is_empty()
        || (note_copies.is_none()
            && current.required_paragraph
            && flattened_execution_items(blocks).is_empty())
    {
        return Ok(current.raw.clone());
    }
    let output = replay_program_blocks(
        program,
        blocks,
        current,
        offset,
        candidate,
        note_copies,
        pending_drawings,
    )?;
    // Allocation canonicalizes bookmarks and relationships before publication.
    // Splice the prepared expansion into that fresh owner, preserving the
    // original callback locations already paired onto the root program.
    let base = if refresh_root {
        canonical_root_base(candidate, current, pending_drawings)?
    } else {
        current.clone()
    };
    let start = base.items.first().expect("nonempty source").range.start - base.ranges[0].start;
    let end = base.items.last().expect("nonempty source").range.end - base.ranges[0].start;
    let mut raw = base.raw.clone();
    raw.splice(start..end, output);
    if current.required_paragraph {
        candidate.template_require_cell_paragraph(&mut raw, &current.scope)?;
    }
    Ok(raw)
}

// Chart text has no authentic Word paragraph location. Prepare built-ins from
// original text operands, and refuse formatter mode before any callback.
fn prepare_chart_edits(
    document: &Document,
    data: &Value,
    versioned: bool,
    has_formatter: bool,
) -> Result<(Vec<PartRewrite>, usize)> {
    let mut edits = Vec::new();
    let mut count = 0;
    for (part, original) in document.template_chart_parts() {
        let mut reader = Reader::from_reader(original.as_slice());
        let mut replacements = Vec::new();
        loop {
            match reader.read_event() {
                Ok(Event::Start(element))
                    if matches_local_name(element.name().as_ref(), b"t")
                        || matches_local_name(element.name().as_ref(), b"v") =>
                {
                    let start = reader.buffer_position() as usize;
                    let text = oxml_core::xml_text::read_element_text(&mut reader, element.name());
                    let end = reader.buffer_position() as usize;
                    let closing = original[start..end]
                        .iter()
                        .rposition(|byte| *byte == b'<')
                        .ok_or_else(|| template_error("chart text closing source is missing"))?
                        + start;
                    let tokens = scalar_tokens(&text, versioned)?;
                    if tokens.is_empty() {
                        continue;
                    }
                    let mut rendered = text;
                    for token in tokens.into_iter().rev() {
                        let value = match token.value {
                            ScalarValue::Literal(value) => value.to_owned(),
                            ScalarValue::Expression(expression) => {
                                if has_formatter {
                                    return Err(template_error(&format!(
                                        "chart scalar in {part} has no original Word paragraph context for a formatter; use render_template_versioned"
                                    )));
                                }
                                count += 1;
                                if versioned {
                                    expression_text(&expression, data, &[])?
                                } else {
                                    let path = expression
                                        .path
                                        .iter()
                                        .map(|step| match step {
                                            PathStep::Key(key) => key.clone(),
                                            PathStep::Index(_) => unreachable!(),
                                        })
                                        .collect::<Vec<_>>();
                                    match resolve_value(data, &[], &path)? {
                                        ResolvedValue::Value(value) => scalar_text(value, &path)?,
                                        ResolvedValue::Deferred => unreachable!(),
                                    }
                                }
                            }
                        };
                        oxml_core::xml::reject_non_xml_characters(
                            "chart template replacement",
                            &value,
                        )?;
                        rendered.replace_range(token.range, &value);
                    }
                    replacements.push((
                        start..closing,
                        quick_xml::escape::escape(&rendered)
                            .into_owned()
                            .into_bytes(),
                    ));
                }
                Ok(Event::Eof) => break,
                Ok(_) => {}
                Err(error) => return Err(template_xml_error(error)),
            }
        }
        let mut rendered = original.clone();
        for (range, value) in replacements.into_iter().rev() {
            rendered.splice(range, value);
        }
        if rendered != original {
            edits.push((part, original, rendered));
        }
    }
    Ok((edits, count))
}

fn canonical_root_base(
    candidate: &mut Document,
    current: &TemplateProgram,
    pending_drawings: &HashSet<u32>,
) -> Result<TemplateProgram> {
    // Dependency allocation changes identifier occupancy. Canonicalize before
    // reading the owner that publication will check again.
    candidate.prepare_staged_package()?;
    // That prepare keeps only drawings already in the package. Repeated copies
    // still live in the unpublished replacement, so their ids stay reserved.
    candidate.retain_pending_drawing_ids(pending_drawings);
    let mut fresh = candidate
        .template_programs()?
        .into_iter()
        .find(|program| {
            program.part == current.part
                && program.story.kind() == current.story.kind()
                && program.story.owner_index() == current.story.owner_index()
        })
        .ok_or_else(|| template_error("template physical owner disappeared during staging"))?;
    retain_original_template_locations(current, &mut fresh)?;
    Ok(fresh)
}

fn retain_original_template_locations(
    original: &TemplateProgram,
    current: &mut TemplateProgram,
) -> Result<()> {
    if original.part != current.part
        || original.story.kind() != current.story.kind()
        || original.ranges.len() != current.ranges.len()
        || original.table_rows != current.table_rows
        || original.items.len() != current.items.len()
    {
        return Err(template_error(
            "source normalization changed template owner correspondence",
        ));
    }
    for (before, after) in original.items.iter().zip(&mut current.items) {
        if before.kind != after.kind || before.children.len() != after.children.len() {
            return Err(template_error(
                "source normalization changed template item correspondence",
            ));
        }
        after.location = before.location.clone();
        for (before, after) in before.children.iter().zip(&mut after.children) {
            retain_original_template_locations(before, after)?;
        }
    }
    Ok(())
}

fn prepare_unowned_textbox_edits(
    document: &Document,
    programs: &[TemplateProgram],
    data: &Value,
    versioned: bool,
    has_formatter: bool,
) -> Result<(Vec<PartRewrite>, usize)> {
    fn exclusions(
        program: &TemplateProgram,
        parts: &mut BTreeMap<String, Vec<std::ops::Range<usize>>>,
    ) {
        let physical_textbox = program.story.kind() == crate::StoryKind::TextBox
            && matches!(Reader::from_reader(program.raw.as_slice()).read_event(), Ok(Event::Start(ref start) | Event::Empty(ref start)) if matches_local_name(start.name().as_ref(),b"txbxContent"));
        if physical_textbox {
            parts
                .entry(program.part.clone())
                .or_default()
                .extend(program.ranges.iter().cloned());
        }
        for item in &program.items {
            for child in &item.children {
                exclusions(child, parts);
            }
        }
    }
    let mut excluded = BTreeMap::new();
    for program in programs {
        exclusions(program, &mut excluded);
    }
    let mut edits = Vec::new();
    let mut count = 0;
    for (part, original) in document.template_raw_parts()? {
        let ranges = excluded.entry(part.clone()).or_default();
        // Word writes one text box twice, and a box can contain another.
        // Those physical ranges overlap. The raw walker accepts only a
        // sorted list whose members do not overlap, so each overlap becomes
        // one covering range. A range that merely touches the previous end
        // stays separate.
        ranges.retain(|range| range.start < range.end);
        ranges.sort_by_key(|range| (range.start, std::cmp::Reverse(range.end)));
        let mut covered: Vec<std::ops::Range<usize>> = Vec::new();
        for range in ranges.iter().cloned() {
            if let Some(last) = covered.last_mut()
                && range.start < last.end
            {
                last.end = last.end.max(range.end);
            } else {
                covered.push(range);
            }
        }
        *ranges = covered;
        let (_, _, sources) =
            placeholder::replace_original_literals_in_xml_part(&original, ranges, &[])?;
        let mut operands = BTreeMap::<String, String>::new();
        for source in sources {
            for token in scalar_tokens(&source, versioned)? {
                let value = match token.value {
                    ScalarValue::Literal(value) => value.to_owned(),
                    ScalarValue::Expression(expression) => {
                        if has_formatter {
                            return Err(template_error(&format!(
                                "raw textbox scalar in {part} has no checked original Word paragraph context for a formatter; use render_template_versioned"
                            )));
                        }
                        count += 1;
                        if versioned {
                            expression_text(&expression, data, &[])?
                        } else {
                            let path = expression
                                .path
                                .iter()
                                .map(|step| match step {
                                    PathStep::Key(key) => key.clone(),
                                    PathStep::Index(_) => unreachable!(),
                                })
                                .collect::<Vec<_>>();
                            match resolve_value(data, &[], &path)? {
                                ResolvedValue::Value(value) => scalar_text(value, &path)?,
                                ResolvedValue::Deferred => unreachable!(),
                            }
                        }
                    }
                };
                oxml_core::xml::reject_non_xml_characters(
                    "raw textbox template replacement",
                    &value,
                )?;
                operands.insert(source[token.range].to_owned(), value);
            }
        }
        if operands.is_empty() {
            continue;
        }
        let operands = operands
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .collect::<Vec<_>>();
        let (rendered, _, _) =
            placeholder::replace_original_literals_in_xml_part(&original, ranges, &operands)?;
        if rendered != original {
            edits.push((part, original, rendered));
        }
    }
    Ok((edits, count))
}

#[allow(clippy::type_complexity)] // Same non-static formatter contract as the evaluation field.
pub(crate) fn render_grammar(
    document: &mut Document,
    data: &Value,
    versioned: bool,
    formatter: Option<
        &mut dyn FnMut(
            &TemplateFormatContext<'_>,
            &mut crate::MailMergeFormattedText,
        ) -> Result<()>,
    >,
) -> Result<usize> {
    let mut candidate = document.clone_for_staging();
    let originals = candidate.template_programs()?;
    candidate.prepare_staged_package()?;
    let mut sources = candidate.template_programs()?;
    if originals.len() != sources.len() {
        return Err(template_error(
            "source normalization changed template root correspondence",
        ));
    }
    for (original, current) in originals.iter().zip(&mut sources) {
        retain_original_template_locations(original, current)?;
    }
    let (raw_textboxes, raw_count) =
        prepare_unowned_textbox_edits(&candidate, &sources, data, versioned, formatter.is_some())?;
    for (part, original, rendered) in &raw_textboxes {
        candidate.template_publish_scalar_part(part, original, rendered.clone())?;
    }
    // Reacquire fresh edit ranges after original-only raw scalar edits. Retain
    // original callback locations through the same checked item correspondence.
    if !raw_textboxes.is_empty() {
        let mut refreshed = candidate.template_programs()?;
        if originals.len() != refreshed.len() {
            return Err(template_error(
                "raw scalar publication changed checked template roots",
            ));
        }
        for (original, current) in originals.iter().zip(&mut refreshed) {
            retain_original_template_locations(original, current)?;
        }
        sources = refreshed;
    }
    let (charts, chart_count) =
        prepare_chart_edits(&candidate, data, versioned, formatter.is_some())?;
    let programs = sources
        .iter()
        .map(|source| parse_program(source, versioned, false))
        .collect::<Result<Vec<_>>>()?;
    for program in &programs {
        validate_program_numbering(program, &program.blocks, &candidate, false)?;
    }
    let mut evaluation = TemplateEvaluation {
        data,
        versioned,
        occurrence: 0,
        formatter: None,
    };
    let mut executions = Vec::new();
    for program in &programs {
        executions.push(ProgramExecution {
            blocks: evaluate_program_blocks(
                program,
                &program.blocks,
                &mut Vec::new(),
                &mut evaluation,
                true,
            )?,
        });
    }
    // Structure is prepared without touching scalar operands. Import allocation
    // facts connect copied physical notes to already evaluated original programs.
    let key = |source: &TemplateProgram| {
        (
            source.story.kind(),
            source.part.clone(),
            source.story.owner_index(),
        )
    };
    let mut bindings = programs
        .iter()
        .enumerate()
        .zip(&executions)
        .map(|((index, program), execution)| (key(program.source), (index, execution.clone())))
        .collect::<HashMap<_, _>>();
    let mut note_origins = HashMap::new();
    for (kind, part, id, owner) in candidate.template_note_identities()? {
        let (index, _) = bindings
            .get(&(kind, part.clone(), owner))
            .ok_or_else(|| template_error("original note has no prepared template owner"))?;
        note_origins.insert((kind, part, id), *index);
    }
    let mut allocations = Vec::new();
    let mut pending_drawings = HashSet::new();
    let mut structural_edits = Vec::new();
    for (program, execution) in programs.iter().zip(&executions) {
        let rendered = replay_program(
            program,
            &execution.blocks,
            program.source,
            0,
            &mut candidate,
            Some(&mut allocations),
            &mut pending_drawings,
            true,
        )?;
        let fresh = canonical_root_base(&mut candidate, program.source, &pending_drawings)?;
        structural_edits.push((fresh, rendered));
    }
    let mut allocated = 0;
    while allocated < allocations.len() {
        let end = allocations.len();
        let current_sources = candidate
            .template_programs()?
            .into_iter()
            .map(|source| (key(&source), source))
            .collect::<HashMap<_, _>>();
        let identities = candidate
            .template_note_identities()?
            .into_iter()
            .map(|(kind, part, id, owner)| ((kind, part, id), owner))
            .collect::<HashMap<_, _>>();
        // Register the complete allocation batch first. Nested references use
        // this same actual old-to-new map, never numeric or hash-map order.
        for (kind, source_part, old_id, destination_part, new_id) in &allocations[allocated..end] {
            let origin = *note_origins
                .get(&(*kind, source_part.clone(), *old_id))
                .ok_or_else(|| template_error("copied note has no original prepared provenance"))?;
            note_origins.insert((*kind, destination_part.clone(), *new_id), origin);
        }
        // Detach this batch. Replaying a copied note appends further allocations.
        let batch = allocations[allocated..end].to_vec();
        for (kind, _, _, destination_part, new_id) in batch {
            let origin = note_origins[&(kind, destination_part.clone(), new_id)];
            let owner = identities
                .get(&(kind, destination_part.clone(), new_id))
                .ok_or_else(|| template_error("allocated note has no checked physical owner"))?;
            let current = current_sources
                .get(&(kind, destination_part.clone(), *owner))
                .ok_or_else(|| template_error("allocated note source is missing"))?;
            let execution = executions[origin].clone();
            let rendered = replay_program(
                &programs[origin],
                &execution.blocks,
                current,
                0,
                &mut candidate,
                Some(&mut allocations),
                &mut pending_drawings,
                true,
            )?;
            if bindings.insert(key(current), (origin, execution)).is_some() {
                return Err(template_error("allocated note source was prepared twice"));
            }
            let fresh = canonical_root_base(&mut candidate, current, &pending_drawings)?;
            structural_edits.push((fresh, rendered));
        }
        allocated = end;
    }
    let mut changed = !charts.is_empty() || !raw_textboxes.is_empty();
    for (source, rendered) in structural_edits {
        let source = canonical_root_base(&mut candidate, &source, &pending_drawings)?;
        changed |= rendered != source.raw;
        candidate.template_publish_program(&source, rendered)?;
    }
    evaluation.formatter = formatter;
    let expanded = candidate.template_programs()?;
    for current in expanded {
        let (origin, mut execution) = bindings
            .remove(&key(&current))
            .ok_or_else(|| template_error("expanded owner has no prepared source provenance"))?;
        let mut position = 0;
        format_program_execution(
            &programs[origin],
            &mut execution.blocks,
            &current,
            &mut position,
            &mut evaluation,
        )?;
        if position != current.items.len() && !(position == 0 && current.required_paragraph) {
            return Err(template_error("expanded owner has unbound source items"));
        }
        let rendered = replay_program(
            &programs[origin],
            &execution.blocks,
            &current,
            0,
            &mut candidate,
            None,
            &mut pending_drawings,
            false,
        )?;
        changed |= rendered != current.raw;
        candidate.template_publish_program(&current, rendered)?;
    }
    if !bindings.is_empty() {
        return Err(template_error("prepared template owner disappeared"));
    }
    let count = evaluation.occurrence + chart_count + raw_count;
    if !changed {
        return Ok(count);
    }
    for (part, original, rendered) in charts {
        candidate.template_publish_scalar_part(&part, &original, rendered)?;
    }
    let prepared = candidate.prepare_and_reopen_staged()?;
    document.commit_template(prepared);
    Ok(count)
}

fn json_string_at(source: &str, cursor: &mut usize) -> Result<String> {
    let start = *cursor;
    if source.as_bytes().get(start) != Some(&b'"') {
        return Err(template_error(
            "filter arguments and quoted keys require JSON strings",
        ));
    }
    *cursor += 1;
    let mut escaped = false;
    while let Some(&byte) = source.as_bytes().get(*cursor) {
        *cursor += 1;
        if escaped {
            escaped = false;
        } else if byte == b'\\' {
            escaped = true;
        } else if byte == b'"' {
            return serde_json::from_str(&source[start..*cursor])
                .map_err(|_| template_error("invalid JSON string in template"));
        }
    }
    Err(template_error("unclosed JSON string in template"))
}

fn skip_space(source: &str, cursor: &mut usize) {
    while let Some(character) = source[*cursor..].chars().next() {
        if !character.is_whitespace() {
            break;
        }
        *cursor += character.len_utf8();
    }
}

fn parse_expression(source: &str) -> Result<Expression> {
    let source = source.trim();
    let mut cursor = 0;
    let mut path = Vec::new();
    let mut need_key = true;
    loop {
        if source.as_bytes().get(cursor) == Some(&b'[') && !need_key {
            cursor += 1;
            skip_space(source, &mut cursor);
            let step = if source.as_bytes().get(cursor) == Some(&b'"') {
                PathStep::Key(json_string_at(source, &mut cursor)?)
            } else {
                let start = cursor;
                while source
                    .as_bytes()
                    .get(cursor)
                    .is_some_and(u8::is_ascii_digit)
                {
                    cursor += 1;
                }
                if start == cursor {
                    return Err(template_error(
                        "brackets require an array index or quoted object key",
                    ));
                }
                PathStep::Index(
                    source[start..cursor]
                        .parse()
                        .map_err(|_| template_error("array index exceeds supported range"))?,
                )
            };
            skip_space(source, &mut cursor);
            if source.as_bytes().get(cursor) != Some(&b']') {
                return Err(template_error("unclosed path bracket"));
            }
            cursor += 1;
            path.push(step);
        } else if need_key && source.as_bytes().get(cursor) == Some(&b'[') && path.is_empty() {
            // A quoted root key admits punctuation without an invented root name.
            need_key = false;
            continue;
        } else if need_key {
            let start = cursor;
            while let Some(character) = source[cursor..].chars().next() {
                if character.is_whitespace()
                    || matches!(
                        character,
                        '.' | '[' | ']' | '|' | '"' | '\'' | '{' | '}' | '%' | '(' | ')' | '\0'
                    )
                {
                    break;
                }
                cursor += character.len_utf8();
            }
            if cursor == start {
                return Err(template_error("expected a template property name"));
            }
            path.push(PathStep::Key(source[start..cursor].into()));
            need_key = false;
        } else if source.as_bytes().get(cursor) == Some(&b'.') {
            cursor += 1;
            need_key = true;
        } else {
            break;
        }
    }
    if need_key || path.is_empty() {
        return Err(template_error("incomplete template path"));
    }
    let spelling = source[..cursor].to_owned();
    let mut filters = Vec::new();
    loop {
        skip_space(source, &mut cursor);
        if cursor == source.len() {
            break;
        }
        if source.as_bytes().get(cursor) != Some(&b'|') {
            return Err(template_error("unexpected text after template path"));
        }
        cursor += 1;
        skip_space(source, &mut cursor);
        let start = cursor;
        while source
            .as_bytes()
            .get(cursor)
            .is_some_and(u8::is_ascii_alphabetic)
        {
            cursor += 1;
        }
        let name = &source[start..cursor];
        skip_space(source, &mut cursor);
        let filter = match name {
            "trim" => Filter::Trim,
            "upper" => Filter::Upper,
            "lower" => Filter::Lower,
            "default" | "join" => {
                if source.as_bytes().get(cursor) != Some(&b'(') {
                    return Err(template_error("filter requires one JSON string argument"));
                }
                cursor += 1;
                skip_space(source, &mut cursor);
                let argument = json_string_at(source, &mut cursor)?;
                skip_space(source, &mut cursor);
                if source.as_bytes().get(cursor) != Some(&b')') {
                    return Err(template_error(
                        "filter requires exactly one JSON string argument",
                    ));
                }
                cursor += 1;
                if name == "default" {
                    Filter::Default(argument)
                } else {
                    Filter::Join(argument)
                }
            }
            _ => return Err(template_error("unknown template filter")),
        };
        filters.push(filter);
    }
    Ok(Expression {
        spelling,
        path,
        filters,
    })
}

fn traverse_expression<'a>(mut value: &'a Value, path: &[PathStep]) -> Result<Option<&'a Value>> {
    for step in path {
        let next = match step {
            PathStep::Key(key) => value
                .as_object()
                .ok_or_else(|| template_error("template path crosses a non-object value"))?
                .get(key),
            PathStep::Index(index) => value
                .as_array()
                .ok_or_else(|| template_error("template path crosses a non-array value"))?
                .get(*index),
        };
        let Some(next) = next else {
            return Ok(None);
        };
        value = next;
    }
    Ok(Some(value))
}

fn expression_value<'a>(
    expression: &Expression,
    root: &'a Value,
    scopes: &'a [Scope],
) -> Result<Option<&'a Value>> {
    if let Some(PathStep::Key(first)) = expression.path.first() {
        for scope in scopes.iter().rev() {
            if &scope.name == first {
                return traverse_expression(&scope.value, &expression.path[1..]);
            }
        }
        for scope in scopes.iter().rev() {
            if scope
                .value
                .as_object()
                .is_some_and(|object| object.contains_key(first))
            {
                return traverse_expression(&scope.value, &expression.path);
            }
        }
    }
    traverse_expression(root, &expression.path)
}

fn expression_text(expression: &Expression, root: &Value, scopes: &[Scope]) -> Result<String> {
    let mut value = expression_value(expression, root, scopes)?.cloned();
    for filter in &expression.filters {
        match filter {
            Filter::Default(fallback) => {
                if value.as_ref().is_none_or(Value::is_null) {
                    value = Some(Value::String(fallback.clone()));
                }
            }
            Filter::Join(separator) => {
                let Some(Value::Array(values)) = value.as_ref() else {
                    return Err(template_error("join requires an array of scalar values"));
                };
                let text = values
                    .iter()
                    .map(|value| scalar_text(value, &[]))
                    .collect::<Result<Vec<_>>>()?
                    .join(separator);
                value = Some(Value::String(text));
            }
            Filter::Trim | Filter::Upper | Filter::Lower => {
                let text = value
                    .as_ref()
                    .ok_or_else(|| template_error("missing template path"))?;
                let text = scalar_text(text, &[])?;
                value = Some(Value::String(match filter {
                    Filter::Trim => text.trim().to_owned(),
                    Filter::Upper => text.to_uppercase(),
                    Filter::Lower => text.to_lowercase(),
                    _ => unreachable!(),
                }));
            }
        }
    }
    scalar_text(
        value
            .as_ref()
            .ok_or_else(|| template_error("missing template path"))?,
        &[],
    )
}

#[cfg(test)]
#[test]
fn versioned_template_expression_paths_and_filters_are_typed() {
    let data = serde_json::json!({"person":{"name":"  Straße  ","values":[1,false,null,"é"]},"a}}|b":"literal", "zero":0, "empty":"", "wrong":3});
    for (source, expected) in [
        ("person.name | trim | upper", "STRASSE"),
        (r#"["a}}|b"]"#, "literal"),
        ("person.values[1]", "false"),
        (r#"person.values | join("/")"#, "1/false//é"),
        (r#"absent | default("fallback")"#, "fallback"),
        (r#"zero | default("fallback")"#, "0"),
        (r#"empty | default("fallback")"#, ""),
    ] {
        assert_eq!(
            expression_text(&parse_expression(source).unwrap(), &data, &[]).unwrap(),
            expected,
            "{source}"
        );
    }
    for source in [
        r#"wrong.child | default("fallback")"#,
        r#"person[0] | default("fallback")"#,
        r#"person.values | join(1)"#,
        "person.name | trim()",
        "person.name | unknown",
        "person.values[-1]",
        "person..name",
    ] {
        assert!(
            parse_expression(source)
                .and_then(|expression| expression_text(&expression, &data, &[]))
                .is_err(),
            "{source}"
        );
    }
}

#[derive(Debug, Clone)]
struct Scope {
    name: String,
    value: Value,
    deferred: bool,
}

enum ResolvedValue<'a> {
    Value(&'a Value),
    Deferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EndKind {
    For,
    If,
}

impl EndKind {
    fn name(self) -> &'static str {
        match self {
            Self::For => "endfor",
            Self::If => "endif",
        }
    }
}

#[derive(Debug, Clone)]
enum Control {
    For { name: String, path: Vec<String> },
    If { path: Vec<String> },
    End(EndKind),
}

#[derive(Debug)]
enum Block<T> {
    Item {
        value: T,
    },
    For {
        name: String,
        path: Vec<String>,
        source_range: std::ops::Range<usize>,
        body: Vec<Block<T>>,
    },
    If {
        path: Vec<String>,
        body: Vec<Block<T>>,
    },
}

fn marker_from_sources(sources: &[String]) -> Result<Option<Control>> {
    let with_controls = sources
        .iter()
        .filter(|source| source.contains("{%") || source.contains("%}"))
        .collect::<Vec<_>>();
    if with_controls.is_empty() {
        return Ok(None);
    }
    let nonempty = sources
        .iter()
        .filter(|source| !source.trim().is_empty())
        .collect::<Vec<_>>();
    if nonempty.len() != 1 || with_controls.len() != 1 {
        return Err(template_error(
            "a control marker must occupy its own paragraph or table row",
        ));
    }
    parse_control(nonempty[0]).map(Some)
}

fn parse_control(source: &str) -> Result<Control> {
    let trimmed = source.trim();
    if !trimmed.starts_with("{%") || !trimmed.ends_with("%}") {
        return Err(template_error(
            "a control marker must occupy its own paragraph or table row",
        ));
    }
    let inner = trimmed[2..trimmed.len() - 2].trim();
    if inner.is_empty()
        || inner.contains("{{")
        || inner.contains("{%")
        || inner.contains("}}")
        || inner.contains("%}")
    {
        return Err(template_error("malformed control tag"));
    }
    let words = inner.split_whitespace().collect::<Vec<_>>();
    match words.as_slice() {
        ["for", name, "in", path] => {
            validate_scope_name(name)?;
            Ok(Control::For {
                name: (*name).to_owned(),
                path: parse_path(path)?.into_iter().map(str::to_owned).collect(),
            })
        }
        ["if", path] => Ok(Control::If {
            path: parse_path(path)?.into_iter().map(str::to_owned).collect(),
        }),
        ["endfor"] => Ok(Control::End(EndKind::For)),
        ["endif"] => Ok(Control::End(EndKind::If)),
        _ => Err(template_error("unsupported or malformed control tag")),
    }
}

fn validate_scope_name(name: &str) -> Result<()> {
    if name.is_empty()
        || name.contains('.')
        || name.chars().any(|character| {
            character.is_whitespace() || matches!(character, '{' | '}' | '%' | '\0')
        })
    {
        return Err(template_error("invalid loop variable"));
    }
    Ok(())
}

fn parse_blocks<T: Clone>(
    items: &[T],
    marker: impl Fn(&T) -> Result<Option<Control>>,
) -> Result<Vec<Block<T>>> {
    let (blocks, next) = parse_block_level(items, 0, None, &marker)?;
    if next != items.len() {
        return Err(template_error("unexpected control marker"));
    }
    Ok(blocks)
}

fn parse_block_level<T: Clone, F: Fn(&T) -> Result<Option<Control>>>(
    items: &[T],
    mut index: usize,
    expected: Option<EndKind>,
    marker: &F,
) -> Result<(Vec<Block<T>>, usize)> {
    let mut blocks = Vec::new();
    while index < items.len() {
        match marker(&items[index])? {
            None => {
                blocks.push(Block::Item {
                    value: items[index].clone(),
                });
                index += 1;
            }
            Some(Control::For { name, path }) => {
                let (body, next) = parse_block_level(items, index + 1, Some(EndKind::For), marker)?;
                blocks.push(Block::For {
                    name,
                    path,
                    source_range: index + 1..next - 1,
                    body,
                });
                index = next;
            }
            Some(Control::If { path }) => {
                let (body, next) = parse_block_level(items, index + 1, Some(EndKind::If), marker)?;
                blocks.push(Block::If { path, body });
                index = next;
            }
            Some(Control::End(actual)) => {
                let Some(expected) = expected else {
                    return Err(template_error(&format!(
                        "unexpected {} marker",
                        actual.name()
                    )));
                };
                if actual != expected {
                    return Err(template_error(&format!(
                        "expected {} before {}",
                        expected.name(),
                        actual.name()
                    )));
                }
                return Ok((blocks, index + 1));
            }
        }
    }
    if let Some(expected) = expected {
        return Err(template_error(&format!(
            "missing {} marker",
            expected.name()
        )));
    }
    Ok((blocks, index))
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(value) => *value,
        Value::Number(value) => value.as_f64().is_some_and(|number| number != 0.0),
        Value::String(value) => !value.is_empty(),
        Value::Array(value) => !value.is_empty(),
        Value::Object(value) => !value.is_empty(),
    }
}

fn resolve_value<'a>(
    root: &'a Value,
    scopes: &'a [Scope],
    path: &[String],
) -> Result<ResolvedValue<'a>> {
    let display_path = path.join(".");
    for scope in scopes.iter().rev() {
        if path
            .first()
            .is_some_and(|component| component == &scope.name)
        {
            if scope.deferred {
                return Ok(ResolvedValue::Deferred);
            }
            return traverse_path(&scope.value, &path[1..], &display_path)
                .map(ResolvedValue::Value);
        }
    }
    let mut saw_deferred = false;
    for scope in scopes.iter().rev() {
        if scope.deferred {
            saw_deferred = true;
            continue;
        }
        if let Some(value) = try_traverse_path(&scope.value, path) {
            return Ok(ResolvedValue::Value(value));
        }
    }
    if let Some(value) = try_traverse_path(root, path) {
        return Ok(ResolvedValue::Value(value));
    }
    if saw_deferred {
        for scope in scopes.iter().rev().filter(|scope| !scope.deferred) {
            if path.first().is_some_and(|component| {
                scope
                    .value
                    .as_object()
                    .is_some_and(|object| object.contains_key(component))
            }) {
                return traverse_path(&scope.value, path, &display_path).map(ResolvedValue::Value);
            }
        }
        if path.first().is_some_and(|component| {
            root.as_object()
                .is_some_and(|object| object.contains_key(component))
        }) {
            return traverse_path(root, path, &display_path).map(ResolvedValue::Value);
        }
        return Ok(ResolvedValue::Deferred);
    }
    traverse_path(root, path, &display_path).map(ResolvedValue::Value)
}

fn try_traverse_path<'a>(mut value: &'a Value, path: &[String]) -> Option<&'a Value> {
    for component in path {
        value = value.as_object()?.get(component)?;
    }
    Some(value)
}

fn traverse_path<'a>(
    mut value: &'a Value,
    path: &[String],
    display_path: &str,
) -> Result<&'a Value> {
    for component in path {
        let Value::Object(object) = value else {
            return Err(template_error(&format!(
                "path `{display_path}` crosses a non-object value"
            )));
        };
        let Some(next) = object.get(component) else {
            return Err(template_error(&format!("missing path `{display_path}`")));
        };
        value = next;
    }
    Ok(value)
}

fn scalar_text(value: &Value, path: &[String]) -> Result<String> {
    match value {
        Value::String(value) => Ok(value.clone()),
        Value::Number(value) => Ok(value.to_string()),
        Value::Bool(value) => Ok(value.to_string()),
        Value::Null => Ok(String::new()),
        Value::Array(_) | Value::Object(_) => Err(template_error(&format!(
            "path `{}` does not resolve to a scalar value",
            path.join(".")
        ))),
    }
}

fn parse_path(inner: &str) -> Result<Vec<&str>> {
    let path = inner.trim();
    if path.is_empty() {
        return Err(template_error("empty scalar path"));
    }
    let components = path.split('.').collect::<Vec<_>>();
    if components.iter().any(|component| {
        component.is_empty()
            || component.chars().any(|character| {
                character.is_whitespace() || matches!(character, '{' | '}' | '%' | '\0')
            })
    }) {
        return Err(template_error("invalid dotted scalar path"));
    }
    Ok(components)
}

fn template_error(message: &str) -> Error {
    Error::Other(format!("invalid template: {message}"))
}

fn collect_table(table: &CT_Tbl, sources: &mut Vec<String>) {
    for (_, _, control) in &table.content_controls {
        collect_control(control, sources);
    }
    for row in &table.rows {
        collect_row(row, sources);
    }
}

fn collect_control(control: &CT_Sdt, sources: &mut Vec<String>) {
    for content in &control.content {
        match content {
            SdtContent::Paragraph(paragraph) => {
                sources.extend(placeholder::replaceable_texts(paragraph));
            }
            SdtContent::Table(table) => collect_table(table, sources),
            SdtContent::Row(row) => collect_row(row, sources),
            SdtContent::Cell(cell) => collect_cell(cell, sources),
            SdtContent::ContentControl(nested) => collect_control(nested, sources),
            SdtContent::Run(_) | SdtContent::RawXml(_) => {}
        }
    }
}

fn collect_row(row: &CT_Row, sources: &mut Vec<String>) {
    for (_, _, control) in &row.content_controls {
        collect_control(control, sources);
    }
    for cell in &row.cells {
        collect_cell(cell, sources);
    }
}

fn collect_row_marker_sources(row: &CT_Row, sources: &mut Vec<String>) {
    for (_, _, control) in &row.content_controls {
        collect_control_marker_sources(control, sources);
    }
    for cell in &row.cells {
        collect_cell_marker_sources(cell, sources);
    }
}

fn collect_cell_marker_sources(cell: &CT_Tc, sources: &mut Vec<String>) {
    for content in &cell.content {
        match content {
            CellContent::Paragraph(paragraph) => {
                sources.extend(placeholder::replaceable_texts(paragraph));
            }
            CellContent::Table(_) => {}
            CellContent::ContentControl(control) => {
                collect_control_marker_sources(control, sources);
            }
        }
    }
}

fn collect_control_marker_sources(control: &CT_Sdt, sources: &mut Vec<String>) {
    for content in &control.content {
        match content {
            SdtContent::Paragraph(paragraph) => {
                sources.extend(placeholder::replaceable_texts(paragraph));
            }
            SdtContent::Table(_) => {}
            SdtContent::Row(row) => collect_row_marker_sources(row, sources),
            SdtContent::Cell(cell) => collect_cell_marker_sources(cell, sources),
            SdtContent::ContentControl(nested) => {
                collect_control_marker_sources(nested, sources);
            }
            SdtContent::Run(run) => sources.push(
                run.content
                    .iter()
                    .filter_map(|content| match content {
                        RunContent::Text(text) => Some(text.text.as_str()),
                        _ => None,
                    })
                    .collect(),
            ),
            SdtContent::RawXml(_) => {}
        }
    }
}

fn collect_cell(cell: &CT_Tc, sources: &mut Vec<String>) {
    for content in &cell.content {
        match content {
            CellContent::Paragraph(paragraph) => {
                sources.extend(placeholder::replaceable_texts(paragraph));
            }
            CellContent::Table(table) => collect_table(table, sources),
            CellContent::ContentControl(control) => collect_control(control, sources),
        }
    }
}

#[cfg(test)]
pub(crate) fn chart_sources(xml: &[u8]) -> Result<Vec<String>> {
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(false);
    let mut sources = Vec::new();
    let mut buffer = Vec::new();

    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Eof) => break,
            Ok(Event::Start(element))
                if matches_local_name(element.name().as_ref(), b"t")
                    || matches_local_name(element.name().as_ref(), b"v") =>
            {
                sources.push(oxml_core::xml_text::read_element_text(
                    &mut reader,
                    element.name(),
                ));
            }
            Ok(_) => {}
            Err(error) => return Err(template_xml_error(error)),
        }
        buffer.clear();
    }

    Ok(sources)
}

fn template_xml_error(error: quick_xml::Error) -> Error {
    Error::Other(format!("invalid template XML: {error}"))
}

#[cfg(test)]
#[test]
fn versioned_template_delimiters_are_quoted_and_escaped_once() {
    let source = r#"A{{ ["x}}|y"] | default("{%literal%}") }}B {{{{name}}}} C {%{% if q %}%}"#;
    let tokens = scalar_tokens(source, true).unwrap();
    let data = serde_json::json!({"x}}|y":"{{inserted}}\u{e000}rdocx-template-0\u{e001}"});
    let mut output = source.to_owned();
    for token in tokens.into_iter().rev() {
        let value = match token.value {
            ScalarValue::Expression(expression) => {
                expression_text(&expression, &data, &[]).unwrap()
            }
            ScalarValue::Literal(text) => text.to_owned(),
        };
        output.replace_range(token.range, &value);
    }
    assert_eq!(
        output,
        "A{{inserted}}\u{e000}rdocx-template-0\u{e001}B {{name}} C {% if q %}"
    );
    for source in [
        "{{ missing",
        "{{ x | default(\"unterminated) }}",
        "{{ x {% }}",
        "bad }}",
    ] {
        assert!(scalar_tokens(source, true).is_err(), "{source}");
    }
}
