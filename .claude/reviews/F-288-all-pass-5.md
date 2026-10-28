# F-288, all, pass 5

**Reviewed**: Codex independent frozen working-tree review at Base 7c5614936133f608d05667e0f1523e51c2fbf607. 18 files, 3337 insertions and 126 deletions. Diff SHA-256 53974bbdfa4a192ecbd43ccfb5ae3c27ac81560c7e179deeb557db6e3020f5ca.
**Verdict**: 2 defects, 1 smell, 0 nitpicks.

## Defects

### D4, factory negative-domain checks narrow before deciding sign

`crates/rdocx-py/src/document.rs:7255`
`crates/rdocx-py/src/document.rs:7296`
`crates/rdocx-py/src/document.rs:7350`

The approved Python error contract distinguishes negative primitives, ValueError, from nonnegative extraction overflow, OverflowError. Each new numeric helper first extracts i128. Negative Python integers below i128::MIN fail that intermediate extraction with OverflowError before the sign check.

Codex directed a read-only mechanical Grok probe of the frozen installed public module. CPython 3.12.13 imported the worker extension with SHA-256 e7a796062cfaa8d97afb363cc31c9013914c99317860d90be9096729f624defe. checkbox(size_half_points=-(2**200)), text_input(max_length=-(2**200)), and drop_down_list(["one"], selected_index=-(2**200)) each actually raised OverflowError. The positive 2**200 controls raised OverflowError, and bool controls raised TypeError as intended. The transcript is /private/tmp/s95.4-f288-review5-numeric-probe.txt. No implementation or test source was changed by that probe.

Determine negativity without a narrower intermediate integer bound, then perform the declared unsigned extraction. Keep bool and wrong-type refusal, positive overflow and native semantic error mapping unchanged. Add exact exception assertions to the existing Python tests. This reopens the numeric boundary portion of D4, not its resolved signatures or checked-payload design.

### D5, text-box inventory and publication skip inline control owners

`crates/rdocx/src/field.rs:1268`
`crates/rdocx/src/field.rs:1373`
`crates/rdocx/src/field.rs:1390`
`crates/rdocx/src/field.rs:1521`
`crates/rdocx/src/field.rs:17077`

Inherited namespace handling now respects source bindings, and the supported drawing-local foreign-shadow control passes. However, the new text-box discovery and publication walks only inspect paragraph.runs. The existing body paragraph collector pushes a paragraph without recursively exposing its inline controls. The separate text-box control setter explicitly ignores SdtContent::Run. Thus a drawing inside a valid inline rich control, with a legacy field inside that drawing's text box, is skipped by inventory and fill. The text-box group collector has the same omission. This is a currently attachable internal story owner, covered by the approved every-currently-attachable-story contract.

Traverse the accepted inline control owners with the same physical order and namespace policy used by ordinary field inventory and existing story publication. Include the drawing's real text-box paragraph, retaining old ordinal prefixes and avoiding duplicate discovery. Do not flatten the inline control or drop its metadata, raw drawing bytes or unrelated content. Add a valid positive control with the text box inside the inline control, not merely an inline field inside a text box. Prove public inventory, exact typed fill, default preservation, unrelated-owner preservation and reopen in main and related owners where currently supported. Challenge enclosing supported revision or hyperlink owners using the existing accepted-owner policy rather than inventing another selection rule.

This remaining wrapper input is source-traced by Codex, not executed by the reviewer. Retain D5 because it is the same physical owner coverage invariant.

## Smells

### S2, the duplicate traversal became a forwarding wrapper

`crates/rdocx/src/field.rs:1532`

textbox_groups_in_paragraph now only allocates a vector, forwards to append_nested_textbox_groups and returns it. The duplicate traversal was removed, but the repository also forbids wrappers that only forward. Remove this single-consumer wrapper and call the actual existing traversal at its consumer with the local output vector.

## Nitpicks

None.

## Resolved and retained observations

The inherited namespace portion of D5 is resolved in the inspected scope stack and whole-related-source path. No lexical w fallback remains there. The receipt separately records the original root-shadow fixture's existing modified-document serialization refusal, and the final supported drawing-local-shadow fixture's passing result. They are not the same-input historical RED and GREEN pair.
D1, D2, D3, D7 and pass 1 S1 remain resolved in the inspected paths. D4's callable signatures, checked native payload, before-return validation and attachment clones remain resolved.
The frozen source has actual 12-case native, 24-case Python, local mypy, fmt and scoped all-features clippy receipts. The new probe reports three failed negative-domain criteria, not acceptance success.

## Not found

No additional panic, dependency direction, new trait, generic, module, child-order or atomic Python-return defect was found. Final Word, hash harness, full installed suite and union gates remain pending. DOCX-055 and the Q4 provider checkpoint remain incomplete.

## Review boundary

No source was changed by this review. Grok may transcribe this exact Codex report only. End the review pass when written. Remediation is a separate orchestration phase.
