# F-X195, notes arithmetic correction, pass47

**Reviewed**: R31 numeric-only proposal, one existing Python test carrier,
one arithmetic expression and explanatory comment. No production change.
**Verdict**: 0 defects,0 smells,0 nitpicks. Unapplied and unexecuted.

## Authentication and correctness

The prepared receipt SHA44287e0695ef00cd7f614c3446763704cba49dbb7989b3824d605d6b9fd5e050
and numeric diff7ab22acf4a2f949a166f633e50aedbd1214134f9b0dc7b5bbc11140d9d45c173
were authenticated. Independently replacing the single original expression
and adding the comment reproduces the entire candidate byte-for-byte.
Candidate AST parsing succeeds. All317 source guards and2679 tracked guards
match the current worker, including byte digests, sizes and modes.

The PDF-font metric helper in test_documented_examples.py:6169 computes
advance times points divided by UPEM. The path helper at6406 instead multiplies
advance by the precomputed quotient, introducing a floating-point association
difference. The proposed expression uses the same physical unit conversion
order while retaining independently read original font metrics. The deciding
exact advance and ROI equality at6537 is unchanged. Quadratic and cubic
references, source owners, negative controls and the .99 threshold remain
unchanged. No tolerance, pixel adjustment or production oracle is introduced.

## Defects

0 defects.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Not found

Correctness, contract, panics, tests and structure produced no findings in
this arithmetic-only proposal. OOXML behavior is unchanged. This review does
not certify the separate variable700 raster diagnosis or counterfactual plan.
The actual three-pixel difference remains unexplained and the failed full104
receipt remains failed. Actual fresh acceptance, native SVG controls, caller
controls, scoped riders and full feature completion remain required.

## Review boundary

Only this review was written. No worker, proposal, cache, mtime, Cargo,
application, baseline or intake mutation occurred. Return to orchestration
before any adoption or execution.
