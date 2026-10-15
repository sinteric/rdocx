# F-X211b, native DiagramML text, pass 1

**Reviewed**: Bounded regression in the frozen shared TextBody parser and its existing DiagramML/chart consumers. Native14 remains byte-identical to the native pass2 capsule. Public pass2 and other clean native behavior are not confirmation-reviewed.

**Verdict**: 1 defect, 0 smells, 0 nitpicks. No whole-child acceptance.

## Contract and evidence

Read the canonical microscope/workflow, approved parent/b contracts and their modeled namespace/source-preservation requirements. All native14 current bytes authenticate to manifest SHA `9e3f79930ce393af96df4ee71c2759dea2d4149ac90fa614b8d858a397d946b1` before report publication.

The root broad native receipt `/private/tmp/S91-F-X211b-root-native-tests-1/receipt.json` SHA `aa3c67c86772e05cdc4086254f0070f5d3ad9235a59b7b23ab272c05624c1766` records actual exit 101 with safely reaped leader and gone process group. Its rpptx library log SHA `a1deb426203b6f8d6b2c826e0b432be546f30a9c1e783e1cdea034545065cdf2` authenticates 146 passed, 1 failed and 5 ignored. The failed existing test is `diagram::tests::authoritative_empty_data_text_never_uses_stale_presentation_text`, not a new test setup correction. No runtime was launched by this review.

## Defects

### D1, shared root namespace restriction rejects existing DiagramML and chart TextBody wrappers

`crates/oxml-drawing/src/text/mod.rs:126`

`from_xml_as_with_namespaces` accepts a caller-selected root local name but then rejects every explicitly resolved root URI except DrawingML main and PresentationML main. It applies this URI check regardless of the selected local name. This breaks the real DiagramML caller at `crates/rpptx-oxml/src/diagram.rs:831`, which passes `t` with the actual enclosing namespace bindings.

That caller has already required the root to be official DiagramML `t` at `crates/rpptx-oxml/src/diagram.rs:819`. Its admission helper at `crates/rpptx-oxml/src/diagram.rs:1690` independently requires the root URI to be DiagramML and every modeled descendant URI to be DrawingML main. The existing official DGM/A source at `crates/rpptx/src/diagram.rs:4991` is therefore valid for this concrete provider. Instead, its parse fails with `foreign text body namespace` and the existing unwrap at `crates/rpptx/src/diagram.rs:4993` exposes the regression. The test's authoritative empty data node cannot be produced, so the promised empty-versus-stale presentation-text distinction never executes.

The same root restriction also contradicts two existing chart consumers. Axis text properties at `crates/oxml-chart/src/lib.rs:7027` and chart-space text properties at `crates/oxml-chart/src/lib.rs:10488` parse caller-selected `txPr`, then serialize `c:txPr`. An official chart-URI declaration carried locally on that wrapper is rejected by the same A/P-only check. This chart consequence is source-proven, not claimed to have been runtime reproduced in this pass. Both consumers call the namespace-free convenience method, so existing inherited-only chart fragments can mask the faulty restriction.

Minimum repair obligations are finite and consumer-driven. Preserve existing `txBody` in official DrawingML/PresentationML, admit official DiagramML only for `t`, and admit official chart only for `txPr`. Qualify the real inherited/local namespace contexts and valid alias/default root spellings for these existing consumers. Continue requiring actual DrawingML child ownership and rejecting foreign/rebound roots, wrong namespace/name combinations and malformed/trailing input. Do not turn the caller-selected local name into a blanket foreign-URI allowance. No current `CT_TextBody` parser caller for chart `rich` was found, so this review does not authorize a speculative additional wrapper tuple.

The successor must run the unchanged original DiagramML test and deciding official-wrapper positive/foreign negative controls in existing test entrypoints. It must preserve authoritative empty text, stale presentation fallback refusal, retained source spellings and actual save/reopen behavior. Retain the original broad failed receipt. Existing scoped native owner controls do not replace this exposed shared consumer regression.

## Smells

0.

## Nitpicks

0.

## Not found

Correctness and contract: D1, one shared root restriction with two concrete consumer families. Panics: no new parser panic defect found, the cited test unwrap exposes the incorrect returned error. OOXML: D1 is a namespace admission defect, not permission to loosen child ownership. Tests: the unchanged original test is a valid regression oracle. Structure: 0 findings, the repair belongs in the existing concrete parser and callers, with no framework/module/dependency expansion.

Native14 and the output worker's tracked sources remain unchanged. No fixes, tests, targets, CP or Office operations were performed. This bounded pass certifies neither a repaired runtime nor whole b, parent frozen100, archive/policy or native Office acceptance.
