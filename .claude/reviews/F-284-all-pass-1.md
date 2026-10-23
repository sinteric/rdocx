# F-284, all, pass 1

**Reviewed**: Working diff against 7843f28a67e1277d2c3e24aa18dc57a6c3c17b6c, 15 files, 4608 insertions and 987 deletions. The untracked `rdocx` abi3 extension in the Python package tree is a local install artifact and is not part of this diff.
**Verdict**: 0 defects, 0 smells, 1 nitpick

## Defects

0 found.

## Smells

0 found.

## Nitpicks

- `crates/rdocx-py/src/document.rs:11112`, every `invalid template:` failure becomes `ValueError`, including an internal staging failure that uses that prefix. The document is still left unpublished.

## Not found

Correctness, contract, panics, OOXML, tests and structure were checked.

Grammar 1 stays opt-in. `render_template` remains the legacy entry. `render_template_versioned` admits only version 1. `render_template_with_formatter` selects grammar 1 and keeps the existing mail-merge callback shape. Python `grammar_version` accepts `None` or `1`. Other versions, bad types and malformed data refuse before `commit_word_revision`. No formatter, WASM or CLI surface was added.

Repeated expansion reuses `import_content_staged`. The new note-copy vector is empty for glossary and rich-field callers. Source-owned numbering is off on those callers. Unpublished repeated drawing ids are put back into `identifiers.drawing_ids` after `prepare_staged_package`, because that prepare replaces occupancy from the live package. `versioned_template_repeated_high_drawing_id_stays_unique` keeps an outside picture at docPr 5, repeats an inside picture twice, and observes three distinct docPr ids. `versioned_template_whole_range_retains_note_comment_bookmark_and_media_closure` still requires distinct notes, comments, bookmarks, drawings and image relationships. The owner-byte check compares a flushed owner with the canonical unexpanded snapshot. It does not write old ids back.

`replay_program` returns early when the current owner has no items, so the later nonempty `expect` is on that same owner after a successful block replay. Nested fragment replay keeps `refresh_root` false. No new trait, generic, crate, module or feature flag appears. `NoteCopy` and `PartRewrite` name two repeated private tuples. HLD 03, 04, 10 and 12 describe the current grammar, the staged fragment transaction and the Python method. The final Word, LibreOffice, hash, pixel, WASM, package and full-suite checks are still pending and are not claimed here.

## Evidence

Observed on this working tree, after the drawing-id retention, the Python `ValueError` mapping and the high-id regression:

- `cargo fmt --all --check` passed.
- `cargo clippy -p rdocx -p rdocx-oxml --all-targets --all-features -- -D warnings` passed.
- `cargo clippy -p rdocx-py --all-targets --all-features -- -D warnings` passed.
- `cargo test -p rdocx --lib versioned_template` passed, 17 tests.
- `cargo test -p rdocx --lib template_same_nested_payload` passed, 1 test.
- `cargo test -p rdocx --lib repeated_table_level_controls` passed, 2 tests.
- Installed `maturin==1.13.3` develop wheel, then `pytest -k test_f284_` passed, 4 tests, on CPython 3.12.13.
- `python3 scripts/prose_check.py` reported 0 violations.

Hash harness, Word, pinned LibreOffice, WASM, docs, package dry run and the workspace suite were not run in this pass.
