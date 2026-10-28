# F-288, all, pass 1

**Reviewed**: Codex independent working-tree review at Base 7c5614936133f608d05667e0f1523e51c2fbf607. 18 tracked files, 1975 insertions and 119 deletions. Diff SHA-256 2d69362167516097bbe1cd8d8f47c1c3746a75152d3ec9e6c113c5ebf4624c0f. Existing local test receipts were inspected and are not new executions by this reviewer.
**Verdict**: 6 defects, 1 smell, 0 nitpicks.

## Defects

### D1, current-value selection does not cover cache publishers

`crates/rdocx-oxml/src/text.rs:2013`
`crates/rdocx-oxml/src/text.rs:1201`
`crates/rdocx/src/field.rs:1228`

The authoritative getter and fresh inventory clone legacy metadata instead of reading effective current text. A text field constructed with current Old and then changed through public cached_result to New still reports Old. set_cached_runs also leaves current metadata and result-presence state unchanged. An explicit empty typed cache on an imported absent-result field can continue displaying its reset fallback. Route direct, typed, annotation, refresh and nested publishers through the approved current and presence policy. Keep checkbox and list metadata authoritative and preserve reset defaults. Add discriminating publisher regressions, including explicit empty publication and candidate-byte no-op controls.

### D2, unchanged nonempty parent cache masks changed children

`crates/rdocx-oxml/src/text.rs:2125`

The override predicate exempts an unchanged original parent only when that original is empty. A nonempty original parent cache remains unchanged when cached_field_mut changes a child. Its difference from the newly derived child text is then misclassified as a direct parent override, hiding the child update. Distinguish actual parent publication from derived cache state, including typed publishers, without altering the retained original source used for preservation. Test nonempty literal spans around the child, changed child alone, and an observable parent override plus changed child.

### D3, semantic layout segments lose existing formatting and content

`crates/rdocx-oxml/src/text.rs:2095`
`crates/rdocx-oxml/src/text.rs:2149`
`crates/rdocx-oxml/src/text.rs:2164`
`crates/rdocx-oxml/src/text.rs:2183`

Plain cached runs without nested fields fall through to one segment with no CT_RPr. The layout engine now consumes this method, so an ordinary bold or otherwise formatted cached field loses formatting. The parsed cached_fields fallback emits only child segments and drops literal spans around them. Nested typed-run conversion also uses content_text instead of retaining the existing page and column break segment policy. Preserve the physical helper's formatting, literal ranges, controls and child owner behavior while adding effective legacy fallback. Keep the borrowed physical APIs unchanged. Assert actual segment properties and positioned layout, plus controls and nested literal text.

### D4, Python factory signatures and validation violate the contract

`crates/rdocx-py/src/document.rs:7280`
`crates/rdocx-py/src/document.rs:7360`
`crates/rdocx-py/src/document.rs:7401`
`crates/rdocx-py/src/document.rs:7454`
`crates/rdocx-py/src/document.rs:7493`

The approved keyword-only text value and checkbox checked defaults are missing. Drop-down selected_index is required positional instead of optional keyword-only. size is exposed instead of size_half_points, and autotext is accepted instead of the approved autoText. Matching stubs reproduce the same incorrect interface. The frozen definition stores unchecked options and definition, and calls native validation only during attachment. Thus empty lists and other native-invalid definitions can be returned successfully by a factory. Store the checked native Field, validate before return with the existing error mapping, and clone it for both attachment routes. Align runtime, exports, stubs and actual local typed use with every approved signature and default.

### D5, text-box inventory and fill use different recursive maps

`crates/rdocx/src/field.rs:1279`
`crates/rdocx/src/field.rs:1401`
`crates/rdocx/src/field.rs:1427`
`crates/rdocx/src/field.rs:1435`

Read inventory recursively collects body paragraphs in a shape text body. The new mutable collector only visits direct cell paragraphs and direct control paragraphs. A field inside a nested table or a table within a content control in that text box is listed but skipped on fill, allowing a missing or wrong ordinal target. Use equivalent recursive physical ordering for read and write, preserving old ordinal prefixes and deduplication. Test nested table and control owners, a text box before an old body field, related text boxes, shared headers and comments with exact targeted fill and reopen assertions.

### D6, named acceptance tests do not prove their advertised boundaries

`crates/rdocx/tests/integration_test.rs:14605`
`crates/rdocx/tests/integration_test.rs:14629`
`crates/rdocx/tests/integration_test.rs:14735`

The invalid-creation test covers only an empty drop-down and an unrelated unchanged document. The nested-story test creates a table but inserts into the added main-body paragraph, proving neither nested attachment nor related owners. The cross-consumer test checks positioned layout and PDF but omits explicit native text assertions and invokes an ambient target/debug CLI without binding its build to reviewed source. Strengthen the approved matrix with real table, nested inline and supported related attachment, both routes, contradictory manual kind and bounded primitive cases, atomic destination and handle assertions, and direct native text before and after empty assignment. Build and authenticate the current CLI before consuming its result. Run the required small local mypy use against the built public module. Keep final installed-suite and Word qualification explicitly pending.

## Smells

### S1, new helper only forwards one operation

`crates/rdocx/src/field.rs:1260`

remember_legacy_paragraph only forwards HashSet.insert, has one caller, and ignores the return. Inline this operation in the existing traversal rather than adding a forwarding wrapper.

## Nitpicks

None.

## Not found

No additional panic, new dependency direction, new trait, generic or module defect was found in the inspected diff. Canonical author child ordering and reset-default preservation were inspected. This does not discharge authentic Word behavior, final hash qualification, the full installed binding suite or the integrated union gate. DOCX-055 remains partial and the local typed-use gate remains unrun, so the provider prefix is not complete.

## Review boundary

This review made no source changes. Grok may mechanically transcribe this exact Codex report only. End this review pass when the report is written. Remediation is a separate orchestration phase.
