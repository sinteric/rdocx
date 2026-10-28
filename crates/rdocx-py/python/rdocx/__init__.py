"""Python bindings for rdocx."""

from __future__ import annotations

from typing import NoReturn as _Never, SupportsIndex as _SupportsIndex, overload as _overload

from .enum import MSO_THEME_COLOR
from .enum.table import WD_CELL_VERTICAL_ALIGNMENT, WD_ROW_HEIGHT_RULE, WD_TABLE_ALIGNMENT
from .enum.text import WD_ALIGN_PARAGRAPH, WD_TAB_ALIGNMENT, WD_TAB_LEADER, WD_UNDERLINE
from .shared import Cm, Emu, Inches, Length, Mm, Pt, RGBColor, Twips


class RdocxError(Exception):
    """Base class for errors raised by rdocx."""


class PackageError(RdocxError):
    """An OPC package, file, or document-part operation failed."""


class XmlError(RdocxError):
    """WordprocessingML could not be parsed or serialized."""


class StaleElementError(RdocxError):
    """A held content handle was invalidated by structural mutation."""


class LayoutError(RdocxError):
    """Document layout or rendering failed."""


class ReplacementCountError(RdocxError):
    """A counted replacement matched a different number of times than expected.

    ``index`` is the position of the failing pair in a ``Document.replace_all``
    batch, and ``None`` for ``Document.try_replace_text``.
    """

    def __init__(
        self, message: str, expected: int, found: int, index: "int | None" = None
    ) -> None:
        super().__init__(message, expected, found, index)
        self.expected = expected
        self.found = found
        self.index = index

    def __str__(self) -> str:
        return str(self.args[0])


from ._rdocx import (
    HtmlDiagnostic,
    FontEmbeddingLicense,
    EmbeddedFont,
    FontDefinition,

    OdtDiagnostic,
    RtfDiagnostic,
    EpubDiagnostic,
    TextDiagnostic,
    ValidationFinding,
    OdtExportResult,
    RtfExportResult,
    EpubExportResult,
    TextReport,
    MathConversionDiagnostic,
    MathConversionResult,
    EquationConversionResult,
    Equation,
    EquationInsertResult,
    FragmentConflictPolicy,
    ContentLocation,
    DocumentFragment,
    Picture,
    PictureOptions,
    ContentControl,
    ContentControlTypePayload,
    MailMergePackageSource,
    MailMergeSettings,
    MailMergeFieldMapping,
    MailMergeRecipient,
    MailMergePackageOptions,
    MailMergePackageDiagnostic,
    MailMergePreparedData,
    LegacyFormField,
    LegacyFormFieldDefinition,
    FormFieldTarget,
    FormFieldValue,
    FormResetValue,
    FormFieldRule,
    FormFieldAssignment,
    FormValidationIssue,
    DocumentProtection,
    CustomFileTime,
    CustomProperty,
    CustomProperties,
    AppProperties,
    ValidationReport,
    validate_bytes,
    validate_file,
    Bookmark,
    BoundingBox,
    Cell,
    CellCollection,
    CellParagraphCollection,
    Comment,
    ComparisonDiagnostic,
    ContentFragment,
    CoreProperties,
    DefaultParagraphFormat,
    Document,
    Font,
    ColorFormat,
    HeaderFooterVariant,
    HeaderFooter,
    SectionHandle,
    Settings,
    FootnoteEndnote,
    Hyperlink,
    LayoutFragment,
    FieldEvaluation,
    FieldsUpdateReport,
    LayoutBackedFieldUpdateReport,
    LayoutPage,
    ListLevel,
    Chart,
    Paragraph,
    ParagraphCollection,
    ParagraphFormat,
    Revision,
    Row,
    RowCollection,
    Run,
    RunCollection,
    RunPosition,
    RunRange,
    Section,
    Story,
    StoryItem,
    StoryRunPosition,
    StoryRunRange,
    Style,
    SvgDiagnostic,
    SvgRenderResult,
    Table,
    TableCollection,
    TocRebuildReport,
)

__all__ = [
    "HtmlDiagnostic",
    "FontEmbeddingLicense",
    "EmbeddedFont",
    "FontDefinition",

    "OdtDiagnostic",
    "RtfDiagnostic",
    "EpubDiagnostic",
    "TextDiagnostic",
    "ValidationFinding",
    "OdtExportResult",
    "RtfExportResult",
    "EpubExportResult",
    "TextReport",
    "MathConversionDiagnostic",
    "MathConversionResult",
    "EquationConversionResult",
    "Equation",
    "EquationInsertResult",
    "FragmentConflictPolicy",
    "ContentLocation",
    "DocumentFragment",
    "Picture",
    "PictureOptions",
    "ContentControl",
    "ContentControlTypePayload",
    "MailMergePackageSource",
    "MailMergeSettings",
    "MailMergeFieldMapping",
    "MailMergeRecipient",
    "MailMergePackageOptions",
    "MailMergePackageDiagnostic",
    "MailMergePreparedData",
    "LegacyFormField",
    "LegacyFormFieldDefinition",
    "FormFieldTarget",
    "FormFieldValue",
    "FormResetValue",
    "FormFieldRule",
    "FormFieldAssignment",
    "FormValidationIssue",
    "DocumentProtection",
    "CustomFileTime",
    "CustomProperty",
    "CustomProperties",
    "AppProperties",
    "ValidationReport",
    "validate_bytes",
    "validate_file",

    "MSO_THEME_COLOR",
    "Bookmark",
    "BoundingBox",
    "Cm",
    "Cell",
    "CellCollection",
    "CellParagraphCollection",
    "Comment",
    "ComparisonDiagnostic",
    "ContentFragment",
    "CoreProperties",
    "DefaultParagraphFormat",
    "Document",
    "Emu",
    "Font",
    "ColorFormat",
    "HeaderFooterVariant",
    "HeaderFooter",
    "SectionHandle",
    "Settings",
    "FootnoteEndnote",
    "Hyperlink",
    "Inches",
    "LayoutError",
    "Length",
    "LayoutFragment",
    "FieldEvaluation",
    "FieldsUpdateReport",
    "LayoutBackedFieldUpdateReport",
    "LayoutPage",
    "ListLevel",
    "Chart",
    "Mm",
    "PackageError",
    "Paragraph",
    "ParagraphCollection",
    "ParagraphFormat",
    "Pt",
    "RGBColor",
    "RdocxError",
    "ReplacementCountError",
    "Revision",
    "Row",
    "RowCollection",
    "Run",
    "RunCollection",
    "RunPosition",
    "RunRange",
    "Section",
    "StaleElementError",
    "Story",
    "StoryItem",
    "StoryRunPosition",
    "StoryRunRange",
    "Style",
    "SvgDiagnostic",
    "SvgRenderResult",
    "Twips",
    "Table",
    "TableCollection",
    "TocRebuildReport",
    "WD_ALIGN_PARAGRAPH",
    "WD_CELL_VERTICAL_ALIGNMENT",
    "WD_ROW_HEIGHT_RULE",
    "WD_TAB_ALIGNMENT",
    "WD_TAB_LEADER",
    "WD_TABLE_ALIGNMENT",
    "WD_UNDERLINE",
    "XmlError",
]


"""Chart data for `Document.add_chart`, the same objects as python-pptx `pptx.chart.data`."""

from typing import Iterable, List, Optional, Tuple
from enum import IntEnum

_ChartParts = Tuple[List[str], List[Tuple[str, List[float]]], Optional[str]]


def _shared_number_format(
    chart_format: str, series_formats: Iterable[Optional[str]]
) -> Optional[str]:
    """Return the one number format every series uses, `None` for `General`."""

    formats = {chart_format if item is None else item for item in series_formats}
    if len(formats) > 1:
        raise ValueError("every series of a chart must use the same number format")
    shared = formats.pop() if formats else chart_format
    return None if shared == "General" else shared


class CategorySeriesData:
    """One series of a category chart, its name and one value per category."""

    def __init__(self, name: str, values: Iterable[float], number_format: Optional[str]) -> None:
        self._name = name
        self._values = tuple(float(value) for value in values)
        self._number_format = number_format

    @property
    def name(self) -> str:
        return self._name

    @property
    def values(self) -> Tuple[float, ...]:
        return self._values

    @property
    def number_format(self) -> Optional[str]:
        """The series number format, or `None` to use the chart's."""

        return self._number_format


class CategoryChartData:
    """Categories and series of a column, line, area, pie, doughnut or radar chart.

    Categories are text labels. Every series uses one number format, so a
    series format that differs from another raises when the chart is added.
    """

    def __init__(self, number_format: str = "General") -> None:
        self._number_format = number_format
        self._categories: List[str] = []
        self._series: List[CategorySeriesData] = []

    @property
    def number_format(self) -> str:
        return self._number_format

    @property
    def categories(self) -> Tuple[str, ...]:
        return tuple(self._categories)

    @categories.setter
    def categories(self, labels: Iterable[str]) -> None:
        labels = list(labels)
        if not all(isinstance(label, str) for label in labels):
            raise TypeError("category labels must be strings")
        self._categories = labels

    def add_series(
        self, name: str, values: Iterable[float] = (), number_format: Optional[str] = None
    ) -> CategorySeriesData:
        """Append a series with one value per category and return it."""

        series = CategorySeriesData(name, values, number_format)
        self._series.append(series)
        return series

    def _data(self) -> _ChartParts:
        number_format = _shared_number_format(
            self._number_format, (series.number_format for series in self._series)
        )
        series = [(item.name, list(item.values)) for item in self._series]
        return list(self._categories), series, number_format


class XySeriesData:
    """One series of an XY scatter chart, its name and its (x, y) points."""

    def __init__(self, name: str, number_format: Optional[str]) -> None:
        self._name = name
        self._number_format = number_format
        self._points: List[Tuple[float, float]] = []

    @property
    def name(self) -> str:
        return self._name

    @property
    def number_format(self) -> Optional[str]:
        """The series number format, or `None` to use the chart's."""

        return self._number_format

    @property
    def x_values(self) -> Tuple[float, ...]:
        return tuple(x for x, _ in self._points)

    @property
    def y_values(self) -> Tuple[float, ...]:
        return tuple(y for _, y in self._points)

    def add_data_point(self, x: float, y: float) -> None:
        """Append one point."""

        self._points.append((float(x), float(y)))


class XyChartData:
    """Series of an XY scatter chart.

    The chart stores one set of x values for all its series, so every series
    must have the same x values, in the same order, when the chart is added.
    """

    def __init__(self, number_format: str = "General") -> None:
        self._number_format = number_format
        self._series: List[XySeriesData] = []

    @property
    def number_format(self) -> str:
        return self._number_format

    def add_series(self, name: str, number_format: Optional[str] = None) -> XySeriesData:
        """Append an empty series and return it for `add_data_point`."""

        series = XySeriesData(name, number_format)
        self._series.append(series)
        return series

    def _data(self) -> _ChartParts:
        number_format = _shared_number_format(
            self._number_format, (series.number_format for series in self._series)
        )
        x_values = self._series[0].x_values if self._series else ()
        if any(series.x_values != x_values for series in self._series):
            raise ValueError("every series of an XY chart must have the same x values")
        series = [(item.name, list(item.y_values)) for item in self._series]
        return [repr(x) for x in x_values], series, number_format


__all__ += ["CategoryChartData", "CategorySeriesData", "XyChartData", "XySeriesData", "XL_CHART_TYPE"]

class XL_CHART_TYPE(IntEnum):
    AREA = 1
    COLUMN_CLUSTERED = 51
    DOUGHNUT = -4120
    LINE = 65
    PIE = 5
    RADAR = 81
    XY_SCATTER = 74


class _StyleSnapshots(tuple[Style, ...]):
    """Immutable snapshots, with an explicit alternative to mutable lookup."""

    @_overload
    def __getitem__(self, index: _SupportsIndex) -> Style: ...

    @_overload
    def __getitem__(self, index: slice) -> tuple[Style, ...]: ...

    @_overload
    def __getitem__(self, index: str) -> _Never: ...

    def __getitem__(
        self, index: _SupportsIndex | slice | str
    ) -> Style | tuple[Style, ...]:
        if isinstance(index, str):
            raise NotImplementedError(
                "styles are snapshots: use document.set_style(name, font_name=..., font_size=...)"
            )
        return tuple.__getitem__(self, index)
