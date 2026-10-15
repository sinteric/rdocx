# F-X206, gradient placement correction, pass 89

**Reviewed**: R36 external diff686527cdbfa15bf16c53deba9bdcfc644b7ab642280cd5405fa31698b6fb7835, receipt38dd2ef9c933e4504f3fdebc0d51bde513588c945dc9390caf714d4bf03c0ac4 against current d7e9dd6e.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None. D1 from pass88 is resolved. Placed substitution remains enabled for rotation-independent gradients that current placed rendering can normalize.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness, contract, panics, OOXML, tests and structure produced no findings. Two actual placement contexts justify the bounded private style policy. Public effective_shape_style retains effects resolution, while boundsless validation defers only the rotation-dependent gradient colours it cannot paint. Placed connector substitution is unchanged. Existing index, effects and source-preservation errors remain deciding.

Common flip, tile and path early boundaries match concrete_fill, and missing-stop ordering still resolves preceding supported colours. New controls distinguish a real referenced red-to-blue placed unrotated gradient from its unsupported rotated partner, background phClr substitution and a preceding supported phClr error. The original five-boundary test and supported error matrix remain unchanged. Imported types and every existing connector call site were checked for the internal signature change.

Actual focused selectors, changed-crate checks and previously unrun oxml/render scopes remain required. No runtime pass, archive expectation change or feature completion follows. Review ends before distinct adoption and execution.
