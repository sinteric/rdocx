# F-X213, repeated rebuild correction, pass 10

**Reviewed**: Current working source and the two-carrier namespace correction against the actual compiler-fix freeze. Complete working diff remains eight existing carriers,1843 additions and six deletions. This pass independently audits the incremental correction and surrounding ownership scanner. Unchanged implementation retains pass8 authority.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authority and source identity

Worker `/private/tmp/rdocx-f-x213-codex` remains on `work/f-x213-codex`, HEAD `3723dea217efdd1d1ee37201ec73b0fd1b5dab20`, index digest `4ef9a90dd6058f25eb6272953a93ba62a8a2732c9acf321059774b9f0f880ed0`. Authenticated all2852 SHA, size and mode records. Sorted indent2 final-LF source map digest is `290484f316c2e7f90464776861140b1bc8785b77d17fe84c5e44a35e750577a9`. Against `.claude/scratch/F-X213-native21/source-compiler-fix.json`, only field.rs and regression_test.rs differ. All2850 other records are exact.

Current field.rs is `7fad20bb2d81143b6bd43930d80c15ad2bea3692ef0e753ebd1e8ead98b970c1`,840038 bytes. Regression is `b8ebead2f9b4b205edb7e1920dd5a77cfe9f684e482ecd73a741481dfa2c31ce`,44122861 bytes. Read approved plan, spec references, progress, workflow, microscope, prior pass8 and actual-failure pass9. Feature remains in-progress in CURRENT_SPRINT and BACKLOG.

## Prior defect closure

Pass9 D1 concerns rejection of the generated tab namespace closure during a repeated content-control rebuild. `crates/rdocx/src/field.rs:9507` now exempts only canonical `xmlns:w` bound to the exact Word URI on a qualified, correctly spelled `w:pStyle` at depth2 or actual `w:tab` at depth3 inside the owned tabs collection. The scanner determines expanded element identity through NsReader before this exemption. Foreign rebinding cannot pass the Word predicate. Default declarations, other prefix declarations, unrelated attributes and opaque content retain the ordinary guard at `crates/rdocx/src/field.rs:9526` and `:9539`. This is not a general namespace or opaque-owner exemption.

The existing positive control at `crates/rdocx/tests/regression_test.rs:398780` exercises three stop forms, two style forms and single/multiple old entries. Every combination retains exact paragraph counts, begin/end ownership, removal of the old stop, repeated rebuild and saved/reopened XML equality. This is twelve fixture combinations, each rebuilt twice, with no new selector. Existing opaque owner controls at `crates/rdocx/tests/regression_test.rs:398912` retain foreign declaration, producer attribute/entity and started-style payload refusal or source-preserving generic fallback. Existing content-control count expectations remain unchanged.

## Defects

0 findings.

## Smells

0 findings.

## Nitpicks

0 findings.

## Six aspects

- Correctness: 0 findings. The exemption is constrained by exact declaration key, value, qualified owner spelling and depth. Repeated generated closure can remain eligible without admitting rebound lookalikes.
- Contract: 0 findings. Ordinary valid standard overlays remain required. Unknown owner facts still choose protected generic topology or the existing atomic refusal. No acceptance boundary was broadened.
- Panics: 0 findings. Attribute decode errors propagate through the existing Result path. The change adds no unchecked source indexing or arithmetic.
- OOXML: 0 findings. Expanded Word identity remains authoritative. The exception concerns a redundant canonical binding attached to an owned generated property. Namespace closure, schema ordering and opaque end-root preservation remain in the unchanged source-aware paths.
- Tests: 0 findings. The standard-overlay matrix directly distinguishes the former false opaque classification. Count, marker and exact save/reopen checks are retained, alongside unknown-owner negatives.
- Structure: 0 findings. Two existing carriers change. No public API, dependency, file, module, trait, generic or forwarding wrapper is introduced.

## Evidence limits

Actual attempt2 failed the required content-control test after six earlier controls passed. That log and pass9 remain historical evidence and are not relabeled. All21 current native selectors and two fresh installed controls still require ROOT execution. This review supplies source correctness evidence, no new compilation, native, installed, paint, hash or completion credit. No Cargo, runtime, source, environment, cache or UI operation occurred. Only this review file was written. Review lease is returned.
