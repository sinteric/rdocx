"""Public enumeration namespaces for rdocx."""

from enum import IntEnum


class MSO_THEME_COLOR(IntEnum):
    """Supported Office theme colours with their pinned standard values."""

    NOT_THEME_COLOR = 0
    DARK_1 = 1
    LIGHT_1 = 2
    DARK_2 = 3
    LIGHT_2 = 4
    ACCENT_1 = 5
    ACCENT_2 = 6
    ACCENT_3 = 7
    ACCENT_4 = 8
    ACCENT_5 = 9
    ACCENT_6 = 10
    HYPERLINK = 11
    FOLLOWED_HYPERLINK = 12
    TEXT_1 = 13
    BACKGROUND_1 = 14
    TEXT_2 = 15
    BACKGROUND_2 = 16
    MIXED = -2


__all__ = ["MSO_THEME_COLOR"]
