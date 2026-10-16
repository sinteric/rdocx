# F-X235, Qualify equation layout against native Word

**Status**: completed
**Sprint**: S93
**Size**: L
**Depends on**: F-X234

## Problem

The fraction rule shares the group baseline while the denominator begins above
it (`crates/rdocx-layout/src/math.rs:518`,558). This can paint the rule through
denominator ink. Issue365 also requires independent qualification of integral
limits, scripts, radicals, glyph coverage and baseline alignment after its exact
formula is correctly converted by F-X234.

## Spec reference

- `docs/hld/14-development-backlog.md`, "F-X235, Qualify equation layout against native Word".
- `docs/hld/08-rendering-spec.md`, "Word OfficeMath layout".
- `docs/hld/12-testing-strategy.md`, "Test taxonomy", OfficeMath layout gate and its1.0point,150DPI,0.99SSIM contract.
- `docs/hld/15-build-and-toolchain.md`, "Deterministic rendering" and "Toolchain pinning".

## Approach

Design approval occurs during the all-sprint design phase. Implementation cannot start
until F-X234 is completed through the dependency-prefix checkpoint. Reuse the
existing MeasuredMath, FontManager and shared backend primitives. Position the
fraction rule around the math axis and derive numerator clearance, denominator
clearance and group ascent/descent independently. Measure actual glyph ink and
surrounding baseline rather than tuning constants against a screenshot.

Qualify layout_nary, limits, radicals and script sizing for the exact formula
and independent single-construct controls. Retain every existing supported
expression family and bounded layout behavior. Production invokes no external
executable. No new module, public API or product file is proposed. The approved internal
graph refinement adds the existing workspace ttf-parser dependency directly to
rdocx-layout. Version0.25.1 is already resolved and used by oxml-layout, so no
new package is introduced. Private math.rs reads actual face_index, glyph ink
and OpenType MATH constants and variants, with bounded fallbacks when absent.
The existing-API candidate clears fraction boxes but its exact matching-font
formula is16.27points too narrow, with maximum independently measured ink
error8.16points. Full-page SSIM0.995086 alone conceals this geometric failure.
This evidence justifies reading real font data instead of tuning constants or
adding a public font API. The1point and0.99 thresholds remain unchanged.

Extend existing `scripts/officemath_oracle_harness.py` and its text manifest
with a separately identified new fixture. Preserve historical Word16.104 build
16.104.25121423, Poppler26.01.0 and their original source digests and outcomes.
New actual producer pins are Word16.113.2 build16.113.26092012 and Poppler26.09.0,
subject to authenticating the exact binaries at execution. Capture source DOCX,
native PDF, normal save/reopen and deterministic product PDF independently.
Retain untracked binary evidence and tracked text provenance in existing carriers.

The new native input explicitly requests Cambria for surrounding text and
Cambria Math for equations. Authenticate actual Word-bundled Cambria font bytes
and inject them through the existing deterministic caller-font render API.
These fonts remain external oracle inputs and are never redistributed. The
bundled Caladea product default and existing sample baselines stay unchanged.
Retain the first Caladea-requested native export as a diagnosed Cambria fallback
mismatch, separate from matching-font qualification and historical evidence.

The matching-font v1 native SaveAs injects omitted paragraph defaults after160
and line278 auto. Preserve that source and its failed absolute-baseline results.
Root approved separately identified v2 controls with explicit before0, after0,
line240 auto and math preSp0, postSp0. The unchanged six expression recipes,
authentic caller fonts and producer pins bind the new sources. Actual glyph ink,
MathLeading and RadicalExtraAscender define the display frame. The private
measured box retains reserved top padding through composition. No comparison
normalizes output coordinates or weakens the original1point/.99 thresholds.

## Rejected alternatives

- Self-recorded product pixels as the independent oracle.
- Relaxed geometry or raster thresholds and replacement of historical pins.
- Fixing conversion losses within this dependent layout story.

## Test plan

The **test gate** is **differential**. Reuse existing math.rs tests and oracle
harness rather than adding another integration binary.

| Category | Test | Asserts |
|---|---|---|
| regression | `fx235_fraction_rule_clears_both_ink_boxes` | Simple and nested fractions do not overlap rule and denominator ink, with correct surrounding baseline |
| differential | exact-formula native Word fixture | Integral limits, fractions, radicals, scripts, tokens and glyphs meet per-expression and aggregate1.0point geometry tolerance |
| differential | pinned150DPI native raster | Existing64by64 luminance blocks meet0.99SSIM, preserving unchanged thresholds |
| regression | rendered1.01point mutation control | Actual geometry and raster comparison reject a perturbed result |
| round-trip | exact-formula document save/reopen | Supported typed structure and opaque source survive layout and native opening |

Run focused layout checks/tests, all thirteen historical expression-family
controls and original source/manifest identities, current pinned native export,
scoped verification and independent zero-finding microscope. Fresh historical
Word exports are not required. Missing evidence is recorded honestly and never
reported as an executed pass. The new fixture requires actual current native export.

## HLD impact

- `docs/hld/08-rendering-spec.md`
- `docs/hld/12-testing-strategy.md`
- `docs/hld/15-build-and-toolchain.md`

## Risk routing

- Layout/pagination/shaping: deterministic bundled-font construction for every
  comparison and baseline. Attribute intentional equation geometry separately.
- External oracle: bind actual Word and Poppler identities, source digest,
  exact commands, geometry, raster and negative control to the current result.
- Dependency graph: read docs/hld/03-architecture.md family layering. The new
  direct edge reaches existing format-neutral ttf-parser only. Check both WASM
  graphs, denied-warning crate rustdoc, verified workspace publication dry run,
  actual affected archive inventory and the unchanged10MiB ceiling. Cargo.lock
  may change only the existing rdocx-layout dependency inventory, no new package.
  Actual22-archive inspection requires updates to the existing root README,
  rdocx-layout README, rdocx-cli README and scripts/readme_doctests.py numeric
  archive rows. The direct edge adds15 generated lockfile member bytes to
  downstream rdocx and CLI archives. Preserve the64byte compressed tolerance
  and10MiB ceiling. No other row or policy changes are authorized.
- Units: retain truncating constructors. This changes math placement, not unit
  conversion policy. State exact harness deltas before accepting a baseline.

## Hash harness

The seven samples generated by generate_all_samples.rs contain no typed
OfficeMath. All49 existing hash entries and the shared pixel manifest must remain
unchanged. F-X235 has no shared baseline write lease. New equation qualification
uses the existing OfficeMath harness and manifest with independently pinned
native evidence, preserving historical sources, thresholds and outcomes.

## Implementation checklist

- [x] Complete F-X234 and claim the dependent isolated worktree.
- [x] Reproduce actual ink overlap and capture pinned native evidence.
- [x] Correct and independently qualify math geometry and glyph coverage.
- [x] Review exact output deltas, preserved histories and mutation controls.
- [x] Pass scoped verification, microscope and structured preparation.

## Open questions

No unresolved product question. F-X234 completed its scoped dependency-prefix
checkpoint at a1a85471 after integration08978027. Implementation starts after
F-X232 source integration. Lease math.rs and the existing OfficeMath oracle
harness/manifest exclusively. F-X233 owns paginator.rs and F-X246 owns both
native integration entrypoints. Root serializes native Office exports, archive
measurements and shared HLD paragraphs. New actual native producer pins remain
separate evidence identities, preserving every historical threshold and result.
