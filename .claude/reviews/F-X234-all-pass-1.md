# F-X234, all aspects, pass 1

**Reviewed**: working diff against81c5b7d2ea58603767c03c4450188ebe86053196, one file,340 inserted and19 removed lines. Source frozen during review. Approved design and conversion, preservation, resource and oracle contracts read before review.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Not found

- Correctness: supported symbols, upright lim, thin-space preservation, single-character script binding and explicit limited invisible-function normalization were checked against the manually authored expected tree and actual pinned Pandoc3.11 forward/reverse output.
- Contract: conversion remains facade-owned in the existing file, with no new public type, product file, dependency or layout change. Historical Pandoc3.10 test remains intact.
- Panics: changed expect is dominated by a present-character match. Existing input, event, node, text, diagnostic and depth limits remain active, with new cap controls.
- OOXML: source-built public DOCX save/reopen retains an exact opaque sibling. Conversion preserves supported properties without changing the XML ownership layer.
- Tests: original three regressions failed on Base. Exact formula uses an independently constructed typed tree, spacing/property round trips, script-attachment control and actual pinned Pandoc gate. Read /private/tmp/fx234-pandoc.log, which records the passing current oracle run. Whole-crate and remaining scoped gates are still pending and are not claimed by this review.
- Structure: small concrete helper centralizes supported property loss checks, with no trait, generic, wrapper or duplicate parser.

The review pass ends here. Return control to sprint orchestration for remaining
verification and preparation. This review does not complete the feature.
