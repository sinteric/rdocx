use std::io::Write;

use oxml_core::OxmlError;
use oxml_core::raw_xml::{capture_element, capture_empty_element};
use oxml_core::xml::matches_local_name;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::{Reader, Writer};

use crate::order::OrderedRawChildren;

pub mod body;
pub mod bullet;
pub mod list_style;
pub mod paragraph;

pub use body::{
    CT_TextBodyProperties, Coordinate32Value, NormalAutofit, TextAnchor, TextAutofit, TextError,
    TextVertical, TextWrap,
};
pub use bullet::{
    TextAutoNumber, TextAutoNumberScheme, TextBullet, TextBulletCharacter, TextBulletChoice,
    TextBulletColor, TextBulletSize, TextBulletSizeValue, TextNoBullet,
};
pub use list_style::CT_TextListStyle;
pub use paragraph::{
    CT_RegularTextRun, CT_TextCharacterProperties, CT_TextField, CT_TextLineBreak,
    CT_TextParagraph, CT_TextParagraphProperties, TextAlignment, TextFont, TextHyperlink,
    TextPointValue, TextRun, TextSpace, TextSpacing, TextStrike, TextUnderline, TextValue,
    escape_invalid_xml_characters,
};

use body::{Result, missing_end};
use paragraph::{
    TextNamespaceScope, text_capture_opaque_event, text_close_generated_child, text_fragment_scope,
    text_namespace_scope, text_owned_name, text_reject_trailing, text_root_spellings_scoped,
};

#[derive(Clone, Debug)]
struct SourceBodyProperties {
    raw: Vec<u8>,
    original: CT_TextBodyProperties,
    _namespaces: Vec<(String, String)>,
}

/// The `a:txBody` shell with typed body properties and opaque later text stages.
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct CT_TextBody {
    pub body_properties: CT_TextBodyProperties,
    list_style: Option<CT_TextListStyle>,
    paragraphs: Vec<CT_TextParagraph>,
    raw_children: OrderedRawChildren,
    source_root: Vec<String>,
    source_tag: String,
    source_body_properties: Option<SourceBodyProperties>,
}
impl PartialEq for CT_TextBody {
    fn eq(&self, other: &Self) -> bool {
        self.body_properties == other.body_properties
            && self.list_style == other.list_style
            && self.paragraphs == other.paragraphs
            && self.raw_children == other.raw_children
    }
}
impl Eq for CT_TextBody {}

impl Default for CT_TextBody {
    fn default() -> Self {
        Self::new()
    }
}

impl CT_TextBody {
    /// Creates a minimal valid text body with one empty paragraph.
    pub fn new() -> Self {
        Self {
            body_properties: CT_TextBodyProperties::default(),
            list_style: None,
            paragraphs: vec![CT_TextParagraph::default()],
            raw_children: OrderedRawChildren::default(),
            source_root: Vec::new(),
            source_tag: String::new(),
            source_body_properties: None,
        }
    }

    /// Parses a complete `a:txBody` while retaining later text stages.
    pub fn from_xml(xml: &[u8]) -> Result<Self> {
        Self::from_xml_with_inherited_namespaces(xml, &[])
    }

    /// Parses a complete text body under a caller-selected root local name.
    pub fn from_xml_as(xml: &[u8], root_local_name: &[u8]) -> Result<Self> {
        Self::from_xml_as_with_namespaces(xml, root_local_name, &[])
    }

    /// Parses a text-body fragment in its actual enclosing namespace scope.
    pub fn from_xml_with_inherited_namespaces(
        xml: &[u8],
        inherited: &[(String, String)],
    ) -> Result<Self> {
        Self::from_xml_as_with_namespaces(xml, b"txBody", inherited)
    }

    /// Parses a caller-selected text-body root in its actual enclosing namespace scope.
    /// Retained source declarations stay separate from inherited lookup bindings.
    pub fn from_xml_as_with_namespaces(
        xml: &[u8],
        root_local_name: &[u8],
        inherited: &[(String, String)],
    ) -> Result<Self> {
        let mut reader = Reader::from_reader(xml);
        let mut buffer = Vec::new();
        loop {
            match reader
                .read_event_into(&mut buffer)
                .map_err(OxmlError::from)?
            {
                Event::Start(element)
                    if matches_local_name(element.name().as_ref(), root_local_name) =>
                {
                    let scope = text_namespace_scope(&element, inherited)?;
                    let name = element.name();
                    let spelling = std::str::from_utf8(name.as_ref()).map_err(OxmlError::from)?;
                    let prefix = spelling.split_once(':').map_or("", |(prefix, _)| prefix);
                    if scope.iter().any(|(key, uri)| {
                        key == prefix
                            && uri != crate::namespace::A_NS
                            && uri != "http://schemas.openxmlformats.org/presentationml/2006/main"
                            && !(root_local_name == b"t"
                                && uri
                                    == "http://schemas.openxmlformats.org/drawingml/2006/diagram")
                            && !(root_local_name == b"txPr"
                                && uri == "http://schemas.openxmlformats.org/drawingml/2006/chart")
                            && !(root_local_name == b"txBody"
                                && uri
                                    == "http://schemas.microsoft.com/office/powerpoint/2018/8/main")
                    }) {
                        return Err(TextError::UnexpectedElement(
                            "foreign text body namespace".to_owned(),
                        ));
                    }
                    let parsed = Self::from_element(
                        &mut reader,
                        root_local_name,
                        &element,
                        &TextNamespaceScope::inherited(inherited),
                    )?;
                    text_reject_trailing(&mut reader)?;
                    return Ok(parsed);
                }
                Event::Empty(element)
                    if matches_local_name(element.name().as_ref(), root_local_name) =>
                {
                    return Err(TextError::MissingBodyProperties);
                }
                Event::Start(element) | Event::Empty(element) => {
                    return Err(TextError::UnexpectedElement(element_name(&element)));
                }
                Event::Eof => {
                    return Err(TextError::Xml(OxmlError::MissingElement(
                        "DrawingML text body".to_owned(),
                    )));
                }
                _ => {}
            }
            buffer.clear();
        }
    }

    fn from_element(
        reader: &mut Reader<&[u8]>,
        root_local_name: &[u8],
        start: &BytesStart<'_>,
        inherited: &TextNamespaceScope,
    ) -> Result<Self> {
        let namespaces = text_fragment_scope(start, inherited)?;
        let source_root = text_root_spellings_scoped(
            start,
            inherited,
            &capture_element(&mut reader.clone(), start)?,
        )?;
        let mut body_properties = None;
        let mut source_body_properties = None;
        let mut list_style = None;
        let mut paragraphs = Vec::new();
        let mut raw_children = OrderedRawChildren::default();
        let mut boundary = 0;
        let mut buffer = Vec::new();

        loop {
            match reader
                .read_event_into(&mut buffer)
                .map_err(OxmlError::from)?
            {
                Event::Start(element) if text_owned_name(&element, &namespaces)? == b"bodyPr" => {
                    if body_properties.is_some() {
                        return Err(TextError::DuplicateElement("bodyPr".to_owned()));
                    }
                    let raw = capture_element(reader, &element)?;
                    let parsed = CT_TextBodyProperties::from_xml(&raw)?;
                    source_body_properties = Some(SourceBodyProperties {
                        raw,
                        original: parsed.clone(),
                        _namespaces: namespaces.physical.clone(),
                    });
                    body_properties = Some(parsed);
                    boundary = boundary.max(1);
                }
                Event::Empty(element) if text_owned_name(&element, &namespaces)? == b"bodyPr" => {
                    if body_properties.is_some() {
                        return Err(TextError::DuplicateElement("bodyPr".to_owned()));
                    }
                    let raw = capture_empty_element(&element)?;
                    let parsed = CT_TextBodyProperties::from_xml(&raw)?;
                    source_body_properties = Some(SourceBodyProperties {
                        raw,
                        original: parsed.clone(),
                        _namespaces: namespaces.physical.clone(),
                    });
                    body_properties = Some(parsed);
                    boundary = boundary.max(1);
                }
                Event::Start(element) if text_owned_name(&element, &namespaces)? == b"lstStyle" => {
                    if list_style.is_some() {
                        return Err(TextError::DuplicateElement("lstStyle".to_owned()));
                    }
                    let raw = capture_element(reader, &element)?;
                    list_style = Some(CT_TextListStyle::from_xml_with_namespaces(
                        &raw,
                        &namespaces,
                    )?);
                    boundary = boundary.max(2);
                }
                Event::Empty(element) if text_owned_name(&element, &namespaces)? == b"lstStyle" => {
                    if list_style.is_some() {
                        return Err(TextError::DuplicateElement("lstStyle".to_owned()));
                    }
                    let raw = capture_empty_element(&element)?;
                    list_style = Some(CT_TextListStyle::from_xml_with_namespaces(
                        &raw,
                        &namespaces,
                    )?);
                    boundary = boundary.max(2);
                }
                Event::Start(element) if text_owned_name(&element, &namespaces)? == b"p" => {
                    let raw = capture_element(reader, &element)?;
                    paragraphs.push(CT_TextParagraph::from_xml_with_namespaces(
                        &raw,
                        &namespaces,
                    )?);
                    boundary = boundary.max(2 + paragraphs.len());
                }
                Event::Empty(element) if text_owned_name(&element, &namespaces)? == b"p" => {
                    let raw = capture_empty_element(&element)?;
                    paragraphs.push(CT_TextParagraph::from_xml_with_namespaces(
                        &raw,
                        &namespaces,
                    )?);
                    boundary = boundary.max(2 + paragraphs.len());
                }
                Event::Start(element) => {
                    raw_children.push(boundary, capture_element(reader, &element)?)
                }
                Event::Empty(element) => {
                    raw_children.push(boundary, capture_empty_element(&element)?)
                }
                Event::End(element)
                    if matches_local_name(element.name().as_ref(), root_local_name) =>
                {
                    break;
                }
                Event::Eof => {
                    return Err(missing_end(&String::from_utf8_lossy(root_local_name)));
                }
                event @ (Event::Comment(_) | Event::PI(_) | Event::Text(_) | Event::CData(_)) => {
                    raw_children.push(boundary, text_capture_opaque_event(event)?);
                }
                _ => {}
            }
            buffer.clear();
        }

        let body_properties = body_properties.ok_or(TextError::MissingBodyProperties)?;
        Ok(Self {
            body_properties,
            list_style,
            paragraphs,
            raw_children,
            source_root,
            source_tag: std::str::from_utf8(start.name().as_ref())
                .map_err(OxmlError::from)?
                .to_owned(),
            source_body_properties,
        })
    }

    /// Writes the shell with canonical outer prefixes and schema order.
    pub fn to_xml(&self) -> Result<Vec<u8>> {
        let mut writer = Writer::new(Vec::new());
        self.write_xml(&mut writer)?;
        Ok(writer.into_inner())
    }

    /// Writes the shell into an existing XML writer.
    pub fn write_xml<W: Write>(&self, writer: &mut Writer<W>) -> Result<()> {
        self.write_xml_as(writer, "a:txBody")
    }

    /// Writes the text body using a caller-selected OOXML wrapper tag.
    pub fn write_xml_as<W: Write>(&self, writer: &mut Writer<W>, tag: &str) -> Result<()> {
        let start = paragraph::text_wrapper_start(tag, &self.source_tag, &self.source_root)?;
        let owned_tag = std::str::from_utf8(start.name().as_ref())
            .map_err(OxmlError::from)?
            .to_owned();
        let tag = owned_tag.as_str();
        let namespaces = paragraph::text_namespace_scope(&start, &[])?;
        writer
            .write_event(Event::Start(start))
            .map_err(OxmlError::from)?;
        emit_raw(writer, self.raw_children.at(0))?;
        if let Some(SourceBodyProperties {
            raw,
            original,
            _namespaces,
        }) = &self.source_body_properties
        {
            if &self.body_properties == original && !_namespaces.is_empty() {
                writer.get_mut().write_all(raw).map_err(OxmlError::from)?;
            } else {
                {
                    let mut child = Writer::new(Vec::new());
                    self.body_properties.write_xml(&mut child)?;
                    writer
                        .get_mut()
                        .write_all(&text_close_generated_child(
                            &child.into_inner(),
                            &namespaces,
                        )?)
                        .map_err(OxmlError::from)?;
                }
            }
        } else {
            {
                let mut child = Writer::new(Vec::new());
                self.body_properties.write_xml(&mut child)?;
                writer
                    .get_mut()
                    .write_all(&text_close_generated_child(
                        &child.into_inner(),
                        &namespaces,
                    )?)
                    .map_err(OxmlError::from)?;
            }
        }
        emit_raw(writer, self.raw_children.at(1))?;
        if let Some(list_style) = &self.list_style {
            {
                let mut child = Writer::new(Vec::new());
                list_style.write_xml(&mut child)?;
                writer
                    .get_mut()
                    .write_all(&text_close_generated_child(
                        &child.into_inner(),
                        &namespaces,
                    )?)
                    .map_err(OxmlError::from)?;
            }
        }
        emit_raw(writer, self.raw_children.at(2))?;
        for (index, paragraph) in self.paragraphs.iter().enumerate() {
            {
                let mut child = Writer::new(Vec::new());
                paragraph.write_xml(&mut child)?;
                writer
                    .get_mut()
                    .write_all(&text_close_generated_child(
                        &child.into_inner(),
                        &namespaces,
                    )?)
                    .map_err(OxmlError::from)?;
            }
            emit_raw(writer, self.raw_children.at(3 + index))?;
        }
        writer
            .write_event(Event::End(BytesEnd::new(tag)))
            .map_err(OxmlError::from)?;
        Ok(())
    }

    /// Returns plain text in paragraph and text-content order.
    pub fn plain_text(&self) -> String {
        self.paragraphs
            .iter()
            .map(|paragraph| {
                let mut text = String::new();
                for run in &paragraph.runs {
                    match run {
                        TextRun::Run(run) => text.push_str(&run.text.value),
                        TextRun::Break(_) => text.push('\n'),
                        TextRun::Field(field) => {
                            if let Some(value) = &field.text {
                                text.push_str(&value.value);
                            }
                        }
                    }
                }
                text
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn has_list_style(&self) -> bool {
        self.list_style.is_some()
    }

    pub fn list_style(&self) -> Option<&CT_TextListStyle> {
        self.list_style.as_ref()
    }

    pub fn paragraph_count(&self) -> usize {
        self.paragraphs.len()
    }

    pub fn paragraphs(&self) -> &[CT_TextParagraph] {
        &self.paragraphs
    }

    /// Replaces all text content while retaining body-level state.
    ///
    /// Each line feed or CRLF pair starts a paragraph, and a vertical tab
    /// creates an `a:br`. New paragraphs copy the first paragraph's formatting.
    pub fn set_text(&mut self, text: &str) {
        let old_paragraph_count = self.paragraphs.len();
        let mut first = self.paragraphs.drain(..).next().unwrap_or_default();
        let mut lines = text.split("\r\n").flat_map(|part| part.split('\n'));
        first.set_text(lines.next().unwrap_or_default());
        let mut template = CT_TextParagraph::default();
        template.properties = first.properties.clone();
        template.runs = first.runs.iter().take(1).cloned().collect();
        template.end_properties = first.end_properties.clone();
        self.paragraphs.push(first);
        for line in lines {
            let mut paragraph = template.clone();
            paragraph.set_text(line);
            self.paragraphs.push(paragraph);
        }
        self.reconcile_paragraph_raw_children(old_paragraph_count, self.paragraphs.len());
    }

    /// Returns an empty body that keeps this body's properties and list style
    /// and the formatting of its first paragraph, as PowerPoint fills the
    /// cells of a new table row or column.
    ///
    /// The one paragraph keeps the first paragraph's properties. The
    /// character properties of that paragraph's first regular run, or its end
    /// properties when the run is missing or has none, become the new end
    /// properties without their hyperlinks. Unmodelled body and paragraph
    /// children are not copied.
    pub fn empty_like(&self) -> Self {
        let mut paragraph = CT_TextParagraph::default();
        if let Some(first) = self.paragraphs.first() {
            let first_run = first.runs.iter().find_map(|run| match run {
                TextRun::Run(run) => Some(run),
                TextRun::Break(_) | TextRun::Field(_) => None,
            });
            paragraph.properties = first.properties.clone();
            paragraph.end_properties = first_run
                .and_then(|run| run.properties.clone())
                .or_else(|| first.end_properties.clone())
                .map(CT_TextCharacterProperties::without_hyperlinks);
        }
        Self {
            body_properties: self.body_properties.clone(),
            list_style: self.list_style.clone(),
            paragraphs: vec![paragraph],
            raw_children: OrderedRawChildren::default(),
            source_root: if self.body_properties.autofit_source.is_some() {
                self.source_root.clone()
            } else {
                Vec::new()
            },
            source_tag: if self.body_properties.autofit_source.is_some() {
                self.source_tag.clone()
            } else {
                String::new()
            },
            source_body_properties: if self.body_properties.autofit_source.is_some() {
                self.source_body_properties.clone()
            } else {
                None
            },
        }
    }

    /// Returns one paragraph for in-place mutation.
    pub fn paragraph_mut(&mut self, index: usize) -> Option<&mut CT_TextParagraph> {
        self.paragraphs.get_mut(index)
    }

    /// Appends one empty paragraph after all existing paragraphs.
    pub fn add_paragraph(&mut self) -> &mut CT_TextParagraph {
        self.raw_children
            .shift_boundaries_from(2 + self.paragraphs.len());
        self.paragraphs.push(CT_TextParagraph::default());
        self.paragraphs.last_mut().expect("paragraph was appended")
    }

    /// Moves visible typed paragraphs to `destination`, retaining body-level
    /// state and leaving one empty paragraph in this body.
    pub fn move_content_to(&mut self, destination: &mut Self) {
        if self.plain_text().is_empty() {
            return;
        }

        let source_paragraph_count = self.paragraphs.len();
        let moved = std::mem::take(&mut self.paragraphs);
        self.paragraphs.push(CT_TextParagraph::default());
        self.reconcile_paragraph_raw_children(source_paragraph_count, 1);

        if destination.plain_text().is_empty() {
            let destination_paragraph_count = destination.paragraphs.len();
            destination.paragraphs = moved;
            destination.reconcile_paragraph_raw_children(
                destination_paragraph_count,
                destination.paragraphs.len(),
            );
            return;
        }

        for paragraph in moved {
            destination
                .raw_children
                .shift_boundaries_from(2 + destination.paragraphs.len());
            destination.paragraphs.push(paragraph);
        }
    }

    fn reconcile_paragraph_raw_children(
        &mut self,
        old_paragraph_count: usize,
        new_paragraph_count: usize,
    ) {
        let mut raw_children = OrderedRawChildren::default();
        for boundary in 0..=2 + old_paragraph_count {
            let new_boundary = if boundary <= 2 {
                boundary
            } else {
                2 + new_paragraph_count
            };
            for child in self.raw_children.at(boundary) {
                raw_children.push(new_boundary, child.to_vec());
            }
        }
        self.raw_children = raw_children;
    }

    pub fn raw_children(&self) -> &OrderedRawChildren {
        &self.raw_children
    }
}

fn emit_raw<'a, W: Write>(
    writer: &mut Writer<W>,
    children: impl Iterator<Item = &'a [u8]>,
) -> Result<()> {
    for child in children {
        writer.get_mut().write_all(child).map_err(OxmlError::from)?;
    }
    Ok(())
}

fn element_name(element: &BytesStart<'_>) -> String {
    String::from_utf8_lossy(element.name().as_ref()).into_owned()
}

#[cfg(test)]
mod tests {
    use std::panic;

    use super::{CT_TextBody, CT_TextListStyle};

    #[test]
    fn autofit_opaque_choice_keeps_inherited_wrapper_namespace_scope() {
        let xml = br#"<q:txBody xmlns:q='http://schemas.openxmlformats.org/drawingml/2006/main' xmlns:e='urn:example'><q:bodyPr><q:normAutofit e:proof='keep&amp;exact'/></q:bodyPr><q:p/></q:txBody>"#;
        let mut body = CT_TextBody::from_xml(xml).unwrap();
        body.body_properties.autofit = Some(super::TextAutofit::NoAutofit);
        let saved = body.to_xml().unwrap();
        let saved = String::from_utf8(saved).unwrap();
        assert!(saved.starts_with("<q:txBody xmlns:q='http://schemas.openxmlformats.org/drawingml/2006/main' xmlns:e='urn:example'>"));
        assert!(saved.contains("<q:noAutofit e:proof='keep&amp;exact'/>"));
        let empty = String::from_utf8(body.empty_like().to_xml().unwrap()).unwrap();
        assert!(empty.contains("xmlns:q='http://schemas.openxmlformats.org/drawingml/2006/main'"));
        assert!(empty.contains("xmlns:e='urn:example'"));
        assert!(empty.contains("<q:noAutofit e:proof='keep&amp;exact'/>"));
        assert_eq!(
            CT_TextBody::from_xml(saved.as_bytes())
                .unwrap()
                .to_xml()
                .unwrap(),
            saved.as_bytes()
        );

        // The retained owner spelling remains qualified even when a is foreign.
        // Generated canonical descendants close their own DrawingML binding.
        let conflicting = String::from_utf8(xml.to_vec())
            .unwrap()
            .replace("xmlns:e=", "xmlns:a='urn:foreign' xmlns:e=");
        let mut body = CT_TextBody::from_xml(conflicting.as_bytes()).unwrap();
        body.body_properties.autofit = Some(super::TextAutofit::NoAutofit);
        let saved = String::from_utf8(body.to_xml().unwrap()).unwrap();
        assert!(saved.starts_with("<q:txBody"));
        assert!(saved.contains("xmlns:a='urn:foreign'"));
        assert!(saved.contains("<q:noAutofit e:proof='keep&amp;exact'/>"));
        assert_eq!(
            CT_TextBody::from_xml(saved.as_bytes())
                .unwrap()
                .to_xml()
                .unwrap(),
            saved.as_bytes()
        );
    }

    #[test]
    fn text_body_reads_any_prefix_and_writes_the_fixed_a_prefix() {
        let xml = br#"<q:txBody><x:before/><q:bodyPr anchor="ctr"><q:noAutofit/></q:bodyPr><x:afterBody/><q:lstStyle><x:listChild/></q:lstStyle><x:beforeParagraph/><q:p><x:run>kept</x:run></q:p><x:afterParagraph/></q:txBody>"#;
        let body = CT_TextBody::from_xml(xml).unwrap();
        assert!(body.has_list_style());
        assert_eq!(body.paragraph_count(), 1);
        assert_eq!(body.to_xml().unwrap(), br#"<a:txBody><x:before/><a:bodyPr anchor="ctr"><a:noAutofit/></a:bodyPr><x:afterBody/><a:lstStyle><x:listChild/></a:lstStyle><x:beforeParagraph/><a:p><x:run>kept</x:run></a:p><x:afterParagraph/></a:txBody>"#);
    }

    #[test]
    fn moving_content_retains_typed_paragraphs_and_source_body_state() {
        let mut destination = CT_TextBody::from_xml(
            br#"<a:txBody><a:bodyPr/><a:p><a:r><a:rPr b="1"/><a:t>one</a:t></a:r></a:p></a:txBody>"#,
        )
        .unwrap();
        let mut source = CT_TextBody::from_xml(
            br#"<a:txBody><a:bodyPr lIns="10"/><a:p><a:r><a:rPr i="1"/><a:t>two</a:t></a:r></a:p><a:p><a:r><a:t>three</a:t></a:r></a:p></a:txBody>"#,
        )
        .unwrap();

        source.move_content_to(&mut destination);

        assert_eq!(destination.plain_text(), "one\ntwo\nthree");
        assert_eq!(destination.paragraph_count(), 3);
        assert_eq!(source.plain_text(), "");
        assert_eq!(source.paragraph_count(), 1);
        assert!(
            String::from_utf8(source.to_xml().unwrap())
                .unwrap()
                .contains(r#"lIns="10""#)
        );
        let written = String::from_utf8(destination.to_xml().unwrap()).unwrap();
        assert!(written.contains(r#"<a:rPr b="1"/>"#));
        assert!(written.contains(r#"<a:rPr i="1"/>"#));
    }

    #[test]
    fn set_text_starts_a_paragraph_at_each_line_feed_and_breaks_at_each_vertical_tab() {
        let mut body = CT_TextBody::from_xml(
            br#"<a:txBody xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:x="urn:x"><a:bodyPr/><a:p><a:pPr algn="ctr"/><a:r><a:rPr b="1"/><a:t>old</a:t></a:r><x:in-paragraph/><a:endParaRPr sz="1200"/></a:p><a:p><a:r><a:t>gone</a:t></a:r></a:p><x:after-paragraphs/></a:txBody>"#,
        )
        .unwrap();
        body.set_text("one\r\ntwo\u{b}soft\u{b}\n");
        assert_eq!(
            String::from_utf8(body.to_xml().unwrap()).unwrap(),
            concat!(
                r#"<a:txBody xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:x="urn:x"><a:bodyPr/>"#,
                r#"<a:p xmlns:x="urn:x"><a:pPr algn="ctr"/><a:r><a:rPr b="1"/><a:t>one</a:t></a:r><x:in-paragraph/><a:endParaRPr sz="1200"/></a:p>"#,
                r#"<a:p><a:pPr algn="ctr"/><a:r><a:rPr b="1"/><a:t>two</a:t></a:r><a:br><a:rPr b="1"/></a:br><a:r><a:rPr b="1"/><a:t>soft</a:t></a:r><a:br><a:rPr b="1"/></a:br><a:endParaRPr sz="1200"/></a:p>"#,
                r#"<a:p><a:pPr algn="ctr"/><a:r><a:rPr b="1"/><a:t/></a:r><a:endParaRPr sz="1200"/></a:p>"#,
                r#"<x:after-paragraphs/></a:txBody>"#,
            )
        );
        assert_eq!(body.plain_text(), "one\ntwo\nsoft\n\n");

        let saved = body.to_xml().unwrap();
        assert_eq!(
            CT_TextBody::from_xml(&saved).unwrap().to_xml().unwrap(),
            saved
        );

        let mut plain = CT_TextBody::new();
        plain.set_text("single line");
        assert_eq!(plain.paragraph_count(), 1);
        assert_eq!(plain.plain_text(), "single line");
    }

    #[test]
    fn schema_valid_text_body_using_all_nine_list_levels_round_trips_structurally() {
        let xml = br#"<q:txBody><q:bodyPr/><q:lstStyle><q:lvl1pPr lvl="0"><q:buChar char="*"/></q:lvl1pPr><q:lvl2pPr marL="100"><q:buAutoNum type="arabicPeriod" startAt="2"/></q:lvl2pPr><q:lvl3pPr><q:buNone/></q:lvl3pPr><q:lvl4pPr><q:defRPr sz="1200"/></q:lvl4pPr><q:lvl5pPr algn="ctr"/><x:extension x:id="5"><x:child>one &amp; two</x:child></x:extension><q:lvl6pPr marR="200"/><q:lvl7pPr><q:spcBef><q:spcPts val="600"/></q:spcBef></q:lvl7pPr><q:lvl8pPr><q:buSzPct val="125000"/><q:buFont typeface="Wingdings"/><q:buChar char="o"/></q:lvl8pPr><q:lvl9pPr indent="-100"/></q:lstStyle><q:p><q:pPr lvl="1"/><q:r><q:t xml:space="preserve"> item </q:t></q:r></q:p></q:txBody>"#;
        let expected = br#"<a:txBody><a:bodyPr/><a:lstStyle><a:lvl1pPr lvl="0"><a:buChar char="*"/></a:lvl1pPr><a:lvl2pPr marL="100"><a:buAutoNum type="arabicPeriod" startAt="2"/></a:lvl2pPr><a:lvl3pPr><a:buNone/></a:lvl3pPr><a:lvl4pPr><a:defRPr sz="1200"/></a:lvl4pPr><a:lvl5pPr algn="ctr"/><x:extension x:id="5"><x:child>one &amp; two</x:child></x:extension><a:lvl6pPr marR="200"/><a:lvl7pPr><a:spcBef><a:spcPts val="600"/></a:spcBef></a:lvl7pPr><a:lvl8pPr><a:buSzPct val="125000"/><a:buFont typeface="Wingdings"/><a:buChar char="o"/></a:lvl8pPr><a:lvl9pPr indent="-100"/></a:lstStyle><a:p><a:pPr lvl="1"/><a:r><a:t xml:space="preserve"> item </a:t></a:r></a:p></a:txBody>"#;

        let body = CT_TextBody::from_xml(xml).unwrap();
        let written = body.to_xml().unwrap();
        assert_eq!(written, expected);
        assert_eq!(CT_TextBody::from_xml(&written).unwrap(), body);
    }

    #[test]
    fn list_style_levels_write_in_ascending_schema_order() {
        let body = CT_TextBody::from_xml(
            br#"<q:txBody><q:bodyPr/><q:lstStyle><q:lvl9pPr indent="-9"/><q:lvl5pPr indent="-5"/><q:lvl1pPr indent="-1"/></q:lstStyle><q:p/></q:txBody>"#,
        )
        .unwrap();
        assert_eq!(
            body.to_xml().unwrap(),
            br#"<a:txBody><a:bodyPr/><a:lstStyle><a:lvl1pPr indent="-1"/><a:lvl5pPr indent="-5"/><a:lvl9pPr indent="-9"/></a:lstStyle><a:p/></a:txBody>"#
        );
    }

    #[test]
    fn unknown_list_style_children_round_trip_byte_for_byte() {
        let body = CT_TextBody::from_xml(
            br#"<q:txBody><q:bodyPr/><q:lstStyle><x:before x:id="1"/><q:lvl1pPr/><x:between><x:nested>one &amp; two</x:nested><!--note--></x:between><q:lvl2pPr/><x:after x:id="9"/></q:lstStyle><q:p/></q:txBody>"#,
        )
        .unwrap();
        assert_eq!(
            body.to_xml().unwrap(),
            br#"<a:txBody><a:bodyPr/><a:lstStyle><x:before x:id="1"/><a:lvl1pPr/><x:between><x:nested>one &amp; two</x:nested><!--note--></x:between><a:lvl2pPr/><x:after x:id="9"/></a:lstStyle><a:p/></a:txBody>"#
        );
    }

    #[test]
    fn list_style_rejects_nested_fixed_prefix_rebinding() {
        let positive = br#"<p:defaultTextStyle xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:d="http://schemas.openxmlformats.org/drawingml/2006/main"><d:lvl1pPr/></p:defaultTextStyle>"#;
        assert!(CT_TextListStyle::from_xml(positive).is_ok());
        let xml = br#"<p:defaultTextStyle xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:d="http://schemas.openxmlformats.org/drawingml/2006/main"><d:lvl1pPr xmlns:a="urn:producer"><a:raw/></d:lvl1pPr></p:defaultTextStyle>"#;
        assert!(CT_TextListStyle::from_xml(xml).is_err());
    }

    #[test]
    fn list_style_rejects_fixed_prefix_rebinding_at_typed_descendants() {
        let descendants: &[&[u8]] = &[
            br#"<d:defRPr xmlns:a="urn:producer"/>"#,
            br#"<d:spcBef xmlns:a="urn:producer"><d:spcPts val="600"/></d:spcBef>"#,
            br#"<d:spcBef><d:spcPts xmlns:a="urn:producer" val="600"/></d:spcBef>"#,
            br#"<d:buChar xmlns:a="urn:producer" char="*"/>"#,
            br#"<d:buClr xmlns:a="urn:producer"><d:srgbClr val="102030"/></d:buClr>"#,
            br#"<d:buClr><d:srgbClr xmlns:a="urn:producer" val="102030"/></d:buClr>"#,
            br#"<d:defRPr><d:solidFill xmlns:a="urn:producer"><d:srgbClr val="102030"/></d:solidFill></d:defRPr>"#,
            br#"<d:defRPr><d:solidFill><d:srgbClr xmlns:a="urn:producer" val="102030"/></d:solidFill></d:defRPr>"#,
            br#"<d:defRPr><d:solidFill><d:srgbClr val="102030"><d:alpha xmlns:a="urn:producer" val="50000"/></d:srgbClr></d:solidFill></d:defRPr>"#,
        ];

        for descendant in descendants {
            let mut xml = br#"<p:defaultTextStyle xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:d="http://schemas.openxmlformats.org/drawingml/2006/main"><d:lvl1pPr>"#.to_vec();
            xml.extend_from_slice(descendant);
            xml.extend_from_slice(br#"</d:lvl1pPr></p:defaultTextStyle>"#);
            assert!(
                CT_TextListStyle::from_xml(&xml).is_err(),
                "typed descendant accepted a conflicting xmlns:a: {}",
                String::from_utf8_lossy(descendant)
            );
        }
    }

    #[test]
    fn opaque_list_style_child_preserves_its_local_prefix_binding() {
        let opaque = br#"<x:extension xmlns:x="urn:extension" xmlns:a="urn:producer"><a:data/></x:extension>"#;
        let xml = br#"<p:defaultTextStyle xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:d="http://schemas.openxmlformats.org/drawingml/2006/main"><x:extension xmlns:x="urn:extension" xmlns:a="urn:producer"><a:data/></x:extension><d:lvl1pPr/></p:defaultTextStyle>"#;
        let parsed = CT_TextListStyle::from_xml(xml).unwrap();
        let written = parsed.to_xml().unwrap();
        assert!(
            written
                .windows(opaque.len())
                .any(|window| window == opaque.as_slice())
        );
    }

    #[test]
    fn opaque_typed_descendants_preserve_their_local_prefix_bindings() {
        let character_extension = br#"<x:extension xmlns:x="urn:extension" xmlns:a="urn:producer"><a:data/></x:extension>"#;
        let transform_with_content =
            br#"<d:alpha xmlns:a="urn:producer" val="50000"><a:data/></d:alpha>"#;
        let xml = br#"<p:defaultTextStyle xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:d="http://schemas.openxmlformats.org/drawingml/2006/main"><d:lvl1pPr><d:defRPr><x:extension xmlns:x="urn:extension" xmlns:a="urn:producer"><a:data/></x:extension><d:solidFill><d:srgbClr val="102030"><d:alpha xmlns:a="urn:producer" val="50000"><a:data/></d:alpha></d:srgbClr></d:solidFill></d:defRPr></d:lvl1pPr></p:defaultTextStyle>"#;

        let written = CT_TextListStyle::from_xml(xml).unwrap().to_xml().unwrap();
        for opaque in [
            character_extension.as_slice(),
            transform_with_content.as_slice(),
        ] {
            assert!(
                written.windows(opaque.len()).any(|window| window == opaque),
                "opaque descendant was not preserved: {}",
                String::from_utf8_lossy(opaque)
            );
        }
    }

    #[test]
    fn invalid_list_levels_return_errors_without_panicking() {
        let cases: &[&[u8]] = &[
            br#"<q:txBody><q:bodyPr/><q:lstStyle><q:lvl0pPr/></q:lstStyle><q:p/></q:txBody>"#,
            br#"<q:txBody><q:bodyPr/><q:lstStyle><q:lvl10pPr/></q:lstStyle><q:p/></q:txBody>"#,
            br#"<q:txBody><q:bodyPr/><q:lstStyle><q:lvl01pPr/></q:lstStyle><q:p/></q:txBody>"#,
            br#"<q:txBody><q:bodyPr/><q:lstStyle><q:lvl1pPr/><q:lvl1pPr/></q:lstStyle><q:p/></q:txBody>"#,
            br#"<q:txBody><q:bodyPr/><q:lstStyle><q:lvl4pPr lvl="9"/></q:lstStyle><q:p/></q:txBody>"#,
            br#"<q:txBody><q:bodyPr/><q:lstStyle><q:lvl7pPr><q:buChar/></q:lvl7pPr></q:lstStyle><q:p/></q:txBody>"#,
        ];

        for xml in cases {
            let result = panic::catch_unwind(|| CT_TextBody::from_xml(xml));
            assert!(result.is_ok(), "list-style parser panicked");
            assert!(result.unwrap().is_err(), "invalid list level parsed");
        }
    }

    #[test]
    fn fx206_text_body_inherited_link_namespace_and_opaque_owner_closure() {
        let inherited = vec![
            ("a".to_owned(), crate::namespace::A_NS.to_owned()),
            (
                "p".to_owned(),
                "http://schemas.openxmlformats.org/presentationml/2006/main".to_owned(),
            ),
            ("q".to_owned(), crate::namespace::A_NS.to_owned()),
            (
                "rel".to_owned(),
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships".to_owned(),
            ),
            ("f".to_owned(), "urn:foreign".to_owned()),
        ];
        let xml = br#"<p:txBody f:body='&#65;'><q:bodyPr f:bodyPr='&#76;'/><q:lstStyle f:list='&#66;'><q:defPPr f:default='&#67;'><q:defRPr><q:hlinkClick rel:id='default'/></q:defRPr></q:defPPr></q:lstStyle><q:p f:paragraph='&#68;'><f:r><q:r><q:t>opaque</q:t></q:r></f:r><q:r f:run='&#69;'><q:rPr xmlns:a='urn:foreign-a' f:properties='&#70;'><a:opaque f:keep='&#80;'/><q:solidFill f:fill='&#79;'><q:srgbClr val='123456'/></q:solidFill><q:latin typeface='Arial' f:font='&#78;'/><f:hlinkClick f:keep='&#71;'><q:hlinkClick rel:id='shadow'/></f:hlinkClick><q:hlinkClick rel:id='owned' tooltip='&#72;'><f:payload f:keep='&#73;'/></q:hlinkClick></q:rPr><q:t f:text='&#77;'>visible</q:t></q:r><q:br xmlns:rel='urn:foreign' f:break='&#74;'><q:rPr><q:hlinkClick rel:id='opaque'/></q:rPr></q:br><q:fld id='{00000000-0000-0000-0000-000000000001}' f:field='&#75;'><q:rPr><q:hlinkClick rel:id='field'/></q:rPr><q:t>field</q:t></q:fld><q:endParaRPr><q:hlinkMouseOver rel:id='hover'/></q:endParaRPr></q:p></p:txBody>"#;
        let mut body = CT_TextBody::from_xml_with_inherited_namespaces(xml, &inherited).unwrap();
        assert_eq!(body.paragraphs.len(), 1);
        assert_eq!(body.paragraphs[0].runs.len(), 3);
        let default = body
            .list_style
            .as_ref()
            .unwrap()
            .default_paragraph_properties
            .as_ref()
            .unwrap()
            .default_run_properties
            .as_ref()
            .unwrap();
        assert_eq!(
            default
                .hyperlink_click
                .as_ref()
                .unwrap()
                .relationship_id
                .as_deref(),
            Some("default")
        );
        let super::TextRun::Run(run) = &mut body.paragraphs[0].runs[0] else {
            panic!("owned regular run");
        };
        let click = run
            .properties
            .as_mut()
            .unwrap()
            .hyperlink_click
            .as_mut()
            .unwrap();
        assert_eq!(click.relationship_id.as_deref(), Some("owned"));
        assert_eq!(click.tooltip.as_deref(), Some("H"));
        click.relationship_id = Some("next".to_owned());
        click.action = Some("ppaction://hlinksldjump".to_owned());
        let super::TextRun::Break(br) = &body.paragraphs[0].runs[1] else {
            panic!("owned break");
        };
        assert_eq!(
            br.properties
                .as_ref()
                .unwrap()
                .hyperlink_click
                .as_ref()
                .unwrap()
                .relationship_id,
            None
        );
        let super::TextRun::Field(field) = &body.paragraphs[0].runs[2] else {
            panic!("owned field");
        };
        assert_eq!(
            field
                .run_properties
                .as_ref()
                .unwrap()
                .hyperlink_click
                .as_ref()
                .unwrap()
                .relationship_id
                .as_deref(),
            Some("field")
        );
        assert_eq!(
            body.paragraphs[0]
                .end_properties
                .as_ref()
                .unwrap()
                .hyperlink_mouse_over
                .as_ref()
                .unwrap()
                .relationship_id
                .as_deref(),
            Some("hover")
        );
        let saved = body.to_xml().unwrap();
        let text = std::str::from_utf8(&saved).unwrap();
        for token in [
            "xmlns:a='urn:foreign-a'",
            "<a:opaque f:keep='&#80;'/>",
            "f:font='&#78;'",
            "f:fill='&#79;'",
            "f:bodyPr='&#76;'",
            "f:text='&#77;'",
            "f:body='&#65;'",
            "f:list='&#66;'",
            "f:default='&#67;'",
            "f:paragraph='&#68;'",
            "f:run='&#69;'",
            "f:properties='&#70;'",
            "tooltip='&#72;'",
            "f:break='&#74;'",
            "f:field='&#75;'",
        ] {
            assert!(text.contains(token), "{token}");
        }
        assert!(text.contains("<f:r><q:r><q:t>opaque</q:t></q:r></f:r>"));
        assert!(text.contains(
            "<f:hlinkClick f:keep='&#71;'><q:hlinkClick rel:id='shadow'/></f:hlinkClick>"
        ));
        assert!(text.contains("<f:payload f:keep='&#73;'/>"));
        assert!(text.contains("rel:id=\"next\""));
        assert!(text.contains("rel:id='opaque'"));
        assert_eq!(
            CT_TextBody::from_xml(&saved).unwrap().to_xml().unwrap(),
            saved
        );
    }
    #[test]
    fn fx206_list_style_foreign_presentation_wrapper_is_not_owned() {
        let input = br#"<p:defaultTextStyle xmlns:p="urn:presentation" xmlns:d="http://schemas.openxmlformats.org/drawingml/2006/main"><d:lvl1pPr/></p:defaultTextStyle>"#;
        assert!(CT_TextListStyle::from_xml(input).is_err());
    }
    #[test]
    fn fx206_presentation_wrappers_preserve_alias_default_and_foreign_canonical_bindings() {
        const P: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
        const A: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
        for local in [
            "defaultTextStyle",
            "titleStyle",
            "bodyStyle",
            "otherStyle",
            "notesStyle",
        ] {
            for prefix in ["q:", ""] {
                let declaration = if prefix.is_empty() {
                    "xmlns"
                } else {
                    "xmlns:q"
                };
                let input = format!(
                    "<{prefix}{local} {declaration}=\"{P}\" xmlns:p='urn:foreign' xmlns:d=\"{A}\"><d:lvl1pPr algn='ctr'/><x:opaque xmlns:x='urn:opaque' x:keep='&#65;'/></{prefix}{local}>"
                );
                let parsed = CT_TextListStyle::from_xml(input.as_bytes()).unwrap();
                let save = |value: &CT_TextListStyle| {
                    let mut writer = quick_xml::Writer::new(Vec::new());
                    value
                        .write_xml_as(&mut writer, &format!("p:{local}"))
                        .unwrap();
                    writer.into_inner()
                };
                let saved = save(&parsed);
                let xml = std::str::from_utf8(&saved).unwrap();
                assert!(xml.starts_with(&format!("<{prefix}{local} ")));
                assert!(xml.contains("xmlns:p='urn:foreign'"));
                assert!(xml.contains("<x:opaque xmlns:x='urn:opaque' x:keep='&#65;'/>"));
                assert_eq!(
                    parsed.level1.as_ref().unwrap().alignment,
                    Some(super::paragraph::TextAlignment::Center)
                );
                assert_eq!(save(&CT_TextListStyle::from_xml(&saved).unwrap()), saved);
                let foreign = input.replacen(P, "urn:not-presentation", 1);
                assert!(CT_TextListStyle::from_xml(foreign.as_bytes()).is_err());
            }
        }
    }
    #[test]
    fn fx206_requested_wrapper_namespace_converts_both_directions_without_rebinding_opaque_prefixes()
     {
        use quick_xml::{NsReader, Writer, events::Event, name::ResolveResult};
        const A: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
        const P: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
        for (source, requested, expected) in [(A, "p:txBody", P), (P, "a:txBody", A)] {
            for alias in ["q:", ""] {
                let declaration = if alias.is_empty() { "xmlns" } else { "xmlns:q" };
                let input = format!(
                    "<{alias}txBody {declaration}='{source}' xmlns:a='{A}' xmlns:p='urn:opaque'><a:bodyPr/><a:lstStyle/><a:p><a:r><a:t>retained text</a:t></a:r></a:p><p:opaque p:keep='&#65;'/></{alias}txBody>"
                );
                let parsed = CT_TextBody::from_xml(input.as_bytes()).unwrap();
                let save = |value: &CT_TextBody| {
                    let mut writer = Writer::new(Vec::new());
                    value.write_xml_as(&mut writer, requested).unwrap();
                    writer.into_inner()
                };
                let saved = save(&parsed);
                let mut reader = NsReader::from_reader(saved.as_slice());
                let (namespace, event) = reader.read_resolved_event().unwrap();
                assert_eq!(
                    namespace,
                    ResolveResult::Bound(quick_xml::name::Namespace(expected.as_bytes()))
                );
                assert!(
                    matches!(event, Event::Start(ref start) if start.local_name().as_ref() == b"txBody")
                );
                let xml = std::str::from_utf8(&saved).unwrap();
                assert!(xml.contains("xmlns:p='urn:opaque'"));
                assert!(xml.contains("<p:opaque p:keep='&#65;'/>"));
                let reopened = CT_TextBody::from_xml(&saved).unwrap();
                assert_eq!(reopened.plain_text(), "retained text");
                assert_eq!(save(&reopened), saved);
            }
        }
    }
}
