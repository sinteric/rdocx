# F-X205, fill, pass 4

**Reviewed**: Immutable fill-production11-capsule, six complete existing carriers. Its context-inclusive working diff has 1,290 added and 51 removed lines. This pass audits the new staged fill and picture lifecycle, gradient stop ownership, pattern/gradient controls, solid alpha and associated enum/stub declarations. Earlier font/bullet/table changes retain their separate review authority. Reviewer: fonts_resume, independent of this implementation.

**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Evidence and scope

Capsule: `/private/tmp/rdocx-f-x205-codex/.claude/scratch/F-X205-start-1/fill-production11-capsule`.

Manifest SHA-256: `f20b7900f31bc3880861df8e74160143c52793b5ea477245d298be98e4f45490`. Source map SHA-256: `5e9a470b422a88c41098b92dd6c55297d98c5f1831d85ba123d156cb321cf683`. Complete diff SHA-256: `87fc7b55ca53d769517b40b07b002d5e253d66093df03ba2ec65086d29b58d43`. All six complete candidate sizes and hashes match the manifest. Citations below address those immutable copies, not the subsequently changing worker.

Actual fill receipt SHA-256: `899918070d4929b72d3f8a2898a9219df57788156a1d65ea1a6c71abfe2ba69a`. Both raw logs match their recorded hashes. The all-target native/binding check exits 0. The exact native `write_tests::fx205_picture_fill_is_atomic_and_retains_shared_image_owners` exits 0 with 1 passed, 0 failed and 0 ignored. Both owned groups have natural cleanup and ESRCH, and the receipt records unchanged source after execution. The aggregate two-row pass count is not two test cases.

## Defects

### D1, transparency getter and stub omit the approved absent value

`crates/rpptx-py/src/dml.rs:632`
`crates/rpptx-py/src/dml.rs:635`
`crates/rpptx-py/python/rpptx/_rpptx.pyi:586`

The approved final contract at `.claude/plans/F-X205-design.md:477` exposes transparency as `float|None`, with a getter that does not create a fill and a setter requiring an existing solid fill. The frozen getter instead raises `TypeError` for every absent or non-solid fill and returns only `f64`. The stub likewise advertises only `float`. Reading transparency on a fresh fill therefore cannot return the approved absent value. Preserve checked owner validation and existing non-solid setter refusal while correcting the getter and its type declaration. Add source-preserving absent/non-solid getter controls alongside the solid alpha controls.

### D2, replacing one fill deletes unrelated retained image relationships

`crates/rpptx/src/lib.rs:4620`
`crates/rpptx/src/lib.rs:4668`
`crates/rpptx/src/lib.rs:4716`
`crates/rpptx/src/lib.rs:4730`

Both replacement paths invoke a cleanup scan of every image relationship on the slide. Its deletion predicate requires only an internal IMAGE relationship absent from the newly serialized slide's references. It does not restrict deletion to relationships released by the selected old fill. An imported slide containing an unrelated, pre-existing unreferenced image relationship therefore loses that relationship when a textbox is assigned a solid fill. Its media part can also be deleted by the subsequent unreachable-part pruning. This violates retained unrelated source ownership and the approved selected-owner transaction at `.claude/plans/F-X205-design.md:342`.

Determine the finite image references released by the replaced owner and restrict cleanup to that set, while retaining the complete current-reference scan that protects shared users. The existing native control at `crates/rpptx/src/lib.rs:18798` proves reachable shared image retention and last-user cleanup, but does not include an unrelated imported unreferenced image relationship. Add that deciding case and preserve the relationship, target bytes and unrelated package members across replacement and save/reopen.

## Smells

None.

## Nitpicks

None.

## Not found

Panics: 0 additional findings. OOXML child order and qualified authored blip ownership: 0 additional findings beyond D2 source loss. Structure: 0 findings. Correctness, contract and tests: no additional findings beyond D1 and D2. Gradient tokens use monotonic identities parallel to the stop vector, preserve surviving identities through insertion/deletion and refuse deletion below two stops. Candidate publication remains staged and fallible before replacing the original presentation.

## Limits

This is a bounded source11 review, not whole-feature acceptance. Pattern/path rendering, installed gradient identity cases, fresh-wheel typing and native visible output remain unqualified here. Later core/app/custom properties, media/export APIs and later renderer changes are excluded. The passing shared-image control does not close D2, and no runtime was performed during this review.
