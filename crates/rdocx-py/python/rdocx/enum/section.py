"""Word section enum names and values from python-docx 1.2.0.

This module supplies enum constants only. It does not change the existing
rdocx section ownership, string-valued properties or insertion semantics.
"""

from enum import IntEnum


class WD_ORIENTATION(IntEnum):
    """Page orientation constants, with the python-docx numeric values."""

    PORTRAIT = 0
    LANDSCAPE = 1


WD_ORIENT = WD_ORIENTATION


class WD_SECTION_START(IntEnum):
    """Section start constants, with the python-docx numeric values."""

    CONTINUOUS = 0
    NEW_COLUMN = 1
    NEW_PAGE = 2
    EVEN_PAGE = 3
    ODD_PAGE = 4


WD_SECTION = WD_SECTION_START


__all__ = ["WD_ORIENTATION", "WD_ORIENT", "WD_SECTION_START", "WD_SECTION"]
