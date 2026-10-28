import hashlib
import importlib.util
import io
import sys
import zipfile

SO = "/private/tmp/s95.4-fx265-verified-_rdocx.so"
EXPECT = "61bea238f1aa339a6360726c8cb55ffb9252fc7ed1c9f77f186fee20a0d9a86d"

print("python", sys.version)
print("executable", sys.executable)
digest = hashlib.sha256(open(SO, "rb").read()).hexdigest()
print("so", SO)
print("sha256", digest)
if digest != EXPECT:
    print("AUTH_FAIL")
    raise SystemExit(2)

spec = importlib.util.spec_from_file_location("rdocx._rdocx", SO)
module = importlib.util.module_from_spec(spec)
sys.modules["rdocx._rdocx"] = module
spec.loader.exec_module(module)
print("loaded", module.__file__)
print("build_note", "preserved F-X265 .so under Python", sys.version.split()[0], "direct native module, worker package not imported")

Document = module.Document


def held_report(paragraph, run):
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


def one(name, mutate, capture):
    document = Document()
    document.add_paragraph("held")
    paragraph = None
    run = None
    if capture:
        paragraph = document.paragraphs[0]
        run = paragraph.runs[0]
    before = document.to_bytes()
    print("---", name)
    try:
        mutate(document)
        print("mutate_ok")
    except BaseException as exc:
        print("mutate_exc", type(exc).__name__, str(exc))
    try:
        after = document.to_bytes()
        print("serializes", True)
        print("byte_identical", before == after)
    except BaseException as exc:
        print("to_bytes_exc", type(exc).__name__, str(exc))
    held_report(paragraph, run)


one("add_paragraph", lambda document: document.add_paragraph("bad" + chr(1)), True)
one("insert_paragraph", lambda document: document.insert_paragraph(0, "bad" + chr(1)), True)

print("--- run_text")
try:
    document = Document()
    document.add_paragraph("held")
    paragraph = document.paragraphs[0]
    run = paragraph.runs[0]
    before = document.to_bytes()
    try:
        setattr(run, "text", "bad" + chr(1))
        print("mutate_ok")
    except BaseException as exc:
        print("mutate_exc", type(exc).__name__, str(exc))
    try:
        after = document.to_bytes()
        print("serializes", True)
        print("byte_identical", before == after)
    except BaseException as exc:
        print("to_bytes_exc", type(exc).__name__, str(exc))
    held_report(paragraph, run)
except BaseException as exc:
    print("setup_exc", type(exc).__name__, str(exc))

print("--- valid_control")
try:
    fresh = Document()
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
        base = Document()
        base.add_paragraph("held")
        raw = base.to_bytes()
        mutated = plant(raw, payload)
        loaded = Document.from_bytes(mutated)
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
