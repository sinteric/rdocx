# F-X214, Bound recursive PowerPoint parsing and traversal

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: none
**Issue**: 344
**Test gate**: regression
**Implementation**: claimed on work/f-x214-codex at639d354eeee67592bb32da6fd62cb88b7d900530. Current parser and traversal body is source13. Scoped installed, native, archive and repository-policy qualification is complete.

## Problem

`crates/rpptx-oxml/src/shape_tree.rs:1868` captures every group child before `capture_group_child` dispatches it through `parse_shape_tree_child` at2047. Nested groups and compatibility content reparse captured descendant subtrees. `timing.rs:1442` captures each list child and reparses it at1484. Neither cycle has a shared checked depth boundary. The issue's measured process crashes are reported evidence, not locally reproduced acceptance.

Downstream `rpptx-layout/src/context.rs:3127` recursively searches group lineage, and:3320 recursively collects timeline group geometry. These consumers must not assume that authoring always passes through a bounded parser. The parsed tree and the renderer group graph need the same safety contract.

## Spec reference

- `docs/hld/06-presentationml-model.md`, The shape tree, required group children and Relationship remapping.
- `docs/hld/04-opc-and-packaging.md`, The package and configured decoded-size limits.
- `docs/hld/08-rendering-spec.md`, Why Group is the whole design and deterministic renderer boundaries.
- `docs/hld/10-bindings-spec.md`, Python API shape and Native PowerPoint timing model.
- `docs/hld/12-testing-strategy.md`, Test taxonomy, The deck corpus and Binding tests.
- `docs/hld/14-development-backlog.md`, F-X214 at7555, size/dependencies/test gate copied exactly.

## Approach

Add one named `MAX_PRESENTATION_NESTING: usize = 32` constant to existing `rpptx-oxml/src/lib.rs`. This proposed conservative default covers the recursive group, AlternateContent and timing ownership cycles. Count a root cycle owner as depth1, allow depth32, refuse before allocating or descending into depth33. Mixed group and AlternateContent chains share the shape-tree counter rather than resetting at a different element kind. Timing child nodes use the same numeric constant. The checked error includes the actual part or operation, cycle, limit32 and offending depth. URI ownership determines recognized nodes. Foreign same-local-name payload remains foreign.

Replace capture-then-reparse for recognized recursive owners with a single event-reader traversal and explicit frames in the existing parser carriers. Parse each recognized owner from the current cursor once. Capture unsupported opaque subtrees once at their actual owned boundary and retain exact token bytes, namespace scope and sibling order. Do not copy each entire descendant subtree into every ancestor. Reuse existing ordered raw carriers and namespace machinery. If preserving a compatibility wrapper requires a private source span, retain one shared immutable source buffer and ranges in existing private preservation state, never a new public registry, module, trait or lifetime generic. Preserve the existing public model shape and equality contract where possible. Enumerate any unavoidable literal/source compatibility change before source admission.

Apply the bound to facade authoring and mutation before staged publication, including grouping, inserting nested compatibility content and setting timing trees. Iterative lineage, geometry collection and render lowering preserve source order, transform multiplication order and member diagnostics. At-limit trees built through supported authoring APIs must text-extract and render. Rejected mutations retain package bytes, IDs, revisions, relationships and live owners. A low-level manually constructed tree is validated before any supported facade walker. Do not promise unchecked arbitrary Rust constructors become a new safe authoring API.

Existing carriers: rpptx-oxml lib.rs/shape_tree.rs/timing.rs, rpptx-layout context.rs, rpptx-render lib.rs, rpptx lib.rs, current error adapters in rpptx-py lib.rs/presentation.rs if mapping is needed, rpptx-cli commands.rs, and their existing inline/integration/Python entrypoints. No new product file, module, crate, dependency or flag.

## Rejected alternatives

- A larger thread stack only moves the abort boundary and leaves quadratic parsing.
- Separate resettable limits allow mixed cycles to bypass the bound.
- Dropping AlternateContent or unknown children defeats source preservation.
- A stopwatch alone cannot prove linear work. Count event visits and retained source bytes as well.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| regression | `fx214_three_cycles_reject_limit_plus_one_without_abort` | Each cycle has valid native public setup, depth32 succeeds, depth33 returns the named checked error, CLI text/inspect/validate and isolated Python subprocess never abort or signal |
| round-trip | `fx214_nested_aliases_and_opaque_children_keep_source_order` | Alias, local rebound prefix, foreign lookalike, Choice/Fallback, comments and opaque siblings retain raw bytes and correct projection at all depths |
| regression | `fx214_mixed_cycles_share_one_bound` | Alternating group/compatibility owners cannot reset depth and refusal is atomic |
| unit | `fx214_recursive_parser_visits_each_owned_event_once` | Doubling bounded breadth/source size doubles bounded visits and retained bytes, no repeated recursive capture/reparse |
| regression | `fx214_authored_limit_tree_extracts_and_renders` | Supported authoring reaches exactly the limit, extraction and deterministic render preserve order, geometry and content, deeper mutation leaves source and handles unchanged |
| golden | `fx214_release_limit_open_extract_render_under_one_second` | Release executable measures actual open plus extraction plus render, no compilation in timer, fixed input/fonts and machine recorded, each cycle finishes well under1s with a target median below0.5s and every run below1s |

Append tests to existing entrypoints, never another integration binary. Genuine Base includes ordinary valid parser/render positives before a separately isolated past-limit child that demonstrates checked error versus actual abort. No compiler failure or missing corpus is a RED. The linearity test is deterministic operation counting and supplements the release timing requirement. Pin the existing deck corpus identity and run its unchanged parser/round-trip controls. Perform a native PowerPoint open/save/reopen on a source-built moderate legal nested deck, record actual app version and no repair. Native Office need not open intentionally over-limit hostile input.

## HLD impact

- docs/hld/02-scope-and-non-goals.md
- docs/hld/06-presentationml-model.md
- docs/hld/08-rendering-spec.md
- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md
- docs/hld/14-development-backlog.md

## Risk routing

Parser/serializer: read04/06, exact opaque capture and schema-order round-trip. Layout/render: read08, deterministic font mode and unchanged ordinary corpus output. Public low-level constant/API: read10 and structural rules, state additive pre-1.0 impact, verified affected archives and10MiB bounds. Python/error mapping: both WASM graphs, fresh binding wheel and installed isolated error controls, strict typing/stubtest if public signatures change. External oracle: pin application and python-pptx1.0.2 versions, no native-open inference. No dependency graph change, new feature flag or structural framework is proposed. Normal scoped changed-crate tests, lint, format, hash and repository policy remain required.

## Hash harness

Expected unchanged for accepted ordinary inputs and render output. Intentional behavior is rejection past the declared bound. Any measured ordinary output delta blocks completion until separately attributed, declared and reviewed. Never record a system-font baseline.

## Implementation checklist

- [x] Independently review the draft and approve the shared depth/counting policy.
- [x] Preserve genuine Base subprocess failures and valid positive setup.
- [x] Implement single-pass bounded parsing and safe authored traversal.
- [x] Prove raw retention, atomic refusal, linear work and release timing.
- [x] Run full changed-crate scoped gates, pinned corpus and native legal-deck checks.
- [x] Update only listed HLD files and obtain zero-finding working review.

## Open questions

No blocking product ambiguity identified. Limit32 and the explicit cycle-owner counting convention are stated proposed defaults for batch approval. Dependencies are none in the registered backlog. No execution or gate credit exists in this draft.

## Batch approval, 2026-10-11

The rolling intake instruction authorizes the full linked issue acceptance.
ROOT reviewed the source, dependencies, test gate, risk riders and HLD list.
The stated defaults are adopted. No blocking product question remains.
Implementation waits for completed dependencies and an exclusive resource wave.

## Rolling intake, PR355

The 2026-10-11 20:23 UTC refresh found [PR355](https://github.com/tensorbee/rdocx/pull/355) at head4005c4e51c3e2ce00d9c14dde1a2163fcaf8c5c1. It fixes Issue344 and joins the existing approved F-X214 parsing and traversal story. The exact contributor patch is7546f000296c4e6fedde1e7d860faea81ed2ebb145f432010b0e33d3d8e3fa4a in /private/tmp/S91-PR355-intake/diff.patch. Reuse its parser and regression work after reconciling the approved depth32 contract, URI ownership, mixed cycles, source retention and authored traversal safety. The donor depth16 limit and reported gates do not replace the approved acceptance or local verification. Read-only contribution assessment is in progress.

There are24 open PRs and38 included open issues. Discussion issues264 and281 remain excluded. No existing PR head changed. This intake adds no duplicate story and marks no implementation complete.

PR355 contribution assessment is complete at /private/tmp/S91-PR355-intake/assessment.md, SHAf8c78341918d25fe20908a0e1845eae23c0cc36f1d6c0ba7601e243b5246a15e. Reuse the same-reader parser mechanisms, iterative condition-target walk, empty timing-node error and eight contributor tests. Reconcile the donor local-name counting and omitted empty owners, recursive parsing and nested raw copies, depth16 policy, and incomplete grouping/manual-tree traversal guards with the approved F-X214 contract. No contributor archive measurements or reported test results are adopted as local evidence. Implementation remains at its existing exclusive source boundary.
