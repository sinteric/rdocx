# F-X268, source piece integration, pass 7

**Reviewed**: Pass 6 caller-accounting D1 remediation and the live nested relationship regression in `work/f-x268-codex`, parent `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. The surrounding nine-file diff contains 7071 added and 228 removed lines. `crates/rdocx/src/document.rs` SHA-256 is `beb81a0035b336102de153c4b80b5209e6bc280c974f4cb01dab86b2cda676d0`.
**Reviewer**: Independent Codex agent `/root/review_source_pieces`. The reviewer did not implement this repair, modify source or run builds or tests.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in this scope. Pass 6 D1 is resolved.

## Defects

None. The caller-accounting D1 from pass 6 is closed.

## Smells

None.

## Nitpicks

None.

## Resolved finding

D1 identified an uncounted whole-main parse in the live nested relationship-remap caller. `remap_authored_nested_story_relationships` now supplies the exact current canonical part record to the shared source reconstruction through `offered_canonical_blocks`. The reconstructed model comes from actual source read proofs. Its current writer memo clears `part` and does not publish package bytes. If reconstruction is unavailable, the whole parser's complete input is charged before the attempt. Inspected path is `crates/rdocx/src/document.rs:21732`.

The existing relationship rename scanner now charges its actual full input to `canonicalized` before creating its reader. Empty remaps still return without a scan. Invalid XML or attributes retain the previous errors, and failed attempts remain counted. The scan itself is still whole-input work, not a newly bounded child operation. Inspected path is `crates/rdocx/src/document.rs:6918`.

## Not found

Correctness, error order, namespace identity, private metadata, source publication, checked fallback, counter units and structure produced no remaining finding in the inspected repair. The exact partition and source-read guards from the previous passes remain active. A current memo does not grant authority over the retained package. The live registered relationship set is updated only after the remap and parse succeed. The force-off reference cannot take the new partial reconstruction branch.

## Evidence examined

`nested_relationship_edits_reuse_source_parse_facts` creates an actual picture relationship, registers it as live nested-story state and remaps it through four distinct values. It compares the helper's result and private model with the force-off whole path, checks retained package parts, and checks the updated registered relationship set. After two changed warmups it requires zero whole-main parse bytes and equal child work at 50 and 200 paragraphs. It prints, rather than hides, the remaining whole rename scan.

`/private/tmp/fx268-codex-nested-remap-red.log` preserves the actual initial failure, 0 passed and 1 failed in 0.08 seconds, with `[3517, 0, 3517, 0, 1, 0]`. `/private/tmp/fx268-codex-nested-remap-green.log` records 11 passed tests in 5.05 seconds, including the actual public story-picture whole-reference controls. The measured nested-remap rows are `[0, 0, 3517, 0, 0, 1]` at 50 paragraphs and `[0, 0, 10217, 0, 0, 1]` at 200. Those rows close the parse-accounting finding without claiming all six units are flat. The successful affected-crate all-target clippy result with warnings denied is recorded in `/private/tmp/fx268-codex-m7-scoped-clippy3.log`, 31.17 seconds. No command was rerun in this review.

## Scope and pending work

This is the final bounded mechanism 7 caller review, not a whole-F-ID microscope or final acceptance. The whole relationship rename scan remains a disclosed linear cost pending mechanism 8. The expanded picture and scoped-replacement operations, A3 and A5 composition, release equality and error order, memory, the 79 timing gates, complete 252-call report, native Word, hash and full integrated verification remain pending. The earlier Python artifact and original A10 observations are not reattributed to this changed source. No completed feature, ready-to-close or release state is supplied.
