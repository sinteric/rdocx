# F-X195, core, pass 1

**Reviewed**: frozen worker core production R1, eight existing files,
1039 added and removed lines. Worker Base2a9bf140, freeze SHA256
4c1a9a1c0eb1551aa20734f4490c78a96f54d57bf7691c24537e8d10daa7c6bc
and diff43152c1ed2974a3f2079120c83ad38bd6a2e958899bb5cddf8e48d1c255f022a.

**Verdict**: 1 defect, 0 smells, 0 nitpicks. This is an incremental core
review. Provider, facade, binding, SVG wiring, private ink bounds, scoped
gates and full feature acceptance remain outside the completed phase.

## Defects

### D1, combined synthetic styles use different stroke geometry in PDF

`crates/oxml-pdf/src/writer.rs:1347`
`crates/oxml-pdf/src/writer.rs:1843`
`crates/oxml-pdf/src/raster.rs:1008`

For a regular-only face requested as bold and italic, raster paints a round
1/24em stroke through the baseline shear transform, deforming both outline
and stroke. PDF instead puts the shear in the text matrix and sets stroke
width in user space. Its stroke remains round in that user space while the
outline is sheared. The two operations differ at slanted edges and corners.
The same split affects multilingual runs at writer.rs:1390. This violates
the approved equivalent synthesis geometry contract.

[PDF32000-1, section9.3.6](https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/PDF32000_2008.pdf)
specifies that text stroke graphics parameters are interpreted in user
space. Apply the baseline shear consistently to the complete painted run,
including its stroke, while retaining the text layer, positions, existing
group transforms and ActualText geometry. Add an independent combined-style
PDF raster comparison alongside the existing analytic raster control.
No local PDF pixel comparison has yet run, so no measured pixel count is
claimed. The mismatch follows from the cited operator and transform paths.

## Smells

None found in the frozen core scope.

## Nitpicks

None.

## Not found

Correctness otherwise, contract otherwise within this core phase, panics,
OOXML, test construction and structure produced no additional finding.
Checked PDF preparation is completed before writer allocation and the
archival path consumes the returned prepared map. The writer preserves
existing successful layout-font allocation order. Genuine variation and
original collection bytes remain owned by the selected font identity.
All eight frozen source hashes were independently checked. No source or
implementation file was changed during this review. Compile, test and
hash acceptance are still pending.
