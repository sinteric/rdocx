# F-X268, canonical frame relationships, pass 1

**Reviewed**: Working mechanism 7 on parent `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. Surrounding diff: 4 source and test files, 5598 additions and 316 deletions. This bounded source pass examines positive frame relationship contributions. The new test binary has not compiled successfully.
**Reviewer**: Codex root. Grok implements and transcribes the report.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, child facts include frame IDs and move tail occurrences before later body blocks

`crates/rdocx/src/document.rs:6039`
`crates/rdocx/src/document.rs:8779`

The child scanner reads the complete head-child-tail wrapper and stores every relationship id it finds. This was safe while the frame was required to contain no ids. The new frame proof permits those ids, but the child fact reader still includes them.

For head H, first body block A, second body block B and tail T, the first child contributes H, A, T. The global composer then publishes H, A, T, B rather than the whole scanner's H, A, B, T. An empty first paragraph can leak T before all later body relationships as well. Global deduplication cannot repair that first-occurrence order.

Collect a child's relationship occurrences only from that child's physical range under the validated wrapper context. Removing frame ids by value is insufficient because an id may first occur in a body child and also occur in the tail. Preserve the existing validator's errors and ordering. A concrete shared scanner with an optional collection range is one possible repair.

The regression must use at least two body blocks, a tail-only id, an id repeated in body and tail, and plain and inherited bindings. Compare the exact ordered vector with the whole scanner. The present single-body-block fixture cannot expose the interleaving fault.

## Smells

None.

## Nitpicks

None.

## Not found

No further finding was established in this bounded source pass. The complete-shell and head-prefix split preserves order for the frame itself, and drawing-bearing frames remain conservative fallbacks.

The pre-change header/footer diagnostic completed successfully and recorded growing canonicalization input, 571458 and 2214708 bytes. It is an observed diagnostic cost, not a failing native assertion. The earlier Python acceptance run is separate actual failing evidence. The current compile failure is a moved `head_ids` borrow in `split_relationship_ids_at_head`, recorded in `/tmp/fx268-m7-canonical-frame-test.log`. No new runtime, Python or feature pass is claimed.
