# F-X215, table header repair, pass 1

**Reviewed**: The frozen change in `crates/oxml-sml/src/lib.rs`, SHA-256 `29b24308983499e98af327ad165525f7495152feabfe573434202f072d2ce234`, against the independently reviewed and archived source `ad638c522bc9872d49d61f37bd02437a4a0965ea7ebe8633e3c7df8fcdd356cd`. One existing carrier changes, 190 additions and two deletions. Worker branch is `work/f-x215-codex`, HEAD `38d1697b1d56bf97f7eabc4c02eb6996acf0ae3d`.

**Verdict**: Zero defects, zero smells, zero nitpicks. This is a bounded implementation review, not feature completion or native production acceptance.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Not found

- Correctness: zero findings. The admission filter at `crates/oxml-sml/src/lib.rs:483` removes only None-valued clear requests inside the actual table header row. Replacement values remain admitted. Removed numeric cells and ordinary non-table headers still clear. Tables with headerRowCount zero retain no header ownership. The existing candidate package is local until successful serialization, so rejected ownership does not publish partial workbook or chart changes.
- Contract: zero findings. The helper at `crates/oxml-sml/src/lib.rs:410` recognizes an additional existing owner of a removed series header. It does not add general table editing, change public signatures, shrink imported tables, rewrite table names, or alter surviving chart-series projection. Preserving the table-owned header is consistent with the approved unrelated workbook preservation contract.
- Panics: zero findings. Root indexing at `crates/oxml-sml/src/lib.rs:459` follows the existing parser's nonempty, qualified, single-root check. Parent indices come from that parser. Cell coordinates use the existing checked address parser. No new unchecked narrowing, arithmetic, production unwrap, or untrusted slicing appears in the repair.
- OOXML: zero findings. Worksheet tablePart recognition at `crates/oxml-sml/src/lib.rs:425` requires SpreadsheetML namespace ownership and the direct tableParts parent under the worksheet root. Relationship identity requires the actual relationships namespace at line437, the exact internal table relationship type at line450, and a qualified table root at line458. Foreign lookalikes do not confer ownership. Table bytes, shared strings, other package entries and relationship XML are not rewritten. Only an admitted source cell splice can change the worksheet.
- Tests: zero findings. The regression at `crates/oxml-sml/src/lib.rs:2474` exercises omitted and explicit headerRowCount one, alias and default namespace forms, headerRowCount zero, a foreign tablePart with no valid relationship, non-table header clearing, numeric data clearing, exact retained header bytes, unrelated cell bytes, every unrelated part, and exact worksheet relationship serialization. Reverting the helper would fail the three positive preservation cases. Actual library results are 13 passed and one preexisting viewer ignore. The selected native corpus test passed once and produced seven source-built candidates.
- Structure: zero findings. One concrete helper in the existing SpreadsheetML carrier separates ownership admission from worksheet splicing. It adds no product file, module, dependency, trait, generic, wrapper, cache, or bindings mutation implementation. The current facade and binding calls continue through the same merge operation.

## Evidence checked

Read the canonical workflow, microscope command, approved F-X215 plan, current sprint and backlog, progress checkpoint, latest whole-feature pass3, and the cited minimal-workbook and OPC preservation contracts. Independently extracted the retained oxml-sml archive source and authenticated its pre-repair hash. Compared it with the frozen current source to establish the exact bounded delta.

Authenticated focused receipt `a715a06dd8d66e753a243dabc9c29e13283242f170c173f57eb598270d615eec` and all four raw log hashes. Actual all-target check, library tests, all-target Clippy and the exact corpus selector succeeded. The receipt records source guards and naturally terminal owned processes. No tests, Cargo processes, Office operations or source changes were performed by this review.

Authenticated header-only native control receipt `893c2d7569e6696e598ae42f80d0c3f25e8aebed6b1b4b7c02192dffaa8c9507`. It records the linked PowerPoint workbook opening without recovery, retained D1 and B/C headers and values, and empty D2 through D5. Independently compared every embedded XLSX member of the source-built fewer candidate `469da06469ca719df2d034f70d0cb2b2227ea975be54b5a408992b4f98aa8982` against that native causal control. All member bytes match. This supports the causal repair but does not substitute for actual source-built native Edit Data, save and normal reopen acceptance. Current affected archive and installed artifact refresh, feature handoff and integrated sprint verification remain the orchestrator's work.
