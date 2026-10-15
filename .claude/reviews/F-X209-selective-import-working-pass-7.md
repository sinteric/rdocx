# F-X209, selective import slice, pass 7

**Reviewer**: fonts_resume, independent read-only review.
**Reviewed**: immutable selective-import-source36-capsule, one complete native carrier containing pass6 MC-policy remediation and fresh selected master/layout import, staged-source initialization and mapping controls.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. Prior pass6 D1 is resolved in this slice. Fresh import still has the D1 below.

## Authentication

Receipt SHA `58810f03c29b6727ec8a6921ca1fadabfd5da2c2f6fe6d1be216a2dfb6c165be` authenticates all 20 records. Complete diff SHA `bf97c62369025ea7f8de9443134fe599a4ae94043c6e8730b2e8e88bd58fa57c`. Before SHA `ae99f6aa6bb1f3ba3f5d3053a6af47c40722d9a7dfcb304f721e042a3a9c6f56`, after SHA `0c858250c978704de757ca17269b870456134392ad8b79838831ba2690c2e811`. Source36 map SHA `e669cc620aef34992307ba294040c99efb9b453380edaead048a96586f039e2b`. Review uses the approved F-X209 selective import/preservation contract and canonical microscope/workflow rules. The moving worker's later source is not a deciding input.

## Defects

### D1, selected imported XML references are not checked against its relationship scope

`/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/selective-import-source36-capsule/lib.rs.after:3797`

`import_selected_master` validates each selected part root/content type and walks existing .rels entries to build the permitted master/layout/theme/image/hyperlink closure. It never inventories the qualified r:id/embed/link references in each retained selected XML part and checks them against that part's relationships. At line 3900 it copies the entire retained selected XML, while the relationship rewrite at lines 3904-3935 is derived only from relationships that already exist.

For example, a selected layout can contain an opaque extension subtree with a correctly qualified `r:embed="missing"`, while its .rels file has no matching entry. Root and typed layout parsing admit and retain that opaque subtree. The .rels iteration never visits the missing reference, the full source is copied to the new part and permissive reopen does not add the required reference-inventory preflight. The operation can therefore publish an imported owner with an unresolved retained dependency. The same problem applies to selected master or theme root extensions outside the separately inventoried three scheme subtrees.

The approved plan requires complete selected closure and namespace-qualified references, including opaque retained source, to be validated before publication. Validate every selected part's actual qualified references against its relationship scope before allocating/publishing, or refuse the unsupported selected payload with owning part/reference diagnostic. Preserve same-spelled foreign attributes and raw source bytes. Add a genuine missing opaque-reference late-owner atomic refusal control, alongside a resolving selected-reference positive. The current import control at line 20380 uses the normal template graph and does not decide missing retained references.

The implementing worker confirmed no additional selected-part reference inventory guard exists in the immutable source36 path. This finding does not rely on the excluded image-copy positive branch.

## Resolved prior finding

Pass6 D1 lost ancestor MC processing policy when closing a copied scheme. `selective-import-source36-capsule/lib.rs.after:8239` now tracks qualified inherited policies separately from namespace declarations, copies supported effective policy onto the selected scheme and refuses unsupported nested merges or rebound/undeclared policy QName prefixes before publication. The root Ignorable/opaque descendant and rebound-prefix control is deciding for the original finding. Source33 MC production is unchanged in the inspected source36 copy. No broader universal MC compatibility claim is made.

## Other inspected behavior

At `selective-import-source36-capsule/lib.rs.after:3615`, the candidate is initialized with the current staged destination package before raw splices. This retains unsaved slide/presentation edits. The actual deciding import control creates an unsaved slide and direct textbox, maps its existing layout to the imported ordered source layout and compares original direct-slide/unrelated part bytes at line 20427.

Selected import creates fresh master/theme/layout part scopes, retargets internal layout backlinks and master-list relationships to those scopes, allocates fresh list/master IDs with checked exhaustion, and retains part-scoped shape IDs and opaque bytes. Explicit mapping rejects duplicate destination keys and out-of-range owners, retargets only slide layout relationship source, and leaves the direct slide XML unchanged. Empty mapping preserves all current slide layout relationships. Errors operate on a clone and publish only after candidate reopen. Existing destination masters/layouts remain in place.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: D1 only in the bounded selective import path.
- Contract: D1 only, complete retained dependency preflight.
- Panics: 0 findings in this slice. Inspected indexing follows collected inventories or guarded lengths, and allocated mapping keys are established before indexing.
- OOXML: D1 only. Source namespace/MC closure, qualified target/list-value patches and direct-slide preservation otherwise have no findings.
- Tests: no additional findings. Missing opaque-reference coverage is included in D1. The exact original direct-source and package-atomicity assertions remain intact.
- Structure: 0 findings. One existing carrier, no new module, trait, dependency or speculative framework.

## Actual evidence and limits

Source36 fresh import receipt SHA `e053cb2c191af3ae8bfb191a898111f9ca35647a986cd3a7cb1f241f2e16c027` binds exactly one test, 1 PASS, 0 ignored, raw log SHA `0e35d0fd169c48f7dd65fc1fe86d91ad3cbcf9cab7428738ee601cd71028c565`. SourceAfterMatches and ownedLeaseSafeToRelease are true, with natural ESRCH and no signals.

The source33 MC row in receipt SHA `75ddd3c39f28ced833c72795dc2a8cc977e3378f4084aa1c6e825aecd43d0e41` passed exactly one test. That combined attempt remains failed because its fresh-import row exited 101. Predecessor import attempts1 through4 exited101 and remain retained. Check20 completed with exit0. No failed attempt is credited as clean.

No source or runtime changes were made by this reviewer. Positive image transfer, Python/installed ownership, deterministic/native branding paint and whole-feature acceptance remain outside this pass. Review ends at this report.
