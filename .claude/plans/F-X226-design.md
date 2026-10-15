# F-X226, Release superseded relationships after checked PowerPoint XML edits

**Status**: completed
**Sprint**: S91.1
**Size**: M
**Depends on**: F-X211b, F-X215, F-X216

All dependencies are done. This repairs an existing frozen-scope acceptance failure. ROOT approves the bounded repair under the instruction to finish the sprint and publication. No material product question remains.

## Problem

Actual main CI38662802341 passes853 PowerPoint Python cases and fails the graphic_data variant of `test_fx215_chart_xml_retires_old_view_and_data_edits_preserve_owned_graph` at `crates/rpptx-py/tests/test_documented_examples.py:10019`. Replacing one chart frame's relationship with the other frame's existing relationship leaves the old slide relationship unused. `validate_relationship_scope` at `crates/rpptx/src/lib.rs:19082` reports that row as dangling even though its target still exists. The debug save assertion at2339 then panics. Optimized builds omit that assertion, explaining earlier optimized-wheel success without establishing platform nondeterminism.

`replace_checked_owner_xml` at5254 stages the XML change and commits it at5296, but does not release relationships superseded by the selected edit. The original raw log is `/private/tmp/S91-main-CI-38662802341-readonly/Python-rpptx-failed-job.log`, SHA256 `f9984474d7a7df884eaaf67cf65a5243b778abe7de3b7bd3fa9177b06b04be1b`. Preserve this failure and the original test assertions.

## Spec reference

- `docs/hld/04-opc-and-packaging.md`, Checked PowerPoint XML transactions: physical owner scope, supported relationship identity, opaque preservation and atomic publication.
- `docs/hld/04-opc-and-packaging.md`, media graph removal and staged facade mutations: prune only newly unreachable candidates, preserve shared targets, raw references and producer orphans.
- `docs/hld/06-presentationml-model.md`, Preservation strategy and shape tree: retain unsupported owner payloads and schema child order.
- `docs/hld/10-bindings-spec.md`, checked XML owner authority and chart editing: unchanged live-handle retirement and complete chart/workbook ownership.
- `docs/hld/09-charts-spec.md`, Python authoring and owned replacement: stable chart data snapshots, source preservation and prepared publication.
- `docs/hld/12-testing-strategy.md`, regression and PowerPoint checked XML qualification: actual failing input, atomic refusal, save and reopen.

## Approach

Keep `validate_relationship_scope`, its known-role reference requirements and all save assertions unchanged. At the existing checked-owner mutation boundary, capture qualified relationship IDs from the selected original fragment. After the candidate XML is assigned and validated, examine the complete actual owning part with the existing owner-reference scanner. Release only a known reference-bearing relationship whose ID belonged to the original selected fragment and is now absent everywhere in the actual owner XML. A sibling, nested group, retained raw subtree or other qualified reference keeps its relationship alive.

Preserve every unrelated relationship row and unsupported source fact. Resolve internal targets from the actual owner. Feed only targets released by this operation to the existing `prune_unreachable_parts`, which preserves shared targets and recursively releases newly unreachable owned chart/workbook descendants. External targets are never fetched or treated as package parts. Preserve staged candidate publication and error rollback. Exact source no-ops do not change relationship or package bytes.

Use existing `crates/rpptx/src/lib.rs` for the implementation and inline native regression cases. Strengthen the existing chart regression in `crates/rpptx-py/tests/test_documented_examples.py` only where necessary to assert actual validation, package relationships and save/reopen facts. No binding implementation change is expected. No new product file, module, crate, dependency, trait, generic parameter, feature, asset or public API is authorized. If the actual repair requires a wider change, revise this plan before implementing it.

**Semver impact**: bug fix only. Public signatures and accepted owner kinds stay unchanged. The intentional behavior change removes superseded owned graph entries after a successful checked edit, allowing the existing supported chart replacement to save safely. Keep it in the feature's labelled commit, separate from representation-only F-X225.

If verified archive tuples change, update only actual affected README observations and their existing numeric catalog in `scripts/readme_doctests.py`, retaining the64-byte normalization tolerance and10MiB ceiling. ROOT reconciles any overlap with F-X225's final archive observations after source integration.

## Rejected alternatives

- Removing debug assertions or weakening relationship validation hides an invalid graph.
- Deleting every unused row or sweeping all package orphans destroys unrelated imported source ownership.
- Changing the existing test or running only optimized wheels loses the defining failure.
- Rebuilding a sibling's chart or workbook changes unrelated content and live owners.

## Test plan

**Test gate**: regression.

| Category | Existing entrypoint or control | Asserts |
|---|---|---|
| regression | Original graphic_data Python variant and new existing-module native counterpart | Genuine Base debug failure, repaired debug and optimized installed execution, unchanged stale-handle and other-chart assertions |
| regression | Existing native checked XML tests | Removing or changing a selected qualified reference releases only its superseded known-role row, and validation/save/reopen succeeds |
| round-trip | Existing inline native module, shared and opaque graph controls | Sibling/group/raw references retain rows and byte-identical payloads, shared chart/workbook targets survive, newly unreachable owned descendants are removed, producer orphans remain |
| regression | No-op and invalid replacement controls | Exact no-op bytes remain equal, malformed XML or missing/invalid relationship identity refuses atomically, old live owner remains usable after refusal |
| regression | Relevant existing chart, XML, image and import controls | Known reference roles keep existing identity/ownership and complete package behavior |
| integration | Existing isolated PowerPoint Python suite for the affected chart/XML selections | Actual debug extension and optimized wheel both execute the public trigger without hiding the save assertion |
| integration | Both WASM target checks, affected strict Clippy and verified archives | Binding facade graphs, public signatures and exact packaged source remain valid |
| golden | Existing deterministic hash harness | All49 outputs unchanged, no baseline update |

Begin with `cargo check -p rpptx --all-targets`, focused native tests and the original public Python case in isolated targets/environments. Record actual collection and execution, not a test declaration. The worker runs the scoped gate and an independent zero-finding microscope. ROOT runs full integrated workspace, Python, source/artifact qualification and hosted CI once over the combined repair result. Preserve all existing mandatory oracle riders.

## HLD impact

- docs/hld/04-opc-and-packaging.md

Describe targeted superseded relationship cleanup in the existing Checked PowerPoint XML transactions section. HLD06 and10 are read and tested, with their existing behavior unchanged. F-X225 owns their separate parser storage documentation. F-X224 owns HLD15.

## Risk routing

Parser/serializer and package graph: read HLD04/06, preserve namespace-qualified references, raw source, child order and unrelated parts, with exact-byte and reopened graph controls. Published native facade and Python binding behavior: read HLD10 and structural rules, preserve public APIs, run both WASM graphs, isolated installed public regression, scoped package verification and10MiB checks. No new external oracle or Office check is earned by this graph repair. Existing hash49 and integrated mandatory oracle union remain required.

## Hash harness

All49 accepted outputs remain unchanged. The defining intentional delta is limited to owned graph cleanup after checked XML reference replacement. No new baseline is authorized.

## Implementation checklist

- [x] Preserve the original hosted failure and reproduce the public debug trigger.
- [x] Implement only targeted candidate cleanup in the existing checked-owner transaction.
- [x] Prove shared, opaque, external and producer-orphan preservation with atomic refusal.
- [x] Pass debug and optimized public chart controls, focused native tests and scoped gate.
- [x] Obtain a zero-finding independent microscope and update HLD04.

## Open questions

None. Intake remains frozen. Discussions264 and281 stay open. S92 bibliography, authoring and navigation remain deferred. No publication or remote closure is authorized by a test pass alone.

## Completion evidence

Scoped completion uses /private/tmp/S91.1-F-X226-implementation-1/final-scoped-evidence.json, SHA256 98f1edf48f91f1b1a6313e79f823dd4d9f2bb91b77e98a362e93e31b2dbc9770. Actual Base native and original public debug triggers fail with the required dangling relationship. Corrected native controls pass, the unchanged six public chart controls pass in both debug and optimized wheels, and current normal native coverage is177 unit passes plus392 integration passes through the explicit two-oracle setup successor. Original corpus/cache and registry setup failures remain retained. Strict Clippy, both WASM checks, policy159 passes plus2 platform skips, README examples, actual verified affected archive and hash49 qualify their stated source scopes. Independent source and numeric metadata reviews report zero defects and zero smells.

The catalog change is only the observed rpptx tuple/date. ROOT owns final combined F-X225 and F-X226 archive reconciliation and full integrated gates. No sibling parser correction, sprint totals, main merge, tag, publication or remote closure is claimed here.
