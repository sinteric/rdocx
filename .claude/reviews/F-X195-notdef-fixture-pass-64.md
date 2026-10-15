# F-X195, notdef fixture, pass64

**Reviewed**: External R40 test-only writer.rs amendment,2225-byte diff within one existing test. ROOT did not author the proposal. The change addresses the actual full PDF gate failure without modifying production.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication

Receipt848d0d5c8ab3c76329818494215cedc4b38e3a2b9c28b87732b570835a1bc06f binds candidate81d9be75d30b10423c5066cd8ef882fd051f9aaf2f15887fad91c9a79ee3fece and diff0af502736d4a25b1fdc2f01bd92916bb765895222905781582304159d9e8453a. ROOT verified all2679 current guards, prospective maps and actual failed native receipte61b514e. Independent exact prefix/suffix comparison confirms only tagged_pdf_with_notdef_does_not_claim_pdfua changes. Production and every other test are byte-identical. Apply check returns0.

## Defects

None. The old fixture omitted font bytes for its contributing FontId1 and now reaches the approved checked missing-font refusal. The amended test explicitly proves that refusal first, then supplies the existing licensed Carlito-Regular bytes under that exact ID. Glyph zero remains the authored glyph. Checked valid PDF output and the original StructTreeRoot, no PDFUA and no Metadata assertions remain mandatory. It now isolates real notdef tagging rather than relying on a missing-font fallback.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness, contract, panics, OOXML, tests and structure have zero findings in this bounded test correction. The source font hash isb4ff23ba370cc95a3c349336b73f9c28514a1371210f89832efc85c4b1ea7131. No new asset, dependency, module or published behavior is introduced. No threshold or assertion is weakened. The distinct gradient failure requires Poppler26.01.0 as its existing assertion states, not a product change.

## Handback

Only this report was written. Source adoption, actual corrected native tests and renewed remaining gates are pending. Prior failures remain immutable and receive no acceptance credit. This review ends and returns control to orchestration.
