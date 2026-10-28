# F-290, whole-feature, pass 2

**Reviewed**: Working diff against d1f42516eb3747442fe4304a6331eab946dc9a0e, 17 tracked files, 7610 insertions and 27 deletions. Frozen diff SHA-256 ed573e9277e7224fae8accab95d2f1018ac0f3f14546c1e8ebe7ccfa62007669.
**Reviewer**: Codex root. Grok performed mechanical probes and transcription only.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, settings-owner recognition still omits modeled configurations

`crates/rdocx/src/field.rs:21060`
`crates/rdocx/src/field.rs:4503`
`crates/rdocx/src/field.rs:4249`

The new owner guard recognizes only five byte needles. A qualified mailMerge containing dataType native and odso/src, without mainDocumentType, dataSource, fieldMapData, recipientData or the owned document variable, is recognized with an internal settings relationship. Changing only that main settings relationship to External makes query return None and prepare report absence. A dataType-only subtree similarly changes from the required missing-source error to absence. Invalid ownership must not hide recognized configuration. This is the same D1 invariant surviving its first repair on different inputs.

Grok ran four CRC-correct in-memory ZIP cases through the public Python API without editing or rebuilding. The imported extension SHA-256 was 39c634557d8dd598f283b66f7701e18896ce32920e1ea543a99795e03c3e28a5, size 123213416. The qualified src relationship id was rId0. Internal source control queried an external source at file:///declared/owner.csv and prepared external_source_blocked. Its External-owner counterpart queried None and prepared mail-merge package is absent. Internal dataType-only control raised mail-merge package source is missing on query and prepare. Its External-owner counterpart queried None and prepared absence.

Reassess the shared ownership/presence invariant across all thirteen modeled settings and qualified graph metadata, including event variants and namespaces. Adding another isolated byte needle does not establish closure. Preserve opaque-only absence and unrelated metadata, and keep valid positive controls.

## Smells

None.

## Nitpicks

None.

## Prior findings and evidence

D2, D3, D4, D5, D6, S1 and S2 from pass 1 are resolved in the inspected source, fixtures and HLD changes. Other D1 graph cases are repaired, but D1 remains open as above. The XML MIME case-sensitivity suspicion is refuted by content_type_is_xml and is not a finding.

The latest scoped native six groups and Python four groups, rebuild identity, fmt, three-crate clippy, typing smoke and two metadata policy tests are local evidence only. Installed strict mypy, stubtest, final packaging, WASM, hash and the full integrated union remain pending. No native Word execution or full-sprint acceptance is inferred.

## Not found

No additional findings in contract, panics, OOXML preservation, tests or structure beyond D1. Binding exports, frozen carriers, URI and recipient parsing, owned-store refusal, Word canonical slots, HLD scope and local publication behavior were examined. This pass ends with this report and makes no implementation changes.
