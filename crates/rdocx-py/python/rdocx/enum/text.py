"""Text-related public enumerations."""

from enum import IntEnum


class WD_ALIGN_PARAGRAPH(IntEnum):
    """Paragraph horizontal alignment."""

    LEFT = 0
    CENTER = 1
    RIGHT = 2
    JUSTIFY = 3


class WD_UNDERLINE(IntEnum):
    """Underline styles supported by the rdocx run facade."""

    NONE = 0
    SINGLE = 1
    WORDS = 2
    DOUBLE = 3
    DOTTED = 4
    THICK = 6
    DASH = 7
    DOT_DASH = 9
    DOT_DOT_DASH = 10
    WAVY = 11


class WD_TAB_ALIGNMENT(IntEnum):
    LEFT = 0
    CENTER = 1
    RIGHT = 2
    DECIMAL = 3
    BAR = 4
    LIST = 6
    CLEAR = 7
    END = 102
    NUM = 103
    START = 104


class WD_TAB_LEADER(IntEnum):
    SPACES = 0
    DOTS = 1
    DASHES = 2
    LINES = 3
    HEAVY = 4
    MIDDLE_DOT = 5


__all__ = ["WD_ALIGN_PARAGRAPH", "WD_UNDERLINE", "WD_TAB_ALIGNMENT", "WD_TAB_LEADER"]


class WD_BREAK_TYPE(IntEnum):
    LINE = 6
    PAGE = 7
    COLUMN = 8


WD_BREAK = WD_BREAK_TYPE


class WD_COLOR_INDEX(IntEnum):
    AUTO = 0
    BLACK = 1
    BLUE = 2
    TURQUOISE = 3
    BRIGHT_GREEN = 4
    PINK = 5
    RED = 6
    YELLOW = 7
    WHITE = 8
    DARK_BLUE = 9
    TEAL = 10
    GREEN = 11
    VIOLET = 12
    DARK_RED = 13
    DARK_YELLOW = 14
    GRAY_50 = 15
    GRAY_25 = 16


__all__ += ["WD_BREAK_TYPE", "WD_BREAK", "WD_COLOR_INDEX"]
