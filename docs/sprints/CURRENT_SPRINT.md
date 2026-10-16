# Current Sprint, S93

**Milestone**: M24, X, rendering and remaining issue repairs.

**Goal**: Finish the remaining issue acceptance with independent rendering and conversion evidence. Qualify legal revision rendering, diagnose paragraph borders, preserve supported LaTeX structure, qualify equation layout, and measure public setter inlining before changing policy. Also preserve chart-data revisions and render chart legends outside the plot when requested. Seven stories total 21 to 27 developer-days of estimated effort.

## Spec references

- `docs/hld/14-development-backlog.md`, F-X232 through F-X236, F-X245 and F-X246 define the issue contracts, dependencies and story gates. The M24 stability boundary remains S99.
- `docs/hld/03-architecture.md`, OfficeMath conversion belongs to the Word facade, layout consumes the typed tree, and checked XML owners preserve unmodelled content.
- `docs/hld/08-rendering-spec.md`, Word revision views, paragraph paint and OfficeMath layout define formatting, projection, baselines and deterministic rendering.
- `docs/hld/10-bindings-spec.md`, revision rendering options and native equation conversion define public surface and compatibility boundaries.
- `docs/hld/12-testing-strategy.md`, revision-view regressions, OfficeMath structure and native geometry/raster oracles define independent acceptance and mutation-sensitive controls.
- `docs/hld/15-build-and-toolchain.md`, pinned tools, deterministic bundled-font mode and build profiles constrain valid rendering and performance comparisons.

## The wave

| F-ID | Title | Size | Status | Owner |
|------|-------|------|--------|-------|
| F-X232 | Render legal revisions and qualify checked XML repros | L | done | - |
| F-X233 | Repair paragraph border geometry | M | done | - |
| F-X234 | Preserve supported LaTeX conversion structure | M | done | - |
| F-X235 | Qualify equation layout against native Word | L | done | - |
| F-X236 | Measure public setter inlining before changing policy | S | done | - |
| F-X245 | Preserve chart data changes in Word comparison | L | done | - |
| F-X246 | Honor chart legend position and overlay during rendering | L | done | - |

## Sequencing note

Rows are listed in dependency order, not F-ID order.

F-X232 assesses PR375 for Issue373 after completed F-X201, F-X211a and F-X211d. Reuse checked-XML and namespace providers and qualify the exact header/footer repros. Decide compatibility for proposed public rendering types during design. Legal revision and border painting share paginator resources and integrate serially.

F-X233 uses the user-approved inferred15point and raw15 controls because Issue366's original input is unavailable. It independently distinguishes width, clearance, clipping and joins while retaining that reproduction limit. F-X234 resolves Issue365's conversion half before F-X235 qualifies equation layout against an independently pinned native Word producer. F-X236 measures a real cross-crate, non-LTO setter consumer before selecting annotations or documenting no change.

F-X245 resolves Issue382 after completed F-X229 and F-X230, preserving chart parts and embedded workbooks in accept-all and reject-all reconstruction. F-X246 resolves Issue383 across Word and PowerPoint, including default legend placement, explicit positions and overlay. Coordinate shared chart resources and attribute chart raster changes separately.

Design every story before implementation. Lease shared sources and test entrypoints explicitly. Only one story may own a deterministic baseline change per wave. Expected product and external raster changes require separate attribution and review. Developer-day estimates are effort estimates, not elapsed-time promises. Expanded scope needs a split rather than weakened gates.

## Definition of done for this sprint

- Complete all seven story contracts through design, implementation, independent zero-defect and zero-smell microscope, scoped verification and integrated full verification.
- Prove Issue373 legal revision colours and double underline without changing stored formatting, with cache identity, native/CLI/Python parity, checked-owner repros and independent native paint evidence.
- Prove Issue366's diagnosed border repair with save/reopen, independently measured geometry and native paint on the approved inferred inputs. Preserve raster thresholds and the missing-original limitation.
- Prove Issue365's exact supplied formula conversion with normalized OfficeMath structure, meaningful symbols/spacing, explicit loss diagnostics and bounded parsing, then qualify layout under the existing one-point geometry and declared 150DPI SSIM contract.
- Resolve Issue367 with cross-crate non-LTO measurements and compiler evidence. Preserve semantics and avoid an unmeasured blanket inline policy.
- Use deterministic font mode for baselines. Attribute every intentional product or independent raster delta to its approved story and retain preservation, schema ordering and resource bounds.
- Reconcile all hash-harness outputs, pass the union of risk riders and obtain a clean integrated sprint review. M24's final stability gate remains a later boundary.
- Prove Issue382 chart-only changes at word and run granularity, chart-plus-text changes and the unchanged-chart control after save/reopen. Accept-all reproduces the edited chart and workbook, reject-all the original.
- Prove Issue383 default, bottom and left legends render differently in both formats without plot overlap when overlay is false or absent. Qualify all supported positions, overlay and hidden-legend controls.
