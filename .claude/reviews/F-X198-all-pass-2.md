# F-X198, all, pass 2

**Reviewed**: Independent review of the frozen working diff against Base1f163e4aff8d9cbdbad4fad53c8d2c67fc4ae811, 7 tracked files, 526 additions and 10 deletions. Approved plan, source ownership helpers, HLD sections, progress notes, prior pass1 and corrected standalone test receipt inspected. All eight frozen source, plan, HLD and prior-review hashes matched before writing this review.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None. Prior pass1 S1 is resolved.

## Nitpicks

None.

## Not found

Correctness: `crates/rdocx/src/document.rs:8309` reads qualified structural routes in the retained source. The opaque-parent condition at `document.rs:8339` prevents foreign and extension wrappers from establishing modeled copies. The final predicate at `document.rs:8388` requires containment of the selected physical owner and another branch of the same AlternateContent. Identical text or a later unrelated box cannot select an alternate. Descendant cell owners use the same containment condition.

Contract: `crates/rdocx/src/document.rs:27106` applies the approved explicit refusal after expected-count mismatch and zero-match handling, before any source splice or live publication. Existing source correspondence checks and replacement validation precede the guard. Document-wide replacement is unchanged. HLD10, HLD12 and HLD14 describe this checked scoped refusal without expanding the public API.

Panics: the production diff adds no unwrap, expect, unchecked indexing or slicing. The detector uses checked stack access and the existing checked element-end reader. Parsing errors return through the existing error path before publication.

OOXML: the guard emits no XML, so refusal does not rewrite namespaces, opaque bytes or schema order. The paired control at `crates/rdocx/tests/regression_test.rs:397782` compares the complete package and retained live story with comment threads. Namespace-qualified Word, MC, DrawingML and VML routes distinguish actual ownership from lookalikes.

Tests: prior S1 is corrected at `crates/rdocx/tests/regression_test.rs:397855`, where the relocated foreign MC wrapper retains the VML and Office namespace bindings. Every standalone control now checks element prefixes at `regression_test.rs:397871` and attribute prefixes at `regression_test.rs:397880` before parsing. The exact corrected standalone regression receipt records exit0, 1 passed, 0 failed, 2337 filtered out and 0.35 seconds test runtime after 1 minute 55 seconds compilation. This test executes all five topology variants and their save/reopen assertions. The earlier clean text-box selection passed 39 of 40 controls, with this fixture as the sole remaining failure. A complete final 40-control rerun is not claimed.

The controls at `crates/rdocx/tests/regression_test.rs:397918` distinguish multiple identical boxes, unequal alternates, MC aliases and multiple Choices. `regression_test.rs:398025` retains document-wide replacement of both copies with count one. `regression_test.rs:398040` covers the descendant table-cell refusal and preserves mismatch and no-op outcomes. The added Python control at `crates/rdocx-py/tests/test_core.py:4132` checks error mapping, complete bytes and reusable held handles. Its execution against a fresh installed worker extension remains pending.

Structure: one private detector in the existing source file reuses concrete namespace and story-frame helpers. No new production file, module, trait, generic, dependency, feature flag or public method is introduced.

This review ran no Cargo or Python test suite and changed only this review file. Full changed-crate tests, fresh installed Python execution, archive measurement maintenance and the scoped verification gate remain pending. Later generated archive carrier changes require their own independent final-diff review. This clean microscope result does not complete those gates.
