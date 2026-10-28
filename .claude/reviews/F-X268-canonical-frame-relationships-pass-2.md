# F-X268, canonical frame relationships, pass 2

**Reviewed**: Working mechanism 7 on parent `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. Surrounding diff: 4 source and test files, 5711 additions and 210 deletions. This bounded pass reviews D1 remediation and the remaining frame performance failure.
**Reviewer**: Codex root. Grok implements and transcribes the report.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

D1 is closed. The shared scanner validates all relationship attributes before limiting collection to the child event range. Exact ordered comparison now passes for multiple body blocks and a body and tail duplicate.

### D2, unchanged pieces retain facts for the old frame permanently

`crates/rdocx/src/document.rs:8219`
`crates/rdocx/src/document.rs:8524`

The serializer reuses the same block Arc when the modeled child is unchanged. Its context-dependent facts remain in OnceLocks for the previous head and tail. A mismatch returns None without making a current piece that can fill facts for the new frame. Once the tail changes, unrelated unchanged pieces therefore keep selecting whole-part fallback on every later edit, even after the new frame has stabilized.

The actual diagnostic in `/tmp/fx268-m7-canonical-frame-context-diagnostic.log` selected one test and failed in 26.66 seconds, exit 101. At 50 paragraphs, 52 of 62 children decline. At 200, 202 of 212 decline. Their root, body declarations, bindings and head match. Their cached tail is 236 bytes and the current tail is 285 bytes. The current frame is identical through both published warmups and the measured edit. Fresh records produce the same XML and exactly match the whole relationship scanner, while retained records return None.

Replace context-dependent piece facts when a frame changes, with current ownership and bounded retention. Preserve the exact context checks rather than accepting stale facts. The refreshed current pieces must survive publication so the two changed warmups can populate them. Reset failed as well as successful old-context facts, and cover identifier and namespace facts that share this lifetime. Copying unchanged bytes or constructing a fresh cache is not another child serialization attempt.

## Smells

None.

## Nitpicks

None.

## Not found

No additional correctness, panic or namespace-preservation defect was established in the occurrence-range repair. Drawing-bearing frames remain checked fallbacks. Error-order tests remain in the native batch.

The corrected native batch actually executed 6 tests: 5 passed and 1 failed in 27.77 seconds. D1's direct ordered-vector regression passed. The pairing gate failed. An earlier command selected zero tests and is not evidence. The installed Python extension remains `2d229a76114df218d07e70a9d12bce9e26ae6ec88b5fc441200f1b2891cf720c`. No new Python build, runtime gate, complete feature review or final sprint pass is claimed.
