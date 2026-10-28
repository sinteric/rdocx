# F-X267, materialized geometry, pass 1

**Reviewer**: Codex, `/root/review_source_pieces`.
**Reviewed**: Seven frozen paths against `69385a922fbad0f4764ca7e274654d844441c223`, 406 insertions and 11 deletions. All seven current hashes match `/private/tmp/s954-q26-materialized-geometry/freeze/manifest.json`.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in this focused batch. Final native acceptance remains pending.

## Scope and independence

Read the canonical microscope command and workflow, current approved F-X267 materialized-geometry contract, progress and frozen diff. Reused the preceding native-rendering review for unchanged mask/profile behavior. This reviewer supplied the earlier read-only recommendation to preserve bounds-resize semantics and scale materialized geometry. The reviewer did not implement this repair or its tests. That design involvement is disclosed. No build, test, render, native UI or source edit occurred in this pass. Only this review file was written.

The scope is the newly observed stale materialized path/text-frame invariant, not a confirmation review of the preceding 17-path batch. The canonical BBC fixture generator and separate native held-wipe alpha experiment are unchanged by this repair.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Source and consumer assessment

`crates/rpptx-layout/src/context.rs:3566` scales every materialized Custom move/line coordinate, cubic control and endpoint, and text-rectangle component with the actual signed axes. Close commands, fill rules, path ordering and geometry ownership remain intact. Rectangle and BoundsFallback return unchanged because their renderer geometry already derives from resized bounds. No second object affine, new API, field, retained cache or framework was introduced.

The helper receives only the private cloned candidate at `crates/rpptx-layout/src/context.rs:3680`. A short-circuited failed coordinate may leave that discarded candidate partially modified, but cannot modify the original shape. Geometry qualification precedes group-clip projection at `crates/rpptx-layout/src/context.rs:3690`. Publication at `crates/rpptx-layout/src/context.rs:3703` requires finite geometry, evaluated state, resized bounds and rotation. Failure retains the original shape and resets the resolved leaf state through the existing diagnostic path. Finite signed and zero factors are preserved without absolute-value normalization or clamping.

The repair closes the actual preset path mismatch because ordinary presets are already resolved as Custom geometry and `crates/rpptx-render/src/lib.rs:923` consumes those materialized paths. Source-resolved image placement continues to use resized bounds, while custom image clipping consumes the corrected paths. Text obtains its boundary from Custom.text_rect through `crates/rpptx-render/src/text.rs:1761`, then applies existing point insets exactly once. Font sizes, inset values, stroke widths, shadows, precomputed paint coordinates and content owners are intentionally unchanged. The amended plan and HLD06/08/12 state that boundary rather than claiming general emphasis fidelity for all paint/content domains.

## Tests and evidence

The new asymmetric custom source test at `crates/rpptx-layout/src/context.rs:9804` includes cubic controls, nonorigin text geometry, a custom picture, a source parent transform, rotation, a group clip and a nondefault fill rule. Assertions compare all path commands and text-frame components while preserving identity, source objects, content, line, shadow and fill. The identity/signed/dynamic-geometry/overflow test at `crates/rpptx-layout/src/context.rs:9937` distinguishes no-op scale, zero/negative policy, no double-scaling of Rectangle/fallback and whole-candidate overflow refusal. The renderer text consumer at `crates/rpptx-render/src/text.rs:5632` checks the exact content box after one inset application.

The integration test at `crates/rpptx/tests/integration.rs:36762` uses the actual canonical source-resolved preset. It selects one red physical owner, compares its positioned corners with independently computed scale/rotation/motion coordinates at1980ms and checks actual raster extents. This rejects stale paths despite numerically correct state and bounds. It does not manufacture a fresh native comparison or close the independent alpha discrepancy.

Consumed recorded execution without rerunning:

- Actual old-code context RED selected and failed both new context cases. Initial import/nesting setup failures and the old painted-owner filter failure are retained without being relabelled as successful discriminating runs.
- The first GREEN group had an incorrect test-only extent assumption and exited101. Its passing cubic/picture and existing transform/clip cases remain actual evidence. It is not reported as an all-green batch.
- The public painted-corners/raster test and existing timeline roundtrip both passed in the recorded paint GREEN batch.
- `/private/tmp/s954-q26-materialized-geometry/final-targeted.json` records actual exit0 for the corrected signed/fallback/overflow case and text consumer, with frozen paths unchanged. The final-targeted log records both selected tests passing.
- The three-crate all-target/all-feature clippy execution recorded in `green.json` exited0 in13.72 seconds. The later source delta was the disclosed test-only extent correction, not product implementation.

Actual historical native1980ms geometry failure remains the external RED. New native comparison, reference admission, integrated hash49, final normal artifacts, performance/memory and other final union riders remain Root-owned and pending. No existing source, native failure, clock, threshold or trusted pin was replaced in this batch.

## Not found

Correctness, approved contract, panic/index safety, OOXML preservation, structural rules and discriminating local test coverage produced no findings within this geometry scope. This zero result does not qualify native alpha/fade execution or broader text/stroke/paint scaling.
