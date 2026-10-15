# F-X221, final working microscope, pass 1

**Reviewed**: Working diff against claim base4b92266b in work/f-x221-codex,58 files,590 insertions and260 deletions. Product and metadata inputs were frozen during this separate read-only pass. Diff SHA256 `188502e472e9bb841346a2d907d8e36d1ceb0e1cd0a931ab9ed3a930df3aec9f`.
**Verdict**: Zero defects, zero smells and zero nitpicks in the reviewed source change. Remaining installed/scoped and final integrated execution gates are explicitly outstanding, not inferred from this review.

## Contract and correctness

The approved F-X221 plan and its HLD10 packaging, HLD14 story and HLD15 release contract retain exact seven-package Word0.16.1 and fifteen-package shared/PowerPoint0.14.1 families. Root independently parsed every selected manifest and workspace pin. All changed Rust carriers and Python project manifests are exact version-only updates. Workspace TOML comparison preserves all external pins, including cfb0.14.0. Cargo.lock differs only in26 local package version fields, with every external record and dependency edge unchanged. Both bindings, both WASM carriers and oxml-py-support remain unpublished on crates.io. The independent rpptx-wasm version remains0.12.1.

The manual CLI family matrix at `.github/workflows/wheels.yml:241` emits both families for workflow_dispatch and exactly the tagged family for push. Manual versions come from the selected manifests and pass the existing unified family preflight. Names distinguish each family and target. The aggregate consumes the matching12 manual or6 tagged archives. Existing registry publication, release creation and CLI attestation remain push-only. No manual event gains publication authority.

Ordered member counts and uniqueness precede ZIP and TAR dictionary construction at `.github/workflows/wheels.yml:434` and line443. Binary, README, licence, regular-file and executable checks remain. The existing-entrypoint regression at `scripts/test_sprint_workflow.py:30` executes the real consumer against both clean event sets and conflicting earlier binary/README/licence entries for both formats. It does not replace consumer assertions with a second simplified validator.

## Tests and artifact evidence

The release regression at `scripts/test_sprint_workflow.py:126` genuinely fails on the old0.16.0 workspace, as retained in old-version-Base.log, then passes on the prepared source in prepared-version-successor.log. It checks the exact selected versions and lock records, family note links and unpublished boundaries. Historical S90 notes retain their original tags while current-family assertions follow the new source. The original mistyped-method invocation and missing-import setup failures remain failed, with the actual corrected CLI control success separately retained in cli-controls-successor-2.log.

Both actual22-package verified publication dry runs completed exit0, including the final metadata successor. Root physically authenticated all22 current archives and333 member records from /private/tmp/S91-F-X221-implementation-1/actual-final22-archive-member-proof.json, SHA256 `daeda6ba8908d93c24d299b3a4d96953d554fa2f0b92abbc932f87be1e266b81`. Every ordinary payload, original manifest and final README matches current source bytes. Member uniqueness, regular-file identity, archive hashes and10MiB ceilings hold. Recorded member counts and normalized VCS member-byte totals match. Compression differences remain within the existing64-byte bound. No tolerance, archive-verification or default hash baseline is relaxed. No upload occurred.

Fresh wheels, paired Python3.9/3.12 runtime/typing, scoped checks and the final full integrated union retain their own actual execution boundaries. At review time the PPT wheel producer has completed and the Word wheel producer remains live. Neither installation nor hosted rehearsal/publication is claimed by that producer status. Remaining checks must pass before prepare and integration.

## Notes and HLD

The two CHANGELOG sections at line4 and line84 cover the selected families with all47 Word and39 PowerPoint record links and authenticated contributor credits. Their overlap is deliberate. The admitted95-of100 result,24 documentation cases and five no-credit divergences agree with the completed F-X211 evidence. Compatibility explains ColorFormat migration, supported owner lifetime, restricted DrawingML watermark opacity and the approved native Word face limit. Bibliography, navigation, standalone authoring and approved S92 interoperability work are excluded from release claims. Source code and notes do not claim contributor branches merged unchanged.

Exactly the three HLD files listed by the plan are updated. Prepared versus published authority remains distinct, shared publication precedes Word, and prior v0.16.0/rpptx-v0.14.0 tags independently resolve to733ce6f7b2b816c5760692abf5c99d68b29ae897. Hosted exact-SHA build-only artifacts, final sprint review/close, registry checks and publication remain subsequent gates. The separately authorized downstream documentation PR is not represented as merged here.

## Not found

Correctness: no defects. Contract: no unauthorized scope, intake or publication change. Panics: no new untrusted runtime parser or panic path. OOXML: no model, child-order, namespace or preservation behavior changes. Tests: genuine old-version failure and current exact-family/consumer controls exist, with remaining execution gates disclosed. Structure: no new product crate, module, dependency, trait, generic, builder or forwarding layer. Documentation: no new unverifiable publication or universal interoperability claim.

This review writes only this report and ends here. The orchestration phase may resume worker checks and delivery preparation.
