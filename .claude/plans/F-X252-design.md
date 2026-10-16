# F-X252, Refuse newly invalid Word raw XML atomically

**Status**: completed
**Sprint**: S95
**Size**: M
**Depends on**: F-X211a

## Problem

Issue385's pinned test at test_docx_everyday.py:232 inserts direct paragraph children w:bogus and foreign foo without Ignorable. The checked native paragraph replacement at crates/rdocx/src/document.rs:22122 retains physical owner and resource checks but does not reject those newly authored children. The modeled traversal predicate admitted_word_child is not a complete schema whitelist and must not become one.

## Spec reference

- docs/hld/14-development-backlog.md, F-X252. Test gate: regression.
- docs/hld/04-opc-and-packaging.md, Checked Word operation source ownership.
- docs/hld/10-bindings-spec.md, Python Word document operations and Word checked XML owner authority.
- docs/hld/12-testing-strategy.md, Integrated Python compatibility qualification.
- Official ECMA-376 fifth-edition Transitional CT_P particle and group expansion, wml.xsd:2236-2240, plus Part3 clauses 7.2 and 9.2 and Annex A.1.2. Authenticated original archives, XSDs, PDF and extracted names remain in private primary research.

## Approach

Add bounded edited-paragraph admission at the existing checked replacement boundary, after current physical ownership/no-op/source resolution and before mutation. Derive actual namespace and accumulated MC context from that selected owner's source ancestors. Do not change ordinary import, a global renderer parser, or other raw XML entrypoints.

The base direct-child grammar is optional single pPr first, followed by the official EG_PContent closure. It admits the 33 qualified Word names pPr, customXml, smartTag, sdt, dir, bdo, r, fldSimple, hyperlink, subDoc, proofErr, permStart, permEnd, bookmarkStart, bookmarkEnd, moveFromRangeStart, moveFromRangeEnd, moveToRangeStart, moveToRangeEnd, commentRangeStart, commentRangeEnd, customXmlInsRangeStart, customXmlInsRangeEnd, customXmlDelRangeStart, customXmlDelRangeEnd, customXmlMoveFromRangeStart, customXmlMoveFromRangeEnd, customXmlMoveToRangeStart, customXmlMoveToRangeEnd, ins, del, moveFrom and moveTo, plus qualified math oMath and oMathPara. Stop expansion at element declarations. Bare math fractions/runs, direct w:t/tbl/drawing/sectPr and unknown Word names are not newly admitted here. Select only the already-supported document vocabulary, without adding Strict import support.

Resolve Ignorable prefix tokens at the declaration element and accumulate namespace URIs through actual ancestry. A local or blank list does not clear ancestors. Later prefix rebinding cannot retarget inherited authority. Restore scope on sibling exit. Unbound declarations, foreign/unqualified Ignorable lookalikes and an understood Word namespace do not grant an unknown Word child permission. Qualified math is admitted by host grammar independently of Ignorable. Namespace attributes are normalized once, while URI equality remains exact.

At the new direct-child admission frontier, preserve an imported opaque child as an exception only when the selected physical owner, ordered structural correspondence, original raw bytes and relevant namespace/MC meaning agree uniquely. Every original direct-child exception must retain its route and occurrence, so moving it into an already admitted wrapper does not transfer authority. An old/new raw-subtree multiset is insufficient because it admits copies or moves. Reuse existing physical source/cursor providers and add a private concrete provenance check in document.rs. Refuse ambiguous occurrence, changed ancestry or reinterpreted scope rather than inventing authority. Exact source no-op remains the earliest accepted path. No new owner model, public snapshot field or public source-evidence API.

This exception governs the direct CT_P frontier, not a recursive unknown-Word policy below already admitted typed owners. Nested content controls, fields, ruby, revision, DrawingML and OMML retain their existing checked providers. Valid nested scalar and typed metadata edits remain supported. The checked-owner scanner describes validation boundaries, not a complete opaque-storage inventory, and must not supply an immutable model-wide range list.

New ignored foreign direct children may remain opaque under a correctly applicable MC grant. Existing valid extensions survive. Newly introduced complex AlternateContent/ProcessContent/MustUnderstand constructs outside an existing admitted route refuse explicitly rather than imply full MC support. Qualify retained MC element islands and unsupported MC attribute declarations at their actual physical routes. An unchanged attribute retains its raw value and resolved declaration meaning without freezing the entire typed Word owner, so unrelated supported text or metadata edits remain possible. Existing descendant/relationship checks, sequence restrictions, parser budgets and candidate reopen remain in force. The new grammar check is finite direct paragraph admission, not full recursive OOXML validation. A pre-existing invalid known sequence can still refuse an attempted edit, while unchanged imported no-op retains its established behavior.

Refusal identifies the operation/expanded child and occurs before candidate publication. Preserve bytes, held handles and unrelated package parts. Existing Python ValueError or RdocxError routes remain acceptable for the exact mutations. No source rewrite merely to manufacture namespace closure, dependency, feature, module or file addition. Label newly refused authored XML behavior separately.

## Rejected alternatives

- The renderer-modeled whitelist rejects valid unmodeled Word markers and math.
- SDK child registration lists are broader than the schema particle.
- A generic schema engine adds unrelated framework and dependencies.
- A global unknown-XML rejection destroys imported opaque source.
- Raw equality without ancestry/context accepts moved or reinterpreted islands.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| reporter | Original test_raw_xml_that_word_refuses_is_refused | Exact two pinned direct-child mutations refuse with source unchanged, original strict report retained |
| native grammar | fx252_edited_paragraph_direct_children_follow_schema | Official 35 names and pPr ordering controls, invalid direct Word/math placement rejected |
| namespace/MC | fx252_ignorable_uris_follow_declaration_ancestry | Self/ancestor accumulation, aliases/default names, local blank lists, prefix rebinding, wrong/unbound attributes, sibling scope and unchanged unsupported MC attributes around valid typed edits |
| preservation | fx252_unchanged_opaque_islands_require_physical_correspondence | Direct-child exception routes retain unique physical correspondence and raw meaning, exact no-op and valid scalar edits preserve source, duplicates/moves/reinterpreted/ambiguous exceptions refuse |
| installed Python | test_fx252_raw_xml_refusal_preserves_current_owners | Both mutations, stale/foreign owner and late relationship failures preserve bytes and authority, valid markers and OMML reopen |
| authority | Authenticated official XSD particle and Part3 controls | Independent grammar and MC evidence, not inferred from the changed product parser |

Use native document/integration carriers and existing Python test_core.py, without a new test binary. Keep current owner, cursor, source marker, recursion, relationship and malformed XML negatives. Run scoped floor, affected genuine publication/size checks and real installed/typing controls.

## HLD impact

- docs/hld/04-opc-and-packaging.md
- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md

## Risk routing

Parser/serializer: read HLD04/HLD06, schema child order, qualified parsing, bounded source scans and byte-exact opaque round-trip. Public behavior: read HLD10/structural rules, describe the finite newly authored XML refusal with unchanged public Rust signatures and ordinary import preservation, affected genuine publication and archive ceiling. Binding/WASM: actual installed reporter/atomicity checks, strict typing/stubtest and both WASM targets. External authority: pin official ECMA edition and exact downloaded schema/Part3 bytes, label derived controls separately from native Word observations.

## Hash harness

All 49 hashes and seven deterministic pixel buffers should remain unchanged. This changes invalid checked-edit admission, not accepted paint semantics. No baseline lease.

## Implementation checklist

- [x] Retain actual pre-repair reporter outcomes and primary authority.
- [x] Add direct-child grammar at the selected checked paragraph boundary.
- [x] Resolve inherited MC URI authority and direct-child exception provenance, preserving delegated nested edits.
- [x] Qualify valid extensions and all owner/no-op/refusal negatives.
- [x] Complete installed/scoped riders, independent microscope and handoff.

## Open questions

No material user question. The finite grammar and accumulated URI semantics are selected from primary authority. Any unforeseen unsupported MC construct remains an explicit refusal or existing unchanged island, not implied full schema support.
