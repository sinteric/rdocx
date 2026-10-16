#!/usr/bin/env python3
"""Source-only OfficeMath fixture and pinned Word PDF oracle verifier."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import plistlib
import re
import shutil
import subprocess
import tempfile
import zipfile
from pathlib import Path
from xml.etree import ElementTree


ROOT = Path(__file__).resolve().parents[1]
MANIFEST_PATH = ROOT / "scripts" / "officemath_oracle_manifest.json"
WORD_IDENTITY = "Microsoft Word 16.104 build 16.104.25121423"
POPPLER_IDENTITY = "pdftoppm version 26.01.0"
SSIM_BLOCK_SIZE = 64


CONTENT_TYPES = b"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/><Override PartName="/word/settings.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"/><Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/></Types>"""

ROOT_RELS = b"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/></Relationships>"""

DOCUMENT_RELS = b"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rIdSettings" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings" Target="settings.xml"/><Relationship Id="rIdStyles" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/></Relationships>"""

SETTINGS = b"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:settings xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><m:mathPr><m:mathFont m:val="Caladea"/></m:mathPr></w:settings>"""

STYLES = b"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:docDefaults><w:rPrDefault><w:rPr><w:rFonts w:ascii="Caladea" w:hAnsi="Caladea"/><w:sz w:val="22"/></w:rPr></w:rPrDefault></w:docDefaults><w:style w:type="paragraph" w:default="1" w:styleId="Normal"><w:name w:val="Normal"/></w:style></w:styles>"""

DOCUMENT = b"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"><w:body><w:p><w:r><w:t>OfficeMath oracle</w:t></w:r></w:p><w:p><m:oMathPara><m:oMathParaPr><m:jc m:val="center"/></m:oMathParaPr><m:oMath><m:r><m:t>x</m:t></m:r><m:f><m:num><m:r><m:t>1</m:t></m:r></m:num><m:den><m:r><m:t>2</m:t></m:r></m:den></m:f><m:sSub><m:e><m:r><m:t>x</m:t></m:r></m:e><m:sub><m:r><m:t>i</m:t></m:r></m:sub></m:sSub><m:sSup><m:e><m:r><m:t>x</m:t></m:r></m:e><m:sup><m:r><m:t>2</m:t></m:r></m:sup></m:sSup><m:sSubSup><m:e><m:r><m:t>x</m:t></m:r></m:e><m:sub><m:r><m:t>i</m:t></m:r></m:sub><m:sup><m:r><m:t>2</m:t></m:r></m:sup></m:sSubSup><m:sPre><m:sub><m:r><m:t>a</m:t></m:r></m:sub><m:sup><m:r><m:t>b</m:t></m:r></m:sup><m:e><m:r><m:t>X</m:t></m:r></m:e></m:sPre><m:rad><m:radPr><m:degHide m:val="0"/></m:radPr><m:deg><m:r><m:t>3</m:t></m:r></m:deg><m:e><m:r><m:t>x</m:t></m:r></m:e></m:rad><m:m><m:mr><m:e><m:r><m:t>a</m:t></m:r></m:e><m:e><m:r><m:t>b</m:t></m:r></m:e></m:mr><m:mr><m:e><m:r><m:t>c</m:t></m:r></m:e><m:e><m:r><m:t>d</m:t></m:r></m:e></m:mr></m:m><m:limLow><m:e><m:r><m:t>lim</m:t></m:r></m:e><m:lim><m:r><m:t>0</m:t></m:r></m:lim></m:limLow><m:limUpp><m:e><m:r><m:t>max</m:t></m:r></m:e><m:lim><m:r><m:t>n</m:t></m:r></m:lim></m:limUpp><m:nary><m:naryPr><m:chr m:val="&#x2211;"/><m:limLoc m:val="undOvr"/><m:subHide m:val="0"/><m:supHide m:val="0"/></m:naryPr><m:sub><m:r><m:t>i=0</m:t></m:r></m:sub><m:sup><m:r><m:t>n</m:t></m:r></m:sup><m:e><m:r><m:t>x</m:t></m:r></m:e></m:nary><m:d><m:dPr><m:begChr m:val="("/><m:sepChr m:val="|"/><m:endChr m:val=")"/></m:dPr><m:e><m:r><m:t>x</m:t></m:r></m:e><m:e><m:r><m:t>y</m:t></m:r></m:e></m:d><m:acc><m:accPr><m:chr m:val="&#x302;"/></m:accPr><m:e><m:r><m:t>x</m:t></m:r></m:e></m:acc></m:oMath></m:oMathPara></w:p><w:sectPr><w:pgSz w:w="12240" w:h="15840"/><w:pgMar w:top="1440" w:right="1440" w:bottom="1440" w:left="1440"/></w:sectPr></w:body></w:document>"""



def fx235_documents() -> dict[str, bytes]:
    """Isolate each geometry family beside the exact F-X234 supported tree."""
    def run_text(text: str, normal: bool = False) -> str:
        from xml.sax.saxutils import escape
        properties = '<m:rPr><m:nor m:val="1"/></m:rPr>' if normal else ''
        return f'<m:r>{properties}<m:t>{escape(text)}</m:t></m:r>'

    def superscript(base: str, script: str) -> str:
        return f'<m:sSup><m:e>{base}</m:e><m:sup>{script}</m:sup></m:sSup>'

    def fraction(numerator: str, denominator: str) -> str:
        return f'<m:f><m:num>{numerator}</m:num><m:den>{denominator}</m:den></m:f>'

    def radical(base: str) -> str:
        return f'<m:rad><m:radPr><m:degHide m:val="1"/></m:radPr><m:deg/>' + f'<m:e>{base}</m:e></m:rad>'

    scripts = superscript(run_text('T'), run_text('2')) + run_text('+') + superscript(run_text('C'), run_text('2'))
    rad = radical(scripts)
    integral = '<m:nary><m:naryPr><m:chr m:val="∫"/><m:limLoc m:val="subSup"/><m:subHide m:val="0"/><m:supHide m:val="0"/></m:naryPr>' + '<m:sub>' + run_text('a') + '</m:sub><m:sup>' + run_text('b') + '</m:sup><m:e>' + superscript(run_text('e'), run_text('iπK')) + '</m:e></m:nary>'
    limit = '<m:sSub><m:e>' + run_text('lim', True) + '</m:e><m:sub>' + run_text('x→∞') + '</m:sub></m:sSub>'
    expressions = {
        'exact-formula': integral + run_text('\u2009dx=') + limit + fraction(run_text('S'), rad),
        'fraction': run_text('x=') + fraction(run_text('S'), run_text('T')) + run_text('+y'),
        'nested-fraction': run_text('x=') + fraction(run_text('S'), fraction(run_text('T'), run_text('C'))) + run_text('+y'),
        'integral': integral + run_text('\u2009dx'),
        'radical': run_text('x=') + rad + run_text('+y'),
        'scripts-limit': superscript(run_text('e'), run_text('iπK')) + run_text('=') + limit,
    }
    start = DOCUMENT.index(b'<m:oMath>') + len(b'<m:oMath>')
    end = DOCUMENT.index(b'</m:oMath>')
    return {name: DOCUMENT[:start] + expression.encode() + DOCUMENT[end:] for name, expression in expressions.items()}


def load_manifest() -> dict:
    return json.loads(MANIFEST_PATH.read_text(encoding="utf-8"))


def source_docx_bytes(
    document: bytes = DOCUMENT, *, text_font: str = "Caladea", math_font: str = "Caladea", explicit_spacing: bool = False
) -> bytes:
    settings = SETTINGS.replace(b"Caladea", math_font.encode())
    styles = STYLES.replace(b"Caladea", text_font.encode())
    if explicit_spacing:
        settings = settings.replace(b"</m:mathPr>", b'<m:preSp m:val="0"/><m:postSp m:val="0"/></m:mathPr>')
        styles = styles.replace(b"</w:docDefaults>", b'<w:pPrDefault><w:pPr><w:spacing w:before="0" w:after="0" w:line="240" w:lineRule="auto"/></w:pPr></w:pPrDefault></w:docDefaults>')
    with tempfile.SpooledTemporaryFile() as stream:
        with zipfile.ZipFile(stream, "w", zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
            for name, data in (
                ("[Content_Types].xml", CONTENT_TYPES),
                ("_rels/.rels", ROOT_RELS),
                ("word/_rels/document.xml.rels", DOCUMENT_RELS),
                ("word/document.xml", document),
                ("word/settings.xml", settings),
                ("word/styles.xml", styles),
            ):
                info = zipfile.ZipInfo(name, (1980, 1, 1, 0, 0, 0))
                info.compress_type = zipfile.ZIP_DEFLATED
                info.external_attr = 0o600 << 16
                archive.writestr(info, data, compress_type=zipfile.ZIP_DEFLATED, compresslevel=9)
        stream.seek(0)
        return stream.read()


def source_sha256() -> str:
    return hashlib.sha256(source_docx_bytes()).hexdigest()


def run(command: list[str]) -> str:
    completed = subprocess.run(command, check=True, capture_output=True, text=True)
    return completed.stdout + completed.stderr


def verify_poppler() -> None:
    identity = run(["pdftoppm", "-v"]).strip().splitlines()[0]
    if identity != POPPLER_IDENTITY:
        raise SystemExit(f"expected {POPPLER_IDENTITY!r}, got {identity!r}")


def pdf_page_size(pdf: Path) -> tuple[float, float]:
    info = run(["pdfinfo", str(pdf)])
    match = re.search(r"^Page size:\s+([0-9.]+) x ([0-9.]+) pts", info, re.MULTILINE)
    if match is None:
        raise SystemExit("pdfinfo did not report a point page size")
    return float(match.group(1)), float(match.group(2))


def pdf_tokens(pdf: Path) -> list[str]:
    text = run(["pdftotext", "-layout", str(pdf), "-"])
    return text.split()


def pdf_math_word_boxes(pdf: Path, *, fixture_header: bool = False) -> list[tuple[float, float, float, float]]:
    document = ElementTree.fromstring(run(["pdftotext", "-bbox-layout", str(pdf), "-"]))
    all_words = document.findall(".//{http://www.w3.org/1999/xhtml}word")
    if fixture_header:
        header = [word for word in all_words if word.text in ("OfficeMath", "oracle")]
        if len(header) != 2:
            raise SystemExit("F-X235 fixture header inventory differs")
        words = [word for word in all_words if word not in header]
    else:
        words = [word for word in all_words if float(word.attrib["yMin"]) > 90.0]
    if not words:
        raise SystemExit("Word PDF contains no equation word boxes")
    return [
        tuple(float(word.attrib[name]) for name in ("xMin", "yMin", "xMax", "yMax"))
        for word in words
    ]


def union_boxes(
    boxes: list[tuple[float, float, float, float]], indices: list[int]
) -> tuple[float, float, float, float]:
    selected = [boxes[index] for index in indices]
    return (
        min(box[0] for box in selected),
        min(box[1] for box in selected),
        max(box[2] for box in selected),
        max(box[3] for box in selected),
    )


def comparable_geometry(box: tuple[float, float, float, float]) -> tuple[float, float, float]:
    return box[2] - box[0], box[1], box[3]


def geometry_matches(
    word: tuple[float, float, float],
    rust: tuple[float, float, float],
    tolerance: float,
) -> bool:
    return all(abs(word_value - rust_value) <= tolerance for word_value, rust_value in zip(word, rust))


def pdf_math_bbox(pdf: Path, *, fixture_header: bool = False) -> tuple[float, float, float, float]:
    boxes = pdf_math_word_boxes(pdf, fixture_header=fixture_header)
    return (
        min(box[0] for box in boxes),
        min(box[1] for box in boxes),
        max(box[2] for box in boxes),
        max(box[3] for box in boxes),
    )


def read_ppm(path: Path) -> tuple[int, int, bytes]:
    data = path.read_bytes()
    match = re.match(rb"P6\s+(\d+)\s+(\d+)\s+255\s", data)
    if match is None:
        raise SystemExit(f"unsupported PPM at {path}")
    width, height = int(match.group(1)), int(match.group(2))
    pixels = data[match.end() :]
    if len(pixels) != width * height * 3:
        raise SystemExit(f"truncated PPM at {path}")
    return width, height, pixels


def raster_box(
    pixels: bytes,
    width: int,
    height: int,
    dpi: int,
    window: list[float],
) -> tuple[float, float, float, float]:
    x_min, x_max, y_min, y_max = window
    columns = range(max(0, int(x_min * dpi / 72.0)), min(width, math.ceil(x_max * dpi / 72.0)))
    rows = range(max(0, int(y_min * dpi / 72.0)), min(height, math.ceil(y_max * dpi / 72.0)))
    ink = []
    for row in rows:
        for column in columns:
            offset = (row * width + column) * 3
            if min(pixels[offset : offset + 3]) < 220:
                ink.append((column, row))
    if not ink:
        raise SystemExit(f"no raster ink in expression window {window}")
    return (
        min(column for column, _ in ink) * 72.0 / dpi,
        min(row for _, row in ink) * 72.0 / dpi,
        (max(column for column, _ in ink) + 1) * 72.0 / dpi,
        (max(row for _, row in ink) + 1) * 72.0 / dpi,
    )


def block_luminance(pixels: bytes, width: int, height: int) -> list[float]:
    values = []
    for y in range(0, height, SSIM_BLOCK_SIZE):
        for x in range(0, width, SSIM_BLOCK_SIZE):
            total = 0.0
            count = 0
            for row in range(y, min(y + SSIM_BLOCK_SIZE, height)):
                for column in range(x, min(x + SSIM_BLOCK_SIZE, width)):
                    offset = (row * width + column) * 3
                    total += (
                        0.2126 * pixels[offset]
                        + 0.7152 * pixels[offset + 1]
                        + 0.0722 * pixels[offset + 2]
                    )
                    count += 1
            values.append(total / count)
    return values


def luminance_ssim(left: bytes, right: bytes, width: int, height: int) -> float:
    if len(left) != len(right) or not left:
        return 0.0
    left_y = block_luminance(left, width, height)
    right_y = block_luminance(right, width, height)
    left_mean = sum(left_y) / len(left_y)
    right_mean = sum(right_y) / len(right_y)
    left_var = sum((value - left_mean) ** 2 for value in left_y) / len(left_y)
    right_var = sum((value - right_mean) ** 2 for value in right_y) / len(right_y)
    covariance = sum((a - left_mean) * (b - right_mean) for a, b in zip(left_y, right_y)) / len(left_y)
    c1 = (0.01 * 255.0) ** 2
    c2 = (0.03 * 255.0) ** 2
    return ((2 * left_mean * right_mean + c1) * (2 * covariance + c2)) / ((left_mean**2 + right_mean**2 + c1) * (left_var + right_var + c2))


def verify_word_pdf(pdf: Path, word_identity: str, rust_pdf: Path | None) -> None:
    manifest = load_manifest()
    if word_identity != WORD_IDENTITY:
        raise SystemExit(f"expected {WORD_IDENTITY!r}, got {word_identity!r}")
    verify_poppler()
    page_width, page_height = pdf_page_size(pdf)
    expected_page = manifest["page_size_pt"]
    tolerance = manifest["geometry_tolerance_pt"]
    if abs(page_width - expected_page[0]) > tolerance or abs(page_height - expected_page[1]) > tolerance:
        raise SystemExit(f"page size {(page_width, page_height)} is outside the {tolerance} point tolerance")
    if manifest["tokens"] != pdf_tokens(pdf):
        raise SystemExit("Word PDF text tokens differ from the exact manifest")
    actual_word_boxes = pdf_math_word_boxes(pdf)
    expected_word_boxes = manifest["word_pdf_glyph_boxes_pt"]
    if len(actual_word_boxes) != len(expected_word_boxes) or any(
        abs(actual - expected) > tolerance
        for actual_box, expected_box in zip(actual_word_boxes, expected_word_boxes)
        for actual, expected in zip(actual_box, expected_box)
    ):
        raise SystemExit("Word PDF glyph boxes differ from the exact geometry manifest")
    actual_bbox = pdf_math_bbox(pdf)
    expected_bbox = manifest["word_pdf_math_bbox_pt"]
    if any(abs(actual - expected) > tolerance for actual, expected in zip(actual_bbox, expected_bbox)):
        raise SystemExit(
            f"Word PDF equation bounds {actual_bbox} are outside the {tolerance} point tolerance"
        )
    if rust_pdf is not None:
        rust_boxes = pdf_math_word_boxes(rust_pdf)
        expression_names = manifest["expression_names"]
        word_indices = manifest["word_expression_glyph_indices"]
        rust_indices = manifest["rust_expression_glyph_indices"]
        if len(expression_names) != len(word_indices) + 2 or len(word_indices) != len(rust_indices):
            raise SystemExit("expression extraction manifest drift")
        for name, word_group, rust_group in zip(expression_names, word_indices, rust_indices):
            word_geometry = comparable_geometry(union_boxes(actual_word_boxes, word_group))
            rust_geometry = comparable_geometry(union_boxes(rust_boxes, rust_group))
            if not geometry_matches(word_geometry, rust_geometry, tolerance):
                raise SystemExit(
                    f"{name} geometry {rust_geometry} differs from Word geometry "
                    f"{word_geometry} by more than {tolerance} point"
                )
        rust_bbox = pdf_math_bbox(rust_pdf)
        if any(abs(rust - word) > tolerance for rust, word in zip(rust_bbox, actual_bbox)):
            raise SystemExit(
                f"Rust PDF equation bounds {rust_bbox} differ from Word bounds {actual_bbox} "
                f"by more than {tolerance} point"
            )
        with tempfile.TemporaryDirectory() as directory:
            directory_path = Path(directory)
            word_prefix = directory_path / "word"
            rust_prefix = directory_path / "rust"
            run(["pdftoppm", "-f", "1", "-singlefile", "-r", str(manifest["dpi"]), str(pdf), str(word_prefix)])
            run(["pdftoppm", "-f", "1", "-singlefile", "-r", str(manifest["dpi"]), str(rust_pdf), str(rust_prefix)])
            word_width, word_height, word_pixels = read_ppm(word_prefix.with_suffix(".ppm"))
            rust_width, rust_height, rust_pixels = read_ppm(rust_prefix.with_suffix(".ppm"))
            if (word_width, word_height) != (rust_width, rust_height):
                raise SystemExit("Rust and Word raster dimensions differ")
            for name, window in zip(expression_names[-2:], manifest["raster_expression_windows_pt"]):
                word_geometry = comparable_geometry(
                    raster_box(word_pixels, word_width, word_height, manifest["dpi"], window)
                )
                rust_geometry = comparable_geometry(
                    raster_box(rust_pixels, rust_width, rust_height, manifest["dpi"], window)
                )
                if not geometry_matches(word_geometry, rust_geometry, tolerance):
                    raise SystemExit(
                        f"{name} raster geometry {rust_geometry} differs from Word geometry "
                        f"{word_geometry} by more than {tolerance} point"
                    )
            score = luminance_ssim(word_pixels, rust_pixels, word_width, word_height)
            if score < manifest["luminance_ssim_floor"]:
                raise SystemExit(f"luminance SSIM {score:.9f} is below the manifest floor")


def verify_fx235(directory: Path, word_identity: str, product_suffix: str, fixture_key: str) -> None:
    """Compare independently exported native files with caller-font product files."""
    manifest = load_manifest()
    fixture = manifest[fixture_key]
    info_path = Path(fixture["word_application"]["info_plist"])
    application = plistlib.loads(info_path.read_bytes())
    if any(application.get(key) != value for key, value in fixture["word_application"]["identity"].items()):
        raise SystemExit("F-X235 installed Word application identity mismatch")
    if word_identity != fixture["word_identity"]:
        raise SystemExit("F-X235 Word identity mismatch")
    identity = run(["pdftoppm", "-v"]).strip().splitlines()[0]
    if identity != fixture["poppler_identity"]:
        raise SystemExit("F-X235 Poppler identity mismatch")
    for path, expected in {**fixture["font_files"], **fixture["tools"]}.items():
        if hashlib.sha256(Path(path).read_bytes()).hexdigest() != expected["sha256"]:
            raise SystemExit(f"F-X235 authentic font/tool digest mismatch: {path}")
    for path in fixture["tools"]:
        actual = shutil.which(Path(path).name)
        if actual is None or Path(actual).resolve() != Path(path).resolve():
            raise SystemExit(f"F-X235 tool resolution mismatch: {path}")
    reopened = fixture.get("native_save_reopen")
    if reopened and hashlib.sha256(Path(reopened["path"]).read_bytes()).hexdigest() != reopened["sha256"]:
        raise SystemExit("F-X235 native save/reopen digest mismatch")
    if fixture["status"] != "native exports captured":
        raise SystemExit("F-X235 independent native receipts are pending")
    results = {}
    errors = []
    for name, document in fx235_documents().items():
        source = source_docx_bytes(document, text_font="Cambria", math_font="Cambria Math", explicit_spacing=fixture["explicit_spacing"])
        if hashlib.sha256(source).hexdigest() != fixture["source_sha256"][name]:
            raise SystemExit(f"F-X235 source recipe drift: {name}")
        prefix = directory / f'{name}-{fixture["source_suffix"]}'
        if prefix.with_suffix(".docx").read_bytes() != source:
            raise SystemExit(f"F-X235 actual source drift: {name}")
        word_pdf = Path(f"{prefix}-word.pdf")
        product_pdf = Path(f"{prefix}-{product_suffix}.pdf")
        if hashlib.sha256(word_pdf.read_bytes()).hexdigest() != fixture["native_pdf_sha256"][name]:
            raise SystemExit(f"F-X235 native PDF digest mismatch: {name}")
        word_box = pdf_math_bbox(word_pdf, fixture_header=True)
        product_box = pdf_math_bbox(product_pdf, fixture_header=True)
        pdf_geometry_pass = all(abs(a - b) <= manifest["geometry_tolerance_pt"] for a, b in zip(word_box, product_box)) and geometry_matches(comparable_geometry(word_box), comparable_geometry(product_box), manifest["geometry_tolerance_pt"])
        expressions = {}
        if name == "exact-formula" and "exact_expression_glyph_indices" in fixture:
            native_boxes = pdf_math_word_boxes(word_pdf, fixture_header=True)
            actual_boxes = pdf_math_word_boxes(product_pdf, fixture_header=True)
            if len(native_boxes) != fixture["exact_word_box_count"] or len(actual_boxes) != len(native_boxes):
                raise SystemExit("F-X235 exact expression extraction inventory differs")
            for label, indices in fixture["exact_expression_glyph_indices"].items():
                native = union_boxes(native_boxes, indices)
                actual = union_boxes(actual_boxes, indices)
                matches = all(abs(a - b) <= manifest["geometry_tolerance_pt"] for a, b in zip(native, actual)) and geometry_matches(comparable_geometry(native), comparable_geometry(actual), manifest["geometry_tolerance_pt"])
                expressions[label] = {"native_bbox_pt": native, "product_bbox_pt": actual, "passed": matches}
                pdf_geometry_pass = pdf_geometry_pass and matches
        with tempfile.TemporaryDirectory(prefix="fx235-raster-") as temporary:
            rasters = []
            for label, pdf in [("word", word_pdf), ("product", product_pdf)]:
                raster = Path(temporary) / label
                run(["pdftoppm", "-f", "1", "-singlefile", "-r", str(manifest["dpi"]), str(pdf), str(raster)])
                rasters.append(read_ppm(raster.with_suffix(".ppm")))
            ww, wh, wp = rasters[0]
            pw, ph, pp = rasters[1]
            if (ww, wh) != (pw, ph):
                raise SystemExit(f"F-X235 raster dimensions differ: {name}")
            word_ink = raster_box(wp, ww, wh, manifest["dpi"], fixture["raster_math_window_pt"])
            product_ink = raster_box(pp, pw, ph, manifest["dpi"], fixture["raster_math_window_pt"])
            raster_geometry_pass = all(abs(a - b) <= manifest["geometry_tolerance_pt"] for a, b in zip(word_ink, product_ink)) and geometry_matches(comparable_geometry(word_ink), comparable_geometry(product_ink), manifest["geometry_tolerance_pt"])
            score = luminance_ssim(wp, pp, ww, wh)
        passed = pdf_geometry_pass and raster_geometry_pass and score >= manifest["luminance_ssim_floor"]
        results[name] = {
            "word_pdf_sha256": hashlib.sha256(word_pdf.read_bytes()).hexdigest(),
            "product_pdf_sha256": hashlib.sha256(product_pdf.read_bytes()).hexdigest(),
            "word_pdf_bbox_pt": word_box, "product_pdf_bbox_pt": product_box,
            "word_ink_bbox_pt": word_ink, "product_ink_bbox_pt": product_ink,
            "pdf_geometry_pass": pdf_geometry_pass, "raster_geometry_pass": raster_geometry_pass,
            "exact_expression_geometry": expressions,
            "luminance_ssim": score, "passed": passed,
        }
        if not passed:
            errors.append(name)
    print(json.dumps({"fixture": fixture["fixture"], "product_suffix": product_suffix, "results": results}, indent=2))
    if errors:
        raise SystemExit("F-X235 native comparison failed: " + ", ".join(errors))


def self_check() -> None:
    manifest = load_manifest()
    if manifest["word_identity"] != WORD_IDENTITY:
        raise SystemExit("manifest Word identity drift")
    if manifest["poppler_identity"] != POPPLER_IDENTITY:
        raise SystemExit("manifest Poppler identity drift")
    if manifest["source_sha256"] != source_sha256():
        raise SystemExit("manifest source DOCX digest drift")
    tolerance = manifest["geometry_tolerance_pt"]
    perturbation = manifest["negative_perturbation_pt"]
    if not math.isclose(tolerance, 1.0) or not math.isclose(perturbation, 1.01):
        raise SystemExit("manifest geometry calibration drift")
    if perturbation <= tolerance:
        raise SystemExit("negative perturbation does not exceed the tolerance")
    if manifest["dpi"] != 150:
        raise SystemExit("manifest DPI drift")
    if manifest["luminance_block_size"] != SSIM_BLOCK_SIZE:
        raise SystemExit("manifest SSIM block-size drift")
    if len(manifest["word_pdf_math_bbox_pt"]) != 4:
        raise SystemExit("manifest Word PDF equation bounds drift")
    if len(manifest["word_pdf_glyph_boxes_pt"]) != 26:
        raise SystemExit("manifest Word PDF glyph-box inventory drift")
    if len(manifest["expression_names"]) != 13:
        raise SystemExit("manifest expression inventory drift")
    if len(manifest["word_expression_glyph_indices"]) != 11:
        raise SystemExit("manifest Word expression extraction drift")
    if len(manifest["rust_expression_glyph_indices"]) != 11:
        raise SystemExit("manifest Rust expression extraction drift")
    if len(manifest["raster_expression_windows_pt"]) != 2:
        raise SystemExit("manifest raster expression extraction drift")
    for key in ("fx235", "fx235_v2"):
        fixture = manifest[key]
        expected_sources = {
            name: hashlib.sha256(source_docx_bytes(document, text_font="Cambria", math_font="Cambria Math", explicit_spacing=fixture["explicit_spacing"])).hexdigest()
            for name, document in fx235_documents().items()
        }
        if fixture["source_sha256"] != expected_sources:
            raise SystemExit(f"{key} exact source recipe drift")
        if fixture["word_identity"] != "Microsoft Word 16.113.2 build 16.113.26092012" or fixture["poppler_identity"] != "pdftoppm version 26.09.0":
            raise SystemExit(f"{key} new producer identity drift")
    if manifest["luminance_ssim_floor"] != 0.99:
        raise SystemExit("manifest SSIM threshold drift")
    print(f"officemath oracle self-check: source {source_sha256()}, thresholds valid")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-check", action="store_true")
    parser.add_argument("--print-source-sha", action="store_true")
    parser.add_argument("--build-source", type=Path)
    parser.add_argument("--build-fx235-sources", type=Path)
    parser.add_argument("--build-fx235-v2-sources", type=Path)
    parser.add_argument("--fx235-fixture", choices=["fx235", "fx235_v2"], default="fx235_v2")
    parser.add_argument("--word-pdf", type=Path)
    parser.add_argument("--verify-fx235", type=Path)
    parser.add_argument("--fx235-product-suffix", default="product")
    parser.add_argument("--word-identity")
    parser.add_argument("--rust-pdf", type=Path)
    args = parser.parse_args()
    if args.print_source_sha:
        print(source_sha256())
    if args.build_source is not None:
        args.build_source.write_bytes(source_docx_bytes())
    for directory, explicit in [(args.build_fx235_sources, False), (args.build_fx235_v2_sources, True)]:
        if directory is not None:
            directory.mkdir(parents=True, exist_ok=True)
            for name, document in fx235_documents().items():
                data = source_docx_bytes(document, text_font="Cambria", math_font="Cambria Math", explicit_spacing=explicit)
                suffix = "cambria-explicit" if explicit else "cambria"
                path = directory / f"{name}-{suffix}.docx"
                path.write_bytes(data)
                print(f"{path}: {hashlib.sha256(data).hexdigest()}")
    if args.self_check:
        self_check()
    if args.verify_fx235 is not None:
        if args.word_identity is None:
            parser.error("--verify-fx235 requires --word-identity")
        verify_fx235(args.verify_fx235, args.word_identity, args.fx235_product_suffix, args.fx235_fixture)
    if args.word_pdf is not None:
        if args.word_identity is None:
            parser.error("--word-pdf requires --word-identity")
        verify_word_pdf(args.word_pdf, args.word_identity, args.rust_pdf)
    if not any((args.self_check, args.print_source_sha, args.build_source, args.build_fx235_sources, args.build_fx235_v2_sources, args.word_pdf, args.verify_fx235)):
        parser.error("choose --self-check, --print-source-sha, --build-source, or --word-pdf")


if __name__ == "__main__":
    main()
