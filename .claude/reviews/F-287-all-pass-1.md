# F-287, all, pass 1

**Reviewed**: uncommitted working tree on `work/f-287-grok` at `2e22da33`. 13 tracked files besides this review, 845 insertions and 14 deletions. Product files are `crates/rdocx/src/content_control.rs`, `crates/rdocx/src/bibliography.rs`, the existing integration entrypoint, `crates/rdocx-py`, and HLD 02, 03, 04, 10, 12 and 14.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

This pass is the feature microscope. It is not the requested Codex sprint review.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness produced no defect. `add_custom_xml_store` rejects empty item bytes, a fragment, bytes that are not XML, and an empty schema URI before `prepare_staged_package` (`crates/rdocx/src/content_control.rs:1432`, `crates/rdocx/src/content_control.rs:1818`). `set_content_control_binding` parses mappings and the path, resolves one store, and selects one element before it splices `w:dataBinding` (`crates/rdocx/src/content_control.rs:1461`). `None` and an empty mapping omit `w:prefixMappings`. An existing `w:dataBinding` is replaced through `property_matches` rather than duplicated (`crates/rdocx/src/content_control.rs:2432`). Value updates stay on `set_content_control_value_by_tag` and `bind_content_controls`. `custom_xml_bind_failure_rolls_back_display_and_store` calls `bind_content_controls` on the live document and compares `to_bytes` with the pre-call bytes.

Contract produced no defect. A1 through A6 and C1 through C5 stay mapped. Q18 stays the approved surface. The return value is the braced item id. `create_collection` still returns the part name (`crates/rdocx/src/bibliography.rs:1890`). An empty schema list writes the bibliography `<ds:schemaRefs/>` string (`crates/rdocx/src/bibliography.rs:1867`). Caller order, including a repeated URI, is kept. The helper stays in `rdocx` and is not an `oxml-*` type. Python exposes both methods and no new binding object (`crates/rdocx-py/src/document.rs:8141`). WASM and CLI gain no method. DOCX-054 stays `partial` with owner F-287, Read `P`, Remove `N`, and WASM and CLI `B` (`docs/hld/02-scope-and-non-goals.md:321`). The dependency-readiness sentence stays historical. The prefix and helper notes are under Deviations. The hash harness checklist item stays open.

Panics produced no finding. Public paths return `Result`. The identity scan uses the existing `ok_or_else` exhaustion error. Test `unwrap` calls are on constructed fixtures.

OOXML produced no finding. Properties children are `ds:datastoreItem` with `ds:itemID`, then `ds:schemaRefs`, then one `ds:schemaRef` per URI (`crates/rdocx/src/bibliography.rs:1866`). Binding attributes are `w:prefixMappings` when present, then `w:xpath`, then `w:storeItemID` (`crates/rdocx/src/content_control.rs:2589`). Insert rank 8 places `w:dataBinding` after `w:showingPlcHdr` and before the type element (`crates/rdocx/src/content_control.rs:2503`). Unknown sibling bytes and comments are not rewritten by the new store writer. The value updater still expands only the selected empty element.

Tests produced no defect. Reverting hole naming would fail `custom_xml_store_round_trips_item_and_schema`, which occupies `item2` and requires the new store on `item1` with `item3` absent. Reverting bibliography byte identity would fail that same test's C5 assertions. Reverting xpath refusal would fail `custom_xml_xpath_stays_inside_the_existing_subset`. Reverting sibling rollback would fail `custom_xml_bind_failure_rolls_back_display_and_store`. Reverting namespace identity would fail `custom_xml_identity_matching_stays_namespace_aware`. The Python test refuses empty bytes, a fragment, and an empty URI, then fills a bound element and reopens the package. Existing `a_bound_custom_xml_value_updates_the_part_and_display_text_atomically`, `fragment_import_preserves_bound_custom_xml_store`, and `custom_xml_identity_decoys_do_not_block_open` stayed green. A missing `add_custom_xml_store` is not a semantic RED at compile time. The missing-store refusal stays in `custom_xml_binding_updates_the_addressed_store`. This story's gate is round trip. It is not a Word oracle.

Structure produced no finding. No new trait, generic, crate, module, or file was added. `StagedCustomXmlStore` is `pub(crate)` and exists so bibliography and the public method share one allocator.

## Unavailable

F-287 has no authentic Word differential in its approved gate. The F-286 Word differential still has no provenance, version, or input identity in the repo. That remains an acceptance blocker for marking F-286 done. The hash harness, the installed `rdocx` suite, and the `rdocx-wasm` check remain final-sprint evidence. This pass does not claim those gates.

## Final gate debt

Hash harness is deferred. A document that does not call the new API must keep its bytes. The hash harness checklist item stays open.
