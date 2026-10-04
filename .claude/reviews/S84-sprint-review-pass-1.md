# S84 sprint review, pass 1

**Reviewed**: `sprint/s84` against merge base `511c10a2d08e6b6d617aa9150b6e78adbe7d7afe`, 102 files, 7,824 changed lines, crates: `oxml-layout`, `rdocx`, `rdocx-cli`, `rdocx-html`, `rdocx-layout`, `rdocx-oxml`, `rdocx-py`, `rpptx`, `rpptx-layout`, `rpptx-oxml`, `rpptx-py`, `rpptx-render`
**Verdict**: 0 blocking, 0 should-fix, 0 nice-to-have

## Blocking

None.

## Should-fix

None.

## Nice-to-have

None.

## Milestone gate

S84 is the cross-cutting X repair wave. Its definition of done in `docs/sprints/CURRENT_SPRINT.md:48` requires the issue contracts, contributor dispositions, separately reviewed baseline deltas, fixture matrices, full verification and sprint review. The GitHub closure clause is explicitly after `/close-sprint` merges to main. The source and evidence gate holds on integrated commit `a2aea4d21d81`: the full workspace suite, 175 Word and 77 presentation Python tests, deterministic 49-entry hash harness, golden pixels, formatting, Clippy, prose and adapter checks, workflow tests, minimal-feature layout tests, WASM checks, documentation tests, publish dry run and dependency audit passed. The report and deck fixtures, 18 by 7 identity matrix, 11 by 8 producer matrix and PR 265 mixed-section regression are covered by the integrated tests. Word for Mac 16.113.2 accepted and rejected all 80 audited revision outcomes. PowerPoint 16.113.3 and LibreOffice 26.2.5.2 displayed the new-shape theme result. The issue-by-issue evidence and post-main PR dispositions are in `docs/hld/12-testing-strategy.md:3460` and `docs/sprints/SPRINT_PLAN.md`.

F-X161, F-X162 and F-X163 made three separately reviewed hash changes. Their integrated deterministic baseline at `scripts/hash_baseline.json:2` matches 49 of 49 entries. F-X163's two changed golden pixels are declared at `scripts/golden_pixel_manifest.json:4`. The PR 265 mixed-orientation test at `crates/rdocx-layout/src/engine.rs:17597` checks multiple pages, text, dimensions and line extents against isolated sections, including content controls. It fails on the old section-selection implementation and passes on the integrated one.

## Not found

Interaction, duplicate helpers, layering, undeclared hash changes, gate adequacy, stale HLD, new dependencies and unrequested public surface produced no finding. No crate manifest or lockfile changed. The accepted-view paragraph and row projection, section geometry and plain-space fit coexist under the full integrated suite. GitHub closure remains for `/close-sprint` after the reviewed main merge.
