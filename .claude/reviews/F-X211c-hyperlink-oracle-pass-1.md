# F-X211c hyperlink oracle pass 1

## Scope and verdict

Read-only bounded audit of the inherited hyperlink test and the proposed expectation reconciliation against the approved parent and child contracts. Defects: 0. Smells: 0. Nitpicks: 0 within this reconciliation proposal. No runtime or whole-feature acceptance is inferred. The inherited failing test remains genuine predecessor evidence.

## Contract and expectation reconciliation

The approved parent explicitly requires a captured hyperlink to identify its original physical owner after unrelated add/remove, while removed owners and ambiguous manually constructed records refuse. See .claude/plans/F-X211-design.md:237 and :318. The child owns precisely this remaining physical-identity contract at .claude/plans/F-X211c-design.md:27. The inherited test instead requires the surviving bold link to refuse after the earlier link is removed at crates/rdocx-py/tests/test_core.py:849. That expectation describes the former whole-story layout digest and conflicts with the already approved physical-owner requirement. Revising this expectation implements the approved contract, rather than relaxing it in response to a failure.

The successor should retain the fixture's two remaining links with equal public text, URL and relationship fields but distinct bold formatting. Editing held links[1] must affect only the original bold link at its new physical position, leaving the plain equal-valued sibling unchanged. A separately captured removed links[0] must refuse with first cause Document.remove_hyperlink, including after an identical replacement exists, with package bytes unchanged on refusal. Rebuilt public-field records must still refuse when more than one current link matches, preserving crates/rdocx-py/tests/test_core.py:855 through :870. Actual successful edits and refused operations need save/reopen and unchanged-byte assertions. These controls are outside the immutable100 specimens and must not rewrite their bodies.

## Candidate source assessment

The candidate inventories hyperlinks by actual native story-link source ordering and local content-owner occurrence at crates/rdocx-py/src/document.rs:4079. Captures bind the actual owner state at :7951. Captured resolution validates origin and retirement before consulting the current owner record, then resolves its current physical story and projection ordinal at :5152 through :5172. It does not choose the first equal text, URL or relationship value.

Removal stages the native mutation and identifies the selected physical record before publication at :7989 through :8006. The operation-specific retained-owner mapping shifts only subsequent local hyperlink occurrences in the same selected content owner at :4818 through :4831. The removed old occurrence is excluded from the successor mapping. Other current records use the existing inventory route proof. The manual-record branch at :5192 through :5207 still requires exactly one matching record. This bounded reading supports the proposed distinguishing controls, but does not replace their actual installed runtime qualification or the exhaustive publisher audit.

## Recovery-text compatibility

The inherited tests use lowercase re-fetch at crates/rdocx-py/tests/test_core.py:793 and :849, while captured-owner validation currently supplies capitalized Re-fetch at crates/rdocx-py/src/document.rs:5156. Adjusting that recovery hint's casing is a message compatibility correction only. The origin check, scope validation, earliest retirement cause and physical-owner resolution must remain intact. It cannot justify refetching a replacement on behalf of a held stale capture. The error conversion at crates/rdocx-py/src/lib.rs:112 preserves the actual stale error as a public exception.

## Evidence and limits

No code, test, frozen corpus, target, interpreter, Git state or application was changed. Only this canonical review was written. Current worker source hashes follow. Proposed successor controls and any lowercase hint amendment remain runtime-pending.

```json
{
  "crates/rdocx-py/src/document.rs": "2c5767ff872d473f98753d3d93fa81824b2a8f24e74ef7762c3f46c3d0c0fec1",
  "crates/rdocx-py/tests/test_core.py": "40b81c9e3f76bf4986db2e4437ac5dcd88b74e86cf210d1ac6cc02ddd8aacb22",
  ".claude/plans/F-X211c-design.md": "264b9f084bc442ba9b65519d713610abfad3a33c3617521f24cba2c481a6eede",
  ".claude/plans/F-X211-design.md": "91b0d2e2a26ad40250670167121ff26ae021c05c9a86a1cc0634122bdf1a9762"
}
```
