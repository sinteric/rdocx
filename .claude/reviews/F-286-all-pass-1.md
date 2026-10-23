# F-286, all, pass 1

**Reviewed**: uncommitted working tree on `work/f-286-grok` at `e615dd73`. 15 tracked files besides this review, 1375 insertions and 14 deletions. Product files are `crates/rdocx/src/content_control.rs`, `crates/rdocx/src/lib.rs`, `crates/rdocx-oxml/src/content_control.rs`, the existing integration entrypoint, `crates/rdocx-py`, and HLD 02, 03, 10, 12 and 14.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

This pass is the feature microscope. It is not the requested Codex sprint review.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness produced no defect. `set_content_control_type` validates the payload before staging (`crates/rdocx/src/content_control.rs:1475`). A missing picture relationship, a drawing that does not embed that id, an empty or repeated list, a bad date format, an empty repeating section, a non-block repeating body, and a repeating item outside a repeating section all return before `publish_candidate`. The picture scan walks the part with its real namespace scope and accepts only an `r:embed` inside a Word `w:drawing` in that shell (`crates/rdocx/src/content_control.rs:2827`). `r:link` does not count. `repeat_content_control` clones the first repeating item through the existing freshen path and appends it inside `sdtContent` (`crates/rdocx/src/content_control.rs:1532`). `remove_content_control` refuses the last direct item of its nearest repeating section before any mutation (`crates/rdocx/src/content_control.rs:1565`, `crates/rdocx/src/content_control.rs:3173`). Combo and drop-down fill share `prepare_text_value` (`crates/rdocx-oxml/src/content_control.rs:228`).

Contract produced no defect. A1 through A8 stay mapped. Q17 stays the approved payload list. Citation, equation, bibliography, and document-part kinds have no payload variant. Python exposes `ContentControlTypePayload` and the two methods, and `insert_content_control` stays payload-free (`crates/rdocx-py/src/document.rs:5536`, `crates/rdocx-py/src/document.rs:8112`). WASM and CLI gain no method. DOCX-053 stays partial with owner F-286, and its native cell stays `P` because the Word differential is pending. The dependency-readiness sentence stays historical. The prefix note is under Deviations. The Word differential checklist item and the live Word compare item stay open.

Panics produced no finding. Public paths return `Result`. Test `unwrap` calls are on constructed fixtures.

OOXML produced no finding. Date children are `w:dateFormat`, `w:storeMappedDataAs` with `text`, then `w:calendar` with `gregorian`, and `w:lid` is omitted (`crates/rdocx/src/content_control.rs:2766`). Checkbox children are `w14:checked` `0`, `w14:checkedState` `2612` in `MS Gothic`, then `w14:uncheckedState` `2610` in `MS Gothic`. List items keep caller order. Type replacement deletes recognized type children and inserts the new element at the first type position, leaving unmodelled children and a foreign-namespace checkbox. A repeating wrap writes one nested `w:sdt` whose type is `w15:repeatingSectionItem`.

Tests produced no defect. Reverting the picture relationship check would fail `typed_picture_control_requires_an_owned_image`. That fixture uses a preserved `wp:inline` drawing and a document-level relationship prefix that is not `r`. Reverting list refusal would fail `typed_list_control_rejects_unknown_and_ambiguous_values`. Reverting the last-item guard would fail `repeating_section_clones_the_template_item`. Reverting the parent rule would fail `repeating_item_requires_a_repeating_section_parent`. The Python test refuses an empty drop-down, a repeated combo value, repeat on a non-section, and removal of the last item. Local tests do not certify Word. Generated XML is not a Word oracle.

Structure produced no finding. No new trait, generic, crate, module, or file was added. The payload enum and the two methods sit on the existing document facade.

## Unavailable

The authentic Word differential has no provenance, version, or input identity in the repo. That gate stays pending and is an acceptance blocker for marking F-286 done. The hash harness, the installed `rdocx` suite, both WASM graphs, and the deterministic font check remain final-sprint evidence. This pass does not claim those gates.

## Final gate debt

Hash harness is deferred. A document that does not call the new API must keep its bytes. The final checklist items stay open.
