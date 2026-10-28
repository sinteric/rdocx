# F-X268, source piece integration, pass 6

**Reviewed**: Drawing-ID caller integration, its two native regressions, current-intent HLD additions and equivalent scoped lint repairs after pass 5 in `work/f-x268-codex`. Parent is `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. The surrounding nine-file diff contains 6978 added and 227 removed lines. `crates/rdocx/src/document.rs` SHA-256 is `9cd2667366f2eadf5ff39104a6b97dec5cd1f690dab97fcbfe1d9605c9c7f669`.
**Reviewer**: Independent Codex agent `/root/review_source_pieces`. The reviewer did not implement this increment, modify source or run builds or tests.
**Verdict**: 1 caller-accounting defect, 0 source-safety findings, 0 smells, 0 nitpicks.

## Defects

### D1, a live nested relationship remap omits its whole-main parse work

`crates/rdocx/src/document.rs:21737`
`crates/rdocx/src/document.rs:21530`

`canonicalize_all_drawing_ids` invokes `remap_authored_nested_story_relationships`. When registered nested relationships are present in the current main XML and `relationship_remap` is nonempty, the helper remaps that XML and parses the complete main part through `CT_Document::from_xml(&updated)`. That actual parse does not charge `staging.parsed` before the attempt. Its bytes can therefore be absent from the reported public-operation work even though this caller reads the whole main part.

Record the actual complete input before that parse, including failed attempts, or route it through a qualified partial source proof with an honestly counted whole fallback. A control with a live registered nested relationship and a nonempty remap is needed. The new modeled drawing-ID regression deliberately exercises a different branch and cannot establish the missing accounting. This finding is established by caller inspection, not a newly executed runtime RED.

## Smells

None.

## Nitpicks

None.

## Not found

Source reconstruction correctness, checked arithmetic, namespace identity, private metadata, source publication and error-order preservation produced no additional safety finding in the inspected increment. D1 concerns work evidence, not an established output mismatch.

The canonical drawing edit starts from the just-produced canonical XML and its exact block record. Slots come from the corresponding checked scanner or exact-part child proof. Reverse ordered edits retain the existing generated span assumptions. The partial reconstruction uses the record's actual part bytes and the explicit proof gate. Its candidate model is assigned only after reconstruction, resource admission and partition checks. The stored writer memo clears `part`, and this branch does not publish package bytes. Whole fallback parsing charges its full input before attempting the read. Inspected path is `crates/rdocx/src/document.rs:21633`.

Retained-source eligibility obtains an actual parse bound to current part bytes, through either the retained block record, the exact opening or the whole reader. It still asks the existing parsed-model equality and serialization-difference questions. Stale part bytes cannot admit a retained block proof. The final retained-ID patch uses actual source reconstruction or the previous whole parser, then installs the exact retained bytes and corresponding current read record together. Existing relationship slot checks, related-story canonicalization, identifier updates, image handling and enclosing signature preparation remain in place. Inspected paths are `crates/rdocx/src/document.rs:21229` and `crates/rdocx/src/document.rs:21658`.

Field, tab and bookmark exclusions remain in the shared source and writer helpers. An unavailable root, frame, child or private-fact proof keeps the whole fallback. Force-off prevents the new partial canonical reconstruction and retained source reconstruction. It is not claimed to disable every unrelated cache, consistent with the corrected HLD wording from pass 5.

The scoped lint changes preserve the existing policies. Collapsed equal branches and matches retain the same conditions, the removed current borrow changes no input, identical test literals retain their fixture bytes, the diagnostic option comparison preserves its guarded result, and the local `StageSample` alias removes repeated tuple spelling without adding a runtime abstraction. The HLD testing and backlog additions describe these implemented callers while retaining whole-mechanism, mechanism 8 and final qualification as pending. The earlier documentation finding remains closed by pass 5.

## Evidence examined

`drawing_id_edits_read_only_the_changed_body_child` performs four distinct ID changes, with two changed warmups, at 50 and 200 paragraphs. It compares private parsed state, output XML, part bytes and canonical ID 1 against the force-off reference. `retained_story_picture_source_matches_whole_canonicalization` repeatedly publishes actual story pictures on canonical and producer-whitespace packages and compares complete part maps, parsed state, saved bytes and paragraph counts with the whole reference. It covers the retained-source branch without qualifying every registered-relationship rename combination.

`/private/tmp/fx268-codex-drawing-source-red.log` preserves the modeled-ID test's actual initial failure, with `[3502, 0, 0, 0, 3, 0]` at 50 paragraphs. `/private/tmp/fx268-codex-drawing-source-green.log` records 9 passed tests in 4.24 seconds. `/private/tmp/fx268-codex-drawing-callers-native.log` records 10 passed tests in 5.02 seconds. The scoped all-target check finished in 28.78 seconds. After the equivalent lint changes, `/private/tmp/fx268-codex-m7-scoped-clippy2.log` records successful affected-crate all-target clippy with warnings denied in 33.64 seconds. Native runtime evidence precedes those lint-only edits. No command was rerun by this reviewer.

The original nine Python work checks plus alternate-root control passed 10 tests in 193.45 seconds before these drawing caller changes. The original grouped native A10 test passed in 69.57 seconds before them. Those observations remain attributable to their earlier source and artifact. The installed Python extension has not been rebuilt for this drawing increment. Current drawing qualification is native only.

## Scope and pending work

D1 needs separate implementation and review. This is not complete mechanism 7, whole-feature or sprint acceptance. Mechanism 8, the expanded picture and scoped-replacement operations, A3 and A5 composition, release equality and error order, HLD completion for subsequent changes, timing, memory, native Word, hash and full integrated verification remain pending. No final status or release transition is supplied.
