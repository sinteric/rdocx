# F-X206 HLD union, pass 57

**Scope:** External eight-carrier HLD proposal against the actual R15 worker preimages and prospective reviewed R16 plus cfg source union. Approved F-X206 design, current progress, canonical workflow and microscope, and prior source and adoption reviews were read. This is a documentation proposal review, not source adoption, runtime acceptance or completion.

**Verdict:** 1 defect, 0 smells, 0 nitpicks.

## Authentication

Packet `/private/tmp/S91-F-X206-HLD-union-prepared` receipt SHA256 `957d896bd1b002b600463048ab58654ebdecf5c8f825ea497e1f67c40591c91d` and complete.diff SHA256 `b68624808785688839eeac7155e8f5a9c03ddf468e0c4e1f63aee7c7d58c5419` match. All eight preimages, candidates, sizes and modes match the manifest. Their physical worker HLD preimages match actual a9 source. Prospective map `315aa583a51f1f920eabf5bb94b0bb8c4fac61ef9d8623eb392afb532f78c6a7` retains all2714 members and changes exactly HLD02/03/05/06/07/08/10/14 relative to the declared prospective code map `ce60a8f830aa1a24178ac07d9dba223e13e9ee2e1a3313fa1a5192cc67988c87`.

The manifest explicitly distinguishes worker preimages from canonical HLD08/10/14. Read-only apply checking against the current canonical prefix succeeds. These are localized integration hunks, not authority to replace the canonical files with stale complete worker candidates. Current canonical F195 caller-font, effective-axis, trace and independent Q/C clauses, F200 formatting/list intent and F213 navigation intent must remain in that localized integration. The patch does not remove their clauses. No adoption was performed.

## Defects

### D1, obsolete global invalidation and replacement mechanics remain in the delivered binding contract

Candidate `/private/tmp/S91-F-X206-HLD-union-prepared/candidate/docs/hld/10-bindings-spec.md:677` to :680 still says adding a group member stales the group and its collection and requires re-fetching them. This contradicts the new owner-survival contract at the same file:61 and :524 and the approved physical-owner survival contract. Prospective `crates/rpptx-py/src/shape.rs:1382` retains unchanged owner proofs during collection addition and allocates the returned shape against prepared successor authority before publication at :1392. Existing group and collection owners are preserved.

The same incomplete reconciliation appears at candidate HLD10:862 to :867. It claims slide/frame replacement invalidates every handle including the receiver, and that neither route clones the presentation, with an in-place frame path without expect. Actual retained `crates/rpptx-py/src/slide.rs:275` clones the presentation, then prepares affected text-owner retirement at :284. Prospective `crates/rpptx-py/src/text.rs:356` also clones the presentation and uses text_changed_owners at :366 before publication. Retained `crates/rpptx-py/src/presentation.rs:1378` marks the selected owner's text scope while preserving other owner proofs. The owning slide/frame and unrelated physical owners are not globally retired by these operations.

Reconcile these existing paragraphs to the same exact staged publication and affected-descendant semantics documented by the new sections. Keep actual count/mismatch/no-match behavior, public operation labels and conservative Word invalidation distinct. Appending a correct general section leaves callers with contradictory operation-specific requirements.

## Smells

0 findings.

## Nitpicks

0 findings.

## Six aspects

- Correctness: D1. No other finding in the localized documentation changes.
- Contract: D1. The eight-file list matches the approved HLD impact. Geometry remains limited to representable affine decomposition, contiguous selections and atomic unsupported cases. Small-caps80percent fallback is explicit product policy rather than claimed native equivalence. Direct tcPr anchoring remains separate from bodyPr. No new metric, archive or future runtime proof is asserted.
- Panics: 0 findings. This patch changes no executable path, error handling or arithmetic. It does not qualify unexecuted source or runtime behavior.
- OOXML: 0 additional findings. Qualified namespace ownership, foreign lookalikes, local namespace closure, preserved raw sibling order, legal picture geometry and unused-owned relationship pruning agree with the prospective source contract. Explicit refusal of nonrepresentable metadata remains documented. No broad normalization or foreign-owner promotion is proposed.
- Tests: 0 additional findings. No test or oracle changes. Runtime, native PowerPoint and LibreOffice, no-repair, installed, recovery, explicit caps oracles, hash attribution and scoped verification remain required independently. The document contains no execution counts or acceptance result invented from the candidate.
- Structure: 0 findings. Exactly eight existing approved HLD carriers. No new product file, module, type, dependency, provider or forwarding abstraction. Shared causal support remains one actual owner inventory with prepared publication and first retirement causes.

## Limits and handback

The accepted F195 source controls and current F200/F213 intent are retained boundaries, not fresh F206 proof. This review does not qualify Issue309/323 native geometry, distribution, picture crop, recovery, caps/highlight or lifetime runtime acceptance. Additional Base prerequisites and later GREEN and full feature gates are unaffected.

Only this review was written. The review pass ends and returns its lease to ROOT for a distinct remediation phase.
