# F-X267, native rendering remediation, pass 1

**Reviewer**: Codex, independent worker `/root/review_source_pieces`.
**Reviewed**: Frozen working diff against `f6272f9511770057e730e8717da274137a3def0e`, 17 paths, 1267 insertions and 102 deletions, on `sprint/s95.4`. All 17 SHA-256 values match `/private/tmp/s954-q26-native-remediation-freeze/manifest.json`.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in this bounded remediation batch. This is not final native acceptance or whole-sprint acceptance.

## Independence and scope

The reviewer did not implement the product changes or author the actual fixture generator. Before this pass, the reviewer copied the frozen generator into a temporary standalone fixture producer and authenticated its historical release dependency closure. That preparation is disclosed rather than represented as an independent native oracle. No compilation, tests, rendering, UI operations or source changes were performed during this review. Only this review file was written.

Read the canonical workflow, microscope command, current approved F-X267 plan including A12 through A15, progress freeze, modified HLD sections and the native SmartArt profile repair map. The existing local-provider completion and outstanding final riders remain distinct. This pass audits the new 17-path batch, not unchanged historical performance implementation or candidate reference pins outside the repository.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Contract and consumer assessment

- **A12, fixture authority**: `crates/rpptx/tests/integration.rs:22685` requires the actual unique authored target, explicit solid fill, no-fill line and empty effect list. The generator at `crates/rpptx/tests/integration.rs:22759` establishes that paint policy without changing the six-slide case inventory. Visibility set, fractional opacity, independent axes and degree rotation use existing OOXML constructors. Guard tests at `crates/rpptx/tests/integration.rs:23105` challenge ownership, duplicate IDs/names, placeholders, missing or inherited paint and invalid geometry. The original nine logical clocks remain. The public roundtrip test at `crates/rpptx/tests/integration.rs:23215` checks resolved state and actual raster paint, including a partial alpha front. Fresh native capture and reference admission remain separate.
- **A13, evaluated state and publication**: `crates/rpptx-layout/src/timeline.rs:341` applies pending entrance hiding only in the absence of an applicable visibility writer. Native numeric cases at `crates/rpptx-layout/src/timeline.rs:1199` retain raw axis factors and degree rotation without altering legacy aliases. The wipe projection at `crates/rpptx-layout/src/timeline.rs:160` implements the approved spatial band and direction/complement policy. `crates/rpptx-layout/src/timeline.rs:147` and `crates/rpptx-layout/src/context.rs:3624` require unchanged source/public clips before carrying the private wipe. Rendering at `crates/rpptx-render/src/lib.rs:838` uses that projection and preserves other clips. Group-target timing retains its explicit flattened hard-clip diagnostic at `crates/rpptx-layout/src/context.rs:3611`.
- **A14, one scene and backend boundaries**: `crates/oxml-layout/src/output.rs:431` centralizes the finite geometry, ordered offset and alpha checks and ignores RGB as approved. Raster at `crates/oxml-pdf/src/raster.rs:482` masks the composited scene after ordinary effects, before final group opacity. SVG at `crates/rdocx/src/svg.rs:654` places mask wrappers outside the filtered scene, retaining one recursive child/text traversal. Multiple masks multiply, invalid masks fail transparent, and transforms and ordinary clips keep their existing ownership. No 64-scene expansion, replacement text raster or new retained cache was introduced. The PDF limitation is intentional, covered at `crates/oxml-pdf/src/writer.rs:3960`, and checked conformance rejects the new effect through `crates/oxml-pdf/src/conformance.rs:135`. The timeline diagnostic is produced at `crates/rpptx-layout/src/timeline.rs:1309` and reaches the public frame diagnostic vector through `crates/rpptx/src/lib.rs:14008`. This does not promise soft-mask PDF fidelity.
- **A15, native profiles**: All six profile callers reach `crates/rpptx/src/diagram.rs:4040`. Native percentage spacing defaults apply independently of horizontal centering, with field-wise explicit paragraph overrides and empty-paragraph preservation at `crates/rpptx/src/diagram.rs:4081`. First/last spacing retains explicit source authority. Cycle text frames at `crates/rpptx/src/diagram.rs:3790` and pyramid text frames at `crates/rpptx/src/diagram.rs:3987` remain separate from decorative/fill geometry. List retains the preset text rectangle, checked at `crates/rpptx-layout/src/context.rs:9678`. The source constants and insets agree with the independent native geometry repair map, rather than fitted raster expectations. Explicit spacing, empty paragraphs, nonorigin/nonsquare scaling and positioned first/last policy have concrete local controls. Six-family native comparisons remain required after the shared default change.

The additive enum variant and doc-hidden inherent query match the approved API amendment. No new trait, module, dependency, feature flag or Python signature was introduced. HLD03/06/08/10/12/14 describe current intent and retain the native/backend limits. HLD04's unchanged package contract is unaffected.

## Evidence consumed and limits

The selected library receipt records actual exit 0 with 129 passed and 5 explicitly ignored external checks. Its recorded source precedes the two disclosed test-only corrections, so it is supporting applicable evidence rather than an assertion that every byte equals the final freeze. The final receipt `/private/tmp/s954-q26-native-remediation-checks/final-focused-receipt.json` authenticates the frozen source and records actual exit 0 for the three focused tests, followed by six-crate, all-target, all-feature clippy exit 0 in 28.82 seconds. The focused log contains one library test and two integration tests. The earlier moved-value compilation failure and test lint failure remain setup/check failures, not semantic REDs or passing executions.

The historical actual visibility failure remains preserved. Neither the self-produced fixture nor aggregate local pass counts discharge native parity. All four external native gates, fresh exact-source capture and reviewed pin adoption, the integrated 49-entry hash gate, current normal artifacts, affected installed runtime/type checks and other final union riders remain pending. Actual neutral Effect packing/archive footprint, final performance and Q18/Q22 memory qualification also remain pending. No reference checks, thresholds or historical failures were removed in this diff, and no acceptance credit is assigned here for work not yet executed.

## Not found

Correctness, contract scope, panic/index safety, OOXML preservation/schema placement, test-oracle integrity and structural rules produced no findings in this batch. The zero result does not close F-X269 shadow fidelity or enlarge the direct-leaf/PDF capability boundary.
