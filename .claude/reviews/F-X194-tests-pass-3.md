# F-X194, tests, pass 3

**Reviewed**: one existing test file, one deletion and one insertion after
tests pass2. Full seventeen-carrier R3 diff SHA256
e97d9034cabaa91a99cd79b5589e698f8925b636c61418112a6553e32a4cd169.
Supplemental diff SHA256
b4e85f8ac566818d839f900fac83c2385c34d0bc03e948e7162ad4dc1f79c6d1.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Tests: removing clone at `crates/rdocx/tests/integration_test.rs:21208`
preserves the exact floating-table value and all three signed-indent controls.
TableFloatPosition derives Copy at `crates/rdocx/src/table.rs:128`. The same
anchor and position enter every loop iteration. The deterministic floating
origin and inline signed displacement assertions remain unchanged.

Correctness, contract, panics, OOXML and structure: the sole supplemental
change introduces no new production behavior, input handling, schema change,
module or abstraction. All seventeen current carrier hashes and the full
working diff were independently authenticated. Sixteen carriers, including
all production, bindings, Python tests and HLD bytes, match reviewed R2.
The earlier complete-source and fixture reviews remain their stated scopes.

This is a source review. The initial clippy command actually failed with
clone-on-copy and its receipt remains failed. No corrected clippy, native,
WASM, hash or package result is asserted here. Installed-wheel production
bytes are unchanged, so this correction does not require new wheel builds
or repeating the passed full Python suites. Worker completion still requires
actual corrected scoped and risk gates.
