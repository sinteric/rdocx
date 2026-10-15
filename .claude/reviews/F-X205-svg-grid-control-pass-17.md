# F-X205 SVG Grid control proposal, pass 17

**Reviewed**: Immutable Word-SVG-Grid-control-proposal-1. One existing SVG test carrier, one additive106-line test. Unadopted and unrun.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Bound input

Manifest SHA `2cf9380e81e46e5a001bcb29b36dfae5ede4b1dfb2a9ae68df513ce218c96e47` and all three artifact hashes/sizes authenticated. Before SHA `52561091666eb1004e3ece2e853b62121651db946340b8696c92d8ecc5014262` equals the complete reviewed whole-source49 SVG carrier. Candidate SHA `6bc727fcfc368770079960d084117b41710d45b82af8b4d8570e235ec6a9ab1a`, diff SHA `956d3fd4049e830b37feac30baf9f846ef0fbe28a171b9e3d3bb18bf825a62e3`. Removing only the new test yields byte-exact preimage, so the Grid production arm and every existing helper remain unchanged.

Paths below refer to `/private/tmp/rdocx-f-x205-codex/.claude/scratch/F-X205-start-1/Word-SVG-Grid-control-proposal-1/svg-candidate.rs`.

## Qualification

At3642 the proposed test supplies a2x2 source palette with opaque red/green, half-alpha blue and quarter-alpha black. Its six expected RGB samples per origin are literal source-derived values, not values read back from emitted SVG or the production rasterizer. At72DPI the existing120x80pt page maps one point to one pixel. The32x32pt painted region begins at8,8. Pixel-center phase relative to each8pt tile correctly selects the declared cells for both8,8 and negative3,negative5 origins. The17,9 and25,13 points independently decide repetition.

At3717 both resvg SVG pixels and the library PNG backend must meet those independent RGB/opaque-alpha values. Blue over white is127,127,255 and quarter-black over white is191,191,191, within the unchanged one-byte tolerance. Both actual outputs have a white page background: SVG production at54 emits it explicitly, and the existing PNG backend paints white. Pixmap premultiplication therefore does not distort these opaque sampled RGB values. The resvg helper at3408 uses exact scene fonts, no host discovery, with120x80 output scaling unchanged. This test contains no glyphs.

At3730 the full-page luminance SSIM must be at least0.99. At3734 the single exact pattern origin is shifted by one point, with source dimensions, colors, path and page unchanged. The proposal asserts the replacement changed bytes before rasterizing and requires the same comparison score below0.99. Six source-operand samples and this phase-negative prevent a shared-output comparison alone from serving as its authority. The finite page leaves the1024-pixel colored area large enough to contribute to the whole-page comparison. Actual negative-control discrimination remains to be executed, not presumed from this static review.

The comparison's native PNG means the library raster backend, not Microsoft Office. Its independent rendering engine and original palette checks qualify backend transport only. It does not prove native Office pattern parity or the full F-X205 paint contract.

The required oxml-pdf dependency and pinned resvg0.48.1 dev dependency already exist in crates/rdocx/Cargo.toml. Existing layout, rasterise_with_resvg and luminance_ssim helpers are reused. No dependency, feature, module or product code is introduced.

## Findings

Defects: 0. Smells: 0. Nitpicks: 0.

## Not found

Correctness, contract, panics, OOXML, tests and structure: zero findings within this test-only proposal. Samples, negative origin, repetition, alpha and background assumptions follow the fixed source operands. No threshold or production behavior is changed.

## Limits

No runtime execution, successful negative-control result, source adoption, new native Office acceptance or whole-feature completion credit is granted. The candidate must execute its actual sample and0.99 positive/negative predicates after authorized adoption. No source or runtime changes were made by the reviewer.
