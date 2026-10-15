# F-X211d, completion, pass 1

**Scope**: Root's four approved HLD impact edits. Metadata successors and the final policy run are not yet included. Read-only source/contract comparison, without rerunning green tests.

**Verdict**: 2 documentation defects, 0 smells, 0 nitpicks. Return for separate HLD remediation before the final completion pass.

## Defects

### D1. HLD overstates lexical preservation of root attributes

`docs/hld/04-opc-and-packaging.md:1898` says original opaque root attributes and adjacent foreign children are preserved exactly, then line1900 says subsequent cycles retain those bytes. The adjacent retained opaque child-byte control is qualified, but this wording also promises universal lexical byte identity for original root attribute markup.

The existing root-record provider at `crates/rdocx-oxml/src/text.rs:463` decodes and normalizes attribute values. It constructs a new attribute record at line505 and pushes reconstructed attributes at line507. Original single versus double quote spelling and equivalent entity spellings are not retained by that path. The scalar/root-stamp controls establish their actual selected metadata and exact adjacent opaque payload behavior, not general lexical root-attribute identity.

Describe root metadata preservation separately from the exact retained foreign-child bytes and keep the actual namespace-closure and qualified stamp controls explicit. Do not claim general original root-markup byte fidelity from these tests or weaken the independently qualified adjacent-child byte assertions. No new product remediation or changed acceptance is asserted by this documentation finding.

### D2. HLD names a style identifier the implementation does not accept

`docs/hld/10-bindings-spec.md:3831` names LightShadingAccent1. The lazy built-in provider at `crates/rdocx/src/document.rs:28371` accepts exactly the ID LightShading-Accent1 and display name Light Shading Accent 1. The binding resolves their actual canonical style definition. The hyphenless spelling is not a supported alias and should not be presented as the current style name or ID.

Use the exact supported ID and display name. Preserve the existing custom-definition retention, unknown-style refusal, lazy materialization and unchanged factory claims.

## Smells

None.

## Nitpicks

None.

## Bounded observations and pending completion

The four HLD paths match the approved impact list at `.claude/plans/F-X211d-design.md:85`. Header/paragraph survival, checked merge provenance and atomic opaque-property refusal are expressed as scoped feature behavior. `docs/hld/12-testing-strategy.md:4192` explicitly retains the final immutable100 and integrated sprint gates rather than claiming the scoped controls replace them.

All22 original current-source verified package receipts were already inspected independently during execution. They remain exact source-qualified originals, with root admission under `/private/tmp/S91-F-X211d-archive-preparation-1/root-22-archive-admission.json`. Metadata mutation successors, README/catalog consistency and actual final policy141 outcome remain a separate forthcoming completion pass. This report supplies no fresh wheel/current-source claim for an amended metadata source, no native application acceptance and no frozen100 completion credit.

No source, HLD, README, worker metadata, runtime, CP environment, test, dependency or Git state was changed. Only this canonical review file was written under the exclusive review-file lease. Review ends and returns control for separate remediation.
