# F-X166, integration, pass 1

**Reviewed**: F-X166 squash on the S84 prefix through F-X170 and the 3 October PR 265 intake.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Reconciliation

- The F-X163 test tail and F-X166 test tail both followed the former last
  regression test. Both are retained in
  `crates/rdocx/tests/regression_test.rs:43365` and
  `crates/rdocx/tests/regression_test.rs:43516`, with the F-X170 style tests
  retained at `crates/rdocx/tests/regression_test.rs:10488`.
- The two worker branches measured different `rdocx` archives. The combined
  package was remeasured and recorded in `README.md:42` and
  `scripts/readme_doctests.py:400`. Its 36 members contain 7,286,836 bytes,
  and the measured gzip archive is 1,257,810 bytes. The README inventory and
  four root examples passed on the combined tree.
- The HLD edits apply to separate section and revision text. No HLD conflict
  required a semantic choice.

## Checks

Word for Mac 16.113.2 opened and resolved all 40 redlines, producing 80
accepted and rejected documents. The saved results passed 24 of 24 picture
outcomes, 40 of 40 core final-block outcomes and 16 of 16 styled outcomes.
Word's removal of explicit false paragraph-mark bold values is semantically
equivalent to the absent values in four saved documents.

On the combined tree, all four Issue 254 and all four Issue 255 focused
regression tests passed with the worker's 8 MiB Rust test thread stack.
Both F-X170 style tests passed. Formatting, prose, generated skill sync,
README inventory and root examples passed. The hash harness matched all 49
entries. Full README compilation, full verification and sprint review remain
at the final S84 gate.
