# F-X268, native child writes, pass 2

**Reviewed**: Codex root reviewed D1, D2 and S1 remediation on worker HEAD `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. The native and binding two-file diff has 497 added and 13 removed lines. This is the bounded mechanism 7A write review.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Resolved findings

D1 is resolved. The serialized-child note occurs before each body_item_xml attempt. A whole-writer fallback has its own note. main_part_xml no longer records only a returned successful total, and the unused count is removed from the private result. The actual five-child retry probe reports 6. The root invalidation case with field, tab and bookmark records N+3. Output remains equal to the whole writer.

D2 is resolved. Debug-only counter calls and dependent assertions are configuration-gated. Functional writes, byte comparisons and resource-error checks remain active in release. The current release test target type-checks.

S1 is resolved. Each changed warmup is followed by its own write. Two later changed samples are measured separately. The mixed window requires exactly 4 writes at both 50 and 200 paragraphs, rather than a lower bound that accepted rewriting every neighbor.

## Evidence and limits

`/tmp/fx268-m7-write-remediation-test.log` records 10 passed and 0 failed in 23.46s, including the seven addressing regressions and the two probes. The scoped check and clippy logs report success. `/tmp/fx268-m7-write-remediation-release.log` is a successful release type-check, not a release test execution.

The quoted-angle root concern stays refuted by the actual escaped writer output and complete range. It is not a finding.

## Not found

No additional output-byte, raw-preservation, resource-admission, history-chain or structural finding was established in this write-only scope. The OXML prerequisite has its separate clean review. The holds_field binding flag and positional alignment retain the existing model guards.

Child-read staging, first-four-byte-slot bounds, complete A10, current release behavior, Python rebuild, HLD, composition, memory and final gates remain pending. Temporary reader fields and dead-code allowances must receive actual consumers before final mechanism 7 acceptance. This is not a mechanism 7 checkpoint.
