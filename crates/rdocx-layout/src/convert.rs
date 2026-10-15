//! Conversion from WordprocessingML flow values to shared layout values.

use oxml_layout::{
    Align, FontManager, FontMetrics, LayoutLine, LineBreakParams, LineItem, LineSpacing, TabAlign,
    TabLeader, TabStop, TextDirection, Underline,
};
use rdocx_oxml::borders::CT_TabStop;
use rdocx_oxml::properties::CT_PPr;
use rdocx_oxml::shared::{ST_Jc, ST_TabJc, ST_TabLeader, ST_Underline};
use rdocx_oxml::units::Twips;

pub(crate) fn alignment(value: Option<ST_Jc>) -> Option<Align> {
    value.map(|value| match value {
        ST_Jc::Start | ST_Jc::Left => Align::Start,
        ST_Jc::Center => Align::Center,
        ST_Jc::End | ST_Jc::Right => Align::End,
        ST_Jc::Both => Align::Justify,
        ST_Jc::Distribute => Align::Distribute,
    })
}

pub(crate) fn alignment_for_direction(
    value: Option<ST_Jc>,
    direction: TextDirection,
) -> Option<Align> {
    value.map(|value| match (value, direction) {
        (ST_Jc::Start, TextDirection::RightToLeft) => Align::End,
        (ST_Jc::End, TextDirection::RightToLeft) => Align::Start,
        (ST_Jc::Start | ST_Jc::Left, _) => Align::Start,
        (ST_Jc::End | ST_Jc::Right, _) => Align::End,
        (ST_Jc::Center, _) => Align::Center,
        (ST_Jc::Both, _) => Align::Justify,
        (ST_Jc::Distribute, _) => Align::Distribute,
    })
}

pub(crate) fn tab_stops(values: &[CT_TabStop]) -> Vec<TabStop> {
    values
        .iter()
        .filter_map(|value| {
            let align = match value.val {
                ST_TabJc::Left | ST_TabJc::Start => TabAlign::Left,
                ST_TabJc::Center => TabAlign::Center,
                ST_TabJc::Right | ST_TabJc::End => TabAlign::Right,
                ST_TabJc::Decimal => TabAlign::Decimal,
                // A list tab, which Word treats as a left stop.
                ST_TabJc::Num | ST_TabJc::List => TabAlign::Left,
                ST_TabJc::Bar => TabAlign::Bar,
                ST_TabJc::Clear => return None,
            };
            let leader = value.leader.map(|leader| match leader {
                ST_TabLeader::None => TabLeader::None,
                ST_TabLeader::Dot => TabLeader::Dot,
                ST_TabLeader::Hyphen => TabLeader::Hyphen,
                ST_TabLeader::Underscore => TabLeader::Underscore,
                ST_TabLeader::Heavy => TabLeader::Heavy,
                ST_TabLeader::MiddleDot => TabLeader::MiddleDot,
            });
            Some(TabStop {
                pos_pt: value.pos.to_pt(),
                align,
                leader,
            })
        })
        .collect()
}

pub(crate) fn underline(value: Option<ST_Underline>) -> Option<Underline> {
    value.and_then(|value| match value {
        ST_Underline::None => None,
        ST_Underline::Single => Some(Underline::Single),
        ST_Underline::Words => Some(Underline::Words),
        ST_Underline::Double => Some(Underline::Double),
        ST_Underline::Thick => Some(Underline::Thick),
        ST_Underline::Dotted => Some(Underline::Dotted),
        ST_Underline::Dash => Some(Underline::Dash),
        ST_Underline::DotDash => Some(Underline::DotDash),
        ST_Underline::DotDotDash => Some(Underline::DotDotDash),
        ST_Underline::Wave => Some(Underline::Wave),
    })
}

pub(crate) fn line_spacing(properties: &CT_PPr) -> LineSpacing {
    match (properties.line_spacing, properties.line_rule.as_deref()) {
        (Some(spacing), Some("exact")) => LineSpacing::Exact(spacing.to_pt()),
        (Some(spacing), Some("atLeast")) => LineSpacing::AtLeast(spacing.to_pt()),
        (Some(spacing), _) => LineSpacing::Multiple(spacing.0 as f64 / 240.0),
        (None, _) => LineSpacing::Single,
    }
}

/// Points between implicit tab stops, from the document `w:defaultTabStop`.
///
/// An absent or zero setting reproduces Word's half-inch default exactly.
pub(crate) fn default_tab_interval_pt(default_tab_stop: Option<Twips>) -> f64 {
    default_tab_stop
        .filter(|value| value.0 > 0)
        .map_or(36.0, |value| value.to_pt())
}

/// How close to a whole grid row a line may be and still take that row.
///
/// Line height is a sum of font metrics, so a line that is arithmetically a
/// whole number of grid rows can land just above one. Without this it would
/// take an extra row. The value is an absolute tolerance on the row count,
/// which is a small number for any real pitch and height.
pub(crate) const GRID_ROW_TOLERANCE: f64 = 1e-9;

pub(crate) fn line_break_params(
    properties: &CT_PPr,
    available_width: f64,
    default_tab_stop: Option<Twips>,
) -> LineBreakParams {
    LineBreakParams {
        line_prefix_widths: Vec::new(),
        line_suffix_widths: Vec::new(),
        available_width,
        ind_left: properties.ind_left.map_or(0.0, |value| value.to_pt()),
        ind_right: properties.ind_right.map_or(0.0, |value| value.to_pt()),
        ind_first_line: properties.ind_first_line.map_or(0.0, |value| value.to_pt()),
        ind_hanging: properties.ind_hanging.map_or(0.0, |value| value.to_pt()),
        tab_stops: properties
            .tabs
            .as_ref()
            .map_or_else(Vec::new, |tabs| tab_stops(&tabs.tabs)),
        line_spacing: line_spacing(properties),
        jc: alignment(properties.jc),
        wrap: true,
        default_tab_interval_pt: default_tab_interval_pt(default_tab_stop),
        clamp_tabs_past_margin: false,
    }
}

/// Restore Word line advance, optionally on a section character grid.
///
/// Word measures a line's text on the Windows metrics of each run's font
/// ([`FontManager::word_line_metrics`]) and puts the external leading above
/// the ascent, so a single line of Calibri is 2500/2048 em with its baseline
/// 1950/2048 em down, and a single line of Arial is 2355/2048 em. An inline
/// picture or chart stands on the baseline. Proportional (`auto`) spacing
/// adds `line / 240 - 1` times the height of the line's text, not of the
/// whole line, so a 400 point picture at 1.1 lines keeps its 400 points plus
/// a tenth of a text line. A line with no text takes that height from its
/// paragraph mark, `paragraph_mark`, the Word metrics of the mark's font,
/// is never shorter than one line of it, and adds none when the mark's font
/// did not resolve. A line with nothing
/// on it, left by a line break, is a line of the break's font, which is the
/// nearest text before it (after it when there is none before), and of the
/// paragraph mark when the paragraph holds no text. Only a line with none of
/// these falls back to 12 points. Exact spacing keeps the metrics the line
/// breaker measured, since its height and baseline are the author's.
///
/// `grid_line_pitch_pt` is `Some` only for a `lines`, `linesAndChars` or
/// `snapToChars` grid on a paragraph that has not opted out with
/// `w:snapToGrid w:val="0"`. `None` is the ungridded path.
pub(crate) fn restore_word_line_heights(
    lines: &mut [LayoutLine],
    spacing: LineSpacing,
    grid_line_pitch_pt: Option<f64>,
    fm: &FontManager,
    paragraph_mark: Option<FontMetrics>,
) {
    let mark = paragraph_mark.map(|mark| (mark.line_gap + mark.ascent, mark.descent));
    let carriers = lines
        .iter()
        .map(|line| last_text_extent(line, fm))
        .collect::<Vec<_>>();
    for (index, line) in lines.iter_mut().enumerate() {
        let empty_line = carriers[..index]
            .iter()
            .rev()
            .chain(&carriers[index + 1..])
            .find_map(|carrier| *carrier)
            .or(mark);
        let text_height = match spacing {
            LineSpacing::Exact(_) => 0.0,
            _ => measure_word_line(line, fm, mark, empty_line),
        };
        let natural = line.ascent + line.descent;
        let (natural, text_height) = if natural < 1.0 {
            (12.0, 12.0)
        } else {
            (natural, text_height)
        };
        line.line_gap = 0.0;
        line.height = match spacing {
            LineSpacing::Exact(points) => points,
            LineSpacing::AtLeast(points) => natural.max(points),
            LineSpacing::Multiple(factor) => natural + (factor - 1.0) * text_height,
            LineSpacing::Single => natural,
        };
        // A gridded line takes whole grid rows. An exact `w:lineRule` is an
        // author's absolute height and stays absolute, which is what Word
        // does with the two together. The pitch is guarded here as well as at
        // the caller, because a zero would turn the height into a `NaN` that
        // then spreads silently through pagination. The tolerance keeps a
        // height that is an exact multiple of the pitch on its own row rather
        // than pushing it onto the next one over a representation error.
        if let Some(pitch) = grid_line_pitch_pt
            && pitch > 0.0
            && !matches!(spacing, LineSpacing::Exact(_))
        {
            let rows = (line.height / pitch - GRID_ROW_TOLERANCE).ceil().max(1.0);
            line.height = pitch * rows;
        }
    }
}

/// The Word extent of the last text run on `line`, the run a line break
/// that ends it would carry.
fn last_text_extent(line: &LayoutLine, fm: &FontManager) -> Option<(f64, f64)> {
    line.items
        .iter()
        .rev()
        .find_map(|item| match word_line_extent(item, fm) {
            Some(WordLineExtent::Text { ascent, descent }) => Some((ascent, descent)),
            _ => None,
        })
}

/// Measure `line` the way Word does, and return the height of its text.
///
/// The line's ascent and descent become the greater of its text and of the
/// inline objects standing on its baseline. A line of objects only takes its
/// text height from `mark`, and a line with neither stands on `empty_line`,
/// or keeps what the line breaker measured when that is `None`.
fn measure_word_line(
    line: &mut LayoutLine,
    fm: &FontManager,
    mark: Option<(f64, f64)>,
    empty_line: Option<(f64, f64)>,
) -> f64 {
    let mut text: Option<(f64, f64)> = None;
    let mut object_ascent: Option<f64> = None;
    for item in &line.items {
        match word_line_extent(item, fm) {
            Some(WordLineExtent::Text { ascent, descent }) => {
                let (text_ascent, text_descent) = text.unwrap_or((0.0, 0.0));
                text = Some((text_ascent.max(ascent), text_descent.max(descent)));
            }
            Some(WordLineExtent::Object { height }) => {
                object_ascent = Some(object_ascent.unwrap_or(0.0).max(height));
            }
            None => {}
        }
    }
    match (text, object_ascent) {
        (None, None) => {
            if let Some((ascent, descent)) = empty_line {
                line.ascent = ascent;
                line.descent = descent;
            }
            line.ascent + line.descent
        }
        (Some((ascent, descent)), object_ascent) => {
            line.ascent = ascent.max(object_ascent.unwrap_or(0.0));
            line.descent = descent;
            ascent + descent
        }
        (None, Some(object_ascent)) => {
            // Word gives a line of objects at least the height of its
            // paragraph mark, with the objects on the bottom of the line.
            let mark_height = mark.map_or(0.0, |(ascent, descent)| ascent + descent);
            line.ascent = object_ascent.max(mark_height);
            line.descent = 0.0;
            mark_height
        }
    }
}

/// How one line item stands on a Word line.
enum WordLineExtent {
    /// Text, whose height proportional spacing scales.
    Text { ascent: f64, descent: f64 },
    /// An inline object resting on the baseline, which it does not scale.
    Object { height: f64 },
}

/// The extent of one line item on a Word line, or `None` when it takes none.
///
/// A text run takes the Word metrics of its font, with the external leading
/// counted into its ascent, which is where Word puts it. A run the engine
/// gave no extent, such as a bookmark target, stays without one, and a run
/// whose font the manager does not hold keeps the extent it was shaped
/// with. A group with a text baseline (an equation, a ruby or an East Asian
/// layout) keeps the extent it was measured with and counts as text, as it
/// always has. A picture, and a group drawn from its top such as a chart, is
/// an object.
fn word_line_extent(item: &LineItem, fm: &FontManager) -> Option<WordLineExtent> {
    let segment = match item {
        LineItem::Text(segment) | LineItem::Marker(segment) => segment,
        LineItem::MultilingualText(segment) => segment.base(),
        LineItem::Image { height, .. } => return Some(WordLineExtent::Object { height: *height }),
        LineItem::Group {
            height, baseline, ..
        } => {
            let baseline = baseline
                .filter(|baseline| baseline.is_finite() && height.is_finite())
                .map(|baseline| baseline.clamp(0.0, height.max(0.0)));
            return Some(match baseline {
                Some(baseline) => WordLineExtent::Text {
                    ascent: baseline,
                    descent: height - baseline,
                },
                None => WordLineExtent::Object { height: *height },
            });
        }
        LineItem::Figure { item, .. } => return word_line_extent(item, fm),
        _ => return None,
    };
    if segment.ascent + segment.descent <= 0.0 {
        return None;
    }
    let (ascent, descent) = fm
        .word_line_metrics(segment.font_id, segment.font_size)
        .map_or((segment.ascent, segment.descent), |metrics| {
            (metrics.line_gap + metrics.ascent, metrics.descent)
        });
    Some(WordLineExtent::Text { ascent, descent })
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxml_layout::{Align, LineSpacing, TabAlign, TabLeader, Underline};

    #[test]
    fn every_word_alignment_maps_to_the_shared_alignment() {
        let cases = [
            (ST_Jc::Start, Align::Start),
            (ST_Jc::Left, Align::Start),
            (ST_Jc::Center, Align::Center),
            (ST_Jc::End, Align::End),
            (ST_Jc::Right, Align::End),
            (ST_Jc::Both, Align::Justify),
            (ST_Jc::Distribute, Align::Distribute),
        ];
        for (word, shared) in cases {
            assert_eq!(alignment(Some(word)), Some(shared));
        }
        assert_eq!(alignment(None), None);
    }

    #[test]
    fn logical_alignment_uses_the_paragraph_base_direction() {
        assert_eq!(
            alignment_for_direction(Some(ST_Jc::Start), TextDirection::RightToLeft),
            Some(Align::End)
        );
        assert_eq!(
            alignment_for_direction(Some(ST_Jc::End), TextDirection::RightToLeft),
            Some(Align::Start)
        );
        assert_eq!(
            alignment_for_direction(Some(ST_Jc::Left), TextDirection::RightToLeft),
            Some(Align::Start)
        );
        assert_eq!(
            alignment_for_direction(Some(ST_Jc::Right), TextDirection::LeftToRight),
            Some(Align::End)
        );
    }

    #[test]
    fn every_word_tab_and_leader_maps_without_rounding_its_position() {
        let cases = [
            (ST_TabJc::Left, TabAlign::Left),
            (ST_TabJc::Center, TabAlign::Center),
            (ST_TabJc::Right, TabAlign::Right),
            (ST_TabJc::Decimal, TabAlign::Decimal),
            (ST_TabJc::Bar, TabAlign::Bar),
            (ST_TabJc::Num, TabAlign::Left),
        ];
        let leaders = [
            (ST_TabLeader::None, TabLeader::None),
            (ST_TabLeader::Dot, TabLeader::Dot),
            (ST_TabLeader::Hyphen, TabLeader::Hyphen),
            (ST_TabLeader::Underscore, TabLeader::Underscore),
            (ST_TabLeader::Heavy, TabLeader::Heavy),
            (ST_TabLeader::MiddleDot, TabLeader::MiddleDot),
        ];

        for ((word_align, shared_align), (word_leader, shared_leader)) in
            cases.into_iter().zip(leaders)
        {
            let converted = tab_stops(&[CT_TabStop {
                val: word_align,
                pos: Twips(1),
                leader: Some(word_leader),
                source_occurrence: None,
            }]);
            assert_eq!(converted.len(), 1);
            assert_eq!(converted[0].align, shared_align);
            assert_eq!(converted[0].leader, Some(shared_leader));
            assert!((converted[0].pos_pt - 0.05).abs() < f64::EPSILON);
        }

        assert!(tab_stops(&[CT_TabStop::new(ST_TabJc::Clear, Twips(720))]).is_empty());
    }

    #[test]
    fn every_rendered_word_underline_maps_to_the_shared_style() {
        let cases = [
            (ST_Underline::Single, Underline::Single),
            (ST_Underline::Words, Underline::Words),
            (ST_Underline::Double, Underline::Double),
            (ST_Underline::Thick, Underline::Thick),
            (ST_Underline::Dotted, Underline::Dotted),
            (ST_Underline::Dash, Underline::Dash),
            (ST_Underline::DotDash, Underline::DotDash),
            (ST_Underline::DotDotDash, Underline::DotDotDash),
            (ST_Underline::Wave, Underline::Wave),
        ];
        for (word, shared) in cases {
            assert_eq!(underline(Some(word)), Some(shared));
        }
        assert_eq!(underline(Some(ST_Underline::None)), None);
        assert_eq!(underline(None), None);
    }

    #[test]
    fn every_word_line_spacing_rule_maps_exactly() {
        let cases = [
            (Some(Twips(1)), Some("exact"), LineSpacing::Exact(0.05)),
            (
                Some(Twips(480)),
                Some("atLeast"),
                LineSpacing::AtLeast(24.0),
            ),
            (Some(Twips(360)), Some("auto"), LineSpacing::Multiple(1.5)),
            (Some(Twips(240)), None, LineSpacing::Multiple(1.0)),
            (None, None, LineSpacing::Single),
        ];
        for (spacing, rule, expected) in cases {
            let properties = CT_PPr {
                line_spacing: spacing,
                line_rule: rule.map(str::to_owned),
                ..Default::default()
            };
            assert_eq!(line_spacing(&properties), expected);
        }
    }

    #[test]
    fn word_line_parameters_keep_wrap_enabled() {
        let params = line_break_params(&CT_PPr::default(), 321.0, None);
        assert_eq!(params.available_width, 321.0);
        assert!(params.wrap);
    }

    /// Calibri 11 as Word measures it: 1950, 550 and 0 units of 2048.
    const CALIBRI_11_MARK: FontMetrics = FontMetrics {
        ascent: 11.0 * 1950.0 / 2048.0,
        descent: 11.0 * 550.0 / 2048.0,
        line_gap: 0.0,
        units_per_em: 2048,
    };

    fn restore(lines: &mut [LayoutLine], properties: &CT_PPr, paragraph_mark: Option<FontMetrics>) {
        let fm = FontManager::new_deterministic().expect("bundled fonts load");
        restore_word_line_heights(lines, line_spacing(properties), None, &fm, paragraph_mark);
    }

    fn auto(line: i32) -> CT_PPr {
        CT_PPr {
            line_spacing: Some(Twips(line)),
            line_rule: Some("auto".to_string()),
            ..Default::default()
        }
    }

    fn line_of(items: Vec<LineItem>, ascent: f64, descent: f64) -> LayoutLine {
        LayoutLine {
            items,
            width: 0.0,
            ascent,
            descent,
            line_gap: 0.0,
            height: ascent + descent,
            indent_left: 0.0,
            available_width: 468.0,
            is_last: true,
            forced_break_after: None,
        }
    }

    fn picture(height: f64) -> LineItem {
        LineItem::Image {
            width: height,
            height,
            media_id: oxml_layout::MediaId(0),
        }
    }

    /// Word keeps a 100 point picture alone on its line at 100 points plus
    /// the proportional share of its Calibri 11 paragraph mark: 101.34 at
    /// 1.1 lines, 106.71 at 1.5 and 113.43 at 2, measured in Word 16.
    #[test]
    fn a_picture_line_scales_only_the_text_height_of_its_paragraph_mark() {
        for (line, expected) in [(240, 100.0), (264, 101.34), (360, 106.71), (480, 113.43)] {
            let mut lines = vec![line_of(vec![picture(100.0)], 100.0, 0.0)];
            restore(&mut lines, &auto(line), Some(CALIBRI_11_MARK));
            assert!(
                (lines[0].height - expected).abs() < 0.01,
                "w:line {line}: {} against Word's {expected}",
                lines[0].height
            );
            assert_eq!((lines[0].ascent, lines[0].descent), (100.0, 0.0));
        }
    }

    /// A caption set in Arial 12 scales by its own 2355/2048 em line, and a
    /// figure wrapped in its alternative text stands exactly as the picture.
    #[test]
    fn a_picture_line_takes_its_text_height_from_the_paragraph_mark_font() {
        let arial_12 = FontMetrics {
            ascent: 12.0 * 1854.0 / 2048.0,
            descent: 12.0 * 434.0 / 2048.0,
            line_gap: 12.0 * 67.0 / 2048.0,
            units_per_em: 2048,
        };
        let figure = LineItem::Figure {
            item: Box::new(picture(100.0)),
            alternate_text: "A figure".to_owned(),
            structure_id: None,
        };
        let mut lines = vec![line_of(vec![figure], 100.0, 0.0)];
        restore(&mut lines, &auto(360), Some(arial_12));
        // Word 16: 106.90.
        assert!((lines[0].height - (100.0 + 0.5 * 12.0 * 2355.0 / 2048.0)).abs() < 1e-9);
        assert!((lines[0].height - 106.90).abs() < 0.01);
    }

    /// Exact and at-least spacing are not proportional, so a picture line
    /// under either keeps the rule it has always had.
    #[test]
    fn exact_and_at_least_spacing_keep_their_picture_line_rules() {
        for (spacing, rule, expected) in [
            (300, "exact", 15.0),
            (300, "atLeast", 100.0),
            (2400, "atLeast", 120.0),
        ] {
            let mut lines = vec![line_of(vec![picture(100.0)], 100.0, 0.0)];
            let properties = CT_PPr {
                line_spacing: Some(Twips(spacing)),
                line_rule: Some(rule.to_owned()),
                ..Default::default()
            };
            restore(&mut lines, &properties, Some(CALIBRI_11_MARK));
            assert_eq!(lines[0].height, expected, "{rule} {spacing}");
        }
    }

    #[test]
    fn word_auto_spacing_uses_glyph_height_not_point_size() {
        let mut lines = vec![LayoutLine {
            items: Vec::new(),
            width: 0.0,
            ascent: 10.0,
            descent: 3.0,
            line_gap: 2.0,
            height: 12.0,
            indent_left: 0.0,
            available_width: 468.0,
            is_last: true,
            forced_break_after: None,
        }];
        let properties = CT_PPr {
            line_spacing: Some(Twips(480)),
            line_rule: Some("auto".to_string()),
            ..Default::default()
        };
        restore(&mut lines, &properties, None);
        assert_eq!(lines[0].height, 26.0);
        assert_eq!(lines[0].line_gap, 0.0);
    }

    #[test]
    fn table_cell_line_advance_matches_measured_row_height() {
        let source = LayoutLine {
            items: Vec::new(),
            width: 0.0,
            ascent: 9.0,
            descent: 3.0,
            line_gap: 0.0,
            height: 12.0,
            indent_left: 0.0,
            available_width: 400.0,
            is_last: true,
            forced_break_after: None,
        };
        for (spacing, rule, expected) in [
            (None, None, 12.0),
            (Some(Twips(360)), Some("exact"), 18.0),
            (Some(Twips(400)), Some("atLeast"), 20.0),
        ] {
            let mut lines = vec![source.clone(); 3];
            let properties = CT_PPr {
                line_spacing: spacing,
                line_rule: rule.map(str::to_owned),
                ..Default::default()
            };
            restore(&mut lines, &properties, None);
            let row_height = 3.0 + lines.iter().map(|line| line.height).sum::<f64>() + 5.0;
            assert_eq!(row_height, 3.0 + 3.0 * expected + 5.0);
        }
    }
}
