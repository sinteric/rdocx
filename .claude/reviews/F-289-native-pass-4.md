# F-289, native, pass 4

**Reviewed**: Root Codex independently reviewed the frozen native working diff on `work/f-289-grok`, HEAD `c0b58c7fa8e4e919b9bfe5c02932f74827cea36b`. Full diff SHA-256 `6390785148533b119d5e9d4b12b59eaf3e63ee3b3e1fedb4a829952ab9445682`. Six tracked files, 5190 insertions and 45 deletions. The three-source-path hash `104539ca79233d1aecfc95ed0d53d54391fa870e05b93cfb75842229c5211de8` is distinct. This is a native checkpoint, not whole-feature completion.
**Verdict**: 1 defect, 1 smell, 0 nitpicks

## Defects

### D10, the checkbox policy shell drops an inherited default namespace
`crates/rdocx/src/content_control.rs:4399`
`crates/rdocx/src/content_control.rs:4406`
`crates/rdocx/src/content_control.rs:4428`
`crates/rdocx-oxml/src/content_control.rs:790`
`crates/rdocx-oxml/src/content_control.rs:856`

The selected-state reader copies the checkbox subtree into a temporary control, but in_scope_xmlns keeps only named prefix bindings. Consider a valid checkbox whose w14 default namespace is declared on its enclosing w:sdtPr, with an unprefixed checkbox and unprefixed checkedState and uncheckedState children. Their value and font attributes remain explicitly w14-qualified. The property and type writers preserve this inherited spelling. In the temporary shell the inherited default binding is missing, so the copied element and its children have different expanded names. The shared checkbox provider then cannot read the same declared state. A valid imported current state becomes an issue or a valid selected assignment is refused, rather than using its declared glyph and font.

The existing default-namespace test declares xmlns on the checkbox itself. That declaration is inside the copied substring, so the passing fixture does not challenge the inherited case. Preserve the entire effective namespace context, including the default binding, while giving the synthetic Word shell a proven Word prefix. A producer binding for the literal prefix w must not determine the shell's expanded name. The copied subtree must retain its own bindings and local overrides.

This is a newly identified read-context defect. D3's font-attribute publication paths are fixed.

## Smells

### S1, removed provenance facts still travel through the metadata walk
`crates/rdocx/src/field.rs:1142`
`crates/rdocx/src/field.rs:1158`
`crates/rdocx/src/field.rs:1344`
`crates/rdocx/src/field.rs:866`

AcceptedLegacySource now contains only the original public metadata and default pair, and is immediately mapped back into that pair. Its source payload was removed. The in_textbox parameter is forwarded through the recursive walk and finally discarded as _in_textbox. Neither adds a case or fact used by a consumer. This retains an extra carrier and parameter path after the ownership design changed, contrary to the repository's no forwarding wrapper and no unused parameter rules. Keep the shared concrete metadata and optional source-capture walk, but remove this unused context and unnecessary pair conversion.

## Nitpicks

None.

## Closed findings and evidence

D1 is closed on the examined paths. Physical anchors now use accepted paragraph spans and the existing qualified typed-field-source checker, rather than the first equal substring in a part. The byte-identical opaque decoy, newly authored field before caller save, and separate identical accepted fields are covered. Public metadata no longer encodes source_replacement. The private Document story mode supplies main-body paragraph boundaries, while related parts retain the existing relationship checks.

D3 is closed for font publication. New property and font nodes are Word-qualified, including existing default-namespace inline runs without rPr, existing properties without fonts, and inserted display. Existing qualified font updates preserve producer attributes and remove conflicting theme attributes. The positioned glyph uses deterministic Liberation Mono for stored Courier New.

D2, D4, D5, D6, D7, D8, and D9 remain closed on the previously examined paths, subject to D10's newly identified imported-context case.

Consumed current scoped evidence: cargo check, fmt, and clippy passed. The final integration command passed 85 tests, 0 failed, 376 filtered, in 3.55 seconds. Its groups are five original form groups, eleven review2 cases, four review3 cases, fifteen content-control cases, and fifty legacy-form cases. The current CLI SHA-256 is `ff835bb82c6dfbd4c0aa881b384a49734858e5bc24c26c53f1ce2a332feb920d`. The render-fallback case ran against that binary. The raw log is `/private/tmp/s95.4-f289-native-final-local.log`.

The unavailable remediation-1 RED log remains unavailable. D10 has source-level evidence here, not a claimed executed RED. This review did not rerun tests or change implementation files.

## Not found

No additional panic, schema-order, or unrelated behavior defect was established in the examined repaired paths. Zero nitpicks is intentional.

## Pending scope

Python bindings, carriers, exports, stubs, typed public use, related-owner publication, three Python groups, and five HLD updates remain pending. Word A7, hash harness, full verification, WASM, installed typing, and publish riders remain pending. This report does not qualify whole-feature completion.
