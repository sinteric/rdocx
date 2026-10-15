# F-X213, authored bookmark closure and helper, pass 11

**Reviewed**: Actual two-carrier correction after native21 attempt3, including its surrounding source-aware bookmark generation and regression helpers. Complete working diff against HEAD remains eight existing carriers,1927 additions and twelve deletions. Unchanged implementation retains prior independent passes8 and10 authority.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Source authority

Worker `/private/tmp/rdocx-f-x213-codex` remains on `work/f-x213-codex`, HEAD `3723dea217efdd1d1ee37201ec73b0fd1b5dab20`, index `4ef9a90dd6058f25eb6272953a93ba62a8a2732c9acf321059774b9f0f880ed0`. All2852 current byte, size and mode records match `.claude/scratch/F-X213-native21/source-alias-helper-fix.json`, SHA `857c37f8738c3b925f4744512782e46eb1541534120cd358490cb6d2d8b11a61`. Only field.rs and regression_test.rs differ from the earlier compiler-fix map. All2850 other records are exact.

Field.rs is `f4e74ade634ab4b591f1e03b32f99d1249fc409057210f9a5f9c19daaf768928`,840253 bytes. Regression is `e36f6ade592179b9a3cf23b6fc64111eb3ea243433c0883d3192c0aa698c894c`,44126531 bytes. Read approved plan, progress, CLAUDE, workflow, microscope and retained pass9/10 failure and closure evidence. The feature remains in-progress. Review scope is incremental correctness, not renewed runtime or completion authority.

## Checked corrections

Five newly authored bookmark generation sites now close their own canonical Word prefix: `crates/rdocx/src/field.rs:8729`, `:8741`, `:8762`, `:9104` and `:9126`. Local declarations qualify both element names and the generated `w:id` and `w:name` attributes regardless of an omitted canonical ancestor or a foreign ancestor binding. The insertion and repair branches still use the same validated offsets, IDs, ownership predicates and escaped names. They do not rewrite imported bookmarks, ancestor declarations, producer aliases or foreign payload. Local closure on an empty authored element has no scope beyond that element.

`crates/rdocx/tests/regression_test.rs:12507` now counts the helper's actual expanded Word pStyle and val identity. Its temporary wrapper supplies the canonical binding omitted by the existing body slice. Original local bindings remain authoritative. The helper removes only the omitted body's literal opening wrapper, not arbitrary declarations or properties. Attribute order and a redundant namespace declaration no longer hide a genuine TOC entry.

`crates/rdocx/tests/regression_test.rs:398832` retains the standard overlay count matrix and qualifies the helper with a Word alias positive, foreign element negative and foreign attribute negative. At `crates/rdocx/tests/regression_test.rs:398543`, alias and default-namespace rebuilds now reject unknown element and attribute prefixes throughout their actual saved output. Rebuild, native parse and reopen checks remain. The rebound producer case at `:398586` retains the exact foreign numPr and pStyle facts. Existing paragraph counts, generated entry counts and schema/balance assertions are not loosened.

## Defects

0 findings.

## Smells

0 findings.

## Nitpicks

0 findings.

## Six aspects

- Correctness: 0 findings. Authored bookmark closure fixes the unavailable target identity without changing source ranges or target allocation. Expanded style recognition addresses the literal helper artifact.
- Contract: 0 findings. Alias/default ownership support, current marker topology, atomic unsupported boundaries and exact source preservation remain required.
- Panics: 0 findings in the production correction. No new unchecked input access, offset arithmetic or ID conversion is introduced. Test helper unwraps apply to validating source-built controls.
- OOXML: 0 findings. Generated namespace scope is local and exact. Imported aliases and foreign rebinding retain their bytes and identity. Generated attributes are explicitly qualified, not inferred from the default namespace.
- Tests: 0 findings. Deciding counts and repeat/save/reopen assertions remain. Helper positives and negatives distinguish genuine expanded ownership from lookalikes, while full output checks distinguish element from attribute closure.
- Structure: 0 findings. Two existing carriers change. No API, module, file, trait, generic or dependency is added.

## Runtime limits

Actual attempt3 ran all21 controls and reported19 passed and two failed against source290484. Its failed receipt and logs remain unchanged. Neither wrapper failure is relabeled and the new source has no runtime credit. ROOT must run all21 current native controls and the fresh installed controls before scoped completion. No Cargo, runtime, environment, cache, source, product import or UI mutation occurred. Only this report was written. Review lease is returned.
