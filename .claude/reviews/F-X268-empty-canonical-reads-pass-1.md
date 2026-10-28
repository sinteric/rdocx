# F-X268, empty canonical reads, pass 1

**Reviewed**: Codex root reviewed the bounded mechanism 7B4A canonical-read prototype on worker HEAD `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. The current three-file working diff contains 2171 added and 96 removed lines, including the earlier reviewed prerequisites. This pass covers temporary serialization, empty validator facts, canonical read callers and the forced-reference evidence.
**Verdict**: 1 defect, 1 smell, 0 nitpicks

## Defects

### D1, relationship inventory bypasses retained namespace replay

`crates/rdocx/src/document.rs:19769`

The old serialize closure writes the model and replays nested_namespace_owners before querying current and masked relationship XML. It is now unused. The two replacement paths at lines 19774 and 19845 use temporary block serialization directly. canonical_source and canonical_source_from perform resource admission and serialization, but neither replays those retained namespaces.

For retained owners whose bindings must be restored after modeled or masked serialization, the queried XML therefore loses the old namespace restoration and its refusal policy. A failed or nonempty block proof only chooses the whole relationship scanner on those already unreplayed bytes. It does not restore the old serialization path. This can change relationship interpretation or error order. The forced-full reference also shares this changed caller, so the passing ordinary comparison cannot establish equivalence for that state.

Keep the optimized empty-owner path only when it proves the same input. For nonempty or unproved retained owners, use the original serialization and replay policy for both current and masked XML before scanning. Preserve its exact refusal and error order. The new unused-closure warning is a symptom of the lost call, not a reason to delete that policy.

## Smells

### S1, the claimed full oracle can still use block proof

`crates/rdocx/src/document.rs:19879`

The existing debug oracle clears parsed on its clone and calls canonicalize_all_drawing_ids. That method now constructs temporary blocks and consumes their cached proof. Clearing parsed does not force the old whole-input validators. The helper switch at line 6476 is available only under test plus debug_assertions and is not set by this oracle. The supposed full comparison can therefore compare two uses of the new proof.

The same switch disappears from release tests, so the current forced-reference fixture cannot supply the planned release old-path comparison as written. Force the authoritative whole-input validator path in the debug oracle and in both debug and release reference tests. Keep counters and debug oracle applicability separate from actual release evidence.

## Nitpicks

None.

## Evidence and limits

`/tmp/fx268-m7-canonical-read-finish-check.log` records check success with the newly unused serialize closure warning. `/tmp/fx268-m7-canonical-read-finish-test.log` records the ordinary pair, 1 passed in 11.54s, and the broader batch, 20 passed in 71.94s. The cargo usage failure in that log came from an invalid filter invocation and is not a product RED.

The ordinary warm pair records staging `[0, 0, 0, 0, 6, 1]` at both sizes, identifier input 1308 at both, and partial canonical input 12924 at both. Saved bytes, models and identifiers agree. The old-algorithm reference records growing whole scans for that ordinary input. This does not cover D1, since both reference and optimized callers currently omit replay. D1 is a source-established policy omission, not a claimed runtime RED observation.

Temporary canonical records leave the flushed package record unchanged and preserve whole-model resource preflight. Nonempty drawing and relationship cases retain whole read fallbacks. Complete positive picture and remap support is not in this prototype.

## Not found

No additional temporary-publication, resource-order, current-byte cache, panic, counter-unit or history-chain finding was established in this bounded inspection.

Current release typecheck, clippy, release behavior, complete native A10 including section and list cases, Python rebuild, HLD, whole-feature composition, timing, memory, Word and final gates remain pending. The installed extension remains the mechanism 6 build. This is not mechanism 7 acceptance or a whole-feature microscope.
