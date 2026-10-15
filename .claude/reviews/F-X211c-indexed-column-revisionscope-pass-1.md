# F-X211c indexed column RevisionScope correction, pass 1

**Reviewed**: The sole indexed Column capture change from immutable f3 to corrected f6, including the actual enum signature and existing prepared-column use. All3366 live and captured source bodies, sizes and modes were authenticated. No execution or mutation outside this report.
**Verdict**: Zero defects, zero smells and zero nitpicks in the corrected source.

## Exact scope and type

Source map SHA256 `383416c1cc7d97f64a974120055a950dfb9a2ab36229670923a57eccaa228afa` and capture receipt `b54bd67e5282ec1ff55e6f2cf7956b9b51fb79f678b33cd1cc152b0763e4137a` were independently rehashed. Exactly one tracked carrier differs from f3 source6079. Its complete diff replaces the ColumnCollection.item None argument with the formatted Some(oxml_py_support::RevisionScope::Grid) expression. Tests, collection ancestor captures and all other source bytes remain unchanged.

At `/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/src/table.rs:424`, the indexed Column now captures the actual public RevisionScope::Grid enum. This matches the capture_path signature at `crates/rpptx-py/src/presentation.rs:2916` and the existing prepared add_column capture at `crates/rpptx-py/src/table.rs:496`. At `crates/rpptx-py/src/presentation.rs:3104`, that scope adds the physical table grid token. The earlier EpochKind expression did not name the required type and was not compilable. Prior static report af3 missed that defect. It supplies no compilation or executable-source credit and is superseded on this point by this correction and its actual frontend evidence.

The indexed item continues to feed integer indexing, slicing and iterator members. Reads, setters and removals retain existing captured-path validation and typed first-cause refusal. The collection ancestor captures remain None, while indexed Row/Cell and prepared added Column Grid captures remain intact. No publication, invalid/foreign argument handling, no-op or failed-operation policy changed. The prior bounded semantic review of those unchanged bodies remains applicable.

## Actual frontend evidence

Independently rehashed `/private/tmp/S91-F-X211c-indexed-column-revisionscope-remediation-1/actual-f6-binding-stage-1/receipt.json` SHA256 `e3f8fc843e0058fbff184a50aaa0850f1581dcfbe5c46a1adcb316c3dc3de15c`. Its three named current PPT binding all-target/all-feature check, strict Clippy and MSRV1.93 rows are actual exit0, qualified and safely terminal. Each retained raw log matches its recorded hash and reports a completed frontend check. This is genuine compilation evidence for the corrected enum, not inference from the earlier source review. Root separately reports terminal formatting success, which is not relabelled as reviewer execution here.

## Findings and limits

Defects: zero. Smells: zero. Nitpicks: zero. Type correctness, literal Grid semantics, child versus collection authority and exact source scope produced no findings.

Original f4/f5 compile failures and the failed f3 installed axis controls remain retained failures. The actual20 fresh wheel controls, typing and any new corpus attempt remain pending. This review ran no builds, tests, CP probes or corpus cases and assigns no acceptance or score. It ends here and releases the canonical review-write lease.
