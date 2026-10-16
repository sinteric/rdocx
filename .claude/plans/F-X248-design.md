# F-X248, Validate identified paragraphs inside block content controls

**Status**: completed
**Sprint**: S95
**Size**: M
**Depends on**: F-X196, F-X211a, F-X222

## Problem

Issue384's unchanged reproduction loses the source validation inventory when a block control contains paraId or textId. The SDT probe in crates/rdocx/src/document.rs:2282 forwards only Word prefixes. CT_Sdt::story_raw_is_typed at crates/rdocx-oxml/src/content_control.rs:1249 then lacks inherited producer namespace bindings needed by paragraph root attributes. The strict owner mismatch at document.rs:2514 is a useful check and must remain.

## Spec reference

- docs/hld/14-development-backlog.md, F-X248. Test gate: regression.
- docs/hld/04-opc-and-packaging.md, Checked Word operation source ownership.
- docs/hld/10-bindings-spec.md, Python Word document operations.
- docs/hld/12-testing-strategy.md, Integrated Python compatibility qualification.

## Approach

Forward the complete effective NsReader resolver scope at the existing SDT probe. Normalize each raw namespace attribute value exactly once using quick-xml Attribute::decoded_and_normalized_value and the actual element decoder, as the existing document.rs namespace declaration reader does. Encode semantic bindings with the established null-delimited scope convention and retain plain Word-prefix entries only for actual Word URIs. Namespace helper visibility and public signatures stay unchanged. Do not merge omitted or shadowed parent declarations into the resolver snapshot.

Keep the physical owner comparison, malformed-source refusal and all existing nesting/resource budgets. Fix the shared native validation path so release CLI validate, installed Document.validate and module validate_file agree. Preserve identifiers, qualified attributes and opaque control children through open/save/reopen. This is a labeled valid-input acceptance correction with no public field, dependency or file addition. It does not introduce a general XML namespace engine or promise support for unrelated previously unsupported document dialects.

## Rejected alternatives

- Removing producer IDs destroys valid source and conceals the cause.
- Relaxing owner comparison accepts genuine owner drift.
- Hardcoding w14 omits other inherited producer bindings and aliases.
- Passing raw URI bytes double-escapes entities when the existing scope serializer owns escaping.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| regression | fx248_identified_block_control_scope_matches_model_and_preserves_source | Exact reporter structure, paraId/textId separately and together, nested controls, TOC and ordinary paragraph validate and preserve expanded attributes |
| regression | paragraph_visibility_checks_owner_slots_and_audit_direct_body_scope | Existing equal-count wrong-owner negative still refuses |
| regression | visible_content_does_not_hide_malformed_remaining_xml | Existing malformed-source refusal remains |
| namespace | fx248_effective_namespace_scope_preserves_producer_attributes | Aliases, local shadowing/default clearing and escaped non-Word producer URI normalize once without resurrecting scope |
| CLI | fx248_validate_reporter_identified_block_control | Unchanged issue reproduction validates with actual release CLI exit zero |
| installed Python | test_fx248_identified_block_control_all_validation_routes | Both public Python routes accept, then saved/reopened IDs and exact opaque bytes independently checked |

Use existing native document tests, crates/rdocx-cli/tests/integration.rs and crates/rdocx-py/tests/test_core.py. Retain the original reporter and establish actual S94 RED separately from strengthened controls. No new test binary or fixture file. Run scoped verification and affected genuine publication dry runs with archive size checks.

## HLD impact

- docs/hld/04-opc-and-packaging.md
- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md

## Risk routing

Parser/serializer: read HLD04 and HLD06, preserve sequence and exact opaque payload, qualify aliases and inherited context. Published API behavior: read HLD10 and structural rules, document valid-input bugfix with unchanged signatures, genuine affected publication dry run and archive ceiling. Binding/WASM: actual installed validation entrypoints, typing/stub checks and both WASM targets. Oracle reproduction remains pinned original evidence, without claiming a new Word capture.

## Hash harness

All 49 hashes and seven deterministic pixel buffers are expected unchanged. No baseline write lease. Unexpected output changes block completion.

## Implementation checklist

- [x] Retain exact Issue384 source and actual pre-repair outcomes.
- [x] Carry normalized full scope at the existing narrow probe.
- [x] Qualify every validation route and independent saved XML preservation.
- [x] Retain genuine owner and malformed-source negatives.
- [x] Complete scoped riders, independent microscope and structured handoff.

## Open questions

No material user decision. The scope and normalization mechanism are selected. Resolve any unexpected source-library behavior against the pinned primary implementation before changing the plan.
