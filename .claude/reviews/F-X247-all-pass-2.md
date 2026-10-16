# F-X247, all, pass 2

**Reviewed**: Frozen working tree in `/private/tmp/rdocx-s94-x247` against Base `90165c94e582b21e510c0c3c00ab1b6375415ba0`, 18 files, 2,099 insertions and 109 deletions. Diff SHA-256 `4403bf9aef2593bfbaf2d7af75d5241fe1b20c0b4eba76fbe58fc2f8768449b8`. All 3,517 tracked source hashes match snapshot `91db417510bc6543dafad58c3041215c4d6fab0724efa79c78dfd0ec93f5c287` before and after inspection. Read the revised approved plan, progress, pass 1, workflow/microscope and affected HLD contracts. Reused the authenticated unchanged portions of pass 1's full audit and inspected the actual source, test, oracle, documentation and catalog successor. No source changes, builds, cache mutations, GUI actions or remediation.

**Verdict**: 0 defects, 0 smells, 1 nitpick. Microscope exit condition met. This is feature evidence, not final integrated sprint verification.

## Defects

0 findings.

Pass 1 D1 is resolved. Private measured-box terminal advance is recorded at `crates/rdocx-layout/src/math.rs:1051`, propagated through transparent sequences at `:2324`, scaled consistently, and consumed before postscript positioning at `:1394` and `:1560`. Pre-script behavior is retained. Actual positive Cambria MATH corrections for x, f and alpha are independently read by the regression at `:3184`. Superscript positions require raw advance plus one correction, subscript positions require raw advance, and combined attachment checks both. Normal text, pi and multi-character identifier controls remain. Authenticated actual predecessor RED and successor GREEN logs demonstrate that the test detects the reviewed duplicate.

Pass 1 D2 is resolved at `crates/rdocx/src/math.rs:130`: the completed normalized MathML tree is validated before conversion returns. Tests at `:3580` cover leading, trailing and paired synthesized boundary spaces at the exact text cap and one byte above. The installed public mutation test in `crates/rdocx-py/tests/test_core.py` checks atomic refusal and actual saved text at the limit.

The subsequently discovered Pandoc semantics/style-wrapper loss is also repaired. `crates/rdocx/src/math.rs:950` admits mstyle to the existing supported-element selection. Tests at `:3607` cover true/false wrappers, preceding annotation and foreign children, preserved fraction operands and diagnosed unknown style values. This retains the approved stacked-Bar canonicalization rather than inventing forced TeX style fidelity.

## Smells

0 findings.

## Nitpicks

- `scripts/officemath_oracle_harness.py:651`, a whitespace-only line contains trailing spaces, reported by `git diff --check`. Optional formatting cleanup. No required verification gate or runtime behavior depends on this, and it does not prevent the microscope exit condition.

## Not found

- **Correctness**: 0 additional findings. Finite symbol/role mappings, distinct phi variants, contextual text spaces, three explicit math-space tokens and reverse canonicalization retain their approved behavior. Physical MATH Bar padding, direct nested-Bar2mu classification, NoBar and non-MATH fallback clearance, mapped glyph coverage and run-only display clamping remain as reviewed.
- **Contract**: 0 findings. The exact73 inputs and independent semantic expectations remain unchanged. New behavior uses existing signatures, types and carriers. General TeX, forced fraction-style identity, full MathKern and universal script fidelity are not promised. The retained supplementary and current13 observations are qualified below.
- **Panics**: 0 findings. Production glyph access remains guarded by complete cmap and mapped glyph counts. The new completed-tree validation closes the synthesized-text cap bypass. No new unchecked untrusted indexing or overflow route was found.
- **OOXML**: 0 findings. Function, plain-math and normal-text roles, xml:space and binomial NoBar/delimiter structure remain distinct in saved native packages. Existing typed authoring and opaque sibling preservation remain authoritative. Style-wrapper selection does not project annotation text into the expression.
- **Tests**: 0 additional findings. Actual source/package mutations, rendered mutations, positive-font regressions and exact resource-limit controls detect their intended failures. Independent retained Pandoc qualification was executed read-only against all146 actual records, passing72 emitted cases per context with the original degree warning/raw fallback retained. No product conversion was rerun by this review.
- **Structure**: 0 findings. The successor adds one private measured-box accounting field, not a public model field or new abstraction layer. No product file, module, trait, generic, font asset or dependency is added. Root-owned later-plan prose changes introduce no inherited F-X238 implementation.

## Native acceptance and bounded limitations

Independently authenticated every indexed evidence hash in `/private/tmp/S94-F-X247-work/pass2-evidence-index.json`, all321 final artifacts in `remediated-native-attempt-06/receipt.json`, actual consumer binary, command logs and the separately closed runner hash. Receipt SHA-256 is `c610e39604fdeea470b2c1a70fbc6c494d6507841563a43d532bf3fbea7c67d8`. Its source snapshot equals the formal review freeze.

The seven original finite paint controls pass absolute PDF bounds, derived width, actual ink at1pt,150DPI and0.99 SSIM. All seven actual1.01pt group mutations reject. Historical captured six positives and exact-expression clusters pass. Historical negatives remain five rejections with radical passing, reflecting the signed residual. New Word16.114 build16.114.26101123 is distinct from historical16.113.2. The strict installed-old-application check actually refuses the mismatch. Captured replay is not fresh old-Word export.

The supplementary f/x/alpha script comparison remains an actual failed width observation of1.142188pt after1.97656pt improvement. It is not an eighth passing gate. The approved plan at `.claude/plans/F-X247-design.md:197`, rendering contract at `docs/hld/08-rendering-spec.md:2350` and testing contract at `docs/hld/12-testing-strategy.md:1604` explicitly retain this diagnostic and exclude full MathKern. The harness includes it only through the explicit diagnostic option at `scripts/officemath_oracle_harness.py:727`, and its recorded actual exit is1. The source-proven duplicate correction is fixed and meaningful sub-point regressions pass. Existing missing MathKern is a compatible bounded limitation, not evidence of a second application or a finite paint pass. The measured residual alone does not independently prove that MathKern explains every remaining difference.

Current13 original Caladea-request source has byte-identical baseline/current wheel PDFs and identical final deterministic-consumer bytes. Its aggregate PDF/ink1pt and0.99 raster comparison pass, and the genuine1.01pt mutation rejects. This is aggregate qualification, not thirteen individual family passes. Word substitutes Cambria in its native export. A separate matched-Cambria variant retains the baseline's8.833383pt lower-bound discrepancy while current width improves. Plan `:199` and HLD12 `:1607` disclose unavailable historical raw PDF/producer receipt and the preexisting mismatch. Recorded-vector checks, current observations and historical raw replay remain distinct. These limits do not establish full matched-font fidelity and do not fail the approved finite increment.

## Gates, archives and preservation

Authenticated actual successful scoped command logs for changed native crates, no-default fonts, both WASM targets, rustdoc, denied-warning clippy, formatting, hash harness, prose and adapters. The original scoped policy receipt contains a failure caused by inherited offline setup. Its separately authenticated successor `workflow-policy-remediated-02.log` reports182 tests,2 skips and OK. The failure is retained, not relabeled green. Fresh installed wheel/source identities match and11 focused Python tests, typing and stubtest pass.

Authenticated verified workspace publication dry run and README/pixel logs. The separate measurement-only cargo package commands use no-verify, while the publication gate actually verifies22 packages. Independently measured the retained affected archives with the existing read-only helper: rdocx3,750,299/52,049,137/37 and layout348,197/1,911,250/15. Recorded compressed values differ by1 and2 bytes within the unchanged64-byte tolerance. Normalized member totals/counts and10MiB limits hold. No threshold or baseline was relaxed. Combined integration measurements remain integrator-owned.

Attempt05's runner.log hash was stale because the log finalized after receipt creation. The unchanged failed authentication and append-only completion record remain retained. Attempt06 removes the mutable self-hash from its artifact inventory and authenticates the completed runner separately, which this review verified. Earlier overwritten iterative raw product/mutation PDFs remain unavailable. Only retained logs, measurements and initial raster pairs are claimed for those attempts. Original native/source/historical artifacts, reviewed predecessor binary and final immutable bytes remain preserved. No lost predecessor artifact is reconstructed or represented as authenticated.

End of review. Return the tracked write lease to the worker and orchestrator. No confirmation review or remediation is initiated by this pass.
