# F-X222, Restore linear Word story range inventory and empty owners

**Status**: completed
**Sprint**: S91
**Size**: M
**Depends on**: F-X203, F-X213
**Issues**: 362
**PRs**: 363
**Test gate**: regression

## Problem

The per-story source lookup at crates/rdocx/src/document.rs:18978 rebuilds source XML and owner scans for every cell or text box. Each paragraph scope is scanned from offset zero. The cache projection at document.rs:18735 repeats source and owner discovery, and the evaluator at crates/rdocx/src/field.rs:13105 independently lists ranges and paragraphs. Issue362 reports quadratic scaling in field and TOC operations. A valid self-closing comment can also run the old owner scan beyond its excerpt and fail before page updates.

## Spec reference

- docs/hld/03-architecture.md, Field evaluation and Physical source identity, checked physical owner/cache binding.
- docs/hld/04-opc-and-packaging.md, Package integrity and preserved unmodelled XML, exact owned source and namespace scope.
- docs/hld/10-bindings-spec.md, Word story snapshots and operation-scoped locations, unchanged identities and live handles.
- docs/hld/12-testing-strategy.md, Regression and deterministic hash harness, genuine equivalence and measured performance.
- docs/hld/14-development-backlog.md, F-X222.

## Admission and sequencing

The user explicitly folds new PR363 and its linked Issue362 into S91. Formal dependencies are completed. F204 currently owns overlapping Word source and releases it before this implementation begins. This is a new repair, not a revision of completed F203 or parked bibliography/navigation. Release preparation F221 also waits for F222. No automatic intake loop is opened.

Full metadata, issue, comments, reviews and two commits are retained at /private/tmp/S91-intake-admission-362-363. The full five-carrier patch SHA is c44653c9f81352bbe25f60d29a9ca5b43a6eeedb7f6420ba4cc0342e1db38b3d. Applying it to actual base733ce6f7 reconstructs all five immutable head Git blobs exactly, qualified by immutable-patch-qualification.json SHA7e89a0a3cdd4b2ee86f4587eb6d89632cf5e6c81574e0276bde8209907007409. Contributor timings and gates are inputs, not local qualification.

## Approach

Reuse the assessed native donor slice in existing document.rs, comments.rs and field.rs. Adapt it to the integrated source rather than importing the older whole facade. Keep source materialization and owner discovery operation-scoped. Preserve stories() order, identity, accepted projection and cross-source de-duplication. Batch owner/control/paragraph namespace scopes per source, map excerpt positions back with checked bounds, and share located source/span facts with text-box cache projection. Existing single-location mutation callers retain their checked coordinates and refusals.

Split existing range pairing into a private concrete helper taking the already-owned paragraph listing. Document::story_ranges and Evaluator::new are the two actual consumers. Range-pairing failure and bookmark-name counting remain independent, preserving existing accepted fallback behavior. Do not add a global cache, index axis, parser, revision engine, public type or dependency.

A valid empty qualified owner yields no paragraphs while its comment record and opaque bytes remain present. Foreign lookalikes and malformed scopes do not become editable owners. Record this empty-owner behavior repair in its own labelled commit. Previously successful ordinary output remains byte-equivalent, with source input untouched and held public owners valid.

Use existing inline native and integration/Python/CLI test carriers. Retain the old algorithm only as a test reference, covering body, cells/nested tables, controls, text boxes, headers/footers, notes/comments, fields/bookmarks and local/rebound aliases. Authenticate before/after saved package parts, not just visible text.

Measure release-mode same-host Base and result on the issue reproduction at scale1/2 and larger discriminating table workloads. Record actual commands, input/output/source hashes, toolchain, warm-up and repeated elapsed samples. Restore the reported pre-regression level with material improvement over the same-source Base, and qualify near-linear doubled-input scaling with unchanged field/TOC count. Separately report doubled TOC-field costs, remaining per-bookmark REF lookups and single-location scans. These donor omissions are not a claim that every document operation is globally linear. If Issue362's actual reproduction remains quadratic, fix the responsible in-scope path before completion.

## Rejected alternatives

- Repeated full-document serialization or a global mutable cache hides invalidation and preserves the regression.
- Local-name-only scanners or flattened paragraph text lose physical ownership and namespace semantics.
- Copying donor archive tuples or reported timings cannot qualify integrated source.
- Expanding parked navigation or bibliography duplicates deferred work.

## Test plan

**Test gate**: regression.

| Category | Test | Asserts |
|---|---|---|
| regression | fx222_inventory_builds_each_source_once | Large table, range listing, range pairing, cache listing and evaluator reuse, actual source/scan counters |
| regression | fx222_inventory_matches_previous_successes | Every story family, nested controls/tables, shared header/footer sources, fields/bookmarks/comments, aliases and exact output/order/coordinates |
| regression | fx222_empty_owners_and_invalid_scopes | Valid empty comments survive public listing/update/save/reopen, foreign owners and malformed scopes/offsets refuse safely |
| regression | fx222_public_fields_toc_and_comments | Actual installed Python and CLI operations, current F204 report retained, inputs/output graphs and live owners unchanged on failures |
| performance | fx222_release_scaling_and_output_equivalence | Same-host measured Base/result, large-table scaling and byte-equivalent successful saved outputs |

Run changed Word native/default/all-target checks, focused tests and affected complete native suites, Clippy, formatting, both WASM graphs and49 deterministic hashes. Use a fresh authenticated Word wheel for actual installed tests, strict mypy/stubtest, retaining the exact current PPT distribution. Verify the affected Word archive with local patches, exact inventory and10MiB bound, then update only its actually measured README tuple/policy values. Independent full microscope and normal scoped verification remain mandatory. No new product test binary or tracked fixture is needed.

## HLD impact

- docs/hld/03-architecture.md
- docs/hld/04-opc-and-packaging.md
- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md
- docs/hld/14-development-backlog.md

## Risk routing

Parser/serializer: HLD04/06, actual namespace ownership, excerpt offset bounds, empty-owner behavior and preserved opaque XML. Published API behavior: HLD10 and structural rules, source-compatible private helpers, verified actual Word archive and10MiB ceiling. Existing PyO3/WASM consumers: fresh Word provenance/typing and both WASM graphs. Performance/field layout acceptance uses deterministic fonts and honest per-operation measurements. No new graph edge, file/module/crate/dependency/asset is authorized.

## Hash harness

All49 existing sample outputs must remain unchanged. Successful old/new field and TOC saved output is byte-equivalent. Empty-owner support changes only its new distinguishing fixture. No baseline rewrite is authorized.

## Implementation checklist

- [x] Capture genuine Base failures, exact reference outputs and release timings.
- [x] Adapt the three assessed native carriers and existing callers.
- [x] Qualify source-build counts, exact ownership/equivalence, empty comments and public field/TOC paths.
- [x] Measure performance and actual archive, preserving original failed receipts.
- [x] Obtain clean independent microscope and scoped verify, update listed HLD and prepare handoff.

## Open questions

No blocking question remains. The user explicitly admits this repair. Existing identity, source preservation and error contracts remain controlling. Implementation waits for the overlapping source lease, and no new dependency, asset or public API is required.
