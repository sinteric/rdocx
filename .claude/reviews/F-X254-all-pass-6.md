# F-X254, all, pass 6

**Reviewed**: separate pass5 D1 remediation against20e2252101797ec160c9c2f163ed5683caff0dc3, 2 changed files, 114 insertions and 2 deletions.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

0. Pass5 D1 is resolved.

## Smells

0.

## Nitpicks

0.

## Assessment

`crates/rpptx/src/embedded.rs:534` filters only InvalidXmlChild from the existing embedded candidate's publication decision. Every preexisting validation category remains blocking at `crates/rpptx/src/embedded.rs:536`. Public validate() itself is unchanged, so the imported diagnostic remains observable. Replacement/removal still use the existing staged package, serialized reopen, ownership, signature and single-publication transaction. No checked-XML admission or source-owner rule is relaxed.

The regression at `crates/rpptx/tests/integration.rs:2616` covers both replacement and removal, each paired with and without DuplicateShapeId. Success requires the retained malformed slide diagnostic, actual changed or removed OLE payload, exact unrelated slide preservation and retained raw invalid island plus MC namespace authority. The ordinary-save baseline acknowledges existing namespace closure instead of promising byte identity to an unclosed imported spelling. The duplicate-ID branches assert the actual old category in the error, unchanged serialized presentation and original extractable payload. These controls distinguish the required finite-category exception from a broad validation bypass.

HLD06:1589-1593 already states the intended report-only versus checked-XML boundary, so no additional HLD edit is required. The approved plan's malformed-source retention and checked transaction scope remain satisfied. No additional correctness, contract, panic, OOXML, test or structural finding arose. No new public API, file, module or dependency appears.

## Evidence and source custody

Readiness SHA25624677a37e4b98cd643545da0b7872635d42be2dbbaf7bd90f91443fbb72e2c83, freeze SHA256a773ba0defad6e91dbd6c1f8d81cad63b1c92911b30ab826df87db7fc49a9f23 and actual binary patch SHA256b608b11f6ab65190512d6385d7f35b3790893ff25d279c4c17678ea7737160fc match the assigned snapshot. All6frozen entries and42declared evidence artifacts authenticate and remain unchanged.

The actual original-source RED fails at the unintended InvalidXmlChild-only OLE replacement refusal. Two successor lexical-baseline failures remain preserved as failures. Final green3 records1PASS covering four replacement/removal and prior-category branches, with the existing embedded regression separately1PASS. Clippy, format and diff checks pass with source/tool snapshots. Production filtering is unchanged across successor fixture refinements. These are inspected author observations with explicit source epochs, not reviewer executions, identical-test-byte RED/GREEN claims or a fresh full-suite pass.

No tests, builds, compilers, product helpers or native applications were executed. No source/test/HLD/progress changes were made. Only this report was written. Review ends and the canonical writer lease returns to root. No confirmation pass is required absent new material.
