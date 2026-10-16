# F-X253, Honor imported chart types and hidden master shapes

**Status**: completed
**Sprint**: S95
**Size**: M
**Depends on**: F-X209, F-X215

## Problem

The Python Chart.chart_type getter at crates/rpptx-py/src/shape.rs:2612 maps a coarse data family to the first enum, and unnecessarily requires cached data projection. The native renderer at crates/rpptx-layout/src/context.rs:283/291 gates master shapes with the layout flag and layout shapes with the slide flag. HLD07 explicitly describes that routing while HLD08's branding contract describes master suppression by either owner. Independent native evidence must resolve this contradiction before a repair rule is selected.

## Spec reference

- docs/hld/14-development-backlog.md, F-X253. Test gate: differential.
- docs/hld/06-presentationml-model.md, The shape tree and Placeholders.
- docs/hld/07-inheritance-and-resolution.md, Draw order and Testing this.
- docs/hld/08-rendering-spec.md, The seam that makes this cheap and Editable branding inheritance.
- docs/hld/09-charts-spec.md, The ChartML model and Python authoring and owned replacement.
- docs/hld/10-bindings-spec.md, Live PowerPoint masters and layouts.
- Pinned python-pptx 1.0.2 chart/plot.py primary classification, with exact reporter commit 91a57108 retained.

## Approach

Read qualified source metadata separately from dense categories/values or data replacement. Add the narrow cross-crate queries read_chart_type(xml: &[u8]) -> Result<Option<i64>> in existing oxml-chart and Presentation::chart_type(slide_index: usize, shape_id: u32) -> Result<Option<i64>>. Existing Python getter becomes int or None. No public Plot, Series or ChartKind fields change. This is additive native read-only API and an intentional more precise Python result contract, without an authoring API expansion.

Reuse actual unique physical frame, chart relationship/content-type and source ownership checks. Split the existing private chart-owner resolution only where needed to separate chart target authority from embedded workbook mutation authority. Metadata reading must not require a workbook or dense cache merely because data replacement does. Existing data/replacement providers retain their stronger checks and current behavior. Malformed source or invalid owner is an error, not None.

Prove horizontal stacked bar58 and stacked column52 from direct direction/grouping, standard marker-free line4 from direct qualified series marker/symbol none, and exploded pie69 from direct series explosion presence including zero. Preserve author-created line65 when no direct series none exists. Plot marker=false alone does not imply4. Preserve the seven current authored forms using their exact source metadata. Other recognized finite direction/grouping combinations may use the primary enum table, while valid unsupported styles/plot combinations return None rather than a family guess. No claim of exhaustive chart classification. Four required known cases cannot satisfy acceptance by returning None.

Retain chart source, data, series/plot order and opaque payload through metadata read/save/reopen. Use existing source scanner and private Series raw children where needed. No revision publication or owner retirement from a read-only getter. Imported sparse charts get independent metadata tests, and grouped/retired handles retain current ownership rules.

Actual PowerPoint16.113.4, build16.113.26100421, and LibreOffice26.2.5.2 agree on the four independently authored slide/layout flag cases. Ordinary master shapes require both effective showMasterSp flags. Ordinary layout shapes require the effective slide flag. Slide shapes and occupied placeholders remain visible in every case, while uninstantiated master/layout placeholders remain unpainted. Preserve inherited styling and backgrounds independently. True and absent flags are equivalent by the existing default. Native Save As canonicalized true1 to absent and retained false0, with all effective pairs and physical relationships unchanged.

| Slide flag | Layout flag | Ordinary master | Ordinary layout | Slide and occupied placeholder |
|---|---|---|---|---|
| true | true | visible | visible | visible |
| false | true | hidden | hidden | visible |
| true | false | hidden | visible | visible |
| false | false | hidden | hidden | visible |

Root performed repair-free open, separate native Save As, close, actual disk reopen and local PDF export using Best for printing. Independent observation at150dpi used the unchanged predeclared interior regions and thresholds from the separate LibreOffice capture. Source, native saved flags/relationships, actual four-page PDF and producer/tool identities are retained in /private/tmp/S95-F-X253-native-final-authentication-1/receipt.json, SHA256409537c130c69e1a914b2e4ae6df754f33735d3f92b23fda51ecd71bc6135c40. The common input SHA2567616aac10b385fd1970f509df7868c1301cf934a8bc4e538947a4ed54a8e83a1 changes only content-types namespace serialization from the first seed. The first seed and LibreOffice admission failures remain retained. No native acceptance of the original serialization is claimed.

Implement this observed rule only at the shared flatten/render pass. The current source gates master shapes only on the layout flag, so add the slide flag to that existing gate. Retain the existing layout gate on the slide flag. Keep placeholders, inherited styling, backgrounds, explicit/absent flags and physical shape ownership distinct. Update the stale source/HLD07 master-only rule, without broadening suppression to other inherited content.

Qualify the original two-slide logo reproduction and true/absent controls across actual Python PDF/PNG and CLI convert/render. A real old-renderer or gate-ignoring mutation must produce the wrong master region and fail the same observer. An over-suppressed layout or occupied-placeholder output must also fail where the native vector distinguishes it. Native references, not product renders, establish expectations. Update the existing Python stub to int | None and include a strict typed consumer that handles None.

## Rejected alternatives

- Using the existing family-only data projection preserves the classification bug and adds unrelated cache requirements.
- New public plot fields or a second XML model are unnecessary.
- Assuming layout visibility or a flag conjunction from current source/spec prose substitutes implementation for an oracle.
- Removing every inherited shape damages layout content and placeholders.
- Updating baselines before native evidence hides an unexplained output change.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| source metadata | fx253_imported_chart_types_use_direct_qualified_metadata | Four required exact enums, zero explosion, per-series none, namespace/foreign controls and current authored forms |
| native ownership | fx253_chart_type_read_is_sparse_and_owner_checked | No dense cache/workbook dependency for metadata, valid unknown None, malformed/ambiguous owner errors and source unchanged |
| installed Python | test_fx253_imported_chart_types_are_exact_after_reopen | Fresh installed four known values independently checked against python-pptx, authored65 and grouped/retired chart controls retained |
| differential | Independent four-flag PowerPoint and LibreOffice captures | Actual M/L/S and placeholder truth table, source/saved physical flags, repair-free reopen and actual PDF regions |
| layout | fx253_native_visibility_truth_table_preserves_distinct_layers | Final observed routing plus explicit/absent controls, styling/background and non-painted templates |
| actual output routes | fx253_pdf_png_convert_render_hide_native_master_logo | Actual four route files, original reporter input, same observer rejects real retained old behavior and wrong suppression mutants |

Use existing oxml-chart tests, rpptx native/integration tests, rpptx-layout context tests, rpptx-cli integration.rs and installed test_documented_examples.py. Preserve original strict XFAIL/XPASS reports and separately qualify unchanged bodies/stronger semantic observations. No new test binary, product fixture file, dependency or feature. Existing native control points remain separate from new qualified evidence.

## HLD impact

- docs/hld/06-presentationml-model.md
- docs/hld/07-inheritance-and-resolution.md
- docs/hld/08-rendering-spec.md
- docs/hld/09-charts-spec.md
- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md

## Risk routing

Layout/rendering: read HLD08, deterministic fonts and reviewed intentional deltas, real native and mutation controls. Parser: read HLD04/HLD06, qualified direct ownership, sequence and exact opaque preservation. Published API: read HLD10/structural rules, additive native queries and Python optional result compatibility, genuine affected publication dry runs and archive ceiling. Binding/WASM: actual installed suites, strict mypy/stubtest and both WASM targets. External oracle: apply differential-testing and pin python-pptx, PowerPoint, LibreOffice, fonts and PDF/raster tools with source/binary identities.

## Hash harness

F-X253 is the only proposed S95 baseline writer. Compare all 49 hashes and seven deterministic pixels before recording anything. Unaffected fixtures should remain unchanged. Any actual visibility delta must name the exact affected fixture/output, match independent native evidence, and be reviewed in its own labeled behavior commit. No system-font baseline, automatic re-recording or undeclared delta is permitted. Root owns the shared baseline and archive catalog lease.

## Implementation checklist

- [x] Preserve reporter and actual S94 baseline results.
- [x] Capture independent native truth table and resolve routing before approval.
- [x] Add exact read-only metadata and optional Python projection with current ownership rules.
- [x] Repair shared visibility routing only after native evidence decides it.
- [x] Qualify actual four output routes, independent positive controls and real failing mutations.
- [x] Attribute any intentional baseline delta, complete scoped riders, microscope and handoff.

## Open questions

Resolved by actual native PowerPoint and separately pinned LibreOffice captures. Both select master visibility as the conjunction and layout visibility as the slide flag, with slide and occupied-placeholder controls retained. No material user decision remains. Exact affected product baseline outputs are measured and reviewed during implementation before any baseline write.


## Completion evidence

The scoped gate passes with 906 native tests, 34 explicit native ignores, 912
freshly installed Python tests, strict mypy, stubtest and both WASM targets.
Actual policy runs 183 tests with two skips. Independent correctness pass 2 and
bounded packaging pass 3 both report zero defects, zero smells and zero nits.

Fresh post-remediation 49 hashes and seven deterministic pixel buffers match.
No baseline was recorded. Current installed Python PDF/PNG and CLI convert/render
outputs match the retained native four-flag observations. Independent
python-pptx 1.0.2-authored 58, 52, 4 and 69 remain exact after reopen. The unchanged
reporter records five strict XPASS results separately from five passing bodies.
Real old-renderer, over-suppression and global-rescan mutations fail their
respective native-region and metadata-work controls.

Genuine affected four-package publication dry runs use all 22 local patches and
allow-dirty, verify actual archives and upload nothing. The final all 22 inventory
validates exact normalized member counts and the unchanged 64-byte compression
rule. Root alone updated exactly eight numeric lines across the four README
observations and catalog. Those five files are the explicit applicability
exceptions to earlier whole-source guards, while all fourteen implementation,
test and HLD file digests remain exact. Final scoped proof is retained in
/private/tmp/S95-F-X253-worker-verification-1/scoped-verification-summary.json,
SHA256 54a4fc73d167e4b00e36a24d98dd4a7517c38a85e7927f86c539024468023a87.
The integrated sprint still requires its own full gate and review.
