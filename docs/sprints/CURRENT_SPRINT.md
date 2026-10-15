# Current Sprint, S91.1

**Milestone**: M24, X, release runtime and CI repairs.

**Goal**: Fix the native CI setup, default-WASM parser and checked chart relationship failures found after S91 merged, then deliver the original first-draft corpus CI criterion. Requalify the still-unpublished Word0.16.1 and shared/PowerPoint0.14.1 families, preserving the frozen40-issue/30-PR scope and open discussions264/281.

## Spec references

- `docs/hld/04-opc-and-packaging.md`, package identity and verbatim opaque XML.
- `docs/hld/06-presentationml-model.md`, shape-tree parser storage and source preservation.
- `docs/hld/10-bindings-spec.md`, actual default-stack WASM facade and package round-trip.
- `docs/hld/12-testing-strategy.md`, regression, binding and exact source/oracle verification.
- `docs/hld/15-build-and-toolchain.md`, pinned tools, CI job matrix and closed-main releases.

## The wave

| F-ID | Title | Size | Status | Owner |
|------|-------|------|--------|-------|
| F-X224 | Repair hosted release tool and stack routing | M | done | - |
| F-X225 | Keep DrawingML text-style parsing within the default WASM stack | M | done | - |
| F-X226 | Release superseded relationships after checked PowerPoint XML edits | M | done | - |
| F-X227 | Execute the unchanged first-draft compatibility corpus in owner CI | L | done | - |

## Sequencing note

All prerequisite features are already done in S91. F-X224 through F-X226 implemented in parallel in separate worktrees with disjoint source leases. F-X227 is implemented and independently reviewed, with actual external acceptance now qualified. ROOT owns shared records, integration and final artifact qualification. No S92 feature work or new contributor intake is included.

## Current qualification

F-X224 through F-X227 are implemented and independently reviewed. Scoped completion records bind their actual worker and integrated evidence. Corrected primary CI38708235000 is SUCCESS at 6d7c7da729e0ecb34252e821341f22844e37a9fd, with receipt /private/tmp/S91.1-primary-corrected-raster-acceptance-1/terminal-receipt.json and SHAad03f29ef28fe9a5174cdb7cb30a8811c4cb25fa237eac4df1b8a0d63d388792. The successful terminal results and all18 job logs were authenticated before completion.

Local integrated b3b qualification passed6463 Rust tests,778 Word and854 PowerPoint Python tests,104 font cases, eight native SVG owners, two F216 controls and Node2/2/3 at the unchanged1MiB stack. Source215 verification passed all15 core stages and supply-chain checks. Its exact-source build-only rehearsal38696969789 passed27 jobs with publication jobs intentionally skipped,12 wheels, two source distributions,12 CLI archives,156 installed priority checks and four typing/stub checks. Complete relevant source/body/mode and artifact bridges preserve those actual producer identities rather than calling them new executions.

Actual owner CI38702668217 atsource215 completed all100 original first attempts with safe cleanup and no timeout. Independent admissions qualify95 correct files,42 hard gates,51 native clauses and24 documentation cases. Per-area counts are BASE24/24, XML13/14, LIFE16/16 and AREA42/46. Five approved non-successes remain in the denominator without correct-file credit. All114 unaffected saved files match historical bytes and the three revised-input cases have fresh observations. Historical native and app witnesses retain their original provenance. The one-job runner was removed with API404 and reaped process groups.

Original main CI38662802341 and primary CI38696954013 remain failed. The latter passed its full native, binding, MSRV, animation and fidelity consumers but failed three stale raster expectations omitted from the approved F-X210 change. Their attributed correction leaves all49 product hashes, four unrelated rasters and exact thresholds unchanged. Failed preparers, refused assignments and missing historical S92 oracles remain separately recorded.

Final reviewed-source full verification of the final reviewed sprint commit and clean sprint review remain due. Normal close, exact-main full verification, both family publications and acceptance-aware reconciliation of the70 included records remain pending. Discussions264 and281 stay open, and S92 deferrals remain unchanged. Issue360 may close only after publication under the approved reviewed-submission delivery boundary, with skills PR37 merge tracked externally. Registry roles, trusted publishers, exact final notes and release URLs must be revalidated at release.

Advanced PR359 remains partially addressed, with new PowerPoint HTML import
whitespace acceptance assigned to the separately planned repair sprint. Its
release-bound comment must state that remaining scope and leave the PR open.
Discussions264,281 and377 remain open. No new intake enters S91.1.

## Definition of done for this sprint

- Preserve original main CI38662802341 failures and exact Node24.11.1 reproduction.
- Pass existing native font/animation consumers and exact executable fractional release-tag controls.
- Pass the unchanged real PowerPoint Node facade under the default1MiB stack, with public APIs, opaque XML, child order and parser bounds intact.
- Pass the original public chart alias edit in debug and optimized builds, preserving validation, sibling graphs and atomic refusal.
- Complete normal scoped lifecycle, zero-finding independent microscopes, fresh integrated full verification and clean sprint review.
- Qualify actual current paired Rust/CLI/Python artifacts,39/312 installed runtime/typing, font104/native-eight riders, primary hosted CI and exact-SHA build-only rehearsal.
- Run the unchanged first-draft corpus in actual owner CI with honest per-area rates under the approved F-X227 contract.
- Close normally without changing S91's tag. Publish both prepared families only after exact-main checks, then reconcile the70 humanized comments. Discussions264 and281 remain open, and all existing S92 work remains deferred.
