# F-289, native interim microscope, pass 2

**Implementer**: Grok
**Actual reviewer**: Codex root
**Reviewed**: Frozen native working diff against c0b58c7fa8e4e919b9bfe5c02932f74827cea36b. Five files, 3878 insertions and 3 deletions, including the acknowledged eight-line plan amendment from canonical 0d8dfb68396e33ddfcaa0d9400c30c006e12d2b7. Diff SHA-256 06865fd72ac672e7dd5b4e2e5f98f43a7d1cf6e96aa47a0f091c2ef2fb3acdbf.
**Verdict**: 4 defects, 0 smells, 0 nitpicks.

This is the independent follow-up to native pass 1. Python and the five HLD updates remain the planned next increment. Grok mechanically transcribes this report and does not author its verdict.

## Defects

### D1, A containing carrier still substitutes for accepted value and field ownership

`crates/rdocx/src/content_control.rs:4447`
`crates/rdocx/src/content_control.rs:4458`
`crates/rdocx/src/content_control.rs:4485`
`crates/rdocx/src/content_control.rs:5695`

The outer control inventory now uses accepted story spans, and the independent-text-box exclusion is present. The retained inner walks still do not prove ownership of the actual value or legacy field.

read_legacy_anchors accepts every ffData contained anywhere inside a carrier or text box. A foreign opaque wrapper inside an otherwise accepted paragraph or run therefore still contributes a raw anchor that F-288 does not inventory. The added test places its opaque decoy outside all accepted paragraphs, which does not challenge this boundary. Move the decoy inside an accepted paragraph or run before the real field. The raw anchor ordinal shifts while the real F-288 ordinal does not, so overlap and reindex checks can miss the selected live field. The accepted-run provider at crates/rdocx/src/field.rs:1268 and its qualified container policy at :1712 are the existing authority, not containment in a paragraph.

direct_display and rewrite_display likewise accept any Word t whose nearest sdt is the target, except text boxes. They do not require it to be accepted content under that control's sdtContent. For example, an opaque producer property under sdtPr containing a Word t is read as form display and overwritten before the actual displayed text. Filling the control can replace the producer property with the assigned value and clear the real text. An opaque wrapper within sdtContent has the same policy gap. Preserve those unmodelled bytes and select only the actual accepted value runs.

D1 remains open on different nested inputs. Reassess the shared private owner and value boundary. Do not add one more wrapper blacklist or another containment approximation.

### D3, Attribute repair still has unchecked insertion and lexical paths

`crates/rdocx/src/content_control.rs:5686`
`crates/rdocx/src/content_control.rs:5967`
`crates/rdocx/src/content_control.rs:5999`

Existing attributes now use expanded identity, but a missing checked child still goes through prefixed_val using the checkbox element prefix. For a default-w14-namespace checkbox without a checked child, assigning false inserts checked with an unqualified val="0". That attribute is not w14 val. A present checked child without a qualified val reads as true, so assigning false can turn an implicitly false checkbox into true metadata while writing the unchecked display.

The new tag_open_end also takes the first greater-than byte, even when it lies inside a legal quoted attribute value. XML retained with a producer attribute such as note="a>b" is truncated before raw_attribute_spans and the valid operation refuses. Reader event boundaries already know the actual end.

Finally, prefix allocation returns w32 or w1432 at index 32 even when that prefix is already bound. That can introduce a duplicate declaration or capture an opaque attribute's namespace. A namespace binding that is already in use is not a fresh prefix.

All metadata publishers, including creation of an absent child, must use the same checked qualified policy. The existing checkbox writer at crates/rdocx-oxml/src/content_control.rs:561 already authors a fully qualified checked child. There is no approved 32-prefix exception.

### D5, Checkbox state projection still differs from the concrete formatter

`crates/rdocx/src/content_control.rs:4372`
`crates/rdocx/src/content_control.rs:5398`
`crates/rdocx-oxml/src/content_control.rs:511`

Invalid hexadecimal glyphs no longer abort the inventory, which fixes that example. The replacement projection validates only Unicode scalar conversion. A stored state code 0000 is a Unicode scalar but cannot appear in XML text. With an empty display it currently yields no issue, whereas the existing checkbox_state provider rejects non-XML glyphs. The existing provider also validates the state font, which this projection ignores.

The glyph and its selected state font travel together in that concrete provider. The new form value patch carries only a glyph and never applies the declared state font. A valid imported checkbox with a state font different from the current display font therefore gets a different result from the existing typed value path, which passes that font into replace_display_text at crates/rdocx/src/content_control.rs:337. A selected private-use glyph can remain in the wrong font.

Reuse the actual glyph and font policy for selected current-state validation and assignment. Unsupported current state is one issue on a valid rule. Missing or different declared state fonts cannot be silently replaced by a second formatter convention. Preserve empty and unrelated schemas and keep genuine XML and package errors distinct.

### D9, Public control ordinals are global rather than source-part scoped

`crates/rdocx/src/content_control.rs:4159`
`crates/rdocx/src/document.rs:21453`

read_form_state enumerates the complete control_ranges vector once across all parts. A first control in a header therefore receives the count of earlier body controls as its ordinal, instead of zero. The approved public FormValidationIssue contract is source-part-scoped preorder over all controls before tag filtering.

The helper also appends controls in story-owner traversal order without restoring physical source order after deduplication. Distinct owner views must not decide a part's public preorder. Keep deterministic part order, sort each part's distinct accepted controls by physical preorder, then assign a separate ordinal for each source part. Challenge a required-empty body control and the first required-empty header control, and a nested cell control that physically precedes a later body control.

## Closed findings from pass 1

- D2: distinct nested matches are no longer discarded before kind and overlap checks. The wrong leaf-only expectation was replaced.
- D4: lastValue is now edited as the actual qualified attribute, and the test no longer requires a child element.
- D6: temporary list markup preserves absent displayText and value attributes.
- D7: the xml namespace space attribute is changed to preserve when needed.
- D8: the shared schema preflight now resolves the reset binding effects and checks their conflicts before validate, fill, reset, or protect.

These closures concern the inspected corrections. D3 still applies to the shared attribute machinery used by some of them.

## Smells

None separately reported.

## Nitpicks

None.

## Not found

No additional reachable panic, independent publication leak, new source module, unjustified trait, or speculative generic was established. The staged original document is still committed only after successful preparation. The native dense positive remains source-built through public APIs and includes positioned text and deterministic rendering.

## Evidence and boundaries

Consumed actual evidence is 5 passing named form groups, 65 passing existing content-control and legacy-form regressions, scoped cargo check, cargo fmt for rdocx, and clippy for rdocx with all targets and all features. CLI build and authentication preceded the regression batch. Its binary SHA-256 is 2513f6118e3dcdca6aa49aaca47e02180e784dc63de54d8f0edf5fb46a5ea970. The earlier CLI NotFound failure has a successful successor.

The pre-repair batch is honestly recorded as 3 passes and 2 failures, not eight independent RED results. Later assertions in those two tests did not execute. The missing original raw log remains a provenance limitation, and the ready note does not invent it. The later stale-span failures were repair defects and are not original-Base semantic RED.

No new probes or tests were executed by Codex in this review. The open findings above are source-traced counterexamples. Python, HLD, installed typing, Word 16.114, full verification, hash, WASM, packaging and final riders remain pending.

No source or test was changed by this review. This pass ends when the report is written. A separate remediation phase must address D1, D3, D5, and D9 before a clean native claim. Reuse those IDs. If a second repair attempt leaves the same fault on a different input, apply the workflow escalation rather than adding another example-only patch.
