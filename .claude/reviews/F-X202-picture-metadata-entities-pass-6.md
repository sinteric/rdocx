# F-X202, picture metadata entities, pass 6

**Reviewer**: fonts_resume, independent read-only review.
**Reviewed**: Immutable picture-metadata-entities-capsule-r6, one complete drawing.rs before/after pair and the 3,876-byte incremental correction. Scope is pass5 D1 URI/value decoding and its new imported-source control only.
**Verdict**: 0 defects, 0 smells, 0 nitpicks. Pass5 D1 is resolved within this scope.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Aspect results

Correctness: 0. Contract: 0. Panics: 0. OOXML preservation: 0. Tests: 0. Structure: 0.

## D1 disposition

Both the extension URI and decorative/@val now use decoded_and_normalized_value with XML 1.0 and the reader decoder before comparison. The existing namespace-qualified direct extLst/ext/decorative chain and unqualified scalar attribute admission are unchanged. Unknown URI, foreign decorative namespace and nested opaque carrier remain unrecognized. Errors propagate as checked parser errors, with no new fallback default or mutation.

The new control serializes an authored decorative inline, gives its physical drawing scope explicit namespace bindings, replaces the extension's opening brace with &#123; and its marker value with &#49;, and asserts a genuinely changed source. It verifies decorative=true and retained title for both conventional and aliased marker prefixes. Foreign namespace, incorrect URI and an extra opaque nesting level remain false. Parsed raw inline serialization stays verbatim within the original drawing source. This tests the actual imported attribute decoding seam rather than replacing expected values from parsed output.

## Authentication and actual limits

Receipt SHA256 a81d5c0b48362811ac592ba7f3c22dd1b1abcd71fa2a0bbe809ac9f2f7a145f7. Manifest SHA256 0da2c45ad92cade0cdff71f916e3c7be7f475a127fefa1f348bedd80fe5789f6. All 15 manifest artifacts were authenticated by size and SHA256. The preimage is byte-identical to pass5's reviewed drawing.rs. The after image is bound to the batch26 source map b835e9ed8847315f266527b54bd8286a9f49efac77bfabab885b5a06f07c71b1.

The retained batch26 entity selector genuinely ran 1 test, with 1 passed, 0 failed and 0 ignored. Its receipt has sourceAfterMatches and ownedLeaseSafeToRelease true, natural ESRCH group cleanup and no signals. Earlier batch24/25 setup failures and logs remain authenticated and visible. Batch25's native/binding check exited 0, but is not described as a fresh batch26 check. The separate physical shared-picture selector also passed in batch26, but its production/inventory contract is excluded from this correction review.

No installed API, anchored-marker runtime, whole picture inventory, Office paint, full feature or fresh scoped Clippy credit is granted. No source or runtime changes were made. This review ends at this report.
