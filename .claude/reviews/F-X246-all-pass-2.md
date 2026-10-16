# F-X246, all aspects, pass 2

**Reviewed**: frozen working diff against046a5fc87409f939a248606ce7435a0dfa83058e,14 existing source/test files,1805 inserted and44 removed lines. Approved plan, cited HLD contracts and pass1 findings read before review. No implementation or documentation edits were made during this audit.
**Verdict**: 0 defects,0 smells,0 nitpicks

## Defects

None found. D1 through D4 from pass1 are resolved in the reviewed source.

## Smells

None found.

## Nitpicks

None found.

## Evidence and resolved findings

- D1: font-relative legend metrics meet the unchanged1point native bound. Independently recomputing the actual80by120pixel,150DPI component SSIM from the retained native/product PNGs gives Word0.9995260801031176 and PowerPoint0.9991474873189379, both above0.99. PDF SHA bindings match the qualification receipt. Every key and label plus at least2.5points of padding fits each crop. The original100pixel failed comparison remains retained because it includes Word's unrelated outer chart frame. This qualifies legend components and the separately measured adjacent right plot edge only, never whole-chart paint parity. The old renderer also fails the corrected component observer.
- D2: the Word scanner requires the complete supported qualified paragraph/run/drawing/inline-or-anchor/graphic/graphicData/chart ancestry. An independent installed public probe wrapping the drawing in an opaque owner now refuses both getter and setter, preserving package bytes. Tests separately retain a valid owned chart beside an opaque lookalike.
- D3: copy-on-write records and rewrites only the selected qualified relationship attribute span. All relationship attributes still contribute conservatively to shared-resource detection. The new installed regression preserves the foreign marker's original relationship while isolating the real chart. Workbook and sidecar controls cover both native families.
- D4: legend entrypoints reuse strict XML1.0 admission plus document-level source checks. An independent installed public probe with trailing document text now refuses both operations without mutation. Existing tests also cover outside-root CDATA, references, additional roots, misplaced declaration and namespace-invalid content, while a valid prolog/trailing comment/PI remains a byte-exact no-op.

Evidence is retained under `/private/tmp/F-X246-native-inputs`, including
`legend-qualification-receipt.json`, original native exports, original red
comparisons, actual old-renderer component failures and corrected component
PNGs. The exact reporter's native empty-legend overlay and original font
mismatches remain explicit diagnostics, not claimed equivalence.

## Not found

- Correctness: effective defaults, all five positions, overlay, absent/hidden state, finite bounded layout and shared-resource isolation show no additional actionable defect.
- Contract: both approved Python surfaces and native facades exist. Hidden settings are discarded, position/overlay-only hidden edits are no-ops and effective repeated values preserve bytes.
- Panics: inspected source spans, bounded layout, indexing and scalar admission. No additional actionable panic found.
- OOXML: expanded-name ownership, local shadows, strict document grammar, schema-ranked insertion and opaque preservation have source and installed controls. No additional issue found.
- Tests: real old-candidate failures establish sensitivity for the ownership/document repairs and native legend metrics. Scoped final gates remain the worker's completion obligation.
- Structure: the concrete settings/layout/reference values live in existing carriers. No new module, trait, generic abstraction, dependency or forwarding layer was introduced.
- Bindings: strict bool/value validation, staged publication, stale/foreign/related owner refusal, live grouped PowerPoint ownership and stubs match the approved surfaces.

This review pass ends here. Return to sprint orchestration for scoped gate
completion and separately reviewed HLD/package metadata preparation. No
confirmation source review is required after this clean pass.
