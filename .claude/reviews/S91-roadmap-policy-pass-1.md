# S91 roadmap policy, pass 1

**Reviewed**: commit e211fe6e92986e21a01da9c10f85f486f93b6c3c against68a9fb59. Only scripts/test_sprint_workflow.py, one insertion and one deletion. Scope is the roadmap sprint-set assertion, not F-X196 source or its prior review.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Evidence

The sole change at `scripts/test_sprint_workflow.py:7947` changes the exclusive range end95 to96. It adds S95 to the expected occupied sprint set while retaining S89 through S94. The inserted S91 and shifted later live sprints require that exact occupied set. The canonical sprint plan records the shifted boundary at `docs/sprints/SPRINT_PLAN.md:2192`.

A read-only standard-library parser examined the immutable e211fe6e planning records. All71 audited F-240 through F-310 stories have exactly one plan placement and one backlog row, with matching sprint, title and size, matching HLD title and size, and no archived audited row. The occupied set exactly equals S70 through S74, S86, and S89 through S95.

| Sprint | Audited stories |
|---|---|
| S70 | F-240 through F-242 |
| S71 | F-243 through F-249 |
| S72 | F-250 through F-256 |
| S73 | F-257 through F-263 |
| S74 | F-264 through F-270 |
| S86 | F-271 through F-273 |
| S89 | F-274 through F-277 |
| S90 | F-278 through F-282 |
| S91 | F-283 |
| S92 | F-284 through F-290 |
| S93 | F-291 through F-296 |
| S94 | F-297 through F-303 |
| S95 | F-304 through F-310 |

The commit leaves the duplicate HLD-heading check unchanged at `scripts/test_sprint_workflow.py:7917`, audited-story membership unchanged at7919, nonempty/unique placement assertions unchanged at7943/7944, exact backlog row uniqueness and cross-record equality unchanged at7964/7976, HLD equality unchanged at7977, and archived exclusion unchanged at7978. Dependency target membership remains checked at8000, sprint order at8006, and cycle detection at8015. No dangling, duplicate, dependency or backlog safeguard is removed or relaxed by expanding the occupied sprint set.

## Not found

Correctness, contract and test-strength review found no issue in this one-line delta. No new panic path, OOXML behavior or structural abstraction is introduced. No Cargo, repository-policy suite or UI action was run. The placement computation is read-only evidence, not a claim that the full suite passed.
