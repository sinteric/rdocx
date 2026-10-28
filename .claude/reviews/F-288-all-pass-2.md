# F-288, all, pass 2

**Reviewed**: Codex independent review of the frozen working diff at Base 7c5614936133f608d05667e0f1523e51c2fbf607. 18 files, 2433 insertions and 120 deletions. Diff SHA-256 1b7cdd20cc27ae1ff1c18b5ae3fa7978843007d83669889c954142f0bc3427ea.
**Verdict**: 5 defects, 0 smells, 0 nitpicks.

## Defects

### D1, nested effective text still disagrees with the authoritative current getter

`crates/rdocx-oxml/src/text.rs:2024`
`crates/rdocx-oxml/src/text.rs:2120`

Direct and ordinary typed text cache reads were repaired. The getter still calls text_current, which returns the raw parent cache and never derives effective children. A legacy text parent with a typed result containing an imported child whose result is absent and default is foo has a physical empty cache but effective display foo. Its current getter and fresh inventory return empty. The approved accessor is the effective current policy, including nested effective text. Fix the shared selection without changing physical parser lengths or borrowed physical APIs. Compare getter, fresh inventory, effective segments, save and reopen on this discriminator.

### D2, typed publications still mask later child edits

`crates/rdocx-oxml/src/text.rs:2184`
`crates/rdocx/tests/integration_test.rs:14601`

The nonempty original-cache case improved, but any typed_cached_runs makes a derived mismatch an override. After set_cached_runs publishes a parent containing a child, cached_field_mut changing that child alone leaves the parent physical cache at its last derived value. The current predicate returns true and hides the child change. The new regression calls refresh_cached_field_projection before checking, so it does not distinguish this failure. An imported virtual-default child in typed parent runs has the same failure even before a later edit. Track actual parent publication versus derived state rather than treating the existence of typed runs as an override. Cover changed child without refresh, with refresh, and a real direct parent override plus changed child. Retain the original source preservation baseline independently.

### D5, text-box fill remains unimplemented at the serialized owner

`crates/rdocx/src/field.rs:1398`
`crates/rdocx/src/field.rs:942`
`crates/rdocx/src/field.rs:950`

The new recursive walker can reach more typed shape paragraphs, but it mutates the typed body while the drawing serializer retains the original raw drawing XML. The worker receipt explicitly confirms that these changes fail closed on reopen. Inventory therefore exposes a field that the promised typed fill cannot update. Atomic refusal is preferable to corruption but does not implement the approved operation. Reuse the existing physical story owner and raw-source publication machinery to patch only the selected field while preserving the rest of the drawing and unknown XML. Do not erase the retained drawing to regenerate unrelated XML. Prove main and related text boxes, nested table or control owners, compatibility prefixes, deduplication and successful exact-owner fill through save and reopen. No acceptance downgrade is approved.

### D6, provider evidence still omits discriminating promised cases

`crates/rdocx/tests/integration_test.rs:14552`
`crates/rdocx/tests/integration_test.rs:14788`
`crates/rdocx/tests/integration_test.rs:14862`

The styled-cache test asserts segment properties but not actual positioned formatting. The ownership test now authors a real nested table field and a header field, but does not exercise nested inline controls, a text box before an old body field, related text boxes, shared headers, or comments as required by C7. It fills only the main table ordinal. The receipt also identifies the broad native batch as preceding the last publisher edit, so those results do not qualify the final shared-helper source. Add the missing bounded discriminators in existing tests and run one fresh affected native batch after the last product edit, plus affected Python and local typed-use checks. Record actual selections and build identities. Existing setup failures and cases without separate pre-fix execution remain honestly distinguished from semantic RED.

### D7, Python setter publishes before preparing its fallible result

`crates/rdocx-py/src/document.rs:10319`
`crates/rdocx-py/src/document.rs:10330`

The changed legacy setter commits the staged document before looking up the default and allocating PyLegacyFormField and its Python members. Either lookup or allocation may fail after mutation, returning an exception despite changed bytes and invalidated handles. The prior implementation prepared the result first. Prepare the fused default lookup and all fallible result objects from the candidate before publish_form_candidate, then return the already prepared object. Preserve existing publication semantics and do not add a post-publication fallible step.

## Smells

None. Pass 1 S1 is resolved.

## Nitpicks

None.

## Resolved and retained observations

Pass 1 D3 is resolved in the inspected segment paths: plain formatted runs, literal source ranges and page or column controls are retained, while physical borrowed APIs stay unchanged. Actual positioned-format coverage remains D6.
Pass 1 D4 is resolved in the inspected factory implementation and stubs: checked native Field, validation before return, approved spelling, keyword-only defaults and reusable clones. Local runtime and mypy receipts exist. The final installed suite and stubtest remain pending.
D1, D2, D5 and D6 retain their original finding IDs because the invariant remains incomplete on the cited inputs.

## Not found

No additional panic, dependency direction, trait, generic, module or canonical child-order defect was found in this pass. No Word, hash harness, full-suite or integrated-union acceptance is claimed. DOCX-055 remains partial with F-288 as live owner and Q4 remains incomplete.

## Review boundary

This review changed no source. Grok may transcribe this exact Codex report only. End the review pass when it is written. Any remediation is a separate orchestration phase.

## Reviewer clarification before remediation

Codex correction to D1's discriminator: use a parsed legacy text parent with a present result containing an imported absent-result child whose default is foo. The original physical parent cache is empty and no typed parent cache was published. Its effective display derives foo while text_current returns empty. In the typed-parent variant originally described, D2 also masks the effective display. These are the same accessor invariant and separate override invariant. No execution is claimed for either source-traced discriminator.
