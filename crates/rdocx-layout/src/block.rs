//! Block-level layout: paragraphs and tables as positioned blocks.

use crate::table::TableBlock;
use oxml_layout::{
    Align, Color, FontMetrics, GroupElement, InlineItem, LayoutLine, LineBreakParams, MediaId,
    SourceNodeId, StructureId, TextDirection,
};
use rdocx_oxml::borders::CT_PBdr;
use rdocx_oxml::drawing::{
    AnchorAlignH, AnchorAlignV, ST_RelativeFromH, ST_RelativeFromV, WrapType,
};
use std::ops::Deref;
use std::sync::Arc;

/// A floating drawing anchored to a paragraph.
///
/// Offsets are kept in points alongside the frame they are measured from. A
/// `wp:anchor` offset is meaningless on its own: the same number means a
/// different place depending on whether it is relative to the page, the
/// margin, the text column or the paragraph.
#[derive(Debug, Clone)]
pub struct AnchoredDrawing {
    /// Rotation of a modeled shape around its own physical center.
    pub rotation_degrees: f64,
    /// Render underneath the text rather than on top of it.
    pub behind_doc: bool,
    /// Frame the horizontal offset is measured from.
    pub rel_h: ST_RelativeFromH,
    /// Horizontal offset in points.
    pub off_h: f64,
    /// Frame the vertical offset is measured from.
    pub rel_v: ST_RelativeFromV,
    /// Vertical offset in points.
    pub off_v: f64,
    /// Width in points.
    pub width: f64,
    /// Height in points.
    pub height: f64,
    /// How text flows around the drawing.
    pub wrap: WrapType,
    /// Space kept between the drawing and the text wrapping around it, in
    /// points.
    pub dist_top: f64,
    pub dist_bottom: f64,
    pub dist_left: f64,
    pub dist_right: f64,
    /// Horizontal alignment, used instead of the offset when present.
    pub align_h: Option<AnchorAlignH>,
    /// Vertical alignment, used instead of the offset when present.
    pub align_v: Option<AnchorAlignV>,
    /// What the drawing actually holds.
    pub content: AnchoredContent,
    /// Source description for an informative drawing.
    pub alternate_text: Option<String>,
    /// Logical figure node allocated before pagination.
    pub structure_id: Option<StructureId>,
}

/// The drawable content of an anchored drawing.
#[derive(Debug, Clone)]
pub enum AnchoredContent {
    /// A picture resolved to its content-addressed shared media identity.
    Image { media_id: MediaId },
    /// A backend-neutral group rendered in child-local chart coordinates.
    Group(GroupElement),
    /// A shape: preset geometry, an optional fill, and optional text.
    ///
    /// The text arrives already laid out, because breaking it into lines needs
    /// a font manager and that only exists in the engine.
    Shape {
        /// Preset geometry we recognise.
        preset: ShapePreset,
        /// Fill colour, or `None` for `a:noFill` and for fills we cannot
        /// resolve. An unfilled shape draws no body, only its text.
        fill: Option<Color>,
        /// Laid-out paragraphs of the shape's text box.
        text: Vec<ParagraphBlock>,
    },
}

/// The preset geometries we can draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapePreset {
    /// A rectangle, drawn as a filled box.
    Rect,
    /// A straight line, drawn along the top edge of the extent.
    Line,
    /// Anything else. The body is not drawn, but text still is.
    Unsupported,
}

impl ShapePreset {
    /// Map an `a:prstGeom@prst` value onto what we can draw.
    pub fn from_prst(prst: Option<&str>) -> Self {
        match prst {
            Some("rect") => ShapePreset::Rect,
            Some("line") | Some("straightConnector1") => ShapePreset::Line,
            _ => ShapePreset::Unsupported,
        }
    }
}

/// A laid-out block element (paragraph or table).
#[derive(Debug, Clone)]
pub enum LayoutBlock {
    Paragraph(ParagraphBlock),
    Table(TableBlock),
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ParagraphSemantics {
    // Facing group edges are removed only on transient layout clones.
    pub border_group_before: bool,
    pub source_node: Option<SourceNodeId>,
    pub structure_id: Option<StructureId>,
    pub reflow_direction: TextDirection,
}

#[derive(Debug, Clone)]
pub(crate) enum CellBlockSemantics {
    Paragraph(ParagraphSemantics),
    Table(TableSemantics),
}

#[derive(Debug, Clone)]
pub(crate) struct CellSemantics {
    pub blocks: Vec<CellBlockSemantics>,
}

#[derive(Debug, Clone)]
pub(crate) struct RowSemantics {
    pub cells: Vec<CellSemantics>,
}

#[derive(Debug, Clone)]
pub(crate) struct TableSemantics {
    pub rows: Vec<RowSemantics>,
}

#[derive(Debug, Clone)]
pub(crate) enum SharedLayoutBlock {
    Owned {
        border_group_before: bool,
        block: Box<LayoutBlock>,
        reflow_direction: TextDirection,
        body_index: Option<usize>,
    },
    Paragraph {
        block: Arc<ParagraphBlock>,
        semantics: ParagraphSemantics,
        body_index: Option<usize>,
    },
    Table {
        block: Arc<TableBlock>,
        /// Finite active story measures share the same logical semantics.
        variants: Vec<(f64, Arc<TableBlock>)>,
        semantics: TableSemantics,
        body_index: Option<usize>,
    },
}

impl SharedLayoutBlock {
    pub(crate) fn set_body_index(&mut self, body_index: usize) {
        match self {
            Self::Owned {
                body_index: owner, ..
            }
            | Self::Paragraph {
                body_index: owner, ..
            }
            | Self::Table {
                body_index: owner, ..
            } => *owner = Some(body_index),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct ParagraphView<'a> {
    pub border_group_before: bool,
    pub block: &'a ParagraphBlock,
    pub semantics: Option<&'a ParagraphSemantics>,
    pub reflow_direction: TextDirection,
    pub reflow_allowed: bool,
}

impl Deref for ParagraphView<'_> {
    type Target = ParagraphBlock;

    fn deref(&self) -> &Self::Target {
        self.block
    }
}

impl ParagraphView<'_> {
    pub fn source_node(self) -> Option<Option<SourceNodeId>> {
        self.semantics.map(|semantics| semantics.source_node)
    }

    pub fn structure_id(self) -> Option<StructureId> {
        self.semantics
            .map_or(self.block.structure_id, |semantics| semantics.structure_id)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct TableView<'a> {
    pub block: &'a TableBlock,
    pub semantics: Option<&'a TableSemantics>,
}

impl Deref for TableView<'_> {
    type Target = TableBlock;

    fn deref(&self) -> &Self::Target {
        self.block
    }
}

pub(crate) trait LayoutBlockLike {
    fn paragraph(&self) -> Option<ParagraphView<'_>>;
    fn table(&self) -> Option<TableView<'_>>;

    fn table_for_measure(&self, _width: f64) -> Option<TableView<'_>> {
        self.table()
    }

    fn body_index(&self) -> Option<usize> {
        None
    }

    fn content_height(&self) -> f64 {
        self.paragraph().map_or_else(
            || self.table().unwrap().content_height(),
            |p| p.content_height(),
        )
    }

    fn space_before(&self) -> f64 {
        self.paragraph()
            .map_or(0.0, |paragraph| paragraph.space_before)
    }

    fn space_after(&self) -> f64 {
        self.paragraph()
            .map_or(0.0, |paragraph| paragraph.space_after)
    }

    fn page_break_before(&self) -> bool {
        self.paragraph()
            .is_some_and(|paragraph| paragraph.page_break_before)
    }
}

impl LayoutBlockLike for LayoutBlock {
    fn paragraph(&self) -> Option<ParagraphView<'_>> {
        match self {
            Self::Paragraph(block) => Some(ParagraphView {
                border_group_before: false,
                block,
                semantics: None,
                reflow_direction: TextDirection::Auto,
                reflow_allowed: true,
            }),
            Self::Table(_) => None,
        }
    }

    fn table(&self) -> Option<TableView<'_>> {
        match self {
            Self::Paragraph(_) => None,
            Self::Table(block) => Some(TableView {
                block,
                semantics: None,
            }),
        }
    }
}

impl LayoutBlockLike for SharedLayoutBlock {
    fn paragraph(&self) -> Option<ParagraphView<'_>> {
        match self {
            Self::Owned {
                block,
                border_group_before,
                reflow_direction,
                ..
            } => block.paragraph().map(|mut paragraph| {
                paragraph.reflow_direction = *reflow_direction;
                paragraph.border_group_before = *border_group_before;
                paragraph
            }),
            Self::Paragraph {
                block, semantics, ..
            } => Some(ParagraphView {
                border_group_before: semantics.border_group_before,
                block,
                semantics: Some(semantics),
                reflow_direction: semantics.reflow_direction,
                reflow_allowed: true,
            }),
            Self::Table { .. } => None,
        }
    }

    fn table(&self) -> Option<TableView<'_>> {
        match self {
            Self::Owned { block, .. } => block.table(),
            Self::Paragraph { .. } => None,
            Self::Table {
                block, semantics, ..
            } => Some(TableView {
                block,
                semantics: Some(semantics),
            }),
        }
    }

    fn table_for_measure(&self, width: f64) -> Option<TableView<'_>> {
        if let Self::Table {
            variants,
            semantics,
            ..
        } = self
            && let Some((_, block)) = variants.iter().find(|(measure, _)| *measure == width)
        {
            return Some(TableView {
                block,
                semantics: Some(semantics),
            });
        }
        self.table()
    }

    fn body_index(&self) -> Option<usize> {
        match self {
            Self::Owned { body_index, .. }
            | Self::Paragraph { body_index, .. }
            | Self::Table { body_index, .. } => *body_index,
        }
    }
}

impl LayoutBlock {
    /// Total height including spacing.
    pub fn total_height(&self) -> f64 {
        match self {
            LayoutBlock::Paragraph(p) => p.total_height(),
            LayoutBlock::Table(t) => t.total_height(),
        }
    }

    /// Content height without spacing.
    pub fn content_height(&self) -> f64 {
        match self {
            LayoutBlock::Paragraph(p) => p.content_height(),
            LayoutBlock::Table(t) => t.content_height(),
        }
    }

    pub fn space_before(&self) -> f64 {
        match self {
            LayoutBlock::Paragraph(p) => p.space_before,
            LayoutBlock::Table(_) => 0.0,
        }
    }

    pub fn space_after(&self) -> f64 {
        match self {
            LayoutBlock::Paragraph(p) => p.space_after,
            LayoutBlock::Table(_) => 0.0,
        }
    }

    pub fn keep_next(&self) -> bool {
        match self {
            LayoutBlock::Paragraph(p) => p.keep_next,
            LayoutBlock::Table(_) => false,
        }
    }

    pub fn keep_lines(&self) -> bool {
        match self {
            LayoutBlock::Paragraph(p) => p.keep_lines,
            LayoutBlock::Table(_) => false,
        }
    }

    pub fn page_break_before(&self) -> bool {
        match self {
            LayoutBlock::Paragraph(p) => p.page_break_before,
            LayoutBlock::Table(_) => false,
        }
    }

    pub fn widow_control(&self) -> bool {
        match self {
            LayoutBlock::Paragraph(p) => p.widow_control,
            LayoutBlock::Table(_) => false,
        }
    }
}

/// What a paragraph needs in order to be broken into lines again.
///
/// Whether a floating drawing overlaps a line is only known once the paragraph
/// has a position on a page, which is after layout and during pagination. The
/// inputs to line breaking therefore have to survive that far.
///
/// `InlineItem::Text` holds the same shaped glyphs `LayoutLine` already holds,
/// so this roughly doubles a paragraph's text memory. It is carried only when
/// the document contains a drawing that wraps, which nearly none do.
#[derive(Debug, Clone)]
pub struct ParagraphReflow {
    pub items: Vec<InlineItem>,
    pub params: LineBreakParams,
    /// The section grid pitch this paragraph's line advance snaps to.
    ///
    /// `LineBreakParams` has no grid, so a re-break around a float would
    /// leave the grid without this. It is `None` for every paragraph off the
    /// grid, including one on an exact `w:lineRule`, so an ungridded reflow
    /// takes the arithmetic it always took.
    pub grid_line_pitch_pt: Option<f64>,
    /// Word's measure of the paragraph mark, which a re-broken line holding
    /// only inline objects takes its proportional spacing from.
    pub paragraph_mark: Option<FontMetrics>,
}

/// A laid-out paragraph with its lines and spacing.
#[derive(Debug, Clone)]
pub struct ParagraphBlock {
    /// Laid-out lines.
    pub lines: Vec<LayoutLine>,
    /// The colour of the change bar the tracked projection draws beside the
    /// paragraph, `None` when it shows no revision.
    pub change_bar: Option<Color>,
    /// Floating drawings anchored to this paragraph.
    ///
    /// These travel with the paragraph so the paginator can resolve a
    /// paragraph-relative or line-relative offset once it knows where the
    /// paragraph actually landed.
    pub anchored: Vec<AnchoredDrawing>,
    /// Space before the paragraph in points.
    pub space_before: f64,
    /// Space after the paragraph in points.
    pub space_after: f64,
    /// Paragraph borders.
    pub borders: Option<CT_PBdr>,
    /// Background shading color.
    pub shading: Option<Color>,
    /// Left indent in points.
    pub indent_left: f64,
    /// Right indent in points.
    pub indent_right: f64,
    /// Paragraph justification.
    pub jc: Option<Align>,
    /// Keep with next paragraph.
    pub keep_next: bool,
    /// Keep all lines together on one page.
    pub keep_lines: bool,
    /// Force page break before this paragraph.
    pub page_break_before: bool,
    /// Widow/orphan control.
    pub widow_control: bool,
    /// Heading level (1-9) if this is a heading paragraph, for outline generation.
    pub heading_level: Option<u32>,
    /// Heading text for outline generation.
    pub heading_text: Option<String>,
    /// Numbering instance and zero-based nesting level for semantic lists.
    pub list: Option<(u32, u8)>,
    /// Logical paragraph node allocated before pagination.
    pub structure_id: Option<StructureId>,
    /// Inputs for re-breaking this paragraph around a floating drawing.
    ///
    /// `None` unless the document holds a drawing that wraps.
    pub reflow: Option<Box<ParagraphReflow>>,
    /// Vertical space kept clear above the first line, for a drawing this
    /// paragraph must clear rather than flow beside.
    pub content_offset_top: f64,
    /// The paragraph's style, the default paragraph style when it names
    /// none: what contextual spacing compares between neighbours.
    pub style: Option<Arc<str>>,
    /// `w:contextualSpacing`: this paragraph's own space before and after is
    /// dropped next to a paragraph of the same style
    /// ([`collapse_contextual_spacing`]).
    pub contextual_spacing: bool,
    /// `w:framePr`: where Word lifts this paragraph out of the text flow.
    pub frame: Option<TextFrame>,
}

/// A paragraph Word places in a text frame (`w:framePr`) instead of the flow.
///
/// Only header and footer stories place frames today
/// (`paginator::render_hf_blocks`), which is where Word's page number gallery
/// puts them. A body paragraph keeps its flow position, a drop cap is not a
/// frame here, and text beside a frame does not wrap around it: those remain
/// F-311.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextFrame {
    /// `w:w`. `None` sizes the frame to its widest line.
    pub width: Option<f64>,
    pub horizontal: FramePlacement,
    pub vertical: FramePlacement,
    /// `w:wrap="notBeside"`: no text sits beside the frame, so it keeps the
    /// vertical space of its lines in the flow.
    pub in_flow: bool,
}

/// One axis of a frame's position: an alignment within the anchor's box, or
/// an offset from the box's leading edge when there is none.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FramePlacement {
    pub anchor: FrameAnchor,
    pub align: Option<FrameAlign>,
    pub offset: f64,
}

/// `w:hAnchor` and `w:vAnchor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameAnchor {
    Page,
    Margin,
    /// The column horizontally, the anchoring paragraph vertically.
    Text,
}

/// `w:xAlign` and `w:yAlign`, with left and top as `Start`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameAlign {
    Start,
    Center,
    End,
    Inside,
    Outside,
}

impl TextFrame {
    /// The frame a `w:framePr` places, or `None` for a drop cap.
    ///
    /// An omitted anchor is the column and the paragraph, which is what Word's
    /// frame dialog starts from and what LibreOffice assumes on import.
    pub fn from_ct(frame: &rdocx_oxml::properties::CT_FramePr) -> Option<Self> {
        if matches!(frame.drop_cap.as_deref(), Some("drop" | "margin")) {
            return None;
        }
        let anchor = |value: Option<&str>| match value {
            Some("page") => FrameAnchor::Page,
            Some("margin") => FrameAnchor::Margin,
            _ => FrameAnchor::Text,
        };
        let align = |value: Option<&str>| match value? {
            "left" | "top" => Some(FrameAlign::Start),
            "center" => Some(FrameAlign::Center),
            "right" | "bottom" => Some(FrameAlign::End),
            "inside" => Some(FrameAlign::Inside),
            "outside" => Some(FrameAlign::Outside),
            _ => None,
        };
        Some(TextFrame {
            width: frame
                .w
                .map(|width| width.to_pt())
                .filter(|width| *width > 0.0),
            horizontal: FramePlacement {
                anchor: anchor(frame.h_anchor.as_deref()),
                align: align(frame.x_align.as_deref()),
                offset: frame.x.map_or(0.0, |x| x.to_pt()),
            },
            vertical: FramePlacement {
                anchor: anchor(frame.v_anchor.as_deref()),
                align: align(frame.y_align.as_deref()),
                offset: frame.y.map_or(0.0, |y| y.to_pt()),
            },
            in_flow: frame.wrap.as_deref() == Some("notBeside"),
        })
    }
}

impl FramePlacement {
    /// The frame's leading edge, given the anchor box `(start, length)` and
    /// the frame's own length along this axis. `odd` says whether the page is
    /// a right-hand page, which is where inside and outside turn.
    pub fn resolve(self, (start, length): (f64, f64), size: f64, odd: bool) -> f64 {
        match self.align {
            None => start + self.offset,
            Some(FrameAlign::Start) => start,
            Some(FrameAlign::Center) => start + (length - size) / 2.0,
            Some(FrameAlign::End) => start + length - size,
            Some(FrameAlign::Inside) if odd => start,
            Some(FrameAlign::Inside) => start + length - size,
            Some(FrameAlign::Outside) if odd => start + length - size,
            Some(FrameAlign::Outside) => start,
        }
    }
}

/// Complete painted width of a supported paragraph border edge.
pub(crate) fn paragraph_border_width(edge: Option<&rdocx_oxml::borders::CT_BorderEdge>) -> f64 {
    let Some(edge) = edge.filter(|edge| {
        !edge.val.is_none() && !matches!(edge.val, rdocx_oxml::shared::ST_Border::Art(_))
    }) else {
        return 0.0;
    };
    let width = edge.sz.unwrap_or(4) as f64 / 8.0;
    if edge.val == rdocx_oxml::shared::ST_Border::Double {
        width * 3.0
    } else {
        width
    }
}

pub(crate) fn paragraph_border_padding(edge: Option<&rdocx_oxml::borders::CT_BorderEdge>) -> f64 {
    let width = paragraph_border_width(edge);
    if width == 0.0 {
        0.0
    } else {
        width + edge.and_then(|edge| edge.space).unwrap_or(0) as f64
    }
}

/// Facing edges of identical adjacent frames belong to the outer group only.
pub(crate) fn paragraph_border_groups(paragraphs: &[Option<&ParagraphBlock>]) -> Vec<(bool, bool)> {
    let joins = |left: Option<&ParagraphBlock>, right: Option<&ParagraphBlock>| {
        matches!((left,right), (Some(left),Some(right)) if !right.page_break_before
            && left.borders.is_some() && left.borders == right.borders
            && left.borders.as_ref().is_some_and(|borders| borders.between.is_none())
            && left.indent_left == right.indent_left && left.indent_right == right.indent_right)
    };
    (0..paragraphs.len())
        .map(|index| {
            (
                index > 0 && joins(paragraphs[index - 1], paragraphs[index]),
                index + 1 < paragraphs.len() && joins(paragraphs[index], paragraphs[index + 1]),
            )
        })
        .collect()
}

pub(crate) fn collapse_shared_border_groups(blocks: &mut [SharedLayoutBlock]) {
    if !blocks.iter().any(|block| {
        block
            .paragraph()
            .is_some_and(|paragraph| paragraph.borders.is_some())
    }) {
        return;
    }
    let paragraphs = blocks
        .iter()
        .map(|block| block.paragraph().map(|paragraph| paragraph.block))
        .collect::<Vec<_>>();
    let groups = paragraph_border_groups(&paragraphs);
    for (block, (top, bottom)) in blocks.iter_mut().zip(groups) {
        if !top && !bottom {
            continue;
        }
        match block {
            SharedLayoutBlock::Paragraph {
                block, semantics, ..
            } => {
                semantics.border_group_before = top;
                Arc::make_mut(block).remove_group_border_edges(top, bottom)
            }
            SharedLayoutBlock::Owned {
                block,
                border_group_before,
                ..
            } => {
                *border_group_before = top;
                if let LayoutBlock::Paragraph(paragraph) = block.as_mut() {
                    paragraph.remove_group_border_edges(top, bottom);
                }
            }
            SharedLayoutBlock::Table { .. } => {}
        }
    }
}

/// Which facing spacing contextual spacing drops, per paragraph of one flow
/// (`None` for a table, which separates its neighbours): `(before, after)`.
///
/// Word drops a paragraph's own space before when the paragraph above is of
/// the same style, and its own space after when the paragraph below is, if
/// the paragraph asks for it (`w:contextualSpacing`, as the List Paragraph
/// style does). Each paragraph's flag governs only its own spacing.
pub(crate) fn contextual_spacing_drops(
    paragraphs: &[Option<&ParagraphBlock>],
) -> Vec<(bool, bool)> {
    let same_style = |left: Option<&ParagraphBlock>, right: Option<&ParagraphBlock>| {
        matches!((left, right), (Some(left), Some(right))
            if left.style.is_some() && left.style == right.style)
    };
    (0..paragraphs.len())
        .map(|index| {
            let Some(paragraph) = paragraphs[index] else {
                return (false, false);
            };
            if !paragraph.contextual_spacing {
                return (false, false);
            }
            let previous = index.checked_sub(1).and_then(|i| paragraphs[i]);
            let next = paragraphs.get(index + 1).copied().flatten();
            (
                same_style(previous, Some(paragraph)),
                same_style(Some(paragraph), next),
            )
        })
        .collect()
}

impl ParagraphBlock {
    /// Drops the spacing [`contextual_spacing_drops`] chose.
    pub(crate) fn drop_contextual_spacing(&mut self, (before, after): (bool, bool)) {
        if before {
            self.space_before = 0.0;
        }
        if after {
            self.space_after = 0.0;
        }
    }
}

/// [`contextual_spacing_drops`] over a section's shared blocks. A cached
/// paragraph is copied before it changes, so the cache keeps its own.
pub(crate) fn collapse_shared_contextual_spacing(blocks: &mut [SharedLayoutBlock]) {
    if !blocks.iter().any(|block| {
        block
            .paragraph()
            .is_some_and(|paragraph| paragraph.contextual_spacing)
    }) {
        return;
    }
    let paragraphs = blocks
        .iter()
        .map(|block| block.paragraph().map(|paragraph| paragraph.block))
        .collect::<Vec<_>>();
    let drops = contextual_spacing_drops(&paragraphs);
    for (block, drop) in blocks.iter_mut().zip(drops) {
        if drop == (false, false) {
            continue;
        }
        match block {
            SharedLayoutBlock::Paragraph { block, .. } => {
                Arc::make_mut(block).drop_contextual_spacing(drop)
            }
            SharedLayoutBlock::Owned { block, .. } => {
                if let LayoutBlock::Paragraph(paragraph) = block.as_mut() {
                    paragraph.drop_contextual_spacing(drop);
                }
            }
            SharedLayoutBlock::Table { .. } => {}
        }
    }
}

impl ParagraphBlock {
    /// Price frame bands into the existing line stream, so every height
    /// consumer, including note flattening and cell splitting, sees them.
    pub(crate) fn add_border_padding(&mut self) {
        let Some(borders) = &self.borders else {
            return;
        };
        let top = paragraph_border_padding(borders.top.as_ref());
        let bottom = paragraph_border_padding(borders.bottom.as_ref());
        if let Some(first) = self.lines.first_mut() {
            first.height += top;
            first.ascent += top;
        }
        if let Some(last) = self.lines.last_mut() {
            last.height += bottom;
        }
    }

    pub(crate) fn remove_group_border_edges(&mut self, top: bool, bottom: bool) {
        if !top && !bottom {
            return;
        }
        let Some(borders) = &mut self.borders else {
            return;
        };
        if top {
            let padding = paragraph_border_padding(borders.top.as_ref());
            if let Some(first) = self.lines.first_mut() {
                first.height -= padding;
                first.ascent -= padding;
            }
            borders.top = None;
        }
        if bottom {
            let padding = paragraph_border_padding(borders.bottom.as_ref());
            if let Some(last) = self.lines.last_mut() {
                last.height -= padding;
            }
            borders.bottom = None;
        }
    }

    /// Total height of the paragraph lines (not including before/after spacing).
    pub fn content_height(&self) -> f64 {
        self.content_offset_top + self.lines.iter().map(|l| l.height).sum::<f64>()
    }

    /// Total height including spacing.
    pub fn total_height(&self) -> f64 {
        self.space_before + self.content_height() + self.space_after
    }

    /// Number of lines.
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}

/// Build a ParagraphBlock from resolved properties and layout lines.
pub fn build_paragraph_block(
    lines: Vec<LayoutLine>,
    space_before: f64,
    space_after: f64,
    borders: Option<CT_PBdr>,
    shading: Option<Color>,
    indent_left: f64,
    indent_right: f64,
    jc: Option<Align>,
    keep_next: bool,
    keep_lines: bool,
    page_break_before: bool,
    widow_control: bool,
) -> ParagraphBlock {
    let mut block = ParagraphBlock {
        lines,
        change_bar: None,
        anchored: Vec::new(),
        space_before,
        space_after,
        borders,
        shading,
        indent_left,
        indent_right,
        jc,
        keep_next,
        keep_lines,
        page_break_before,
        widow_control,
        heading_level: None,
        heading_text: None,
        list: None,
        structure_id: None,
        reflow: None,
        content_offset_top: 0.0,
        style: None,
        contextual_spacing: false,
        frame: None,
    };
    block.add_border_padding();
    block
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contextual_spacing_drops_only_the_flagged_paragraphs_facing_spacing() {
        let paragraph = |style: &str, contextual: bool| {
            let mut block = build_paragraph_block(
                Vec::new(),
                6.0,
                10.0,
                None,
                None,
                0.0,
                0.0,
                None,
                false,
                false,
                false,
                true,
            );
            block.style = Some(Arc::from(style));
            block.contextual_spacing = contextual;
            block
        };
        let normal = paragraph("Normal", false);
        let item = paragraph("ListParagraph", true);
        let plain_item = paragraph("ListParagraph", false);
        let drops = contextual_spacing_drops(&[
            Some(&normal),
            Some(&item),
            Some(&item),
            Some(&plain_item),
            None,
            Some(&item),
        ]);
        assert_eq!(
            drops,
            [
                (false, false),
                // Normal above: its own space before stays.
                (false, true),
                (true, true),
                // Not flagged: keeps its own spacing beside the same style.
                (false, false),
                (false, false),
                // A table above separates it.
                (false, false),
            ]
        );
    }

    #[test]
    fn paragraph_block_height() {
        let block = ParagraphBlock {
            anchored: Vec::new(),
            change_bar: None,
            lines: vec![
                LayoutLine {
                    items: vec![],
                    width: 0.0,
                    ascent: 10.0,
                    descent: 3.0,
                    line_gap: 0.0,
                    height: 13.0,
                    indent_left: 0.0,
                    available_width: 468.0,
                    is_last: false,
                    forced_break_after: None,
                },
                LayoutLine {
                    items: vec![],
                    width: 0.0,
                    ascent: 10.0,
                    descent: 3.0,
                    line_gap: 0.0,
                    height: 13.0,
                    indent_left: 0.0,
                    available_width: 468.0,
                    is_last: true,
                    forced_break_after: None,
                },
            ],
            space_before: 6.0,
            space_after: 8.0,
            borders: None,
            shading: None,
            indent_left: 0.0,
            indent_right: 0.0,
            jc: None,
            keep_next: false,
            keep_lines: false,
            page_break_before: false,
            widow_control: true,
            heading_level: None,
            heading_text: None,
            list: None,
            structure_id: None,
            reflow: None,
            content_offset_top: 0.0,
            style: None,
            contextual_spacing: false,
            frame: None,
        };
        assert!((block.content_height() - 26.0).abs() < 0.01);
        assert!((block.total_height() - 40.0).abs() < 0.01);
    }

    /// Only the presets we can actually draw map to a drawable body. Anything
    /// else still renders its text, so it must not be treated as a rectangle.
    #[test]
    fn shape_presets_map_to_what_we_can_draw() {
        assert_eq!(ShapePreset::from_prst(Some("rect")), ShapePreset::Rect);
        assert_eq!(ShapePreset::from_prst(Some("line")), ShapePreset::Line);
        assert_eq!(
            ShapePreset::from_prst(Some("straightConnector1")),
            ShapePreset::Line
        );
        assert_eq!(
            ShapePreset::from_prst(Some("roundRect")),
            ShapePreset::Unsupported,
            "an unhandled preset must not silently draw as a plain rectangle"
        );
        assert_eq!(ShapePreset::from_prst(None), ShapePreset::Unsupported);
    }
}
