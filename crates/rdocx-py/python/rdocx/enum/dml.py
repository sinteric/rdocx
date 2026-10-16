"""Word color kinds with pinned python-docx numeric values."""

from enum import IntEnum


class MSO_COLOR_TYPE(IntEnum):
    """Supported direct Word color kinds."""

    RGB = 1
    THEME = 2
    AUTO = 101


__all__ = ["MSO_COLOR_TYPE"]
