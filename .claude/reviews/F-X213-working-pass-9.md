# F-X213, repeated rebuild, pass 9

**Reviewed**: Current source guard8f2d3a9cdf50cdbd4365423e27ebd4efe8076aa7ad2e0251e4e1fe6c07ed11e5, the compiler-only byte iteration correction and the actual native21 attempt2. Six controls pass before the content-control control fails.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, repeated rebuild rejects its own generated tab namespace closure

`crates/rdocx/src/field.rs:9502`
`crates/rdocx/src/field.rs:9521`
`crates/rdocx/tests/regression_test.rs:12703`

The property scan treats every local namespace declaration as opaque. The existing content-control control rebuilds twice. An earlier rebuild closes the generated tab stop with the canonical Word namespace declaration. The next rebuild treats that generated declaration as an unsafe owner and selects generic topology, introducing a blank begin paragraph. The actual control reports seven paragraphs against six required. Retain protection for unknown owner facts and declarations while allowing this generated, semantically unchanged namespace closure to survive a repeated standard overlay. Do not relax the count assertion or discard imported opaque data.

Actual failure is retained at `/private/tmp/rdocx-f-x213-codex/.claude/scratch/F-X213-native21/attempt2/toc_rebuild_keeps_the_word_layout_of_a_toc_content_control.log`, SHA75eb750d08c22c52db1aa197128b1ebbd4949b1903bddf7acb7206307b03cf14. Compilation succeeded. No later control receives pass credit.

## Smells

None found.

## Nitpicks

None found.

## Not found

The compiler-only change removes an unconstrained AsRef conversion while retaining direct byte iteration and the same four XML whitespace values. It introduces no new API, file, dependency, panic or behavior change. No source changes were made in this review.
