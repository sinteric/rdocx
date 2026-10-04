# F-X168, Current issue and contribution closure evidence

**Status**: completed
**Sprint**: S84
**Size**: M
**Depends on**: F-X149, F-X158, F-X160, F-X161, F-X162, F-X163, F-X164, F-X165, F-X166, F-X167, F-X169, F-X170

## Problem

The nine open issues through the 3 October refresh have different remaining contracts, including the
umbrella's two fixture workflows, namespace-safe edited packages, exact line
breaks, shape visibility and revision acceptance
(`docs/sprints/SPRINT_PLAN.md:1807`). Issue 264 adds a high-level style API
contract. The S82 ledger still names Issues 158,
160 and 226 as unresolved (`docs/hld/12-testing-strategy.md:3452`). A related
PR or a focused test alone cannot close any of these issues.

## Spec reference

- `docs/hld/14-development-backlog.md`, "F-X168, Current issue and contribution closure evidence".
- `docs/hld/12-testing-strategy.md`, "The Word corpus", "The deck corpus", "Binding tests" and "S82 original issue closure ledger".
- `docs/hld/10-bindings-spec.md`, "CLIs" and "Python bindings".

## Approach

On the final integrated S84 result, run the exact acceptance cases in the
nine-issue matrix and link each criterion to executable or performed manual
evidence. Review the contributor comments and the dispositions of all 32 PRs.
PR 265, opened on 3 October, offers the same section ownership fix as the
completed F-X162. Adopt its distinct multipage, mixed-orientation table and
content-control regression on the integrated implementation before disposition.
Prepare individual, human-written closure explanations for `/close-sprint`.
That command merges the fully verified sprint to main and then closes only
items whose complete criteria pass. Recheck GitHub after its reconciliation
step and require zero open PRs and zero open issues through the final refresh.

## Rejected alternatives

- Close an issue when its associated PR merges. Several issues require fixture,
  viewer or cross-surface evidence beyond one PR.
- Run a full workspace gate after each F-ID. Scoped dependency checkpoints and
  one final integrated gate provide the required evidence with less repetition.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| integration | `test_issue_158_word_fixture_acceptance`, `test_issue_158_complete_word_workflow`, `test_issue_158_deck_fixture_acceptance` | Both SHA-bound fixtures and complete Word and deck workflows pass |
| regression | `test_issue_159_identity_matrix_across_operations`, `test_issue_160_producer_matrix_across_operations_and_picture` | The full 18 by 7 and 11 by 8 matrices still pass |
| integration | Issue 226, 244, 245, 253, 254 and 255 gates named by F-X161 through F-X167 | Every remaining criterion passes on the combined result, including viewer checks |
| round-trip | Issue 264 style API example and F-X170 gate | Common style formatting survives save and reopen and applies to content |
| regression | PR 265 mixed-section fixture on integrated F-X162 | Multipage portrait and landscape sections, tables, margins and content controls match isolated section renders |
| integration | `python3 scripts/hash_harness.py --check`, `/verify --full`, `/sprint-review S84` | The final HEAD has explained baseline changes and a clean integrated review |
| integration | After `/close-sprint`, compare live GitHub state with the 3 October refresh | All 32 PRs and nine issues are closed with linked evidence |

**Test gate**: integration. The worker prepares criterion-level evidence and
all PR dispositions, runs the scoped gate and records the final integrated
checks for the sprint integrator. The S84 boundary then runs full verification
and sprint review. After `/close-sprint` merges the reviewed result to main,
close all nine issues and 32 PRs with links to the verified evidence.

## HLD impact

- `docs/hld/12-testing-strategy.md`, update the live acceptance gate after
  the integrated S84 evidence. Preserve the historical S82 ledger.

## Risk routing

- External oracle comparison: read `.claude/skills/differential-testing.md`.
  Pin and record python-docx, python-pptx, LibreOffice, Word and PowerPoint
  versions used by the individual gates. Compare trees for model parity and
  deterministic-font renders at declared DPI and tolerance.
- Layout regression from PR 265: read `docs/hld/08-rendering-spec.md` and use
  deterministic fonts for its mixed-section comparison. Preserve the F-X162
  section ownership implementation while adapting the test.

## Hash harness

No new delta belongs to this evidence story. Reconcile the separately labelled
F-X161, F-X162 and F-X163 changes on the final integrated result.

## Implementation checklist

- [x] Run both complete fixture workflows and both acceptance matrices.
- [x] Run each remaining issue gate and inspect the pinned manual viewer evidence.
- [x] Adopt and run PR 265's distinct mixed-section regression on integrated F-X162.
- [x] Record criterion-level evidence and all PR dispositions for the sprint
  integrator, with no unresolved worker gap.
- [x] Pass scoped verification and a zero-finding microscope for this evidence
  record, then hand off the final integrated full gate and sprint review.
- [x] Prepare individual closure explanations for `/close-sprint` to use only
  after the verified result reaches main.

## Open questions

None. Full closure of the current open queue is the user's S84 exit gate.
