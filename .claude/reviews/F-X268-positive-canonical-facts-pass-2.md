# F-X268, positive canonical facts, pass 2

**Reviewed**: Codex root reviewed D3, S1 and S2 remediation on worker HEAD `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. The current three-file working diff contains 3180 added and 153 removed lines, including earlier reviewed work. This is the bounded positive canonical and parallel identifier fact review.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Resolved findings

D3 is resolved for these new consumers. The shared closed_word_body policy uses the existing story-owner scanner and single_frame_body coverage check. Canonical and identifier child fills require one Word body covering the complete child range. Their frame fills also require the insertion gap to lie in that one body. Cached child facts must match the current checked frame, including its head and tail bytes, so a changed opaque frame cannot reuse a stale proof merely because namespace vectors happen to match.

The prefixed and default-namespace raw cross-body regressions now decline the shortcut. The old raw writer remains unchanged, the whole scanners retain their relationship and drawing values, and the full document parser still rejects the unsupported two-body form. The valid closed-parent control agrees with the old scanners on both fill and hit. This is fallback and error preservation, not a new schema-admission policy.

S1 is resolved. The unused child empty-proof field and its two obsolete methods and initializers are removed. The frame empty proof remains because the positive path uses it. No dead-code allowance was added.

S2 is resolved in the new composer. A HashSet tracks membership and the output vector preserves first occurrence. Each incoming relationship performs one membership insertion rather than scanning every previously emitted id. Its private work note identifies actual membership operations, without claiming to count internal hash-table probes. Full A9 work measurement remains pending.

## Evidence and limits

The diagnostic pre-fix RED remains `/tmp/fx268-m7-cross-body-scope-probe.log`, 0 passed and 1 failed in 0.02s. `/tmp/fx268-m7-positive-remediation-test.log` records 3 focused tests passed in 0.09s and the non-diagnostic batch, 23 passed and 0 failed in 66.90s. Those behavioral runs precede configuration and lint-only cleanup.

`/tmp/fx268-m7-positive-remediation-check2.log` records check success. The current release test targets type-check successfully in `/tmp/fx268-m7-positive-remediation-release2.log`. Scoped clippy succeeds in `/tmp/fx268-m7-positive-remediation-clippy2.log`. The debug-only diagnostic vector now compiles in release, borrowed paragraph results are discarded without drop misuse, shared drawing-slot aliases reduce signature complexity, and the earlier release-only child_count warning is removed. No release behavioral executable has run.

The last original A10 table-row observation still has growing namespace scan work, with canonicalization work zero. That known diagnostic is not treated as passed or xfailed. Section and list cases have not executed. Partial scope-eligibility input is recorded separately and does not change the six approved staging units. Identifier inventory input units remain unchanged and do not claim to sum every internal parser pass.

## Not found

No additional scope, cached-context, relationship-order, drawing-selection, span, raw-admission, fallback-error, counter-unit or structural finding was established in this remediation inspection. The new aliases and guard stay private. Replay authority and the force-full reference remain intact.

Namespace reuse, complete original A10 coverage, Python rebuild, HLD, full composition, mechanism 8, release behavior, timing, memory, Word and final gates remain pending. The installed extension remains the mechanism 6 build. This is not mechanism 7 acceptance or a whole-feature microscope.
