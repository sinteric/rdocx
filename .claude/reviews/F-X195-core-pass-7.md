# F-X195, incremental core, pass 7

**Reviewed**: Two existing files, 23 additions and 12 deletions. Root separate
runtime remediation R9, not a whole-feature confirmation pass.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Identity and scope

Worker branch work/f-x195-codex, HEAD
2a9bf14072231e46fcb34df70f88cf7aa79fbd79. F-X195 is in-progress in the
worker CURRENT_SPRINT and BACKLOG. Read CLAUDE, WORKFLOW, microscope,
the complete approved plan, cited rendering, bindings and toolchain sections,
progress and earlier core review. The contract requires centered 1/24em
synthetic stroke, round joins and caps, baseline-relative 12-degree shear,
unchanged shaped advances and independent static-instance controls.

Receipt /private/tmp/S91-F-X195-runtime-remediation-r9/receipt.json SHA256
df5e1114431c436f9094970942e5b7da57de6dd64e91995d2d130a8527d90686.
Exact remediation.diff SHA256
b72a1407093306a007203272579a9bb3364fb2aad21cac02922015107e6207df.
Current font.rs SHA256
1d2aa042d5bc0a001985c8d129884419b1fbed89c45c8c11f3ef537eb88ba3ac,
raster.rs SHA256
594a2c8f3b1809933c240bdbeb85f6016d891dcdbb0a48d79ba2e1548f319fbe.
Independent inverse application in memory recovers both recorded previous
hashes exactly. All other 189 tracked Rust files match the authenticated
R8 manifest bytes and modes. No source was changed by this review.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Aspects checked

Correctness: zero findings. crates/oxml-pdf/src/raster.rs:997 preserves the
original fill transform and painting order. At :1001 only the synthetic-bold
stroke path is scaled uniformly from font units into points. Its :1006 width
is font_size/24, with the same round cap and join. The residual :1015
transform retains Y inversion, run-relative shear, placement, outer affine
transform and mask. Uniform scale commutes with the reflection, so ideal
painted geometry remains the approved centered stroke before shear. Advances
at :1021 and :1024 remain unchanged. No new broad transformation of ordinary
fill or static genuine bold faces is introduced.

Contract: zero findings. The point-unit stroke addresses scale-dependent
round-join approximation without changing approved synthesis constants.
The inspected tiny-skia-path0.12.0 path_geometry.rs:876 converts round-join
conics with fixed0.25 path-unit tolerance. Its stroker.rs:1430 constructs
those arcs in the original path coordinates. The painter resolution scale
therefore does not make font-unit and point-unit conic subdivisions identical.
Point-unit normalization uses the same unit system as ordinary layout paths.
This is a source explanation, not proof that the coverage control now passes.

Tests: zero findings. crates/oxml-pdf/src/font.rs:570 and :575 now draw and
remap the source font character glyph instead of using the independently
subsetted oracle glyph ID as a source ID. Authenticated diagnosis receipts,
both source/static font hashes and exact retained hb-shape outputs establish
source U+0E20 glyph79 versus static-oracle glyph71. Both selected700 oracle
advances are637. The independent static700 bounds remain [22,-10,567,561].
The :580 advance, :584 bounds, :599 declared width, static-instance and
ToUnicode assertions retain their prior expectations and tolerances. No
production width repair or acceptance relaxation is inferred from this test
fixture correction. The unchanged raster style matrix still refuses a
coverage difference above one level, and the independent painted-rectangle
control remains intact.

Panics: zero findings. New unwrap calls at font.rs:570 and :571 belong to the
fixed bundled-font test fixture. Production path transformation remains
fallible and conditional, with no added unwrap, indexing or user-input panic.

OOXML: zero findings. These changes do not edit OOXML parsing, child order,
namespaces, unknown XML preservation or source serialization.

Structure: zero findings. No new file, module, dependency, trait, generic,
flag or public API. The concrete stroke algorithm remains beside the glyph
painting implementation.

## Runtime qualification

R8 actual PDF exit101 with four passes and two failures remains retained.
The named combined painted-rectangle PDF test passed in that failed target.
The regular-only raster matrix failed only the bold cases, each with worst
coverage difference32. Root also reports actual Word exit101. Neither failure
is waived or relabeled as a pass by this incremental review. R9 rustfmt exit0
is recorded in the source receipt. No Cargo, import, render, oracle invocation,
mtime change or corrected runtime was performed in this pass. Corrected PDF
controls, affected Word controls and all remaining required gates are pending.
The prepared historical reversal remains unexecuted and requires separate
authenticated current GREEN evidence and explicit leases.
