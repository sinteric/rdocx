# F-X195, axis fixture precondition admission, pass 29

**Reviewed**: Two tests in one existing carrier, structural SFNT literal
correction patch e4ad9d5788e0cfbbda69b8c7f2585e157c5ae7af113f01f8630d52c1704b2e1f.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in the corrected proposal.

## Corrected precondition and prior evidence limit

The R16 failures are authentic executions of invalid synthetic controls.
They cannot establish a supported valid-input product defect. Prior pass28
authenticated controls and causal assertions but missed structural fixture
metadata. This pass corrects that qualification without rewriting history.

Primary OpenType Data Types requires non-NULL subtable offsets unless NULL
is explicitly permitted. gvar has no such exception for sharedTuplesOffset.
A zero-count shared tuple array must use a nonzero offset. The proposed32
points to the existing empty array at the glyph data boundary. head bit1,
maxp maxZones and hhea contour-derived extrema correct independent metadata
preconditions. Whole SFNT checksums were independently recomputed.

Read https://learn.microsoft.com/en-us/typography/opentype/spec/otff,
gvar, head, maxp and hhea. All four inline arrays match the authenticated
new variable and independently instanced HarfBuzz14.4 controls. Original
glyf, loca, hmtx, cmap, fvar and tuple/delta bytes remain exact. Assertions,
expected coordinates/outlines/width/CMap, production prefix, existing weight
control and current317 source guards remain unchanged.

## Defects

None in the corrected proposal.

## Smells

None.

## Nitpicks

None.

## Not found and limits

Correctness, contract, panics, tests and structure produced no additional
findings in the corrected test-only diff. OOXML is unaffected. Original
R16 RED and installed104 failures remain retained on their original input.
No product normalization is admitted. Fresh actual native and installed
controls must establish outcomes. No licensed ital/slnt-axis parity or
complete F195 acceptance is established. Review writes only itself and ends.
