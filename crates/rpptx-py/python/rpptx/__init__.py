"""Python bindings for rpptx."""

from .enum.dml import MSO_COLOR_TYPE, MSO_THEME_COLOR_INDEX, MSO_THEME_COLOR, MSO_PATTERN_TYPE, MSO_PATTERN
from .dml.color import RGBColor
from .enum.shapes import MSO_SHAPE, PP_PLACEHOLDER, PP_PLACEHOLDER_TYPE
from .enum.text import MSO_ANCHOR, MSO_AUTO_SIZE, MSO_UNDERLINE, PP_ALIGN
from .util import Inches, Length, Pt


class RpptxError(Exception):
    """Base class for errors raised by rpptx."""


class PackageError(RpptxError):
    """An OPC package, file, or presentation operation failed."""


class XmlError(RpptxError):
    """PresentationML could not be parsed or serialized."""


class StaleElementError(RpptxError):
    """A held content handle was invalidated by structural mutation."""


class ReplacementCountError(RpptxError):
    """A counted replacement matched a different number of times than expected."""

    def __init__(self, message: str, expected: int, found: int) -> None:
        super().__init__(message, expected, found)
        self.expected = expected
        self.found = found
        self.index: int | None = None
        self.placeholder: str | None = None

    def __str__(self) -> str:
        return str(self.args[0])


from ._rpptx import FontEmbeddingLicense, EmbeddedFont, EmbeddedFontFace, EmbeddedTypeface
from ._rpptx import Element, Comment, CommentAuthor, CommentReply, Presentation, PresentationTextExport, CoreProperties, AppProperties, CustomProperties, CustomProperty, CustomFileTime, Section, MediaInfo, GradientStops, GradientStop
from ._rpptx import NotesSlide, NotesTextFrame, SlideTransition, HeaderFooter, PlaceholderFormat, SlideMaster, SlideMasterCollection, Theme, ThemeFonts, ThemeFontSet, ThemeColors, MasterTextStyles, TextStyleList
from ._rpptx import FitReport, FitFrameReport, FitTrial, AutofitRefreshResult, AutofitUpdate, BoundingBox, TextFrameLayout, TextLineLayout, ValidationIssue

__all__ = [
    "Element",
    "FontEmbeddingLicense",
    "EmbeddedFont",
    "EmbeddedFontFace",
    "EmbeddedTypeface",

    "MSO_PATTERN_TYPE", "MSO_PATTERN", "CoreProperties", "AppProperties", "CustomProperties", "CustomProperty", "CustomFileTime", "Section", "MediaInfo", "GradientStops", "GradientStop",
    "MSO_THEME_COLOR", "MSO_THEME_COLOR_INDEX", "MSO_COLOR_TYPE",
    "NotesSlide", "NotesTextFrame",
    "BoundingBox",
    "FitReport",
    "FitFrameReport",
    "FitTrial",
    "AutofitRefreshResult",
    "AutofitUpdate",
    "Inches",
    "Length",
    "MSO_ANCHOR",
    "MSO_AUTO_SIZE",
    "MSO_SHAPE",
    "PP_PLACEHOLDER",
    "PP_PLACEHOLDER_TYPE",
    "HeaderFooter",
    "SlideTransition",
    "PlaceholderFormat",
    "SlideMaster",
    "SlideMasterCollection",
    "Theme", "ThemeColors", "MasterTextStyles", "TextStyleList",
    "ThemeFonts",
    "ThemeFontSet",
    "MSO_UNDERLINE",
    "PP_ALIGN",
    "PackageError",
    "Comment",
    "CommentAuthor",
    "CommentReply",
    "Presentation",
    "PresentationTextExport",
    "Pt",
    "RGBColor",
    "ReplacementCountError",
    "RpptxError",
    "StaleElementError",
    "TextFrameLayout",
    "TextLineLayout",
    "ValidationIssue",
    "XmlError",
]


"""Chart data for `add_chart`, mirroring python-pptx `pptx.chart.data`."""

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


from ._rpptx import Chart, ChartSeries, Shape

GraphicFrame = Shape

__all__ += ["Chart", "ChartSeries", "GraphicFrame"]
