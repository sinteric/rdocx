# F-X254, all, pass 7

**Reviewed**: remediation4 test-only diff against2636dca5a7f9791c39b7a70c3bb8e68e151a99ac. One existing integration test changes one lexical lookup to the existing qualified helper, 4 insertions and1deletion.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Assessment

`crates/rpptx/tests/integration.rs:21019` now locates the actual qualified grpSp start through the existing owned_xml_starts helper. The helper at `crates/rpptx/tests/integration.rs:30084` resolves namespace URI and local name while retaining the physical byte offset. It does not depend on an exact bare opening-tag spelling.

The captured saved slide contains exactly one group start with a locally preserved p14 namespace declaration. That valid tag cannot match the old literal `<p:grpSp>`. The new first qualified group is the same intended fixture owner. This corrects an accidental lexical assumption without relaxing namespace identity or changing production serialization.

The transform-before-preserved-fill assertion and all existing parsed x/y/width/height/rotation assertions remain unchanged. The test still requires the original preserved fill and inserted transform serialization. No fixture, geometry expectation, product code, HLD, dependency or public behavior changes. The approved F254 contract preserves required namespace scope, so the assertion should tolerate its actual declaration.

## Evidence

All3frozen source/reference entries and29declared private artifacts authenticate. The original H3 log matches its retained SHA and records412PASS/1FAIL/29ignored, not a passed command. The diagnostic capture reproduces the bare-tag lookup failure and retains actual XML plus the single group tag. The focused successor records only this test1PASS. Scoped clippy/format/diff evidence passes. The actual working patch matches the retained patch. Frozen files remain unchanged after review.

This report inspects retained source-qualified evidence and does not rerun it. No tests, builds, compilers, product helpers or native applications were executed. Only this report was written. Review ends and the canonical writer lease returns to root. No confirmation review is required absent new material.
