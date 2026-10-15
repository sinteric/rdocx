# F-X213, read-only rich-query qualification, pass 13

**Reviewed**: Three-carrier incremental test and contract amendment against source491088. Production remains unchanged.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Source and scope

Authenticated all2852 byte, size and mode records against worker `.claude/scratch/F-X213-native21/source-readonly-query-qualification.json`, SHA `62a3d88de220d5a61da683a85ce6c183d0182af2e5ec9c723104c5af47d49c2c`. Only the F-X213 plan, regression_test.rs and HLD04 differ from source491088. All2849 other records and production are exact. Worker HEAD remains `3723dea217efdd1d1ee37201ec73b0fd1b5dab20`, index `4ef9a90dd6058f25eb6272953a93ba62a8a2732c9acf321059774b9f0f880ed0`.

Read workflow, microscope, compatibility amendment, current progress and actual preceding focused failure. Scope is the query assertion correction, not a renewed independent review of unchanged production or a feature completion verdict.

## Checked assertion boundary

At `crates/rdocx/tests/regression_test.rs:60134`, the existing rich query must report the exact retained paragraph namespace-owner error before the attempted move. After the named strict move refusal, `crates/rdocx/tests/regression_test.rs:60164` requires that same query error. These assertions do not substitute arbitrary failure or imply that exact retained save grants richer query admission.

The independent literal evidence at `crates/rdocx/tests/regression_test.rs:60140` and `:60169` calls the existing `CT_P::comment_range_text_from_source` on the original complete main member and on actual saved members after refusal. Both must return SomeSOURCE. The source reader at `crates/rdocx-oxml/src/text.rs:6509` validates the complete input for unresolved element and attribute prefixes before selecting qualified Word markers and identity attributes. This preserves raw range ownership without relying on the refused canonical rich projection.

All preceding original-main equality, repeated saves, ordinary reopen, snapshot equality, member equality, thread IDs/text and named move-refusal assertions remain. The amendment at `.claude/plans/F-X213-design.md:68` and `docs/hld/04-opc-and-packaging.md:1742` explicitly separates richer replay from raw-source literal evidence. It does not broaden comment placement, mutable ownership or serialization acceptance.

## Defects

0 findings.

## Smells

0 findings.

## Nitpicks

0 findings.

## Six aspects

- Correctness: 0 findings. Before/after query refusal is qualified independently from unchanged raw anchoring and failed mutation.
- Contract: 0 findings. Exact retained saves remain permitted only under the previously reviewed unchanged-source boundary. Mutable and rich-query replay guards remain required.
- Panics: 0 findings in production, which is unchanged. Test unwraps qualify known source-built operands and exact expected failure paths.
- OOXML: 0 findings. The raw reader validates expanded namespaces and full-source closure, preserving complete source bytes and identity-qualified markers.
- Tests: 0 findings. Exact named errors replace an unsupported rich-query success assumption. Independent original and saved raw-source assertions retain meaningful SOURCE anchoring, alongside all atomicity and package checks.
- Structure: 0 findings. One existing test and two existing contract documents change. No production API, dependency, module or framework is added.

## Actual evidence limits

The source491088 focused run reached its final rich query after member, thread and snapshot checks passed, then failed with the retained paragraph namespace-owner error. Its failure remains unchanged. ROOT reports the source62a3 focused run subsequently completed successfully in readonly-compatibility-focused-2. This review is source auditing and does not independently authenticate that runtime receipt or expand it to full regression, installed, paint or completion credit. Full current scoped gates remain required. No Cargo, runtime, environment, cache, source or UI action occurred. Only this report was written after the canonical claim window ended. Review lease is returned.
