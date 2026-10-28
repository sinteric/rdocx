# F-288, all, pass 6

**Reviewed**: Codex independent frozen working-tree review at Base 7c5614936133f608d05667e0f1523e51c2fbf607. 18 files, 3564 insertions and 126 deletions. Diff SHA-256 d709b693a532834a285b67895ef0abdab8dfed28fa89fb9a8fb800123ac2dfe5.
**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Defects

### D5, raw span selection still differs from the accepted owner inventory

`crates/rdocx/src/field.rs:1552`
`crates/rdocx/src/field.rs:1606`
`crates/rdocx/src/field.rs:1634`
`crates/rdocx/src/field.rs:16570`

The typed discovery and grouping now correctly traverse accepted inline controls, hyperlinks and insertion owners. The raw word_txbx_paragraph_spans scanner still counts every qualified Word paragraph below any qualified txbxContent, regardless of the enclosing accepted-view owner. It therefore counts a deleted text box that accepted_run_paths correctly excludes.

Codex directed a read-only mechanical Grok probe on the frozen extension SHA-256 991319c975a577f0a89a24e41140384e9540a9d47400ac93eba2e512ca70c5fc. The fixture was built only in memory from a public blank document and its header. One header paragraph contained an ordinary active drawing and a second drawing inside w:del. Both used the valid native test shape. Public inventory contained active only, part /word/header1.xml, ordinal 0, current old. Filling active with new actually raised RdocxError, text box paragraph boundaries do not match the typed story owner. Bytes stayed unchanged, fresh and reopened current stayed old, and the deletion remained. Transcript: /private/tmp/s95.4-f288-review6-mixed-owner-probe.txt.

Use the same accepted physical-owner selection for the raw span map and typed inventory before associating addresses. Namespace identity alone is insufficient. Preserve the deleted and opaque XML rather than removing it to align counts. Do not repair this only by comparing counts or silently skipping a mismatching retained copy. Audit the same selection boundary for move-from, opaque wrappers and supported AlternateContent choices or fallbacks so ignored physical copies cannot steal positions. Reuse the existing concrete accepted-owner policy and selected drawing identity, rather than introducing another loosely equivalent grammar.

Add a discriminating regression mixing live and deleted text boxes in the SAME related paragraph, with successful exact-target fill and reopen, unchanged deleted bytes, stable ordinal and default, and no partial publication. Retain existing positive inline, revision, namespace, duplicate and opaque controls. This retains D5 because read and write still disagree about the physical owner map.

### D8, document attachment uses value-update publication for a structural insertion

`crates/rdocx-py/src/document.rs:10237`
`crates/rdocx-py/src/document.rs:9000`
`crates/rdocx-py/src/run.rs:454`

The approved Python lifetime contract says structural insertion uses existing conservative publication. Run.add_legacy_form_field correctly requests Conservative. Document.insert_legacy_form_field instead calls publish_form_candidate, which selects FormValues and can preserve old owner certificates.

The frozen public-module probe authenticated the same extension as D5. In two independently created documents, each with held and target body paragraphs, an old held paragraph and run were acquired before inserting a field into target. Document insertion left held.text alive while heldrun.text raised StaleElementError. Run-level insertion made both old handles raise StaleElementError. Both actually inserted the requested field. Transcript: /private/tmp/s95.4-f288-review6-publication-probe.txt. A preliminary StoryItem.runs setup mistake was corrected only in the in-memory probe and is not evidence for the lifetime behavior.

Use the existing Conservative publisher for document structural attachment, retaining the checked position, staged candidate and atomic refusal. Do not change the shared FormValues helper used by actual value setters. Add public binding assertions for both attachment routes, including the unchanged-byte refusal control and re-fetch after successful structural insertion.

## Smells

None. S2 is resolved: the forwarding wrapper was removed.

## Nitpicks

None.

## Resolved and retained observations

D4 is resolved for the inspected exact numeric contract. The new sign helper runs before unsigned extraction, and the existing Python test asserts exact negative, positive-overflow and bool exception classes across all three factories.
The plain inline-control, accepted hyperlink and insertion portions of D5 are resolved. The prior test places the deleted drawing in a separate paragraph, which does not discriminate the mixed-owner refusal demonstrated above.
D1, D2, D3, D7 and pass 1 S1 remain resolved in the inspected paths.
The frozen implementation has actual 13-case native, 24-case Python, local mypy, fmt and five-crate all-features clippy receipts. The two additional probes are failed acceptance observations, not successful qualification.

## Not found

No additional panic, dependency direction, new trait, generic, module, child-order or Python result-allocation-order defect was found. Final Word, hash harness, complete installed suite and union checks remain pending. DOCX-055 and Q4 remain incomplete.

## Review boundary

No source was changed by this review. Grok may transcribe this exact Codex report only. End the review pass when written. Remediation is a separate orchestration phase.
