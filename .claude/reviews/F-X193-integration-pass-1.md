# F-X193 integration reconciliation, pass1

**Reviewed**: Staged squash onto canonical sprint HEAD9bba77d7. Independent integration review of the approved F-X193 and F-X196 contracts, exact worker source identity, scoped handoff/reviews/receipts and the two resolved shared-record conflicts. No Cargo, source edit or gate rerun.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Evidence

Production source: read-only git comparison against work/f-x193-codex returned no diff for crates/rdocx and README.md. Comparison against preintegration HEAD returned no diff for crates/rdocx-cli. Thus the squash preserves the independently reviewed HTML worker source and the already integrated CLI source. The semantic field at `crates/rdocx/src/html.rs:175`, inheritance at196, final mark cascade at3335 and child-first highlight selection at3535 retain the approved behavior. Regression controls at3905,3926 and3956 distinguish child backgrounds, nested marks, ordinary background scope, unmarked siblings and opaque-source round trip. No production semantic conflict was resolved during integration.

Measurement reconciliation: `scripts/readme_doctests.py:405` retains the X193 rdocx observation3525710 compressed bytes,50767756 normalized member bytes,37 members. `README.md:42` matches. The worker progress record states two actual identical archive rebuilds and successful existing validation. Independent F-X193-all-pass-2.md confirms actual archive inspection. Adjacent `scripts/readme_doctests.py:406` retains the X196 CLI observation76692/347960/8, matching `crates/rdocx-cli/README.md:28`. Independent F-X196-all-pass-2.md recorded76691 compressed bytes on inspection with identical normalized members/count and the unchanged64-byte compression tolerance. These are distinct package inventories with byte-identical source retained, not a choice of one worker's full measurement dictionary over another. The staged script diff changes only the rdocx tuple. Final integrated archive verification remains a subsequent integrator action, not a pass asserted by this review.

Policy strength: `scripts/readme_doctests.py:394` compression tolerance and existing inventory validators are unchanged by the staged diff. There is no manifest, lockfile, version, publication, source inventory, baseline or scripts/test_sprint_workflow.py change in this squash. The previously independent S91-roadmap-policy-pass-1.md remains applicable to the separate inserted-sprint policy correction. No additional assertion was relaxed to combine the package records.

Plan reconciliation: completed checks at `.claude/plans/F-X193-design.md:95` through97 correspond to the actual scoped evidence and prepared handoff. `.claude/handoffs/F-X193-ready.md:17` records library514, integration364, regression2331 and doctests2, with documented CI8MiB stack and unchanged consumer timeout. Its policy statement at22 accurately distinguishes recovered terminal unittest success from unavailable tool-session exit after daemon restart. Read-only SHA-256 verification matched the retained policy and hash logs to S91-F-X193-hash-policy-receipt.json. The policy log explicitly states141 tests in496.024s, OK with2 skips. The X196 corrected policy log separately states141 tests in500.696s, OK with2 skips. No default-stack abort, interrupted attempt, contributor claim or unavailable exit was relabeled as a successful process exit.

Hash and gate boundary: the X193 receipt retains actual hash exit0 with49 matches and no baseline change. The prepared handoff is feature evidence. It does not substitute for final integrated full sprint verification or sprint review. This integration pass writes only this review file and ends here.
