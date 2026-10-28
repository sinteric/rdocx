# F-288, all, pass 4

**Reviewed**: Codex independent frozen working-tree review at Base 7c5614936133f608d05667e0f1523e51c2fbf607. 18 files, 3101 insertions and 126 deletions. Diff SHA-256 dd50a97b6e8a2d5446b4867859929ab8040fdc7098ed39d1a7d381d2b601da1b.
**Verdict**: 1 defect, 1 smell, 0 nitpicks.

## Defects

### D5, physical text-box spans lose inherited namespace identity

`crates/rdocx/src/field.rs:1466`
`crates/rdocx/src/field.rs:1589`
`crates/rdocx/src/field.rs:16399`
`crates/rdocx-oxml/src/drawing.rs:2127`
`crates/rdocx-oxml/src/drawing.rs:2194`

The new owner-bounded publication correctly retains identical occurrences and rejects the tested opaque CDATA lookalike. The related-story positive control now fills and reopens.

However, word_txbx_paragraph_spans parses isolated retained anchor XML or an outer-paragraph slice without its actual inherited bindings. It guesses unresolved names only when their lexical prefix is w. A document declaring q as the Word namespace at the document or header root can legitimately use q:txbxContent and q:p in the drawing. The existing drawing parser understands that inherited binding. The retained raw anchor deliberately closes only bindings lost with the regenerated drawing wrapper, and keeps ancestor bindings at their original owner. Consequently the new span scan sees unresolved q and produces no paragraphs, while the typed shape has one. A valid supported fill then fails with the cardinality refusal.

Use the actual inherited namespace context for the projection and preserve physical source offsets. The anchor already retains source_namespace_bindings. Related parts can be scanned in their whole qualified source context or use the equivalent existing scope mechanism. Do not add q as another guessed Word prefix and do not treat a foreign rebound w as Word. Preserve the raw producer declarations and bytes instead of injecting new declarations into saved XML for lookup.

Add successful fill and reopen controls using a noncanonical Word alias inherited above the raw fragment, in both main and related text boxes. Include a foreign shadow of canonical w alongside the real Word alias, with opaque same-local-name XML preserved. These are namespace identity controls for the same physical owner invariant, not a new supported insertion context. This remaining failure is source-traced by Codex, not executed by the reviewer.

## Smells

### S2, text-box run traversal is implemented twice

`crates/rdocx/src/field.rs:1515`
`crates/rdocx/src/field.rs:1526`

append_nested_textbox_groups and textbox_groups_in_paragraph contain the same run, content and drawing traversal, differing only in whether they receive or allocate the output vector. Consolidate the actual traversal into one existing concrete helper and use it from both consumers. Do not add another forwarding helper or abstraction.

## Nitpicks

None.

## Resolved and retained observations

The pass 3 duplicate-field and related-story failures have actual pre-fix RED and fixed-source GREEN receipts. The strengthened test fills the nested table field, the shared header's text-box field and the comment, verifies reopen and preserved defaults, and preserves the untouched inline and body values. The duplicate test fills the second identical field and then the first while keeping the opaque lookalike byte-identical.
D1, D2, D3, D4, D7 and pass 1 S1 remain resolved in the inspected paths.
The frozen source has 11 named native cases, 24 focused Python cases, local mypy, fmt and scoped all-features clippy receipts. The next namespace repair invalidates its affected publication observations until rerun.

## Not found

No additional panic, dependency direction, new trait, generic, module, canonical child-order or atomic Python-return defect was found. No Word, hash, full-suite or integrated-union qualification is claimed. DOCX-055 remains partial and Q4 is incomplete.

## Review boundary

No source was changed by this review. Grok may transcribe this exact Codex report only. End the review pass when the report is written. Remediation is a separate phase.
