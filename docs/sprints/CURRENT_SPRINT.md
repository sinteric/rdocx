# Current Sprint, S92

**Milestone**: M24, X, contribution and issue repairs.

**Goal**: Resolve the planned contribution and issue repairs against the published S91.1 result. Qualify faster CI, finish clearable run formatting, repair comparison reconstruction and readable revision grouping, and preserve PowerPoint HTML import whitespace. Restore linear Python table-owner inventory for newly reported Issue378. The six stories total 15 to 18 developer-days of estimated effort.

## Spec references

- `docs/hld/14-development-backlog.md`, F-X228, F-X178 and F-X229 through F-X231 and F-X237 define the repair contracts, prerequisites and story gates.
- `docs/hld/02-scope-and-non-goals.md`, HTML slide import and Word interchange boundaries retain bounded supported constructs and explicit diagnostics.
- `docs/hld/04-opc-and-packaging.md`, package ownership and relationship identity govern comparison reconstruction and save/reopen preservation.
- `docs/hld/10-bindings-spec.md`, Word formatting, tracked revision resolution, comparison options and native presentation HTML import define the current public surfaces.
- `docs/hld/12-testing-strategy.md`, comparison reconstruction, source-preservation regressions, installed binding checks and the CI job matrix define required evidence.
- `docs/hld/15-build-and-toolchain.md`, CI routing, pinned native tools and build profiles constrain performance improvements without weakening gates.

## The wave

| F-ID | Title | Size | Status | Owner |
|------|-------|------|--------|-------|
| F-X228 | Qualify faster CI without weakening gates | M | done | - |
| F-X178 | Clearable direct run formatting setters | S | done | - |
| F-X231 | Preserve PowerPoint HTML import whitespace | M | done | - |
| F-X229 | Repair comparison reconstruction correctness | L | done | - |
| F-X230 | Group comparison changes for readable revisions | M | done | - |
| F-X237 | Restore linear Python table owner inventory | L | done | - |

## Sequencing note

Rows are listed in dependency order, not F-ID order.

Qualify and integrate F-X228 from PR376 first using actual current-head CI. Preserve every required suite, pinned native setup, debug assertions, overflow checks, binding profile and owner-only dispatch boundary. Contributor timing claims alone are not acceptance evidence.

F-X178 depends on completed F-X200 and resolves Issue364 through the remaining width-scale and position reset forms. F-X231 depends on completed F-X217 and finishes advanced PR359's PowerPoint emphasis-edge spaces, span-boundary whitespace and NBSP. Qualify equivalent delivered Word cases once. Confirm applicable native and installed entrypoints during design against the current public surface.

F-X229 depends on completed F-X203 and resolves Issues368 through371 using PR374's correctness subset. F-X230 starts only after F-X229 completes, then resolves Issue372 with the separately attributed grouping changes. Reconstruction correctness must remain established while revision grouping changes intentionally.

CI, setters and HTML import can proceed in separate claimed worktrees. Explicitly lease shared native sources and integration test entrypoints. Rebase or extract minimal contribution slices against the published S91.1 result, reconcile semantic conflicts against both contracts and avoid duplicating delivered fixes. Only one story may own a deterministic baseline change in a wave.

S93 retains rendering, border, LaTeX, equation and setter-inlining work. Bibliography, navigation, standalone authoring and the later benchmark and Excel programme retain their planned sprint assignments. Discussion issues264,281 and377 remain open.

Issue378 was explicitly added during execution on2026-10-15. F-X237 restores linear table open and single-edit work, with authenticated release0.16.0 timing comparison and the exact40-by6 fill. It proceeds in a separate worktree from comparison grouping, with shared Word facade and package measurement carriers reconciled explicitly. This adds no Issue316 implementation scope.

## Definition of done for this sprint

- Complete all six approved story contracts through design, implementation, scoped verification and independent zero-defect, zero-smell microscope review.
- Qualify actual integrated CI improvements from PR376 without dropping suites, native oracles, setup, assertions or overflow checks.
- Prove explicit set/clear and save/reopen behavior for Issue364, including absent-property and existing explicit-false controls.
- Prove accept-all and reject-all reconstruction for Issues368 through371, preserving properties, relationships, tables, stories and opaque XML. Prove Issue372 grouping respects fields, comments, bookmarks, hyperlinks and formatting boundaries.
- Prove PR359's remaining PowerPoint HTML whitespace cases through applicable public entrypoints and save/reopen, retaining diagnostics and existing semantic export acceptance.
- Prove Issue378 linear cell inventory and route matching, same-host open/edit timing acceptance and the explicitly accepted measured fill tradeoff, physical owner and stale-generation preservation, atomic refusal and full fresh Word Python acceptance.
- Pass the integrated full verification and sprint review, including applicable installed bindings and native/oracle controls. Attribute every intentional deterministic hash or raster delta separately and retain unchanged controls.
- Close issues and contribution PRs only against their full criteria and required publication evidence. Only /close-sprint merges to main, and only /release publishes an approved reviewed SHA. Discussions264,281 and377 remain open.

The sprint clock starts at its first /start-feature. All stories are pending and unclaimed at opening.
