# F-X211a, cell range, pass 1

**Reviewed**: One frozen native carrier, actual source36 to37, 45 additions and3 deletions. The owning helper patch preserves actual cell pointer correspondence and replaces the incorrect ordinal mapping. One additional existing-module control covers nested/opaque/refusal/reopen facts. No F222 scanner or model carrier changes.

**Verdict**: 0 defects, 0 smells, 0 nitpicks within this helper repair. Public migration and whole-child acceptance remain pending.

## Contract and authentication

Read the canonical microscope/workflow, approved parent/childa physical-owner and immutable-corpus contracts, and the existing checked story/range/model providers. A facade paragraph ordinal must resolve the actual paragraph of the selected modeled cell, never a combined item ordinal or matching text/value.

Checkpoint `cell-range-checkpoint-37.json` SHA `b9bc7a07a91782500cfe177b5fa339a8d4933316176461019a5cb7f908e521b4` authenticates retained preimage SHA `cae4f5d69043f01961ce2e7af4f0dbd10c9e9dfdaea912cfbc173af21d2bda43`, current/copy document.rs SHA `0167ae8bce45e70cd7211d4c78fdc14deeee295d7f61e92246cd4f882d0ea50d` and owning exact helper patch SHA `7ab93ca9237bee539e877228de9cee50f981e50caab2216d74019530505e715f`. The complete retained preimage/current diff has only the helper and additional control hunks. The frozen native carrier remains byte-identical before and after writing. Disjoint public Python implementation changes are permitted and are not falsely guarded against the historical full source37 snapshot.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Aspects checked

- Correctness: zero findings. `crates/rdocx/src/document.rs:24394` selects the modeled CT_Tc, and the existing route at24405 retains real pointer equality against that selected cell. The repair at24418 obtains checked paragraph ranges for that same actual owner, sorts physical source starts at24422, compares source/model paragraph counts at24423 and selects the requested paragraph only from that paragraph inventory. It no longer constructs [paragraph] as a combined story item index. Snapshot validation remains at24428.
- Contract: zero findings. The retained `story_range_paragraph_spans` provider at `document.rs:6596` constructs actual source item indexes and supported two-segment control paths. Inline fields/drawings and preserved nodes do not become paragraphs by ordinal. Count mismatch explicitly refuses unsupported model/source correspondence before a location escapes. This does not establish lifetime survival by itself. Public capture/retirement and current/future scope authority are separate childa obligations.
- Panics: zero findings. Invalid table/row/cell and paragraph lookups remain fallible. The new expect at24419 consumes the exactly one inventory produced from the one supplied checked owner. No raw pointer is dereferenced or retained after this call, it is compared only while the staged candidate remains unmodified. Sorting/range derivation creates no publication or counter mutation. The caller's original Document remains unchanged on refusal.
- OOXML: zero findings. The actual owner/namespace qualification and F222 scanners remain unchanged. `document.rs:11162` admits only the selected control's direct qualified paragraphs and excludes descendants of another control, while separate nested-control inventories provide their own paragraphs. Physical sorting restores the nested lexical order. Modeled cell traversal at `crates/rdocx-oxml/src/table.rs:2337` and control traversal at `crates/rdocx-oxml/src/content_control.rs:1458` retain their current owner-filtered rules. Nested table profiles that the two inventories cannot prove equally are refused, not silently flattened. Foreign lookalike paragraphs remain opaque.
- Tests: zero findings. The unchanged deciding control at `document.rs:37681` checks all three actual paragraphs after a p0 simple field. The additional control at37708 checks outer-before/nested/outer-after source order, a foreign-default opaque paragraph, all invalid coordinate axes, a foreign paragraph replacement refusal, exact preserved opaque bytes and actual save/reopen. Neither test matches paragraphs only by duplicate text to manufacture physical identity.
- Structure: zero findings. Existing checked range and modeled traversal providers are reused. No new scanner, type, trait, generic, registry, module, file, dependency or revision framework is introduced. No public c behavior is implemented by this repair.

## Actual failure and successor

Original36 receipt SHA `2eb1aa6c0e7e883a76cac6f6feed7d4578f73271187f25fcb2e8968e45284554` and raw logs authenticate. It preserves the initial wrapper-handler failure honestly. The actual executed deciding test exits101 and reports both defects: paragraph1 selects a Field instead of Paragraph, and paragraph2 selects middle instead of last. This is a genuine behavioral RED, not a missing API/compiler failure. The retained original36 executable SHA `30268a75899ab6bf89db9343a1d1e7cae1deee607d132995050f8c56003d7aa4` independently rehashes correctly.

Current37 receipt SHA `a14d146a7784272fa91c35bf9ef3ba83c436a941bfb1a9fd81f14d3e72b34d75` authenticates three actual safe exit0 rows: native test binary preparation and both exact controls. All raw log hashes match, with reaped leaders and gone owned groups. The controls use ordinary2097152-byte stacks and30-second finite deadlines, with no timeout. Current37 binary SHA `da908ddc3090cbe3a6a14a3d6e01bfb9cd33f9a18a191ac2fd4e9a0e0f391f39` rehashes correctly. Originals are not relabeled passing.

These two controls explicitly execute the simple-field case, not an inline-drawing producer or default-Word-namespace producer. The foreign-default namespace negative is distinct from a default-Word positive. There is no deciding count-mismatch fixture in this bounded pair. Those later consumer/profile qualifications are not invented here, and this repair's use of existing qualified scanners is not a full public or all-profile acceptance claim. No broad suite, formatting/Clippy, installed Python, immutable100, original85, package, CP, Office or whole-feature gate is credited by this pass.

Only this canonical review file was written. No source fix, test, Cargo, target, CP, UI, index, ledger or commit operation was performed. Review ends and returns control to ROOT.
