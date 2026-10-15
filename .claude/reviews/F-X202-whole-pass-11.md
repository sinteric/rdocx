# F-X202 whole working implementation, pass 11

**Reviewer:** fonts_resume, independent read-only review.
**Reviewed:** Complete immutable r11 implementation against claim Base30775a483ae73c06e448dd594b002dbb4ea2881d. Twenty-three complete product, binding, test and HLD carriers. Owned diff ab1b7f4441e8cb6650dbc373becc2b7173ffd0c991deb61511b49d0fa4d360cb, 5815 lines, 4770 additions and 217 deletions. Receipt aeaeb854e39b6a7b14700dea35a20045f89ffd105c94a70d119b488652f7768f and full source map e03d07eacca427abec1e60a0844e026f82d8ed302789faa9fe1d098c80b88e2d.
**Verdict:** 2 defects, 0 smells, 0 nitpicks. No feature completion approval.

## Defects

### D1: Heading-link destinations exclude approved physical owners

`/private/tmp/rdocx-f-x202-codex/.claude/scratch/F-X202-start/whole-working-capsule-r11/source/crates/rdocx/src/document.rs:19693`
`/private/tmp/rdocx-f-x202-codex/.claude/scratch/F-X202-start/whole-working-capsule-r11/source/crates/rdocx/src/document.rs:19698`

add_heading_hyperlink_at refuses every destination outside the main Body story and then requires a direct body child. A genuine Heading1 paragraph in a table cell, text box or supported related story is rejected before heading qualification, even though its exact owner is supported by the existing paragraph/story providers. The approved Bookmarks and internal links contract requires a supported native heading paragraph, with each newly authored range contained in one exact supported owner. It does not authorize a direct-body-only destination restriction. Related source support does not close this destination gap. The worker confirms no later amendment narrows that contract.

Use the checked physical owner and exact range machinery for supported destinations, preserving atomic bookmark/link staging, suitable complete range reuse and explicit unsupported-boundary refusals. Add deciding nested and related destination controls, including stale/foreign/late-failure rollback. The proposed HLD10 wording that promises only a main-document heading is an unapproved narrowing and must follow the repaired implementation rather than redefine acceptance.

### D2: New external hyperlink tooltip publishes invalid XML characters

`/private/tmp/rdocx-f-x202-codex/.claude/scratch/F-X202-start/whole-working-capsule-r11/source/crates/rdocx-py/src/paragraph.rs:529`
`/private/tmp/rdocx-f-x202-codex/.claude/scratch/F-X202-start/whole-working-capsule-r11/source/crates/rdocx-py/src/paragraph.rs:540`

The new public tooltip operand on Paragraph.add_hyperlink is forwarded directly to add_hyperlink_with_tooltip in the external URL branch. That branch edits the candidate and commits the revision/live document without validating tooltip XML characters or preparing/reopening the complete staged source. For an ordinary body paragraph, add_hyperlink("link", "https://example.invalid", tooltip="\u0000") inserts an invalid attribute value into the live model and publishes structural invalidation instead of refusing atomically. The internal link branch explicitly checks tooltip characters through append_internal_hyperlink_staged, so the two public branches differ for this new operand. No upstream external-tooltip preflight exists in the frozen slice, as confirmed by the worker.

Validate the newly supplied tooltip before relationship allocation, owner mutation or revision preparation. Add a genuine external-tooltip invalid-character control that retains exact package bytes, relationships and existing handles, alongside a valid escaped tooltip round trip. This finding concerns the new tooltip operand and does not relabel the inherited external hyperlink provider as a new implementation.

## Smells

0.

## Nitpicks

0.

## Authentication

All fifty-two declared capsule artifacts authenticate by size and SHA-256. All twenty-three base carriers independently equal git show at the exact claim Base, and every current carrier equals its declared full-map record. The approved plan, complete manifest, SCOPE, evidence pointers and four HLD candidates are included. No moving worker source is substituted for immutable copies. No source, index, runtime, Cargo, CP, Office, baseline or tracker mutation was performed.

## Whole implementation audit

Tables use existing row ranges and topology validation, logical reference-column widths, omissions and vMerge consistency. Empty append separates formatting from content and refuses opaque formatting cloning. Split discovers the full validated region, preserves origin content once and refuses ambiguous continuation content. Local candidates publish only after checked validation, native facade staging and reopen. Column deletion reconciles cut bookmarks/comments. Python related and nested routes use full physical locations, captured revisions and fresh returned handles. Table style resolution checks real type and ambiguity, look setters preserve default banding, and unmerged split remains a no-op. The cell formatting model retains unmodelled tracked tcPr content in extra_xml, which the new-owner clone guard rejects, so no separate copied-revision finding is established.

Physical Picture snapshots distinguish source location and occurrence from relationship identity. Native equality includes private source bytes, metadata, owned media and current owner correspondence. Python origin/revision checks precede writes. One-occurrence resize transports numeric edits back onto physical source, preserving bindings, crop, anchors and unrelated media/owners. Legacy native relationship-wide resize and renamed Python compatibility operation remain distinct. Configured imports reuse checked package ownership in body, cells and related stories. Decorative extension parsing checks qualified owners, extension URI and decoded values, while raw imported source is retained. Frozen snapshots and PictureOptions runtime registration, exports and stubs follow the reviewed factory/private-constructor boundary.

Internal links validate grammar and complete unique targets, author w:anchor without external relationships and stage heading bookmark/source link together. Source owner refresh and Python stale/foreign checks remain explicit. D1 and D2 are the remaining contract and atomicity gaps identified above.

Paired text-box replacement retains qualified physical AlternateContent discovery, signatures, sibling uniqueness, structural paths and independent physical occurrence counts. Count mismatch and zero-match precede correspondence admission. Source edits apply in descending range order after each counterpart succeeds, retaining separate formatting, fields, comments and opaque bytes. Unselected owners, asymmetric cardinality/text, identical sibling ambiguity, foreign MC and unsupported nested correspondence remain explicit refusal boundaries. The corrected unequal-run fixture now proves actual setup, typed bold state and distinct run counts. The prior global replacement contract remains separate.

All four HLD impact carriers retain unrelated accepted sections and document topology, occurrence identity, compatibility migration, typed publication and paired scope. D1's heading narrowing is covered by the same defect rather than treated as an approved boundary. No new product file, module, crate, trait, generic extension point or dependency is added. Explicit public picture operands justify the localized argument-count allowance.

## Prior findings

Pass1 reference-column semantics are resolved by reviewed pass2. Pass3 paired deciding-fixture precondition is resolved by pass4. Pass5 metadata entity decoding is resolved by pass6. Pass7 related-source, bookmark grammar and private Picture typing findings are resolved within pass8's stated scope. Picture typing migration pass9 and installed contract/fixture pass10 remain valid bounded authority. Their clean dispositions do not cover the new whole-review heading destination and external-tooltip findings. No confirmation runtime was repeated.

## Evidence and outstanding gates

Referenced installed logs authenticate. Original installed attempt1 reports 34 PASS and 9 FAIL, with strict mypy0 and stubtest30 errors retained. Attempt2 on e03d07ea reports strict mypy0 over seven source files and stubtest0 over six modules. Its wrapper status remains failed because collection/protocol bookkeeping failed, so it is not reported as an all-green attempt. Attempt3 reports exactly nine corrected public successors PASS, source guards and cleanup safe. The wheel proof reports native SO and all other members exact, with only stub, RECORD and volatile SBOM fields changed. Thus the 43-node qualification is an honest composite of retained34 plus corrected9, not a fresh whole43 pass in this review. Paired package provenance and unchanged PPT checks remain their recorded scope.

The regression gate was active when r11 was sealed. This review gives no completion credit for it, native Word/LibreOffice reopen or paint, pinned python-docx comparisons, hash49, WASM/no-default, changed-crate suites, full scoped Clippy, affected verified archives, metadata policy or final handoff. Those declared gates remain due until actual source-qualified terminal evidence is supplied. Original product, fixture, compiler and wrapper failures remain retained. Intentional API and behavioral changes still require workflow-labelled history and independently qualified deltas, with the exclusive baseline untouched.

## Aspect results

Correctness: D2. Contract: D1 and D2. Panics: 0 additional findings. OOXML preservation: D2's invalid-character admission, 0 additional preservation findings. Tests: deciding destination and external-tooltip negatives are required by D1 and D2, 0 separate manufactured findings. Structure: 0 findings. Total: 2 defects, 0 smells, 0 nitpicks.

Review ends at this report. Remediation is a distinct phase.
