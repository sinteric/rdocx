# Hanji, vertical page stories, floating measure review, pass 8

**Reviewed**: renderer 79e8ab8991b3086e7820b3346c28a53575fa97c4, selected table measure round-trip through landscape transposition
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, exact measure lookup loses the selected table variant after rounding

`crates/rdocx-layout/src/block.rs:315`

The table lookup compares prepared raw story measures to the transposed page's
content width with exact floating-point equality. Reconstructing content width
from page margins can round that value by a few ulps. For a landscape page
792 by 612 pt, a 300.05 pt first header and a 100.1 pt footer, the prepared
measure is 139.85 pt while the transposed measure is 139.84999999999997 pt.
The lookup therefore misses the first-page variant and returns the narrower
conservative primary table, sized for the 400.05 pt default header.

The source-owned public API case loses 100 pt of first-page table length,
painting y=336.05..375.9 pt where its selected band is 336.05..475.9 pt.
Page two uses y=436.05..475.9 pt correctly. The DOCX, both rendered SVGs and
the independent geometry comparison are saved as `fractional-landscape*`
under `../hanji-vertical-docx-design-evidence/final-review/`.

The lookup must tolerate floating-point roundoff in the finite prepared
measure set, with a sub-rendering tolerance. Cache keys and authored geometry
must retain their existing precision. Add an end-to-end fractional landscape
regression with selected-band geometry and source plus cold/warm conservation.

## Smells

None.

## Nitpicks

None.

## Not found

The pass 6 percentage spacing correction remains effective. Original six API
contracts, the spacing contract, 316 Word-layout tests, unchanged hashes and
focused native, byte-only and WASI integration checks remain passing.
Paragraph logical cursors and primary numbering state are unchanged.
Final Hanji 1378739 reached full CI success with 440 native, 87 byte-only
and 403 WASI tests. Renderer 79e8ab89 has passed every selected job except
the still-running Presentation fidelity job. This review finding comes from
an additional source-derived landscape case, not a CI failure.
