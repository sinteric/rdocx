# F-X245, Preserve chart data changes in Word comparison

**Status**: completed
**Sprint**: S93
**Size**: L
**Depends on**: F-X229, F-X230

## Problem

The exact Issue382 script demonstrates ignored chart-only changes at word and
run granularity, chart-plus-text failure and an unchanged-chart control.
Comparison carries only image and hyperlink relationships
(`crates/rdocx/src/comparison.rs:1347`), leaving chart drawing ids unchanged at7911
and permitting the no-op shortcut at851. Existing normalization hashes a chart
part but not its embedded workbook closure at3133,3152.

## Spec reference

- `docs/hld/14-development-backlog.md`, "F-X245, Preserve chart data changes in Word comparison".
- `docs/hld/09-charts-spec.md`, "Why a chart needs three parts" and source-preserving chart package ownership.
- `docs/hld/10-bindings-spec.md`, "Native Word facade stability", comparison staging, granularity and reconstruction.
- `docs/hld/12-testing-strategy.md`, "Test taxonomy", comparison reconstruction and drawing preservation coverage.
- `docs/hld/04-opc-and-packaging.md`, checked source ownership and relationship target resolution.
- `docs/hld/06-presentationml-model.md`, "Parts", raw XML preservation.

## Approach

Capture the exact authenticated Issue382 script and execute all four supplied
cases against Base before implementation. Native comparison identifies chart
content by its complete owned resource closure before drawing/run alignment and
before the unchanged shortcut. Include internal relationship topology, content
types, embedded workbook and retained sidecars, with deterministic handling of
shared parts and cycles. Distinguish external target metadata without fetching
network resources. Missing or unsafe resources retain atomic refusal boundaries.

Carry changed edited chart resources under collision-safe part names and owner
relationships while preserving the original closure for reject-all. Reuse the
existing bounded package-closure traversal/copying mechanisms in field.rs around
5090 and5172 through narrow crate-private visibility if their ownership contract
fits. Do not introduce a second generic package importer or invoke whole-body
fragment import to copy one chart. Relationship remapping must update only the
qualified selected drawing references and preserve opaque XML/source order.

Use closure-sensitive identities in reconstruction postconditions as well as
alignment. A workbook-only edit must not be erased because ChartML cache bytes
remain equal. Shared source targets, colliding names/ids and original-versus-edited
relationships must remain independently reachable until selected resolution.
Retain existing picture comparison, revision grouping and resource limits.
No new public API, module, file, trait, generic, feature or dependency is proposed.
The existing README.md rdocx archive measurement row and numeric
ARCHIVE_MEASUREMENTS tuple in scripts/readme_doctests.py are remeasured to satisfy
repository inventory policy. The 64 byte compression tolerance and 10 MiB ceiling
remain unchanged. This mandatory existing-carrier clarification is approved by
the sprint orchestrator.

The authenticated script's north() observer reads the fixed
word/charts/chart1.xml member rather than the chart drawn by the resolved owner.
Retain and report its unmodified output separately, including the unchanged
control. A separate explicitly corrected observer follows the surviving drawing
relationship without changing any source cases, expected chart values or text.
Comparison preserves collision-safe graph ownership and unrelated retained parts.
It does not promise canonical chart part names or remove unused original parts.
This observer clarification was approved by the sprint orchestrator.

Qualify native comparison and a fresh installed Word Python wheel. Independently
inspect both reopened resolutions using ZIP/XML and workbook cells rather than
only the production normalizer. Source inputs remain unchanged on success and
failure. Coordinate Word facade/test entrypoints serially with F-X232/F-X246.

## Rejected alternatives

- Comparing only chart relationship ids or only the ChartML cache bytes.
- Replacing one shared chart part in place, losing the rejected original.
- A parallel graph importer or acceptance through self-normalized equality alone.

## Test plan

The **test gate** is **regression**. Existing comparison.rs test module,
`crates/rdocx-py/tests/test_python_docx_parity.py` carry new controls.
The existing comparison unit module carries native package regressions so shared
integration and core Python carriers remain with their parallel feature lease.

| Category | Test | Asserts |
|---|---|---|
| regression | `fx245_chart_data_changes_resolve_both_versions` | Exact chart-only word/run cases produce revisions and reopened accept/reject reproduce edited/original chart and workbook |
| regression | `fx245_chart_and_adjacent_text_changes_compare_together` | Supplied mixed case reconstructs both complete sources |
| regression | `fx245_unchanged_chart_does_not_create_a_revision` | Supplied unchanged control remains unchanged |
| regression | `fx245_workbook_only_changes_remain_owned` | Workbook-only edits remain visible to comparison and resolve both exact workbook payloads |
| regression | `fx245_shared_chart_cycles_and_opaque_sidecars_survive_collision_remapping` | Actual selected drawings retain shared targets, exact sidecars, cycles and external metadata with one bounded resource copy |
| regression | `fx245_distinct_chart_roots_share_an_aggregate_resource_limit` | Distinct individually valid roots share one atomic operation-wide part and byte budget |
| regression | `fx245_existing_fragment_copy_keeps_distinct_case_alias_destinations` | Existing rich-fragment allocation semantics retain both distinct preallocated alias destinations |
| regression | `fx245_invalid_chart_closure_refuses_atomically` | Invalid relationships, missing parts and limit failures preserve both inputs and publish no partial redline |
| round-trip | independent package inspection | Both saved/reopened resolutions have exact expected ChartML caches, worksheet values, content types and qualified target graph |
| integration | `test_fx245_exact_issue382_script_and_independent_package_graph` | Exact original probe is retained separately from the corrected relationship observer. Every original expected value and text is checked with independently followed ZIP/XML/workbook graph ownership |

Run focused native/binding checks, existing image/grouping/postcondition matrices,
scoped verification and independent zero-finding microscope.

## HLD impact

- `docs/hld/09-charts-spec.md`
- `docs/hld/10-bindings-spec.md`
- `docs/hld/12-testing-strategy.md`

## Risk routing

- Parser/serializer and package traversal: read OPC and model contracts,
  namespace-qualified remapping, schema order, exact unmodelled subtree bytes,
  bounded graph closure and atomic refusal.
- PyO3: fresh installed Word script and focused owner controls, both WASM
  graphs and final workspace tests excluding both extension crates.
- External package inspection: pin the actual independent Python XML/ZIP/workbook
  inspection tools. No oracle becomes a production dependency.

## Hash harness

Expected unchanged, all49 existing outputs. New comparison chart fixtures are
explicit source-built controls. Any unrelated rendering delta blocks completion.

## Implementation checklist

- [x] Capture exact input, claim and establish Base failures.
- [x] Implement closure identity and collision-safe carried graph ownership.
- [x] Qualify both granularities, mixed/unchanged cases and atomic refusal.
- [x] Independently inspect reopened packages and installed Python outcomes.
- [x] Pass scoped verify, microscope and prepared handoff.

## Open questions

No unresolved product question. F-X229 and F-X230 are done in the shared backlog.
