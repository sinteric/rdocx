# F-X268 final performance remediation, pass 1

**Reviewer:** Independent Codex agent `/root/review_source_pieces`.
**Scope:** Frozen canonical working diff against `67e161046bef0d3e3d22b40ab93a548259866118`, only `crates/rdocx/src/document.rs`, 485 added and 30 removed lines. Scope includes the namespace refusal probe, producer read bootstrap, scoped normalization memo and direct-cell routing changes and their four new tests.
**Source SHA-256:** `df5b079b77215295f14803a2dc29e7f967644e9c38c25dd58e2e2c432814c888`.
**Method:** Read-only source and existing evidence inspection under `CLAUDE.md`, `.claude/WORKFLOW.md`, `/microscope`, the approved F-X268 design and cited architecture/packaging intent. No builds, tests, source edits or delegation.
**Verdict:** One contract/admission defect. Zero additional correctness, panic, structural-smell or nitpick findings established. The repair is not yet a clean microscope result. The original required timing failures remain failed pending actual affected measurement.

## D1. Producer bootstrap does not enforce its default-namespace exclusion

The new producer bootstrap checks root declarations with `prefix.is_empty()` to decline default namespaces (`crates/rdocx/src/document.rs:23747`). Those strings are declaration names, not prefixes. `apply_namespace_declarations` stores the actual names `xmlns` and `xmlns:w` (`crates/rdocx/src/document.rs:2812`, `crates/rdocx/src/document.rs:2823`), and `document_namespace_scopes` returns that representation (`crates/rdocx/src/document.rs:2630`). Neither name is empty. The root default-declaration guard therefore cannot select its intended fallback.

A source with canonical `w:document` and `w:body`, a producer-normalizing XML declaration and a root `xmlns="urn:producer"` declaration can satisfy the literal QName checks while bypassing this exclusion. Normalized empty-frame equality does not substitute for the explicit unsupported-context policy: the typed frame can preserve that same declaration. The approved mechanism 7 contract retains checked whole reads for alternate/default namespace context rather than expanding admission merely because normalization agrees (`.claude/plans/F-X268-design.md:84`). The investigated repair likewise explicitly retained default-shadow fallback.

This finding identifies a concrete guard and contract defect. It does not infer corrupt output from successful admission. The subsequent actual source reads and normalized comparisons remain meaningful protections.

**Remediation:** Compare the declaration name with `xmlns`, or apply the existing `namespace_prefix` conversion before testing the empty prefix (`crates/rdocx/src/document.rs:2930`). Add a source-only producer default-declaration discriminator that reaches this bootstrap and requires it to decline, while the whole parser and original operation retain their expected bytes/error behavior. Keep canonical `w` producer header normalization as the positive case. Do not alter a valid default-shadow fixture to avoid the fallback requirement.

## Remaining implementation assessment

### Actual producer reads remain distinct from writer proof

The opening now retains one optional producer-source record (`crates/rdocx/src/document.rs:7209`). Its bootstrap uses physical direct-child ranges, checks the parsed empty frame against the canonical frame, rejects non-whitespace uncovered content and unsupported child kinds, performs actual source-child reads, and explicitly clears `staged` (`crates/rdocx/src/document.rs:23712`, `crates/rdocx/src/document.rs:23758`). Field-containing and private-fact-ineligible children decline. It verifies the complete partition and normalizes each paired child before retaining the source record. It does not seed an actual source AST using normalized writer input.

`source_blocks_in` remains responsible for exact retained-part identity, changed ranges, actual changed-child parsing, resource validation and final partition agreement. The existing source-read versus writer-eligibility control remains in the affected execution. No additional writer-provenance defect was established beyond D1's admission boundary.

### Scoped normalization memo

The memo keys correspondence to the source and canonical child bytes plus exact source head/tail, canonical head/tail and typed writer frame (`crates/rdocx/src/document.rs:9479`, `crates/rdocx/src/document.rs:9514`). Its cold path still performs the two actual normalization reads and compares both models and written bytes. Marker absence is checked against the stored normalized bytes on a hit. Cache population excludes fields and private-fact-ineligible normalized models (`crates/rdocx/src/document.rs:9529`). It supplies neither a source AST nor writer eligibility.

Fresh-context pieces clear the new lock together with the other context-sensitive proofs (`crates/rdocx/src/document.rs:8447`). Changed source pieces are newly created rather than editing a proved piece's bytes in place. Storage is a single `OnceLock` per piece, not an appended per-edit result list. No explicit predecessor-opening chain was introduced. These source properties are not a measured memory result: the stored canonical block reference, normalized bytes and context add retention and must be included in the pending Q18/Q22 qualification.

### Namespace refusal and empty-paragraph probe

The unsafe-root check still precedes the fast refusal computation. Retained nested-owner facts require exact package identity and explicit root/body dependency agreement (`crates/rdocx/src/document.rs:32545`, `crates/rdocx/src/document.rs:32573`). Failed or unavailable facts retain legacy computation. An empty positive fact can return no namespace refusal without treating arbitrary current-model serialization as proved.

For nonempty facts, the existing unsafe-prefix and replay matching rules remain authoritative. Current canonical bytes receive their own proved span list. The insertion probe assembles the current canonical blocks and a newly serialized empty paragraph through the existing frame writer, preserving final-section placement and conditional root declarations (`crates/rdocx/src/document.rs:32595`). Probe failure declines to legacy computation. Current replay errors retain their diagnostic, while the insertion-probe result keeps the existing namespace-error classification. No additional error-order or probe-byte equivalence defect was established in this scope.

### Direct versus controlled table coordinates

Initial modeled table/row/cell validation remains before physical qualification. Direct tables use `content_index_of_table`, `direct_body_item_location` and checked `story_table_cell` (`crates/rdocx/src/document.rs:41710`). Any unproved route falls back to the old pointer-correspondence search. Thus tables included in the public table ordinal through controlled content are not mistaken for direct body slots. The new paired controlled/direct table test checks which cell changes and verifies refusal leaves bytes unchanged (`crates/rdocx/src/document.rs:48511`). No additional coordinate-selection or atomicity defect was established.

## Evidence and limits

- `/private/tmp/s954-final-performance-red.log` records two actual initial failing native controls. `/private/tmp/s954-final-performance-green-1.log` records their initial successful repair execution. These are distinct outcomes, not compile failures relabelled as semantic RED.
- `/private/tmp/s954-final-performance-native-batch.log` completed with 771 passed, two failed and 13 ignored in 79.28 seconds. All four `final_gate_*` tests passed. The two failures are tool-identity assertions: observed `pdftotext` 26.09.0 versus pinned 26.01.0 and the chart rasterizer identity. The full batch is not described as green.
- `/private/tmp/s954-final-performance-oracle-retry.log` records both affected tests passing with qualified tools using the same already-built binary, with no source or build change.
- `/private/tmp/s954-final-performance-clippy.log` records all-target/all-feature scoped rdocx clippy success in 35.32 seconds. The earlier all-target check predates the final tests and cache guard and is not substituted for the later applicable clippy execution.
- The new positive controls exercise actual source AST agreement without writer eligibility, repeated distinct producer edits, whole-path package/serialization agreement, equal counted work at 25/100, namespace preflight work and direct/controlled cell selection. They do not contain the missing root-default rejection assertion identified in D1.

The pinned release run at `/private/tmp/s954-word-scaling/summary.json` still records 15 required flat-cost misses. No new timing pass is claimed. Memory qualification must include both new caches and remains pending. No native Word execution permission has been supplied. Missing external PowerPoint resources and all other final riders retain their pending status. No final feature, sprint or release acceptance is claimed.
