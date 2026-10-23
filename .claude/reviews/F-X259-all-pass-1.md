# F-X259, all, pass 1

**Reviewed**: uncommitted working tree on `sprint/s95.3` after `527a3ca5`. 42 files, 773 insertions and 94 deletions. Product behaviour is `style_resolver.rs`, `field.rs`, `template.rs`, `document.rs`, `rpptx/src/lib.rs` and HLD 02, 03, 08 and 12. Process behaviour is the canonical commands, three skills, generated adapters, `sprint_workflow.py` and its tests.
**Verdict**: 0 defects, 0 smells, 1 nitpick

This pass is the feature microscope. It is not the requested Codex sprint review.

## Defects

None.

## Smells

None.

## Nitpicks

- `.claude/plans/F-X259-design.md:104`, the open-question sentence still calls the F-X260 through F-X262 plans draft after those plans were approved.

## Not found

Correctness produced no defect. `numbered_reference_text` applies the absent-caller default only when the instruction is STYLEREF (`crates/rdocx-layout/src/style_resolver.rs:92`). REF with `None` still enters `relative_to` without that default (`crates/rdocx-layout/src/style_resolver.rs:161`). The facade passes the caller's actual numbering (`crates/rdocx/src/field.rs:14375`). The positioned STYLEREF loop does the same (`crates/rdocx-layout/src/engine.rs:8214`). The bookmark path also passes actual caller numbering (`crates/rdocx-layout/src/engine.rs:11134` and `crates/rdocx/src/field.rs:14104`). There is no second fallback.

`deferred_expression` defers a named scope only when that scope is deferred, and defers an implicit key only when some scope is deferred and the key is absent from the root and from every populated scope (`crates/rdocx/src/template.rs:405`). A known root key still falls through to evaluation. The new refusal test covers `config.missing`, `wrong.child` and a malformed expression on an empty loop (`crates/rdocx/src/document.rs:39978`).

Contract produced no defect. A1 is the evaluator outcome plus positioned field text before update, after update and after reopen (`crates/rdocx/src/document.rs:39885`). A2 covers empty, one and many records, implicit and explicit paths, and both grammars, plus a nested empty loop (`crates/rdocx/src/document.rs:39924`). A3 checks unchanged bytes on refusal. A4 marks DOCX-050 and DOCX-051 complete with owner `-` and leaves the Word comparison in HLD 12 as a final gate (`docs/hld/02-scope-and-non-goals.md:317`, `docs/hld/12-testing-strategy.md:14`). The Python column for DOCX-051 follows the existing `render_template` pass-through. A5 names the frontier cases without changing the predicate grouping (`crates/rpptx/src/lib.rs:1762`). A6 and A7 are the command, skill and workflow diffs. The historical S95.11 evaluation was not rewritten.

Panics produced no finding. The new product paths return `Result` or a boolean. The formatter unit test uses `unwrap` on a constructed instruction (`crates/rdocx-layout/src/style_resolver.rs:2501`), which is test-only.

OOXML produced no finding. This diff does not add a writer, a prefix or a child sequence.

Tests produced no defect. The STYLEREF expectations distinguish current level, omitted text and full context, and the positioned walk is a separate assertion from the evaluator (`crates/rdocx/src/document.rs:39912`). Reverting the shared default would fail the `None` caller cases. Reverting implicit deferral would fail the empty `name` loop. The recorded RED and GREEN logs in `.claude/scratch/F-X259-progress.md` were consumed. They were not rerun. The final 49-hash, seven-pixel, installed, native, WASM and package gates remain pending.

Structure produced no finding. No new trait, generic, crate, module or file was added. The named frontier booleans are local bindings in the existing function.

## Final gate debt

Hash harness expected unchanged. Native Word comparison, installed suites, WASM and the sprint risk union stay pending. This pass does not tick the final acceptance checklist item.
