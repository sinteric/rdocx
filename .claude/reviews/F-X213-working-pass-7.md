# F-X213, working implementation, pass 7

**Reviewed**: the complete final working diff against worker HEAD `3723dea217efdd1d1ee37201ec73b0fd1b5dab20`, eight existing carriers, 1755 insertions and six deletions. The scope includes both production carriers, 19 native controls, two installed controls and four approved HLD changes.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Authority

Canonical CLAUDE, WORKFLOW and microscope, the approved design and cited preservation, rendering and testing sections, current sprint/backlog, progress and working pass6 govern this pass. F-X213 remains in-progress. All 2852 current tracked byte, size and mode records reproduce sorted indent2 JSON with final LF digest `ce4d05ffe511ec27bc53571937ec6b346ab34b0510023697f3082abda682b61c`. Only field.rs and regression_test.rs differ from the R5 map. Their current hashes are `034beca4b5a42afccfdc3b854d309c64ed3e53b1525aa7e93a32381b38b9c72e` and `1075e89f5e1718b1e757b0440dfec2c6744314b0088a030cbb224b2e9d09d0eb`. HEAD remains unchanged.

Historical authorship includes the original bounded implementation and namespace-inventory correction. The latest opaque-body remediation is separately authored. This disclosure limits the claim of implementation independence, but does not replace inspection of the complete final source.

## Defects

### D1, unknown owned-namespace attributes on the removed end property owner are not protected

`crates/rdocx/src/field.rs:9499`, `crates/rdocx/src/field.rs:9500`, `crates/rdocx/src/field.rs:9503`, `crates/rdocx/src/field.rs:9270`, `crates/rdocx/src/field.rs:9285`, `crates/rdocx/src/field.rs:9319`.

Outside a tab stop, the scanner treats a property child's attribute as opaque only when it is outside the Word namespace. An unknown Word-qualified attribute on a recognized child such as pStyle is therefore not protected. During a two-entry-to-one-entry collapse, the last root has no paragraph attributes and `has_opaque_properties` remains false. The first entry's properties are retained, while the original end pPr disappears without any copy of its unknown attribute.

Use the existing two-entry, one-heading fixture, with only the last pPr changed to:

```xml
<w:pPr><w:pStyle w:val="TOC1" w:producer='&#65;'/></w:pPr>
```

The qualified val is a generated entry fact. The producer attribute and its lexical entity are unmodeled imported XML. This is the same preservation distinction already enforced for unknown Word-qualified tab attributes at `crates/rdocx/src/field.rs:9503`. Recognizing the parent as pStyle does not make every attribute on it an owned replacement fact. The approved exact preservation requirement at `.claude/plans/F-X213-design.md:43` requires preserving the opaque attribute or retaining its protected root under the existing safe topology. Foreign attributes and active property-body tokens already take that path.

The source scanner and splice establish this finding. No runtime reproduction ran. The new body-token matrix at `crates/rdocx/tests/regression_test.rs:398814` cannot distinguish this attribute-only input. A control on the end owner must cover unknown qualified attributes as well as foreign attributes and body tokens.

## Previous finding resolved

Working pass6 D1 is resolved. `crates/rdocx/src/field.rs:9648` recognizes body tokens only while a property root remains active. Comment, PI, CDATA and entity references are opaque. Non-whitespace text is opaque, while ordinary XML indentation remains mergeable at `crates/rdocx/src/field.rs:9656`. The result feeds the existing protected-end-root decision at `crates/rdocx/src/field.rs:9266`, preserving that root without discarding its body. The existing unsafe tab-stop guard at `crates/rdocx/src/field.rs:9665` still blocks replacing any stop body payload, including whitespace.

The five-case regression at `crates/rdocx/tests/regression_test.rs:398814` checks exact payload identity, separate end-root topology, field markers, repeat rebuild and typed parse/save/reopen. The positive indentation control at `crates/rdocx/tests/regression_test.rs:398852` proves ordinary whitespace still permits a single generated entry. The prior two tab selectors and original 15 native controls remain intact. Single unsafe tab overlays retain the named atomic refusal. No assertion was weakened.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Six aspects

- Correctness: D1. No additional count, marker, adjacent-field, single/empty rebuild or namespace-closure finding.
- Contract: D1 violates opaque source preservation. No additional scope or behavior finding.
- Panics: 0 findings. Scanner depth, source ranges, generated entry slicing and property-root ownership retain their checked construction paths.
- OOXML: D1. No additional namespace, child-order or lexical-body finding.
- Tests: 0 separate findings. Current controls discriminate both prior repairs and the normal path. D1 needs an attribute-only removed-owner control.
- Structure: 0 findings. Existing concrete carriers and helpers remain sufficient. No public API, trait, generic, dependency, module or product file was introduced.

## Limits

This review ran no Cargo, test, formatter, product import, installation, environment, target or native UI action. ROOT's live verification and remaining scoped gates receive no acceptance credit here. The original genuine Base and failed verification attempts remain immutable. Only this canonical review file was written, and this review phase ends here.
