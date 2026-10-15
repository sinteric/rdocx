# F-X207, analytic paint reference, pass 48

**Reviewed**: Two external Python candidates and source-only geometry evidence. Receipt17a5f6e427bafe374b1fe3f57453cdbe601cec6a78cbfed5a01d57a7e13597b2, source9e7aacc88ed911b17e61b24bf5764b8969e7e48d78452133c204f3727d5c488c.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None in this bounded diagnostic.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness, contract, tests and attribution produced no findings. The independent reference reads authenticated original OOXML and original Arial. It admits only explicit uniform horizontal ASCII scenes, rejecting unsupported cases. Its baseline and anchor formulas agree with the unchanged ordinary policy at crates/rpptx-render/src/text.rs:1089 and :1113. The original group and owner matrices retain source flips and rotations. Eleven physical extents match refreshed source exactly. Two affine coordinate components differ by one EMU, explicitly reported and not fitted.

Glyph contours and shaping inputs come from the original font. The comparator derives its crop from independent transformed source ink bounds, requires visible ink and preserves the .99 threshold. Six-point shift, blank and wrong-glyph references must each fail that threshold. No actual bitmap supplies an origin correction. Qualify both raster engines and source/tool bytes, then execute the concrete bounded comparisons under the owned lifecycle. A raster or shaping failure remains failed and requires attribution.

The native 9823EMU allocation difference remains a separate unresolved predicate. The preexisting ordinary baseline difference cannot justify changing fractional stored rendering. Native affine, rich vertical and caller700 evidence remain distinct barriers. This review accepts neither paint parity nor feature completion. Review ends here.
