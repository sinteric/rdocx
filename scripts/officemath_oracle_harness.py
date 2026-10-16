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


def verify_fx235(directory: Path, word_identity: str, product_suffix: str, fixture_key: str, *, historical_producer: bool = False) -> None:
    """Compare independently exported native files with caller-font product files."""
    manifest = load_manifest()
    fixture = manifest[fixture_key]
    info_path = Path(fixture["word_application"]["info_plist"])
    application = plistlib.loads(info_path.read_bytes())
    application_matches = all(application.get(key) == value for key, value in fixture["word_application"]["identity"].items())
    if not application_matches and not historical_producer:
        raise SystemExit("F-X235 installed Word application identity mismatch")
    historical_receipt_sha256 = None
    if historical_producer:
        receipt = Path(fixture["native_save_reopen"]["path"]).parent / "native-word-export-receipts.json"
        historical_receipt_sha256 = hashlib.sha256(receipt.read_bytes()).hexdigest()
        if historical_receipt_sha256 != fixture["native_receipt_sha256"]:
            raise SystemExit("F-X235 historical native producer receipt drift")
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
    print(json.dumps({"fixture": fixture["fixture"], "product_suffix": product_suffix,
        "producer_binding": {"historical_export_verification": historical_producer,
            "strict_installed_application_gate_passed": application_matches,
            "historical_native_receipt_sha256": historical_receipt_sha256,
            "export_producer": fixture["word_identity"],
            "installed_application": {key: application.get(key) for key in fixture["word_application"]["identity"]},
            "current_application_matches_export_producer": application_matches},
        "results": results}, indent=2))
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



def fx247_expected_equations() -> list[list]:
    """Independent source-authored Issue365 expectations, never a converter output."""
    def r(text, normal=None, style=None):
        return ["run", text, normal, style]

    def a(text):
        return [r(text)]

    def sup(base, script):
        return ["sup", a(base), a(script)]

    def frac(numerator, denominator, kind="bar"):
        return ["fraction", kind, numerator, denominator]

    expected = [[r("α+β")], [r("π"), sup("r", "2")]]
    expected += [[r(text)] for text in ["θ", "λ", "μ", "σ", "ω", "ε", "ϕ", "φ"]]
    expected += [[r("Δ", None, "p"), r("x")]]
    expected += [[r(text, None, "p")] for text in ["Σ", "Ω", "Γ"]]
    # Each relation/operator owns its original left and right operand.
    for left, symbol, right in [
        ("a", "≤", "b"), ("a", "≤", "b"), ("a", "≥", "b"), ("a", "≥", "b"),
        ("a", "≠", "b"), ("a", "≠", "b"), ("a", "≈", "b"), ("a", "≡", "b"),
        ("a", "∼", "b"), ("a", "∝", "b"), ("x", "∈", "A"), ("A", "⊂", "B"),
        ("a", "×", "b"), ("a", "⋅", "b"), ("a", "÷", "b"), ("a", "±", "b"),
        ("a", "∓", "b"), ("A", "∪", "B"), ("A", "∩", "B"),
    ]:
        expected.append([r(left), r(symbol, None, "p"), r(right)])
    expected += [a("a→b"), a("a→b"), [r("a"), r("⇒", None, "p"), r("b")],
                 [r("a"), r("←", None, "p"), r("b")], a("∞"), a("∂f"),
                 [r("∇", None, "p"), r("f")], [r("∀", None, "p"), r("x")],
                 [r("∃", None, "p"), r("x")], [r("°", None, "p")],
                 [r("…", None, "p")], [r("⋯", None, "p")]]
    expected += [[["sub", [r("lim", True)], a("x→0")], r("x")]]
    expected += [[r(function, True), r("x")] for function in ["sin", "log", "ln", "exp"]]
    expected += [[r("tr", True), r("A")], [r("d", None, "p"), r("x")],
                 [r("if ", True, "p"), r("x")],
                 [["delimiter", "(", ")", "", True, [[frac(a("n"), a("k"), "noBar")]]]],
                 [frac(a("1"), a("2"))], [frac(a("1"), a("2"))], a("a\u2001b"),
                 [frac(a("1"), a("2"))], [["radical", [], a("2")]],
                 [["radical", a("3"), a("x")]]]
    expected += [[["nary", operator, a("i=1"), a("n"), a("i")]] for operator in ["∑", "∏"]]
    expected += [[["nary", "∫", a("0"), a("1"), a("x")], r("\u2009dx")],
                 [["nary", "∮", [], [], []]],
                 [["delimiter", "(", ")", "|", None, [a("x")]]]]
    expected += [[["accent", accent, a(base)]] for accent, base in
                 [("\u0304", "x"), ("\u0302", "x"), ("\u0304", "x"), ("\u20d7", "v"), ("\u0307", "x")]]
    expected += [a("a\u2004b")]
    mass = [r("M="), frac([r("4"), sup("π", "2"), sup("a", "3")], [r("G"), sup("T", "2")])]
    expected += [mass, mass]
    assert len(expected) == 73
    return expected


def fx247_omml_semantics(equation: ElementTree.Element) -> list:
    """Read selected saved OMML using only stdlib XML and independently fixed roles."""
    ns = "{http://schemas.openxmlformats.org/officeDocument/2006/math}"

    def attr(element, name, default=None):
        child = element.find(ns + name)
        return default if child is None else child.get(ns + "val", "1")

    def argument(element):
        if element is None:
            return []
        values = []
        for child in element:
            name = child.tag.removeprefix(ns)
            if name.endswith("Pr"):
                continue
            if name == "r":
                properties = child.find(ns + "rPr")
                normal = attr(properties, "nor") if properties is not None else None
                style = attr(properties, "sty") if properties is not None else None
                value = ["run", "".join(t.text or "" for t in child.findall(ns + "t")),
                         None if normal is None else normal in ("1", "true", "on"), style]
                if values and values[-1][0] == "run" and values[-1][2:] == value[2:]:
                    values[-1][1] += value[1]
                    continue
            elif name == "f":
                properties = child.find(ns + "fPr")
                value = ["fraction", attr(properties, "type", "bar") if properties is not None else "bar",
                         argument(child.find(ns + "num")), argument(child.find(ns + "den"))]
            elif name in ("sSub", "sSup"):
                role, script = ("sub", "sub") if name == "sSub" else ("sup", "sup")
                value = [role, argument(child.find(ns + "e")), argument(child.find(ns + script))]
            elif name == "rad":
                value = ["radical", argument(child.find(ns + "deg")), argument(child.find(ns + "e"))]
            elif name == "nary":
                properties = child.find(ns + "naryPr")
                value = ["nary", attr(properties, "chr", "∫"), argument(child.find(ns + "sub")),
                         argument(child.find(ns + "sup")), argument(child.find(ns + "e"))]
            elif name == "d":
                properties = child.find(ns + "dPr")
                grow = attr(properties, "grow") if properties is not None else None
                value = ["delimiter", attr(properties, "begChr", "("), attr(properties, "endChr", ")"),
                         attr(properties, "sepChr", "|"), None if grow is None else grow in ("1", "true", "on"),
                         [argument(e) for e in child.findall(ns + "e")]]
            elif name == "acc":
                value = ["accent", attr(child.find(ns + "accPr"), "chr", "\u0302"), argument(child.find(ns + "e"))]
            else:
                raise ValueError(f"unexpected selected OMML element {child.tag}")
            values.append(value)
        return values

    return argument(equation)


def verify_fx247_package(path: Path) -> dict:
    with zipfile.ZipFile(path) as archive:
        raw = archive.read("word/document.xml")
    root = ElementTree.fromstring(raw)
    namespace = "{http://schemas.openxmlformats.org/officeDocument/2006/math}"
    equations = root.findall(".//" + namespace + "oMath")
    expected = fx247_expected_equations()
    if len(equations) != len(expected):
        raise ValueError(f"expected73 selected equations, got{len(equations)}")
    rows = []
    for index, (equation, wanted) in enumerate(zip(equations, expected), 1):
        actual = fx247_omml_semantics(equation)
        if actual != wanted:
            raise ValueError(f"row{index} saved semantics differ: expected{wanted!r}, actual{actual!r}")
        rows.append({"row": index, "semantics": actual,
                     "selected_omml_sha256": hashlib.sha256(ElementTree.tostring(equation)).hexdigest()})
    return {"package": str(path), "sha256": hashlib.sha256(path.read_bytes()).hexdigest(), "rows": rows, "passed": True}



def verify_fx247_pandoc(directory: Path) -> dict:
    """Qualify retained actual3.11 outputs against independent finite semantics.

    This is captured-output qualification. The producer receipt binds each
    actual command and output, including the original degree warning.
    """
    out = directory
    receipt = json.loads((out / "receipt.json").read_text())
    assert receipt["identity"].splitlines()[0] == "pandoc 3.11"
    assert receipt["binary_sha256"] == "23908d6bc347072befa2bfe312f2c1c44a7c9c2e9e60bb5d41069245454b3236"
    assert hashlib.sha256(Path(receipt["binary"]).read_bytes()).hexdigest() == receipt["binary_sha256"]
    assert receipt["source_math_sha256"] == hashlib.sha256((ROOT / "crates/rdocx/src/math.rs").read_bytes()).hexdigest()
    expected = fx247_expected_equations()
    ns = "{http://schemas.openxmlformats.org/officeDocument/2006/math}"
    # This comparison projects independent logical semantics and declared writer defaults.
    # It does not call any product exporter or normalizer.
    def normalize(values):
     result=[]
     for value in values:
      tag=value[0]
      if tag=='run':
       _,text,normal,style=value
       if text in ['lim','sin','log','ln','exp','tr'] and (normal is True or style=='p'):
        result.append(['function',text]);continue
       if normal is True:
        result.append(['text',text]);continue
       if style=='p' and text and all(c.isascii() and c.isalpha() for c in text):
        result.append(['upright-math',text]);continue
       for char in text:
        result.append(['symbol',char])
      elif tag=='fraction':result.append([tag,value[1],normalize(value[2]),normalize(value[3])])
      elif tag in ['sup','sub','radical']:result.append([tag,normalize(value[1]),normalize(value[2])])
      elif tag=='delimiter':
       # A single argument has no separator boundary. Missing grow is the OOXML
       # default true, independently of the product's explicit property choice.
       separator='' if len(value[5])==1 else value[3]
       grow=True if value[4] is None else value[4]
       result.append([tag,value[1],value[2],separator,grow,[normalize(v) for v in value[5]]])
      elif tag=='nary':
       if not any(value[2:]):result.append(['symbol',value[1]])
       else:result.append([tag,value[1],*[normalize(v) for v in value[2:]]])
      elif tag=='accent':result.append([tag,'̄' if value[1] in ['¯','‾','̅'] else value[1],normalize(value[2])])
      else:raise AssertionError(tag)
     return result
    
    def raw_semantics(path):
     with zipfile.ZipFile(path) as z: root=ElementTree.fromstring(z.read('word/document.xml'))
     eq=root.find('.//'+ns+'oMath');assert eq is not None,path
     # The existing upright-lim convention retains lower-script ownership even
     # when a display writer chooses the equivalent under-limit placement.
     for low in list(eq.iter(ns+'limLow')):
      low.tag=ns+'sSub'
      low.find(ns+'lim').tag=ns+'sub'
     # Pandoc's overline DOCX writer uses a genuine top bar rather than combining accent.
     for bar in list(eq.iter(ns+'bar')):
      props=bar.find(ns+'barPr');pos=props.find(ns+'pos') if props is not None else None
      assert pos is None or pos.get(ns+'val')=='top'
      bar.tag=ns+'acc';pr=ElementTree.Element(ns+'accPr');ch=ElementTree.SubElement(pr,ns+'chr');ch.set(ns+'val','̄')
      if props is not None:bar.remove(props)
      bar.insert(0,pr)
     return fx247_omml_semantics(eq)
    rows=[]
    assert len(receipt['rows']) == 146
    assert {(r['row'], r['context']) for r in receipt['rows']} == {(i, c) for i in range(1, 74) for c in ['inline', 'display']}
    for row in receipt['rows']:
     assert row['source'] == load_manifest()['fx247']['cases'][row['row']-1]
     assert row['context'] in ['inline','display']
     assert row['html_exit'] == row['docx_exit'] == 0
     wrapper = '$' if row['context'] == 'inline' else '$$'
     assert (out/f"case-{row['row']:02}"/f"{row['context']}.tex").read_text() == wrapper + row['source'] + wrapper
     for name, digest in row['outputs'].items():
      path=out/f"case-{row['row']:02}"/name
      assert hashlib.sha256(path.read_bytes()).hexdigest() == digest, path
     i=row['row'];context=row['context'];folder=out/f'case-{i:02}'
     if i==43:
      assert not row['emitted_math'] and 'degree' in row['html_stderr'];rows.append({'row':i,'context':context,'classification':'actual unsupported degree warning, no math emitted; independentUnicode/nativeauthority required'});continue
     assert row['import_diagnostics']==[{'path':'/math[1]/@display','message':'unsupported MathML attribute was discarded'},{'path':'/math[1]/semantics[1]','message':'MathML semantics metadata was discarded'}],(i,context,row['import_diagnostics'])
     native=raw_semantics(folder/f'{context}.docx');imported=raw_semantics(folder/f'{context}-product-from-mathml.docx');want=normalize(expected[i-1]);nw=normalize(native);mw=normalize(imported);differences=[]
     if i==56:
      assert nw[0][0:2]==['fraction','lin'];nw[0][1]='bar';differences.append('PandocDOCX writes realLinear tfrac; product canonicalstackedBar intentional')
     # Generic standalone contour and mathfont styling defaults normalize as above.
     assert nw==want,('DOCX',i,context,nw,want)
     assert mw==want,('MathML',i,context,mw,want)
     if i in [47,48,49,50,51]:
      xml=(folder/f'{context}-no-annotation.xml').read_text();assert ('⁡' in xml or '&#8289;' in xml);differences.append('MathMLfunctionmarker independentlypresent; nativeupright carrier normalized')
     if i==53:
      assert '0.333em' in (folder/f'{context}-no-annotation.xml').read_text();assert ['text','if '] in mw;differences.append('explicitandlexicalboundaryspacecountonce')
     if i in [55,56]:differences.append('forceddisplaystylemstylenormalizedtoOfficeMathBar')
     if i==54:
      assert nw[0][5][0][0][1]=='noBar';differences.append('noBarparenspreserved; actualstructuralreverseordinaryfraclossretained')
     rows.append({'row':i,'context':context,'logical_semantics':want,'declared_normalizations':differences,'passed':True})
    return {"source_receipt_sha256": hashlib.sha256((out / "receipt.json").read_bytes()).hexdigest(),
        "compared72_per_context": True, "degree_unemitted": True,
        "normalization": "writer styling defaults, marked upright functions, combining bar aliases, standalone contours, canonical fraction style and single-argument delimiter defaults",
        "rows": rows, "passed": True}


def verify_fx247_native(directory: Path, product_suffix: str, *, include_diagnostic: bool = False) -> dict:
    """Measure actual Word and product PDF bounds, widths, ink and raster together."""
    manifest = load_manifest()
    fixture = manifest["fx247"]
    application = fixture["word_application"]
    info = Path(application["info_plist"])
    if hashlib.sha256(info.read_bytes()).hexdigest() != application["sha256"]:
        raise ValueError("F-X247 current native producer application drift")
    actual = plistlib.loads(info.read_bytes())
    if any(actual.get(key) != value for key, value in application["identity"].items()):
        raise ValueError("F-X247 current native producer identity drift")
    if run(["pdftoppm", "-v"]).strip().splitlines()[0] != fixture["poppler_identity"]:
        raise ValueError("F-X247 Poppler version drift")
    for entry in [fixture["native_producer_receipt"], fixture["independent_capture_audit"], *fixture.get("supplemental_native_producer_receipts", [])]:
        if hashlib.sha256(Path(entry["path"]).read_bytes()).hexdigest() != entry["sha256"]:
            raise ValueError("F-X247 independent native receipt drift")
    for path, expected in {**fixture["font_files"], **fixture["tools"]}.items():
        if hashlib.sha256(Path(path).read_bytes()).hexdigest() != expected["sha256"]:
            raise ValueError(f"F-X247 physical font/tool drift: {path}")
        if path in fixture["tools"] and Path(shutil.which(Path(path).name) or "missing").resolve() != Path(path).resolve():
            raise ValueError(f"F-X247 tool resolution drift: {path}")
    results = {}
    tolerance = manifest["geometry_tolerance_pt"]
    controls = dict(fixture["native_controls"])
    if include_diagnostic:
        controls.update(fixture["diagnostic_native_controls"])
    for name, control in controls.items():
        source = directory / f"{name}-source.docx"
        reopened = directory / f"{name}-native-reopened.docx"
        native = directory / f"{name}-native.pdf"
        product = directory / f"{name}-{product_suffix}.pdf"
        for path, key in [(source, "source_sha256"), (reopened, "native_reopened_sha256"), (native, "native_pdf_sha256")]:
            if hashlib.sha256(path.read_bytes()).hexdigest() != control[key]:
                raise ValueError(f"F-X247 independent native artifact drift: {path}")
        start = DOCUMENT.index(b"<m:oMath>")
        end = DOCUMENT.index(b"</m:oMath>") + len(b"</m:oMath>")
        document = DOCUMENT[:start] + control["authored_omml"].encode() + DOCUMENT[end:]
        recipe = source_docx_bytes(document, text_font="Cambria", math_font="Cambria Math", explicit_spacing=True)
        if source.read_bytes() != recipe:
            raise ValueError(f"F-X247 exact source recipe differs: {name}")
        native_box = pdf_math_bbox(native, fixture_header=True)
        product_box = pdf_math_bbox(product, fixture_header=True)
        pdf_match = all(abs(a-b) <= tolerance for a,b in zip(native_box, product_box)) and geometry_matches(comparable_geometry(native_box), comparable_geometry(product_box), tolerance)
        rasters = []
        with tempfile.TemporaryDirectory(prefix="fx247-raster-") as tmp:
            for label, pdf in [("native", native), ("product", product)]:
                path = Path(tmp) / label
                run(["pdftoppm", "-f", "1", "-singlefile", "-r", str(manifest["dpi"]), str(pdf), str(path)])
                rasters.append(read_ppm(path.with_suffix(".ppm")))
            nw, nh, np = rasters[0]
            pw, ph, pp = rasters[1]
            if (nw, nh) != (pw, ph):
                raise ValueError(f"F-X247 raster page dimensions differ: {name}")
            native_ink = raster_box(np, nw, nh, manifest["dpi"], [70, 540, 86, 150])
            product_ink = raster_box(pp, pw, ph, manifest["dpi"], [70, 540, 86, 150])
            ink_match = all(abs(a-b) <= tolerance for a,b in zip(native_ink, product_ink)) and geometry_matches(comparable_geometry(native_ink), comparable_geometry(product_ink), tolerance)
            ssim = luminance_ssim(np, pp, nw, nh)
        results[name] = {"native_pdf_bbox_pt": native_box, "product_pdf_bbox_pt": product_box,
            "native_ink_bbox_pt": native_ink, "product_ink_bbox_pt": product_ink,
            "pdf_geometry_pass": pdf_match, "raster_geometry_pass": ink_match,
            "luminance_ssim": ssim, "passed": pdf_match and ink_match and ssim >= manifest["luminance_ssim_floor"],
            "product_pdf_sha256": hashlib.sha256(product.read_bytes()).hexdigest(),
            "native_pdf_sha256": control["native_pdf_sha256"]}
    return {"fixture": fixture["fixture"], "word_identity": fixture["word_identity"],
        "product_suffix": product_suffix, "includes_supplementary_diagnostic": include_diagnostic, "results": results, "passed": all(v["passed"] for v in results.values())}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-check", action="store_true")
    parser.add_argument("--verify-fx247-package", type=Path)
    parser.add_argument("--verify-fx247-pandoc", type=Path)
    parser.add_argument("--verify-fx247-native", type=Path)
    parser.add_argument("--fx247-product-suffix", default="product")
    parser.add_argument("--fx247-include-diagnostic", action="store_true")
    parser.add_argument("--print-source-sha", action="store_true")
    parser.add_argument("--build-source", type=Path)
    parser.add_argument("--build-fx235-sources", type=Path)
    parser.add_argument("--build-fx235-v2-sources", type=Path)
    parser.add_argument("--fx235-fixture", choices=["fx235", "fx235_v2"], default="fx235_v2")
    parser.add_argument("--word-pdf", type=Path)
    parser.add_argument("--verify-fx235", type=Path)
    parser.add_argument("--fx235-product-suffix", default="product")
    parser.add_argument("--fx235-historical-producer", action="store_true")
    parser.add_argument("--word-identity")
    parser.add_argument("--rust-pdf", type=Path)
    args = parser.parse_args()
    if args.verify_fx247_pandoc is not None:
        print(json.dumps(verify_fx247_pandoc(args.verify_fx247_pandoc), indent=2, ensure_ascii=False))
    if args.verify_fx247_native is not None:
        receipt = verify_fx247_native(args.verify_fx247_native, args.fx247_product_suffix, include_diagnostic=args.fx247_include_diagnostic)
        print(json.dumps(receipt, indent=2))
        if not receipt["passed"]:
            raise SystemExit("F-X247 native comparison failed")
    if args.verify_fx247_package is not None:
        print(json.dumps(verify_fx247_package(args.verify_fx247_package), indent=2, ensure_ascii=False))
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
        verify_fx235(args.verify_fx235, args.word_identity, args.fx235_product_suffix, args.fx235_fixture, historical_producer=args.fx235_historical_producer)
    if args.word_pdf is not None:
        if args.word_identity is None:
            parser.error("--word-pdf requires --word-identity")
        verify_word_pdf(args.word_pdf, args.word_identity, args.rust_pdf)
    if not any((args.self_check, args.print_source_sha, args.build_source, args.build_fx235_sources, args.build_fx235_v2_sources, args.word_pdf, args.verify_fx235, args.verify_fx247_package, args.verify_fx247_native, args.verify_fx247_pandoc)):
        parser.error("choose --self-check, --print-source-sha, --build-source, or --word-pdf")


if __name__ == "__main__":
    main()
