import hashlib
import sys

import rdocx
import rdocx._rdocx as ext

path = ext.__file__
digest = hashlib.sha256(open(path, "rb").read()).hexdigest()
print("python", sys.version)
print("importpath", path)
print("sha256", digest)


def report_held(paragraph, run):
    try:
        print("held_paragraph_text", repr(paragraph.text))
    except BaseException as exc:
        print("held_paragraph_exc", type(exc).__name__, str(exc))
    try:
        print("held_run_text", repr(run.text))
    except BaseException as exc:
        print("held_run_exc", type(exc).__name__, str(exc))


def one(name, mutate):
    document = rdocx.Document()
    document.add_paragraph("held")
    paragraph = document.paragraphs[0]
    run = paragraph.runs[0]
    before = document.to_bytes()
    print("---", name)
    try:
        mutate(document, paragraph, run)
        print("mutate_ok")
    except BaseException as exc:
        print("mutate_exc", type(exc).__name__, str(exc))
    try:
        after = document.to_bytes()
        print("serializes", True)
        print("byte_identical", before == after)
    except BaseException as exc:
        print("to_bytes_exc", type(exc).__name__, str(exc))
    report_held(paragraph, run)


def add_paragraph(document, paragraph, run):
    document.add_paragraph("bad" + chr(1))


def insert_paragraph(document, paragraph, run):
    document.insert_paragraph(0, "bad" + chr(1))


def run_text(document, paragraph, run):
    setattr(run, "text", "bad" + chr(1))


one("add_paragraph", add_paragraph)
one("insert_paragraph", insert_paragraph)
one("run_text", run_text)

print("--- valid_control")
try:
    fresh = rdocx.Document()
    fresh.add_paragraph("ordinary")
    fresh.insert_paragraph(0, "ordinary-insert")
    fresh.paragraphs[1].runs[0].text = "ordinary-run"
    payload = fresh.to_bytes()
    print("serializes", True)
    print("nbytes", len(payload))
    print("texts", [item.text for item in fresh.paragraphs])
except BaseException as exc:
    print("exc", type(exc).__name__, str(exc))
