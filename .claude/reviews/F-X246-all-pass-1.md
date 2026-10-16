# F-X246, all aspects, pass 1

**Reviewed**: frozen working diff against046a5fc87409f939a248606ce7435a0dfa83058e,14 existing files,1555 inserted and44 removed lines. Approved plan and cited ownership, rendering, packaging and preservation contracts read first. No implementation or documentation edits were made during this audit.
**Verdict**: 4 defects,0 smells,0 nitpicks

## Defects

### D1, font-matched legend geometry and paint fail the declared native gate

`crates/oxml-chart/src/lib.rs:3175`
`crates/oxml-chart/src/lib.rs:3110`

Keys remain7points and rows15points. The independently exported9point Aptos control has approximately4.943point keys and16.875point rows. Key and accumulated row errors exceed the unchanged1point component bound. Actual150DPI translation-only legend crops produce SSIM0.95597718845 for Word and0.98024928466 for PowerPoint, below0.99. The real failed comparison is retained in `/private/tmp/F-X246-native-inputs/fontmatched-paint-red.log`. Qualify corrected geometry and paint against the frozen native controls before completion. The exact reporter's empty-legend Word overlap and original font mismatches remain separate diagnostics.

### D2, opaque descendants can grant a Word chart owner

`crates/rdocx/src/document.rs:36204`
`crates/rdocx/src/document.rs:25722`

Ownership checks only a qualified chart's immediate graphicData parent. It does not prove the supported paragraph/run/drawing/inline-or-anchor/graphic path or exclude opaque ancestors. In the installed public candidate, wrapping the original w:drawing in an unknown u:opaque child leaves a getter returning visible/right/false and permits a position mutation that changes the package. This is an unknown owner under the approved contract. Require the actual supported drawing owner and preserve opaque payloads. A valid owned chart beside an opaque lookalike also needs its own control.

### D3, shared Word chart isolation rewrites opaque relationship owners

`crates/rdocx/src/document.rs:25842`

The generic paragraph-wide relationship remapper rewrites every matching relationship attribute, including an unmodelled foreign element. A selected real chart and a foreign chart marker sharing rId1, with a second paragraph forcing copy-on-write, change that foreign marker to rId2. The actual installed failure and before/after XML are retained in `/private/tmp/F-X246-foreign-owner-red.log` and `/private/tmp/F-X246-native-inputs/foreign-owner-red.json`. Retarget only the selected qualified chart attribute. Preserve the opaque marker and its original graph ownership exactly.

### D4, the new legend entrypoints accept and publish malformed XML documents

`crates/oxml-chart/src/lib.rs:7483`

Counting one root element does not validate document-level text. The reused source-node scan ignores non-whitespace text outside that root. Appending NOT_XML_DOCUMENT_CONTENT after the chartSpace closing tag in an actual package still permits both the installed getter and a position setter. The setter changes the package and retains the malformed trailer. Reject malformed document-level content before admitting or publishing an owned chart edit, while preserving valid opaque content within its proper owner.

## Smells

None found.

## Nitpicks

None found.

## Not found

- Panics: inspected new indexing, known scalar-name expect, source spans and finite geometry admission. No additional actionable panic found.
- Structure: concrete position/settings and private legend layout values stay in existing files. No new module, dependency, trait, generic abstraction or forwarding layer was introduced.
- Bindings: bool validation, keyword-only Word inputs, stale/foreign/related owner refusals, hidden-state no-ops and staged publication follow existing mechanisms. No additional binding defect found beyond the Word ownership defects above.
- OOXML: expanded-name scalar projection, local shadowing, schema-ranked insertion and exact effective no-op bytes have focused controls. No additional finding beyond D2 through D4.
- Tests: native and installed lifecycle controls establish real new behavior. Pending native qualification is not called passed. Full scoped acceptance and metadata/HLD preparation remain separate obligations.

The review pass ends here. Return to sprint orchestration for a distinct
remediation phase, then a fresh independent numbered pass.
