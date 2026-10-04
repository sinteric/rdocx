# F-X161, Compact Word XML and namespace preservation

**Status**: completed
**Sprint**: S84
**Size**: L
**Depends on**: F-X144, F-X160, F-X169

## Problem

`push_root_attribute_record` repeats the canonical `xmlns:w` binding on
elements carrying producer attributes (`crates/rdocx-oxml/src/text.rs:135`).
The document writer indents every element of a rewritten part
(`crates/rdocx-oxml/src/document.rs:2929`). Issue 245's one-word edit therefore
changes the whole XML diff and grows the part substantially.

## Spec reference

- `docs/hld/04-opc-and-packaging.md`, "The package" and "Package integrity".
- `docs/hld/12-testing-strategy.md`, "The Word corpus" and "The hash harness".
- `docs/hld/14-development-backlog.md`, "F-X161, Compact Word XML and namespace preservation".

## Approach

Review only PR 251's increment against the F-X160 prefix. Skip a retained
canonical Word prefix declaration only where that binding is already in scope.
Preserve alias and foreign bindings, raw unknown subtrees, producer attributes
and root `mc:Ignorable`. Use compact writers for changed document, note,
header, footer, style and numbering parts. Keep untouched package parts and
regions byte-identical. Recheck the PR's stale candidate digest against the
current prefix before recording any baseline change. Remeasure the `rdocx` and
`rdocx-oxml` archive inventories and update their README assertions after the
serializer changes.

## Rejected alternatives

- Remove all repeated namespace declarations. An alias or shadowed binding can
  be required for valid XML.
- Pretty-print only the edited element. That still rewrites surrounding modeled
  XML and fails Issue 245's compact-part contract.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| round-trip | Issue 245 source-built 50-paragraph edit and reopen | One root `xmlns:w`, compact size and layout, preserved attributes and untouched content |
| round-trip | Word part writer cases for document, notes, headers, footers, styles and numbering | Required namespace bindings, schema order and raw subtree bytes survive |
| regression | `test_issue_160_producer_matrix_across_operations_and_picture` | The 11 by 8 producer matrix and valid `mc:Ignorable` survive serialized output |
| harness | `python3 scripts/hash_harness.py --check` after the labelled baseline review | Exactly the declared 20 OOXML entries change |

**Test gate**: round-trip. Issue 245's edited package retains untouched
regions and valid namespaces, and its 20 declared hash entries reconcile.

## HLD impact

- `docs/hld/04-opc-and-packaging.md`, edited Word part serialization.
- `docs/hld/09-charts-spec.md`, only if the PR's changed chart-related source
  contract is retained after incremental review.

## Risk routing

- Parser or serializer: read `docs/hld/04-opc-and-packaging.md` and
  `docs/hld/06-presentationml-model.md`. Check `xsd:sequence`, prefix tolerance
  and byte-preserving `capture_element` round trips.
- External oracle: read `.claude/skills/differential-testing.md`. Pin the
  python-docx and Word versions and compare structure, not writer bytes.

## Hash harness

Expect exactly 20 OOXML hash entries from compact serialization, individually
explained. No PDF or PNG delta is expected. Review and commit this baseline
separately before F-X162 or F-X163 begins.

## Implementation checklist

- [x] Reconcile PR 251 against the integrated prefix and its stale digest.
- [x] Implement canonical binding and compact writer changes without losing
  source attributes or raw subtrees.
- [x] Run the Issue 245 reproduction and full Issue 160 producer matrix.
- [x] Label and review each of the 20 hash changes.
- [x] Remeasure the two changed package archives and their README assertions.
- [x] Pass focused tests, risk riders, scoped verification and microscope.

## Open questions

None. Recompute the stale candidate digest on the integrated prefix and treat
any unrelated difference as a separate defect.
