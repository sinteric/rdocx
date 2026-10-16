import importlib.metadata
import hashlib
import os
import re
import struct
import subprocess
import urllib.request
import zipfile
import zlib
from pathlib import Path

import pytest


ORACLE_DISTRIBUTION = "python-docx"
ORACLE_VERSION = "1.2.0"
TAGGED_SOURCE_ROOT = (
    "https://raw.githubusercontent.com/python-openxml/python-docx/"
    "v1.2.0/docs/user"
)
TAGGED_SOURCE_PAGES = {
    "documents": f"{TAGGED_SOURCE_ROOT}/documents.rst",
    "quickstart": f"{TAGGED_SOURCE_ROOT}/quickstart.rst",
    "text": f"{TAGGED_SOURCE_ROOT}/text.rst",
}
HELD_ROW_SOURCE_BODY = (
    "row = table.rows[1]\n"
    "row.cells[0].text = 'Foo bar to you.'\n"
    "row.cells[1].text = 'And a hearty foo bar to you too sir!'\n"
)
HELD_ROW_RDOCX_BODY = (
    "row = table.rows[1]\n"
    "row.cells[0].text = 'Foo bar to you.'\n"
    "row = document.tables[0].rows[1]\n"
    "row.cells[1].text = 'And a hearty foo bar to you too sir!'\n"
)

EXPECTED_MANIFEST_IDS = frozenset(
    {
        "documents.opening-a-document",
        "documents.really-opening-a-document",
        "quickstart.opening-a-document",
        "quickstart.adding-a-paragraph",
        "quickstart.adding-a-table",
        "quickstart.table-cell-text",
        "quickstart.held-row-two-cell-assignment",
        "quickstart.table-iteration",
        "quickstart.table-style",
        "quickstart.adding-runs",
        "text.horizontal-alignment",
        "text.indentation",
        "text.line-spacing",
        "text.paragraph-spacing",
        "text.pagination-properties",
        "text.font-name-and-size",
        "text.font-tristate-and-underline",
    }
)

DOCUMENTED_S33_EXAMPLES = (
    {
        "id": "documents.opening-a-document",
        "page": TAGGED_SOURCE_PAGES["documents"],
        "heading": "Opening a document",
        "body": "from docx import Document\n\ndocument = Document()\ndocument.save('test.docx')\n",
        "transformation": "namespace-only",
        "setup": "empty",
        "observation": "files",
        "expected": ("test.docx",),
    },
    {
        "id": "documents.really-opening-a-document",
        "page": TAGGED_SOURCE_PAGES["documents"],
        "heading": "REALLY opening a document",
        "body": "document = Document('existing-document-file.docx')\ndocument.save('new-file-name.docx')\n",
        "transformation": "namespace-only",
        "setup": "existing-document",
        "observation": "paragraph-texts",
        "expected": ("existing",),
    },
    {
        "id": "quickstart.opening-a-document",
        "page": TAGGED_SOURCE_PAGES["quickstart"],
        "heading": "Opening a document",
        "body": "from docx import Document\n\ndocument = Document()\n",
        "transformation": "namespace-only",
        "setup": "empty",
        "observation": "paragraph-texts",
        "expected": (),
    },
    {
        "id": "quickstart.adding-a-paragraph",
        "page": TAGGED_SOURCE_PAGES["quickstart"],
        "heading": "Adding a paragraph",
        "body": "paragraph = document.add_paragraph('Lorem ipsum dolor sit amet.')\n",
        "transformation": "namespace-only",
        "setup": "document",
        "observation": "paragraph-texts",
        "expected": ("Lorem ipsum dolor sit amet.",),
    },
    {
        "id": "quickstart.adding-a-table",
        "page": TAGGED_SOURCE_PAGES["quickstart"],
        "heading": "Adding a table",
        "body": "table = document.add_table(rows=2, cols=2)\n",
        "transformation": "namespace-only",
        "setup": "document",
        "observation": "table-shape",
        "expected": ((2, 2),),
    },
    {
        "id": "quickstart.table-cell-text",
        "page": TAGGED_SOURCE_PAGES["quickstart"],
        "heading": "Adding a table",
        "body": "cell = table.cell(0, 1)\ncell.text = 'parrot, possibly dead'\n",
        "transformation": "namespace-only",
        "setup": "table",
        "observation": "table-cells",
        "expected": (("", "parrot, possibly dead"), ("", "")),
    },
    {
        "id": "quickstart.held-row-two-cell-assignment",
        "page": TAGGED_SOURCE_PAGES["quickstart"],
        "heading": "Adding a table",
        "body": (
            "row = table.rows[1]\n"
            "row.cells[0].text = 'Foo bar to you.'\n"
            "row.cells[1].text = 'And a hearty foo bar to you too sir!'\n"
        ),
        "transformation": "namespace-and-held-row-refetch",
        "setup": "table",
        "observation": "table-cells",
        "expected": (
            ("", ""),
            ("Foo bar to you.", "And a hearty foo bar to you too sir!"),
        ),
    },
    {
        "id": "quickstart.table-iteration",
        "page": TAGGED_SOURCE_PAGES["quickstart"],
        "heading": "Adding a table",
        "body": "for row in table.rows:\n    for cell in row.cells:\n        print(cell.text)\n",
        "transformation": "namespace-only",
        "setup": "populated-table",
        "observation": "table-cells",
        "expected": (("alpha", "beta"), ("gamma", "delta")),
    },
    {
        "id": "quickstart.table-style",
        "page": TAGGED_SOURCE_PAGES["quickstart"],
        "heading": "Adding a table",
        "body": "table.style = 'LightShading-Accent1'\n",
        "transformation": "namespace-only",
        "setup": "table",
        "observation": "table-style",
        "expected": "LightShading-Accent1",
    },
    {
        "id": "quickstart.adding-runs",
        "page": TAGGED_SOURCE_PAGES["quickstart"],
        "heading": "Applying bold and italic",
        "body": "paragraph = document.add_paragraph('Lorem ipsum ')\nparagraph.add_run('dolor sit amet.')\n",
        "transformation": "namespace-only",
        "setup": "document",
        "observation": "runs",
        "expected": (("Lorem ipsum ", "dolor sit amet."),),
    },
    {
        "id": "text.horizontal-alignment",
        "page": TAGGED_SOURCE_PAGES["text"],
        "heading": "Horizontal alignment (justification)",
        "body": "from docx.enum.text import WD_ALIGN_PARAGRAPH\nparagraph_format.alignment = WD_ALIGN_PARAGRAPH.CENTER\n",
        "transformation": "namespace-only",
        "setup": "paragraph-format",
        "observation": "alignment",
        "expected": 1,
    },
    {
        "id": "text.indentation",
        "page": TAGGED_SOURCE_PAGES["text"],
        "heading": "Indentation",
        "body": "from docx.shared import Inches\nfrom docx.shared import Pt\nparagraph_format.left_indent = Inches(0.5)\nparagraph_format.right_indent = Pt(24)\nparagraph_format.first_line_indent = Inches(-0.25)\n",
        "transformation": "namespace-only",
        "setup": "paragraph-format",
        "observation": "indentation",
        "expected": (457200, 304800, -228600),
    },
    {
        "id": "text.line-spacing",
        "page": TAGGED_SOURCE_PAGES["text"],
        "heading": "Line spacing",
        "body": "paragraph_format.line_spacing = Pt(18)\nparagraph_format.line_spacing = 1.75\n",
        "transformation": "namespace-only",
        "setup": "paragraph-format",
        "observation": "line-spacing",
        "expected": 1.75,
    },
    {
        "id": "text.paragraph-spacing",
        "page": TAGGED_SOURCE_PAGES["text"],
        "heading": "Paragraph spacing",
        "body": "paragraph_format.space_before = Pt(18)\nparagraph_format.space_after = Pt(12)\n",
        "transformation": "namespace-only",
        "setup": "paragraph-format",
        "observation": "spacing",
        "expected": (228600, 152400),
    },
    {
        "id": "text.pagination-properties",
        "page": TAGGED_SOURCE_PAGES["text"],
        "heading": "Pagination properties",
        "body": "paragraph_format.keep_with_next = True\nparagraph_format.page_break_before = False\n",
        "transformation": "namespace-only",
        "setup": "paragraph-format",
        "observation": "pagination",
        "expected": (True, False),
    },
    {
        "id": "text.font-name-and-size",
        "page": TAGGED_SOURCE_PAGES["text"],
        "heading": "Apply character formatting",
        "body": "from docx.shared import Pt\nfont.name = 'Calibri'\nfont.size = Pt(12)\n",
        "transformation": "namespace-only",
        "setup": "font",
        "observation": "font-name-and-size",
        "expected": ("Calibri", 152400),
    },
    {
        "id": "text.font-tristate-and-underline",
        "page": TAGGED_SOURCE_PAGES["text"],
        "heading": "Apply character formatting",
        "body": "font.italic = True\nfont.italic = False\nfont.italic = None\nfont.underline = True\nfont.underline = WD_UNDERLINE.DOT_DASH\n",
        "transformation": "namespace-only",
        "setup": "font",
        "observation": "font-tristate-and-underline",
        "expected": (None, 9),
    },
)


def _assert_oracle_version():
    assert importlib.metadata.version(ORACLE_DISTRIBUTION) == ORACLE_VERSION


def _namespace_only(body, namespace):
    if namespace == "docx":
        return body
    assert namespace == "rdocx"
    transformed = body.replace("from docx ", "from rdocx ")
    transformed = transformed.replace("from docx.", "from rdocx.")
    assert transformed.replace("from rdocx ", "from docx ").replace(
        "from rdocx.", "from docx."
    ) == body
    return transformed


def _example_body(example, namespace):
    transformed = _namespace_only(example["body"], namespace)
    if namespace == "docx" or example["transformation"] == "namespace-only":
        return transformed
    assert example["id"] == "quickstart.held-row-two-cell-assignment"
    assert example["transformation"] == "namespace-and-held-row-refetch"
    assert transformed == HELD_ROW_SOURCE_BODY
    return HELD_ROW_RDOCX_BODY


def _fresh_cell(document, row, column):
    return document.tables[0].rows[row].cells[column]


def _example_context(setup, root):
    import rdocx

    namespace = {
        "Document": rdocx.Document,
        "Pt": rdocx.Pt,
        "WD_UNDERLINE": rdocx.WD_UNDERLINE,
    }
    if setup == "empty":
        return namespace

    document = rdocx.Document()
    namespace["document"] = document
    if setup == "existing-document":
        source = rdocx.Document()
        source.add_paragraph("existing")
        source.save(root / "existing-document-file.docx")
        return namespace
    if setup == "document":
        return namespace

    if setup in ("table", "populated-table"):
        table = document.add_table(rows=2, cols=2)
        namespace["table"] = table
    if setup == "table":
        return namespace
    if setup == "populated-table":
        for row, values in enumerate((("alpha", "beta"), ("gamma", "delta"))):
            for column, value in enumerate(values):
                _fresh_cell(document, row, column).text = value
        namespace["table"] = document.tables[0]
        return namespace

    paragraph = document.add_paragraph("")
    if setup == "paragraph-format":
        namespace["paragraph_format"] = paragraph.paragraph_format
        return namespace
    if setup == "font":
        run = paragraph.add_run("")
        namespace["font"] = run.font
        return namespace
    raise AssertionError(f"unknown example setup: {setup}")


def _table_cells(document):
    return tuple(
        tuple(cell.text for cell in row.cells) for row in document.tables[0].rows
    )


def _example_observation(example, namespace, root):
    observation = example["observation"]
    if observation == "files":
        return tuple(sorted(path.name for path in root.glob("*.docx")))
    document = namespace["document"]
    if observation == "paragraph-texts":
        return tuple(paragraph.text for paragraph in document.paragraphs)
    if observation == "table-shape":
        return tuple(
            (len(table.rows), len(table.rows[0].cells)) for table in document.tables
        )
    if observation == "table-cells":
        return _table_cells(document)
    if observation == "table-style":
        return document.tables[0].style
    if observation == "runs":
        return tuple(
            tuple(run.text for run in paragraph.runs)
            for paragraph in document.paragraphs
        )
    paragraph = document.paragraphs[-1]
    paragraph_format = paragraph.paragraph_format
    if observation == "alignment":
        value = paragraph_format.alignment
        return int(value) if value is not None else None
    if observation == "indentation":
        return (
            int(paragraph_format.left_indent),
            int(paragraph_format.right_indent),
            int(paragraph_format.first_line_indent),
        )
    if observation == "line-spacing":
        return paragraph_format.line_spacing
    if observation == "spacing":
        return (int(paragraph_format.space_before), int(paragraph_format.space_after))
    if observation == "pagination":
        return (
            paragraph_format.keep_with_next,
            paragraph_format.page_break_before,
        )
    font = paragraph.runs[-1].font
    if observation == "font-name-and-size":
        return (font.name, int(font.size))
    if observation == "font-tristate-and-underline":
        underline = font.underline
        return (font.italic, int(underline) if underline is not None else None)
    raise AssertionError(f"unknown example observation: {observation}")


def _optional_int(value):
    return int(value) if value is not None else None


def _optional_enum(value):
    if value is None or isinstance(value, bool):
        return value
    return int(value)


def _line_spacing_record(value):
    if value is None:
        return None
    if isinstance(value, float):
        return ("relative", value)
    return ("length", int(value))


def _paragraph_record(paragraph, oracle):
    paragraph_format = paragraph.paragraph_format
    runs = []
    for run in paragraph.runs:
        font = run.font
        color = font.color.rgb
        runs.append(
            (
                run.text,
                font.name,
                _optional_int(font.size),
                tuple(color) if color is not None else None,
                font.bold,
                font.italic,
                _optional_enum(font.underline),
                font.strike,
            )
        )
    return (
        paragraph.text,
        _optional_enum(paragraph_format.alignment),
        _optional_int(paragraph_format.space_before),
        _optional_int(paragraph_format.space_after),
        _optional_int(paragraph_format.left_indent),
        _optional_int(paragraph_format.right_indent),
        _optional_int(paragraph_format.first_line_indent),
        _line_spacing_record(paragraph_format.line_spacing),
        paragraph_format.keep_with_next,
        paragraph_format.keep_together,
        paragraph_format.page_break_before,
        paragraph_format.widow_control,
        tuple(runs),
    )


def _table_style(table, oracle):
    if table.style is None:
        return None
    style = table.style.style_id if oracle else table.style
    return None if oracle and style == "TableNormal" else style


def _document_record(document, oracle):
    paragraphs = tuple(
        _paragraph_record(paragraph, oracle) for paragraph in document.paragraphs
    )
    tables = []
    for table in document.tables:
        rows = []
        for row in table.rows:
            rows.append(
                tuple(
                    (
                        cell.text,
                        _optional_int(cell.width),
                        _optional_enum(cell.vertical_alignment),
                        tuple(
                            _paragraph_record(paragraph, oracle)
                            for paragraph in cell.paragraphs
                        ),
                    )
                    for cell in row.cells
                )
            )
        tables.append(
            (
                _table_style(table, oracle),
                _optional_enum(table.alignment),
                tuple(rows),
            )
        )
    return paragraphs, tuple(tables)


def _author_parity_document(
    Document,
    Inches,
    Pt,
    RGBColor,
    WD_ALIGN_PARAGRAPH,
    WD_UNDERLINE,
    WD_TABLE_ALIGNMENT,
    WD_CELL_VERTICAL_ALIGNMENT,
    set_color,
    source_path,
    path,
):
    document = Document(source_path)
    paragraph = document.add_paragraph("Alpha ")
    paragraph.alignment = WD_ALIGN_PARAGRAPH.CENTER
    paragraph_format = paragraph.paragraph_format
    paragraph_format.space_before = Pt(3)
    paragraph_format.space_after = Pt(6)
    paragraph_format.left_indent = Inches(0.5)
    paragraph_format.right_indent = Inches(0.25)
    paragraph_format.first_line_indent = Inches(-0.25)
    paragraph_format.line_spacing = Pt(18)
    paragraph_format.keep_with_next = True
    paragraph_format.keep_together = False
    paragraph_format.page_break_before = False
    paragraph_format.widow_control = True
    run = paragraph.add_run("beta")
    font = run.font
    font.name = "Aptos"
    font.size = Pt(12)
    set_color(font, RGBColor(0x12, 0x34, 0x56))
    font.bold = True
    font.italic = False
    font.underline = WD_UNDERLINE.DOT_DASH
    font.strike = True

    second = document.add_paragraph("Second")
    second.paragraph_format.line_spacing = 1.75
    table = document.add_table(rows=2, cols=2)
    table.style = "LightShading-Accent1"
    table.alignment = WD_TABLE_ALIGNMENT.CENTER
    for row, values in enumerate((("one", "two"), ("three", "four"))):
        for column, value in enumerate(values):
            cell = document.tables[0].rows[row].cells[column]
            cell.text = value
            cell = document.tables[0].rows[row].cells[column]
            cell.width = Inches(2)
            cell.vertical_alignment = WD_CELL_VERTICAL_ALIGNMENT.BOTTOM
    cell = document.tables[0].rows[0].cells[0]
    nested = cell.add_paragraph("nested")
    nested.paragraph_format.space_after = Pt(6)
    document.save(path)


def test_documented_s33_examples_run_with_declared_transformations(
    tmp_path, monkeypatch
):
    _assert_oracle_version()
    assert TAGGED_SOURCE_ROOT == (
        "https://raw.githubusercontent.com/python-openxml/python-docx/"
        "v1.2.0/docs/user"
    )
    assert {example["id"] for example in DOCUMENTED_S33_EXAMPLES} == (
        EXPECTED_MANIFEST_IDS
    )
    assert len(DOCUMENTED_S33_EXAMPLES) == len(EXPECTED_MANIFEST_IDS)
    for example in DOCUMENTED_S33_EXAMPLES:
        section = example["id"].split(".", 1)[0]
        assert example["page"] == TAGGED_SOURCE_PAGES[section]
        expected_transformation = (
            "namespace-and-held-row-refetch"
            if example["id"] == "quickstart.held-row-two-cell-assignment"
            else "namespace-only"
        )
        assert example["transformation"] == expected_transformation

    held_row = next(
        example
        for example in DOCUMENTED_S33_EXAMPLES
        if example["id"] == "quickstart.held-row-two-cell-assignment"
    )
    assert held_row["body"] == HELD_ROW_SOURCE_BODY
    assert _example_body(held_row, "docx") == HELD_ROW_SOURCE_BODY
    assert _example_body(held_row, "rdocx") == HELD_ROW_RDOCX_BODY
    line_spacing = next(
        example
        for example in DOCUMENTED_S33_EXAMPLES
        if example["id"] == "text.line-spacing"
    )
    assert line_spacing["body"] == (
        "paragraph_format.line_spacing = Pt(18)\n"
        "paragraph_format.line_spacing = 1.75\n"
    )

    for example in DOCUMENTED_S33_EXAMPLES:
        root = tmp_path / example["id"]
        root.mkdir()
        monkeypatch.chdir(root)
        namespace = _example_context(example["setup"], root)
        exec(_example_body(example, "rdocx"), namespace)
        assert _example_observation(example, namespace, root) == example["expected"]


def test_rdocx_and_python_docx_round_trip_the_same_normalized_content(tmp_path):
    _assert_oracle_version()

    import rdocx
    from docx import Document as OracleDocument
    from docx.enum.table import WD_CELL_VERTICAL_ALIGNMENT as OracleCellAlignment
    from docx.enum.table import WD_TABLE_ALIGNMENT as OracleTableAlignment
    from docx.enum.text import WD_ALIGN_PARAGRAPH as OracleParagraphAlignment
    from docx.enum.text import WD_UNDERLINE as OracleUnderline
    from docx.shared import Inches as OracleInches
    from docx.shared import Pt as OraclePt
    from docx.shared import RGBColor as OracleRGBColor

    assert int(rdocx.Inches(1)) == int(OracleInches(1)) == 914400
    assert int(rdocx.Pt(12)) == int(OraclePt(12)) == 152400
    assert int(rdocx.WD_ALIGN_PARAGRAPH.CENTER) == int(
        OracleParagraphAlignment.CENTER
    )
    assert int(rdocx.WD_UNDERLINE.DOT_DASH) == int(OracleUnderline.DOT_DASH)
    assert int(rdocx.WD_TABLE_ALIGNMENT.CENTER) == int(OracleTableAlignment.CENTER)
    assert int(rdocx.WD_CELL_VERTICAL_ALIGNMENT.BOTTOM) == int(
        OracleCellAlignment.BOTTOM
    )

    rdocx_path = tmp_path / "rdocx-authored.docx"
    oracle_path = tmp_path / "python-docx-authored.docx"
    source_path = tmp_path / "styles-source.docx"
    OracleDocument().save(source_path)
    _author_parity_document(
        rdocx.Document,
        rdocx.Inches,
        rdocx.Pt,
        rdocx.RGBColor,
        rdocx.WD_ALIGN_PARAGRAPH,
        rdocx.WD_UNDERLINE,
        rdocx.WD_TABLE_ALIGNMENT,
        rdocx.WD_CELL_VERTICAL_ALIGNMENT,
        lambda font, color: setattr(font, "color", color),
        source_path,
        rdocx_path,
    )
    _author_parity_document(
        OracleDocument,
        OracleInches,
        OraclePt,
        OracleRGBColor,
        OracleParagraphAlignment,
        OracleUnderline,
        OracleTableAlignment,
        OracleCellAlignment,
        lambda font, color: setattr(font.color, "rgb", color),
        source_path,
        oracle_path,
    )

    records = []
    for path in (rdocx_path, oracle_path):
        rdocx_record = _document_record(rdocx.Document(path), oracle=False)
        oracle_record = _document_record(OracleDocument(path), oracle=True)
        assert rdocx_record == oracle_record
        assert tuple(
            paragraph[7] for paragraph in rdocx_record[0]
        ) == (("length", 228600), ("relative", 1.75))
        assert rdocx_record[1][0][0] == "LightShading-Accent1"
        records.append(rdocx_record)
    assert records[0] == records[1]

# Issues 157, 159 and 160: the attached producer and identity matrices are
# built in source so every row exercises the same document operations.
_MATRIX_TIMESTAMP = "2026-09-27T12:00:00Z"
_MATRIX_W14 = "{http://schemas.microsoft.com/office/word/2010/wordml}"
_MATRIX_XML_SPACE = "{http://www.w3.org/XML/1998/namespace}space"
_MATRIX_IDENTITY_ROWS = (
    (None, None),
    ("paragraphs", "w:rsidR"),
    ("paragraphs", "w:rsidRDefault"),
    ("paragraphs", "w:rsidP"),
    ("paragraphs", "w14:paraId"),
    ("paragraphs", "w14:textId"),
    ("runs", "w:rsidR"),
    ("runs", "w:rsidRPr"),
    ("runs", "w:rsidDel"),
    ("field runs", "w:rsidR"),
    ("field runs", "w:rsidRPr"),
    ("footer field runs", "w:rsidR"),
    ("footer field runs", "w:rsidRPr"),
    ("content control", "w:id"),
    ("content control", "w:tag"),
    ("table rows", "w:rsidR"),
    ("table rows", "w:rsidTr"),
    ("table rows", "w14:paraId"),
)
_MATRIX_PRODUCER_ROWS = (
    "control: none",
    "xml:space=preserve on every w:t",
    'w:val="0" toggles on every run',
    "empty <w:pPr/> on plain paragraphs",
    'pageBreakBefore/keepNext w:val="0"',
    'w:orient="portrait" on pgSz',
    "default namespace on the root",
    "packed TOC field run",
    "packed footer fields, no cached result",
    "inline content control on first runs",
    "empty comments part",
)


def _matrix_field(paragraph, instruction, cached=None):
    from docx.oxml.ns import qn

    result = []
    for kind, text in (("begin", None), ("instr", instruction), ("separate", None)):
        run = paragraph.makeelement(qn("w:r"), {})
        if kind == "instr":
            item = run.makeelement(qn("w:instrText"), {})
            item.text = text
            item.set(_MATRIX_XML_SPACE, "preserve")
        else:
            item = run.makeelement(qn("w:fldChar"), {})
            item.set(qn("w:fldCharType"), kind)
        run.append(item)
        paragraph.append(run)
        result.append(run)
    if cached is not None:
        run = paragraph.makeelement(qn("w:r"), {})
        text = run.makeelement(qn("w:t"), {})
        text.text = cached
        run.append(text)
        paragraph.append(run)
        run = paragraph.makeelement(qn("w:r"), {})
        end = run.makeelement(qn("w:fldChar"), {})
        end.set(qn("w:fldCharType"), "end")
        run.append(end)
        paragraph.append(run)
        result.append(run)
    return result


def _matrix_fixture(path, where=None, attr=None, word="alpha"):
    from docx import Document as OracleDocument
    from docx.oxml.ns import qn

    document = OracleDocument()
    body = document.element.body
    field_runs = []
    entries = ["Chapter 1", "Section 1.1", "Chapter 2", "Section 2.1", "Chapter 3", "Section 3.1"]
    for index, title in enumerate(entries):
        paragraph = document.add_paragraph()
        if index == 0:
            field_runs.extend(_matrix_field(paragraph._p, ' TOC \\o "1-3" \\h \\z \\u '))
        paragraph.add_run(title + "\t1")
        if index == len(entries) - 1:
            run = paragraph._p.makeelement(qn("w:r"), {})
            end = run.makeelement(qn("w:fldChar"), {})
            end.set(qn("w:fldCharType"), "end")
            run.append(end)
            paragraph._p.append(run)
            field_runs.append(run)
    for index in range(1, 4):
        document.add_heading(f"Chapter {index}", level=1)
        document.add_paragraph(
            f"Body text of chapter {index}, lorem {word if index == 2 else 'ipsum'} dolor."
        )
        document.add_heading(f"Section {index}.1", level=2)
        document.add_paragraph(f"Body text of section {index}.1.")
    table = document.add_table(rows=2, cols=2)
    for cell in table._cells:
        cell.text = "cell"
    control = body.makeelement(qn("w:sdt"), {})
    properties = control.makeelement(qn("w:sdtPr"), {})
    tag = properties.makeelement(qn("w:tag"), {})
    tag.set(qn("w:val"), "block")
    properties.append(tag)
    control.append(properties)
    content = control.makeelement(qn("w:sdtContent"), {})
    for value in ("Inside the content control, one.", "Inside the content control, two."):
        paragraph = document.add_paragraph(value)._p
        body.remove(paragraph)
        content.append(paragraph)
    control.append(content)
    body.find(qn("w:tbl")).addnext(control)
    footer = document.sections[0].footer.paragraphs[0]._p
    run = footer.makeelement(qn("w:r"), {})
    text = run.makeelement(qn("w:t"), {})
    text.text = "Page "
    text.set(_MATRIX_XML_SPACE, "preserve")
    run.append(text)
    footer.append(run)
    footer_runs = _matrix_field(footer, " PAGE ", "1") + _matrix_field(footer, " NUMPAGES ", "1")
    targets = {
        "paragraphs": list(body.iter(qn("w:p"))),
        "runs": list(body.iter(qn("w:r"))),
        "field runs": field_runs,
        "footer field runs": footer_runs,
        "content control": [properties],
        "table rows": list(body.iter(qn("w:tr"))),
    }
    if where:
        assert targets[where], where
        for element in targets[where]:
            if attr == "w:tag":
                element.find(qn("w:tag")).set(qn("w:val"), "block-2")
            elif attr == "w:id":
                identity = element.makeelement(qn("w:id"), {})
                identity.set(qn("w:val"), "-2000000001")
                element.insert(0, identity)
            elif attr.startswith("w14:"):
                element.set(_MATRIX_W14 + attr[4:], "1A2B3C4D")
            else:
                element.set(qn(attr), "00A1B2C3")
    document.save(path)
    return path


def _matrix_part(path, part):
    with zipfile.ZipFile(path) as package:
        return package.read(part)


def _matrix_attribute_count(path, attr):
    data = _matrix_part(path, "word/document.xml") + _matrix_part(path, "word/footer1.xml")
    if attr == "w:tag":
        return data.count(b"<w:tag ")
    if attr == "w:id":
        return data.count(b"<w:id ")
    return data.count(b" " + attr.encode() + b"=")


def _matrix_comparison_count(original, edited):
    import rdocx

    compared = rdocx.Document(original)
    compared.compare(rdocx.Document(edited), "R", _MATRIX_TIMESTAMP)
    return len(compared.revisions)


@pytest.mark.parametrize("where,attr", _MATRIX_IDENTITY_ROWS)
def test_issue_159_identity_matrix_across_operations(tmp_path, where, attr):
    _assert_oracle_version()
    import rdocx

    plain = _matrix_fixture(tmp_path / "plain.docx")
    source = _matrix_fixture(tmp_path / "source.docx", where, attr)
    edited = _matrix_fixture(tmp_path / "edited.docx", where, attr, word="ALPHA")
    assert len(_MATRIX_IDENTITY_ROWS) == 18
    assert len(_MATRIX_IDENTITY_ROWS) * 7 == 126
    if attr is not None:
        assert _matrix_attribute_count(source, attr) > 0
    if attr == "w:tag":
        assert b'block-2' in _matrix_part(source, "word/document.xml")
    document = rdocx.Document(source)
    assert document.try_replace_text("Body text of section 3.1.", "Body text of section three.") == 1
    saved = tmp_path / "saved.docx"
    document.save(saved)
    if attr is not None:
        assert _matrix_attribute_count(source, attr) == _matrix_attribute_count(saved, attr)
    else:
        assert _matrix_part(source, "word/document.xml") != _matrix_part(saved, "word/document.xml")

    document = rdocx.Document(source)
    assert document.try_replace_text("lorem", "LOREM") == 3
    replaced = tmp_path / "replaced.docx"
    document.save(replaced)
    assert b"LOREM" in _matrix_part(replaced, "word/document.xml")
    assert rdocx.Document(source).rebuild_toc().entry_count == 6
    fields = rdocx.Document(source)
    fields.update_page_fields()
    refreshed = tmp_path / "refreshed.docx"
    fields.save(refreshed)
    assert len(re.findall(rb'fldCharType="separate"/>(?:</w:r><w:r>)?<w:t>([^<]*)</w:t>', _matrix_part(refreshed, "word/footer1.xml"))) == 2
    assert rdocx.Document(source).to_pdf().startswith(b"%PDF")
    assert _matrix_comparison_count(plain, source) == 0
    assert _matrix_comparison_count(source, edited) == 2


def _matrix_rewrite(path, trait):
    empty_comments = trait == "empty comments part"
    with zipfile.ZipFile(path) as package:
        items = [(item, package.read(item.filename)) for item in package.infolist()]
    with zipfile.ZipFile(path, "w", zipfile.ZIP_DEFLATED) as package:
        for item, data in items:
            if item.filename == "word/document.xml":
                xml = data.decode()
                if trait == "xml:space=preserve on every w:t":
                    xml = xml.replace("<w:t>", '<w:t xml:space="preserve">')
                elif trait == 'w:val="0" toggles on every run':
                    xml = xml.replace("<w:r>", '<w:r><w:rPr><w:b w:val="0"/><w:i w:val="0"/></w:rPr>')
                elif trait == "empty <w:pPr/> on plain paragraphs":
                    xml = xml.replace("<w:p><w:r>", "<w:p><w:pPr/><w:r>")
                elif trait == 'pageBreakBefore/keepNext w:val="0"':
                    xml = xml.replace("<w:p><w:r>", '<w:p><w:pPr><w:keepNext w:val="0"/><w:pageBreakBefore w:val="0"/></w:pPr><w:r>')
                elif trait == 'w:orient="portrait" on pgSz':
                    xml = xml.replace("<w:pgSz ", '<w:pgSz w:orient="portrait" ')
                elif trait == "default namespace on the root":
                    xml = xml.replace("<w:document ", '<w:document xmlns="http://schemas.microsoft.com/office/tasks/2019/documenttasks" ', 1)
                elif trait == "packed TOC field run":
                    xml = re.sub(
                        r'<w:r><w:fldChar w:fldCharType="begin"/></w:r><w:r>(<w:instrText[^>]*>[^<]*</w:instrText>)</w:r><w:r><w:fldChar w:fldCharType="separate"/></w:r>',
                        r'<w:r><w:fldChar w:fldCharType="begin"/>\1<w:fldChar w:fldCharType="separate"/></w:r>',
                        xml,
                    )
                elif trait == "inline content control on first runs":
                    xml = re.sub(
                        r'(<w:p>(?:<w:pPr>.*?</w:pPr>)?)(<w:r>.*?</w:r>)',
                        r'\1<w:sdt><w:sdtPr><w:tag w:val="goog_rdk_0"/></w:sdtPr><w:sdtContent>\2</w:sdtContent></w:sdt>',
                        xml,
                    )
                data = xml.encode()
            elif item.filename == "word/footer1.xml" and trait == "packed footer fields, no cached result":
                xml = data.decode()
                xml = re.sub(
                    r'<w:r><w:fldChar w:fldCharType="begin"/></w:r><w:r>(<w:instrText[^>]*>[^<]*</w:instrText>)</w:r><w:r><w:fldChar w:fldCharType="separate"/></w:r>',
                    r'<w:r><w:fldChar w:fldCharType="begin"/>\1<w:fldChar w:fldCharType="separate"/></w:r>',
                    xml,
                )
                xml = xml.replace(
                    '<w:fldChar w:fldCharType="separate"/></w:r><w:r><w:t>1</w:t></w:r><w:r><w:fldChar w:fldCharType="end"/></w:r>',
                    '<w:fldChar w:fldCharType="separate"/><w:fldChar w:fldCharType="end"/></w:r>',
                )
                data = xml.encode()
            if empty_comments and item.filename == "[Content_Types].xml":
                data = data.replace(b"</Types>", b'<Override PartName="/word/comments.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"/></Types>')
            if empty_comments and item.filename == "word/_rels/document.xml.rels":
                data = data.replace(b"</Relationships>", b'<Relationship Id="rId99" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments" Target="comments.xml"/></Relationships>')
            package.writestr(item, data)
        if empty_comments:
            package.writestr(
                "word/comments.xml",
                b'<?xml version="1.0" encoding="UTF-8" standalone="yes"?>\n<w:comments xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml"/>',
            )
    _matrix_assert_trait(path, trait)
    return path


def _matrix_assert_trait(path, trait):
    empty_comments = trait == "empty comments part"
    document = _matrix_part(path, "word/document.xml")
    footer = _matrix_part(path, "word/footer1.xml")
    signatures = {
        "xml:space=preserve on every w:t": (document, b'xml:space="preserve">Body text'),
        'w:val="0" toggles on every run': (document, b'<w:b w:val="0"/>'),
        "empty <w:pPr/> on plain paragraphs": (document, b'<w:p><w:pPr/><w:r>'),
        'pageBreakBefore/keepNext w:val="0"': (document, b'<w:pageBreakBefore w:val="0"/>'),
        'w:orient="portrait" on pgSz': (document, b'<w:pgSz w:orient="portrait"'),
        "default namespace on the root": (document, b'xmlns="http://schemas.microsoft.com/office/tasks/2019/documenttasks"'),
        "packed TOC field run": (document, b'fldCharType="begin"/><w:instrText'),
        "packed footer fields, no cached result": (footer, b'fldCharType="begin"/><w:instrText'),
        "inline content control on first runs": (document, b'goog_rdk_0'),
        "empty comments part": (_matrix_part(path, "word/comments.xml") if empty_comments else b"", b'<w:comments'),
    }
    if trait in signatures:
        payload, marker = signatures[trait]
        assert marker in payload, trait


def _matrix_png():
    def chunk(name, data):
        return struct.pack(">I", len(data)) + name + data + struct.pack(">I", zlib.crc32(name + data))

    return (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", struct.pack(">IIBBBBB", 1, 1, 8, 2, 0, 0, 0))
        + chunk(b"IDAT", zlib.compress(b"\x00\xff\x00\x00"))
        + chunk(b"IEND", b"")
    )


@pytest.mark.parametrize("trait", _MATRIX_PRODUCER_ROWS)
def test_issue_160_producer_matrix_across_operations_and_picture(tmp_path, trait):
    _assert_oracle_version()
    import rdocx

    assert len(_MATRIX_PRODUCER_ROWS) == 11
    assert len(_MATRIX_PRODUCER_ROWS) * 8 == 88

    source = _matrix_rewrite(_matrix_fixture(tmp_path / "source.docx"), trait)
    edited = _matrix_rewrite(_matrix_fixture(tmp_path / "edited.docx", word="ALPHA"), trait)
    document = rdocx.Document(source)
    noop = tmp_path / "noop.docx"
    document.save(noop)
    assert _matrix_part(source, "word/document.xml") == _matrix_part(noop, "word/document.xml")

    document = rdocx.Document(source)
    assert document.try_replace_text("alpha", "ALPHA") == 1
    replaced = tmp_path / "replaced.docx"
    document.save(replaced)
    assert rdocx.Document(source).rebuild_toc().entry_count == 6
    fields = rdocx.Document(source)
    fields.update_page_fields()
    refreshed = tmp_path / "refreshed.docx"
    fields.save(refreshed)
    footer = _matrix_part(refreshed, "word/footer1.xml").decode()
    assert _matrix_part(refreshed, "word/footer1.xml") == _matrix_part(source, "word/footer1.xml")
    expected_caches = 0 if trait == "packed footer fields, no cached result" else 2
    assert len(re.findall(r'fldCharType="separate"/>(?:</w:r><w:r>)?<w:t>([^<]*)</w:t>', footer)) == expected_caches
    assert rdocx.Document(source).to_pdf().startswith(b"%PDF")
    assert isinstance(_matrix_comparison_count(source, refreshed), int)
    assert _matrix_comparison_count(source, edited) == 2
    assert _matrix_comparison_count(source, replaced) == 2

    pictured = rdocx.Document(source)
    pictured.add_picture(_matrix_png(), "matrix.png", width=914400, height=914400)
    pictured_path = tmp_path / "pictured.docx"
    pictured.save(pictured_path)
    reopened = rdocx.Document(pictured_path)
    assert "word/media/image1.png" in zipfile.ZipFile(pictured_path).namelist()
    _matrix_assert_trait(pictured_path, trait)
    assert reopened.try_replace_text("alpha", "ALPHA") == 1
    assert reopened.to_pdf().startswith(b"%PDF")


_ISSUE158_REPORT_SHA256 = "d05f9c753c00eb804c6e345126ef7a1f7a4fc635d2c9b653b829922030cd875e"
_ISSUE158_REPORT_URL = "https://github.com/user-attachments/files/32701104/fixture-report.docx"


def _issue158_report(tmp_path):
    source = os.environ.get("RDOCX_ISSUE158_REPORT")
    if source:
        data = Path(source).read_bytes()
    else:
        with urllib.request.urlopen(_ISSUE158_REPORT_URL, timeout=30) as response:
            data = response.read()
    assert hashlib.sha256(data).hexdigest() == _ISSUE158_REPORT_SHA256
    path = tmp_path / "fixture-report.docx"
    path.write_bytes(data)
    return path


def test_issue_158_word_fixture_acceptance(tmp_path):
    _assert_oracle_version()
    import rdocx
    from docx import Document as OracleDocument

    assert len(_MATRIX_IDENTITY_ROWS) == 18
    assert len(_MATRIX_PRODUCER_ROWS) == 11
    assert (len(_MATRIX_IDENTITY_ROWS) * 7, len(_MATRIX_PRODUCER_ROWS) * 8) == (126, 88)
    report = _issue158_report(tmp_path)
    oracle = OracleDocument(report)
    assert len(oracle.paragraphs) == 90
    assert [(len(table.rows), len(table.columns)) for table in oracle.tables] == [
        (57, 6), (8, 5), (6, 2)
    ]
    document = rdocx.Document(report)
    assert [paragraph.text for paragraph in document.paragraphs[:2]] == [
        "Riverton Footbridge",
        "Principal inspection and condition survey, 2025",
    ]
    assert [len(table.rows) for table in document.tables] == [57, 8, 6]


def test_issue_158_complete_word_workflow(tmp_path):
    _assert_oracle_version()
    import rdocx

    source = _issue158_report(tmp_path)
    assert rdocx.Document(source).rebuild_toc().entry_count == 21
    document = rdocx.Document(source)
    word = "described"
    assert sum(paragraph.text.count(word) for paragraph in document.paragraphs) == 1
    probe = rdocx.Document.from_bytes(document.to_bytes())
    assert probe.try_replace_text(word, word.upper()) == 1
    assert document.try_replace_text(word, word.upper()) == 1

    xml = _matrix_part(source, "word/document.xml").decode()
    controls = re.findall(
        r'<w:sdt>(?:(?!</w:sdtPr>).)*?<w:tag w:val="([^"]+)"/>.*?</w:sdtPr><w:sdtContent>(.*?)</w:sdtContent>',
        xml,
    )
    assert [tag for tag, _ in controls] == ["goog_rdk_0", "goog_rdk_1"]
    for _, inner in controls:
        text = "".join(re.findall(r"<w:t(?: [^>]*)?>([^<]*)</w:t>", inner))
        probe = rdocx.Document.from_bytes(document.to_bytes())
        assert probe.try_replace_text(text.split(".")[0], "X") == 1

    index = next(i for i, paragraph in enumerate(document.paragraphs) if word.upper() in paragraph.text)
    body_index = document.find_content_index(document.paragraphs[index])
    document.clone_content(document.paragraphs[index], body_index + 1)
    for i, run in enumerate(document.paragraphs[index + 1].runs):
        run.text = "Inserted paragraph." if i == 0 else ""
    assert document.paragraphs[index + 1].text == "Inserted paragraph."
    table = document.tables[0]
    row_count = len(table.rows)
    table.clone_row(row_count - 1)
    for column in range(len(document.tables[0].rows[row_count].cells)):
        document.tables[0].rows[row_count].cells[column].text = "cloned"
    document.tables[0].remove_row(row_count)
    assert len(document.tables[0].rows) == row_count == 57

    first_table = document.find_content_index(document.tables[0])
    def direct_body_index(paragraph):
        try:
            return document.find_content_index(paragraph)
        except ValueError:
            return -1

    target_index = next(
        index for index, paragraph in enumerate(document.paragraphs)
        if len(paragraph.runs) == 1 and len(paragraph.text) > 60
        and direct_body_index(paragraph) > first_table
    )
    target = document.paragraphs[target_index]
    text = target.text
    body_index = document.find_content_index(target)
    document.split_run(body_index, 0, 30)
    document.split_run(body_index, 0, 10)
    assert document.paragraphs[target_index].runs[1].text == text[10:30]
    bounds = rdocx.RunRange(
        start=rdocx.RunPosition(body_index=body_index, run_index=1),
        end=rdocx.RunPosition(body_index=body_index, run_index=2),
    )
    comment_id = document.add_comment(bounds, author="Reviewer", text="Comment on a piece of text.", initials="R")
    document.reply_to(comment_id, author="Reviewer", text="Reply.")
    document.resolve_comment(comment_id)
    assert document.paragraphs[target_index].text == text

    drawing = next(
        item.xml.decode() if isinstance(item.xml, bytes) else item.xml
        for item in document.story_items
        if item.xml and "r:embed" in (item.xml.decode() if isinstance(item.xml, bytes) else item.xml)
    )
    image_id = re.search(r'r:embed="([^"]+)"', drawing).group(1)
    document.replace_image(image_id, _matrix_png())
    assert document.image_data(image_id) == _matrix_png()
    assert document.update_layout_backed_fields().updated_count == 0
    assert document.rebuild_toc().entry_count == 21
    edited = tmp_path / "edited.docx"
    document.save(edited)
    assert zipfile.ZipFile(edited).namelist()[0] == "[Content_Types].xml"

    redline = tmp_path / "redline.docx"
    cli = os.environ.get("RDOCX_CLI")
    command = [cli] if cli else [
        "cargo", "run", "--quiet", "--locked", "-p", "rdocx-cli", "--"
    ]
    result = subprocess.run(
        command + ["compare", str(source), str(edited), "--author", "Reviewer",
                   "--timestamp", _MATRIX_TIMESTAMP, "-o", str(redline)],
        cwd=Path(__file__).resolve().parents[3], capture_output=True, text=True,
    )
    assert result.returncode == 0, result.stdout + result.stderr
    assert len(rdocx.Document(redline).revisions) > 0
    assert rdocx.Document(edited).to_pdf().startswith(b"%PDF")


def test_issue_161_rebuilt_toc_compares_and_resolves_both_sides(tmp_path):
    _assert_oracle_version()
    import rdocx
    from docx import Document as OracleDocument
    from docx.oxml import OxmlElement

    source = _matrix_fixture(tmp_path / "source.docx")
    oracle = OracleDocument(source)
    body = oracle.element.body
    control = OxmlElement("w:sdt")
    control.append(OxmlElement("w:sdtPr"))
    content = OxmlElement("w:sdtContent")
    for paragraph in list(body)[:6]:
        body.remove(paragraph)
        content.append(paragraph)
    control.append(content)
    body.insert(0, control)
    oracle.save(source)
    edited = rdocx.Document(source)
    assert edited.rebuild_toc().entry_count == 6
    edited_path = tmp_path / "edited.docx"
    edited.save(edited_path)
    redline = rdocx.Document(source)
    assert redline.compare(
        rdocx.Document(edited_path), "Ada", _MATRIX_TIMESTAMP, granularity="word"
    ) == ()
    accepted = rdocx.Document.from_bytes(redline.to_bytes())
    accepted.accept_all()
    assert accepted.compare(rdocx.Document(edited_path), "Ada", _MATRIX_TIMESTAMP) == ()
    rejected = rdocx.Document.from_bytes(redline.to_bytes())
    rejected.reject_all()
    assert rejected.compare(rdocx.Document(source), "Ada", _MATRIX_TIMESTAMP) == ()


def test_issue_161_toc_entry_hyperlink_transition_tracks_boundary(tmp_path):
    _assert_oracle_version()
    import rdocx
    from docx import Document as OracleDocument
    from docx.oxml import OxmlElement
    from docx.oxml.ns import qn

    original = OracleDocument()
    original.add_paragraph("Chapter 1")
    original.add_paragraph("Following body paragraph")
    original_path = tmp_path / "toc_entry_original.docx"
    original.save(original_path)
    edited = OracleDocument(original_path)
    paragraph = edited.paragraphs[0]._p
    for child in list(paragraph):
        paragraph.remove(child)
    hyperlink = OxmlElement("w:hyperlink")
    hyperlink.set(qn("w:anchor"), "_Toc1")
    run = OxmlElement("w:r")
    text = OxmlElement("w:t")
    text.text = "Chapter 1"
    run.append(text)
    hyperlink.append(run)
    paragraph.append(hyperlink)
    edited_path = tmp_path / "toc_entry_edited.docx"
    edited.save(edited_path)

    redline = rdocx.Document(original_path)
    assert redline.compare(rdocx.Document(edited_path), "Ada", _MATRIX_TIMESTAMP) == ()
    accepted = rdocx.Document.from_bytes(redline.to_bytes())
    accepted.accept_all()
    assert accepted.compare(rdocx.Document(edited_path), "Ada", _MATRIX_TIMESTAMP) == ()
    rejected = rdocx.Document.from_bytes(redline.to_bytes())
    rejected.reject_all()
    assert rejected.compare(rdocx.Document(original_path), "Ada", _MATRIX_TIMESTAMP) == ()


# Exact source: https://github.com/tensorbee/rdocx/issues/382
# SHA-256: 6383e57d2986af0a16229fe3c96d896fc52b7c2e880392519e2cee3121f44d30
FX245_ISSUE382_SCRIPT = r'''import os
import re
import tempfile
import zipfile

import rdocx

TMP = tempfile.mkdtemp()


def build(name, north_q2, closing="End of report."):
    doc = rdocx.Document()
    doc.add_paragraph("Quarterly volume by region.")
    data = rdocx.CategoryChartData()
    data.categories = ["Q1", "Q2", "Q3"]
    data.add_series("North", [12, north_q2, 15])
    data.add_series("South", [9, 11, 10])
    data.add_series("West", [7, 8, 13])
    doc.add_chart(rdocx.XL_CHART_TYPE.COLUMN_CLUSTERED, data, rdocx.Inches(6), rdocx.Inches(2.6))
    doc.add_paragraph(closing)
    path = os.path.join(TMP, name)
    doc.save(path)
    return path


def north(path):
    """The cached values of the first series of the chart part."""
    with zipfile.ZipFile(path) as z:
        xml = z.read("word/charts/chart1.xml").decode()
    return re.findall(r"<c:v>([^<]*)</c:v>", re.search(r"<c:val>.*?</c:val>", xml, re.S).group(0))


def check(case, a, b, granularity="word"):
    """Expected: compare succeeds and records the change; accept_all() gives B's chart values and text,
    reject_all() A's."""
    doc = rdocx.Document(a)
    try:
        doc.compare(rdocx.Document(b), "Reviewer", "2026-01-01T00:00:00Z", granularity=granularity)
    except Exception as e:
        print("[FAIL] %s (%s)\n       %s: %s" % (case, granularity, type(e).__name__, e))
        return
    out = os.path.join(TMP, "redline.docx")
    doc.save(out)
    acc, rej = rdocx.Document(out), rdocx.Document(out)
    acc.accept_all()
    rej.reject_all()
    acc.save(os.path.join(TMP, "accepted.docx"))
    rej.save(os.path.join(TMP, "rejected.docx"))
    got_acc, got_rej = north(os.path.join(TMP, "accepted.docx")), north(os.path.join(TMP, "rejected.docx"))
    ok = (got_acc == north(b) and got_rej == north(a)
          and acc.text() == rdocx.Document(b).text() and rej.text() == rdocx.Document(a).text())
    print("[%s] %s (%s)\n       %d revisions; North after accept_all %s (B %s), after reject_all %s (A %s)"
          % (" OK " if ok else "FAIL", case, granularity, len(doc.revisions), got_acc, north(b), got_rej, north(a)))


a = build("a.docx", 14)
check("1a one chart value changed (North, Q2: 14 -> 18)", a, build("b1.docx", 18))
check("1a one chart value changed (North, Q2: 14 -> 18)", a, build("b1.docx", 18), granularity="run")
check("1b the same value and the next paragraph changed", a, build("b2.docx", 18, "End of the report."))
check("1c control: only the next paragraph changed", a, build("b3.docx", 14, "End of the report."))
'''


def test_fx245_exact_issue382_script_and_independent_package_graph(capsys):
    import xml.etree.ElementTree as ET
    import posixpath

    scope = {}
    exec(compile(FX245_ISSUE382_SCRIPT, "issue382.py", "exec"), scope)
    output = capsys.readouterr().out
    # The exact reporter observer hardcodes chart1.xml, an unused original after
    # collision-safe replacement. Retain its execution, then inspect the actual
    # surviving drawing relationship without changing any expected values.
    assert output.count(" revisions;") == 4, output
    assert "RdocxError" not in output, output

    def graph(path):
        with zipfile.ZipFile(path) as package:
            chart_ns = "{http://schemas.openxmlformats.org/drawingml/2006/chart}"
            office_ns = "{http://schemas.openxmlformats.org/officeDocument/2006/relationships}"
            drawing = ET.fromstring(package.read("word/document.xml")).find(".//" + chart_ns + "chart")
            edge_id = drawing.get(office_ns + "id")
            edges = ET.fromstring(package.read("word/_rels/document.xml.rels"))
            edge = next(edge for edge in edges if edge.get("Id") == edge_id)
            assert edge.get("Type").endswith("/chart")
            chart_name = posixpath.normpath(posixpath.join("word", edge.get("Target"))).lstrip("/")
            chart = package.read(chart_name)
            rels_name = posixpath.join(posixpath.dirname(chart_name), "_rels", posixpath.basename(chart_name) + ".rels")
            edges = ET.fromstring(package.read(rels_name))
            external = ET.fromstring(chart).find(chart_ns + "externalData")
            workbook_edge = next(edge for edge in edges if edge.get("Id") == external.get(office_ns + "id"))
            assert workbook_edge.get("Type").endswith("/package")
            workbook_name = posixpath.normpath(posixpath.join(posixpath.dirname(chart_name), workbook_edge.get("Target"))).lstrip("/")
            workbook = package.read(workbook_name)
            with zipfile.ZipFile(__import__("io").BytesIO(workbook)) as book:
                cells = ET.fromstring(book.read("xl/worksheets/sheet1.xml"))
                values = {cell.get("r"): "".join(cell.itertext()) for cell in cells.iter() if cell.tag.endswith("}c")}
                assert "B3" in values
            return chart, workbook, values

    def north_from_surviving_relationship(path):
        chart = ET.fromstring(graph(path)[0])
        ns = "{http://schemas.openxmlformats.org/drawingml/2006/chart}"
        return [value.text for value in chart.find(".//" + ns + "val").iter(ns + "v")]

    scope["north"] = north_from_surviving_relationship
    for value, closing, granularity in ((18, "End of report.", "word"), (18, "End of report.", "run"),
                                        (18, "End of the report.", "word"), (14, "End of the report.", "word")):
        edited = scope["build"]("independent-b.docx", value, closing)
        scope["check"]("independent graph", scope["a"], edited, granularity)
        output = capsys.readouterr().out
        assert "[FAIL]" not in output, output
        assert graph(os.path.join(scope["TMP"], "accepted.docx")) == graph(edited)
        assert graph(os.path.join(scope["TMP"], "rejected.docx")) == graph(scope["a"])


@pytest.mark.parametrize("initial", [None, 127000])
@pytest.mark.parametrize("refused", [12, 6349, -1, -6350, True, False])
def test_fx239_word_font_quantization_refuses_before_publication(initial, refused):
    import rdocx
    document = rdocx.Document()
    held = document.add_paragraph("kept")
    run = held.add_run("size")
    font = run.font
    font.size = initial
    before = document.to_bytes()
    with pytest.raises((TypeError, ValueError), match="EMU|Pt|bool"):
        font.size = refused
    assert document.to_bytes() == before and held.text == "keptsize"
    assert font.size == initial
    font.size = rdocx.Pt(12)
    assert font.size == rdocx.Pt(12)


def test_fx239_word_font_storage_boundaries_and_clear():
    import rdocx
    from xml.etree import ElementTree as ET
    import io
    document = rdocx.Document()
    font = document.add_paragraph("").add_run("size").font
    font.size = 6350
    with zipfile.ZipFile(io.BytesIO(document.to_bytes())) as package:
        root = ET.fromstring(package.read("word/document.xml"))
    w = "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}"
    assert next(root.iter(w + "sz")).get(w + "val") == "1"
    font.size = 0
    assert font.size == 0
    reopened = rdocx.Document.from_bytes(document.to_bytes())
    assert reopened.paragraphs[0].runs[0].font.size == 0
    font.size = None
    assert font.size is None


def test_fx239_valid_font_boundary_matches_pinned_python_docx():
    import docx
    import rdocx
    import io
    from xml.etree import ElementTree as ET
    assert importlib.metadata.version("python-docx") == "1.2.0"
    w = "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}"
    values = []
    for module in [rdocx, docx]:
        document = module.Document()
        paragraph = document.add_paragraph("")
        shared = __import__(module.__name__ + ".shared", fromlist=["Pt"])
        paragraph.add_run("boundary").font.size = shared.Pt(0.5)
        raw = document.to_bytes() if module is rdocx else None
        if raw is None:
            stream = io.BytesIO()
            document.save(stream)
            raw = stream.getvalue()
        with zipfile.ZipFile(io.BytesIO(raw)) as package:
            root = ET.fromstring(package.read("word/document.xml"))
        values.append([node.get(w + "val") for node in root.iter(w + "sz")])
    assert values == [["1"], ["1"]]


def test_fx238_word_names_create_on_one_prepared_candidate():
    import rdocx
    document = rdocx.Document()
    assert document.add_paragraph().text == ""
    heading = document.add_heading("Heading", 1)
    assert heading.style == "Heading1"
    paragraph = document.add_paragraph("Styled", style="Normal")
    run = paragraph.add_run(" emphasized")
    run.bold, run.italic, run.underline = True, False, True
    assert (run.font.bold, run.font.italic, run.font.underline) == (True, False, True)
    run.bold, run.italic, run.underline = None, None, None
    assert (run.bold, run.italic, run.underline) == (None, None, None)
    table = document.add_table(1, 1, style="Table Grid")
    assert table.style == "TableGrid"
    document.add_page_break()
    reopened = rdocx.Document.from_bytes(document.to_bytes())
    with zipfile.ZipFile(__import__("io").BytesIO(reopened.to_bytes())) as package:
        from xml.etree import ElementTree as ET
        root = ET.fromstring(package.read("word/document.xml"))
    w = "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}"
    assert any(node.get(w + "type") == "page" for node in root.iter(w + "br"))


def test_fx238_stream_input_and_output_are_both_qualified(tmp_path):
    import io
    import rdocx
    original = rdocx.Document()
    original.add_paragraph("original")
    path = tmp_path / "input.docx"
    path.write_bytes(original.to_bytes())
    with path.open("rb") as stream:
        document = rdocx.Document(stream)
        assert not stream.closed
    document.add_paragraph("stream roundtrip")
    stream = io.BytesIO()
    document.save(stream)
    assert not stream.closed
    assert "stream roundtrip" in rdocx.Document.open(io.BytesIO(stream.getvalue())).text()
    before = document.to_bytes()
    held = document.paragraphs[0]

    class ShortWriter:
        def write(self, value):
            return len(value) - 1

    with pytest.raises((ValueError, OSError), match="write|short|bytes"):
        document.save(ShortWriter())
    with pytest.raises((TypeError, ValueError), match="binary|bytes|from_bytes"):
        rdocx.Document(io.StringIO("not binary"))
    class ReadFailure:
        def seek(self, offset):
            raise OSError("seek-failed-control")

        def read(self):
            raise OSError("read-failed-control")

    with pytest.raises(OSError, match="seek-failed-control"):
        rdocx.Document(ReadFailure())
    assert document.to_bytes() == before and held.text == "original"


def test_fx238_styles_are_an_immutable_tuple_with_an_actionable_pointer():
    import rdocx
    document = rdocx.Document()
    styles = document.styles
    assert isinstance(styles, tuple) and type(styles) is not tuple
    assert styles[0] == tuple(styles)[0]
    assert styles[-1] == tuple(styles)[-1]
    assert type(styles[:1]) is tuple
    assert styles == tuple(styles) and len(styles) == len(tuple(styles))
    with pytest.raises(NotImplementedError, match="set_style"):
        styles["Normal"]
    old_names = tuple(style.name for style in styles)
    document.add_style("Later snapshot control", font_name="Arial")
    assert tuple(style.name for style in styles) == old_names
    assert len(document.styles) == len(styles) + 1
    assert "Later snapshot control" not in old_names
    with pytest.raises((AttributeError, TypeError)):
        styles[0].name = "changed"


def test_fx238_section_module_matches_pinned_enum_values():
    from enum import IntEnum
    import rdocx.enum.section as section
    import rdocx.enum.text as text
    from docx.enum.section import WD_SECTION as oracle_section, WD_ORIENT as oracle_orientation

    assert importlib.metadata.version(ORACLE_DISTRIBUTION) == ORACLE_VERSION
    assert section is not text
    assert section.WD_SECTION is section.WD_SECTION_START
    assert section.WD_ORIENT is section.WD_ORIENTATION
    assert issubclass(section.WD_SECTION, IntEnum)
    assert issubclass(section.WD_ORIENT, IntEnum)
    assert {item.name: int(item) for item in section.WD_SECTION} == {
        item.name: int(item) for item in oracle_section
    } == {"CONTINUOUS": 0, "NEW_COLUMN": 1, "NEW_PAGE": 2, "EVEN_PAGE": 3, "ODD_PAGE": 4}
    assert {item.name: int(item) for item in section.WD_ORIENT} == {
        item.name: int(item) for item in oracle_orientation
    } == {"PORTRAIT": 0, "LANDSCAPE": 1}
    assert set(section.__all__) == {"WD_SECTION", "WD_SECTION_START", "WD_ORIENT", "WD_ORIENTATION"}
    assert not hasattr(text, "WD_SECTION") and not hasattr(text, "WD_ORIENT")


def test_fx238_highlight_and_first_page_names_are_not_import_only():
    import io
    import rdocx
    from rdocx.enum.text import WD_BREAK, WD_COLOR_INDEX
    from rdocx.enum.table import WD_ALIGN_VERTICAL, WD_CELL_VERTICAL_ALIGNMENT
    from rdocx.enum.section import WD_SECTION, WD_SECTION_START, WD_ORIENT, WD_ORIENTATION
    assert WD_SECTION is WD_SECTION_START and WD_ORIENT is WD_ORIENTATION
    assert WD_ALIGN_VERTICAL is WD_CELL_VERTICAL_ALIGNMENT
    assert int(WD_SECTION.NEW_PAGE) == 2 and int(WD_ORIENT.LANDSCAPE) == 1
    document = rdocx.Document()
    run = document.add_paragraph().add_run("highlight")
    run.font.highlight_color = WD_COLOR_INDEX.YELLOW
    run.add_break(WD_BREAK.PAGE)
    assert run.font.highlight_color == WD_COLOR_INDEX.YELLOW
    from xml.etree import ElementTree as ET
    with zipfile.ZipFile(io.BytesIO(document.to_bytes())) as package:
        root = ET.fromstring(package.read("word/document.xml"))
    w = "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}"
    assert next(root.iter(w + "highlight")).get(w + "val") == "yellow"
    for name in ("header", "footer", "first_page_header", "different_first_page_header_footer"):
        with pytest.raises(NotImplementedError, match="document.section"):
            getattr(document.sections[0], name)
    with pytest.raises(NotImplementedError, match="document.section"):
        document.sections[0].different_first_page_header_footer = True
    live = document.section(0)
    live.different_first_page_header_footer = True
    live.first_page_header.add_paragraph("cover")
    assert rdocx.Document.from_bytes(document.to_bytes()).section(0).different_first_page_header_footer


def test_fx238_no_new_native_semantics_refusals_name_alternatives():
    import rdocx
    document = rdocx.Document()
    paragraph = document.add_paragraph("keep")
    table = document.add_table(1, 1)
    for operation, pointer in ((lambda: document.add_section(), "insert_section"),
                               (lambda: table.add_column(rdocx.Inches(1)), "insert_column"),
                               (lambda: table.columns, "set_column_width"),
                               (lambda: setattr(paragraph, "style", "List Number"), "numbering")):
        before = document.to_bytes()
        with pytest.raises(Exception, match=pointer):
            operation()
        assert document.to_bytes() == before
    before = document.to_bytes()
    # Retain the established package exception while naming the supported call.
    with pytest.raises(rdocx.RdocxError, match="width=..., height=..."):
        document.add_picture(_matrix_png(), width=rdocx.Inches(1))
    assert document.to_bytes() == before and paragraph.text == "keep"
    with pytest.raises(NotImplementedError, match="add_internal_hyperlink"):
        paragraph.add_hyperlink("anchor", anchor=paragraph)
    assert document.to_bytes() == before and paragraph.text == "keep"
    with pytest.raises(NotImplementedError, match="remove_content"):
        paragraph._element
    with pytest.raises(NotImplementedError, match="set_style"):
        document.styles["Normal"]


def test_fx238_insertion_uses_physical_body_index_and_refuses_related_owners():
    import rdocx
    document = rdocx.Document()
    document.add_table(1, 1)
    paragraph = document.add_paragraph("last")
    inserted = paragraph.insert_paragraph_before("first", style="Normal")
    assert inserted.text == "first"
    assert [p.text for p in document.paragraphs] == ["first", "last"]
    cell = document.tables[0].cell(0, 0).paragraphs[0]
    before = document.to_bytes()
    with pytest.raises(NotImplementedError, match="direct body|insert_paragraph"):
        cell.insert_paragraph_before("unsupported")
    assert document.to_bytes() == before


def test_fx238_invalid_creation_operands_leave_source_and_owners_unchanged():
    import rdocx
    document = rdocx.Document()
    held = document.add_paragraph("kept")
    for operation in (lambda: document.add_paragraph("bad", style="unknown-fx238-style"),
                      lambda: document.add_heading("bad", True),
                      lambda: document.add_heading("bad", 10),
                      lambda: document.add_table(1, 1, style="unknown-fx238-style")):
        before = document.to_bytes()
        with pytest.raises((ValueError, TypeError, KeyError)):
            operation()
        assert document.to_bytes() == before and held.text == "kept"


def test_fx238_word_names_match_pinned_python_docx():
    import io
    import docx
    import rdocx
    from xml.etree import ElementTree as ET
    assert importlib.metadata.version("python-docx") == "1.2.0"
    w = "{http://schemas.openxmlformats.org/wordprocessingml/2006/main}"
    projections = []
    for module in (rdocx, docx):
        document = module.Document()
        document.add_heading("Heading", 1)
        paragraph = document.add_paragraph("Body", style="Normal")
        paragraph.add_run("strong").bold = True
        document.add_page_break()
        stream = io.BytesIO()
        document.save(stream)
        with zipfile.ZipFile(io.BytesIO(stream.getvalue())) as package:
            root = ET.fromstring(package.read("word/document.xml"))
        projections.append(([(p.find(w + "pPr/" + w + "pStyle").get(w + "val") if p.find(w + "pPr/" + w + "pStyle") is not None else "Normal") for p in root.iter(w + "p")],
                            [t.text for t in root.iter(w + "t") if t.text],
                            [n.get(w + "type") for n in root.iter(w + "br")]))
    assert projections[0] == projections[1]


# The issue's 32 specimens stay distinct from the immutable runner's needs marks.
FX238_WORD_REQUIREMENTS = {
    **dict.fromkeys(("exp_pdf", "fmt_headings", "tpl_append_to_template"), ("add_style", "set_style")),
    **dict.fromkeys(("list_heading_and_bullets", "fmt_paragraph_style_kwarg",
                     "list_bullet_style", "list_nested_bullets", "list_number_style",
                     "list_style_assignment"), ("add_numbering_definition",)),
    **dict.fromkeys(("fmt_font_color_rgb", "fmt_font_size_and_name",
                     "fmt_run_formatting_loop", "fmt_bold_italic_runs",
                     "tbl_cell_bold_centered", "doc_insert_paragraph_before",
                     "sec_page_break", "sec_run_add_break", "fmt_highlight_color",
                     "tbl_row_height_and_valign", "doc_open_and_save_stream",
                     "tbl_style_and_alignment"), ()),
    **dict.fromkeys(("hf_header_text", "hf_footer_text", "hf_footer_page_number",
                     "hf_different_first_page"), ("document.section",)),
    "link_internal_anchor": ("add_internal_hyperlink",),
    "sec_add_section": ("insert_section",),
    **dict.fromkeys(("pic_add_picture_width", "pic_centered_picture",
                     "pic_from_stream"), ("width=..., height=...",)),
    "tbl_add_column": ("insert_column",),
    "tbl_column_widths": ("set_column_width",),
}


def _fx238_contract_disposition(failure, alternatives):
    """A saved-file check failure never receives explicit-refusal credit."""
    if failure is None:
        return "correct-saved-file"
    first_line = failure.splitlines()[0]
    if first_line.startswith("script: ") and any(
        alternative in first_line for alternative in alternatives
    ):
        return "explicit-alternative"
    return "unresolved"


def test_fx238_immutable_first_draft_contract_classification():
    assert len(FX238_WORD_REQUIREMENTS) == 32
    classify = _fx238_contract_disposition
    assert classify(None, ("set_style",)) == "correct-saved-file"
    assert classify("script: NotImplementedError: use document.set_style(name)",
                    ("set_style",)) == "explicit-alternative"
    assert classify("script: AttributeError: missing style", ("set_style",)) == "unresolved"
    assert classify("script: ValueError: bad value\nuse set_style", ("set_style",)) == "unresolved"
    assert classify("check: AssertionError: wrong saved set_style value",
                    ("set_style",)) == "unresolved"


def test_fx238_created_owners_follow_current_form_and_signing_publication():
    import rdocx
    from test_core import _fx240_material, _fx241_five_kind_document

    document = _fx241_five_kind_document()
    paragraph = document.add_paragraph("held alias paragraph", style="Normal")
    run = paragraph.add_run(" held run")
    table = document.add_table(1, 1, style="Table Grid")
    cell = table.cell(0, 0)
    cell.text = "held cell"
    section = document.section(0)
    section_width = section.page_width
    run = paragraph.runs[-1]
    assert run.text == " held run"
    assert document.bind_content_controls({"plain": "Ada"}) == 1
    assert paragraph.text == "held alias paragraph held run"
    assert cell.text == "held cell" and len(table.rows) == 1
    assert section.page_width == section_width
    with pytest.raises(rdocx.StaleElementError, match=r"Document\.bind_content_controls"):
        _ = run.text
    document.set_legacy_form_field_value("legacy", "filled")
    assert paragraph.text == "held alias paragraph held run" and cell.text == "held cell"
    fresh_run = paragraph.runs[0]
    key, certificate = _fx240_material()
    document.sign(key, certificate)
    assert fresh_run.text == "held alias paragraph"
    assert cell.text == "held cell" and section.page_width == section_width
    before = document.to_bytes()
    assert document.bind_content_controls({"plain": "Ada"}) == 1
    assert document.to_bytes() == before and fresh_run.text == "held alias paragraph"


def test_fx238_stream_save_preserves_signed_forms_and_refusal_authority():
    import io
    import rdocx
    from test_core import _fx240_material, _fx241_five_kind_document

    document = _fx241_five_kind_document()
    document.bind_content_controls({"plain": "Ada", "check": "true", "choice": "two", "date": "2026-10-16"})
    document.set_legacy_form_field_value("legacy", "filled")
    paragraph = document.add_paragraph("held stream owner", style="Normal")
    key, certificate = _fx240_material()
    document.sign(key, certificate)
    before = document.to_bytes()
    buffer = io.BytesIO()
    document.save(buffer)
    assert not buffer.closed and buffer.getvalue() == before
    reopened = rdocx.Document.open(io.BytesIO(buffer.getvalue()))
    report, = reopened.verify_signatures()
    assert report["cryptographically_valid"] and report["coverage_complete"]
    assert {control.tag: control.text for control in reopened.content_controls()}["plain"] == "Ada"
    assert reopened.legacy_form_fields()[0].value == "filled"

    class ShortWriter(io.BytesIO):
        def write(self, data):
            super().write(data)
            return len(data) - 1

    refused = ShortWriter()
    with pytest.raises(OSError, match="short or invalid"):
        document.save(refused)
    assert not refused.closed and document.to_bytes() == before
    assert paragraph.text == "held stream owner"
    assert document.verify_signatures()[0]["cryptographically_valid"]
