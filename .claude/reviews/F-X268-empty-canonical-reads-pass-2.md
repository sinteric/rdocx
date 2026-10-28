# F-X268, empty canonical reads, pass 2

**Reviewed**: Codex root reviewed D1 and S1 remediation on worker HEAD `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. The current three-file working diff contains 2296 added and 96 removed lines, including the earlier reviewed prerequisites. This is the bounded mechanism 7B4A canonical-read review.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Resolved findings

D1 is resolved. Current and masked relationship inventory uses the original serialization and retained-namespace replay when owner facts are nonempty. Replayed bytes are authoritative. Optional block proof is retained only when its serialized bytes exactly equal that replayed string. An optional proof error cannot replace a successful legacy replay or turn its result into an empty list. The empty-owner path retains the bounded temporary source.

The new regression observed actual pre-fix RED. Two retained sdt owners whose text collides caused the new path to return Ok([]), while the legacy path refused with `cannot identify retained sdt nested namespace owner after mutation`. That exact refusal now agrees, alongside the valid compatible-owner control. The serialize closure is used again and its new unused warning is gone.

S1 is resolved. The private force-full switch and its scoped guard compile under test or debug_assertions. The debug oracle enters that guard before running its full canonicalization clone. Reference tests enter it outside debug-only counter blocks, so release tests also force the old validators. Drop restores the previous state on normal return, error and unwind. No stable API, new trait or generic parameter was added.

## Evidence and limits

`/tmp/fx268-m7-canonical-remediation-red.log` records the actual retained-owner RED, 0 passed and 1 failed in 0.06s. `/tmp/fx268-m7-canonical-remediation-test.log` records the targeted GREEN, 1 passed in 0.13s, and the batch, 21 passed and 0 failed in 54.94s. Ordinary measured edits retain staging `[0, 0, 0, 0, 6, 1]`, identifier input 1308 and partial canonical input 12924 at both sizes. The forced full path retains growing actual scan work and the same models, output bytes and identifiers.

The scoped check passes. Release test targets type-check in 55.51s with the earlier addressing code's unused child_count warning. Clippy completes after two lint-only changes, lifetime elision and the equivalent matches expression. The recorded behavioral batch and release typecheck precede those two lint-only changes. Release behavior has not been executed.

## Not found

No additional retained-namespace policy, temporary-publication, cache-context, resource-order, full-reference, counter-unit, panic or structural finding was established in the remediation inspection. Current canonical records remain temporary until the existing flush publishes them. Partial validator work and whole-model walks, writes, copies and clones remain distinct costs.

Positive picture and relationship-remap fast paths, complete native A10 including section and list cases, Python rebuild, HLD, whole-feature composition, release behavior, timing, memory, Word and final gates remain pending. The installed extension remains the mechanism 6 build. This is not mechanism 7 acceptance or a whole-feature microscope.
