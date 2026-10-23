import io
from pathlib import Path
from typing import TYPE_CHECKING, Callable, assert_type

from rpptx import (
    Element,
    BoundingBox,
    AutofitRefreshResult,
    AutofitUpdate,
    FitReport,
    FitFrameReport,
    FitTrial,
    Comment,
    CommentAuthor,
    CommentReply,
    Inches,
    Length,
    MSO_ANCHOR,
    MSO_AUTO_SIZE,
    MSO_SHAPE,
    MSO_UNDERLINE,
    PP_ALIGN,
    Presentation,
    PresentationTextExport,
    Pt,
    ReplacementCountError,
    RGBColor,
    TextFrameLayout,
    TextLineLayout,
    ValidationIssue,
)
from rpptx._rpptx import (
    Chart,
    ThemeColors,
    MasterTextStyles,
    TextStyleList,
    AdjustmentCollection,
    Background,
    Cell,
    ColorFormat,
    Column,
    ColumnCollection,
    FillFormat,
    Font,
    Hyperlink,
    Image,
    LineEndFormat,
    LineFormat,
    Paragraph,
    ParagraphCollection,
    PlaceholderCollection,
    Row,
    RowCollection,
    Run,
    RunCollection,
    ShadowFormat,
    Shape,
    ShapeClickAction,
    ShapeCollection,
    ShapeHyperlink,
    Slide,
    SlideCollection,
    SlideLayout,
    SlideLayoutCollection,
    Table,
    TextFrame,
)
from rpptx.enum.dml import (
    MSO_ARROWHEAD_LENGTH,
    MSO_ARROWHEAD_STYLE,
    MSO_ARROWHEAD_WIDTH,
    MSO_FILL_TYPE,
    MSO_LINE_DASH_STYLE,
)
from rpptx.enum.shapes import MSO_CONNECTOR, MSO_SHAPE_TYPE


def exercise_rpptx_types(path: Path) -> None:
    presentation = Presentation(path)
    layout: SlideLayout = presentation.slide_layouts[0]
    slide: Slide = presentation.slides.add_slide(layout)
    shape: Shape = slide.shapes.add_textbox(
        Inches(1), Inches(1), Inches(4), Inches(2)
    )
    shape.text = "typed"
    element: Element = shape.element
    snapshot: str = element.xml
    assert_type(element.replace_xml(snapshot), None)
    assert_type(slide.element.xml, str)
    assert_type(layout.element.xml, str)
    assert_type(presentation.slide_masters[0].element.xml, str)
    assert_type(shape.text_frame.element.xml, str)

    shape_left: Length | None = shape.left
    shape_top: Length | None = shape.top
    shape_width: Length | None = shape.width
    shape_height: Length | None = shape.height
    shape_id: int | None = shape.shape_id
    shape_name: str | None = shape.name
    autofit: str | None = shape.text_frame.autofit
    paragraph: Paragraph = presentation.slides[0].shapes[-1].text_frame.paragraphs[0]
    paragraph.level = 1
    paragraph.font.bold = True
    paragraph.font.size = Pt(12)
    returned_size: Length | None = (
        presentation.slides[0].shapes[-1].text_frame.paragraphs[0].font.size
    )
    run: Run = paragraph.runs[0]
    run.text = "updated"
    run_font_name: str | None = run.font.name
    run_font_size: Length | None = run.font.size
    run_font_color: RGBColor | None = run.font.color.rgb
    frame: TextFrame = presentation.slides[0].shapes[-1].text_frame
    frame.margin_left = Inches(0.1)
    frame.margin_right = None
    frame.margin_top = Pt(2)
    frame.margin_bottom = 0
    margins: tuple[Length | None, ...] = (
        frame.margin_left,
        frame.margin_right,
        frame.margin_top,
        frame.margin_bottom,
    )
    autofit_result: AutofitRefreshResult = presentation.refresh_autofit(fonts=[], font_dirs=[])
    autofit_updates: tuple[AutofitUpdate, ...] = autofit_result.updates
    autofit_diagnostics: tuple[str, ...] = autofit_result.diagnostics
    fitted: AutofitUpdate = frame.fit_text(None, 18, fonts=[], font_dirs=[])
    fit_bounds: tuple[int, int, int, int] = fitted.new_bounds
    fit_point_size: int | None = fitted.point_size
    fit_diagnostics: tuple[str, ...] = fitted.diagnostics
    frame.vertical_anchor = MSO_ANCHOR.MIDDLE
    anchor: MSO_ANCHOR | None = frame.vertical_anchor
    frame.word_wrap = True
    word_wrap: bool | None = frame.word_wrap
    frame.auto_size = MSO_AUTO_SIZE.TEXT_TO_FIT_SHAPE
    auto_size: MSO_AUTO_SIZE | None = frame.auto_size
    paragraph = frame.paragraphs[0]
    paragraph.alignment = PP_ALIGN.CENTER
    alignment: PP_ALIGN | None = paragraph.alignment
    paragraph.line_spacing = 1.5
    paragraph.line_spacing = Pt(18)
    line_spacing: float | Length | None = paragraph.line_spacing
    paragraph.space_before = Pt(6)
    paragraph.space_after = 0.5
    spacing: tuple[Length | float | None, ...] = (
        paragraph.space_before,
        paragraph.space_after,
    )
    paragraph.left_indent = Inches(0.5)
    paragraph.right_indent = None
    paragraph.first_line_indent = -Inches(0.25)
    indents: tuple[Length | None, ...] = (
        paragraph.left_indent,
        paragraph.right_indent,
        paragraph.first_line_indent,
    )
    paragraph.bullet = "-"
    paragraph.bullet = False
    bullet: str | bool | None = paragraph.bullet
    added: Run = paragraph.add_run("added")
    added = frame.paragraphs[0].add_run()
    font: Font = added.font
    font.name = "Arial"
    font.color = RGBColor(0x12, 0x34, 0x56)
    font.color = "123456"
    font.color = None
    font.italic = True
    font.underline = True
    font.underline = MSO_UNDERLINE.DOUBLE_LINE
    underline: bool | MSO_UNDERLINE | None = font.underline
    font.strike = False
    font.all_caps = None
    font_flags: tuple[bool | None, ...] = (font.italic, font.strike, font.all_caps)
    broad_shape_factory: Callable[[int, int, int, int, int], Shape] = (
        presentation.slides[0].shapes.add_shape  # type: ignore[assignment]
    )
    length: Length = Pt(12)
    points: float = length.pt
    emu: int = length.emu
    shape = presentation.slides[0].shapes.add_shape(
        MSO_SHAPE.CHEVRON, Inches(1), Inches(1), Inches(2), Inches(1)
    )
    shape = presentation.slides[0].shapes.add_table(
        1, 1, Inches(1), Inches(1), Inches(4), Inches(1)
    )
    table: Table = shape.table
    column: Column = table.columns[0]
    column.width = Inches(2)
    cell: Cell = table.cell(0, 0)
    cell_anchor: MSO_ANCHOR | None = cell.vertical_anchor
    cell.vertical_anchor = MSO_ANCHOR.MIDDLE
    cell.vertical_anchor = None
    cell.text = paragraph.text
    shapes: list[Shape] = presentation.slides[0].shapes[:]
    for current_slide in presentation.slides:
        for current_shape in current_slide.shapes:
            current_shape.has_text_frame
    package_bytes: bytes = presentation.to_bytes()
    reopened: Presentation = Presentation.from_bytes(package_bytes)
    replaced: int = presentation.try_replace_text("typed", "checked")
    replacement_counts: tuple[int, int] = (0, 0)
    try:
        replaced = presentation.try_replace_text("checked", "typed", expect=2)
    except ReplacementCountError as error:
        replacement_counts = (error.expected, error.found)
    slide_replaced: int = presentation.slides[0].try_replace_text(
        "typed", "checked", expect=None, notes=False
    )
    frame_replaced: int = (
        presentation.slides[0].shapes[0].text_frame.try_replace_text("typed", "checked", expect=0)
    )
    issues: tuple[ValidationIssue, ...] = presentation.validate()
    issue_lines: list[tuple[str, str]] = [(issue.kind, issue.message) for issue in issues]
    pdf_bytes: bytes = presentation.to_pdf()
    slide_png: bytes | None = presentation.render_slide_to_png(0)
    slide_pngs: list[bytes] = presentation.render_all_slides()
    notes_pdf: bytes = presentation.to_notes_pdf()
    notes_pngs: list[bytes] = presentation.render_all_notes()
    notes_text: str | None = presentation.slides[0].notes_text
    presentation.slides[0].notes_text = "Updated speaker note"
    presentation.add_comment_author(
        id="{11111111-1111-1111-1111-111111111111}",
        name="Ada",
        user_id="ada@example.com",
        provider_id="local",
    )
    authors: tuple[CommentAuthor, ...] = presentation.comment_authors
    current_slide = presentation.slides[0]
    current_slide.add_comment(
        id="{22222222-2222-2222-2222-222222222222}",
        author_id=authors[0].id,
        created="2026-09-14T10:30:00Z",
        text="Review",
    )
    comments: tuple[Comment, ...] = presentation.slides[0].comments
    reply: CommentReply = comments[0].replies[0]
    presentation.slides[0].resolve_comment(comments[0].id)
    presentation.slides[0].remove_comment(reply.id)
    slide_width: Length | None = presentation.slide_width
    presentation.slide_height = Inches(6)
    current_slide = presentation.slides[0]
    slide_layout: SlideLayout = current_slide.slide_layout
    layout_index: int = presentation.slide_layouts.index(slide_layout)
    same_layout: bool = slide_layout == presentation.slide_layouts[0]
    current_slide.slide_layout = presentation.slide_layouts[1]
    current_slide = presentation.slides[0]
    hidden: bool = current_slide.hidden
    current_slide.hidden = True
    background: Background = current_slide.background
    background_fill: FillFormat = background.fill
    follows_master: bool = current_slide.follow_master_background
    current_slide.follow_master_background = False
    shape = current_slide.shapes.add_shape(
        "roundRect", Inches(1), Inches(1), Inches(2), Inches(1)
    )
    shape.left = Inches(2)
    shape.top = Inches(2)
    shape.width = Inches(3)
    shape.height = Inches(1)
    geometry: tuple[Length, Length, Length, Length] | None = shape.effective_geometry()
    shape.name = "Typed"
    shape.rotation = 15.0
    rotation: float = shape.rotation
    shape_type: MSO_SHAPE_TYPE | None = shape.shape_type
    shape.auto_shape_type = MSO_SHAPE.ROUNDED_RECTANGLE
    shape.auto_shape_type = "chevron"
    auto_shape_type: MSO_SHAPE | None = shape.auto_shape_type
    member: MSO_SHAPE = MSO_SHAPE.from_xml("roundRect")
    adjustments: AdjustmentCollection = shape.adjustments
    adjustments[0] = 0.25
    first_adjustment: float = adjustments[0]
    all_adjustments: list[float] = list(adjustments)
    fill: FillFormat = shape.fill
    fill.solid()
    fill_type: MSO_FILL_TYPE | None = fill.type
    fore_color: ColorFormat = fill.fore_color
    fore_color.rgb = RGBColor(0x12, 0x34, 0x56)
    rgb: RGBColor | None = fore_color.rgb
    line: LineFormat = shape.line
    line.width = Pt(1)
    line.width = None
    line_width: Length = line.width
    line.color.rgb = RGBColor.from_string("FF0000")
    line_fill: FillFormat = line.fill
    line.dash_style = MSO_LINE_DASH_STYLE.DASH
    line.dash_style = None
    dash_style: MSO_LINE_DASH_STYLE | None = line.dash_style
    tail_end: LineEndFormat = line.tail_end
    tail_end.type = MSO_ARROWHEAD_STYLE.TRIANGLE
    tail_end.width = MSO_ARROWHEAD_WIDTH.WIDE
    tail_end.length = None
    end_type: MSO_ARROWHEAD_STYLE | None = line.head_end.type
    end_width: MSO_ARROWHEAD_WIDTH | None = tail_end.width
    end_length: MSO_ARROWHEAD_LENGTH | None = tail_end.length
    shadow: ShadowFormat = shape.shadow
    shadow.inherit = False
    inherits: bool = shadow.inherit
    shadow.visible = True
    visible: bool = shadow.visible
    shadow.color.rgb = RGBColor(0, 0, 0)
    shadow.alpha = 0.4
    alpha: float | None = shadow.alpha
    shadow.blur_radius = Pt(4)
    blur_radius: Length | None = shadow.blur_radius
    shadow.distance = Pt(3)
    distance: Length | None = shadow.distance
    shadow.direction = 45.0
    direction: float | None = shadow.direction
    shadow.align = "tl"
    align: str | None = shadow.align
    shadow.rotate_with_shape = False
    rotate_with_shape: bool | None = shadow.rotate_with_shape
    shape_xml: bytes = shape.xml
    connector: Shape = presentation.slides[0].shapes.add_connector(
        MSO_CONNECTOR.STRAIGHT, 0, 0, Inches(1), Inches(1)
    )
    connector.theme_effect_index = 0
    theme_effect_index: int | None = connector.theme_effect_index
    group: Shape = presentation.slides[0].shapes.add_group_shape()
    picture = presentation.slides[0].shapes.add_picture(
        io.BytesIO(b""), 0, 0
    )
    image: Image = picture.image
    blob: bytes = image.blob
    picture.replace_image(b"")
    picture.replace_image(path)
    presentation.slides[0].shapes.remove(group)
    duplicated: Slide = presentation.slides.duplicate(presentation.slides[0])
    presentation.slides.move(0, -1)
    imported: Slide = presentation.slides.import_slide(
        presentation.slides[0], layout=presentation.slide_layouts[0], index=0
    )
    presentation.slides.remove(presentation.slides[0])
    presentation.save(path)
    (
        package_bytes,
        pdf_bytes,
        slide_png,
        slide_pngs,
        notes_pdf,
        notes_pngs,
        notes_text,
        authors,
        comments,
        reply,
        shapes,
        returned_size,
        points,
        emu,
        broad_shape_factory,
        shape_left,
        shape_top,
        shape_width,
        shape_height,
        shape_id,
        shape_name,
        autofit,
        run_font_name,
        run_font_size,
        run_font_color,
        margins,
        anchor,
        word_wrap,
        auto_size,
        alignment,
        line_spacing,
        spacing,
        indents,
        bullet,
        underline,
        font_flags,
        slide_width,
        layout_index,
        same_layout,
        hidden,
        background_fill,
        follows_master,
        rotation,
        shape_type,
        auto_shape_type,
        member,
        first_adjustment,
        all_adjustments,
        fill_type,
        rgb,
        line_width,
        line_fill,
        shape_xml,
        connector,
        theme_effect_index,
        blob,
        image.content_type,
        image.ext,
        duplicated,
        reopened,
        replaced,
        replacement_counts,
        slide_replaced,
        frame_replaced,
        issue_lines,
    )


def exercise_rpptx_table_types(table: Table) -> None:
    origin: Cell = table.cell(0, 0)
    cell_frame: TextFrame = origin.text_frame
    cell_frame.word_wrap = True
    cell_frame.paragraphs[0].font.bold = True
    origin.merge(table.cell(1, 1))
    spans: tuple[bool, bool, int, int] = (
        origin.is_merge_origin,
        origin.is_spanned,
        origin.span_height,
        origin.span_width,
    )
    origin.split()
    cell_fill: FillFormat = origin.fill
    cell_fill.solid()
    origin.margin_left = Inches(0.1)
    origin.margin_right = None
    cell_margins: tuple[Length | None, ...] = (
        origin.margin_left,
        origin.margin_right,
        origin.margin_top,
        origin.margin_bottom,
    )
    rows: RowCollection = table.rows
    row: Row = rows[0]
    row.height = Inches(1)
    row_heights: list[Length] = [current.height for current in rows]
    borders: tuple[LineFormat, ...] = (
        origin.border_left,
        origin.border_right,
        origin.border_top,
        origin.border_bottom,
    )
    borders[0].width = Pt(1)
    (spans, cell_margins, row_heights, rows[:])


def exercise_rpptx_table_structure_types(table: Table) -> None:
    appended: Row = table.rows.add_row()
    inserted: Row = table.rows.add_row(0)
    table.rows.remove(table.rows[-1])
    column: Column = table.columns.add_column()
    table.columns.add_column(index=-1)
    table.columns.remove(table.columns[0])
    (appended, inserted, column)


def exercise_rpptx_picture_crop_types(picture: Shape) -> None:
    picture.crop_left = 0.25
    picture.crop_top = 0
    crop: tuple[float, float, float, float] = (
        picture.crop_left,
        picture.crop_top,
        picture.crop_right,
        picture.crop_bottom,
    )
    picture.crop_right = crop[0]
    picture.crop_bottom = -0.1


def exercise_rpptx_z_order_types(slide: Slide) -> None:
    slide.shapes.move(0, -1)


def exercise_rpptx_group_population_types(group: Shape) -> None:
    members: ShapeCollection = group.shapes
    textbox: Shape = members.add_textbox(0, 0, Inches(1), Inches(1))
    nested: Shape = group.shapes.add_group_shape()
    nested.shapes.add_shape("rect", 0, 0, Inches(1), Inches(1))
    group.shapes.add_connector(MSO_CONNECTOR.STRAIGHT, 0, 0, Inches(1), Inches(1))
    group.shapes.add_table(1, 2, 0, 0, Inches(2), Inches(1))
    group.shapes.add_picture(io.BytesIO(b""), 0, 0)


def exercise_rpptx_hyperlink_types(run: Run) -> None:
    hyperlink: Hyperlink = run.hyperlink
    hyperlink.address = "https://example.com"
    address: str | None = hyperlink.address
    hyperlink.address = None
    (address,)


def exercise_rpptx_click_action_types(shape: Shape, slide: Slide) -> None:
    action: ShapeClickAction = shape.click_action
    link: ShapeHyperlink = action.hyperlink
    link.address = "https://example.com"
    link.address = None
    action.target_slide = slide
    target: Slide | None = action.target_slide
    action.target_slide = None
    same: bool = target == slide
    (same,)


def exercise_rpptx_text_layout_types(presentation: Presentation) -> None:
    frames: tuple[TextFrameLayout, ...] = presentation.text_layout()
    narrower: tuple[TextFrameLayout, ...] = presentation.text_layout(width_factor=0.95)
    for frame in frames + narrower:
        slide_index: int = frame.slide_index
        shape_id: int | None = frame.shape_id
        name: str | None = frame.name
        autofit: str = frame.autofit
        box: BoundingBox = frame.frame
        usable: BoundingBox = frame.usable
        box_edges: tuple[float, float, float, float] = (
            usable.x,
            usable.y,
            usable.width,
            usable.height,
        )
        font_scale: float = frame.font_scale
        height: float = frame.height
        overflow: bool = frame.overflow
        lines: tuple[TextLineLayout, ...] = frame.lines
        for line in lines:
            paragraph_index: int = line.paragraph_index
            text: str = line.text
            bounds: BoundingBox = line.bounds
            baseline: float = line.baseline
            font_size: float = line.font_size
            (paragraph_index, text, bounds, baseline, font_size)
        (
            slide_index,
            shape_id,
            name,
            autofit,
            box,
            box_edges,
            font_scale,
            height,
            overflow,
        )


if TYPE_CHECKING:
    BoundingBox()  # type: ignore[call-arg]
    AdjustmentCollection()  # type: ignore[call-arg]
    Background()  # type: ignore[call-arg]
    Cell()  # type: ignore[call-arg]
    ColorFormat()  # type: ignore[call-arg]
    FillFormat()  # type: ignore[call-arg]
    Image()  # type: ignore[call-arg]
    LineFormat()  # type: ignore[call-arg]
    LineEndFormat()  # type: ignore[call-arg]
    Comment()  # type: ignore[call-arg]
    CommentAuthor()  # type: ignore[call-arg]
    CommentReply()  # type: ignore[call-arg]
    Column()  # type: ignore[call-arg]
    ColumnCollection()  # type: ignore[call-arg]
    Font()  # type: ignore[call-arg]
    Hyperlink()  # type: ignore[call-arg]
    Paragraph()  # type: ignore[call-arg]
    ParagraphCollection()  # type: ignore[call-arg]
    PlaceholderCollection()  # type: ignore[call-arg]
    Row()  # type: ignore[call-arg]
    RowCollection()  # type: ignore[call-arg]
    Run()  # type: ignore[call-arg]
    RunCollection()  # type: ignore[call-arg]
    ShadowFormat()  # type: ignore[call-arg]
    Shape()  # type: ignore[call-arg]
    ShapeClickAction()  # type: ignore[call-arg]
    ShapeCollection()  # type: ignore[call-arg]
    ShapeHyperlink()  # type: ignore[call-arg]
    Slide()  # type: ignore[call-arg]
    SlideCollection()  # type: ignore[call-arg]
    SlideLayout()  # type: ignore[call-arg]
    SlideLayoutCollection()  # type: ignore[call-arg]
    Table()  # type: ignore[call-arg]
    TextFrame()  # type: ignore[call-arg]
    TextFrameLayout()  # type: ignore[call-arg]
    TextLineLayout()  # type: ignore[call-arg]
    ValidationIssue()  # type: ignore[call-arg]


def colour_formats() -> None:
    from rpptx import Presentation, MSO_THEME_COLOR
    from rpptx._rpptx import ColorFormat
    from rpptx.dml.color import RGBColor
    presentation = Presentation()
    slide = presentation.slides.add_slide(presentation.slide_layouts[6])
    font = slide.shapes.add_textbox(0, 0, 914400, 914400).text_frame.paragraphs[0].font
    assert_type(font.color, ColorFormat)
    assert_type(font.color.rgb, RGBColor | None)
    assert_type(font.color.theme_color, MSO_THEME_COLOR)
    assert_type(font.color.brightness, float | None)
    font.color = "#123456"
    font.color = (1, 2, 3)
    font.color.rgb = None
    font.color.theme_color = MSO_THEME_COLOR.ACCENT_1
    font.color.brightness = 0.25


def fx195_caller_font_render_signatures(presentation: Presentation, directory: Path) -> None:
    fonts: list[tuple[str, bytes]] = [("caller", b"font")]
    dirs: list[str | Path] = [directory, str(directory)]
    assert_type(presentation.to_pdf(fonts=fonts, font_dirs=dirs), bytes)
    assert_type(presentation.render_slide_to_png(0, fonts=fonts, font_dirs=dirs), bytes | None)
    assert_type(presentation.render_all_slides(fonts=fonts, font_dirs=dirs), list[bytes])
    assert_type(presentation.text_layout(fonts=fonts, font_dirs=dirs), tuple[TextFrameLayout, ...])
    assert_type(presentation.to_notes_pdf(fonts=fonts, font_dirs=dirs), bytes)
    assert_type(presentation.render_all_notes(fonts=fonts, font_dirs=dirs), list[bytes])
    assert_type(presentation.to_pdf(font_dir=directory), bytes)


def _fx206_run_target_types(presentation: Presentation, slide: Slide) -> None:
    link = slide.shapes[0].text_frame.paragraphs[0].runs[0].hyperlink
    assert_type(link.target_slide, Slide | None)
    link.target_slide = slide
    link.target_slide = None

def fx210_notes_diagnostic_companion_signatures(presentation: Presentation, directory: Path) -> None:
    fonts: list[tuple[str, bytes]] = [("Aptos", b"font")]
    dirs: list[str | Path] = [directory, str(directory)]
    assert_type(presentation.to_notes_pdf_with_diagnostics(fonts=fonts, font_dir=directory, font_dirs=dirs), tuple[bytes, tuple[str, ...]])
    assert_type(presentation.render_all_notes_with_diagnostics(72.0, fonts=fonts, font_dir=directory, font_dirs=dirs), tuple[list[bytes], tuple[str, ...]])


def fx208_authoring_signatures(presentation: Presentation, slide: Slide) -> None:
    from rpptx import HeaderFooter, PlaceholderFormat, SlideMaster, SlideTransition, Theme
    from rpptx.dml.color import RGBColor
    from rpptx.enum.shapes import PP_PLACEHOLDER_TYPE
    presentation.set_header_footer(slide_number=True, footer="footer", date="auto", hide_on_title=False, date_format="datetime13", date_cache="")
    assert_type(presentation.slide_number_start, int | None)
    presentation.slide_number_start = 0
    presentation.slide_number_start = None
    footer = slide.header_footer
    assert_type(footer, HeaderFooter)
    assert_type(footer.slide_number_policy, bool | None)
    assert_type(footer.footer_policy, bool | None)
    assert_type(footer.date_policy, bool | None)
    assert_type(footer.policy_source, str)
    assert_type(footer.date_cache, str | None)
    footer.date = "fixed"
    footer.footer = None
    master = presentation.slide_masters[0]
    assert_type(master, SlideMaster)
    theme = master.theme
    assert_type(theme, Theme)
    assert_type(theme.colors, ThemeColors)
    assert_type(theme.fonts.major.latin, str)
    assert_type(theme.fonts.minor.east_asian, str)
    transition = slide.transition
    assert_type(transition, SlideTransition)
    transition.type = "split"
    transition.direction = "horizontal-in"
    transition.duration = 1.25
    assert_type(transition.advance_on_click, bool | None)
    transition.advance_on_click = False
    transition.advance_after = 0.0
    transition.apply_to_all()
    metadata = slide.shapes[0].placeholder_format
    assert_type(metadata, PlaceholderFormat)
    assert_type(metadata.type, PP_PLACEHOLDER_TYPE)
    assert_type(metadata.idx, int)
    slide.shapes[0].text_frame.paragraphs[0].add_field("slidenum", "")


def _fx205_typed_parity_surface(prs: Presentation) -> None:
    from rpptx._rpptx import AppProperties, CoreProperties, CustomProperties, Section, MediaInfo
    assert_type(prs.core_properties, CoreProperties)
    assert_type(prs.app_properties, AppProperties)
    assert_type(prs.custom_properties, CustomProperties)
    assert_type(prs.sections, tuple[Section, ...])
    slide = prs.slides[0]
    assert_type(slide.media, tuple[MediaInfo, ...])
    paragraph = slide.shapes[0].text_frame.paragraphs[0]
    font = paragraph.runs[0].font
    assert_type(font.baseline, float | None)
    assert_type(font.spacing, Length | None)
    assert_type(font.language, str | None)
    assert_type(font.east_asian_name, str | None)
    assert_type(font.complex_script_name, str | None)
    paragraph.auto_number = "arabicPeriod"
    paragraph.auto_number_start = 3
    paragraph.bullet_color = RGBColor(0x33, 0x66, 0x99)
    paragraph.bullet_font = "Arial"
    assert_type(paragraph.bullet_size, float | Length | None)
    paragraph.bullet_size = 1.25
    paragraph.bullet_size = Pt(17.85)
    paragraph.bullet_size = None
    fill = slide.shapes[0].fill
    assert_type(fill.transparency, float | None)
    fill.gradient_path = "circle"
    assert_type(prs.render_slides(format="jpeg", quality=90, fonts=[]), list[bytes])
    assert_type(prs.render_slides(format="tiff", fonts=[]), bytes)
    assert_type(prs.to_pdfa(profile="pdfa-2b", fonts=[]), bytes)
    assert_type(prs.to_handout_pdf(slides_per_page=6, fonts=[]), bytes)
    assert_type(prs.to_odp(), tuple[bytes, list[tuple[str, str]]])


def _fx209_part_owned_typed_surface(prs: Presentation) -> None:
    master = prs.slide_master
    layout = prs.slide_layouts[0]
    assert_type(master.text_styles, MasterTextStyles)
    assert_type(master.text_styles.body, TextStyleList)
    assert_type(master.text_styles.body.level(1), Paragraph)
    assert_type(layout.used_by_slides, tuple[Slide, ...])
    assert_type(layout.show_master_shapes, bool | None)
    layout.show_master_shapes = False
    layout.follow_master_background = True
    copied = prs.slide_layouts.duplicate(layout)
    assert_type(copied, SlideLayout)
    prs.slide_layouts.remove(copied)
    colors = master.theme.colors
    assert_type(colors.keys(), tuple[str, ...])
    assert_type(colors["accent1"], RGBColor | None)
    colors["accent1"] = RGBColor(17, 34, 51)
    master.theme.fonts.major.latin = "Carlito"
    assert_type(prs.apply_theme(prs.to_bytes(), source_master=0,
                               import_masters=False, layout_map=[]), None)


def fx204_shared_operation_types(prs: Presentation) -> None:
    assert_type(prs.replace_all([("x", "y", 1)]), tuple[int, ...])
    assert_type(prs.slides.add_slide_at(prs.slide_layouts[6], 0), Slide)
    report = prs.fit_report()
    assert_type(report, FitReport)
    assert_type(report.frames, tuple[FitFrameReport, ...])
    frame = report.frames[0]
    assert_type(frame.quantized_trial, FitTrial | None)
    assert_type(frame.persisted_trial, FitTrial | None)
    assert_type(frame.update.child_path, list[int])
    assert_type(frame.update.name, str | None)


def check_fx220_font_embedding_types() -> None:
    from rpptx import Presentation, FontEmbeddingLicense, EmbeddedTypeface, EmbeddedFontFace
    from typing import assert_type
    value = Presentation()
    license = FontEmbeddingLicense(authorized=True, identity="OFL 1.1")
    assert_type(license.authorized, bool)
    assert_type(license.identity, str)
    value.embed_font("typed face", b"font", license=license)
    assert_type(value.embedded_fonts, tuple[EmbeddedTypeface, ...])
    assert_type(value.remove_embedded_font("typed face"), EmbeddedFontFace | None)
    assert_type(value.remove_font("typed face"), EmbeddedTypeface | None)
    assert_type(value.embed_true_type_fonts, bool | None)
    value.embed_true_type_fonts = True
    value.save_subset_fonts = False


def fx220_licensed_embedding_recipe(font_path: Path, output_path: Path) -> None:
    from rpptx import FontEmbeddingLicense, EmbeddedTypeface
    value = Presentation()
    license = FontEmbeddingLicense(authorized=True, identity="SIL Open Font License 1.1")
    value.embed_font("My Face", font_path, "regular", license=license)
    value.save(output_path)
    assert_type(value.embedded_fonts, tuple[EmbeddedTypeface, ...])
    value.embed_true_type_fonts = False
    value.remove_embedded_font("My Face", "regular")


def fx215_chart_typing() -> None:
    from rpptx import CategoryChartData, XyChartData, XL_CHART_TYPE
    data = CategoryChartData()
    data.categories = ["East", "West"]
    series = data.add_series("Sales", [11.0, 23.0])
    assert_type(series.values, tuple[float, ...])
    xy = XyChartData()
    xy.add_series("A").add_data_point(1.0, 2.0)
    from rpptx import Presentation, Chart, Inches
    presentation = Presentation()
    slide = presentation.slides.add_slide(presentation.slide_layouts[6])
    chart = slide.shapes.add_chart(XL_CHART_TYPE.LINE, Inches(1), Inches(1), Inches(4), Inches(2), data).chart
    assert_type(chart, Chart)
    assert_type(chart.has_legend, bool)
    assert_type(chart.legend_overlay, bool)
    chart.has_legend = True
    chart.legend_position = "b"
    chart.legend_overlay = False
    assert_type(chart.categories, tuple[str, ...])
    chart.replace_data(data)


def semantic_text_exports(prs: Presentation, path: Path) -> None:
    assert_type(prs.to_html(include_notes=True), str)
    assert_type(prs.to_markdown(skip_hidden=True), str)
    result = prs.export_html(inline_images=False)
    assert_type(result, PresentationTextExport)
    assert_type(result.text, str)
    assert_type(result.assets, tuple[tuple[str, bytes], ...])
    assert_type(result.diagnostics, tuple[str, ...])
    assert_type(prs.export_markdown(), PresentationTextExport)
    assert_type(prs.save_html(path), tuple[str, ...])
    assert_type(prs.save_markdown(path, inline_images=True), tuple[str, ...])


def fx211c_compatibility_aliases(prs: Presentation, shape: Shape, row: Row, font: Font) -> None:
    assert_type(row.cells, tuple[Cell, ...])
    paragraph = shape.text_frame.paragraphs[0]
    assert_type(paragraph.bullet_char, str | bool | None)
    paragraph.bullet_char = "•"
    paragraph.bullet_char = False
    paragraph.bullet_char = None
    assert_type(shape.alt_text, str | None)
    shape.alt_text = "Description"
    font.highlight_color = "FFFF00"
    assert_type(font.highlight_color, ColorFormat)
    theme = prs.slide_masters[0].theme
    theme.name = "Typed theme"
    assert_type(theme.name, str | None)
    assert_type(prs.core_properties.author, str | None)
    shapes = prs.slides[0].shapes
    shapes.align([shape], "left", relative_to="selection")
    shapes.distribute([shape], "horizontal", relative_to="selection")


if TYPE_CHECKING:
    from rpptx import NotesSlide, NotesTextFrame
    from rpptx.enum.dml import MSO_COLOR_TYPE, MSO_THEME_COLOR_INDEX

    def fx239_notes_typing(slide: Slide) -> None:
        present: bool = slide.has_notes_slide
        notes: NotesSlide = slide.notes_slide
        frame: NotesTextFrame | None = notes.notes_text_frame
        if frame is not None:
            text: str = frame.text
            frame.text = text
        color: int = MSO_COLOR_TYPE.RGB
        theme: int = MSO_THEME_COLOR_INDEX.ACCENT_1
        assert present or color or theme


def fx240_owned_signature_reports(owner: Presentation, private_key: bytes, certificate: bytes) -> None:
    report = owner.sign(private_key, certificate)
    part: str = report["signature_part"]
    valid: bool = report["cryptographically_valid"]
    complete: bool = report["coverage_complete"]
    parts: list[str] = report["covered_parts"]
    if report["signer"] is not None:
        subject: str = report["signer"]["subject"]
        issuer: str = report["signer"]["issuer"]
        serial: str = report["signer"]["serial_number"]
        print(subject, issuer, serial)
    for relationship in report["covered_relationships"]:
        print(relationship["source_part"], relationship["relationship_id"])
    for issue in report["issues"]:
        details: dict[str, str] = issue["details"]
        print(issue["kind"], details)
    for checked in owner.verify_signatures():
        print(checked["signature_part"], checked["cryptographically_valid"])
    print(part, valid, complete, parts)


def fx253_optional_chart_type(chart: "Chart") -> int:
    assert_type(chart.chart_type, int | None)
    value = chart.chart_type
    if value is None:
        return -1
    assert_type(value, int)
    return value


def fx258_powerpoint_names(slide: Slide, font: Font) -> None:
    from rpptx.enum.dml import MSO_COLOR_TYPE, MSO_THEME_COLOR
    from rpptx.enum.lang import MSO_LANGUAGE_ID
    assert_type(slide.slide_id, int)
    assert_type(font.color.type, MSO_COLOR_TYPE | None)
    assert_type(font.color.theme_color, MSO_THEME_COLOR)
    assert_type(font.language_id, MSO_LANGUAGE_ID)
    assert_type(MSO_LANGUAGE_ID(1033), MSO_LANGUAGE_ID)
    font.language_id = MSO_LANGUAGE_ID.FRENCH
    font.language_id = 1033
    font.language_id = None
    font.language = "en-US"
