# F-X215, Create charts in Python and replace existing chart data

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: F-X197, F-X206
**Issues**: 345, 346
**Test gate**: regression
**Implementation**: Native and both Python chart creation/read/replacement surfaces are qualified. Current table-header repair, verified affected archives, fresh paired installed controls, all twelve native Office linked-data/save/reopen cases and independent completion composition are accepted through the root scoped gate. Worker preparation retains integration and final sprint verification as separate steps.

## Problem

Native `rdocx::Document::add_chart` at `crates/rdocx/src/document.rs:20855` appends an editable chart paragraph, and `rpptx::Presentation::add_chart` at lib.rs4682 stages chart/workbook parts before adding a graphic frame. Neither requested Python authoring surface is present. Existing `CT_GraphicFrame::chart_relationship_id` at rpptx-oxml/graphic_frame.rs257 only exposes the owning relationship. The facade has no requested typed read/replace operation.

`oxml-chart/src/lib.rs:128` supports Bar, Line, Pie, Doughnut, Area, Scatter and Radar. ChartData at140 owns one categories vector and named value vectors. `authored_chart_parts` at150 validates them and writes consistent ChartML caches/formulas and one minimal editable workbook. Replacement must preserve unrelated existing ChartML and workbook content rather than recreate an arbitrary imported chart wholesale.

## Spec reference

- docs/hld/09-charts-spec.md, Why a chart needs three parts, oxml-sml deliberately minimal, The ChartML model and typed caches.
- docs/hld/04-opc-and-packaging.md, The package, Relationship types and relationship ownership.
- docs/hld/10-bindings-spec.md, Python API shape and checked owner publication.
- docs/hld/08-rendering-spec.md, Word chart pagination.
- docs/hld/12-testing-strategy.md, The deck corpus and Binding tests.
- docs/hld/14-development-backlog.md, F-X215 at7563, F-124 and F-158 native authoring contracts.

## Approach

Expose concrete CategoryChartData and XyChartData-compatible Python objects through existing extension carriers and __init__.py exports. Categories, ordered named series and optional number_format map to existing native ChartData. Xy accepts the existing numeric-category shared-X contract. Validate that each series uses identical ordered X values and reject unsupported heterogeneous X vectors explicitly before publication. Do not silently discard X values or expand native chart types beyond the seven existing variants. Root exports can retain python-pptx class and method names without introducing a new chart/data product file. Shared checked extraction may use existing oxml-py-support/lib.rs utilities, with actual Word and PPT callers, never a forwarding-only abstraction.

PPT signature: `slide.shapes.add_chart(chart_type, left, top, width, height, chart_data) -> GraphicFrame`. Return the actual owned graphic-frame handle, with `has_chart` and `.chart` access. Word signature: `Document.add_chart(chart_type, chart_data, width, height, *, after=None) -> Paragraph`. Omitted after appends. A supplied Paragraph must be a live owner of this document and inserts immediately after its actual body paragraph. Add the minimal native paragraph-relative chart insertion operation in document.rs, reusing existing checked package assembly. Foreign, retired and unsupported story owners refuse atomically. This is a requested Word authoring surface, not fictitious python-docx parity.

Add `Presentation::chart_data(slide_index: usize, shape_id: u32) -> Result<(ChartKind, ChartData)>` and `Presentation::replace_chart_data(slide_index: usize, shape_id: u32, data: &ChartData) -> Result<()>` in existing rpptx/lib.rs. Resolve the actual graphic frame by unique owned shape identity, including existing nested-owner traversal, rather than a mutable ordinal alias. Read kind, categories and ordered named series from the chart part caches through existing typed CT_ChartSpace/Series. Python `.chart.chart_type`, `.chart.categories`, `.chart.series` and `.chart.replace_data(chart_data)` use the same native owner. Unsupported plot types remain byte-preserved and report precise unsupported-projection errors. Read should not mutate package or synthesize missing data.

Replacement resolves the frame's chart relationship, chart package relationship, style/colour parts and actual workbook scope. Validate kind/data/finite values and owned formula ranges, allocate candidate XML/workbook before publication, then update caches, formulas and worksheet cells as one native candidate transaction. Match surviving series by ordinal, retaining original stable series idx/order and raw series formatting. Added series receive the existing native default style, not copied unsupported styling. Removed series lose only their owned modeled series nodes. Preserve axes, chart title, legend, extension payload, aliases and chart root attributes. Shared chart/workbook relationships require copy-on-write or an explicit checked multi-owner operation, never silently changing another chart frame.

For an imported embedded workbook, target only the series/formula-owned sheet and ranges. Preserve unrelated sheets, styles, comments, names, relationship graphs and opaque entries. Use existing OPC/SpreadsheetML carriers to merge known chart cells. The minimal oxml-sml writer remains a writer, not a general spreadsheet product. Reject an unsupported formula/linked external workbook before any mutation, naming the actual boundary, rather than overwrite unrelated workbook content. Source-built and qualifying corpus bar/line charts with ordinary embedded ranges are mandatory supported cases. The patch must not use this boundary to refuse the whole acceptance corpus.

Bindings use the actual completed F206 checked owner provider, staging native candidates and allocating returned handles before commit under exact public operation labels. Reads return owned snapshots or bound live owners consistently. Errors/no-op leave serial, package, workbook and existing owners unchanged. Foreign object checks precede lookup. No new revision engine or legacy bump-based factory.

Existing carriers: oxml-chart/lib.rs, oxml-sml/lib.rs only for necessary chart-owned cell utilities, rdocx/document.rs, rpptx/lib.rs, relevant existing oxml-opc package utilities, oxml-py-support/lib.rs, rdocx-py/document.rs/lib.rs, rpptx-py/shape.rs/slide.rs/lib.rs and existing Python __init__.py/_rdocx.pyi/_rpptx.pyi and test entrypoints. No new product file/module/crate/dependency. CLI replace-data is optional only if the current command structure supplies a concrete established input format. Its omission cannot remove native/Python acceptance. F205 retains existing projection/export ownership and may consume this chart accessor. It does not reimplement chart data authoring or replacement. No F205 dependency cycle is added.

## Rejected alternatives

- Chart images lose editability and cannot satisfy Edit Data.
- Recreating the whole imported chart or workbook drops styling and opaque package content.
- Accepting independent XY data while using the first series' X vector silently corrupts data.
- A bindings-only ZIP editor creates a second native mutation implementation.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| regression | `fx215_installed_word_and_ppt_bar_line_creation` | Both installed bindings author bar and line, save/reopen kind/categories/names/values, correct graphic frame/paragraph and checked after insertion |
| unit | `fx215_existing_chart_data_projection` | Exact cached categories, strings, numeric series, number formats and seven supported kind projections, unsupported explicit errors |
| round-trip | `fx215_replace_data_updates_cache_formula_and_workbook` | Qualified corpus fewer/more series and more categories, cached values plus actual XLSX cells/formula targets, package validation and reopen |
| round-trip | `fx215_surviving_series_and_workbook_opaque_payload` | Surviving ordinal formatting, foreign attributes, unknown nodes, namespace aliases, unrelated workbook sheets/entries and style/colour parts remain exact |
| regression | `fx215_invalid_data_and_shared_chart_owner_are_atomic` | Nonfinite/mismatched vectors, foreign/retired owner, external formula and allocation failures preserve package/serial, current owner publication and copy-on-write identity |
| differential | `fx215_category_and_shared_xy_match_python_pptx` | python-pptx1.0.2 normalized created chart caches/series, both XY shared-coordinate positivity and heterogeneous explicit refusal |
| golden | `fx215_created_and_replaced_charts_render_native_values` | Native/CLI convert --to pdf renders both family bar/line and replaced values under deterministic fonts, geometric/data-specific positives and stale-values negative |

The named regression gate comprises the installed creation and native/corpus replacement controls, never missing-API compiler failure. Native Office acceptance is mandatory: Word opens both created kinds without repair, PowerPoint opens created and all three replacement cardinalities without repair, Edit Data visibly reads the new values, normal save/reopen retains exact values and correct rendered chart. Record actual app/build, input/output package hashes and native worksheet evidence. Pin python-pptx1.0.2 and python-docx1.2.0 for surrounding document ownership only, because python-docx has no add_chart oracle. Run fresh affected wheels, real selected installed node membership, strict mypy and stubtest in both families. Inline controls remain meaningful when external corpus is unavailable, but missing required corpus/native acceptance is uncredited.

## HLD impact

- docs/hld/02-scope-and-non-goals.md
- docs/hld/04-opc-and-packaging.md
- docs/hld/09-charts-spec.md
- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md
- docs/hld/14-development-backlog.md

## Risk routing

Parser/serializer04/06: exact chart/workbook unknown preservation, schema sequence and relationships. Public API10: additive facade/binding surface, no new chart-kind expansion, verified affected packages with local patches and10MiB ceiling. Bindings: fresh wheels/import provenance/typing/stubtest, both WASM graphs. Layout/render08: deterministic fonts, chart data-specific render oracle. Units01: retain checked EMU/Length truncation, refuse invalid dimensions before narrowing. External oracle: version-pinned Python/native Office, exact structural comparison and declared raster metric. Dependency graph03: no low-level dependency on either facade and no new published crate. All ordinary scoped gates remain mandatory.

## Hash harness

Existing default samples expected unchanged. Newly authored/replaced requested chart outputs change only their test inputs. Any library-output delta in the existing49 requires separate attribution, approved declaration and labelled behavior commit before exclusive baseline handling. Source metrics/archive tuples require actual current-source measurements.

## Implementation checklist

- [x] Approve complete exact native-supported data contract and wait for F197/F206 done.
- [x] Preserve genuine Base behavioral/capability controls after valid owner/data setup.
- [x] Implement native read/replace, source-aware workbook merge and atomic ownership.
- [x] Expose both authoring bindings, returned chart owners and every new stub/export.
- [x] Complete creation/replacement corpus, native Office/Edit Data and deterministic render evidence.
- [x] Run changed crates, fresh installed/typing, WASM/package/hash/policy and zero-finding review.

## Open questions

No blocking product choice. Existing shared numeric-category XY scope is explicit. Optional CLI exposure is assessed against actual established commands and is not a prerequisite substitute. Canonical F197 and F206 are complete. Implementation uses the completed provider and exclusive worker source ownership.

## Batch approval, 2026-10-11

The rolling intake instruction authorizes the full linked issue acceptance.
ROOT reviewed the source, dependencies, test gate, risk riders and HLD list.
The stated defaults are adopted. No blocking product question remains.
Implementation waits for completed dependencies and an exclusive resource wave.

## Contributor PR356 intake, 2026-10-11

[PR356](https://github.com/tensorbee/rdocx/pull/356) at head e82a2c0c0492517d918c48ee0bde93d291e1b369 joins existing approved F-X215 for Issues 345 and 346, charts creation and data replacement. Its exact captured patch is a15fdd633c8ab55446a7e63cf6c50036f9ed172701a6c044fb649c1c0d75837b at /private/tmp/S91-PR356-intake/diff.patch. Read-only contribution assessment is complete at /private/tmp/S91-PR356-intake/assessment.md, SHA eb542fe436252ded0f2ab12e999abf5723ae1f73429279af166b25f09f6b9a56. Reuse chart projection/data checks and creation/replacement tests. Reconcile workbook and raw ChartML preservation, shared chart ownership, qualified relationship lookup and prepared binding publication. Localize types into the approved existing carriers. Native Office and full local acceptance remain due. Reuse qualifying contributor code and tests within the original approved acceptance and existing carriers. Contributor test claims, archive measurements, omitted acceptance and new product files do not replace local verification or grant additional structural approval. Implementation retains its dependency and exclusive source boundaries.
