# F-X204, correction, pass 2

**Reviewed**: Independent correction audit of14 carriers changed from reviewed source2 to source5. Covers four full-pass-1 findings, report exports and typing, binding allocation, CLI help and controls, HLD10 and inherited image fixtures. Original full32-carrier review applies to unchanged product source. Worker HEAD4bb17d5fe0dc6187553fa5f8c63aba224f5d621c.

**Verdict**: 0 defects, 0 smells, 0 nitpicks in this bounded correction. Final archive metadata review remains required before completion.

## Source and evidence binding

Source5 manifest SHA c410e9ae4d7a2a53c9841f998e8d3d40ab1edf68e9aa80e5a1aa0c2031d1cf95 authenticated all3197 tracked files, mode and size. HEAD and logical index SHA2db16757b08c55284a78801006d7446ba9afdb5223c6054d8c160426d091874e remained exact. The14-carrier delta map identifies every source2 difference. Reviewed production code, controls, stubs, exports, HLD and fixture diff. No source modification, runtime test, CP or Office action performed in this review.

Authenticated terminal native regression receipt e239c1c9ab7cabe44b30092a9c90826bff860f6d9b80c94577e18a27b2424fae, final scoped successor7c564c75cc0f509aa73d11e51f1ae9c070f728b54027c6b9deb8ae1dc83a594d and installed typing successor667699e4140222723f02f154b05cb364b2d7aac6ebc04851d348d014fda4cfff with recorded log hashes. Native regression records2396 passes, no failures and7 existing ignores. Actual MemoryError refusal preserves source bytes and held owner. Fresh Word wheel authentication and evidence-only installed successor preserve actual native build and two runtime passes rather than replaying or relabelling failed wrapper attempts. Final49 hash and Word WASM successor are terminal. Archive metadata remains outside this review boundary.

## Prior findings

D1 resolved. Public field parser first requires exact19-byte ASCII civil syntax, then Gregorian and time bounds. Fractions, missing seconds, date-only, offsets and leap seconds refuse. Help and deciding CLI controls agree.

D2 resolved. CLI filename and full-path facts derive from canonical selected input, never staging output. Saved and reopened filename/path assertions retain original input bytes.

D3 resolved. Common FieldsUpdateReport retains ordered original evaluation identities, instructions and caches, complete outcome facts, ordinary/layout reports, combined diagnostics, total and saved-cache policy. CLI and Python consume the same projection. Exports and stubs agree.

D4 resolved. Python allocates nested outcomes, evaluation/diagnostic tuples, layout and enclosing owned report before publish_api_candidate. Returning the owned Py report requires no enclosing result allocation after publication. Injected allocation failure distinguishes rollback and preserves held owners.

Inherited fixture corrections use decoded PNGs with CRC-valid private ancillary identity markers. Exact full payload assertions and ownership/refusal checks remain. Checked production decoder is unchanged. Source4-to-source5 borrow correction changes test code only.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness, contract, panics, OOXML, tests and structure produced no additional finding in this correction boundary. No new trait, generic, product file, module, crate, dependency or reverse family edge introduced. Outcome dictionaries are owned snapshot data under the documented Python contract.

Review ends here. Final measured archive metadata and later source changes require their own bounded review before full completion.
