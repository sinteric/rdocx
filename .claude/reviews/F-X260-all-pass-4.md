# F-X260, all, pass 4

**Reviewed**: uncommitted product and HLD diff against `f57e23de`, 18 files, 519 insertions and 261 deletions.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

This pass is the feature microscope for the A5 cost delta. It is not the requested Codex sprint review.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness produced no defect. `BodyContent` stores every variant in a box (`crates/rdocx-oxml/src/document.rs:735`). `CT_Body::remove` still uses `Vec::remove`, so the shift copies the 16-byte enum. Pointer identity compares the heap child through `as_ref()` (`crates/rdocx/src/document.rs:26868`). A plain paragraph replacement writes the new value into the existing box (`crates/rdocx/src/document.rs:27100`). `CT_Tbl::default` and `CT_P::default` both call `new()`, so `Box::default()` is the same value.

`ShapeIdAllocator::scan` collects typed ids and preserved blobs (`crates/rpptx-oxml/src/shape_tree.rs:107`). It serializes the whole tree only when `ancestor_scope` is set (`crates/rpptx-oxml/src/shape_tree.rs:124`). A preserved `p:cNvPr` stays local because an unbound `p` is PresentationML (`crates/rpptx-oxml/src/shape_tree.rs:344`). Pictures, connectors and graphic frames serialize that one child. Custom geometry serializes that shape only.

Contract produced no defect. The diff implements Q10. It does not add a second owner registry and it does not skip validation, namespace custody or stale-handle checks. HLD 03 states the boxing and the scan rule (`docs/hld/03-architecture.md:92`). HLD 10, 12 and 14 already state the visit and ratio contract and were not edited. The plan checklist item for the release ratio and installed suites stays open because the installed suites of this delta were not rerun.

Panics produced no new finding. The scan's `allocate` wrap and the existing section-owner `expect` calls are unchanged in kind. OOXML child order is unchanged because writers still dereference the boxes.

Tests produced no defect. `shape_id_allocator_occupies_a_typed_tree_id` and `shape_id_allocator_uses_typed_ids_for_textboxes` reject a scan that drops a typed id. Five `shape_id_allocator` tests passed. `issue389_local_edits_do_not_scale_with_unrelated_package` in `rdocx` passed after the in-place replacement and asserts one visit. It does not measure wall-clock ratio. The release binary rebuilt from this tree printed `body_content_bytes=16` and all seven ratios below 3. The highest was `word remove_content` at 2.832 under load averages 31.13, 18.69 and 19.08. The methodology is the median of five, one timed call per fresh package, construction outside the timer, and a debug build refused. `python3 scripts/hash_harness.py --check` then reported `hash_harness: 49 entries match`.

Structure produced no defect. `OrderedRawChildren::iter` is a method on the existing type (`crates/oxml-drawing/src/order.rs:36`). No new trait, generic, crate, module or file.

## Final gate debt

Installed product suites have not been rerun on this delta. The README archive rows were measured at `f57e23de` and still need a remeasure after this source lands. The authentic Word content-control differential remains pending and is the F-286 acceptance blocker. This pass does not tick the final acceptance checklist item.
