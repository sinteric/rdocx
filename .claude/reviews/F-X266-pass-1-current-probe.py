import hashlib
import io
import sys
import zipfile

import rdocx
import rdocx._rdocx as ext

EXPECT = "a89567b2475960789967adf4ba03219ff17fe0c5314f78811e3fa72b41aa02fd"
path = ext.__file__
digest = hashlib.sha256(open(path, "rb").read()).hexdigest()
print("python", sys.version)
print("executable", sys.executable)
print("importpath", path)
print("sha256", digest)
if digest != EXPECT:
    print("AUTH_FAIL")
    raise SystemExit(2)

whole = getattr(ext, "_word_whole_body_work", None)
splice = getattr(ext, "_word_splice_work", None)
print("has_whole", whole is not None)
print("has_splice", splice is not None)


def counters(tag):
    if whole is None or splice is None:
        print(tag, "counters_absent")
        return
    try:
        print(tag, "whole", whole(), "splice", splice())
    except BaseException as exc:
        print(tag, "counter_exc", type(exc).__name__, str(exc))


def reset():
    counters("reset_read")


def one(name, capture, mutate):
    document = rdocx.Document()
    document.add_paragraph("held")
    paragraph = None
    run = None
    if capture:
        paragraph = document.paragraphs[0]
        run = paragraph.runs[0]
    before = document.to_bytes()
    print("---", name)
    reset()
    try:
        mutate(document)
        print("mutate_ok")
    except BaseException as exc:
        print("mutate_exc", type(exc).__name__, str(exc))
    counters("after_mutate")
    try:
        after = document.to_bytes()
        print("serializes", True)
        print("byte_identical", before == after)
    except BaseException as exc:
        print("to_bytes_exc", type(exc).__name__, str(exc))
    counters("after_to_bytes")
    if paragraph is not None:
        try:
            print("held_paragraph_text", repr(paragraph.text))
        except BaseException as exc:
            print("held_paragraph_exc", type(exc).__name__, str(exc))
    if run is not None:
        try:
            print("held_run_text", repr(run.text))
        except BaseException as exc:
            print("held_run_exc", type(exc).__name__, str(exc))
    counters("after_held_read")


one("add_no_handle", False, lambda document: document.add_paragraph("bad" + chr(1)))
one("add_with_handle", True, lambda document: document.add_paragraph("bad" + chr(1)))
one("insert_no_handle", False, lambda document: document.insert_paragraph(0, "bad" + chr(1)))
one("insert_with_handle", True, lambda document: document.insert_paragraph(0, "bad" + chr(1)))

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


def plant(raw, payload):
    out = io.BytesIO()
    with zipfile.ZipFile(io.BytesIO(raw), "r") as source, zipfile.ZipFile(out, "w") as dest:
        for item in source.infolist():
            dest.writestr(item, source.read(item.filename))
        dest.writestr("word/unrelated.xml", payload)
    return out.getvalue()


def unrelated(label, payload):
    print("---", label)
    try:
        base = rdocx.Document()
        base.add_paragraph("held")
        loaded = rdocx.Document.from_bytes(plant(base.to_bytes(), payload))
        print("load_ok")
    except BaseException as exc:
        print("load_exc", type(exc).__name__, str(exc))
        return
    try:
        again = loaded.to_bytes()
        print("to_bytes_ok", len(again))
    except BaseException as exc:
        print("to_bytes_exc", type(exc).__name__, str(exc))
    try:
        fragment = loaded.paragraphs[0].xml
        print("paragraph_xml_ok", len(fragment))
    except BaseException as exc:
        print("paragraph_xml_exc", type(exc).__name__, str(exc))


unrelated("unrelated_u0001", b"<extra>" + bytes([1]) + b"</extra>")
unrelated("unrelated_valid", b"<extra>ok</extra>")
