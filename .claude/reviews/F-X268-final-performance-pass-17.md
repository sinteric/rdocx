# F-X268, final performance, pass 17

**Reviewed**: Frozen working diff against `198978eb8bce519d90aa9e2cbba967fd6834eeeb` on `sprint/s95.4`. Six existing files, 167 insertions and 7 deletions. Reviewed the private relationship validator, dense control index, retained fragment selection, four tests and plan/HLD updates under the canonical workflow and microscope contract.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Source review is clean. GREEN verification and final acceptance remain pending.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Correctness and contract

The public relationship validator at `crates/rdocx/src/document.rs:28795` still checks the current owner and fingerprint before calling the extracted relationship-only helper. The helper retains the exact relationship lookup, expected-type, internal-mode, target-resolution and missing-part checks in their original order. Public stale or wrong-owner handles do not bypass validation.

The picture inventory call at `crates/rdocx/src/document.rs:29097` uses that private helper only with a story constructed from an owner scanned out of its current source. `story_sources` has already built the same canonical source family and related sources before iteration. The inventory holds an immutable document reference, so intervening drawings cannot change package bytes or stale that owner. The extraction removes redundant canonical-source construction, not actual relationship validation. Linked images, physical correspondence, ambiguity refusal, positive extent, media identity, content type and image-byte checks remain unchanged. No persistent validation cache or cross-call validity token is introduced.

At `crates/rdocx/src/content_control.rs:2524`, direct lookup of an element by its ID follows a local invariant: every Start or Empty event assigns the next ID and immediately appends exactly one indexed element. End events neither delete nor reorder entries. Thus ID equals append position for every open element. The bounds and ID check retain the old lost-element diagnostic, while unmatched ends and unfinished stacks retain their existing error paths. Parent IDs, lexical namespace classification, opening boundaries and closing boundaries are unchanged. Empty children consume IDs without entering the open stack, which the direct lookup correctly handles.

The new close hook counts each attempted direct lookup at its actual call site. It does not fabricate growing work or count only a successful match. Different index-building callers retain distinct event and close keys.

Scoped replacement at `crates/rdocx/src/document.rs:42782` reuses the canonical source that has just passed current owner and fingerprint validation after preparation. No mutation occurs before the retained view is selected. The new helper independently reads package bytes and scans retained owners. It preserves canonical versus retained owner-kind comparison, scoped source proof, replacement-count refusal, publication and reopen checks. Other fragment consumers still enter through the original checked wrapper. No source coordinates or read authority are transferred across preparation boundaries.

No new findings in panics, OOXML preservation, error ordering or structural rules. The extracted helpers have actual checked and inventory consumers. No public API, cache field, module or source file was added.

## Tests and evidence

The picture test requires exactly one actual assembly for four pictures, checks image payloads, and separately exercises the public validator. The close-work test performs a real changed control move at both sizes under the unchanged five-times bound. The nested/empty index test checks IDs, parents, qualified identity, physical ranges and malformed endings. The retained-view test compares all stories discovered by its fixture with the checked wrapper and verifies that the private view does not build another source. It does not claim that this fixture alone covers every possible story kind.

`/private/tmp/s954-inventory-close-red.log` records two actual failures in 2.33 seconds. Picture inventory assembled five sources rather than one. The old close lookup exceeded the work bound. These are historical semantic and work REDs, not compilation failures or GREEN evidence for this frozen source.

The diagnostic content-control profile supports the old repeated index search, but is not an installed-wheel timing result. The earlier source check predates the final two tests and documentation. The final GREEN batch has not been consumed here. No native count, scoped clippy, hash or timing pass is inferred for the current batch. Independent installed v7 measurement belongs to its authenticated earlier source and must be assessed separately.

The plan and HLD accurately describe current qualification and the limited counter scope. Whole assembled-output bytes are distinct from every allocation, memory retention and release wall-clock timing. Those gates, native oracles and final integrated qualification remain required.

## Exact source identity

- `crates/rdocx/src/document.rs`: `a6a03554f574650bec41381376afa62c8300af2831491abd5b4072445f4abc14`.
- `crates/rdocx/src/content_control.rs`: `c3bc64ddc07e65d2d5ee211e030c9a39556ac355a8c62a276607ab90a45d4ca3`.

The reviewer changed no source and ran no builds or tests.
