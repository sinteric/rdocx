use std::borrow::Cow;
use std::collections::HashMap;

use oxml_layout::{
    Align, Color, FieldKind, FontManager, FontTraceEvent, GlyphRun, InlineItem, LayoutError,
    LayoutLine, LineBreakParams, LineItem, LineSpacing, MultilingualGlyphRun, Paint, Point,
    PositionedElement, Rect, TextSegment, Underline, break_multilingual_into_lines,
};
use rpptx_layout::{
    ParagraphAlignment, ResolvedAutofit, ResolvedBullet, ResolvedBulletSize, ResolvedParagraph,
    ResolvedRunStyle, ResolvedShape, ResolvedTextBody, ResolvedTextRun, ResolvedTextSpacing,
    TextAnchor, TextDirection,
};

use crate::TextLineLayout;

const DEFAULT_FONT_SIZE: f64 = 18.0;

/// PowerPoint's single line spacing, in ems of the largest run on the line.
///
/// It is the same for every font, whatever its ascent, descent and line gap.
const SINGLE_SPACING_EM: f64 = 1.2;

struct AutoNumberSequence {
    scheme: String,
    next: u32,
}

struct NumberingState {
    levels: Vec<Option<AutoNumberSequence>>,
}

#[derive(Default)]
struct ShapingCache {
    entries: Vec<(String, ResolvedRunStyle, TextSegment, Vec<FontTraceEvent>)>,
    bytes: usize,
}

const NATIVE_SHAPING_CACHE_MAX_BYTES: usize = 16 * 1024 * 1024;
const NATIVE_SHAPING_CACHE_MAX_ENTRIES: usize = 4_096;

fn shaping_cache_entry_bytes(
    text: &str,
    style: &ResolvedRunStyle,
    segment: &TextSegment,
    events: &Vec<FontTraceEvent>,
) -> usize {
    let strings = [
        &style.latin_typeface,
        &style.east_asian_typeface,
        &style.complex_script_typeface,
        &style.symbol_typeface,
        &style.language,
        &style.hyperlink_url,
        &segment.hyperlink_url,
    ];
    let mut bytes = text.len().saturating_add(segment.text.capacity());
    for string in strings {
        bytes = bytes.saturating_add(string.as_ref().map_or(0, String::capacity));
    }
    bytes = bytes
        .saturating_add(
            segment
                .glyph_ids
                .capacity()
                .saturating_mul(std::mem::size_of::<u16>()),
        )
        .saturating_add(
            segment
                .advances
                .capacity()
                .saturating_mul(std::mem::size_of::<f64>()),
        )
        .saturating_add(
            events
                .capacity()
                .saturating_mul(std::mem::size_of::<FontTraceEvent>()),
        );
    if let Some(Paint::Linear { stops, .. } | Paint::Radial { stops, .. }) = &style.fill {
        bytes = bytes.saturating_add(
            stops
                .capacity()
                .saturating_mul(std::mem::size_of::<oxml_layout::GradientStop>()),
        );
    }
    if let Some(Paint::Grid { colors, .. }) = &style.fill {
        bytes = bytes.saturating_add(
            colors
                .capacity()
                .saturating_mul(std::mem::size_of::<oxml_layout::Color>()),
        );
    }
    for event in events {
        if let FontTraceEvent::Contribution(contribution) = event {
            bytes = bytes.saturating_add(
                contribution
                    .requested_family
                    .as_ref()
                    .map_or(0, String::capacity),
            );
        }
    }
    bytes
}

impl ShapingCache {
    fn shape(
        &mut self,
        font_manager: &mut FontManager,
        text: &str,
        style: &ResolvedRunStyle,
    ) -> Result<TextSegment, LayoutError> {
        if let Some((_, _, segment, contributions)) = self
            .entries
            .iter()
            .find(|(cached_text, cached_style, _, _)| cached_text == text && cached_style == style)
            && font_manager
                .validate_font_trace_events(contributions)
                .is_ok()
        {
            let ids = contributions
                .iter()
                .filter_map(|event| match event {
                    FontTraceEvent::Resolution(id) => Some(*id),
                    FontTraceEvent::Contribution(_) => None,
                })
                .collect::<Vec<_>>();
            font_manager.replay_layout_font_trace(&ids);
            return Ok(segment.clone());
        }
        if text.is_empty() {
            return shape_run(font_manager, text, style);
        }
        let Some(scope) = font_manager.begin_font_contribution_scope() else {
            return shape_run(font_manager, text, style);
        };
        let result = shape_run(font_manager, text, style);
        if let Ok(segment) = &result {
            font_manager.record_font_contribution(
                segment.font_id,
                typeface_for_text(style, &segment.text),
                &segment.text,
            )?;
        }
        let captured = font_manager.finish_font_contribution_scope(scope)?;
        let segment = result?;
        if let Some(contributions) = captured {
            let entry_bytes = shaping_cache_entry_bytes(text, style, &segment, &contributions);
            let vector_bytes =
                self.entries
                    .len()
                    .saturating_add(1)
                    .saturating_mul(std::mem::size_of::<(
                        String,
                        ResolvedRunStyle,
                        TextSegment,
                        Vec<FontTraceEvent>,
                    )>());
            if self.entries.len() < NATIVE_SHAPING_CACHE_MAX_ENTRIES
                && self
                    .bytes
                    .saturating_add(entry_bytes)
                    .saturating_add(vector_bytes)
                    <= NATIVE_SHAPING_CACHE_MAX_BYTES
            {
                self.entries.reserve_exact(1);
                let actual_vector_bytes =
                    self.entries.capacity().saturating_mul(std::mem::size_of::<(
                        String,
                        ResolvedRunStyle,
                        TextSegment,
                        Vec<FontTraceEvent>,
                    )>());
                if self
                    .bytes
                    .saturating_add(entry_bytes)
                    .saturating_add(actual_vector_bytes)
                    <= NATIVE_SHAPING_CACHE_MAX_BYTES
                {
                    self.entries.push((
                        text.to_owned(),
                        style.clone(),
                        segment.clone(),
                        contributions,
                    ));
                    self.bytes = self.bytes.saturating_add(entry_bytes);
                }
            }
        }
        Ok(segment)
    }
}

impl Default for NumberingState {
    fn default() -> Self {
        Self {
            levels: std::iter::repeat_with(|| None).take(9).collect(),
        }
    }
}

impl NumberingState {
    fn marker(
        &mut self,
        font_manager: &mut FontManager,
        shaping_cache: &mut ShapingCache,
        paragraph: &ResolvedParagraph,
    ) -> Result<Option<TextSegment>, LayoutError> {
        let level = usize::from(paragraph.level.min(8));
        self.levels[level + 1..].fill_with(|| None);

        let (text, font, color, size) = match paragraph.bullet.as_ref() {
            Some(ResolvedBullet::Character {
                character,
                font,
                color,
                size,
            }) => {
                self.levels[level] = None;
                (character.clone(), font, color, size)
            }
            Some(ResolvedBullet::AutoNumber {
                scheme,
                start_at,
                font,
                color,
                size,
            }) => {
                let value = self.levels[level]
                    .as_ref()
                    .filter(|sequence| sequence.scheme == *scheme)
                    .map_or(*start_at, |sequence| sequence.next);
                self.levels[level] = Some(AutoNumberSequence {
                    scheme: scheme.clone(),
                    next: value.saturating_add(1),
                });
                (format_auto_number(scheme, value), font, color, size)
            }
            None => {
                self.levels[level] = None;
                return Ok(None);
            }
        };

        let mut marker = shape_marker(
            font_manager,
            shaping_cache,
            paragraph,
            &text,
            font,
            *color,
            size,
        )?;
        if paragraph.indent < 0.0 {
            marker.width = -paragraph.indent;
        }
        Ok(Some(marker))
    }
}

#[cfg(test)]
pub(super) fn inline_items(
    font_manager: &mut FontManager,
    paragraph: &ResolvedParagraph,
) -> Result<Vec<InlineItem>, LayoutError> {
    let mut contexts = HashMap::new();
    let items = inline_items_cached(
        font_manager,
        &mut ShapingCache::default(),
        paragraph,
        1,
        &mut contexts,
    )?;
    for (index, item) in items.iter().enumerate() {
        if let InlineItem::Text(segment) = item {
            font_manager.record_font_contribution(
                segment.font_id,
                contexts.get(&index).and_then(|family| family.as_deref()),
                &segment.text,
            )?;
        }
    }
    Ok(items)
}

#[cfg(test)]
fn inline_items_cached(
    font_manager: &mut FontManager,
    shaping_cache: &mut ShapingCache,
    paragraph: &ResolvedParagraph,
    page_number: usize,
    contexts: &mut HashMap<usize, Option<String>>,
) -> Result<Vec<InlineItem>, LayoutError> {
    inline_items_with_caps(
        font_manager,
        shaping_cache,
        paragraph,
        page_number,
        &mut Vec::new(),
        contexts,
    )
}

fn inline_items_with_caps(
    font_manager: &mut FontManager,
    shaping_cache: &mut ShapingCache,
    paragraph: &ResolvedParagraph,
    page_number: usize,
    requests: &mut Vec<(bool, bool, Option<f64>, Option<String>)>,
    contexts: &mut HashMap<usize, Option<String>>,
) -> Result<Vec<InlineItem>, LayoutError> {
    let mut items = Vec::with_capacity(paragraph.runs.len());
    let mut requested_context_bytes = 0usize;
    let mut requested_context_entries = 0usize;
    for run in &paragraph.runs {
        let item_start = items.len();
        match run {
            ResolvedTextRun::Text { text, style } => {
                push_displayed_text(&mut items, text, style, |line| {
                    let shaped = shaping_cache.shape(font_manager, line, style)?;
                    requests.push((
                        style.small_caps,
                        style.all_caps,
                        style.spacing,
                        style.language.clone(),
                    ));
                    Ok(shaped)
                })?;
            }
            ResolvedTextRun::Field {
                text,
                field_type,
                style,
            } => {
                let slide_number = field_type
                    .as_deref()
                    .is_some_and(|field_type| field_type.eq_ignore_ascii_case("slidenum"));
                let rendered = if slide_number {
                    page_number.to_string()
                } else {
                    text.clone()
                };
                push_displayed_text(&mut items, &rendered, style, |line| {
                    let mut segment = shaping_cache.shape(font_manager, line, style)?;
                    requests.push((
                        style.small_caps,
                        style.all_caps,
                        style.spacing,
                        style.language.clone(),
                    ));
                    if slide_number {
                        segment.field_kind = Some(FieldKind::Page);
                    }
                    Ok(segment)
                })?;
            }
            ResolvedTextRun::Break => items.push(InlineItem::LineBreak),
        }
        let style = match run {
            ResolvedTextRun::Text { style, .. } | ResolvedTextRun::Field { style, .. } => {
                Some(style)
            }
            ResolvedTextRun::Break => None,
        };
        if let Some(style) = style {
            for (index, item) in items.iter().enumerate().skip(item_start) {
                if let InlineItem::Text(segment) = item {
                    let family = font_manager.copy_font_request_context(
                        typeface_for_text(style, &segment.text),
                        &mut requested_context_bytes,
                        &mut requested_context_entries,
                    );
                    contexts.insert(index, family);
                }
            }
        }
    }
    Ok(items)
}

/// Pushes run text as PowerPoint displays it when `a:t` holds a line
/// separator. Measured with PowerPoint for Mac, each line feed and each
/// carriage return breaks the line as an `a:br` does, so a CRLF pair leaves an
/// empty line, while U+0085, U+2028 and U+2029 show as a space. U+001C to
/// U+001E, which only reach here from memory since XML cannot carry them, also
/// show as a space. No Unicode paragraph separator reaches line layout, which
/// reorders each line within one bidi paragraph.
fn push_displayed_text(
    items: &mut Vec<InlineItem>,
    text: &str,
    style: &ResolvedRunStyle,
    mut shape: impl FnMut(&str) -> Result<TextSegment, LayoutError>,
) -> Result<(), LayoutError> {
    let spaced = |character: char| {
        matches!(
            character,
            '\u{1c}'..='\u{1e}' | '\u{85}' | '\u{2028}' | '\u{2029}'
        )
    };
    let displayed = if text.contains(spaced) {
        Cow::Owned(text.replace(spaced, " "))
    } else {
        Cow::Borrowed(text)
    };
    for (index, line) in displayed.split(['\n', '\r']).enumerate() {
        if index > 0 {
            items.push(InlineItem::LineBreak);
        }
        if !line.is_empty() {
            // Keep shaping context within each script. Common whitespace,
            // punctuation and combining marks stay with their preceding face.
            // Identical resolved slots retain the original single-run path.
            let mut start = 0;
            let mut family = None;
            for (index, character) in line.char_indices() {
                let common = character.is_whitespace()
                    || character.is_ascii_punctuation()
                    || matches!(character as u32, 0x0300..=0x036F);
                let mut encoded = [0; 4];
                let selected = if common && index > 0 {
                    family
                } else {
                    typeface_for_text(style, character.encode_utf8(&mut encoded))
                };
                if index > start && selected != family {
                    items.push(InlineItem::Text(shape(&line[start..index])?));
                    start = index;
                }
                family = selected;
            }
            items.push(InlineItem::Text(shape(&line[start..])?));
        }
    }
    Ok(())
}

fn shape_run(
    font_manager: &mut FontManager,
    text: &str,
    style: &ResolvedRunStyle,
) -> Result<TextSegment, LayoutError> {
    let rendered_text = if style.all_caps {
        text.to_uppercase()
    } else {
        text.to_owned()
    };
    let font_size = style.font_size.unwrap_or(DEFAULT_FONT_SIZE);
    let typeface = typeface_for_text(style, &rendered_text);
    let font_id =
        font_manager.resolve_font_for_text(typeface, style.bold, style.italic, &rendered_text)?;
    let metrics = font_manager.metrics(font_id, font_size)?;
    let mut shaped = font_manager.shape_text(font_id, &rendered_text, font_size)?;
    if let Some(spacing) = style.spacing {
        for advance in &mut shaped.advances {
            *advance += spacing;
        }
        shaped.width += spacing * shaped.advances.len() as f64;
    }

    Ok(TextSegment {
        text: text.to_owned(),
        direction: oxml_layout::TextDirection::Auto,
        source: None,
        font_id,
        font_size,
        glyph_ids: shaped.glyph_ids,
        advances: shaped.advances,
        width: shaped.width,
        ascent: metrics.ascent,
        descent: metrics.descent,
        line_gap: metrics.line_gap,
        color: text_color(style.fill.as_ref()),
        bold: style.bold,
        italic: style.italic,
        underline: style.underline.then_some(Underline::Single),
        strike: style.strike,
        dstrike: false,
        highlight: style.highlight,
        baseline_offset: style.baseline.unwrap_or(0.0) * font_size,
        hyperlink_url: style.hyperlink_url.clone(),
        field_kind: None,
        field_source: None,
        note: None,
        note_reference_source: None,
    })
}

fn shape_marker(
    font_manager: &mut FontManager,
    shaping_cache: &mut ShapingCache,
    paragraph: &ResolvedParagraph,
    text: &str,
    font: &Option<String>,
    color: Option<Color>,
    size: &Option<ResolvedBulletSize>,
) -> Result<TextSegment, LayoutError> {
    let mut style = first_run_style(paragraph).clone();
    style.small_caps = false;
    style.highlight = None;
    let base_size = style.font_size.unwrap_or(DEFAULT_FONT_SIZE);
    style.font_size = Some(match size {
        Some(ResolvedBulletSize::Percent(factor)) => base_size * factor,
        Some(ResolvedBulletSize::Points(points)) => *points,
        None => base_size,
    });
    if let Some(font) = font {
        style.latin_typeface = Some(font.clone());
        style.east_asian_typeface = Some(font.clone());
        style.complex_script_typeface = Some(font.clone());
        style.symbol_typeface = Some(font.clone());
    }
    if let Some(color) = color {
        style.fill = Some(Paint::Solid(color));
    }
    let (displayed, _) = font_manager.resolve_symbol_text(
        typeface_for_text(&style, text),
        style.bold,
        style.italic,
        text,
        true,
    )?;
    shaping_cache.shape(font_manager, &displayed, &style)
}

fn first_run_style(paragraph: &ResolvedParagraph) -> &ResolvedRunStyle {
    paragraph
        .runs
        .iter()
        .find_map(|run| match run {
            ResolvedTextRun::Text { style, .. } | ResolvedTextRun::Field { style, .. } => {
                Some(style)
            }
            ResolvedTextRun::Break => None,
        })
        .unwrap_or(&DEFAULT_RUN_STYLE)
}

static DEFAULT_RUN_STYLE: ResolvedRunStyle = ResolvedRunStyle {
    font_size: None,
    bold: false,
    italic: false,
    all_caps: false,
    small_caps: false,
    language: None,
    highlight: None,
    underline: false,
    strike: false,
    spacing: None,
    baseline: None,
    fill: None,
    latin_typeface: None,
    east_asian_typeface: None,
    complex_script_typeface: None,
    symbol_typeface: None,
    hyperlink_url: None,
};

fn format_auto_number(scheme: &str, value: u32) -> String {
    match scheme {
        "arabicPlain" => value.to_string(),
        "arabicParenR" => format!("{value})"),
        "arabicParenBoth" => format!("({value})"),
        "alphaLcPeriod" => format!("{}.", alphabetic_number(value, false)),
        "alphaUcPeriod" => format!("{}.", alphabetic_number(value, true)),
        "romanLcPeriod" => format!("{}.", roman_number(value, false)),
        "romanUcPeriod" => format!("{}.", roman_number(value, true)),
        "arabicPeriod" => format!("{value}."),
        _ => format!("{value}."),
    }
}

fn alphabetic_number(value: u32, uppercase: bool) -> String {
    let mut value = value.max(1);
    let mut letters = Vec::new();
    while value > 0 {
        value -= 1;
        let base = if uppercase { b'A' } else { b'a' };
        letters.push(char::from(base + (value % 26) as u8));
        value /= 26;
    }
    letters.into_iter().rev().collect()
}

fn roman_number(value: u32, uppercase: bool) -> String {
    let mut value = value.max(1);
    let mut result = String::new();
    for (amount, numeral) in [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ] {
        while value >= amount {
            result.push_str(numeral);
            value -= amount;
        }
    }
    if uppercase {
        result
    } else {
        result.to_ascii_lowercase()
    }
}

fn typeface_for_text<'a>(style: &'a ResolvedRunStyle, text: &str) -> Option<&'a str> {
    let preferred = if text.chars().any(is_symbol_character) {
        style.symbol_typeface.as_deref()
    } else if text.chars().any(is_east_asian_character) {
        style.east_asian_typeface.as_deref()
    } else if text.chars().any(is_complex_script_character) {
        style.complex_script_typeface.as_deref()
    } else {
        style.latin_typeface.as_deref()
    };
    preferred
        .or(style.latin_typeface.as_deref())
        .or(style.east_asian_typeface.as_deref())
        .or(style.complex_script_typeface.as_deref())
        .or(style.symbol_typeface.as_deref())
}

fn is_symbol_character(character: char) -> bool {
    matches!(character as u32, 0xE000..=0xF8FF)
}

fn is_east_asian_character(character: char) -> bool {
    matches!(
        character as u32,
        0x1100..=0x11FF
            | 0x2E80..=0xA4CF
            | 0xAC00..=0xD7AF
            | 0xF900..=0xFAFF
            | 0xFE30..=0xFE4F
            | 0xFF00..=0xFFEF
            | 0x20000..=0x323AF
    )
}

fn is_complex_script_character(character: char) -> bool {
    matches!(
        character as u32,
        0x0590..=0x10FF | 0x1780..=0x17FF | 0xFB1D..=0xFDFF | 0xFE70..=0xFEFF
    )
}

fn text_color(fill: Option<&Paint>) -> Color {
    match fill {
        Some(Paint::Solid(color)) => *color,
        Some(Paint::Linear { stops, .. }) | Some(Paint::Radial { stops, .. }) => {
            stops.first().map_or(Color::BLACK, |stop| stop.color)
        }
        Some(Paint::Grid { colors, .. }) => colors.first().copied().unwrap_or(Color::BLACK),
        Some(Paint::Tile { .. }) | None => Color::BLACK,
    }
}

pub(super) struct StackedText {
    pub(super) elements: Vec<PositionedElement>,
    /// One entry per laid-out line, anchored like `elements`.
    pub(super) lines: Vec<TextLineLayout>,
    line_ranges: Vec<(usize, usize)>,
    pub(super) width: f64,
    pub(super) height: f64,
    pub(super) font_scale: f64,
    pub(super) width_fits: bool,
    pub(super) diagnostics: Vec<String>,
}

impl StackedText {
    /// Applies the fit test that normal autofit uses to accept a candidate.
    pub(super) fn fits(&self, content: Rect) -> bool {
        self.width_fits && self.height <= content.height + 0.01
    }

    /// Explicit authoring measures unanchored physical participating segments.
    /// Ordinary rendering continues using summed line advances in `fits`.
    pub(super) fn authoring_extent(
        &self,
        fonts: &mut FontManager,
        text: &ResolvedTextBody,
        content: Rect,
    ) -> Result<(f64, bool), LayoutError> {
        let mut bottom = content.y;
        for (index, (line, &(start, end))) in self.lines.iter().zip(&self.line_ranges).enumerate() {
            let mut line_bottom = line.baseline;
            let mut physical = false;
            for element in &self.elements[start..end] {
                match element {
                    PositionedElement::Text(run) => {
                        physical = true;
                        line_bottom = line_bottom
                            .max(run.origin.y + fonts.metrics(run.font_id, run.font_size)?.descent);
                    }
                    PositionedElement::MultilingualText(run) => {
                        physical = true;
                        let descent = fonts.metrics(run.font_id, run.font_size)?.descent;
                        let mut pen_y = 0.0;
                        for (offset, advance) in run.y_offsets.iter().zip(&run.y_advances) {
                            line_bottom = line_bottom.max(run.origin.y - pen_y - offset + descent);
                            pen_y += advance;
                        }
                    }
                    _ => {}
                }
            }
            if !physical {
                let style = &text.paragraphs[line.paragraph_index].end_style;
                let id = fonts.resolve_font_for_metrics(
                    style.latin_typeface.as_deref(),
                    style.bold,
                    style.italic,
                )?;
                line_bottom = line.baseline + fonts.metrics(id, line.font_size)?.descent;
            }
            if index + 1 == self.lines.len() && text.space_first_last_paragraph {
                let paragraph = &text.paragraphs[line.paragraph_index];
                line_bottom += paragraph_spacing(
                    paragraph.space_after.as_ref(),
                    first_run_font_size(paragraph) * self.font_scale,
                );
            }
            bottom = bottom.max(line_bottom);
        }
        let extent = (bottom - content.y).max(0.0);
        if !extent.is_finite() {
            return Err(LayoutError::Layout(
                "autofit physical extent is non-finite".to_owned(),
            ));
        }
        Ok((extent, self.width_fits && extent <= content.height + 0.01))
    }
}

#[cfg(test)]
pub(super) fn stack_text(
    font_manager: &mut FontManager,
    content: Rect,
    text: &ResolvedTextBody,
) -> Result<StackedText, LayoutError> {
    stack_text_for_page(font_manager, content, text, 1)
}

#[cfg(test)]
pub(super) fn stack_text_for_page(
    font_manager: &mut FontManager,
    content: Rect,
    text: &ResolvedTextBody,
    page_number: usize,
) -> Result<StackedText, LayoutError> {
    stack_text_for_page_with_directions(font_manager, content, text, page_number, &[])
}

pub(super) fn stack_text_for_page_with_directions(
    font_manager: &mut FontManager,
    content: Rect,
    text: &ResolvedTextBody,
    page_number: usize,
    paragraph_directions: &[oxml_layout::TextDirection],
) -> Result<StackedText, LayoutError> {
    let mut diagnostics = Vec::new();
    let shaped_paragraphs = shape_paragraphs(
        font_manager,
        text,
        page_number,
        paragraph_directions,
        &mut diagnostics,
    )?;
    let mut stacked = (|| match text.autofit {
        ResolvedAutofit::None | ResolvedAutofit::Shape => {
            stack_shaped_text(font_manager, content, text, &shaped_paragraphs, 1.0, None)
        }
        ResolvedAutofit::Normal {
            font_scale: None,
            line_spacing_reduction: None,
        } => {
            let mut floor =
                stack_shaped_text(font_manager, content, text, &shaped_paragraphs, 1.0, None)?;
            if floor.fits(content) {
                return Ok(floor);
            }
            for step in 1..=30 {
                let scale = 1.0 - f64::from(step) * 0.025;
                let candidate = stack_shaped_text(
                    font_manager,
                    content,
                    text,
                    &shaped_paragraphs,
                    scale,
                    None,
                )?;
                if candidate.fits(content) {
                    return Ok(candidate);
                }
                floor = candidate;
            }
            Ok(floor)
        }
        ResolvedAutofit::Normal {
            font_scale,
            line_spacing_reduction,
        } => stack_shaped_text(
            font_manager,
            content,
            text,
            &shaped_paragraphs,
            font_scale.unwrap_or(1.0),
            line_spacing_reduction,
        ),
    })()?;
    stacked.diagnostics = diagnostics;
    Ok(stacked)
}

fn shape_paragraphs(
    font_manager: &mut FontManager,
    text: &ResolvedTextBody,
    page_number: usize,
    paragraph_directions: &[oxml_layout::TextDirection],
    diagnostics: &mut Vec<String>,
) -> Result<Vec<(Vec<InlineItem>, oxml_layout::TextDirection)>, LayoutError> {
    let mut numbering = NumberingState::default();
    let mut shaping_cache = ShapingCache::default();
    text.paragraphs
        .iter()
        .enumerate()
        .map(|(paragraph_index, paragraph)| {
            let base_direction = paragraph_directions
                .get(paragraph_index)
                .copied()
                .unwrap_or(oxml_layout::TextDirection::Auto);
            let marker = numbering.marker(font_manager, &mut shaping_cache, paragraph)?;
            let mut requests = Vec::new();
            let mut contexts = HashMap::new();
            let legacy_items = inline_items_with_caps(font_manager, &mut shaping_cache, paragraph, page_number, &mut requests, &mut contexts)?;
            let uses_rich_text = requests.iter().any(|(small, all, _, _)| *small || *all) || base_direction != oxml_layout::TextDirection::Auto
                || legacy_items.iter().any(|item| {
                    matches!(item, InlineItem::Text(segment) if needs_multilingual_layout(&segment.text))
                });
            let mut items = if uses_rich_text {
                shape_multilingual_items_preserving_controls(
                    font_manager,
                    legacy_items,
                    base_direction,
                    &requests,
                    diagnostics,
                    &contexts,
                )?
            } else {
                for (index, item) in legacy_items.iter().enumerate() {
                    if let InlineItem::Text(segment) = item {
                        font_manager.record_font_contribution(segment.font_id,
                            contexts.get(&index).and_then(|family| family.as_deref()), &segment.text)?;
                    }
                }
                legacy_items
            };
            let has_visible_text = items.iter().any(|item| match item {
                InlineItem::Text(segment) => !segment.text.is_empty(),
                InlineItem::MultilingualText(segment) => !segment.text().is_empty(),
                _ => false,
            });
            if items.is_empty() {
                items.push(InlineItem::Text(shaping_cache.shape(
                    font_manager,
                    "",
                    &paragraph.end_style,
                )?));
            }
            if let Some(marker) = marker.filter(|_| has_visible_text) {
                let explicit = match paragraph.bullet.as_ref() {
                    Some(ResolvedBullet::Character { font, .. })
                    | Some(ResolvedBullet::AutoNumber { font, .. }) => font.as_deref(),
                    None => None,
                };
                let requested = explicit.or_else(|| typeface_for_text(first_run_style(paragraph), &marker.text));
                font_manager.record_font_contribution(marker.font_id, requested, &marker.text)?;
                if let Some(ResolvedBullet::Character { character, .. }) = &paragraph.bullet {
                    append_symbol_marker_diagnostics(
                        font_manager,
                        requested,
                        character,
                        &marker,
                        diagnostics,
                    );
                }
                items.insert(0, InlineItem::Marker(marker));
            }
            Ok((items, base_direction))
        })
        .collect()
}

// Unknown semantic mappings are distinct from glyph coverage. A fallback face's
// private-use glyph does not authenticate the requested symbol's meaning.
fn append_symbol_marker_diagnostics(
    fonts: &FontManager,
    requested: Option<&str>,
    original: &str,
    marker: &TextSegment,
    diagnostics: &mut Vec<String>,
) {
    for message in
        fonts.unknown_symbol_diagnostics(marker.font_id, requested, original, &marker.text)
    {
        // At most 1024 canonical family/original-code pairs (512 per family).
        // Only accepted visible markers enter this local layout vector.
        if !diagnostics.contains(&message) {
            diagnostics.push(message);
        }
    }
}

fn shape_multilingual_items_preserving_controls(
    font_manager: &mut FontManager,
    legacy_items: Vec<InlineItem>,
    base_direction: oxml_layout::TextDirection,
    requests: &[(bool, bool, Option<f64>, Option<String>)],
    diagnostics: &mut Vec<String>,
    contexts: &HashMap<usize, Option<String>>,
) -> Result<Vec<InlineItem>, LayoutError> {
    let mut occurrence_languages = requests.iter();
    let styled = legacy_items
        .iter()
        .filter_map(|item| match item {
            InlineItem::Text(segment) => Some((
                segment.clone(),
                occurrence_languages
                    .next()
                    .and_then(|(_, _, _, language)| language.clone()),
            )),
            _ => None,
        })
        .collect();
    let (spans, fallback_diagnostics) = font_manager.shape_multilingual_paragraph_with_caps(
        styled,
        base_direction,
        false,
        &requests
            .iter()
            .map(|(small, all, _, _)| (*small, *all))
            .collect::<Vec<_>>(),
    )?;
    diagnostics.extend(fallback_diagnostics);
    let mut shaped = std::collections::VecDeque::from(spans);
    let mut items = Vec::new();
    let mut occurrence = 0usize;
    for (index, item) in legacy_items.into_iter().enumerate() {
        let requested = contexts.get(&index).and_then(|family| family.as_deref());
        let InlineItem::Text(segment) = item else {
            items.push(item);
            continue;
        };
        let spacing = requests
            .get(occurrence)
            .and_then(|(_, _, spacing, _)| *spacing);
        occurrence += 1;
        let target_bytes = segment.text.len();
        let mut consumed_bytes = 0usize;
        while consumed_bytes < target_bytes {
            let Some(mut span) = shaped.pop_front() else {
                return Err(LayoutError::Layout(
                    "multilingual paragraph shaping lost a styled text run".to_owned(),
                ));
            };
            consumed_bytes = consumed_bytes
                .checked_add(span.text().len())
                .filter(|consumed| *consumed <= target_bytes)
                .ok_or_else(|| {
                    LayoutError::Layout(
                        "multilingual paragraph shaping crossed a styled text boundary".to_owned(),
                    )
                })?;
            if let Some(spacing) = spacing {
                let mut base = span.base().clone();
                let advances = span
                    .x_advances()
                    .iter()
                    .map(|advance| advance + spacing)
                    .collect::<Vec<_>>();
                base.advances = advances.clone();
                base.width = advances.iter().sum();
                span = oxml_layout::MultilingualTextSegment::new(
                    base,
                    span.logical_index(),
                    span.language().map(str::to_owned),
                    span.script(),
                    span.direction(),
                    span.bidi_level(),
                    advances,
                    span.y_advances().to_vec(),
                    span.x_offsets().to_vec(),
                    span.y_offsets().to_vec(),
                    span.clusters().to_vec(),
                    span.break_after(),
                )?;
            }
            font_manager.record_font_contribution(span.base().font_id, requested, span.text())?;
            items.push(InlineItem::MultilingualText(span));
        }
    }
    if !shaped.is_empty() {
        return Err(LayoutError::Layout(
            "multilingual paragraph shaping produced an unmatched text run".to_owned(),
        ));
    }
    Ok(items)
}

fn needs_multilingual_layout(text: &str) -> bool {
    text.chars().any(|character| {
        matches!(
            character as u32,
            0x0590..=0x08ff
                | 0x0900..=0x097f
                | 0x0e00..=0x0e7f
                | 0x3000..=0x30ff
                | 0x3400..=0x9fff
                | 0xf900..=0xfaff
        )
    })
}

fn stack_shaped_text(
    font_manager: &FontManager,
    content: Rect,
    text: &ResolvedTextBody,
    shaped_paragraphs: &[(Vec<InlineItem>, oxml_layout::TextDirection)],
    font_scale: f64,
    line_spacing_reduction: Option<f64>,
) -> Result<StackedText, LayoutError> {
    let mut elements = Vec::new();
    let mut line_ranges = Vec::new();
    let mut laid_out_lines = Vec::new();
    let mut y = content.y;
    let mut width = 0.0_f64;
    let mut width_fits = true;

    let last_paragraph = text.paragraphs.len().saturating_sub(1);
    for (index, (paragraph, (shaped_items, base_direction))) in
        text.paragraphs.iter().zip(shaped_paragraphs).enumerate()
    {
        let font_size = first_run_font_size(paragraph) * font_scale;
        if index > 0 || text.space_first_last_paragraph {
            y += paragraph_spacing(paragraph.space_before.as_ref(), font_size);
        }
        let items = scaled_inline_items(shaped_items, font_scale, paragraph.indent);
        let params = line_break_params(
            paragraph,
            content.width,
            text.wrap,
            line_spacing_reduction.unwrap_or(0.0),
        );
        let lines = break_multilingual_into_lines(&items, &params, font_manager, *base_direction)?;

        for line in &lines {
            width_fits &= ink_width(line) <= line.available_width + 0.01;
            let line_size = line_font_size(line).unwrap_or(font_size);
            let baseline = y + baseline_offset(line, SINGLE_SPACING_EM * line_size);
            let element_start = elements.len();
            let (start_x, occupied_width) = emit_line_items(
                line,
                paragraph.alignment,
                content.x,
                baseline,
                &mut elements,
            );
            line_ranges.push((element_start, elements.len()));
            laid_out_lines.push(TextLineLayout {
                paragraph_index: index,
                text: line_text(line),
                bounds: Rect {
                    x: start_x,
                    y,
                    width: occupied_width,
                    height: line.height,
                },
                baseline,
                font_size: line_size,
            });
            width = width.max(occupied_width);
            y += line.height;
        }

        if index < last_paragraph || text.space_first_last_paragraph {
            y += paragraph_spacing(paragraph.space_after.as_ref(), font_size);
        }
    }

    let height = y - content.y;
    anchor_lines(
        &mut elements,
        &mut laid_out_lines,
        &line_ranges,
        text.anchor,
        content.height - height,
    );

    Ok(StackedText {
        elements,
        lines: laid_out_lines,
        line_ranges,
        width,
        height,
        font_scale,
        width_fits,
        diagnostics: Vec::new(),
    })
}

/// Returns a line's run and field text without its marker.
///
/// Rich spans are ordered by their logical index, as `emit_line_items` orders
/// them for extraction. A paragraph shapes either every run on the rich path or
/// none, so legacy runs never share a line with rich spans and keep line order.
fn line_text(line: &LayoutLine) -> String {
    let mut spans = line
        .items
        .iter()
        .enumerate()
        .filter_map(|(position, item)| match item {
            LineItem::Text(segment) => Some((position, segment.text.as_str())),
            LineItem::MultilingualText(segment) => Some((segment.logical_index(), segment.text())),
            _ => None,
        })
        .collect::<Vec<_>>();
    spans.sort_by_key(|(order, _)| *order);
    spans.into_iter().map(|(_, text)| text).collect()
}

/// Returns the widths of the spaces hanging off a line's visual left and
/// right, which alignment and the fit test leave out.
fn hanging_widths(line: &LayoutLine) -> (f64, f64) {
    let (start, end) = line.hanging_space_counts();
    let left = line.items[..start].iter().map(LineItem::width).sum();
    let right = line.items[line.items.len() - end..]
        .iter()
        .map(LineItem::width)
        .sum();
    (left, right)
}

/// Returns a line's width without the spaces hanging off either side.
fn ink_width(line: &LayoutLine) -> f64 {
    let (left, right) = hanging_widths(line);
    line.width - left - right
}

/// Returns the largest scaled run size on a line, ignoring its marker.
fn line_font_size(line: &LayoutLine) -> Option<f64> {
    line.items
        .iter()
        .filter_map(|item| match item {
            LineItem::Text(segment) => Some(segment.font_size),
            LineItem::MultilingualText(segment) => Some(segment.base().font_size),
            _ => None,
        })
        .reduce(f64::max)
}

fn scaled_inline_items(items: &[InlineItem], scale: f64, indent: f64) -> Vec<InlineItem> {
    items
        .iter()
        .cloned()
        .map(|item| match item {
            InlineItem::Text(mut segment) => {
                scale_segment(&mut segment, scale);
                InlineItem::Text(segment)
            }
            InlineItem::MultilingualText(segment) => {
                let mut base = segment.base().clone();
                scale_segment(&mut base, scale);
                InlineItem::MultilingualText(
                    oxml_layout::MultilingualTextSegment::new(
                        base,
                        segment.logical_index(),
                        segment.language().map(str::to_owned),
                        segment.script(),
                        segment.direction(),
                        segment.bidi_level(),
                        segment
                            .x_advances()
                            .iter()
                            .map(|value| value * scale)
                            .collect(),
                        segment
                            .y_advances()
                            .iter()
                            .map(|value| value * scale)
                            .collect(),
                        segment
                            .x_offsets()
                            .iter()
                            .map(|value| value * scale)
                            .collect(),
                        segment
                            .y_offsets()
                            .iter()
                            .map(|value| value * scale)
                            .collect(),
                        segment.clusters().to_vec(),
                        segment.break_after(),
                    )
                    .expect("scaling preserves multilingual segment invariants"),
                )
            }
            InlineItem::Marker(mut segment) => {
                scale_segment(&mut segment, scale);
                if indent < 0.0 {
                    segment.width = -indent;
                }
                InlineItem::Marker(segment)
            }
            other => other,
        })
        .collect()
}

fn scale_segment(segment: &mut TextSegment, scale: f64) {
    segment.font_size *= scale;
    segment.width *= scale;
    segment.ascent *= scale;
    segment.descent *= scale;
    segment.line_gap *= scale;
    segment.baseline_offset *= scale;
    for advance in &mut segment.advances {
        *advance *= scale;
    }
}

/// Distance from a line box's top to its baseline, where PowerPoint puts it.
///
/// A line taller than single spacing puts its baseline three quarters of the
/// way down, or at the font's ascent share of the line when that is smaller. A
/// line no taller than single spacing puts its descent at the bottom of the
/// line, so it gives up space above the glyphs first, unless that lifts the
/// baseline above the same share.
fn baseline_offset(line: &LayoutLine, single_height: f64) -> f64 {
    let glyph_height = line.ascent + line.descent;
    let share = if glyph_height > 0.0 {
        (line.ascent / glyph_height).min(0.75)
    } else {
        0.75
    };
    let proportional = line.height * share;
    if line.height > single_height + 0.01 {
        proportional
    } else {
        proportional.max(line.height - line.descent)
    }
}

fn anchor_lines(
    elements: &mut [PositionedElement],
    lines: &mut [TextLineLayout],
    line_ranges: &[(usize, usize)],
    anchor: TextAnchor,
    spare_height: f64,
) {
    let line_count = line_ranges.len();
    for (line_index, (&(start, end), line)) in line_ranges.iter().zip(lines).enumerate() {
        let offset = match anchor {
            TextAnchor::Top => 0.0,
            TextAnchor::Center => spare_height / 2.0,
            TextAnchor::Bottom => spare_height,
            TextAnchor::Justified if spare_height > 0.0 && line_count > 1 => {
                spare_height * line_index as f64 / (line_count - 1) as f64
            }
            TextAnchor::Distributed if spare_height > 0.0 && line_count > 1 => {
                spare_height * (line_index as f64 + 0.5) / line_count as f64
            }
            TextAnchor::Justified | TextAnchor::Distributed
                if spare_height > 0.0 && line_count == 1 =>
            {
                spare_height / 2.0
            }
            TextAnchor::Justified | TextAnchor::Distributed => 0.0,
        };
        for element in &mut elements[start..end] {
            translate_element_y(element, offset);
        }
        line.bounds.y += offset;
        line.baseline += offset;
    }
}

fn translate_element_y(element: &mut PositionedElement, offset: f64) {
    match element {
        PositionedElement::Text(run) => run.origin.y += offset,
        PositionedElement::MultilingualText(run) => run.origin.y += offset,
        PositionedElement::Line { start, end, .. } => {
            start.y += offset;
            end.y += offset;
        }
        PositionedElement::FilledRect { rect, .. }
        | PositionedElement::Image { rect, .. }
        | PositionedElement::LinkAnnotation { rect, .. } => rect.y += offset,
        PositionedElement::Group(group) => group.transform.f += offset,
        PositionedElement::Path(_) => {}
        _ => {}
    }
}

/// `line_spacing_reduction` is a stored `a:normAutofit/@lnSpcReduction`. It
/// comes off the percentage, and omitted spacing counts as 100 percent.
fn line_break_params(
    paragraph: &ResolvedParagraph,
    available_width: f64,
    wrap: bool,
    line_spacing_reduction: f64,
) -> LineBreakParams {
    let (ind_first_line, ind_hanging) = if paragraph.indent < 0.0 {
        (0.0, -paragraph.indent)
    } else {
        (paragraph.indent, 0.0)
    };
    LineBreakParams {
        // Slide text has nothing floating beside it to flow around.
        line_prefix_widths: Vec::new(),
        line_suffix_widths: Vec::new(),
        available_width,
        ind_left: paragraph.left_margin,
        ind_right: paragraph.right_margin,
        ind_first_line,
        ind_hanging,
        tab_stops: Vec::new(),
        line_spacing: match paragraph.line_spacing.as_ref() {
            Some(ResolvedTextSpacing::Percent(factor)) => LineSpacing::Multiple(
                SINGLE_SPACING_EM * (factor - line_spacing_reduction).max(0.0),
            ),
            Some(ResolvedTextSpacing::Points(points)) => LineSpacing::Exact(*points),
            None => {
                LineSpacing::Multiple(SINGLE_SPACING_EM * (1.0 - line_spacing_reduction).max(0.0))
            }
        },
        jc: Some(paragraph_align(paragraph.alignment)),
        wrap,
        // Slide text has no document-level tab interval, so it keeps the
        // half-inch default this field replaced.
        default_tab_interval_pt: 36.0,
        clamp_tabs_past_margin: false,
    }
}

fn paragraph_align(alignment: ParagraphAlignment) -> Align {
    match alignment {
        ParagraphAlignment::Left => Align::Start,
        ParagraphAlignment::Center => Align::Center,
        ParagraphAlignment::Right => Align::End,
        ParagraphAlignment::Justified => Align::Justify,
        ParagraphAlignment::Distributed => Align::Distribute,
    }
}

fn first_run_font_size(paragraph: &ResolvedParagraph) -> f64 {
    paragraph
        .runs
        .iter()
        .find_map(|run| match run {
            ResolvedTextRun::Text { style, .. } | ResolvedTextRun::Field { style, .. } => {
                Some(style.font_size.unwrap_or(DEFAULT_FONT_SIZE))
            }
            ResolvedTextRun::Break => None,
        })
        .unwrap_or_else(|| paragraph.end_style.font_size.unwrap_or(DEFAULT_FONT_SIZE))
}

/// A percentage is a share of single spacing, as for line spacing.
fn paragraph_spacing(spacing: Option<&ResolvedTextSpacing>, font_size: f64) -> f64 {
    match spacing {
        Some(ResolvedTextSpacing::Percent(factor)) => SINGLE_SPACING_EM * font_size * factor,
        Some(ResolvedTextSpacing::Points(points)) => *points,
        None => 0.0,
    }
}

/// Emits one line and returns its start x and the width its items occupy,
/// hanging spaces included.
fn emit_line_items(
    line: &LayoutLine,
    alignment: ParagraphAlignment,
    content_x: f64,
    baseline: f64,
    elements: &mut Vec<PositionedElement>,
) -> (f64, f64) {
    let element_start = elements.len();
    let (hanging_left, _) = hanging_widths(line);
    let (hang_start, hang_end) = line.hanging_space_counts();
    // Spaces hanging past the line end are not gaps to widen.
    let ink = hang_start..line.items.len() - hang_end;
    let remaining = line.available_width - ink_width(line);
    let distribute = match alignment {
        ParagraphAlignment::Justified if !line.is_last => {
            let gaps = word_gap_count(&line.items[ink.clone()]);
            (gaps > 0).then_some((remaining.max(0.0) / gaps as f64, false))
        }
        ParagraphAlignment::Distributed => {
            let gaps = glyph_gap_count(&line.items[ink.clone()]);
            (gaps > 0).then_some((remaining.max(0.0) / gaps as f64, true))
        }
        _ => None,
    };
    let alignment_offset = match alignment {
        ParagraphAlignment::Center => remaining / 2.0,
        ParagraphAlignment::Right => remaining,
        _ => 0.0,
    };
    // Spaces hanging off the visual left sit outside the aligned ink.
    let mut x = content_x + line.indent_left + alignment_offset - hanging_left;
    let start_x = x;
    let mut distributed_gaps = distribute
        .filter(|(_, every_glyph)| *every_glyph)
        .map_or(0, |_| glyph_gap_count(&line.items[ink.clone()]));

    for (index, item) in line.items.iter().enumerate() {
        match item {
            LineItem::Text(segment) => {
                let (advances, effective_width) = distributed_advances(
                    segment,
                    distribute.filter(|_| ink.contains(&index)),
                    &mut distributed_gaps,
                );
                emit_segment(
                    segment,
                    advances,
                    effective_width,
                    x,
                    baseline,
                    line.height,
                    elements,
                );
                x += effective_width;
            }
            LineItem::MultilingualText(segment) => {
                emit_multilingual_segment(segment, x, baseline, line.height, elements);
                x += segment.width();
            }
            LineItem::Marker(segment) => {
                emit_segment(
                    segment,
                    segment.advances.clone(),
                    segment.width,
                    x,
                    baseline,
                    line.height,
                    elements,
                );
                x += segment.width;
            }
            LineItem::Tab { width, leader, .. } => {
                if let Some(segment) = leader {
                    emit_segment(
                        segment,
                        segment.advances.clone(),
                        *width,
                        x,
                        baseline,
                        line.height,
                        elements,
                    );
                }
                x += width;
            }
            LineItem::Image { width, .. } => x += width,
            LineItem::Group { width, .. } => x += width,
            _ => x += item.width(),
        }
    }
    let rich_positions = (element_start..elements.len())
        .filter(|index| matches!(elements[*index], PositionedElement::MultilingualText(_)))
        .collect::<Vec<_>>();
    let mut logical_runs = rich_positions
        .iter()
        .filter_map(|index| match &elements[*index] {
            PositionedElement::MultilingualText(run) => Some(run.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    logical_runs.sort_by_key(|run| run.logical_index);
    for (position, run) in rich_positions.into_iter().zip(logical_runs) {
        elements[position] = PositionedElement::MultilingualText(run);
    }
    (start_x, x - start_x)
}

fn emit_segment(
    segment: &TextSegment,
    advances: Vec<f64>,
    effective_width: f64,
    x: f64,
    baseline: f64,
    line_height: f64,
    elements: &mut Vec<PositionedElement>,
) {
    if segment.text.is_empty() {
        return;
    }
    let adjusted_baseline = baseline - segment.baseline_offset;
    if let Some(color) = segment.highlight {
        elements.push(PositionedElement::FilledRect {
            rect: Rect {
                x,
                y: adjusted_baseline - segment.ascent,
                width: effective_width,
                height: line_height,
            },
            color,
        });
    }
    elements.push(PositionedElement::Text(GlyphRun {
        origin: Point {
            x,
            y: adjusted_baseline,
        },
        font_id: segment.font_id,
        font_size: segment.font_size,
        glyph_ids: segment.glyph_ids.clone(),
        advances,
        text: segment.text.clone(),
        source: None,
        color: segment.color,
        bold: segment.bold,
        italic: segment.italic,
        field_kind: segment.field_kind,
        field_source: segment.field_source,
        note: segment.note,
        note_reference_source: segment.note_reference_source,
        tab_aligned: None,
    }));

    if segment.underline.is_some() {
        let y = adjusted_baseline + segment.descent * 0.3;
        elements.push(PositionedElement::Line {
            start: Point { x, y },
            end: Point {
                x: x + effective_width,
                y,
            },
            width: segment.font_size / 18.0,
            color: segment.color,
            dash_pattern: None,
        });
    }
    if segment.strike {
        let y = adjusted_baseline - segment.ascent * 0.3;
        elements.push(PositionedElement::Line {
            start: Point { x, y },
            end: Point {
                x: x + effective_width,
                y,
            },
            width: segment.font_size / 24.0,
            color: segment.color,
            dash_pattern: None,
        });
    }
    if let Some(url) = &segment.hyperlink_url {
        elements.push(PositionedElement::LinkAnnotation {
            rect: Rect {
                x,
                y: adjusted_baseline - segment.ascent,
                width: effective_width,
                height: line_height,
            },
            url: url.clone(),
        });
    }
}

fn emit_multilingual_segment(
    segment: &oxml_layout::MultilingualTextSegment,
    x: f64,
    baseline: f64,
    line_height: f64,
    elements: &mut Vec<PositionedElement>,
) {
    let base = segment.base();
    if base.text.is_empty() {
        return;
    }
    let adjusted_baseline = baseline - base.baseline_offset;
    if let Some(color) = base.highlight {
        elements.push(PositionedElement::FilledRect {
            rect: Rect {
                x,
                y: adjusted_baseline - base.ascent,
                width: segment.width(),
                height: line_height,
            },
            color,
        });
    }
    elements.push(PositionedElement::MultilingualText(MultilingualGlyphRun {
        origin: Point {
            x,
            y: adjusted_baseline,
        },
        font_id: segment.font_id(),
        font_size: base.font_size,
        glyph_ids: segment.glyph_ids().to_vec(),
        x_advances: segment.x_advances().to_vec(),
        y_advances: segment.y_advances().to_vec(),
        x_offsets: segment.x_offsets().to_vec(),
        y_offsets: segment.y_offsets().to_vec(),
        clusters: segment.clusters().to_vec(),
        logical_text: segment.text().to_owned(),
        logical_index: segment.logical_index(),
        source: base.source,
        script: segment.script(),
        language: segment.language().map(str::to_owned),
        direction: segment.direction(),
        bidi_level: segment.bidi_level(),
        color: base.color,
        bold: base.bold,
        italic: base.italic,
        field_kind: base.field_kind,
        field_source: base.field_source,
        note: base.note,
        note_reference_source: base.note_reference_source,
    }));

    if base.underline.is_some() {
        let y = adjusted_baseline + base.descent * 0.3;
        elements.push(PositionedElement::Line {
            start: Point { x, y },
            end: Point {
                x: x + segment.width(),
                y,
            },
            width: base.font_size / 18.0,
            color: base.color,
            dash_pattern: None,
        });
    }
    if base.strike {
        let y = adjusted_baseline - base.ascent * 0.3;
        elements.push(PositionedElement::Line {
            start: Point { x, y },
            end: Point {
                x: x + segment.width(),
                y,
            },
            width: base.font_size / 24.0,
            color: base.color,
            dash_pattern: None,
        });
    }
    if let Some(url) = &base.hyperlink_url {
        elements.push(PositionedElement::LinkAnnotation {
            rect: Rect {
                x,
                y: adjusted_baseline - base.ascent,
                width: segment.width(),
                height: line_height,
            },
            url: url.clone(),
        });
    }
}

fn word_gap_count(items: &[LineItem]) -> usize {
    items
        .iter()
        .filter_map(text_segment)
        .map(|segment| {
            segment
                .text
                .chars()
                .filter(|character| character.is_whitespace())
                .count()
        })
        .sum()
}

fn glyph_gap_count(items: &[LineItem]) -> usize {
    items
        .iter()
        .filter_map(text_segment)
        .map(|segment| segment.advances.len())
        .sum::<usize>()
        .saturating_sub(1)
}

fn text_segment(item: &LineItem) -> Option<&TextSegment> {
    match item {
        LineItem::Text(segment) => Some(segment),
        LineItem::MultilingualText(segment) => Some(segment.base()),
        LineItem::Marker(_)
        | LineItem::Tab { .. }
        | LineItem::Image { .. }
        | LineItem::Group { .. } => None,
        _ => None,
    }
}

fn distributed_advances(
    segment: &TextSegment,
    distribution: Option<(f64, bool)>,
    distributed_gaps: &mut usize,
) -> (Vec<f64>, f64) {
    let Some((extra, every_glyph)) = distribution else {
        return (segment.advances.clone(), segment.width);
    };
    let mut advances = segment.advances.clone();
    let mut additions = 0_usize;
    if every_glyph {
        for advance in &mut advances {
            if *distributed_gaps == 0 {
                break;
            }
            *advance += extra;
            additions += 1;
            *distributed_gaps -= 1;
        }
    } else {
        let characters = segment.text.chars().collect::<Vec<_>>();
        if characters.len() == advances.len() {
            for (character, advance) in characters.into_iter().zip(&mut advances) {
                if character.is_whitespace() {
                    *advance += extra;
                    additions += 1;
                }
            }
        } else if let Some(last) = advances.last_mut() {
            let whitespace_count = characters
                .into_iter()
                .filter(|character| character.is_whitespace())
                .count();
            if whitespace_count > 0 {
                *last += extra * whitespace_count as f64;
                additions = whitespace_count;
            }
        }
    }
    (advances, segment.width + extra * additions as f64)
}

pub(super) fn content_box(shape: &ResolvedShape, text: &ResolvedTextBody) -> Rect {
    let boundary = text_boundary(shape);
    Rect {
        x: boundary.x + text.insets.left,
        y: boundary.y + text.insets.top,
        width: (boundary.width - text.insets.left - text.insets.right).max(0.0),
        height: (boundary.height - text.insets.top - text.insets.bottom).max(0.0),
    }
}

fn text_boundary(shape: &ResolvedShape) -> Rect {
    match &shape.geometry {
        rpptx_layout::ResolvedGeometry::Custom {
            text_rect: Some(rect),
            ..
        } => *rect,
        _ => Rect {
            x: 0.0,
            y: 0.0,
            width: shape.bounds.width,
            height: shape.bounds.height,
        },
    }
}

pub(super) fn oriented_content_box(
    content: Rect,
    direction: TextDirection,
) -> (Rect, Option<oxml_layout::Transform>) {
    let degrees = match direction {
        TextDirection::Horizontal => return (content, None),
        TextDirection::Vertical
        | TextDirection::EastAsianVertical
        | TextDirection::WordArtVertical => 90.0,
        TextDirection::Vertical270
        | TextDirection::MongolianVertical
        | TextDirection::WordArtVerticalRtl => -90.0,
    };
    let center_x = content.x + content.width / 2.0;
    let center_y = content.y + content.height / 2.0;
    (
        Rect {
            x: center_x - content.height / 2.0,
            y: center_y - content.width / 2.0,
            width: content.height,
            height: content.width,
        },
        Some(oxml_layout::Transform::rotate_about(
            degrees, center_x, center_y,
        )),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use oxml_layout::{Color, Paint, Transform};
    use rpptx_layout::{
        ParagraphAlignment, ResolvedAutofit, ResolvedBullet, ResolvedBulletSize, ResolvedContent,
        ResolvedGeometry, ResolvedRunStyle, ResolvedSlide, ResolvedTextRun, ResolvedTextSpacing,
        TextAnchor, TextDirection, TextInsets,
    };

    use crate::{RenderInput, RenderInputError, layout_presentation, layout_slide_with_fonts};

    fn style() -> ResolvedRunStyle {
        ResolvedRunStyle {
            font_size: Some(22.0),
            bold: true,
            italic: true,
            underline: true,
            strike: true,
            spacing: Some(1.5),
            baseline: Some(0.25),
            fill: Some(Paint::Solid(Color {
                r: 0.8,
                g: 0.1,
                b: 0.2,
                a: 1.0,
            })),
            latin_typeface: Some("Carlito".to_owned()),
            ..ResolvedRunStyle::default()
        }
    }

    fn shape(bounds: Rect, geometry: ResolvedGeometry) -> ResolvedShape {
        ResolvedShape {
            group_transform: Transform::IDENTITY,
            bounds,
            rotation_deg: 0.0,
            flip_h: false,
            flip_v: false,
            geometry,
            fill: None,
            image_fill: None,
            line: None,
            head_end: None,
            tail_end: None,
            shadow: None,
            content: ResolvedContent::None,
            unsupported: None,
        }
    }

    fn text_body(insets: TextInsets) -> ResolvedTextBody {
        ResolvedTextBody {
            insets,
            anchor: TextAnchor::Top,
            wrap: true,
            vertical: TextDirection::Horizontal,
            space_first_last_paragraph: false,
            autofit: ResolvedAutofit::None,
            paragraphs: Vec::new(),
        }
    }

    #[test]
    fn preset_text_rectangle_minus_unequal_insets_produces_the_computed_content_box() {
        let shape = shape(
            Rect {
                x: 30.0,
                y: 40.0,
                width: 120.0,
                height: 70.0,
            },
            ResolvedGeometry::Custom {
                paths: Vec::new(),
                text_rect: Some(Rect {
                    x: 10.0,
                    y: 15.0,
                    width: 80.0,
                    height: 40.0,
                }),
            },
        );
        let text = text_body(TextInsets {
            left: 3.0,
            top: 5.0,
            right: 7.0,
            bottom: 11.0,
        });

        assert_eq!(
            content_box(&shape, &text),
            Rect {
                x: 13.0,
                y: 20.0,
                width: 70.0,
                height: 24.0,
            }
        );
    }

    #[test]
    fn missing_text_rectangle_falls_back_to_local_shape_bounds() {
        let text = text_body(TextInsets {
            left: 4.0,
            top: 6.0,
            right: 8.0,
            bottom: 10.0,
        });

        for geometry in [
            ResolvedGeometry::Rectangle,
            ResolvedGeometry::BoundsFallback,
        ] {
            let shape = shape(
                Rect {
                    x: 30.0,
                    y: 40.0,
                    width: 120.0,
                    height: 70.0,
                },
                geometry,
            );
            assert_eq!(
                content_box(&shape, &text),
                Rect {
                    x: 4.0,
                    y: 6.0,
                    width: 108.0,
                    height: 54.0,
                }
            );
        }
    }

    #[test]
    fn insets_larger_than_the_text_rectangle_do_not_create_negative_extents() {
        let shape = shape(
            Rect {
                x: 0.0,
                y: 0.0,
                width: 30.0,
                height: 20.0,
            },
            ResolvedGeometry::Custom {
                paths: Vec::new(),
                text_rect: Some(Rect {
                    x: 1.0,
                    y: 2.0,
                    width: 10.0,
                    height: 8.0,
                }),
            },
        );
        let text = text_body(TextInsets {
            left: 8.0,
            top: 9.0,
            right: 7.0,
            bottom: 6.0,
        });

        let content = content_box(&shape, &text);
        assert_eq!(
            content,
            Rect {
                x: 9.0,
                y: 11.0,
                width: 0.0,
                height: 0.0,
            }
        );
        assert!(content.x.is_finite());
        assert!(content.y.is_finite());
        assert!(content.width.is_finite());
        assert!(content.height.is_finite());
    }

    #[test]
    fn vertical_text_uses_a_transposed_box_and_rotated_group() {
        let content = Rect {
            x: 10.0,
            y: 20.0,
            width: 80.0,
            height: 30.0,
        };

        let (transposed, transform) = oriented_content_box(content, TextDirection::Vertical);

        assert_eq!(
            transposed,
            Rect {
                x: 35.0,
                y: -5.0,
                width: 30.0,
                height: 80.0,
            }
        );
        let transform = transform.expect("vertical transform");
        assert_close(transform.a, 0.0);
        assert_close(transform.b, 1.0);
        assert_close(transform.c, -1.0);
        assert_close(transform.d, 0.0);
        assert_close(transform.e, 85.0);
        assert_close(transform.f, -15.0);

        let text = ResolvedTextBody {
            vertical: TextDirection::Vertical,
            paragraphs: vec![ResolvedParagraph {
                runs: vec![ResolvedTextRun::Text {
                    text: "Visible".to_owned(),
                    style: ResolvedRunStyle {
                        font_size: Some(12.0),
                        latin_typeface: Some("Carlito".to_owned()),
                        ..ResolvedRunStyle::default()
                    },
                }],
                ..ResolvedParagraph::default()
            }],
            ..text_body(TextInsets::default())
        };
        let mut text_shape = shape(
            Rect {
                x: 0.0,
                y: 0.0,
                width: 80.0,
                height: 30.0,
            },
            ResolvedGeometry::Rectangle,
        );
        text_shape.content = ResolvedContent::Text(text);
        let input = RenderInput {
            slides: vec![ResolvedSlide {
                unresolved_member_count: 0,
                size: (80.0, 30.0),
                background: None,
                shapes: vec![text_shape],
                diagnostics: Vec::new(),
            }],
            media: HashMap::new(),
            fonts: Vec::new(),
            metadata: None,
        };
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");

        let page = layout_slide_with_fonts(&input, 0, &mut fonts).expect("layout vertical text");
        let oxml_layout::PositionedElement::Group(shape_group) = &page.elements[0] else {
            panic!("shape should lower to a group");
        };
        let text_group = shape_group
            .children
            .iter()
            .find_map(|element| match element {
                oxml_layout::PositionedElement::Group(group) => Some(group),
                _ => None,
            })
            .expect("vertical text should lower to a nested group");
        assert_close(text_group.transform.b, 1.0);
        assert_close(text_group.transform.c, -1.0);
        assert_eq!(glyph_runs(&text_group.children)[0].text, "Visible");
    }

    #[test]
    fn vertical_270_uses_the_opposite_quarter_turn() {
        let content = Rect {
            x: 10.0,
            y: 20.0,
            width: 80.0,
            height: 30.0,
        };

        let (_, vertical) = oriented_content_box(content, TextDirection::Vertical);
        let (_, vertical_270) = oriented_content_box(content, TextDirection::Vertical270);

        let vertical = vertical.expect("vertical transform");
        let vertical_270 = vertical_270.expect("vertical-270 transform");
        assert_close(vertical.b, 1.0);
        assert_close(vertical.c, -1.0);
        assert_close(vertical_270.b, -1.0);
        assert_close(vertical_270.c, 1.0);
        assert_eq!(
            vertical.apply(Point { x: 50.0, y: 20.0 }),
            vertical_270.apply(Point { x: 50.0, y: 50.0 })
        );
    }

    #[test]
    fn unsupported_vertical_variants_use_the_documented_quarter_turns() {
        let content = Rect {
            x: 10.0,
            y: 20.0,
            width: 80.0,
            height: 30.0,
        };

        for direction in [
            TextDirection::EastAsianVertical,
            TextDirection::WordArtVertical,
        ] {
            let (_, transform) = oriented_content_box(content, direction);
            let transform = transform.expect("vertical fallback transform");
            assert_close(transform.b, 1.0);
            assert_close(transform.c, -1.0);
        }
        for direction in [
            TextDirection::MongolianVertical,
            TextDirection::WordArtVerticalRtl,
        ] {
            let (_, transform) = oriented_content_box(content, direction);
            let transform = transform.expect("vertical-270 fallback transform");
            assert_close(transform.b, -1.0);
            assert_close(transform.c, 1.0);
        }
    }

    #[test]
    fn resolved_runs_emit_glyph_items_with_concrete_style_and_break_boundaries() {
        let paragraph = ResolvedParagraph {
            runs: vec![
                ResolvedTextRun::Text {
                    text: "Alpha".to_owned(),
                    style: style(),
                },
                ResolvedTextRun::Break,
                ResolvedTextRun::Field {
                    text: "42".to_owned(),
                    field_type: Some("slidenum".to_owned()),
                    style: style(),
                },
            ],
            ..ResolvedParagraph::default()
        };
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");

        let items = inline_items(&mut fonts, &paragraph).expect("shape resolved runs");

        assert_eq!(items.len(), 3);
        let InlineItem::Text(first) = &items[0] else {
            panic!("text run should become a text item");
        };
        assert_eq!(first.text, "Alpha");
        assert_eq!(first.font_size, 22.0);
        assert_eq!(first.color.r, 0.8);
        assert!(first.bold);
        assert!(first.italic);
        assert!(first.underline.is_some());
        assert!(first.strike);
        assert_eq!(first.baseline_offset, 5.5);
        assert!(first.glyph_ids.iter().any(|glyph| *glyph != 0));
        assert!(matches!(items[1], InlineItem::LineBreak));
        let InlineItem::Text(field) = &items[2] else {
            panic!("field should become a text item");
        };
        assert_eq!(field.text, "1");
        assert_eq!(field.font_size, 22.0);
        assert_eq!(field.field_kind, Some(FieldKind::Page));
    }

    #[test]
    fn fx205_multilingual_spacing_reaches_text_and_field_advances() {
        for field in [false, true] {
            for text in ["العربية", "漢字", "Latin العربية 漢字"] {
                let mut widths = Vec::new();
                let mut images = Vec::new();
                for spacing in [None, Some(2.0)] {
                    let style = ResolvedRunStyle {
                        font_size: Some(18.0),
                        latin_typeface: Some("Carlito".into()),
                        spacing,
                        language: Some("ar-SA".into()),
                        ..Default::default()
                    };
                    let run = if field {
                        ResolvedTextRun::Field {
                            text: text.into(),
                            field_type: Some("datetime1".into()),
                            style,
                        }
                    } else {
                        ResolvedTextRun::Text {
                            text: text.into(),
                            style,
                        }
                    };
                    let body = ResolvedTextBody {
                        paragraphs: vec![ResolvedParagraph {
                            runs: vec![run],
                            ..Default::default()
                        }],
                        ..text_body(TextInsets::default())
                    };
                    let mut fonts = FontManager::new_deterministic().unwrap();
                    let stacked = stack_text(&mut fonts, test_content_box(2_000.0), &body).unwrap();
                    widths.push(stacked.width);
                    let layout = oxml_layout::LayoutResult::new(
                        vec![
                            oxml_layout::PageFrame::new(1, 2_000.0, 100.0, stacked.elements).into(),
                        ],
                        fonts.all_font_data(),
                        None,
                        Vec::new(),
                    );
                    images.push(oxml_pdf::render_page_to_png(&layout, 0, 72.0).unwrap());
                }
                assert!(
                    widths[1] > widths[0] + 2.0,
                    "FX205_MULTILINGUAL_SPACING: {text} field={field} {widths:?}"
                );
                assert_ne!(
                    images[0], images[1],
                    "spacing must change actual painted glyph positions"
                );
            }
        }
    }

    #[test]
    fn fx205_mixed_script_slots_keep_distinct_physical_faces_for_text_and_fields() {
        let style = ResolvedRunStyle {
            font_size: Some(18.0),
            latin_typeface: Some("Carlito".into()),
            east_asian_typeface: Some("Noto Sans SC".into()),
            complex_script_typeface: Some("Noto Sans Arabic".into()),
            ..Default::default()
        };
        let text = "Latin العربية 你好";
        for field in [false, true] {
            let mut fonts = FontManager::new_deterministic().unwrap();
            for (sample, family) in [
                ("Latin", "Carlito"),
                ("العربية", "Noto Sans Arabic"),
                ("你好", "Noto Sans SC"),
            ] {
                let shaped = shape_run(&mut fonts, sample, &style).unwrap();
                assert_eq!(fonts.font_data(shaped.font_id).unwrap().family, family);
                assert!(
                    shaped.glyph_ids.iter().all(|&g| g != 0),
                    "source face must cover deciding text"
                );
            }
            let run = if field {
                ResolvedTextRun::Field {
                    text: text.into(),
                    field_type: Some("datetime1".into()),
                    style: style.clone(),
                }
            } else {
                ResolvedTextRun::Text {
                    text: text.into(),
                    style: style.clone(),
                }
            };
            let paragraph = ResolvedParagraph {
                runs: vec![run],
                ..Default::default()
            };
            let items = inline_items(&mut fonts, &paragraph).unwrap();
            let segments = items
                .iter()
                .filter_map(|item| {
                    if let InlineItem::Text(s) = item {
                        Some(s)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            assert_eq!(
                segments.iter().map(|s| s.text.as_str()).collect::<String>(),
                text
            );
            for (sample, family) in [
                ("Latin", "Carlito"),
                ("العربية", "Noto Sans Arabic"),
                ("你好", "Noto Sans SC"),
            ] {
                let segment = segments
                    .iter()
                    .find(|s| s.text.contains(sample))
                    .expect("retained source text");
                assert_eq!(
                    fonts.font_data(segment.font_id).unwrap().family,
                    family,
                    "FX205_MIXED_SCRIPT_FACE field={field} sample={sample}"
                );
                assert!(segment.glyph_ids.iter().all(|&g| g != 0));
            }
            let body = ResolvedTextBody {
                paragraphs: vec![paragraph],
                ..text_body(TextInsets::default())
            };
            let stacked = stack_text(&mut fonts, test_content_box(2000.0), &body).unwrap();
            let mut painted = Vec::new();
            oxml_layout::walk(&stacked.elements, &mut |element, _| match element {
                oxml_layout::PositionedElement::Text(run) => {
                    painted.push((run.text.clone(), run.font_id, run.glyph_ids.clone()))
                }
                oxml_layout::PositionedElement::MultilingualText(run) => {
                    painted.push((run.logical_text.clone(), run.font_id, run.glyph_ids.clone()))
                }
                _ => {}
            });
            for (sample, family) in [
                ("Latin", "Carlito"),
                ("العربية", "Noto Sans Arabic"),
                ("你好", "Noto Sans SC"),
            ] {
                let selected = painted
                    .iter()
                    .filter(|(text, _, _)| {
                        text.chars()
                            .any(|c| sample.contains(c) && !c.is_whitespace())
                    })
                    .collect::<Vec<_>>();
                assert!(
                    !selected.is_empty(),
                    "deciding source glyphs must reach paint"
                );
                for (_, id, glyphs) in selected {
                    assert_eq!(fonts.font_data(*id).unwrap().family, family);
                    assert!(glyphs.iter().all(|&g| g != 0));
                }
            }
            let layout = oxml_layout::LayoutResult::new(
                vec![oxml_layout::PageFrame::new(1, 2000.0, 100.0, stacked.elements).into()],
                fonts.all_font_data(),
                None,
                Vec::new(),
            );
            let png = oxml_pdf::render_page_to_png(&layout, 0, 72.0).unwrap();
            assert!(png.len() > 100);
        }
    }

    #[test]
    fn script_specific_text_selects_its_resolved_concrete_typeface() {
        let style = ResolvedRunStyle {
            latin_typeface: Some("Latin face".to_owned()),
            east_asian_typeface: Some("East Asian face".to_owned()),
            complex_script_typeface: Some("Complex face".to_owned()),
            symbol_typeface: Some("Symbol face".to_owned()),
            ..ResolvedRunStyle::default()
        };

        assert_eq!(typeface_for_text(&style, "Latin"), Some("Latin face"));
        assert_eq!(typeface_for_text(&style, "漢字"), Some("East Asian face"));
        assert_eq!(typeface_for_text(&style, "مرحبا"), Some("Complex face"));
        assert_eq!(typeface_for_text(&style, "\u{F0B7}"), Some("Symbol face"));
    }

    #[test]
    fn missing_run_size_and_fill_use_visible_renderer_defaults() {
        let paragraph = ResolvedParagraph {
            runs: vec![ResolvedTextRun::Text {
                text: "Visible".to_owned(),
                style: ResolvedRunStyle::default(),
            }],
            ..ResolvedParagraph::default()
        };
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");

        let items = inline_items(&mut fonts, &paragraph).expect("shape default run");

        let InlineItem::Text(segment) = &items[0] else {
            panic!("text run should become a text item");
        };
        assert_eq!(segment.font_size, 18.0);
        assert_eq!(segment.color, Color::BLACK);
    }

    #[test]
    fn text_shaping_failures_return_a_render_error_without_panicking() {
        let text = ResolvedTextBody {
            insets: TextInsets::default(),
            anchor: TextAnchor::Top,
            wrap: true,
            vertical: TextDirection::Horizontal,
            space_first_last_paragraph: false,
            autofit: ResolvedAutofit::None,
            paragraphs: vec![ResolvedParagraph {
                runs: vec![ResolvedTextRun::Text {
                    text: "No font".to_owned(),
                    style: ResolvedRunStyle::default(),
                }],
                ..ResolvedParagraph::default()
            }],
        };
        let mut text_shape = shape(
            Rect {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 50.0,
            },
            ResolvedGeometry::Rectangle,
        );
        text_shape.content = ResolvedContent::Text(text);
        let input = RenderInput {
            slides: vec![ResolvedSlide {
                unresolved_member_count: 0,
                size: (100.0, 50.0),
                background: None,
                shapes: vec![text_shape],
                diagnostics: Vec::new(),
            }],
            media: HashMap::new(),
            fonts: Vec::new(),
            metadata: None,
        };
        let mut fonts = FontManager::new_with_fonts(Vec::new());

        let error =
            layout_slide_with_fonts(&input, 0, &mut fonts).expect_err("missing font should fail");

        assert!(matches!(error, RenderInputError::TextLayout { .. }));
        let error = crate::layout_presentation_with_font_manager(
            &input,
            FontManager::new_with_fonts(Vec::new()),
        )
        .expect_err("missing font should fail");
        let RenderInputError::TextLayout { detail } = error else {
            panic!("expected a text layout error, found {error:?}");
        };
        assert!(
            detail.starts_with("slide 1, shape 1 in draw order: "),
            "{detail}"
        );
    }

    #[test]
    fn line_separators_inside_run_text_display_as_breaks_or_spaces() {
        let paragraph = ResolvedParagraph {
            runs: vec![
                ResolvedTextRun::Text {
                    text: "a\nb\rc\r\nd\u{85}e\u{2028}f\u{2029}g\u{1e}h".to_owned(),
                    style: style(),
                },
                ResolvedTextRun::Field {
                    text: "\nfield".to_owned(),
                    field_type: Some("datetime".to_owned()),
                    style: style(),
                },
            ],
            ..ResolvedParagraph::default()
        };
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");

        let items = inline_items(&mut fonts, &paragraph).expect("shape resolved runs");

        let described = items
            .iter()
            .map(|item| match item {
                InlineItem::Text(segment) => segment.text.clone(),
                InlineItem::LineBreak => "|".to_owned(),
                _ => panic!("unexpected inline item"),
            })
            .collect::<Vec<_>>();
        assert_eq!(
            described,
            ["a", "|", "b", "|", "c", "|", "|", "d e f g h", "|", "field"]
        );

        // Each separator used to split the bidi paragraph and fail the slide.
        let mut text_shape = shape(
            Rect {
                x: 0.0,
                y: 0.0,
                width: 200.0,
                height: 200.0,
            },
            ResolvedGeometry::Rectangle,
        );
        text_shape.content = ResolvedContent::Text(ResolvedTextBody {
            insets: TextInsets::default(),
            anchor: TextAnchor::Top,
            wrap: true,
            vertical: TextDirection::Horizontal,
            space_first_last_paragraph: false,
            autofit: ResolvedAutofit::None,
            paragraphs: vec![paragraph],
        });
        let input = RenderInput {
            slides: vec![ResolvedSlide {
                unresolved_member_count: 0,
                size: (200.0, 200.0),
                background: None,
                shapes: vec![text_shape],
                diagnostics: Vec::new(),
            }],
            media: HashMap::new(),
            fonts: Vec::new(),
            metadata: None,
        };
        layout_slide_with_fonts(&input, 0, &mut fonts).expect("separators lay out");
    }

    #[test]
    fn presentation_layout_collects_every_font_used_by_shape_text() {
        let text = ResolvedTextBody {
            insets: TextInsets::default(),
            anchor: TextAnchor::Top,
            wrap: true,
            vertical: TextDirection::Horizontal,
            space_first_last_paragraph: false,
            autofit: ResolvedAutofit::None,
            paragraphs: vec![ResolvedParagraph {
                runs: vec![
                    ResolvedTextRun::Text {
                        text: "Regular".to_owned(),
                        style: ResolvedRunStyle {
                            latin_typeface: Some("Carlito".to_owned()),
                            ..ResolvedRunStyle::default()
                        },
                    },
                    ResolvedTextRun::Text {
                        text: "Bold".to_owned(),
                        style: ResolvedRunStyle {
                            bold: true,
                            latin_typeface: Some("Carlito".to_owned()),
                            ..ResolvedRunStyle::default()
                        },
                    },
                ],
                ..ResolvedParagraph::default()
            }],
        };
        let mut text_shape = shape(
            Rect {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 50.0,
            },
            ResolvedGeometry::Rectangle,
        );
        text_shape.content = ResolvedContent::Text(text);
        let input = RenderInput {
            slides: vec![ResolvedSlide {
                unresolved_member_count: 0,
                size: (100.0, 50.0),
                background: None,
                shapes: vec![text_shape],
                diagnostics: Vec::new(),
            }],
            media: HashMap::new(),
            fonts: Vec::new(),
            metadata: None,
        };

        let layout = layout_presentation(&input).expect("layout presentation text");

        assert_eq!(layout.fonts.len(), 2);
        assert!(layout.fonts.iter().any(|font| !font.bold));
        assert!(layout.fonts.iter().any(|font| font.bold));
    }

    #[test]
    fn paragraphs_stack_wrapped_lines_with_spacing_and_alignment() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let body = ResolvedTextBody {
            wrap: true,
            paragraphs: vec![
                ResolvedParagraph {
                    alignment: ParagraphAlignment::Center,
                    line_spacing: Some(ResolvedTextSpacing::Points(20.0)),
                    space_after: Some(ResolvedTextSpacing::Points(5.0)),
                    runs: vec![ResolvedTextRun::Text {
                        text: "one two".to_owned(),
                        style: ResolvedRunStyle {
                            font_size: Some(12.0),
                            latin_typeface: Some("Carlito".to_owned()),
                            ..ResolvedRunStyle::default()
                        },
                    }],
                    ..ResolvedParagraph::default()
                },
                ResolvedParagraph {
                    space_before: Some(ResolvedTextSpacing::Points(3.0)),
                    line_spacing: Some(ResolvedTextSpacing::Points(20.0)),
                    runs: vec![ResolvedTextRun::Text {
                        text: "three".to_owned(),
                        style: ResolvedRunStyle {
                            font_size: Some(12.0),
                            latin_typeface: Some("Carlito".to_owned()),
                            ..ResolvedRunStyle::default()
                        },
                    }],
                    ..ResolvedParagraph::default()
                },
            ],
            ..text_body(TextInsets::default())
        };
        let content = Rect {
            x: 10.0,
            y: 7.0,
            width: 35.0,
            height: 20.0,
        };

        let stacked = stack_text(&mut fonts, content, &body).expect("stack text");
        // The space hanging past the first line is a run of its own.
        let runs = glyph_runs(&stacked.elements)
            .into_iter()
            .filter(|run| !run.text.trim().is_empty())
            .collect::<Vec<_>>();

        assert_eq!(runs.len(), 3);
        assert_close(runs[1].origin.y - runs[0].origin.y, 20.0);
        assert_close(runs[2].origin.y - runs[1].origin.y, 28.0);
        assert!(runs[0].origin.x > content.x);
        assert!(runs[1].origin.x > content.x);
        assert_close(stacked.height, 68.0);
        assert!(stacked.width <= content.width);
    }

    #[test]
    fn wrap_none_breaks_only_at_explicit_line_breaks() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let body = ResolvedTextBody {
            wrap: false,
            paragraphs: vec![ResolvedParagraph {
                line_spacing: Some(ResolvedTextSpacing::Points(18.0)),
                runs: vec![
                    ResolvedTextRun::Text {
                        text: "alpha beta gamma".to_owned(),
                        style: ResolvedRunStyle {
                            font_size: Some(12.0),
                            latin_typeface: Some("Carlito".to_owned()),
                            ..ResolvedRunStyle::default()
                        },
                    },
                    ResolvedTextRun::Break,
                    ResolvedTextRun::Text {
                        text: "delta".to_owned(),
                        style: ResolvedRunStyle {
                            font_size: Some(12.0),
                            latin_typeface: Some("Carlito".to_owned()),
                            ..ResolvedRunStyle::default()
                        },
                    },
                ],
                ..ResolvedParagraph::default()
            }],
            ..text_body(TextInsets::default())
        };

        let stacked = stack_text(
            &mut fonts,
            Rect {
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 10.0,
            },
            &body,
        )
        .expect("stack unwrapped text");
        let runs = glyph_runs(&stacked.elements);

        let baselines = distinct_baselines(&runs);
        assert_eq!(baselines.len(), 2);
        assert_close(baselines[1] - baselines[0], 18.0);
        assert!(stacked.width > 10.0);
    }

    #[test]
    fn point_and_percentage_paragraph_spacing_produce_computed_baselines() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let run = || ResolvedTextRun::Text {
            text: "line".to_owned(),
            style: ResolvedRunStyle {
                font_size: Some(20.0),
                latin_typeface: Some("Carlito".to_owned()),
                ..ResolvedRunStyle::default()
            },
        };
        let body = ResolvedTextBody {
            paragraphs: vec![
                ResolvedParagraph {
                    line_spacing: Some(ResolvedTextSpacing::Points(24.0)),
                    space_after: Some(ResolvedTextSpacing::Percent(0.5)),
                    runs: vec![run()],
                    ..ResolvedParagraph::default()
                },
                ResolvedParagraph {
                    line_spacing: Some(ResolvedTextSpacing::Points(24.0)),
                    space_before: Some(ResolvedTextSpacing::Points(3.0)),
                    runs: vec![run()],
                    ..ResolvedParagraph::default()
                },
            ],
            ..text_body(TextInsets::default())
        };

        let stacked = stack_text(
            &mut fonts,
            Rect {
                x: 0.0,
                y: 2.0,
                width: 100.0,
                height: 10.0,
            },
            &body,
        )
        .expect("stack spaced paragraphs");
        let runs = glyph_runs(&stacked.elements);

        // Half of single spacing at 20 points is 12 points. PowerPoint lays
        // spcBef or spcAft of 50, 100 and 200 % at 12, 24 and 48 points for
        // 20 pt Arial.
        assert_eq!(runs.len(), 2);
        assert_close(runs[1].origin.y - runs[0].origin.y, 24.0 + 12.0 + 3.0);
        assert_close(stacked.height, 24.0 + 12.0 + 3.0 + 24.0);
    }

    #[test]
    fn text_and_marker_items_share_one_baseline_emitter() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let segment = shape_run(
            &mut fonts,
            "x",
            &ResolvedRunStyle {
                font_size: Some(12.0),
                latin_typeface: Some("Carlito".to_owned()),
                ..ResolvedRunStyle::default()
            },
        )
        .expect("shape segment");
        let mut decorated = segment.clone();
        decorated.underline = Some(Underline::Single);
        decorated.strike = true;
        let line = oxml_layout::LayoutLine {
            items: vec![
                oxml_layout::LineItem::Marker(decorated.clone()),
                oxml_layout::LineItem::Text(decorated.clone()),
            ],
            width: decorated.width * 2.0,
            ascent: decorated.ascent,
            descent: decorated.descent,
            line_gap: decorated.line_gap,
            height: decorated.ascent + decorated.descent + decorated.line_gap,
            indent_left: 0.0,
            available_width: 100.0,
            is_last: true,
            forced_break_after: None,
        };
        let mut elements = Vec::new();

        emit_line_items(
            &line,
            ParagraphAlignment::Left,
            4.0,
            9.0 + line.ascent,
            &mut elements,
        );
        let runs = glyph_runs(&elements);

        assert_eq!(runs.len(), 2);
        assert_close(runs[0].origin.y, runs[1].origin.y);
        assert_close(runs[1].origin.x - runs[0].origin.x, decorated.width);
        assert_eq!(
            elements
                .iter()
                .filter(|element| matches!(element, PositionedElement::Line { .. }))
                .count(),
            4
        );
    }

    #[test]
    fn resolved_paragraph_fields_map_to_shared_line_break_parameters() {
        let paragraph = ResolvedParagraph {
            left_margin: 12.0,
            right_margin: 7.0,
            indent: -5.0,
            alignment: ParagraphAlignment::Right,
            line_spacing: Some(ResolvedTextSpacing::Percent(1.25)),
            ..ResolvedParagraph::default()
        };

        let params = line_break_params(&paragraph, 80.0, false, 0.0);

        assert_eq!(params.available_width, 80.0);
        assert_eq!(params.ind_left, 12.0);
        assert_eq!(params.ind_right, 7.0);
        assert_eq!(params.ind_first_line, 0.0);
        assert_eq!(params.ind_hanging, 5.0);
        assert_eq!(
            params.line_spacing,
            LineSpacing::Multiple(SINGLE_SPACING_EM * 1.25)
        );
        assert_eq!(params.jc, Some(Align::End));
        assert!(!params.wrap);
    }

    #[test]
    fn percentage_line_spacing_uses_effective_first_run_font_size() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let body = ResolvedTextBody {
            paragraphs: vec![ResolvedParagraph {
                line_spacing: Some(ResolvedTextSpacing::Percent(1.2)),
                runs: vec![ResolvedTextRun::Text {
                    text: "line".to_owned(),
                    style: ResolvedRunStyle {
                        font_size: Some(20.0),
                        latin_typeface: Some("Carlito".to_owned()),
                        ..ResolvedRunStyle::default()
                    },
                }],
                ..ResolvedParagraph::default()
            }],
            ..text_body(TextInsets::default())
        };
        let stacked = stack_text(
            &mut fonts,
            Rect {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 10.0,
            },
            &body,
        )
        .expect("stack percentage-spaced line");

        assert_close(stacked.height, 1.2 * SINGLE_SPACING_EM * 20.0);
    }

    /// Lays out `lines` paragraphs of `runs`, each a typeface and a size, and
    /// returns the first baseline under the content top and the average pitch.
    fn pitch_and_first_baseline(
        fonts: &mut FontManager,
        runs: &[(&str, f64)],
        spacing: Option<ResolvedTextSpacing>,
        lines: usize,
    ) -> (f64, f64) {
        let paragraph = ResolvedParagraph {
            line_spacing: spacing,
            runs: runs
                .iter()
                .map(|(typeface, size)| ResolvedTextRun::Text {
                    text: "Hxgj ".to_owned(),
                    style: ResolvedRunStyle {
                        font_size: Some(*size),
                        latin_typeface: Some((*typeface).to_owned()),
                        ..ResolvedRunStyle::default()
                    },
                })
                .collect(),
            ..ResolvedParagraph::default()
        };
        let body = ResolvedTextBody {
            paragraphs: vec![paragraph; lines],
            ..text_body(TextInsets::default())
        };
        let stacked = stack_text(fonts, test_content_box(400.0), &body).expect("stack lines");
        let first = stacked.lines[0].baseline;
        let last = stacked.lines[lines - 1].baseline;
        (first, (last - first) / (lines - 1) as f64)
    }

    #[test]
    fn line_pitch_and_first_baseline_match_powerpoint() {
        use ResolvedTextSpacing::{Percent, Points};
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");

        // PowerPoint for Mac, average pitch over 24 lines of 14 points: single
        // spacing is 1.2 em for every font, percentages multiply it, and a
        // line takes the pitch of its largest run.
        let pitches = [
            ("Arial", 28.0, None, 33.64),
            ("Arial", 14.0, None, 16.79),
            ("Arial", 14.0, Some(Percent(1.0)), 16.79),
            ("Arial", 14.0, Some(Percent(1.5)), 25.21),
            ("Arial", 14.0, Some(Percent(0.8)), 13.43),
            ("Arial", 14.0, Some(Points(20.0)), 20.0),
            ("Calibri", 14.0, Some(Percent(1.0)), 16.83),
            ("Times New Roman", 14.0, Some(Percent(1.0)), 16.79),
            ("Courier New", 14.0, Some(Percent(1.0)), 16.82),
        ];
        for (typeface, size, spacing, expected) in pitches {
            let runs = [(typeface, 14.0), (typeface, size)];
            let (_, pitch) = pitch_and_first_baseline(&mut fonts, &runs, spacing.clone(), 24);
            assert!(
                (pitch - expected).abs() < 0.06,
                "{typeface} {size} {spacing:?}: pitch {pitch}, PowerPoint {expected}"
            );
        }

        // First baseline under the top inset at 30 points, averaged over eight
        // boxes. PowerPoint puts baselines on whole points.
        let firsts = [
            ("Arial", Some(Percent(1.0)), 29.0),
            ("Arial", Some(Percent(1.5)), 41.0),
            ("Arial", Some(Percent(0.8)), 22.0),
            ("Arial", Some(Points(50.0)), 38.0),
            ("Calibri", None, 28.0),
            ("Calibri", Some(Percent(1.5)), 41.0),
            ("Calibri", Some(Percent(0.8)), 22.0),
            ("Times New Roman", Some(Percent(1.0)), 29.0),
            ("Times New Roman", Some(Points(50.0)), 38.0),
            ("Courier New", Some(Percent(1.0)), 26.0),
            ("Courier New", Some(Percent(1.5)), 40.0),
            ("Courier New", Some(Percent(0.8)), 21.0),
        ];
        for (typeface, spacing, expected) in firsts {
            let (first, _) =
                pitch_and_first_baseline(&mut fonts, &[(typeface, 30.0)], spacing.clone(), 2);
            assert!(
                (first - expected).abs() <= 1.0,
                "{typeface} {spacing:?}: first baseline {first}, PowerPoint {expected}"
            );
        }
    }

    #[test]
    fn a_space_hanging_past_the_frame_does_not_shrink_normal_autofit_text() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let mut body = autofit_body(
            ResolvedAutofit::Normal {
                font_scale: None,
                line_spacing_reduction: None,
            },
            20.0,
            "still in service, or later",
        );
        body.wrap = true;
        let style = first_run_style(&body.paragraphs[0]).clone();
        let ink = shape_run(&mut fonts, "still in service,", &style)
            .expect("shape first line")
            .width;
        let content = Rect {
            x: 0.0,
            y: 0.0,
            width: ink + 1.0,
            height: 400.0,
        };

        // python-pptx text styles set the paragraph direction, which takes the
        // rich line path.
        let stacked = stack_text_for_page_with_directions(
            &mut fonts,
            content,
            &body,
            1,
            &[oxml_layout::TextDirection::LeftToRight],
        )
        .expect("stack directed paragraph");

        assert_eq!(stacked.font_scale, 1.0);
        assert_eq!(
            stacked
                .lines
                .iter()
                .map(|line| line.text.as_str())
                .collect::<Vec<_>>(),
            ["still in service, ", "or later"]
        );
    }

    #[test]
    fn spaces_hang_off_the_visual_end_of_directed_lines() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let cases = [
            (
                "שלום עולם זה טקסט ארוך בעברית שנשבר לכמה שורות בתוך התיבה",
                ParagraphAlignment::Left,
                oxml_layout::TextDirection::RightToLeft,
            ),
            (
                "Repainted in 3 weeks while still in service, or re-coated next spring",
                ParagraphAlignment::Right,
                oxml_layout::TextDirection::LeftToRight,
            ),
        ];
        for (text, alignment, direction) in cases {
            for width in (100..240).step_by(2) {
                let body = ResolvedTextBody {
                    paragraphs: vec![ResolvedParagraph {
                        alignment,
                        runs: vec![ResolvedTextRun::Text {
                            text: text.to_owned(),
                            style: ResolvedRunStyle {
                                font_size: Some(14.0),
                                latin_typeface: Some("Arial".to_owned()),
                                ..ResolvedRunStyle::default()
                            },
                        }],
                        ..ResolvedParagraph::default()
                    }],
                    ..text_body(TextInsets::default())
                };
                let content = test_content_box(f64::from(width));
                let stacked = stack_text_for_page_with_directions(
                    &mut fonts,
                    content,
                    &body,
                    1,
                    &[direction],
                )
                .expect("stack directed paragraph");
                let ink = stacked
                    .elements
                    .iter()
                    .filter_map(|element| match element {
                        PositionedElement::MultilingualText(run)
                            if run.logical_text.trim() != "" =>
                        {
                            Some((
                                run.origin.y,
                                run.origin.x,
                                run.origin.x + run.x_advances.iter().sum::<f64>(),
                            ))
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                let last_baseline = ink.iter().map(|(y, _, _)| *y).fold(f64::MIN, f64::max);
                for (baseline, left, right) in &ink {
                    assert!(
                        *left >= -0.01 && *right <= content.width + 0.01,
                        "{width}: ink {left}..{right} leaves the frame"
                    );
                    // Right-aligned wrapped lines end flush, as in PowerPoint.
                    if alignment == ParagraphAlignment::Right && *baseline < last_baseline {
                        let line_right = ink
                            .iter()
                            .filter(|(y, _, _)| y == baseline)
                            .map(|(_, _, right)| *right)
                            .fold(f64::MIN, f64::max);
                        assert!(
                            (line_right - content.width).abs() < 0.01,
                            "{width}: {line_right}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn plain_aligned_lines_end_flush_past_their_spaces() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let text = "Repainted in 3 weeks while still in service, or re-coated next spring";
        for alignment in [
            ParagraphAlignment::Right,
            ParagraphAlignment::Justified,
            ParagraphAlignment::Distributed,
        ] {
            for width in (100..240).step_by(2) {
                let body = ResolvedTextBody {
                    paragraphs: vec![ResolvedParagraph {
                        alignment,
                        runs: vec![ResolvedTextRun::Text {
                            text: text.to_owned(),
                            style: ResolvedRunStyle {
                                font_size: Some(14.0),
                                latin_typeface: Some("Arial".to_owned()),
                                ..ResolvedRunStyle::default()
                            },
                        }],
                        ..ResolvedParagraph::default()
                    }],
                    ..text_body(TextInsets::default())
                };
                let content = test_content_box(f64::from(width));
                // No direction keeps the paragraph on the plain path.
                let stacked = stack_text_for_page_with_directions(
                    &mut fonts,
                    content,
                    &body,
                    1,
                    &[oxml_layout::TextDirection::Auto],
                )
                .expect("stack plain paragraph");
                let mut line_ends = Vec::<(f64, f64)>::new();
                for element in &stacked.elements {
                    let PositionedElement::Text(run) = element else {
                        panic!("a plain paragraph paints plain runs");
                    };
                    if run.text.trim().is_empty() {
                        continue;
                    }
                    let right = run.origin.x + run.advances.iter().sum::<f64>();
                    match line_ends.last_mut() {
                        Some((baseline, end)) if *baseline == run.origin.y => *end = right,
                        _ => line_ends.push((run.origin.y, right)),
                    }
                }
                assert!(line_ends.len() > 1, "{width}: the text wraps");
                // The space that ends each wrapped line hangs past the edge,
                // as in PowerPoint, so the ink ends flush with it, and it is
                // not widened.
                for element in &stacked.elements {
                    if let PositionedElement::Text(run) = element
                        && run.text.trim().is_empty()
                    {
                        let natural = fonts
                            .shape_text(run.font_id, &run.text, run.font_size)
                            .expect("shape the hanging space")
                            .width;
                        let painted = run.advances.iter().sum::<f64>();
                        assert!(
                            (painted - natural).abs() < 0.001,
                            "{alignment:?} {width}: a hanging space is {painted} wide"
                        );
                    }
                }
                for (_, end) in &line_ends[..line_ends.len() - 1] {
                    assert!(
                        (end - content.width).abs() < 0.01,
                        "{alignment:?} {width}: a wrapped line ends at {end}"
                    );
                }
            }
        }
    }

    #[test]
    fn empty_paragraph_uses_end_style_size_for_line_and_spacing() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let end_style = ResolvedRunStyle {
            font_size: Some(30.0),
            latin_typeface: Some("Carlito".to_owned()),
            ..ResolvedRunStyle::default()
        };
        let body = ResolvedTextBody {
            space_first_last_paragraph: true,
            paragraphs: vec![ResolvedParagraph {
                space_before: Some(ResolvedTextSpacing::Percent(0.5)),
                end_style,
                ..ResolvedParagraph::default()
            }],
            ..text_body(TextInsets::default())
        };

        let stacked = stack_text(
            &mut fonts,
            Rect {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 100.0,
            },
            &body,
        )
        .expect("stack empty paragraph");

        assert!(stacked.elements.is_empty());
        assert_close(stacked.height, SINGLE_SPACING_EM * 30.0 * 1.5);
    }

    #[test]
    fn empty_paragraph_suppresses_bullet_and_keeps_end_metrics() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let end_style = ResolvedRunStyle {
            font_size: Some(30.0),
            latin_typeface: Some("Carlito".to_owned()),
            ..ResolvedRunStyle::default()
        };
        let body = ResolvedTextBody {
            paragraphs: vec![ResolvedParagraph {
                bullet: Some(ResolvedBullet::Character {
                    character: "•".to_owned(),
                    font: Some("Carlito".to_owned()),
                    color: None,
                    size: None,
                }),
                end_style,
                ..ResolvedParagraph::default()
            }],
            ..text_body(TextInsets::default())
        };

        let stacked = stack_text(&mut fonts, test_content_box(100.0), &body)
            .expect("stack empty bullet paragraph");
        assert!(stacked.elements.is_empty());
        assert_close(stacked.height, SINGLE_SPACING_EM * 30.0);
    }

    #[test]
    fn all_caps_keep_logical_text_and_shape_uppercase() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let caps_style = ResolvedRunStyle {
            font_size: Some(18.0),
            all_caps: true,
            latin_typeface: Some("Carlito".to_owned()),
            ..ResolvedRunStyle::default()
        };
        let normal_style = ResolvedRunStyle {
            all_caps: false,
            ..caps_style.clone()
        };

        let caps = shape_run(&mut fonts, "Straße", &caps_style).expect("shape all caps");
        let expected = shape_run(&mut fonts, "STRASSE", &normal_style).expect("shape uppercase");

        assert_eq!(caps.text, "Straße");
        assert_eq!(caps.font_id, expected.font_id);
        assert_eq!(caps.glyph_ids, expected.glyph_ids);
        assert_eq!(caps.advances, expected.advances);
        assert_close(caps.width, expected.width);
    }

    #[test]
    fn horizontal_alignment_and_distribution_use_the_available_line_width() {
        let segment = TextSegment {
            text: "ab".to_owned(),
            direction: oxml_layout::TextDirection::Auto,
            source: None,
            font_id: oxml_layout::FontId(0),
            font_size: 12.0,
            glyph_ids: vec![1, 2],
            advances: vec![5.0, 5.0],
            width: 10.0,
            ascent: 8.0,
            descent: 2.0,
            line_gap: 0.0,
            color: Color::BLACK,
            bold: false,
            italic: false,
            underline: None,
            strike: false,
            dstrike: false,
            highlight: None,
            baseline_offset: 0.0,
            hyperlink_url: None,
            field_kind: None,
            field_source: None,
            note: None,
            note_reference_source: None,
        };
        let line = oxml_layout::LayoutLine {
            items: vec![
                oxml_layout::LineItem::Text(segment.clone()),
                oxml_layout::LineItem::Text(segment.clone()),
            ],
            width: 20.0,
            ascent: 8.0,
            descent: 2.0,
            line_gap: 0.0,
            height: 10.0,
            indent_left: 10.0,
            available_width: 80.0,
            is_last: true,
            forced_break_after: None,
        };

        for (alignment, expected_x) in [
            (ParagraphAlignment::Left, 14.0),
            (ParagraphAlignment::Center, 44.0),
            (ParagraphAlignment::Right, 74.0),
        ] {
            let mut elements = Vec::new();
            let (start_x, width) = emit_line_items(&line, alignment, 4.0, 20.0, &mut elements);
            let runs = glyph_runs(&elements);
            assert_close(runs[0].origin.x, expected_x);
            assert_close(start_x, expected_x);
            assert_close(width, 20.0);
        }

        let mut elements = Vec::new();
        let (_, width) = emit_line_items(
            &line,
            ParagraphAlignment::Distributed,
            4.0,
            20.0,
            &mut elements,
        );
        let runs = glyph_runs(&elements);
        assert_close(runs[0].origin.x, 14.0);
        assert_close(width, 80.0);
        assert_close(
            runs.iter().flat_map(|run| run.advances.iter()).sum::<f64>(),
            80.0,
        );

        let mut justified_segment = segment;
        justified_segment.text = "a b".to_owned();
        justified_segment.glyph_ids = vec![1, 3, 2];
        justified_segment.advances = vec![5.0, 5.0, 5.0];
        justified_segment.width = 15.0;
        let mut justified_line = oxml_layout::LayoutLine {
            items: vec![oxml_layout::LineItem::Text(justified_segment)],
            width: 15.0,
            ascent: 8.0,
            descent: 2.0,
            line_gap: 0.0,
            height: 10.0,
            indent_left: 0.0,
            available_width: 45.0,
            is_last: false,
            forced_break_after: None,
        };
        let mut elements = Vec::new();
        let (_, width) = emit_line_items(
            &justified_line,
            ParagraphAlignment::Justified,
            0.0,
            20.0,
            &mut elements,
        );
        assert_close(width, 45.0);
        assert_close(glyph_runs(&elements)[0].advances.iter().sum(), 45.0);

        justified_line.is_last = true;
        elements.clear();
        let (_, width) = emit_line_items(
            &justified_line,
            ParagraphAlignment::Justified,
            0.0,
            20.0,
            &mut elements,
        );
        assert_close(width, 15.0);
        assert_close(glyph_runs(&elements)[0].advances.iter().sum(), 15.0);
    }

    #[test]
    fn justified_ligature_text_still_expands_its_word_gap() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let segment = shape_run(
            &mut fonts,
            "office space",
            &ResolvedRunStyle {
                font_size: Some(12.0),
                latin_typeface: Some("Carlito".to_owned()),
                ..ResolvedRunStyle::default()
            },
        )
        .expect("shape ligature-bearing text");
        assert_ne!(
            segment.text.chars().count(),
            segment.advances.len(),
            "fixture must exercise a non-1:1 shaped run"
        );
        let available_width = segment.width + 30.0;
        let line = LayoutLine {
            items: vec![LineItem::Text(segment)],
            width: available_width - 30.0,
            ascent: 8.0,
            descent: 2.0,
            line_gap: 0.0,
            height: 10.0,
            indent_left: 0.0,
            available_width,
            is_last: false,
            forced_break_after: None,
        };
        let mut elements = Vec::new();

        let (_, width) = emit_line_items(
            &line,
            ParagraphAlignment::Justified,
            0.0,
            20.0,
            &mut elements,
        );

        assert_close(width, available_width);
        assert_close(
            glyph_runs(&elements)[0].advances.iter().sum(),
            available_width,
        );
    }

    #[test]
    fn top_center_and_bottom_anchors_use_zero_half_and_full_spare_height() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let paragraph = ResolvedParagraph {
            line_spacing: Some(ResolvedTextSpacing::Points(20.0)),
            runs: vec![ResolvedTextRun::Text {
                text: "anchor".to_owned(),
                style: ResolvedRunStyle {
                    font_size: Some(12.0),
                    latin_typeface: Some("Carlito".to_owned()),
                    ..ResolvedRunStyle::default()
                },
            }],
            ..ResolvedParagraph::default()
        };
        let content = Rect {
            x: 0.0,
            y: 7.0,
            width: 100.0,
            height: 100.0,
        };
        let mut baselines = Vec::new();

        for anchor in [TextAnchor::Top, TextAnchor::Center, TextAnchor::Bottom] {
            let body = ResolvedTextBody {
                anchor,
                paragraphs: vec![paragraph.clone()],
                ..text_body(TextInsets::default())
            };
            let stacked = stack_text(&mut fonts, content, &body).expect("stack anchored text");
            baselines.push(glyph_runs(&stacked.elements)[0].origin.y);
        }

        // A 20 point line is taller than single spacing at 12 points, so its
        // baseline sits three quarters of the way down.
        assert_close(baselines[0], content.y + 0.75 * 20.0);
        assert_close(baselines[1] - baselines[0], 40.0);
        assert_close(baselines[2] - baselines[0], 80.0);
    }

    #[test]
    fn overflowing_anchored_text_remains_visible_without_a_clip() {
        let text = ResolvedTextBody {
            anchor: TextAnchor::Bottom,
            wrap: false,
            paragraphs: vec![ResolvedParagraph {
                line_spacing: Some(ResolvedTextSpacing::Points(20.0)),
                runs: vec![ResolvedTextRun::Text {
                    text: "visible overflow".to_owned(),
                    style: ResolvedRunStyle {
                        font_size: Some(12.0),
                        latin_typeface: Some("Carlito".to_owned()),
                        ..ResolvedRunStyle::default()
                    },
                }],
                ..ResolvedParagraph::default()
            }],
            ..text_body(TextInsets::default())
        };
        let mut text_shape = shape(
            Rect {
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 2.0,
            },
            ResolvedGeometry::Rectangle,
        );
        text_shape.content = ResolvedContent::Text(text);
        let input = RenderInput {
            slides: vec![ResolvedSlide {
                unresolved_member_count: 0,
                size: (100.0, 50.0),
                background: None,
                shapes: vec![text_shape],
                diagnostics: Vec::new(),
            }],
            media: HashMap::new(),
            fonts: Vec::new(),
            metadata: None,
        };
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");

        let page = layout_slide_with_fonts(&input, 0, &mut fonts).expect("layout overflow");
        let PositionedElement::Group(shape_group) = &page.elements[0] else {
            panic!("shape should lower to a group");
        };
        let baseline = glyph_runs(&shape_group.children)[0].origin.y;

        assert!(shape_group.clip.is_none());
        assert!(
            baseline < 0.0,
            "bottom anchor should preserve negative offset"
        );
    }

    #[test]
    fn justified_and_distributed_anchors_allocate_positive_spare_height() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let paragraph = || ResolvedParagraph {
            line_spacing: Some(ResolvedTextSpacing::Points(10.0)),
            runs: vec![ResolvedTextRun::Text {
                text: "line".to_owned(),
                style: ResolvedRunStyle {
                    font_size: Some(8.0),
                    latin_typeface: Some("Carlito".to_owned()),
                    ..ResolvedRunStyle::default()
                },
            }],
            ..ResolvedParagraph::default()
        };
        let content = Rect {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 60.0,
        };

        let baselines = |anchor, fonts: &mut FontManager| {
            let body = ResolvedTextBody {
                anchor,
                paragraphs: vec![paragraph(), paragraph(), paragraph()],
                ..text_body(TextInsets::default())
            };
            let stacked = stack_text(fonts, content, &body).expect("stack distributed text");
            distinct_baselines(&glyph_runs(&stacked.elements))
        };

        let justified = baselines(TextAnchor::Justified, &mut fonts);
        assert_close(justified[1] - justified[0], 25.0);
        assert_close(justified[2] - justified[1], 25.0);
        let distributed = baselines(TextAnchor::Distributed, &mut fonts);
        assert_close(distributed[0] - justified[0], 5.0);
        assert_close(distributed[1] - distributed[0], 20.0);
        assert_close(distributed[2] - distributed[1], 20.0);
    }

    #[test]
    fn bottom_center_text_in_an_inset_box_lands_at_the_computed_baseline() {
        let insets = TextInsets {
            left: 10.0,
            top: 8.0,
            right: 14.0,
            bottom: 12.0,
        };
        let style = ResolvedRunStyle {
            font_size: Some(12.0),
            latin_typeface: Some("Carlito".to_owned()),
            ..ResolvedRunStyle::default()
        };
        let text = ResolvedTextBody {
            anchor: TextAnchor::Bottom,
            paragraphs: vec![ResolvedParagraph {
                alignment: ParagraphAlignment::Center,
                line_spacing: Some(ResolvedTextSpacing::Points(20.0)),
                runs: vec![ResolvedTextRun::Text {
                    text: "baseline".to_owned(),
                    style: style.clone(),
                }],
                ..ResolvedParagraph::default()
            }],
            ..text_body(insets)
        };
        let mut text_shape = shape(
            Rect {
                x: 0.0,
                y: 0.0,
                width: 120.0,
                height: 80.0,
            },
            ResolvedGeometry::Rectangle,
        );
        text_shape.content = ResolvedContent::Text(text);
        let input = RenderInput {
            slides: vec![ResolvedSlide {
                unresolved_member_count: 0,
                size: (120.0, 80.0),
                background: None,
                shapes: vec![text_shape],
                diagnostics: Vec::new(),
            }],
            media: HashMap::new(),
            fonts: Vec::new(),
            metadata: None,
        };
        let mut expected_fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let expected = shape_run(&mut expected_fonts, "baseline", &style).expect("shape baseline");
        let content_width = 120.0 - insets.left - insets.right;
        let content_height = 80.0 - insets.top - insets.bottom;
        let expected_x = insets.left + (content_width - expected.width) / 2.0;
        let expected_y = insets.top + (content_height - 20.0) + 0.75 * 20.0;
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");

        let page = layout_slide_with_fonts(&input, 0, &mut fonts).expect("layout anchored text");
        let PositionedElement::Group(shape_group) = &page.elements[0] else {
            panic!("shape should lower to a group");
        };
        let run = glyph_runs(&shape_group.children)[0];

        assert!(matches!(
            shape_group.children.first(),
            Some(PositionedElement::Path(_))
        ));
        assert_close(run.origin.x, expected_x);
        assert_close(run.origin.y, expected_y);
    }

    #[test]
    fn shape_text_stays_above_the_path_and_overflows_without_a_clip() {
        let text = ResolvedTextBody {
            wrap: false,
            paragraphs: vec![ResolvedParagraph {
                runs: vec![ResolvedTextRun::Text {
                    text: "overflowing text".to_owned(),
                    style: ResolvedRunStyle {
                        font_size: Some(12.0),
                        latin_typeface: Some("Carlito".to_owned()),
                        ..ResolvedRunStyle::default()
                    },
                }],
                ..ResolvedParagraph::default()
            }],
            ..text_body(TextInsets::default())
        };
        let mut text_shape = shape(
            Rect {
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 20.0,
            },
            ResolvedGeometry::Rectangle,
        );
        text_shape.content = ResolvedContent::Text(text);
        let input = RenderInput {
            slides: vec![ResolvedSlide {
                unresolved_member_count: 0,
                size: (100.0, 50.0),
                background: None,
                shapes: vec![text_shape],
                diagnostics: Vec::new(),
            }],
            media: HashMap::new(),
            fonts: Vec::new(),
            metadata: None,
        };
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");

        let page = layout_slide_with_fonts(&input, 0, &mut fonts).expect("layout shape text");
        let PositionedElement::Group(shape_group) = &page.elements[0] else {
            panic!("shape should lower to a group");
        };

        assert!(shape_group.clip.is_none());
        assert!(matches!(
            shape_group.children.first(),
            Some(PositionedElement::Path(_))
        ));
        let text_runs = glyph_runs(&shape_group.children);
        assert!(!text_runs.is_empty());
        assert!(
            text_runs
                .iter()
                .map(|run| run.origin.x + run.advances.iter().sum::<f64>())
                .fold(0.0_f64, f64::max)
                > 10.0
        );
    }

    #[test]
    fn wingdings_f0b7_bullet_renders_as_a_visible_unicode_glyph() {
        let body = ResolvedTextBody {
            paragraphs: vec![bullet_paragraph(
                0,
                Some(ResolvedBullet::Character {
                    character: "\u{f0b7}".to_owned(),
                    font: Some("Wingdings".to_owned()),
                    color: None,
                    size: None,
                }),
                "visible",
            )],
            ..text_body(TextInsets::default())
        };
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let stacked =
            stack_text(&mut fonts, test_content_box(120.0), &body).expect("stack character bullet");
        let runs = glyph_runs(&stacked.elements);

        assert_eq!(runs[0].text, "\u{2022}");
        assert!(runs[0].glyph_ids.iter().all(|glyph| *glyph != 0));
    }

    #[test]
    fn fx211c_unknown_symbol_marker_facts_are_owned_ordered_and_not_missing_claims() {
        let mut body = ResolvedTextBody {
            paragraphs: [
                ("Wingdings", "\u{f0ff}"),
                ("Wingdings", "\u{f0ff}"),
                ("Wingdings", "\u{f0a7}"),
                ("Symbol", "\u{f0b7}"),
                ("Arial", "\u{f0ff}"),
            ]
            .into_iter()
            .map(|(family, character)| {
                bullet_paragraph(
                    0,
                    Some(ResolvedBullet::Character {
                        character: character.to_owned(),
                        font: Some(family.to_owned()),
                        color: None,
                        size: None,
                    }),
                    "visible",
                )
            })
            .collect(),
            ..text_body(TextInsets::default())
        };
        let mut fonts = FontManager::new_deterministic().unwrap();
        let cold = stack_text(&mut fonts, test_content_box(120.0), &body).unwrap();
        assert_eq!(
            cold.diagnostics,
            ["Unknown Wingdings symbol U+F0FF: original code retained"]
        );
        let runs = glyph_runs(&cold.elements);
        assert_eq!(runs[0].text, "\u{f0ff}");
        assert!(
            runs[0].glyph_ids.contains(&0),
            "actual missing glyph is separately observable"
        );
        assert!(!cold.diagnostics[0].to_ascii_lowercase().contains("missing"));
        let warm = stack_text(&mut fonts, test_content_box(120.0), &body).unwrap();
        assert_eq!(warm.diagnostics, cold.diagnostics);
        assert_eq!(glyph_runs(&warm.elements)[0].text, "\u{f0ff}");
        for paragraph in &mut body.paragraphs {
            paragraph.runs.clear();
        }
        let empty = stack_text(&mut fonts, test_content_box(120.0), &body).unwrap();
        assert!(
            empty.diagnostics.is_empty(),
            "discarded and empty layouts cannot leak marker facts"
        );
        assert!(
            !fonts
                .current_font_diagnostics()
                .iter()
                .any(|fact| fact.message.contains("Unknown Wingdings"))
        );
    }

    #[test]
    fn fx211c_unknown_symbol_semantics_do_not_claim_an_existing_glyph_is_missing() {
        let mut fonts = FontManager::new_deterministic().unwrap();
        fonts.load_additional_fonts(&[oxml_layout::FontFile {
            family: "Wingdings".to_owned(),
            data: include_bytes!("../../oxml-layout/fonts/Carlito-Regular.ttf").to_vec(),
        }]);
        let style = ResolvedRunStyle {
            latin_typeface: Some("Wingdings".to_owned()),
            symbol_typeface: Some("Wingdings".to_owned()),
            ..ResolvedRunStyle::default()
        };
        let (displayed, _) = fonts
            .resolve_symbol_text(Some("Wingdings"), false, false, "ÿ", true)
            .unwrap();
        let marker = shape_run(&mut fonts, &displayed, &style).unwrap();
        assert!(marker.glyph_ids.iter().all(|id| *id != 0));
        let mut diagnostics = Vec::new();
        append_symbol_marker_diagnostics(&fonts, Some("Wingdings"), "ÿ", &marker, &mut diagnostics);
        assert_eq!(
            diagnostics,
            ["Unknown Wingdings symbol U+00FF: original code retained"]
        );
        assert!(!diagnostics[0].to_ascii_lowercase().contains("missing"));
    }

    #[test]
    fn fx211c_symbol_marker_original_scalar_dedup_is_bounded() {
        let original = (0..=0xff)
            .chain(0xf000..=0xf0ff)
            .map(|code| char::from_u32(code).unwrap())
            .collect::<String>();
        let mut fonts = FontManager::new_deterministic().unwrap();
        let mut diagnostics = Vec::new();
        for family in ["Symbol", "Wingdings"] {
            let (displayed, _) = fonts
                .resolve_symbol_text(Some(family), false, false, &original, true)
                .unwrap();
            let style = ResolvedRunStyle {
                latin_typeface: Some(family.to_owned()),
                symbol_typeface: Some(family.to_owned()),
                ..ResolvedRunStyle::default()
            };
            let marker = shape_run(&mut fonts, &displayed, &style).unwrap();
            append_symbol_marker_diagnostics(
                &fonts,
                Some(family),
                &original,
                &marker,
                &mut diagnostics,
            );
            let before = diagnostics.clone();
            append_symbol_marker_diagnostics(
                &fonts,
                Some(family),
                &original,
                &marker,
                &mut diagnostics,
            );
            assert_eq!(diagnostics, before);
        }
        assert!(diagnostics.len() > 512 && diagnostics.len() <= 1024);
        assert!(
            diagnostics
                .iter()
                .any(|fact| fact.contains("Wingdings symbol U+00FF"))
        );
        assert!(
            diagnostics
                .iter()
                .any(|fact| fact.contains("Wingdings symbol U+F0FF"))
        );
    }

    #[test]
    fn eight_common_auto_number_schemes_format_exact_markers() {
        let schemes = [
            "arabicPlain",
            "arabicPeriod",
            "arabicParenR",
            "arabicParenBoth",
            "alphaLcPeriod",
            "alphaUcPeriod",
            "romanLcPeriod",
            "romanUcPeriod",
        ];
        let body = ResolvedTextBody {
            paragraphs: schemes
                .into_iter()
                .map(|scheme| {
                    bullet_paragraph(
                        0,
                        Some(ResolvedBullet::AutoNumber {
                            scheme: scheme.to_owned(),
                            start_at: 1,
                            font: None,
                            color: None,
                            size: None,
                        }),
                        "item",
                    )
                })
                .collect(),
            ..text_body(TextInsets::default())
        };
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let stacked = stack_text(&mut fonts, test_content_box(120.0), &body)
            .expect("stack automatic bullets");
        let markers: Vec<&str> = glyph_runs(&stacked.elements)
            .into_iter()
            .map(|run| run.text.as_str())
            .filter(|text| *text != "item")
            .collect();

        assert_eq!(markers, ["1", "1.", "1)", "(1)", "a.", "A.", "i.", "I."]);
    }

    #[test]
    fn automatic_bullets_increment_and_reset_by_level() {
        let auto = |level, scheme: &str, start_at| {
            bullet_paragraph(
                level,
                Some(ResolvedBullet::AutoNumber {
                    scheme: scheme.to_owned(),
                    start_at,
                    font: None,
                    color: None,
                    size: None,
                }),
                "item",
            )
        };
        let body = ResolvedTextBody {
            paragraphs: vec![
                auto(0, "arabicPeriod", 3),
                auto(1, "alphaLcPeriod", 1),
                auto(1, "alphaLcPeriod", 1),
                auto(0, "arabicPeriod", 3),
                auto(1, "alphaLcPeriod", 1),
                auto(0, "romanUcPeriod", 4),
                bullet_paragraph(0, None, "plain"),
                auto(0, "romanUcPeriod", 4),
                bullet_paragraph(
                    0,
                    Some(ResolvedBullet::Character {
                        character: "*".to_owned(),
                        font: None,
                        color: None,
                        size: None,
                    }),
                    "item",
                ),
                auto(0, "romanUcPeriod", 4),
            ],
            ..text_body(TextInsets::default())
        };
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let stacked =
            stack_text(&mut fonts, test_content_box(120.0), &body).expect("stack numbered outline");
        let markers: Vec<&str> = glyph_runs(&stacked.elements)
            .into_iter()
            .map(|run| run.text.as_str())
            .filter(|text| *text != "item" && *text != "plain")
            .collect();

        assert_eq!(
            markers,
            ["3.", "a.", "b.", "4.", "a.", "IV.", "IV.", "*", "IV."]
        );
    }

    #[test]
    fn bullet_style_overrides_and_fallbacks_are_independent() {
        let fallback_style = ResolvedRunStyle {
            font_size: Some(20.0),
            bold: true,
            fill: Some(Paint::Solid(Color {
                r: 0.1,
                g: 0.2,
                b: 0.8,
                a: 1.0,
            })),
            latin_typeface: Some("Carlito".to_owned()),
            ..ResolvedRunStyle::default()
        };
        let bullet = |font, color, size| ResolvedBullet::Character {
            character: "*".to_owned(),
            font,
            color,
            size,
        };
        let paragraph = |bullet| ResolvedParagraph {
            bullet: Some(bullet),
            runs: vec![ResolvedTextRun::Text {
                text: "item".to_owned(),
                style: fallback_style.clone(),
            }],
            ..ResolvedParagraph::default()
        };
        let override_color = Color {
            r: 0.8,
            g: 0.2,
            b: 0.1,
            a: 1.0,
        };
        let body = ResolvedTextBody {
            paragraphs: vec![
                paragraph(bullet(Some("Caladea".to_owned()), None, None)),
                paragraph(bullet(
                    None,
                    Some(override_color),
                    Some(ResolvedBulletSize::Points(11.0)),
                )),
                paragraph(bullet(None, None, Some(ResolvedBulletSize::Percent(1.5)))),
            ],
            ..text_body(TextInsets::default())
        };
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let stacked =
            stack_text(&mut fonts, test_content_box(120.0), &body).expect("stack styled bullets");
        let markers: Vec<_> = glyph_runs(&stacked.elements)
            .into_iter()
            .filter(|run| run.text == "*")
            .collect();
        let families: HashMap<_, _> = fonts
            .all_font_data()
            .into_iter()
            .map(|font| (font.id, font.family))
            .collect();

        assert_eq!(families[&markers[0].font_id], "Caladea");
        assert_eq!(markers[0].font_size, 20.0);
        assert_eq!(markers[0].color, text_color(fallback_style.fill.as_ref()));
        assert!(markers[0].bold);
        assert_eq!(families[&markers[1].font_id], "Carlito");
        assert_eq!(markers[1].font_size, 11.0);
        assert_eq!(markers[1].color, override_color);
        assert_eq!(markers[2].font_size, 30.0);
    }

    #[test]
    fn bullet_marker_uses_left_and_hanging_indent() {
        let mut paragraph = bullet_paragraph(
            0,
            Some(ResolvedBullet::Character {
                character: "*".to_owned(),
                font: None,
                color: None,
                size: None,
            }),
            "one two three four five",
        );
        paragraph.left_margin = 20.0;
        paragraph.indent = -10.0;
        let body = ResolvedTextBody {
            paragraphs: vec![paragraph],
            ..text_body(TextInsets::default())
        };
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let stacked =
            stack_text(&mut fonts, test_content_box(55.0), &body).expect("stack indented bullet");
        let runs = glyph_runs(&stacked.elements);
        let marker_x = runs
            .iter()
            .find(|run| run.text == "*")
            .expect("marker")
            .origin
            .x;
        // Spaces hanging past the wrapped lines are runs of their own.
        let text_runs: Vec<_> = runs
            .into_iter()
            .filter(|run| run.text != "*" && !run.text.trim().is_empty())
            .collect();

        assert_close(marker_x, 10.0);
        assert_close(text_runs[0].origin.x, 20.0);
        assert!(
            text_runs
                .iter()
                .skip(1)
                .all(|run| (run.origin.x - 20.0).abs() < 0.001)
        );
    }

    #[test]
    fn distributed_bullet_keeps_the_fixed_hanging_slot() {
        let mut paragraph = bullet_paragraph(
            0,
            Some(ResolvedBullet::Character {
                character: "*".to_owned(),
                font: None,
                color: None,
                size: None,
            }),
            "ab",
        );
        paragraph.left_margin = 20.0;
        paragraph.indent = -10.0;
        paragraph.alignment = ParagraphAlignment::Distributed;
        let body = ResolvedTextBody {
            paragraphs: vec![paragraph],
            ..text_body(TextInsets::default())
        };
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");

        let stacked =
            stack_text(&mut fonts, test_content_box(55.0), &body).expect("stack bullet text");
        let runs = glyph_runs(&stacked.elements);
        let marker = runs.iter().find(|run| run.text == "*").expect("marker");
        let text = runs.iter().find(|run| run.text == "ab").expect("body text");

        assert_close(marker.origin.x, 10.0);
        assert_close(text.origin.x, 20.0);
    }

    #[test]
    fn wide_auto_number_marker_keeps_text_on_the_paragraph_margin() {
        let mut paragraph = bullet_paragraph(
            0,
            Some(ResolvedBullet::AutoNumber {
                scheme: "arabicPeriod".to_owned(),
                start_at: 12_345,
                font: None,
                color: None,
                size: None,
            }),
            "one two three four five",
        );
        paragraph.left_margin = 20.0;
        paragraph.indent = -10.0;
        let body = ResolvedTextBody {
            paragraphs: vec![paragraph],
            ..text_body(TextInsets::default())
        };
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let stacked = stack_text(&mut fonts, test_content_box(55.0), &body)
            .expect("stack wide automatic bullet");
        let runs = glyph_runs(&stacked.elements);
        let marker = runs
            .iter()
            .find(|run| run.text == "12345.")
            .expect("automatic marker");
        // Spaces hanging past the wrapped lines are runs of their own.
        let text_runs: Vec<_> = runs
            .iter()
            .filter(|run| run.text != "12345." && !run.text.trim().is_empty())
            .collect();

        assert!(marker.advances.iter().sum::<f64>() > 10.0);
        assert_close(marker.origin.x, 10.0);
        assert_close(text_runs[0].origin.x, 20.0);
        assert!(
            text_runs
                .iter()
                .skip(1)
                .all(|run| (run.origin.x - 20.0).abs() < 0.001)
        );
    }

    #[test]
    fn unsupported_auto_number_scheme_keeps_a_visible_marker() {
        let body = ResolvedTextBody {
            paragraphs: vec![bullet_paragraph(
                0,
                Some(ResolvedBullet::AutoNumber {
                    scheme: "thaiNumPeriod".to_owned(),
                    start_at: 7,
                    font: None,
                    color: None,
                    size: None,
                }),
                "item",
            )],
            ..text_body(TextInsets::default())
        };
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let stacked =
            stack_text(&mut fonts, test_content_box(120.0), &body).expect("stack fallback bullet");
        let runs = glyph_runs(&stacked.elements);

        assert_eq!(runs[0].text, "7.");
        assert!(runs[0].glyph_ids.iter().all(|glyph| *glyph != 0));
    }

    #[test]
    fn stored_font_scale_renders_at_exactly_sixty_two_point_five_percent() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let mut body = autofit_body(
            ResolvedAutofit::Normal {
                font_scale: Some(0.625),
                line_spacing_reduction: None,
            },
            20.0,
            "stored scale",
        );
        body.paragraphs[0].bullet = Some(ResolvedBullet::Character {
            character: "*".to_owned(),
            font: None,
            color: None,
            size: Some(ResolvedBulletSize::Points(10.0)),
        });

        let stacked = stack_text(&mut fonts, test_content_box(200.0), &body)
            .expect("stack stored autofit text");
        let runs = glyph_runs(&stacked.elements);

        assert_close(runs[0].font_size, 6.25);
        assert_close(runs[1].font_size, 12.5);
    }

    #[test]
    fn fx206_caps_and_font_contribution_cache_keep_occurrences_on_cold_and_warm_paths() {
        let text = "aBß";
        let style = ResolvedRunStyle {
            latin_typeface: Some("Carlito".into()),
            font_size: Some(24.0),
            language: Some("en".into()),
            ..Default::default()
        };
        let mut capped = style.clone();
        capped.small_caps = true;
        let paragraph = ResolvedParagraph {
            runs: vec![
                ResolvedTextRun::Text {
                    text: text.into(),
                    style: capped,
                },
                ResolvedTextRun::Text {
                    text: text.into(),
                    style,
                },
            ],
            ..Default::default()
        };
        let mut fonts = FontManager::new_deterministic().unwrap();
        let mut cache = ShapingCache::default();
        let mut observations = Vec::new();
        for _ in 0..2 {
            fonts.begin_layout();
            fonts.begin_paragraph_font_trace();
            let mut contexts = HashMap::new();
            let mut requests = Vec::new();
            let provisional = inline_items_with_caps(
                &mut fonts,
                &mut cache,
                &paragraph,
                1,
                &mut requests,
                &mut contexts,
            )
            .unwrap();
            assert_eq!(requests.len(), 2);
            assert!(requests[0].0);
            assert!(!requests[1].0);
            let mut diagnostics = Vec::new();
            let accepted = shape_multilingual_items_preserving_controls(
                &mut fonts,
                provisional,
                oxml_layout::TextDirection::LeftToRight,
                &requests,
                &mut diagnostics,
                &contexts,
            )
            .unwrap();
            let spans = accepted
                .iter()
                .filter_map(|item| match item {
                    InlineItem::MultilingualText(span) => Some(span),
                    _ => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(
                spans.iter().map(|span| span.text()).collect::<String>(),
                "aBßaBß"
            );
            assert!(spans.iter().any(|span| span.base().font_size == 24.0));
            let events = fonts.finish_paragraph_font_events().unwrap();
            let contributions = events
                .iter()
                .filter_map(|event| match event {
                    FontTraceEvent::Contribution(contribution) => Some(contribution),
                    FontTraceEvent::Resolution(_) => None,
                })
                .collect::<Vec<_>>();
            assert!(!contributions.is_empty());
            assert!(
                contributions
                    .iter()
                    .all(|item| item.requested_family.as_deref() == Some("Carlito"))
            );
            for span in &spans {
                assert!(
                    contributions
                        .iter()
                        .any(|item| item.font_id == span.base().font_id)
                );
            }
            let physical = contributions
                .iter()
                .map(|item| {
                    let face = fonts.font_data(item.font_id).unwrap();
                    (
                        item.requested_family.clone(),
                        face.family,
                        face.face_index,
                        face.data.to_vec(),
                        face.variations,
                        face.synthetic_bold,
                        face.synthetic_italic,
                    )
                })
                .collect::<Vec<_>>();
            let shapes = spans
                .iter()
                .map(|span| {
                    (
                        span.text().to_owned(),
                        span.base().font_size,
                        span.glyph_ids().to_vec(),
                        span.x_advances().to_vec(),
                        span.clusters().to_vec(),
                    )
                })
                .collect::<Vec<_>>();
            observations.push((physical, shapes, diagnostics));
        }
        assert_eq!(observations[0], observations[1]);
        assert_eq!(
            cache.entries.len(),
            2,
            "equal text with different cap requests uses distinct bounded cache entries"
        );
        assert!(cache.bytes < NATIVE_SHAPING_CACHE_MAX_BYTES);
    }

    #[test]
    fn fx205_grid_fill_cache_obeys_owned_heap_budget() {
        let mut fonts = FontManager::new_deterministic().unwrap();
        let mut cache = ShapingCache::default();
        let fill = Paint::grid(
            oxml_layout::Rect {
                x: 0.0,
                y: 0.0,
                width: 8.0,
                height: 8.0,
            },
            256,
            256,
            vec![oxml_layout::Color::BLACK; 65_536],
        )
        .unwrap();
        let style = ResolvedRunStyle {
            latin_typeface: Some("Carlito".into()),
            font_size: Some(20.0),
            fill: Some(fill),
            ..Default::default()
        };
        for i in 0..10 {
            fonts.begin_layout();
            fonts.begin_paragraph_font_trace();
            let text = format!("owned grid {i}");
            let segment = cache.shape(&mut fonts, &text, &style).unwrap();
            assert!(!segment.glyph_ids.is_empty());
            fonts.finish_paragraph_font_trace().unwrap();
        }
        assert!(
            !cache.entries.is_empty(),
            "below-budget grid owners are cached"
        );
        assert!(
            cache.entries.len() < 10,
            "over-budget owners must remain uncached"
        );
        let retained_colors = cache
            .entries
            .iter()
            .map(|(_, style, _, _)| match &style.fill {
                Some(Paint::Grid { colors, .. }) => {
                    colors.capacity() * std::mem::size_of::<oxml_layout::Color>()
                }
                _ => 0,
            })
            .sum::<usize>();
        assert!(cache.bytes >= retained_colors);
        assert!(
            cache.bytes
                + cache.entries.capacity()
                    * std::mem::size_of::<(
                        String,
                        ResolvedRunStyle,
                        TextSegment,
                        Vec<FontTraceEvent>
                    )>()
                <= NATIVE_SHAPING_CACHE_MAX_BYTES
        );
    }

    #[test]
    fn fx195_shaping_cache_multilingual_probe_is_not_a_contribution() {
        let paragraph = ResolvedParagraph {
            runs: vec![ResolvedTextRun::Text {
                text: "कार्यालय".into(),
                style: ResolvedRunStyle {
                    latin_typeface: Some("Carlito".into()),
                    complex_script_typeface: Some("Carlito".into()),
                    font_size: Some(20.0),
                    ..Default::default()
                },
            }],
            ..Default::default()
        };
        let mut fonts = FontManager::new_deterministic().unwrap();
        let primary = fonts.resolve_font(Some("Carlito"), false, false).unwrap();
        let mut cache = ShapingCache::default();
        let mut captures = Vec::new();
        for _ in 0..2 {
            fonts.begin_layout();
            fonts.begin_paragraph_font_trace();
            let mut contexts = HashMap::new();
            let provisional =
                inline_items_cached(&mut fonts, &mut cache, &paragraph, 1, &mut contexts).unwrap();
            let requests = provisional
                .iter()
                .filter(|item| matches!(item, InlineItem::Text(_)))
                .map(|_| (false, false, None, None))
                .collect::<Vec<_>>();
            let accepted = shape_multilingual_items_preserving_controls(
                &mut fonts,
                provisional,
                oxml_layout::TextDirection::LeftToRight,
                &requests,
                &mut Vec::new(),
                &contexts,
            )
            .unwrap();
            let events = fonts.finish_paragraph_font_events().unwrap();
            let contributions = events
                .iter()
                .filter_map(|event| match event {
                    FontTraceEvent::Contribution(contribution) => Some(contribution),
                    FontTraceEvent::Resolution(_) => None,
                })
                .collect::<Vec<_>>();
            assert!(!contributions.is_empty());
            assert!(
                events
                    .iter()
                    .any(|event| matches!(event, FontTraceEvent::Resolution(id) if *id == primary))
            );
            assert!(
                contributions
                    .iter()
                    .all(|contribution| contribution.font_id != primary)
            );
            assert!(contributions.iter().all(|contribution| contribution.requested_family.as_deref() == Some("Carlito")));
            let faces = fonts
                .current_layout_fonts()
                .iter()
                .map(|id| {
                    let face = fonts.font_data(*id).unwrap();
                    (
                        face.id,
                        face.family,
                        face.face_index,
                        face.data.to_vec(),
                        face.variations,
                        face.synthetic_bold,
                        face.synthetic_italic,
                    )
                })
                .collect::<Vec<_>>();
            captures.push((format!("{accepted:?}"), faces, events));
        }
        assert_eq!(
            captures[0], captures[1],
            "full font order, bytes, glyph IDs and actual requests"
        );
        assert_eq!(cache.entries.len(), 1);
    }

    #[test]
    fn fx195_shaping_cache_final_field_and_marker_requests() {
        let paragraph = ResolvedParagraph {
            bullet: Some(ResolvedBullet::Character {
                character: "*".into(),
                font: Some("MarkerRequested".into()),
                color: None,
                size: None,
            }),
            runs: vec![ResolvedTextRun::Field {
                text: "42".into(),
                field_type: Some("slidenum".into()),
                style: ResolvedRunStyle {
                    latin_typeface: Some("FieldRequested".into()),
                    font_size: Some(12.0),
                    ..Default::default()
                },
            }],
            ..Default::default()
        };
        let body = ResolvedTextBody {
            paragraphs: vec![paragraph],
            ..text_body(TextInsets::default())
        };
        let mut fonts = FontManager::new_deterministic().unwrap();
        fonts.begin_paragraph_font_trace();
        let stacked = stack_text(&mut fonts, test_content_box(200.0), &body).unwrap();
        assert!(!glyph_runs(&stacked.elements).is_empty());
        let contributions = fonts.finish_paragraph_font_contributions().unwrap();
        for requested in ["FieldRequested", "MarkerRequested"] {
            assert!(
                contributions
                    .iter()
                    .any(|entry| entry.requested_family.as_deref() == Some(requested))
            );
        }
        assert_eq!(
            contributions.len(),
            2,
            "only final field and visible marker"
        );
    }

    #[test]
    fn fx195_shaping_cache_empty_and_oversized_payloads_bypass_storage() {
        let mut fonts = FontManager::new_deterministic().unwrap();
        let mut cache = ShapingCache::default();
        cache
            .shape(&mut fonts, "", &ResolvedRunStyle::default())
            .unwrap();
        assert!(cache.entries.is_empty());
        let style = ResolvedRunStyle {
            hyperlink_url: Some("x".repeat(NATIVE_SHAPING_CACHE_MAX_BYTES + 1)),
            ..Default::default()
        };
        let result = cache.shape(&mut fonts, "A", &style).unwrap();
        assert!(!result.glyph_ids.is_empty());
        assert!(cache.entries.is_empty());
        assert_eq!(cache.bytes, 0);
    }

    #[test]
    fn fx195_shaping_cache_oversized_language_preserves_cold_revisited_contributions() {
        for oversized_length in [true, false] {
            let mut language = String::with_capacity(NATIVE_SHAPING_CACHE_MAX_BYTES + 1);
            if oversized_length {
                language.push_str(&"x".repeat(NATIVE_SHAPING_CACHE_MAX_BYTES + 1));
            } else {
                language.push_str("en-US");
            }
            assert!(language.capacity() > NATIVE_SHAPING_CACHE_MAX_BYTES);
            if !oversized_length {
                assert_eq!(language.len(), 5, "capacity, not length, decides this case");
            }
            let paragraph = ResolvedParagraph {
                runs: vec![ResolvedTextRun::Text {
                    text: "same contributing text".into(),
                    style: ResolvedRunStyle {
                        latin_typeface: Some("Aptos".into()),
                        font_size: Some(20.0),
                        language: Some(language),
                        ..Default::default()
                    },
                }],
                ..Default::default()
            };
            let mut fonts = FontManager::new_deterministic().unwrap();
            let mut cache = ShapingCache::default();
            let mut captures = Vec::new();
            for _ in 0..2 {
                fonts.begin_layout();
                fonts.begin_paragraph_font_trace();
                let mut contexts = HashMap::new();
                let mut requests = Vec::new();
                let provisional = inline_items_with_caps(
                    &mut fonts,
                    &mut cache,
                    &paragraph,
                    1,
                    &mut requests,
                    &mut contexts,
                )
                .unwrap();
                let accepted = shape_multilingual_items_preserving_controls(
                    &mut fonts,
                    provisional,
                    oxml_layout::TextDirection::LeftToRight,
                    &requests,
                    &mut Vec::new(),
                    &contexts,
                )
                .unwrap();
                let contributions = fonts.finish_paragraph_font_contributions().unwrap();
                assert!(!accepted.is_empty());
                assert!(!contributions.is_empty());
                assert!(
                    contributions
                        .iter()
                        .all(|entry| { entry.requested_family.as_deref() == Some("Aptos") })
                );
                let faces = fonts
                    .current_layout_fonts()
                    .iter()
                    .map(|id| {
                        let face = fonts.font_data(*id).unwrap();
                        (
                            face.id,
                            face.family,
                            face.face_index,
                            face.data.to_vec(),
                            face.variations,
                            face.synthetic_bold,
                            face.synthetic_italic,
                        )
                    })
                    .collect::<Vec<_>>();
                assert!(
                    cache.entries.is_empty(),
                    "oversized language must bypass admission"
                );
                assert_eq!(cache.bytes, 0);
                assert_eq!(
                    cache.entries.capacity(),
                    0,
                    "reject before reserving cache storage"
                );
                captures.push((format!("{accepted:?}"), faces, contributions));
            }
            assert_eq!(
                captures[0], captures[1],
                "exact cold/revisited glyphs, font inventory and actual requests"
            );
        }
    }

    #[test]
    fn fx195_shaping_cache_replays_contributing_font_identity() {
        let mut traces = Vec::new();
        let mut mismatches = Vec::new();
        for requested in ["Aptos", "Calibri", "Aptos Display"] {
            let mut fonts = FontManager::new_deterministic().unwrap();
            let style = ResolvedRunStyle {
                font_size: Some(20.0),
                latin_typeface: Some(requested.to_owned()),
                ..ResolvedRunStyle::default()
            };
            let mut cache = ShapingCache::default();
            fonts.begin_paragraph_font_trace();
            let first = cache
                .shape(&mut fonts, "same contributing text", &style)
                .unwrap();
            let cold = fonts.finish_paragraph_font_trace().unwrap();
            assert!(!cold.is_empty());
            fonts.begin_paragraph_font_trace();
            let second = cache
                .shape(&mut fonts, "same contributing text", &style)
                .unwrap();
            let warm = fonts.finish_paragraph_font_trace().unwrap();
            assert_eq!(first.glyph_ids, second.glyph_ids);
            assert_eq!(first.advances, second.advances);
            if cold != warm {
                mismatches.push(format!("{requested}: missing exact warm events"));
            }
            let face = fonts.font_data(first.font_id).unwrap();
            traces.push(((face.family, face.face_index, face.data.to_vec()), cold));
        }
        assert_eq!(traces[0].0, traces[1].0, "same actual fallback bytes");
        assert_eq!(traces[0].0, traces[2].0, "same actual fallback bytes");
        // The legacy trace exposes FontId only. Original request distinctions
        // belong to the new request-aware carrier, not this compatible API.
        assert!(mismatches.is_empty(), "{mismatches:?}");
    }

    #[test]
    fn fx195_shaping_cache_cross_request_warm_contributions() {
        for order in [
            ["Aptos", "Calibri", "Aptos Display"],
            ["Aptos Display", "Calibri", "Aptos"],
        ] {
            let mut fonts = FontManager::new_deterministic().unwrap();
            let mut cache = ShapingCache::default();
            let mut captures = Vec::new();
            let mut physical_captures = Vec::new();
            for requested in order.into_iter().chain(order) {
                let paragraph = ResolvedParagraph {
                    runs: vec![ResolvedTextRun::Text {
                        text: "same contributing text".into(),
                        style: ResolvedRunStyle {
                            latin_typeface: Some(requested.into()),
                            font_size: Some(20.0),
                            ..Default::default()
                        },
                    }],
                    ..Default::default()
                };
                fonts.begin_layout();
                fonts.begin_paragraph_font_trace();
                let mut contexts = HashMap::new();
                let provisional =
                    inline_items_cached(&mut fonts, &mut cache, &paragraph, 1, &mut contexts)
                        .unwrap();
                let requests = provisional
                    .iter()
                    .filter(|item| matches!(item, InlineItem::Text(_)))
                    .map(|_| (false, false, None, None))
                    .collect::<Vec<_>>();
                let accepted = shape_multilingual_items_preserving_controls(
                    &mut fonts,
                    provisional,
                    oxml_layout::TextDirection::LeftToRight,
                    &requests,
                    &mut Vec::new(),
                    &contexts,
                )
                .unwrap();
                let events = fonts.finish_paragraph_font_events().unwrap();
                let contributions = events
                    .iter()
                    .filter_map(|event| match event {
                        FontTraceEvent::Contribution(c) => Some(c.clone()),
                        FontTraceEvent::Resolution(_) => None,
                    })
                    .collect::<Vec<_>>();
                assert!(!contributions.is_empty());
                assert!(
                    contributions
                        .iter()
                        .all(|c| c.requested_family.as_deref() == Some(requested))
                );
                let faces = fonts
                    .current_layout_fonts()
                    .iter()
                    .map(|id| {
                        let font = fonts.font_data(*id).unwrap();
                        (
                            font.id,
                            font.family,
                            font.face_index,
                            font.data.to_vec(),
                            font.bold,
                            font.italic,
                            font.variations,
                            font.synthetic_bold,
                            font.synthetic_italic,
                        )
                    })
                    .collect::<Vec<_>>();
                let physical_face = |font: oxml_layout::FontData| {
                    (
                        font.family,
                        font.face_index,
                        font.data.to_vec(),
                        font.bold,
                        font.italic,
                        font.variations,
                        font.synthetic_bold,
                        font.synthetic_italic,
                    )
                };
                let inventory_ids = fonts.current_layout_fonts();
                let physical_faces = inventory_ids
                    .iter()
                    .map(|id| physical_face(fonts.font_data(*id).unwrap()))
                    .collect::<Vec<_>>();
                assert!(!accepted.is_empty());
                let normalized = accepted
                    .iter()
                    .map(|item| {
                        let InlineItem::MultilingualText(segment) = item else {
                            panic!("controlled text must retain final multilingual spans");
                        };
                        let actual_id = segment.font_id();
                        let inventory_index = inventory_ids
                            .iter()
                            .position(|id| *id == actual_id)
                            .expect("every actual span font belongs to the selected inventory");
                        assert_eq!(
                            physical_face(fonts.font_data(actual_id).unwrap()),
                            physical_faces[inventory_index],
                            "actual span font resolves to its selected physical face"
                        );
                        assert!(contributions.iter().any(|c| c.font_id == actual_id));
                        let mut base = segment.base().clone();
                        base.font_id = oxml_layout::FontId(inventory_index as u32);
                        InlineItem::MultilingualText(
                            oxml_layout::MultilingualTextSegment::new(
                                base,
                                segment.logical_index(),
                                segment.language().map(str::to_owned),
                                segment.script(),
                                segment.direction(),
                                segment.bidi_level(),
                                segment.x_advances().to_vec(),
                                segment.y_advances().to_vec(),
                                segment.x_offsets().to_vec(),
                                segment.y_offsets().to_vec(),
                                segment.clusters().to_vec(),
                                segment.break_after(),
                            )
                            .expect("only transient font identity changes in the cloned span"),
                        )
                    })
                    .collect::<Vec<_>>();
                physical_captures.push((format!("{normalized:?}"), physical_faces));
                let identity = (format!("{accepted:?}"), faces, contributions);
                if captures.len() >= 3 {
                    assert_eq!(
                        cache.entries.len(),
                        3,
                        "revisited requests reuse the same cache"
                    );
                    assert_eq!(
                        identity,
                        captures[captures.len() - 3],
                        "exact warm span glyphs, inventory and request"
                    );
                }
                captures.push(identity);
            }
            for (index, capture) in captures[1..3].iter().enumerate() {
                assert_eq!(
                    physical_captures[0],
                    physical_captures[index + 1],
                    "same final glyph geometry and ordered physical fallback inventory"
                );
                assert_ne!(
                    captures[0].2, capture.2,
                    "original requests cannot collapse"
                );
            }
        }
    }

    #[test]
    fn repeated_resolved_runs_reuse_one_shaped_cache_entry() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let style = ResolvedRunStyle {
            font_size: Some(20.0),
            latin_typeface: Some("Carlito".to_owned()),
            ..ResolvedRunStyle::default()
        };
        let mut cache = ShapingCache::default();

        let first = cache
            .shape(&mut fonts, "cached", &style)
            .expect("shape first cached run");
        let second = cache
            .shape(&mut fonts, "cached", &style)
            .expect("reuse cached run");

        assert_eq!(cache.entries.len(), 1);
        assert_eq!(first.glyph_ids, second.glyph_ids);
        assert_eq!(first.advances, second.advances);
    }

    #[test]
    fn stored_line_spacing_reduction_comes_off_the_percentage_not_point_spacing() {
        // PowerPoint, lnSpcReduction="20000" on 20 pt Arial: omitted spacing
        // and 100 % lay lines 19.2 points apart, 150 % lays them 31.2 apart
        // (130 %, not 120 %), and 30 points stays 30 points.
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let mut body = autofit_body(
            ResolvedAutofit::Normal {
                font_scale: Some(1.0),
                line_spacing_reduction: Some(0.2),
            },
            20.0,
            "leading",
        );

        let single =
            stack_text(&mut fonts, test_content_box(200.0), &body).expect("stack reduced single");
        assert_close(single.height, 19.2);

        body.paragraphs[0].line_spacing = Some(ResolvedTextSpacing::Percent(1.5));
        let percentage = stack_text(&mut fonts, test_content_box(200.0), &body)
            .expect("stack reduced percentage");
        assert_close(percentage.height, 31.2);

        body.paragraphs[0].line_spacing = Some(ResolvedTextSpacing::Points(30.0));
        let points =
            stack_text(&mut fonts, test_content_box(200.0), &body).expect("stack exact points");
        assert_close(points.height, 30.0);
    }

    #[test]
    fn first_and_last_paragraph_spacing_obeys_body_property() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let run = || ResolvedTextRun::Text {
            text: "line".to_owned(),
            style: ResolvedRunStyle {
                font_size: Some(20.0),
                latin_typeface: Some("Carlito".to_owned()),
                ..ResolvedRunStyle::default()
            },
        };
        let mut body = ResolvedTextBody {
            paragraphs: vec![
                ResolvedParagraph {
                    line_spacing: Some(ResolvedTextSpacing::Points(20.0)),
                    space_before: Some(ResolvedTextSpacing::Points(7.0)),
                    space_after: Some(ResolvedTextSpacing::Points(3.0)),
                    runs: vec![run()],
                    ..ResolvedParagraph::default()
                },
                ResolvedParagraph {
                    line_spacing: Some(ResolvedTextSpacing::Points(20.0)),
                    space_before: Some(ResolvedTextSpacing::Points(5.0)),
                    space_after: Some(ResolvedTextSpacing::Points(11.0)),
                    runs: vec![run()],
                    ..ResolvedParagraph::default()
                },
            ],
            ..text_body(TextInsets::default())
        };

        let default_edges = stack_text(&mut fonts, test_content_box(200.0), &body)
            .expect("stack default edge spacing");
        assert_close(default_edges.height, 48.0);

        body.space_first_last_paragraph = true;
        let explicit_edges = stack_text(&mut fonts, test_content_box(200.0), &body)
            .expect("stack explicit edge spacing");
        assert_close(explicit_edges.height, 66.0);
    }

    #[test]
    fn shape_autofit_trusts_the_stored_extent() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let body = autofit_body(ResolvedAutofit::Shape, 20.0, "stored shape extent");
        let content = Rect {
            width: 1.0,
            height: 1.0,
            ..test_content_box(0.0)
        };

        let stacked = stack_text(&mut fonts, content, &body).expect("stack shape autofit text");

        assert_close(glyph_runs(&stacked.elements)[0].font_size, 20.0);
        assert!(stacked.width > content.width || stacked.height > content.height);
    }

    #[test]
    fn no_autofit_overflows_without_a_clip() {
        let text = autofit_body(ResolvedAutofit::None, 20.0, "visible overflow");
        let mut text_shape = shape(
            Rect {
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
            },
            ResolvedGeometry::Rectangle,
        );
        text_shape.content = ResolvedContent::Text(text);
        let input = RenderInput {
            slides: vec![ResolvedSlide {
                unresolved_member_count: 0,
                size: (100.0, 50.0),
                background: None,
                shapes: vec![text_shape],
                diagnostics: Vec::new(),
            }],
            media: HashMap::new(),
            fonts: Vec::new(),
            metadata: None,
        };
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");

        let page = layout_slide_with_fonts(&input, 0, &mut fonts).expect("layout visible overflow");
        let PositionedElement::Group(shape_group) = &page.elements[0] else {
            panic!("shape should lower to a group");
        };
        let run = glyph_runs(&shape_group.children)[0];

        assert!(shape_group.clip.is_none());
        assert_close(run.font_size, 20.0);
        assert!(run.advances.iter().sum::<f64>() > 1.0);
    }

    #[test]
    fn bare_normal_autofit_uses_quantised_two_point_five_percent_steps() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let body = autofit_body(
            ResolvedAutofit::Normal {
                font_scale: None,
                line_spacing_reduction: None,
            },
            10.0,
            "quantised ladder",
        );
        let segment = shape_run(
            &mut fonts,
            "quantised ladder",
            first_run_style(&body.paragraphs[0]),
        )
        .expect("shape ladder text");
        let content = Rect {
            width: segment.width * 0.94,
            ..test_content_box(0.0)
        };

        let stacked = stack_text(&mut fonts, content, &body).expect("stack ladder text");

        assert_close(glyph_runs(&stacked.elements)[0].font_size, 9.25);
    }

    #[test]
    fn bare_normal_autofit_measures_text_inside_paragraph_margins() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let mut body = autofit_body(
            ResolvedAutofit::Normal {
                font_scale: None,
                line_spacing_reduction: None,
            },
            10.0,
            "margin fit",
        );
        let segment = shape_run(
            &mut fonts,
            "margin fit",
            first_run_style(&body.paragraphs[0]),
        )
        .expect("shape margin text");
        body.paragraphs[0].left_margin = segment.width * 0.15;
        let content = Rect {
            width: segment.width * 1.1,
            ..test_content_box(0.0)
        };

        let stacked = stack_text(&mut fonts, content, &body).expect("stack margin text");

        assert_close(glyph_runs(&stacked.elements)[0].font_size, 9.5);
    }

    #[test]
    fn bare_normal_autofit_keeps_the_twenty_five_percent_floor_visible() {
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");
        let body = autofit_body(
            ResolvedAutofit::Normal {
                font_scale: None,
                line_spacing_reduction: None,
            },
            20.0,
            "visible at the floor",
        );
        let content = Rect {
            width: 1.0,
            height: 1.0,
            ..test_content_box(0.0)
        };

        let stacked = stack_text(&mut fonts, content, &body).expect("stack floor text");
        let run = glyph_runs(&stacked.elements)[0];

        assert_close(run.font_size, 5.0);
        assert!(stacked.width > content.width || stacked.height > content.height);
    }

    #[test]
    fn rich_powerpoint_text_path_keeps_visual_runs_and_logical_text_together() {
        let source = "abc العربية कि ภาษาไทย 〈中〉、 אבג 123";
        let body = ResolvedTextBody {
            paragraphs: vec![ResolvedParagraph {
                runs: vec![ResolvedTextRun::Text {
                    text: source.to_owned(),
                    style: ResolvedRunStyle {
                        font_size: Some(18.0),
                        latin_typeface: Some("Carlito".to_owned()),
                        ..ResolvedRunStyle::default()
                    },
                }],
                ..ResolvedParagraph::default()
            }],
            ..text_body(TextInsets::default())
        };
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");

        let stacked = stack_text(&mut fonts, test_content_box(2_000.0), &body)
            .expect("shape multilingual PowerPoint text");
        let runs = stacked
            .elements
            .iter()
            .filter_map(|element| match element {
                PositionedElement::MultilingualText(run) => Some(run),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(runs.len() > 4);
        assert!(runs.iter().all(|run| {
            run.glyph_ids.len() == run.x_advances.len()
                && run.glyph_ids.len() == run.x_offsets.len()
                && run.glyph_ids.len() == run.y_offsets.len()
        }));
        assert!(runs.iter().any(|run| run.bidi_level % 2 == 1));
        let logical_indices = runs.iter().map(|run| run.logical_index).collect::<Vec<_>>();
        let logical = runs
            .iter()
            .map(|run| run.logical_text.as_str())
            .collect::<String>();

        assert_eq!(logical, source);
        assert_eq!(
            logical_indices,
            (0..logical_indices.len()).collect::<Vec<_>>()
        );
        assert!(
            runs.windows(2)
                .any(|pair| pair[0].origin.x > pair[1].origin.x),
            "logical extraction order must coexist with visual positions"
        );

        let layout = oxml_layout::LayoutResult::new(
            vec![oxml_layout::PageFrame::new(1, 2_000.0, 200.0, stacked.elements).into()],
            fonts.all_font_data(),
            None,
            Vec::new(),
        );
        let pdf = oxml_pdf::render_to_pdf(&layout);
        let png = oxml_pdf::render_page_to_png(&layout, 0, 72.0)
            .expect("raster consumes positioned multilingual runs");
        assert!(pdf.starts_with(b"%PDF"));
        assert!(png.starts_with(b"\x89PNG"));
    }

    #[test]
    fn explicit_rtl_styled_runs_around_a_forced_break_keep_logical_order() {
        let body = ResolvedTextBody {
            paragraphs: vec![ResolvedParagraph {
                runs: vec![
                    ResolvedTextRun::Text {
                        text: "אב".to_owned(),
                        style: ResolvedRunStyle {
                            font_size: Some(18.0),
                            ..ResolvedRunStyle::default()
                        },
                    },
                    ResolvedTextRun::Text {
                        text: "גד".to_owned(),
                        style: ResolvedRunStyle {
                            font_size: Some(18.0),
                            italic: true,
                            ..ResolvedRunStyle::default()
                        },
                    },
                    ResolvedTextRun::Break,
                    ResolvedTextRun::Text {
                        text: "הו".to_owned(),
                        style: ResolvedRunStyle {
                            font_size: Some(18.0),
                            ..ResolvedRunStyle::default()
                        },
                    },
                    ResolvedTextRun::Text {
                        text: "זח".to_owned(),
                        style: ResolvedRunStyle {
                            font_size: Some(18.0),
                            bold: true,
                            ..ResolvedRunStyle::default()
                        },
                    },
                ],
                ..ResolvedParagraph::default()
            }],
            ..text_body(TextInsets::default())
        };
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");

        let stacked = stack_text_for_page_with_directions(
            &mut fonts,
            test_content_box(2_000.0),
            &body,
            1,
            &[oxml_layout::TextDirection::RightToLeft],
        )
        .unwrap();
        let runs = stacked
            .elements
            .iter()
            .filter_map(|element| match element {
                PositionedElement::MultilingualText(run) => Some(run),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            runs.iter()
                .map(|run| { (run.logical_text.as_str(), run.logical_index, run.bidi_level,) })
                .collect::<Vec<_>>(),
            [("אב", 0, 1), ("גד", 1, 1), ("הו", 2, 1), ("זח", 3, 1)]
        );
        assert!(runs[0].origin.x > runs[1].origin.x);
        assert!(runs[2].origin.x > runs[3].origin.x);
        assert!(runs[0].origin.y < runs[2].origin.y);
    }

    #[test]
    fn stacked_lines_move_with_their_anchored_runs_and_leave_the_marker_out_of_their_text() {
        let rtl_run = |text: &str| ResolvedTextRun::Text {
            text: text.to_owned(),
            style: ResolvedRunStyle {
                font_size: Some(18.0),
                ..ResolvedRunStyle::default()
            },
        };
        let body = ResolvedTextBody {
            anchor: TextAnchor::Bottom,
            paragraphs: vec![
                bullet_paragraph(
                    0,
                    Some(ResolvedBullet::Character {
                        character: "*".to_owned(),
                        font: None,
                        color: None,
                        size: None,
                    }),
                    "first",
                ),
                ResolvedParagraph {
                    runs: vec![rtl_run("אב"), rtl_run("גד")],
                    ..ResolvedParagraph::default()
                },
            ],
            ..text_body(TextInsets::default())
        };
        let content = test_content_box(2_000.0);
        let mut fonts = FontManager::new_deterministic().expect("deterministic fonts");

        let stacked = stack_text_for_page_with_directions(
            &mut fonts,
            content,
            &body,
            1,
            &[
                oxml_layout::TextDirection::Auto,
                oxml_layout::TextDirection::RightToLeft,
            ],
        )
        .expect("stack bullet and right-to-left paragraphs");

        assert_eq!(
            stacked
                .lines
                .iter()
                .map(|line| (line.paragraph_index, line.text.as_str()))
                .collect::<Vec<_>>(),
            [(0, "first"), (1, "אבגד")]
        );
        let marker = glyph_runs(&stacked.elements)[0];
        assert_eq!(marker.text, "*");
        assert_close(stacked.lines[0].bounds.x, marker.origin.x);
        assert_close(stacked.lines[0].baseline, marker.origin.y);
        assert_close(stacked.lines[0].font_size, 12.0);
        let last = &stacked.lines[1];
        assert_close(last.font_size, 18.0);
        assert_close(
            last.bounds.y + last.bounds.height,
            content.y + content.height,
        );
        let rich_baselines = stacked
            .elements
            .iter()
            .filter_map(|element| match element {
                PositionedElement::MultilingualText(run) => Some(run.origin.y),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(rich_baselines.len(), 2);
        for baseline in rich_baselines {
            assert_close(baseline, last.baseline);
        }
    }

    fn bullet_paragraph(
        level: u8,
        bullet: Option<ResolvedBullet>,
        text: &str,
    ) -> ResolvedParagraph {
        ResolvedParagraph {
            level,
            left_margin: 10.0,
            indent: -5.0,
            bullet,
            runs: vec![ResolvedTextRun::Text {
                text: text.to_owned(),
                style: ResolvedRunStyle {
                    font_size: Some(12.0),
                    latin_typeface: Some("Carlito".to_owned()),
                    ..ResolvedRunStyle::default()
                },
            }],
            ..ResolvedParagraph::default()
        }
    }

    fn autofit_body(autofit: ResolvedAutofit, font_size: f64, text: &str) -> ResolvedTextBody {
        ResolvedTextBody {
            wrap: false,
            autofit,
            paragraphs: vec![ResolvedParagraph {
                runs: vec![ResolvedTextRun::Text {
                    text: text.to_owned(),
                    style: ResolvedRunStyle {
                        font_size: Some(font_size),
                        latin_typeface: Some("Carlito".to_owned()),
                        ..ResolvedRunStyle::default()
                    },
                }],
                ..ResolvedParagraph::default()
            }],
            ..text_body(TextInsets::default())
        }
    }

    fn test_content_box(width: f64) -> Rect {
        Rect {
            x: 0.0,
            y: 0.0,
            width,
            height: 400.0,
        }
    }

    fn glyph_runs(elements: &[oxml_layout::PositionedElement]) -> Vec<&oxml_layout::GlyphRun> {
        elements
            .iter()
            .filter_map(|element| match element {
                oxml_layout::PositionedElement::Text(run) => Some(run),
                _ => None,
            })
            .collect()
    }

    fn distinct_baselines(runs: &[&oxml_layout::GlyphRun]) -> Vec<f64> {
        let mut baselines = Vec::new();
        for run in runs {
            if baselines
                .last()
                .is_none_or(|baseline: &f64| (run.origin.y - baseline).abs() > 0.001)
            {
                baselines.push(run.origin.y);
            }
        }
        baselines
    }

    #[test]
    fn fx207_physical_extent_uses_selected_face_descent_markers_and_shifted_runs() {
        let mono = include_bytes!("../../oxml-layout/fonts/LiberationMono-Regular.ttf").as_slice();
        let serif =
            include_bytes!("../../oxml-layout/fonts/LiberationSerif-Regular.ttf").as_slice();
        // Decode static hhea/head metrics independently from the original SFNT.
        fn descent(bytes: &[u8], size: f64) -> f64 {
            let table = |tag: &[u8]| {
                let count = u16::from_be_bytes(bytes[4..6].try_into().unwrap()) as usize;
                let record = (0..count)
                    .map(|i| &bytes[12 + i * 16..28 + i * 16])
                    .find(|r| &r[..4] == tag)
                    .unwrap();
                u32::from_be_bytes(record[8..12].try_into().unwrap()) as usize
            };
            let hhea = table(b"hhea");
            let head = table(b"head");
            let os2 = table(b"OS/2");
            assert_eq!(
                u16::from_be_bytes(bytes[os2 + 62..os2 + 64].try_into().unwrap()) & 128,
                0,
                "this source control deliberately uses static hhea metrics"
            );
            let units = u16::from_be_bytes(bytes[head + 18..head + 20].try_into().unwrap());
            -f64::from(i16::from_be_bytes(
                bytes[hhea + 6..hhea + 8].try_into().unwrap(),
            )) * size
                / f64::from(units)
        }
        let mut fonts = FontManager::new_deterministic().unwrap();
        let mut paragraph = bullet_paragraph(
            0,
            Some(ResolvedBullet::Character {
                character: "M".to_owned(),
                font: Some("Liberation Mono".to_owned()),
                color: None,
                size: Some(ResolvedBulletSize::Points(32.0)),
            }),
            "body",
        );
        paragraph.left_margin = 50.0;
        paragraph.indent = -45.0;
        paragraph.runs = vec![
            ResolvedTextRun::Text {
                text: "Mixed ".to_owned(),
                style: ResolvedRunStyle {
                    font_size: Some(11.0),
                    latin_typeface: Some("Liberation Mono".to_owned()),
                    baseline: Some(-0.3),
                    ..Default::default()
                },
            },
            ResolvedTextRun::Text {
                text: "face".to_owned(),
                style: ResolvedRunStyle {
                    font_size: Some(24.0),
                    latin_typeface: Some("Liberation Serif".to_owned()),
                    ..Default::default()
                },
            },
        ];
        let body = ResolvedTextBody {
            paragraphs: vec![paragraph],
            ..text_body(TextInsets::default())
        };
        let content = test_content_box(400.0);
        let stacked = stack_text(&mut fonts, content, &body).unwrap();
        let mut expected = content.y;
        let mut marker = false;
        let mut shifted = false;
        for element in &stacked.elements {
            let PositionedElement::Text(run) = element else {
                continue;
            };
            let selected = fonts.font_data(run.font_id).unwrap();
            let original = if selected.family == "Liberation Mono" {
                mono
            } else {
                assert_eq!(selected.family, "Liberation Serif");
                serif
            };
            assert_eq!(selected.data.as_ref(), original);
            expected = expected.max(run.origin.y + descent(original, run.font_size));
            marker |= run.text == "M" && run.font_size == 32.0;
            shifted |= run.text.contains("Mixed") && run.origin.y > stacked.lines[0].baseline;
        }
        assert!(marker && shifted);
        let (height, fits) = stacked
            .authoring_extent(&mut fonts, &body, content)
            .unwrap();
        assert_close(height, expected - content.y);
        assert!(fits);
        assert!(
            (height - stacked.height).abs() > 0.01,
            "physical and line-advance facts must stay distinct"
        );
        let insufficient = Rect {
            height: height - 0.02,
            ..content
        };
        assert!(
            !stacked
                .authoring_extent(&mut fonts, &body, insufficient)
                .unwrap()
                .1
        );
    }

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 0.001,
            "expected {expected}, got {actual}"
        );
    }
    #[test]
    #[ignore = "required ROOT physical-font contour oracle, explicit named --ignored gate"]
    fn fx206_caps_paint_matches_independent_hb_contours_with_negative_controls() {
        let oracle = std::env::var("F206_CONTOUR_ORACLE").expect("admitted contour helper path");
        let python = std::env::var("FONT_ORACLE_PYTHON").expect("pinned qualified interpreter");
        let hb = std::env::var("HB_SHAPE").expect("pinned HarfBuzz path");
        let artifacts = std::path::PathBuf::from(std::env::var("F206_ORACLE_OUTPUT").unwrap());
        assert!(
            artifacts.is_dir(),
            "ROOT-created fresh owned evidence directory"
        );
        for case in ["smcp-default", "smcp-turkish", "fallback"] {
            let prefix = if case == "fallback" {
                "F206_FALLBACK"
            } else {
                "F206_SMCP"
            };
            let path = std::env::var(format!("{prefix}_FONT")).unwrap();
            let sha = std::env::var(format!("{prefix}_SHA256")).unwrap();
            let source = std::fs::read(&path).unwrap();
            let mut fonts = FontManager::new_deterministic().unwrap();
            let alias = "F206 physical contour control";
            fonts.load_additional_fonts(&[oxml_layout::FontFile {
                family: alias.to_owned(),
                data: source.clone(),
            }]);
            let (logical, language) = match case {
                "smcp-default" => ("office", "en"),
                "smcp-turkish" => ("i", "tr"),
                _ => ("aBß", "en"),
            };
            let style = ResolvedRunStyle {
                font_size: Some(48.0),
                latin_typeface: Some(alias.to_owned()),
                ..ResolvedRunStyle::default()
            };
            let seed = ShapingCache::default()
                .shape(&mut fonts, logical, &style)
                .unwrap();
            assert_eq!(
                fonts.font_data(seed.font_id).unwrap().data.as_ref(),
                source.as_slice(),
                "actual selected physical face"
            );
            let (spans, diagnostics) = fonts
                .shape_multilingual_paragraph_with_caps(
                    vec![(seed, Some(language.to_owned()))],
                    oxml_layout::TextDirection::LeftToRight,
                    false,
                    &[(true, false)],
                )
                .unwrap();
            assert_eq!(
                spans.iter().map(|span| span.text()).collect::<String>(),
                logical
            );
            assert_eq!(
                diagnostics.is_empty(),
                case != "fallback",
                "truthful feature versus fallback route"
            );
            let output = std::process::Command::new(&python)
                .args([
                    &oracle,
                    "--font",
                    &path,
                    "--font-sha256",
                    &sha,
                    "--hb-shape",
                    &hb,
                    "--case",
                    case,
                    "--output",
                    artifacts.join(format!("{case}.json")).to_str().unwrap(),
                    "--execute-with-root-oracle-lease",
                ])
                .output()
                .unwrap();
            std::fs::write(artifacts.join(format!("{case}.stdout")), &output.stdout).unwrap();
            std::fs::write(artifacts.join(format!("{case}.stderr")), &output.stderr).unwrap();
            assert!(
                output.status.success(),
                "independent oracle prerequisite: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            let stdout = String::from_utf8(output.stdout).unwrap();
            let mut expected_gids = Vec::new();
            let mut expected_advance = None;
            let mut images = Vec::new();
            let mut builder = tiny_skia::PathBuilder::new();
            for row in stdout.lines() {
                let fields = row.split_whitespace().collect::<Vec<_>>();
                let number = |i: usize| fields[i].parse::<f32>().unwrap();
                match fields[0] {
                    "EXPECTED" | "NEGATIVE" => builder = tiny_skia::PathBuilder::new(),
                    "ADV" if images.is_empty() => {
                        expected_advance = Some(fields[1].parse::<f64>().unwrap())
                    }
                    "GIDS" if images.is_empty() => {
                        expected_gids = fields[1]
                            .split(',')
                            .map(|v| v.parse::<u16>().unwrap())
                            .collect()
                    }
                    "ADV" | "GIDS" => {}
                    "M" => builder.move_to(number(1), number(2)),
                    "L" => builder.line_to(number(1), number(2)),
                    "Q" => builder.quad_to(number(1), number(2), number(3), number(4)),
                    "C" => builder.cubic_to(
                        number(1),
                        number(2),
                        number(3),
                        number(4),
                        number(5),
                        number(6),
                    ),
                    "Z" => builder.close(),
                    "END" => {
                        let path = std::mem::take(&mut builder)
                            .finish()
                            .expect("nonempty independent ink");
                        let mut pixmap = tiny_skia::Pixmap::new(960, 240).unwrap();
                        pixmap.fill(tiny_skia::Color::WHITE);
                        let mut paint = tiny_skia::Paint::default();
                        paint.set_color_rgba8(0, 0, 0, 255);
                        pixmap.fill_path(
                            &path,
                            &paint,
                            tiny_skia::FillRule::Winding,
                            tiny_skia::Transform::from_scale(2.0, 2.0),
                            None,
                        );
                        images.push(pixmap);
                    }
                    _ => panic!("unexpected oracle record"),
                }
            }
            assert_eq!(images.len(), 2);
            assert_eq!(
                spans
                    .iter()
                    .flat_map(|span| span.glyph_ids().iter().copied())
                    .collect::<Vec<_>>(),
                expected_gids
            );
            let width = spans.iter().map(|span| span.width()).sum::<f64>();
            assert!(
                (width - expected_advance.unwrap()).abs() < 1e-7,
                "independently measured physical advance"
            );
            let mut elements = Vec::new();
            let mut x = 20.0;
            for span in &spans {
                emit_multilingual_segment(span, x, 80.0, 60.0, &mut elements);
                x += span.width();
            }
            let ids = spans
                .iter()
                .map(|span| span.font_id())
                .collect::<std::collections::BTreeSet<_>>();
            let data = ids
                .into_iter()
                .map(|id| fonts.font_data(id).unwrap())
                .collect();
            let result = oxml_layout::LayoutResult::new(
                vec![std::sync::Arc::new(oxml_layout::PageFrame::new(
                    1, 480.0, 120.0, elements,
                ))],
                data,
                None,
                Vec::new(),
            );
            let png = oxml_pdf::render_page_to_png(&result, 0, 144.0).unwrap();
            std::fs::write(artifacts.join(format!("{case}-actual.png")), &png).unwrap();
            images[0]
                .save_png(artifacts.join(format!("{case}-expected.png")))
                .unwrap();
            images[1]
                .save_png(artifacts.join(format!("{case}-negative.png")))
                .unwrap();
            let actual = tiny_skia::Pixmap::decode_png(&png).unwrap();
            assert_eq!((actual.width(), actual.height()), (960, 240));
            let ssim = |left: &tiny_skia::Pixmap, right: &tiny_skia::Pixmap| {
                let values = |p: &tiny_skia::Pixmap| {
                    p.pixels()
                        .iter()
                        .map(|p| {
                            (f64::from(p.red()) + f64::from(p.green()) + f64::from(p.blue())) / 3.0
                        })
                        .collect::<Vec<_>>()
                };
                let a = values(left);
                let b = values(right);
                let n = a.len() as f64;
                let ma = a.iter().sum::<f64>() / n;
                let mb = b.iter().sum::<f64>() / n;
                let va = a.iter().map(|v| (v - ma).powi(2)).sum::<f64>() / (n - 1.0);
                let vb = b.iter().map(|v| (v - mb).powi(2)).sum::<f64>() / (n - 1.0);
                let cov = a
                    .iter()
                    .zip(&b)
                    .map(|(a, b)| (a - ma) * (b - mb))
                    .sum::<f64>()
                    / (n - 1.0);
                ((2.0 * ma * mb + 6.5025) * (2.0 * cov + 58.5225))
                    / ((ma * ma + mb * mb + 6.5025) * (va + vb + 58.5225))
            };
            let positive = ssim(&actual, &images[0]);
            let negative = ssim(&actual, &images[1]);
            assert!(
                positive >= 0.99,
                "independent contour paint {case}: {positive}"
            );
            assert!(
                negative < 0.99,
                "features-off/ordinary paint must distinguish {case}: {negative}"
            );
        }
    }

    #[test]
    fn native_profile_percent_spacing_keeps_first_last_and_anchor_policy() {
        let mut fonts = FontManager::new_deterministic().unwrap();
        let p = ResolvedParagraph {
            line_spacing: Some(ResolvedTextSpacing::Percent(0.9)),
            space_before: Some(ResolvedTextSpacing::Percent(0.0)),
            space_after: Some(ResolvedTextSpacing::Percent(0.35)),
            runs: vec![ResolvedTextRun::Text {
                text: "line".to_owned(),
                style: ResolvedRunStyle {
                    font_size: Some(20.0),
                    latin_typeface: Some("Carlito".to_owned()),
                    ..ResolvedRunStyle::default()
                },
            }],
            ..ResolvedParagraph::default()
        };
        let mut body = ResolvedTextBody {
            paragraphs: vec![p.clone(), p],
            ..text_body(TextInsets::default())
        };
        let natural = stack_text(&mut fonts, test_content_box(200.0), &body).unwrap();
        assert_close(natural.height, 51.6);
        body.space_first_last_paragraph = true;
        let edges = stack_text(&mut fonts, test_content_box(200.0), &body).unwrap();
        assert_close(edges.height - natural.height, 8.4);
        body.paragraphs.truncate(1);
        body.space_first_last_paragraph = false;
        body.anchor = TextAnchor::Center;
        let single = stack_text(&mut fonts, test_content_box(200.0), &body).unwrap();
        assert_close(single.height, 21.6);
        body.paragraphs[0].space_after = None;
        let no_after = stack_text(&mut fonts, test_content_box(200.0), &body).unwrap();
        assert_eq!(single.lines[0].baseline, no_after.lines[0].baseline);
    }

    #[test]
    fn native_scaled_custom_text_rectangle_consumes_legacy_insets_once() {
        let s = shape(
            Rect {
                x: 37.0,
                y: 41.0,
                width: 45.0,
                height: 30.0,
            },
            ResolvedGeometry::Custom {
                paths: Vec::new(),
                text_rect: Some(Rect {
                    x: 3.0,
                    y: 2.25,
                    width: 34.5,
                    height: 24.0,
                }),
            },
        );
        let text = text_body(TextInsets {
            left: 1.0,
            top: 3.0,
            right: 2.0,
            bottom: 4.0,
        });
        assert_eq!(
            content_box(&s, &text),
            Rect {
                x: 4.0,
                y: 5.25,
                width: 31.5,
                height: 17.0
            }
        );
    }
}
