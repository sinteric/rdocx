# F-285, all, pass 1

**Reviewed**: uncommitted working tree on `work/f-285-grok` at `2a6078ce`. 13 tracked files, 2438 insertions and 14 deletions, plus the plan deviation record. Product files are `crates/rdocx/src/content_control.rs`, `crates/rdocx/src/document.rs`, `crates/rdocx/src/lib.rs`, the existing integration entrypoint, `crates/rdocx-py`, and HLD 02, 03, 10, 12 and 14.
**Verdict**: 0 defects, 0 smells, 1 nitpick

This pass is the feature microscope. It is not the requested Codex sprint review.

## Defects

None.

## Smells

None.

## Nitpicks

- `crates/rdocx/tests/integration_test.rs:25958`, the two `w:val="1"` branches of the first id assertion are the same string.

## Not found

Correctness produced no defect. Insert validates kind, tag and alias before staging, and a parent mismatch returns before `commit_staged_mutation` or `publish_candidate` (`crates/rdocx/src/content_control.rs:1363`, `crates/rdocx/src/content_control.rs:1834`). The id is the smallest unused positive `i32` taken from word `w:id` children of `w:sdtPr` (`crates/rdocx/src/content_control.rs:1555`, `crates/rdocx/src/content_control.rs:2142`). Metadata edits splice one property and leave `sdtContent` (`crates/rdocx/src/content_control.rs:2167`). Clearing placeholder matches only the Word `showingPlcHdr` (`crates/rdocx/src/content_control.rs:2222`). Move stays in one story and refuses a destination inside the source (`crates/rdocx/src/content_control.rs:1485`, `crates/rdocx/src/content_control.rs:1507`). Clone of a nested control freshens through the existing identity path (`crates/rdocx/src/content_control.rs:2599`). Removing the last paragraph of a cell returns the original package (`crates/rdocx/src/content_control.rs:2567`).

Contract produced no defect. A1 through A6 stay mapped. Q16 and Q21 stay the approved surface. Python does not expose `ContentControlSpec`, move, or clone (`crates/rdocx-py/src/document.rs:7888`). WASM and CLI gain no method. `Document::content_controls` stays the body inventory, and the header assertion checks that exclusion (`crates/rdocx/tests/integration_test.rs:26012`). The file lease is recorded as met under Deviations. The historical readiness sentence stays. DOCX-052 is complete for rich and plain shells, and DOCX-053 still owns typed payloads. The owner exclusion records that split (`scripts/test_sprint_workflow.py:8878`).

Panics produced no finding. Public paths return `Result`. Test `unwrap` calls are on constructed fixtures.

OOXML produced no finding. Created property order is alias, tag, id, lock, placeholder, type, then `w15:appearance` (`crates/rdocx/src/content_control.rs:1686`). Child order is `w:sdtPr` then `w:sdtContent`. A row shell has one `w:tr` whose cell count follows the first direct row or the grid (`crates/rdocx/src/content_control.rs:2071`). A cell shell is `w:tcPr` then one paragraph (`crates/rdocx/src/content_control.rs:1733`). Unknown `sdtPr` children have no rank and are not reordered (`crates/rdocx/src/content_control.rs:2248`). The story-item scan seeds ancestor prefix bindings and does not call `story_namespace_scope_at` (`crates/rdocx/src/document.rs:12872`).

Tests produced no defect. Reverting the parent check would fail `content_control_create_round_trips_each_placement`. Reverting raw-slot preservation would fail `content_control_metadata_edit_preserves_raw_slots`. Reverting id freshening would fail `content_control_move_and_clone_keep_or_refresh_identity`. Reverting the cell-paragraph refusal would fail `content_control_remove_is_atomic`. Reverting fill rollback would fail `content_control_create_then_fill_uses_existing_setter`. Reverting the kind refusal would fail `content_control_typed_payload_is_refused_until_f286`. The Python test refuses a mismatched placement, a checkbox, and a non-bool placeholder, then fills and removes the shell. Q21 has no content argument, so an inline paragraph payload is the imported unwrap refusal rather than a second create argument. That deviation is recorded in the plan.

Structure produced no finding. No new trait, generic, crate, module or file was added. The Python insert keeps the approved operands on one method.

## Unavailable

Old-source RED was not recorded. The methods did not exist, so compile-time absence is not a semantic failure. The hash harness, the installed `rdocx` suite, both WASM graphs and the deterministic font check remain final-sprint evidence. This pass does not claim those gates. Full `cargo test -p rdocx` did not finish. The three isolated failures are the local pdftotext pin, the local rasterizer line, and a math-limit stack overflow that also occurs on the clean sprint tree.

## Final gate debt

Hash harness is deferred. Creation must not alter the bytes of a document that does not call the new API. The final checklist item stays open.
