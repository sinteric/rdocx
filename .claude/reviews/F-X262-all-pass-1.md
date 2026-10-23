# F-X262, all, pass 1

**Reviewed**: uncommitted working tree on `work/f-x262-grok` at `2fdb6bf3`. 22 tracked files, 1164 product insertions and 113 deletions, plus the plan deviation record. Product files are the preset generator, `oxml-drawing`, `rpptx-oxml`, `rpptx`, `rpptx-render`, `rpptx-py`, and HLD 06, 08, 10, 12 and 14.
**Verdict**: 0 defects, 0 smells, 1 nitpick

This pass is the feature microscope. It is not the requested Codex sprint review.

## Defects

None.

## Smells

None.

## Nitpicks

- `tools/gen-presets/generate.py:220`, the failure text still says unique source name count after the check includes the inserted `upArrow` definition.

## Not found

Correctness produced no defect. Slide placeholder order sorts by idx and keeps document order for equal indexes (`crates/rpptx-py/src/shape.rs:2490`). Layout and master paths keep document order. `__getitem__` still looks up the idx on the top-level shape list (`crates/rpptx-py/src/shape.rs:2524`). `percentage_thousandths` uses `round_ties_even` (`crates/rpptx-py/src/dml.rs:26`). Bool is rejected before extraction. Empty paragraph text drops runs and keeps paragraph properties (`crates/oxml-drawing/src/text/paragraph.rs:3960`). `field_paragraph` inserts a run after that empty assignment (`crates/rpptx/src/lib.rs:29154`). `TextFrame.clear` publishes through `text_changed_owners`, which retires paragraph and run owners under that frame only (`crates/rpptx-py/src/presentation.rs:4152`). Cell addresses go through `set_cell_run_hyperlink` (`crates/rpptx/src/lib.rs:12277`) and `click_text_body_mut` (`crates/rpptx/src/lib.rs:15191`). `target_slide` still refuses a cell before mutation (`crates/rpptx-py/src/text.rs:1782`). Painted row growth is added to the last row of the span and is not written back to `a:tr/@h` (`crates/rpptx-render/src/lib.rs:1157`).

Contract produced no defect. A1 through A9 stay mapped. Q2 through Q7 stay the approved behaviour. Notes slides do not gain a placeholder collection. The layout lookup test checks idx lookup rather than document position. HLD 15 is unchanged because its preset pin text does not move. The file lease is recorded as met under Deviations. The product contract is unchanged. PR394 stays unmerged.

Panics produced no finding. The new library paths return `Result` or a boolean. Test `expect` calls are on constructed fixtures.

OOXML produced no finding. `clear_style` drops `p:style` and leaves the remaining children in place (`crates/rpptx-oxml/src/shape_tree.rs:1435`, `crates/rpptx-oxml/src/connector.rs:157`). The connector clears both the typed view and the preserved bytes because the writer emits the bytes. Empty text keeps paragraph properties and end properties. The generated `upArrow` geometry parses as custom geometry.

Tests produced no defect. Reverting the idx sort would fail `issue392_placeholder_iteration_crosses_index_gaps`. Reverting ties-to-even would fail the stored `574`, `25002` and `-2` assertions, which also reject the half-away alternatives. Reverting row growth would fail `issue392_table_rows_grow_without_overlapping_following_rows`. Reverting frame-scoped retirement would fail the live run on the other shape. The python-pptx oracle tails in the older preset tests stay skipped in this environment. They are not the A3 gate. `issue392_up_arrow_has_reproducible_geometry` and the local half of `test_add_shape_accepts_preset_names_and_every_mso_shape_member` ran.

Structure produced no finding. No new trait, generic, crate, module or file was added. The cell coordinate is one tuple argument on the existing method.

## Unavailable

Old-source RED was not recorded. The functional parent `f9d165e77392b57a58223046d8cd551220c61a3a` was not re-read. Installed `rpptx` suites, the pinned viewer, the PowerPoint pixel baseline, both WASM graphs and the hash harness remain final-sprint evidence. This pass does not claim those gates. The Word 49 hash and seven-pixel baseline are declared unchanged and were not re-recorded here.

## Final gate debt

Hash harness is deferred. Any PowerPoint paint delta belongs to wrapped table rows and to this story only. `upArrow` does not move an existing baseline unless a current fixture draws it. The pixel baseline, viewer and wasm checklist item stays open.
