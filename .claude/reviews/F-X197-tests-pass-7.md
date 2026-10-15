# F-X197, tests, pass 7

**Reviewed**: The R13 to R7 remediation in one existing test carrier, two additions and two deletions. All 318 guarded inputs and modes were checked. Only the two error assertion lines differ from R13.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness and contract: malformed border and shading strings now expect the approved parameter-qualified ValueError from the shared extractor. The actual R13 full suite failed at the old border error assertion, with 353 passing tests and one failure. Both setters validate before mutation at crates/rdocx-py/src/table.rs:1238 and crates/rdocx-py/src/table.rs:1260. The shared malformed-string contract is at crates/oxml-py-support/src/lib.rs:15.

Tests: the whole-document byte equality and subsequent live table, row and cell operations remain at crates/rdocx-py/tests/test_formatting_tables.py:479. Unrelated border-width, margins, grid and height error assertions retain their prior contracts. The exact two-line inverse reproduces the authenticated R13 carrier hash. No production, package, manifest, wheel or installed input changed.

Panics, OOXML and structure: this assertion-only delta introduces no parsing, mutation, indexing, new abstraction or file. The two messages match the actual parameter names and six ASCII hexadecimal digit rule. No findings in these categories.

The clean review does not credit pending R14 execution, the unrun full PowerPoint suite or remaining scoped riders. This review wrote only this report and ends here.
