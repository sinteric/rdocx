# F-X201 live sections, note policy and nested tables, pass 4

Reviewer: fonts_resume, independent of the implementation.

**Reviewed**: bounded immutable live SectionHandle, note-policy, nested-table and absent-story-unlink slice. The capsule contains 14 existing carriers and a complete working diff, but this pass does not re-review their unrelated rich-owner implementation. Page colour, watermarks, later worker changes, native rendered oracles and full feature acceptance are excluded.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication

Input `/private/tmp/S91-F-X201-live-policy-nested-review/receipt.json`, SHA-256 `cff13fac6249a6e47407833d805daa07eafe55e940bfaa7a2784019b4bdfb254`. Independently authenticated all 14 copied carriers, the 3076-entry source map SHA-256 `0c84e1bd1841f335fe66f6dce3c191d03004fd75d662ea91d7bb4a3c841a7364` and working diff SHA-256 `70bac0c84cc700646ba7c233a4eae1fe06da75da8d8bd2cf62088bd8457c8f12`. The receipt binds HEAD `2d2f4b8f133e3a4c1a097154d39119552d8050ec` and retained index identity. This audit uses the immutable copies, not advancing live worker files. All citations below refer to those copied paths relative to the repository.

Deciding binding carriers are document.rs SHA-256 `8a9a1ec074ea8cc50997189847066ee1cf5172a38f89310db11d9109890b1c59`, table.rs `d16ad8f8df3095968d95caa2174c96664d3eb1e7408c7f2baba9e9cb47fd6fed`, and test_core.py `c89f5e64cfb728e1a3bdef81143f31245db77456b118f64f154965bc6791553b`.

## Six aspects

- Correctness: 0 findings. The absent slot is reported linked and setting linkage false creates an explicit story instead of invoking the native inherited-story unlink operation on absence (`crates/rdocx-py/src/document.rs:1254`, `:1265`). Existing inherited and direct cases remain distinct. Geometry setters reuse the checked update operation, with rollback of section properties on a late native setter error (`crates/rdocx-py/src/document.rs:839`, `:4215`).
- Contract: 0 findings. SectionHandle validates document revision and section ordinal before resolving furniture (`crates/rdocx-py/src/document.rs:813`, `:899`). Note policy accepts the actual named native family/format/restart/placement values and delegates positive-start and family-placement validation to native policy (`crates/rdocx-py/src/document.rs:1134`, `:1054`, `:4280`). The new stub declarations match the inspected policy and nested-table bindings (`crates/rdocx-py/python/rdocx/_rdocx.pyi:399`, `:694`, `:1447`).
- Panics and errors: 0 findings. Checked section indexing precedes its internal expect. Invalid policy inputs and zero table dimensions fail before publication. Nested-table correspondence is checked before committing the staged revision (`crates/rdocx-py/src/table.rs:1309`). No additional unsafe indexing or overflow defect was found in this bounded slice.
- OOXML and ownership: 0 findings. Native section/document note policy updates existing parsed property owners, preserving unrelated records and opaque children (`crates/rdocx/src/document.rs:3903`, `:25639`). Nested table selection derives the actual physical cell story rather than a main-body ordinal (`crates/rdocx-py/src/table.rs:1160`, `:1286`). Related table refresh retains kind, part and owner identity, then uses the checked native reader/editor (`crates/rdocx-py/src/table.rs:31`). Candidate publication follows successful native mutation and owner correspondence validation.
- Tests: 0 source defects. The signed-margin correction preserves invalid-type and atomic-byte assertions while checking the native truncation behavior (`crates/rdocx-py/tests/test_core.py:4340`). The absent-footer case exercises explicit creation, owning footer content and save/reopen (`:4358`). Policy controls retain existing note bytes and check rejected start/placement atomicity (`:4424`). Nested body/footer controls exercise actual nested cells, zero-dimension refusal, reopen and terminal cell paragraph schema order (`:4463`). These tests are meaningful source controls, not a claim that corrected installed execution has passed.
- Structure: 0 findings. Existing native and binding carriers contain the work. SectionHandle is a concrete checked owner. No new module, dependency or speculative trait is introduced by this slice.

## Evidence limits

Authenticated note-policy native/binding check receipt SHA-256 `0618cfe759c7ccc8fb877a130b8b826817a880e7ee50664857239768cdce5323` and nested-table check receipt SHA-256 `6617b6291dc1e3a2deca7aff7ab79df0cab121c41a7387e09323f0cf14b874bb` each record a successful check command. Their raw log hashes were independently checked. Both record natural cleanup, leader reaping and ESRCH with no signals. These are compilation evidence, not installed or rendered acceptance.

The retained installed attempt records six passes and two failures. The copied source includes the signed-margin fixture correction and absent-story unlink correction after that attempt. Corrected installed execution and fresh wheel qualification remain due. No runtime or product imports were performed during this review. Existing earlier focused reviews retain their own scope and authority. This report does not qualify the rest of F-X201 or its pending page/watermark paths.

Return control for implementation and actual focused verification.
