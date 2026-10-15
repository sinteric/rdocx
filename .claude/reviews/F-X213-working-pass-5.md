# F-X213, working implementation, pass 5

**Reviewed**: the complete current working diff against worker HEAD `3723dea217efdd1d1ee37201ec73b0fd1b5dab20`, eight existing files, 1572 insertions and six deletions. This covers both production carriers, the complete native and installed test additions, and all four approved HLD changes.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Source and contract

The approved F-X213 plan requires exact opaque paragraph-property preservation, URI-qualified ownership, balanced first/last entry boundaries, single and zero entry support, safe generic topology or named atomic refusal when overlay is unsafe, and repeat/save/reopen stability. The full source map at `/private/tmp/S91-F-X213-hld-and-gates-prepared/root-hld-adoption-r5/source-after.json` has SHA-256 `235729b8feb0a8bc74bc27eeffc0a888819cb5c0762f6ce97e8f996a2120f31a`. All 2852 current byte, size and mode records match it. The staged index remains `4ef9a90dd6058f25eb6272953a93ba62a8a2732c9acf321059774b9f0f880ed0`.

Read the approved design, cited preservation and TOC HLD sections, current progress, preceding proposal and working reviews, compiler and namespace-inventory corrections, and scoped execution review. The document namespace-inventory correction remains constrained by complete typed-story equality and an empty modeled relationship candidate list. It does not grant a general dirty-source bypass.

## Defects

### D1, replacing tab stops silently discards opaque facts attached to the stop

`crates/rdocx/src/field.rs:9499`, `crates/rdocx/src/field.rs:9503`, `crates/rdocx/src/field.rs:9640`, `crates/rdocx/src/field.rs:9840`.

The boundary scanner resolves each attribute's namespace but discards its local name. It blocks tab-stop replacement only for attributes outside the Word namespace. An unknown Word-qualified attribute therefore passes the safety test. The same scanner ignores comment, processing-instruction and character-data events inside a started stop. The overlay subsequently substitutes generated stops for the complete original `tab` ranges, deleting these facts rather than preserving them or declining the unsafe entry overlay.

For a minimal source example, put the following pPr in the first entry paragraph of the existing three-entry fixture, leaving its begin/instruction/separator runs, last entry's end run and three Heading1 paragraphs unchanged. `w` is bound to the ordinary WordprocessingML namespace.

```xml
<w:pPr><w:pStyle w:val="TOC1"/><w:tabs><w:tab w:val="right" w:pos="100" w:producer="keep"/></w:tabs></w:pPr>
```

The unsupported `w:producer` attribute is unmodeled imported XML. A schema-valid XML comment provides an independent body-payload example without requiring an unknown attribute:

```xml
<w:pPr><w:pStyle w:val="TOC1"/><w:tabs><w:tab w:val="right" w:pos="100"><!--producer token--></w:tab></w:tabs></w:pPr>
```

In both cases, the qualified style and actual cached run make the boundary eligible at `crates/rdocx/src/field.rs:9200`. The default generated entry creates replacement stops at `crates/rdocx/src/field.rs:8940`. The property overlay invokes the stop splice at `crates/rdocx/src/field.rs:9705`, and the original owner including its attribute or comment is discarded at `crates/rdocx/src/field.rs:9840`. The first paragraph is retained successfully, so this is silent data loss rather than the approved safe generic topology. The source-preservation contract at `.claude/plans/F-X213-design.md:43` requires retaining the opaque facts or taking the specified safe fallback. Classify exact owned stop attributes and all opaque body events before choosing replacement, or preserve their source through a justified overlay.

This finding is established by the current source path. No failing runtime reproduction was executed during the review. Existing foreign-attribute and foreign-element handling does not cover unknown attributes in the owned namespace or non-element payload inside a replaced stop.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Six aspects

- Correctness: D1, silent imported-data loss in an otherwise successful rebuild. No other correctness defect found.
- Contract: D1 violates exact opaque pPr preservation and the unsafe-overlay fallback policy. No additional contract defect found.
- Panics: 0 findings. Checked source ranges, scanner depth, complete style ranges and generated-entry slicing were inspected in context.
- OOXML: D1, unmodeled attributes and body payload lost during qualified tab replacement. No additional namespace, field-balance or schema-order finding.
- Tests: 0 separate findings. The 15 native definitions and two installed definitions test genuine existing public behavior, including count changes, single/zero entries, aliases/rebinding, opaque properties, duplicate-property refusal, suffix closure, adjacent fields and repeat/reopen. D1 needs a new discriminating preservation control.
- Structure: 0 findings. Existing concrete carriers and helpers contain the implementation. No new public method, trait, generic, dependency, module or product file was introduced.

## Verification limits

This is a whole working-source microscope, not runtime acceptance. The progress record retains genuine Base failures, the initial compiler failure, and the later namespace-inventory semantic failure after nine controls passed. R4 production and R5 documentation are the current reviewed source. Corrected complete native and installed gates, scoped verification, deterministic hash, packaging and completion obligations remain separate. This review ran no Cargo, product imports, installation, source formatting, environment mutation or native UI. Only this review artifact was written.
