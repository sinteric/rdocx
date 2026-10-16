# F-X233, Repair paragraph border geometry

**Status**: completed
**Sprint**: S93
**Size**: M
**Depends on**: none

## Problem

Issue366 has two screenshots and a15tp description, with no source DOCX, code
or unit definition. The renderer already divides sz by8
(`crates/rdocx-layout/src/paginator.rs:5803`). Border geometry uses paragraph
content bounds at3889 and independently offsets edges by space around5862.
Width, clearance, clipping and corner joins need separate measurements before
selecting a repair.

## Spec reference

- `docs/hld/14-development-backlog.md`, "F-X233, Repair paragraph border geometry".
- `docs/hld/08-rendering-spec.md`, paragraph paint and "Word section geometry and page numbering", shared border-edge rendering.
- `docs/hld/12-testing-strategy.md`, "Test taxonomy", render geometry and native raster comparison contracts.
- `docs/hld/15-build-and-toolchain.md`, "Deterministic rendering".
- `docs/hld/01-glossary.md`, units and OOXML eighth-point border size.

## Approach

The user authorized an explicit diagnostic interpretation on2026-10-15: start
with15points and take a screenshot, then try raw15 if needed. Source-build a
15point candidate with w:sz="120", save/reopen and independently inspect its
actual XML. Capture deterministic product and pinned native renders and compare
the relevant crops to the issue screenshots. Retain a distinct w:sz="15"
candidate when the first interpretation does not explain the report. Both
remain inferred candidates, not the unavailable original source document.

Measure stroke thickness, w:space clearance, outside extents, clipping and join
continuity separately. Vary only one property at a time. Diagnose before changing
code. Repair the proved geometry defect in existing paginator.rs/paint carriers,
preserving unit constructors, exact schema/source representation and unrelated
paragraph placement. Include single/double styles, asymmetric edges, adjacent
paragraphs, indentation and page fragments. If both candidates already match
native paint, record that outcome and the missing-original limitation rather
than inventing a conversion fix.

No new public surface, module, dependency, product file or raster threshold is
proposed. Tests use the existing paginator.rs unit module and a scratch public facade
save/reopen consumer. No facade dev-dependency is added to the layout crate.
The Word integration entrypoint remains exclusively with F-X246 in the parallel
wave. Native evidence records actual current Word build and Poppler identity
independently from historical pins. Start after F-X232 source integration.
F-X233 alone holds the shared hash and pixel baseline lease in this wave.

## Measured geometry clarification

Actual Microsoft Word16.113.2 build16.113.26092012 exports of the frozen
inferred inputs and geometry controls corroborate the repair scope. Native
receipts are retained at
`/private/tmp/F-X233-native-inputs/native-word-export-receipts.json`.
The first foreign opaque property control is refused by Word. Separately
named minimal inputs open normally at both sz120 and sz15, so that refusal
does not establish a size-conversion defect. Original source remains missing.

Width retains sz/8 points per stroke. Double borders retain three stroke-widths
of total band. Word's exported widths quantize to2.88points for sz24,
6.00points for sz48 and1.68points for sz15. These observations do not justify
changing OOXML units. Word adds a1.5point horizontal paragraph allowance to the
edge-specific space. This is supported by single, double, dashed, indented
and asymmetric controls. Top and bottom border bands plus their space consume
the owning paragraph's vertical flow. A20point content frame with3point single
edges occupies26points at space0 and42points at space8. The same accounting
must reach fit, keep, split, table-cell and related-story measurements. Following
content moves only as a consequence of the bordered frame. Unbordered flow
and shared page-border geometry remain unchanged.

A same-border adjacent pair forms one outer frame. The selected native split
paragraph has its top edge on the first fragment, vertical edges on every
fragment and its bottom edge on the last fragment. LibreOffice instead closes
every fragment, so actual Word determines this topology. Corners extend to
adjacent outside bounds and paired double strokes join their matching bands.
The measured nonzero paragraph-spacing controls have an outer frame of
73.68points at both after8/before12 and after12/before8. The32point native
baseline advance confirms ordinary maximum spacing collapse and continuous
vertical sides. Private metadata carries the already-priced group gap to body
and table painting. Header/footer and note groups price that same interior gap
through existing private carriers, including footer anchoring and flattened
note lines. Cached original borders and unbordered spacing remain unchanged.
Group bridges stop at fragment boundaries instead of painting above the page.
Wrap reservation passes measure freshly broken content at the painted origin
below the top border band. Padding is stripped before reservation and restored
once afterward, so a short square drawing crossing only that shifted text band
still reserves first-line width without stacking frame height.
The repair stays within existing paginator, block, engine, table and note
carriers, using private helpers and no public fields, types or dependency
changes.

Independent vector measurement and pinned150DPI crops qualify width, clear
space, outside bounds and topology separately. The new native component
metric bound is0.36points for stroke thickness and0.60points for frame
positions and flow extents, covering the observed0.24point export grid and
compound double-band rounding. These bounds are declared before implementation
and change no existing raster or similarity threshold. Full-page font-contour
parity and exact-original reproduction remain unclaimed.

## Rejected alternatives

- Multiplying or dividing border size before reproducing the defect.
- Calling either inferred candidate the exact original input.
- Loosening raster thresholds or fixing all border styles speculatively.

## Test plan

The **test gate** is **regression**.

| Category | Test | Asserts |
|---|---|---|
| regression | `fx233_fifteen_point_border_has_correct_paint_geometry` | Saved sz120, space, measured stroke width, clearance, extents and joins agree with independently measured geometry |
| regression | `fx233_raw_fifteen_border_size_keeps_eighth_point_units` | Distinct sz15 control preserves1.875point interpretation and makes unit differences explicit |
| regression | border fragment/corner controls | Adjacent, indented, asymmetric and split paragraphs do not clip or detach the selected border edges |
| regression | `cached_table_border_groups_keep_continuous_sides_across_paragraph_spacing` | Actual table-cache hits retain complete cold/warm paint and provenance for typed direct and nested bordered cells with nonzero paragraph spacing |
| round-trip | saved border input inspection | Size, space, style and opaque siblings survive reopen without source changes from rendering |
| differential | inferred input native paint comparisons | Pinned native raster and geometry qualify the diagnosed repair under unchanged thresholds, with interpretation limitation retained |

Run focused layout checks/tests, deterministic captures and independent crop
measurement, scoped verify and independent microscope. A missing original
prevents claiming exact-original reproduction even when candidate gates pass.

## HLD impact

- `docs/hld/08-rendering-spec.md`
- `docs/hld/12-testing-strategy.md`
- `docs/hld/14-development-backlog.md`, F-X233 acceptance wording for the user-approved inferred inputs when original source is unavailable.

## Risk routing

- Layout/pagination: deterministic font mode, independent geometry and explicit
  baseline attribution. Do not change unrelated paragraph layout.
- Units: preserve truncating constructors and eighth-point size semantics.
- External oracle: pin actual native producer, source XML, output digests,
  rasterizer and DPI. Preserve every existing tolerance.
- Package inventory: remeasure the existing layout README and matching numeric
  and date catalog entries after the cache regression adds packaged source.
  Preserve exact member totals, the64byte compressed allowance and10MiB ceiling.

## Hash harness

Any intentional change must be limited to fixtures exercising the diagnosed
border behavior, with independent expected width/clearance/join evidence.
Unrelated outputs remain unchanged. No baseline change until the diagnosis
and exact affected entries have been reviewed.

The expected candidate delta is18entries: PDF bytes and PDF page-content hashes
for contract, feature_showcase, invoice, letter, proposal, quote and report,
plus page1PNG for contract, invoice, proposal and quote. Every affected input
contains paragraph borders. XML and resource hashes remain unchanged. The
new output prices border-owned flow and joins frame corners, with native
geometry bounds and first/last-fragment topology qualified independently.
A genuine detached Base check matches49entries. A paired invoice with only
paragraph borders removed retains table borders and yields byte-identical
Base/candidate deterministic PDF and PNG output. Exact old/new entries and
source-border ownership are retained at
`/private/tmp/F-X233-remediated-hash-attribution.json`. Independent review of
these exact deltas completed with no findings before baseline recording.

The pinned Poppler26.01.0 golden manifest moves exactly four decoded page1
RGBA buffers: contract, invoice, proposal and quote. Every buffer remains
1275by1650pixels at150DPI. Feature_showcase, letter and report remain exact.
The genuine detached Base buffers match the prior manifest, and candidate
PDF hashes match the same reviewed18-entry attribution. Exact old/new buffers
are retained at `/private/tmp/F-X233-exact-pixel-attribution.json` and were
independently reviewed before recording. No rasterizer pin or threshold moves.

## Implementation checklist

- [x] Claim after all sprint designs are approved.
- [x] Build15point candidate, inspect saved XML and capture screenshot.
- [x] Compare raw15 candidate if necessary and record interpretation limits.
- [x] Diagnose and repair only the independently proved defect.
- [x] Pass scoped checks, independent microscope and structured handoff.

## Open questions

Resolved by user steering: try15points first and raw15 second if needed.
The original document remains unavailable, which is a recorded evidence limit,
not an unanswered product choice.

## Acceptance evidence

Independent source pass2 and metadata pass1 report zero defects and zero
smells. Pass1's short-wrap defect is retained and remediated separately.
The final layout suite passes347unit tests and1doctest. Denied-warning
all-target/all-feature layout Clippy, format, prose and generated skill checks
pass. Eleven focused controls and six restored semantic mutation failures
qualify the feature gate. Genuine Word receipts and refreshed public probes
retain the declared native bounds and unchanged unbordered bytes.

The scoped policy successor `/private/tmp/F-X233-policy-equipped-online.log`
passes182tests with2opt-in skips. The earlier offline attempt is retained at
`/private/tmp/F-X233-policy-final.log` and failed only the deliberately fresh
historical registry-graph lookup. README/doctest and all22package inventory
checks pass in the equipped successor. The layout archive observer records
335898compressed bytes,1844928normalized member bytes and15members under the
unchanged64byte compression tolerance and10MiB limit. Numeric measurement
carriers retain their existing schema.

The reviewed intentional18entry hash delta and four page1RGBA deltas are
recorded. Final checks report49hash matches and7pixel buffers matching actual
Poppler26.01.0 at150DPI. A one-pixel contract mutation fails as required.
Full integrated sprint verification remains the integrator's separate gate.
