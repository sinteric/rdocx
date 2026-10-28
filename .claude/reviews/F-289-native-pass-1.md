# F-289, native interim microscope, pass 1

**Implementer**: Grok
**Actual reviewer**: Codex root
**Reviewed**: Frozen native working diff against c0b58c7fa8e4e919b9bfe5c02932f74827cea36b in work/f-289-grok. Three files, 2952 insertions and 1 deletion. Diff SHA-256 a3e53d7df2d40e766d2cc32897ec4bdcb633d36134a9c86aec5fa19e5137548f.
**Verdict**: 8 defects, 0 smells, 0 nitpicks.

This is an interim review of the native increment. Python bindings and the five HLD updates are the planned next increment and are not treated as missing completed work. This report does not approve F-289 completion. Codex read the approved plan, native implementation, changed tests and affected existing providers. Grok transcribes this report and is not its reviewer.

## Defects

### D1, Raw scans bypass the authorable physical-owner inventory

`crates/rdocx/src/content_control.rs:4151`
`crates/rdocx/src/content_control.rs:4392`
`crates/rdocx/src/content_control.rs:5542`

Every qualified sdt anywhere in a story part is admitted, without proving that it belongs to an accepted authorable story or container. This reaches Word-namespaced lookalikes inside opaque wrappers, deleted or otherwise unsupported contexts. The display walk likewise uses only nearest-sdt ancestry, so text in an independent text box can be read and rewritten as the containing control's text.

The separate ffData scan also numbers every raw occurrence, unlike F-288's accepted field inventory. An opaque ffData before a live field can therefore shift the overlap anchors away from the legacy ordinal supplied by the real provider. Selection, overlap checks and mutation must share the accepted physical-owner policy. Part-name deduplication is not that proof. Add ignored-wrapper and independent-text-box controls beside valid authorable siblings, plus an opaque ffData before a selected live legacy owner. Preserve the supported positive cases.

### D2, Distinct nested controls disappear before kind and overlap checks

`crates/rdocx/src/content_control.rs:4584`
`crates/rdocx/src/content_control.rs:4605`
`crates/rdocx/tests/integration_test.rs:31054`

most_specific drops every matching outer control that contains another matching control, even though their physical spans differ. This is not deduplication of two story views of the same physical control. A structural outer control and a scalar descendant with the same tag bypass the required whole-batch wrong-kind refusal. Heterogeneous and overlapping selections are similarly hidden before validation.

Deduplication is by the same part, start and end. Most-specific story ownership must not discard a different physical target. The existing nested test explicitly expects only the leaf to change and thus ratifies the wrong selection policy. Challenge a same-tag structural parent and scalar child as well as valid distinct nonoverlapping fan-out.

### D3, Metadata writes match local spelling instead of expanded attribute identity

`crates/rdocx/src/content_control.rs:5674`
`crates/rdocx/src/content_control.rs:5684`
`crates/rdocx/src/content_control.rs:5703`

find_attribute searches raw bytes for a local name and accepts any prefix. For a date carrying f:fullDate before the real qualified Word fullDate, reset removes the foreign attribute and leaves the active date. A checkbox carrying f:val before its real w14 val similarly edits the foreign attribute while leaving the authoritative state unchanged. Legal whitespace around the equals sign is not recognized. Literal attribute values can also contain the searched text.

The new-attribute path derives the attribute prefix from the element name. A default-namespaced Word or w14 element has no prefix, but its unprefixed attributes have no namespace, so that path writes a nonauthoritative val or fullDate. Use checked namespace-qualified attribute spans and preserve opaque lookalikes. The existing qualified metadata provider tests at crates/rdocx-oxml/src/content_control.rs:2576 already demonstrate the required foreign and alias distinction.

### D4, List selection hints are written as child elements instead of attributes

`crates/rdocx/src/content_control.rs:5502`
`crates/rdocx/src/content_control.rs:5512`
`crates/rdocx/tests/integration_test.rs:31419`
`crates/rdocx/tests/integration_test.rs:31436`

The imported list hint is the qualified lastValue attribute on comboBox or dropDownList. edit_last_value instead finds or inserts a lastValue child. Placeholder reset therefore leaves a real imported lastValue attribute in place, and value assignment emits a child outside the declared list-item sequence. The new tests construct and assert that same incorrect child form. Existing imported examples at crates/rdocx-oxml/src/content_control.rs:2764 and :2867 use the actual attribute.

Read-only paths must preserve the imported hint. Placeholder reset clears the actual qualified attribute. If an assigned value updates the hint, it must use its actual schema slot. Do not normalize an unrelated foreign attribute or manufacture a child to satisfy a test.

### D5, Invalid current glyph metadata aborts inventory construction

`crates/rdocx/src/content_control.rs:4310`
`crates/rdocx/src/content_control.rs:4326`
`crates/rdocx/src/content_control.rs:4329`

A nonhex or nonscalar imported checkedState or uncheckedState value propagates Err while reading the entire form state. A valid checkbox rule with a valid Checked reset must instead produce one current-state issue for an unsupported or invalid current state. The eager scan also makes an unrelated malformed checkbox affect an empty or differently selected schema. Keep the invalid current fact as data until selected validation, while retaining genuine package, XML and unresolved-store ownership errors as Err.

### D6, Reconstructed list metadata changes absent attributes into empty values

`crates/rdocx/src/content_control.rs:5094`
`crates/rdocx/src/content_control.rs:5097`

list_shell always authors both displayText and value, replacing None with an empty string. The existing resolver at crates/rdocx-oxml/src/content_control.rs:244 falls back from an absent displayText to value, while refusing a missing required value when a selected item has only a label. A valid imported item with only value="r" should display r. This adapter turns its display into empty text, reports the existing display as inconsistent and clears it on assignment. Conversely, an absent value becomes an explicit empty bound value instead of following the existing refusal.

Preserve absence and reuse the actual resolver semantics. Include value-only and label-only imported items beside the usual two-attribute items.

### D7, Text assignments fail to replace xml:space default

`crates/rdocx/src/content_control.rs:5652`

ensure_preserve returns whenever xml:space occurs, regardless of its value. Assigning leading or trailing XML whitespace to a text element carrying xml:space="default" leaves preservation false. The modeled serializer also retains preserve_space only for the preserve value, as shown at crates/rdocx-oxml/src/text.rs:3665 and :4006. The stored text therefore lacks the preservation required for its assigned value. Update the actual xml namespace attribute to preserve when needed and retain unrelated attributes. Test an explicit default value with leading and trailing spaces through fill, save and reopen.

### D8, Conflicting reset bindings are not part of schema preflight

`crates/rdocx/src/content_control.rs:4560`
`crates/rdocx/src/content_control.rs:4566`
`crates/rdocx/src/content_control.rs:4567`

select_rules checks reset payloads separately and reads bindings, but never resolves conflicts among the schema's reset writes. Two distinct controls bound to the same scalar can carry different typed reset values. With consistent current displays, validate_form reports no issue and protect_form can succeed. An empty or partial fill accepts the same schema. reset_form later rejects it only in preview_bindings because its two reset writes conflict.

The plan requires complete schema and ownership preflight, including conflicting bound targets, even for partial fill. Resolve physical bound selections and conflicting reset effects at that shared boundary. Keep legitimate consistent binding sharing distinct from conflicting writes, according to the existing edit resolver.

## Smells

None separately reported. D1, D3, D4 and D6 include the concrete consequences of duplicated policy and are not counted again as smells.

## Nitpicks

None.

## Not found

- Panics: no additional reachable panic was established in the reviewed native paths.
- Atomic publication: the clone and final commit boundary show no separate publication leak in the paths inspected. The selection and XML defects above still block correctness.
- Structure: no additional unjustified source file, module, trait or speculative abstraction finding.
- Tests: the source-built dense positive uses public creation, binding, permission and field APIs. It includes positioned text and a deterministic PNG invocation. That is local evidence and does not discharge the pending independent Word gate.

## Evidence and exclusions

Consumed actual worker evidence is five passing named native groups and cargo check -p rdocx --all-targets --offline exit 0. The additional existing batch has 64 passes and one failure before form assertions because the worktree-relative CLI executable is missing. That failure is not a semantic pass and needs a checked artifact route and successful successor. No new tests or probes were executed by this review. Findings are source-traced, not claimed runtime reproductions.

Python, HLD, fmt, changed-crate clippy, current Python rebuild and typed use remain next-increment work. Final Word 16.114 comparison, full suite, hash harness and risk riders remain pending. No source or test was changed by Codex in this review. The review pass ends when this report is mechanically written. Remediation is a distinct subsequent phase.
