# F-X211b, PowerPoint checked XML owners

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: F-X194, F-X206, F-X209, F-X200, F-X210 and F-X214 actual presentation part/text/topology providers. F-X211c owns final cross-family acceptance.
**Test gate**: regression

## Problem

Nine approved PowerPoint element cases need checked owner-aware writes. Existing concrete parsers and authoring providers are reused, not counted as complete checked replacement. Borrowed mutations alone lack package relationship scope.

## Spec reference

- docs/hld/10-bindings-spec.md, Python API shape and The invalidation problem, handled loudly.
- docs/hld/04-opc-and-packaging.md, actual owner relationships and preserved parts.
- docs/hld/06-presentationml-model.md, physical owners, child sequence and retained source.
- docs/hld/03-architecture.md, allowed dependency graph and concrete ownership.
- docs/hld/02-scope-and-non-goals.md, compatibility and raw XML escape boundary.
- docs/hld/12-testing-strategy.md, Binding tests and What CI runs.
- docs/hld/14-development-backlog.md, F-X211 acceptance and provider ownership.
- docs/hld/08-rendering-spec.md, deterministic saved/rendered evidence.
- .claude/plans/F-X211-design.md, approved original contract and all refinements, retained immutable corpus and evidence.

## Approach

Own exactly nine cases: DrawingML paragraph p, regular run r, table tbl, cell tc, actual captured shape kind, actual p:txBody or a:txBody, PresentationML slide sld, layout sldLayout and master sldMaster. Cover sp/pic/grpSp/cxnSp/graphicFrame within the one shape case. Public element.xml:str and replace_xml(xml:str)->None preserve legacy Shape.xml bytes. Regular Run refuses fld/br, and shape kind conversion is not implicitly authorized.

Use real ShapeOwner Slide/Layout/Master, physical group paths and existing part records. Parse complete qualified input, close actual namespaces and validate modeled attributes/sequence/invariants, identity and source-part relationship target/type/scope before mutation. Preserve unrelated opaque bytes, source parts and existing alternate-copy barriers. Refuse unprojectable retained masters and unsupported alternate mutations. No implicit cross-part import. Clone candidate, validate affected owner/package, serialize/reopen and publish atomically. Exact equivalent replacement is no-op. Family-local replacement owns targeted retirement/ancestor survival, including retained table owner versus unsafe grid descendants.

Own existing rpptx facade/oxml carriers, oxml-drawing table/text parser carriers including actual qualified hyperlink alias behavior, necessary PPT Python carriers/stubs/tests. Shared DrawingML effects on Word are root-serialized with a. Shared revision primitive changes are root-only serial prerequisites.

Reuse the independently reviewed F-X218 bounded image validator as an explicitly serialized source prerequisite. Transfer only its constants and helper region into the existing rpptx crate root, with the four dependency edges already approved for F-X218. This does not complete F-X218 or start its dependent stories. Exact supported raster MIME and complete bounded decoding are required for checked image relationships. New unsupported formats refuse. An existing unsupported image fact may survive only with the actual original relationship role, target bytes and content type unchanged, without claiming successful decoding. Malformed supported raster data never receives that exception. Preserve actual transfer preimages and qualify the XML caller independently.

The approved F-X211 parent remains binding. This split transfers its acceptance once without adding or weakening app/manual obligations. Existing material approvals persist. Each child follows design/start/implement/microscope/scoped-verify/complete. Parent closes only after all three children close. No new product file, module, crate, dependency, feature flag, generic XML framework or revision registry is authorized. Normal child design records are workflow artifacts. Root alone updates shared ledgers, HLD/archive composition and the hash baseline. Workers retain original failed/unqualified evidence and use existing test entrypoints. Shared primitive changes are explicit root-serial prerequisites, never concurrent edits.

## Test plan

**Test gate**: regression.

| Category | Test | Asserts |
|---|---|---|
| regression | powerpoint_element_xml_nine_owners | P-XML-01..09 plus bundled shape variants, actual root URI/kind and save/reopen facts |
| regression | powerpoint_qualified_alias_and_atomic_refusals | Default/alias positives, foreign/rebound/wrong-root/trailing/DTD/entity and late failure preserve source/tokens |
| regression | powerpoint_relationship_and_opaque_owner_safety | Actual part scope/types, shared media/hyperlinks/IDs, physical alternatives and unrelated opaque bytes |
| regression | powerpoint_xml_local_owner_retirement | No-op/failure, descendant retirement and approved table/shape/part ancestor identity |
| integration | installed_powerpoint_element_surface | Current fresh wheel/all-member/source matching, exact public nodes, strict mypy/full stubtest and meaningful prepublication allocation refusals |

Use existing native integration and test_documented_examples/typing entrypoints. Named gates: affected native crate tests, all-target/all-feature strict changed-crate Clippy, cargo fmt, hash_harness --check, repository policy/prose/adapters, declared WASM graph and MSRV/dependency riders, affected archives with actual size/member provenance. Root applies archive/HLD composition. Retain pinned parent native PowerPoint/source/save/reopen or deterministic render obligations where applicable, without new blanket manual gates. No final100 rate or duplicated provider repair.

## HLD impact

- docs/hld/02-scope-and-non-goals.md
- docs/hld/03-architecture.md
- docs/hld/04-opc-and-packaging.md
- docs/hld/06-presentationml-model.md
- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md
- docs/hld/14-development-backlog.md

Root reconciles each child's current intent in these exact shared files after reviewed composition. This child worker prepares its authorized HLD edits and affected archive measurement tuples. Shared delivery totals remain integrator-owned.

## Risk routing

Parser/serializer04/06: qualified complete roots, sequence, namespace preservation and correct actual part ownership. API10: source compatibility and affected archives. PyO3 10: native-backed installed provenance/typing, atomic allocations and WASM graphs. Dependency03: concrete ownership, no cross-family reversal. Unit/rendering01/08: canonical lengths and deterministic font evidence where changed behavior requires it. Oracle12: exact parent source authorities. Existing parsers are not described as full XSD validation.

The shared image prerequisite also requires the approved dependency graph, MSRV, no-default-feature, WASM and affected package checks over the composed source. Its mechanical relocation review and focused source-export controls do not substitute for the checked XML image acceptance or final feature gates. Historical archives and installed wheels remain evidence for their original source only.

## Hash harness

No default output delta is expected from additive XML, aliases, metrics or errors. Explicit selected-target edits retain opaque sibling controls. Root holds the exclusive baseline lease. Any genuine intentional default behavior change requires a separate labelled reviewed delta under its actual owner. Never adjust the frozen corpus, denominators, expected hashes or baselines to manufacture acceptance.

## Implementation checklist

- [x] Inventory actual nine owners and concrete parser/read gaps.
- [x] Complete native/public checked operations and compatible legacy reads.
- [x] Qualify DrawingML aliases and true part relationship ownership.
- [x] Qualify all nine cases/shape variants, atomicity and local retirement.
- [x] Complete declared scoped gates, root HLD/archive composition and clean independent microscope.
- [x] Prepare valid child handoff only after genuine completion.

## Open questions

No acceptance amendment. Shared DrawingML/token prerequisites are explicitly root-serialized. a/b implementation remains disjoint. c owns shared residual aliases/lifetimes/errors and final frozen100.
