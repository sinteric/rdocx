# F-X210 independent PDF reference, pass18

## Scope

ROOT reviewed the source-only PDF reference proposal0770ed79698706596788913a18a1357feb67b932acbfe9f10c223ecc98515b52, its generatorbe8156471ca922e3e1e83ae48bb65588bb639b20291a01b9e034f58359156865 and verifier diff3d9cb17cfcaa0cf348be83201855bf031ccf8c28921e9a492d3f896d3dcc5dde. Original actualR7 receiptf554 remains failed. This review is not comparator execution credit.

## Findings

Zero defects. Zero smells. Zero nitpicks.

The generator at /private/tmp/S91-F-X200-F210-checkbox-pdf-reference-r8-prepared/pdf-reference.py:42 reads the authenticated original full font and the separately reviewed source authority. FontTools draws original glyf paths at52. Quadratic conversion at16 uses the exact two-thirds control-point identity. The PDF transform at56 uses source units48/UPEM, source origin90,123.312 and baseline792 minus123.312. It reads no product PDF, embedded subset outline or product bitmap. Empty PDF resources at26 prevent an implicit host-font or product-image oracle. No fitted displacement or glyph hint compensation is introduced.

The checker remains unchanged and retains the source-derived ROI and positive .99 threshold plus strict wrong-character, blank and6pt displacement negatives. The verifier diff changes only PDF reference selection and binds its independent source proof. Native PNG still uses original source SVG rasterization. Product PDF and independently authored PDF path both use pinned Poppler26.01. Matching their raster engine removes an unqualified cross-engine equality assumption without changing the geometry or required score.

## Result

The correction is qualified for comparator-only execution. Require all four original owner and route comparisons, physical source glyph and unused-resource checks and strict negatives. Font hinting versus path rasterization can still cause a genuine failure, which must be preserved. Reuse the actual four native outputs and original setup without product rerender, relink or installation. Do not relabel the original failed receipt or lower its threshold.
