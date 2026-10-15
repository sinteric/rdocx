# F-X213, read-only namespace compatibility, pass 12

**Reviewed**: Three-carrier test and contract amendment. No production changes. Current source2852 is authenticated against `.claude/scratch/F-X213-native21/source-readonly-compatibility.json`, SHA `49108842b863896ba658f00dcef49551d1d425ec0692175fd7eb25c02c7b32c9`.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authority

Worker `/private/tmp/rdocx-f-x213-codex` remains on `work/f-x213-codex`, HEAD `3723dea217efdd1d1ee37201ec73b0fd1b5dab20`, index `4ef9a90dd6058f25eb6272953a93ba62a8a2732c9acf321059774b9f0f880ed0`. All2852 current byte, size and mode records match. Exactly the design plan, regression_test.rs and HLD04 change against the earlier source map. All2849 other records, including both production carriers, are exact.

Current plan SHA is `e85d95f710fe2b7562c38484ab449ddbd6e103999897d83e2e96ac7d0f6100d0`. Regression SHA is `b02f0479a66e2453918280cf17b8d89d6646ad7e7ba23e8a024fee465e85e3a9`. HLD04 SHA is `58851043c031dcb90962dac6ef7e72cdc6f7939f55d28e315e3fa57581809201`. Read approved design and compatibility amendment, current progress, workflow, microscope and retained actual full-regression failure context. Prior independent production review authority remains unchanged.

## Checked compatibility boundary

The explicit amendment at `.claude/plans/F-X213-design.md:59` supersedes only the historical initial read-only save refusal. The existing production predicate at `crates/rdocx/src/document.rs:13845` requires an empty modeled relationship mask and equality between parsed retained source and the current CT_Document. It inventories qualified IDs from retained source directly. Dirty state or modeled candidates continues through checked namespace replay. HLD04 at `docs/hld/04-opc-and-packaging.md:1737` states this same finite boundary without granting mutable owner admission.

The existing test at `crates/rdocx/tests/regression_test.rs:59828` retains its literal text, zero-width, alias and foreign-lookalike cases. The changed table and cell cases at `:60030` require complete original main-part bytes, repeated saved member payload equality and ordinary reopen equality. Adding a comment still must produce the precise retained tbl or tc namespace-owner refusal at `:60065`, retain source snapshots, publish no comments and preserve all saved package members.

The duplicate paragraph-owner case at `crates/rdocx/tests/regression_test.rs:60110` requires the same literal source and member preservation before mutation. The move at `:60134` must fail with one of two specific existing namespace ownership diagnostics. The test then requires exact source snapshots, saved member payloads, thread IDs/text and the original SOURCE anchor. These are named strict failures, not arbitrary Err acceptance. Existing successful beside-field placement and crossing-field refusal remain separate controls.

## Defects

0 findings.

## Smells

0 findings.

## Nitpicks

0 findings.

## Six aspects

- Correctness: 0 findings. Read-only inventory bypasses no dirty replay proof. Tests distinguish source preservation from permission to relocate comment ownership.
- Contract: 0 findings. The amendment explicitly records the historical compatibility change and retains all mutable refusal obligations. It does not expand comment placement or claim unrelated feature completion.
- Panics: 0 findings in production, which is unchanged. Test-only ZIP and source accesses validate source-built fixtures and unique saved member names.
- OOXML: 0 findings. Entire retained main-part bytes are required, including local aliases. Dirty refusal preserves package members and source owners without normalization or namespace substitution.
- Tests: 0 findings. Exact retained source and member comparisons strengthen successful read-only serialization. Named refusals, metadata and anchor checks preserve deciding mutable assertions.
- Structure: 0 findings. One existing test and two existing contract documents change. No implementation API, module, dependency, framework or new product file is introduced.

## Actual evidence limits

The prior current source passed21 focused controls. Actual full library525 passed with seven ignored. Full regression reported2351 passed, one failed and seven ignored. Its historical initial-save expectation failure remains retained. The amended selector's dirty outcomes and current full regression still require actual qualification. This review does not relabel the failed receipt or claim new runtime credit. No Cargo, runtime, environment, cache, source, ledger or UI mutation occurred. Only this canonical report was written. Review lease is returned.
