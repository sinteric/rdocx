# F-X201 focused geometry and even-footer review, pass 1

Reviewer: Codex fonts_resume. Independent review of the two operation bodies and their focused controls. Verdict: 0 defects, 0 smells, 0 nitpicks.

## Bound input and scope

Immutable input: `/private/tmp/S91-F-X201-geometry-even-review`. Receipt SHA-256 `f18aa8809e1dd22e7bdacc2ca631704ab71c19eba4fe68ad19715874f039af9c`. All three copied carrier byte lengths, modes and hashes authenticated. HEAD `2d2f4b8f133e3a4c1a097154d39119552d8050ec`, declared index SHA-256 `3c6e655bc171159b300e85d478bd5ad879ba4998d89e67b5ce188bc992c4f518`.

The inspected `create_section_story` span starts at `crates/rdocx/src/document.rs:24311` and ends before the following variant-operation documentation. Its 3035 bytes hash to `576fd4830fa3c143940e7f6accf0cb64bc9e64637a0038ba51c27aa41aed564c`. The `insert_section_in_place` span starts at `crates/rdocx/src/document.rs:24750` and ends before `remove_section_in_place`. Its 3274 bytes hash to `00d0f4d7839da412602a2656f8e097c7e889c0bf7c9dfa7e97cbe996e848c9b7`. Both exact spans were compared with the live worker immediately before report creation and match. Full-file document hash is `182c0717260452c66334dfd9e9da402343b9f1d64039b1140c53d6bb648d6cbd` for the immutable input only.

This report excludes subsequent units, validation, rich-story and related-owner changes, full F201 implementation, HLD completion and final installed GREEN qualification. It grants no whole-current source or runtime claim. External review staging avoids concurrent writes to the implementing worker. The worker may copy this exact report into its review directory.

## Correctness

Zero findings. Insertion obtains the preceding resolved geometry, using the first section for index zero, and supplies the existing layout defaults only for absent fields. It copies dimensions, orientation, margins, gutter and header/footer distances into otherwise empty section properties. It retains the former final section when appending and preserves existing boundaries when inserting. Public insertion stages before reopening and publishing at `crates/rdocx/src/document.rs:24727`.

## Contract

Zero findings. The slice implements the approved geometry-only inheritance and even-story atomicity. New sections omit inherited header/footer references, columns, title-page state and page numbering. Even-story creation stages settings and story ownership together, including enabling a disabled existing direct even story. Invalid ordinal checks precede publication at `crates/rdocx/src/document.rs:24311` and `crates/rdocx/src/document.rs:24750`.

## Panics and failure handling

Zero findings. Fallible package preparation and story resolution occur before the single publication. The settings candidate uses checked relationship and part allocation at `crates/rdocx/src/document.rs:26650`. Index subtraction is saturating. The post-resolution identity assertion does not replace fallible ownership checks.

## OOXML and preservation

Zero findings. Insertion creates only the declared geometry fields and does not clone unrelated section policy. Even creation uses the existing checked settings mutation and story installation path, retaining untouched package members and the existing schema-aware settings writer. The inspected change does not introduce global XML normalization or broad foreign-namespace admission.

## Tests and evidence

Zero findings within this slice. Existing regression controls at `crates/rdocx/tests/regression_test.rs:399148` and `crates/rdocx/tests/regression_test.rs:399207` exercise first, middle and appended geometry inheritance, excluded policies, invalid-index source equality, saved/reopened geometry, invalid even-story refusal, setting publication and re-enabling an existing even footer.

Actual native evidence: `.claude/scratch/F-X201-start/actual-geometry-even-2/receipt.json`, SHA-256 `241b6d6be269798f7983dd678b558100342ff63160c927aa927aa689bfe186fc`, records native all-target check exit zero and the two named regression tests with 2 passed, 0 failed, 0 ignored. Check log SHA-256 `ce799aab585fc78294b867541f2c459dc62442441ac1beeabba8b52df5054227`. Test log SHA-256 `e6d6c3f3950d4cf4dcf7178da943b7e54c7330ca90bcc6a5d7a0b25dc9c34772`. Their historical source snapshot is retained as historical evidence, not relabelled as the later complete worker. No new tests or product commands ran during this review. Native Office paint and final fresh-wheel installed acceptance remain outside this report.

## Structure

Zero findings. The two operations reuse existing staging, checked package allocation, section patching and reopen paths. They add no module, dependency, trait or forwarding abstraction.
