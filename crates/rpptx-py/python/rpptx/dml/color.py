"""Colour values used by the public rpptx API."""


class RGBColor(tuple[int, int, int]):
    """An immutable red, green, blue colour triple."""

    def __new__(cls, r: int, g: int, b: int) -> "RGBColor":
        channels = (r, g, b)
        if any(not isinstance(channel, int) or not 0 <= channel <= 255 for channel in channels):
            raise ValueError("RGBColor() takes three integer values 0-255")
        return super().__new__(cls, channels)

    @classmethod
    def from_string(cls, rgb_hex_str: str) -> "RGBColor":
        """Create a colour from six ASCII hex digits with an optional leading hash."""

        if not isinstance(rgb_hex_str, str):
            raise TypeError(f"RGBColor.from_string accepts a six-digit ASCII hex string with optional #, got {type(rgb_hex_str).__name__}")
        source = rgb_hex_str.removeprefix("#")
        if len(source) != 6 or any(character not in "0123456789abcdefABCDEF" for character in source):
            raise ValueError("RGBColor.from_string requires exactly six ASCII hexadecimal digits with an optional single #")
        return cls(*(int(source[offset : offset + 2], 16) for offset in (0, 2, 4)))

    def __str__(self) -> str:
        return "%02X%02X%02X" % self


__all__ = ["RGBColor"]
