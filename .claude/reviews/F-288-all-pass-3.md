# F-288, all, pass 3

**Reviewed**: Codex independent frozen working-tree review at Base 7c5614936133f608d05667e0f1523e51c2fbf607. 18 files, 2765 insertions and 122 deletions. Diff SHA-256 dc523719f8176061e0b08b7f1ad02938a47daf6cd6149204d3e975c838e639e8.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D5, text-box publication still selects content rather than the physical owner

`crates/rdocx/src/field.rs:1484`
`crates/rdocx/src/field.rs:1521`
`crates/rdocx/src/field.rs:938`
`crates/rdocx/src/field.rs:16380`
`crates/rdocx/src/field.rs:16548`

The main text-box positive case now works. Two supported owner cases remain wrong.

First, changed_drawing_field_replacements removes unchanged occurrences before splice_field_sources searches for source bytes. With two byte-identical fields in one drawing, filling the second produces only its changed replacement. The byte search selects the first occurrence. Reopen normally rejects the requested update because the second stayed old, so a valid target cannot be filled. A same-typed-value display repair can also evade that value-only check. Source content is not physical identity. The unrestricted search can additionally match field-shaped bytes in opaque retained XML instead of a typed owner. Use a namespace-qualified, bounded physical source map. Keep unchanged occurrence positions when distinguishing duplicates. Preserve opaque spans and validate the selected owner, not merely the reopened requested value.

Second, the related-story setter still calls patch_legacy_story_field_sources on the outer paragraph. staged_paragraph_field_source_replacements now contributes drawing-descendant field fragments, but find_typed_field_source accepts only the direct paragraph and its narrow inline wrappers. typed_field_ancestors rejects the intervening run, drawing and anchor ancestors of a text-box field. Therefore the listed header text-box field cannot be successfully filled through the new path. Publish through the real selected text-box paragraph boundary while retaining the settings and story relationship owner. Do not broaden the existing direct-field ancestor guard to admit arbitrary opaque or foreign XML.

`crates/rdocx/tests/integration_test.rs:14710`
`crates/rdocx/tests/integration_test.rs:14730`

The current C7 regression lists the related, nested, inline and comment fields but fills only the main box. That does not discriminate either remaining error. Add exact-owner successful fill and reopen controls for related and nested owners, comments and shared-header deduplication, plus identical field sources targeting the second occurrence and a preserved opaque lookalike. Compare all untouched fields and raw opaque bytes. Retain this finding ID because it is the same promised read/write physical-owner invariant.

These remaining inputs were source-traced by Codex, not executed by the reviewer. Capture actual pre-fix failures where possible before repairing.

## Smells

None.

## Nitpicks

None.

## Resolved and checked

D1 is resolved in the inspected getter and parsed-parent regression. The accessor follows effective display and retains raw physical cache behavior.
D2 is resolved in the inspected derived baseline and unfiltered effective-run path. The regression now observes the changed child before refresh and after refresh, with direct parent overrides and explicit clearing.
D3, D4 and S1 remain resolved.
D7 is resolved: the complete Python return object is prepared from the candidate before publication.
D6 improved with positioned bold rendering, nested table attachment, inline and comment discovery, a fresh 10-case native batch, 24 Python checks and local mypy. The remaining related-owner mutation discrimination is included in D5 rather than counted twice.

## Not found

No additional panic, dependency direction, trait, generic, module, default-preservation or canonical child-order defect was found in this pass.

## Pending gate evidence

This is not a scoped verification certificate or a provider prefix. After the next source repair, consume or rerun affected local cases with explicit named membership, including the six plan-named groups. Complete the actual scoped formatter and changed-crate clippy configuration required by verify, including --all-features and the affected layout and CLI crates. Final Word, hash harness, installed full suite, full verification and union riders remain pending. DOCX-055 remains partial with live owner F-288.

## Review boundary

No source was changed by this review. Grok may transcribe this exact Codex report only. End the review pass when it is written. Remediation is a separate orchestration phase.
