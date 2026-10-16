# F-X240, all, pass 1

**Reviewed**: Working diff in /private/tmp/rdocx-s94-x240 against b55f295276f0863305e2fb82ec241774c9cc53d4. 60 files, 2,629 insertions and 183 deletions. Approved signing design, its cited HLD contracts, progress notes and the explicitly approved ZIP9 compatibility revision are included.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Not found

- **Correctness**: No actionable finding. Both Python signers stage native candidates and allocate reports before owner publication. Word uses its distinct Signing publication policy with physical source and ancestor certificates, exact run paths, unique paragraph sources for hyperlinks and complete story/item equality before retaining snapshot authority. Verification observes serialized current Word state without publication. PowerPoint reuses native staged-current verification and its existing prepared owner publication. See crates/rdocx-py/src/document.rs:4346, :5428, :5447, :5558, :6584 and :6603, and crates/rpptx-py/src/presentation.rs:906, :934, :3193 and :3429.
- **Contract**: No actionable finding. The four actual consumers select signing explicitly, including CLI dependency feature selection under no-default-features. Shared mappings retain all eleven issue variants and payloads. Private stub descriptors describe ordinary owned dictionaries without promising runtime classes. CLI verification requires nonempty, cryptographically valid and complete reports, with invalid preceding partial. See crates/oxml-cli-support/src/lib.rs:439, crates/oxml-py-support/src/lib.rs:662, crates/rdocx-cli/src/commands.rs:3660, crates/rpptx-cli/src/commands.rs:2126, crates/rdocx-py/python/rdocx/_rdocx.pyi:13 and crates/rpptx-py/python/rpptx/_rpptx.pyi:38.
- **Panics**: No actionable finding in added production paths. Material reads and normalization are bounded at 1 MiB, PEM normalization feeds existing strict DER/key matching, and CLI output failures propagate through explicit exit handling. New unwraps and fixture header indexing are in controlled tests. See crates/oxml-opc/src/signature.rs:156, crates/rdocx-cli/src/commands.rs:3619 and crates/rpptx-cli/src/commands.rs:2085.
- **OOXML**: No actionable finding. Signature construction continues through the existing graph validator, schema-ordered creator and complete-coverage verifier. Wrapper publication does not introduce an XML parser or reorder document children. ZIP9 name decoding reaches existing normalization and duplicate checks. ODT retains its raw UTF8 precheck and ODP retains contextual admission and first-stored-mimetype checks. See crates/oxml-opc/src/signature.rs:196, crates/oxml-opc/src/package.rs:159, crates/rdocx/src/odt.rs:2673 and crates/rpptx/src/odp.rs:425.
- **Tests**: No actionable source-level finding. Added controls cover owned reports, absent signer and multiple reports, genuine partial coverage, typed unsaved tampering, strict material refusal, byte and held-owner preservation, form/sign interactions and closed-reader stdout failures. ZIP controls distinguish flag-independent UTF8/CP437 identity, decoded aliases, ODT raw-name refusal and ODP mimetype admission. Workflow policy checks pin actual selected CLI program execution and reject four weakened routes. See crates/rdocx-py/tests/test_core.py:7636, crates/rpptx-py/tests/test_documented_examples.py:10924, crates/rdocx-cli/tests/integration.rs:6927, crates/rpptx-cli/tests/integration.rs:3647, crates/oxml-opc/src/package.rs:1212, crates/rdocx/src/odt.rs:4890, crates/rpptx/tests/integration.rs:1965 and scripts/test_sprint_workflow.py:3459. This assessment does not substitute for pending execution.
- **Structure**: No actionable finding. The concrete report helpers have two actual consumers apiece. The published CLI-support feature is optional and default-off, while both CLI declarations explicitly select it. No new product file, module, trait, generic, codec, error wrapper or dependency alias was added. The historical registry test authenticates the original published archive and unchanged published lock, follows normal registry dependency edges and uses locked metadata/tree without changing yanked policy. See crates/oxml-cli-support/Cargo.toml:15, crates/oxml-py-support/Cargo.toml:19 and scripts/test_sprint_workflow.py:4984.

## Source and artifact authentication

The reviewer independently matched all 3,523 tracked inputs to /private/tmp/F-X240-frozen-successor-9/source.json. Its SHA256 is 6ce325a19fdf7a8f5f533f74f45772583ed0a40869dfe7e3c70c205420cc5247. The actual binary working diff and frozen diff.patch both hash to bf4cd5e3cb0fdf73379891e09c98cee9c9b221b2de8e464a04ecd4c3330b1384. The same checks passed immediately before writing this record.

The reviewer independently read all22 retained round3 archives from /private/tmp/F-X240-zip9-archives-1. Their hashes and measured tuples match receipt.json, SHA256 be30b2fe0a558f1dd24b0344dcba4639a16d32efa0a9a485eba591ffcb5628ef. Normalized member bytes and member counts match the current catalog exactly. Compressed observations differ from recorded values by -2 to +5 bytes, within the unchanged64-byte allowance. All archives are below10MiB. Existing local source members and READMEs present in those archives match current source bytes. This is genuine source-archive measurement evidence, not verified compilation or a publication pass. The catalog retains the original rule at scripts/readme_doctests.py:1337.

## Qualification boundaries

Fresh ZIP9 native scoped execution, ordinary installed wheels and full Python suites, paired old8/new9 raw-package observations, current release CLI acceptance, verified publication and remaining local riders are still running or pending in /private/tmp/F-X240-review-freeze-9.md. Earlier ZIP8 runtime totals are predecessor evidence. This review does not relabel them as current ZIP9 execution and does not assert a completed scoped gate.

The retained old8 native outputs already have ordinary made-by version20. The earlier source-only45-to20 prediction was refuted, as now stated in docs/hld/04-opc-and-packaging.md:2045. Actual paired new9 output comparison remains pending. No raw-byte neutrality or intentional hash delta is inferred, and no baseline was changed by this review.

Hosted wheel, selected CLI including musl, sdist and CLI-assets acceptance remains pending exact reviewed remote source. The source-availability checkpoint and ordinary completion conditions remain applicable. No dispatch, push, release, certificate-chain trust or integrated acceptance is established here. See .claude/WORKFLOW.md:41 and docs/hld/12-testing-strategy.md:997.

Private predecessor environment metadata required redaction. Earlier overwritten verify-only executable bytes are not retrospectively retained. The candid limitations in /private/tmp/F-X240-review-freeze-9.md and /private/tmp/F-X240-evidence-index-1.md remain in force, without exposing credential details. No complete preservation claim is made.

The microscope exit condition of zero defects and zero smells is met for this frozen diff. Completion remains conditional on actual required local and external gates. The reviewer ran no build, test, Cargo, shared-cache or UI operation and changed only this review record. Tracked write ownership is returned to the orchestrator.
