# F-X205, Word chart golden attribution, pass 18

**Reviewed**: One constant in crates/rdocx/src/document.rs:36623, one removal and one addition. Immutable proposal manifest eb53b56f9c7e2b3fab3a09e8c499510d65270ac9bc704d03572206c6f7b28038. Source54 remains unchanged during this review.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness and contract: All proposal files, seven references and source-map hashes authenticate. The candidate equals its preimage after exactly one PowerPoint hash replacement. The current preimage is exact. Exact claimed Base records the original selector failure before F-X205. All original failures remain retained.

OOXML and preservation: All three archives have the same ordered 44 members. Old to claim changes exactly presentation.xml and slide1.xml. Removing the exact namespace closure on the default text style and two text bodies, then restoring the two placeholder names, recovers both complete old member byte sequences. Claim to current changes exactly the nine reviewed template members, with every before and after hash matching template-delta.json. Chart, workbook and relationship members remain exact. The inverse archive authenticates to the original complete golden hash.

Tests: The Word golden, chart data, deterministic render calls, pinned pdftoppm version, crop dimensions and exact zero-differing-pixel assertion remain byte-identical. Raw logged outcomes and their hashes authenticate. This admission allows the retained pixel comparison to execute. It does not claim that comparison has passed.

Panics and structure: No production behavior, dependency, file, module or runtime construct changes. No findings in these aspects.

## Limits

This is a bounded test-only source review. The actual successor selector remains required with the pinned rasterizer. Current verified rdocx package evidence predates this same-length test-only constant change and needs explicit final source qualification or refreshed archive evidence. No final sprint verification, Office oracle or issue closure is claimed.
