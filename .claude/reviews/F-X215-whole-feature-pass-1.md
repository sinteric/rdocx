# F-X215 whole-feature implementation review pass 1

**Reviewed:** Frozen working implementation on `work/f-x215-codex`, HEAD `38d1697b1d56bf97f7eabc4c02eb6996acf0ae3d`. Twenty-one tracked carriers, 4,376 insertions and 26 deletions. This includes the previously unreviewed native chart/workbook and PPT phase, Word paragraph-relative creation, both Python surfaces, tests, stubs and six HLD impact files.

**Verdict:** One defect, zero smells, zero nitpicks. Full feature acceptance remains pending.

## Defects

### D1, new workbook cells use an ancestor prefix that can be rebound on their destination row

`crates/oxml-sml/src/lib.rs:864`
`crates/oxml-sml/src/lib.rs:955`

The worksheet merger derives the emitted cell prefix from sheetData once, then uses it when inserting new cells inside an existing qualified row. A row can legally use another SpreadsheetML prefix and locally rebind the ancestor prefix. For example, let the root bind both s and t to the SpreadsheetML namespace, use s:sheetData and use t:row with a local xmlns:s="urn:foreign". Its existing t:c cells are valid SpreadsheetML. A more-series replacement choosing an unused column inserts s:c into that row. Those cells and generated values are in urn:foreign, so the saved workbook does not contain the requested SpreadsheetML chart values.

The final `chart_xml_nodes` call at `crates/oxml-sml/src/lib.rs:1004` checks the qualified worksheet root and XML balance, not that every emitted target cell resolves to SpreadsheetML. Consequently this path can succeed and publish caches that disagree with the editable workbook. This violates the approved imported namespace-alias and cache/formula/workbook consistency contract.

Emit new nodes with a namespace guaranteed in their actual insertion context, using the destination owner bindings or explicit namespace closure. Preserve unrelated row declarations and bytes. Add a deciding imported-workbook control with an aliased row rebinding the sheetData prefix, add a series, and assert actual qualified worksheet values against every updated formula after save/reopen. A structural substring or root-only parse does not prove this property. No runtime reproduction or fix is credited by this review.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Aspects checked

- Correctness: D1. Zero additional findings in chart cache projection, ordinary formula parsing, bounded range edits, surviving series identity, no-op, current frame resolution or shared chart/workbook isolation.
- Contract: D1 violates qualified imported-workbook preservation and consistent editable values. Zero additional implementation findings in seven chart kinds, explicit shared-X restriction, checked Word after insertion and both public chart surfaces.
- Panics: zero findings in the reviewed change. Cache count is bounded and cache indices are checked by existing layout validation. Formula/address arithmetic is checked. Internal unwraps examined follow validated owner, root or nonempty-series conditions. Existing PPT constructor stack failure remains separately recorded rather than attributed to this implementation.
- OOXML: D1. Zero additional findings in source-spliced relationship targets, foreign attributes, chart child order, stable series formatting, unrelated workbook entries or qualified frame relationship selection.
- Tests: D1 needs its namespace-context control. Zero additional findings in actual installed membership, native/corpus lifecycle, shared-owner refusal, exact no-op and real returned-type allocation failure evidence.
- Structure: zero findings. Concrete code stays in approved existing carriers, with no new module, file, dependency, trait or generic. The workbook merger remains limited to chart-owned formula ranges.

## Native and public paths audited

`crates/oxml-chart/src/lib.rs:294` reads one supported plot from actual typed caches, preserving sparse missing values and refusing incompatible category vectors. Replacement at `crates/oxml-chart/src/lib.rs:627` checks data/formula boundaries, retains surviving ordinal idx/order and formatting, adds default series and source-splices only owned references and removed/added series. Workbook updates reject populated unowned collisions and preserve unrelated entries. Numeric formats append existing style definitions rather than replacing original styles.

`crates/rpptx/src/lib.rs:8224` resolves a uniquely identified actual chart frame, including group ownership. Relationship kind, uniqueness, content types and internal targets are checked. Replacement at `crates/rpptx/src/lib.rs:8316` stages chart/workbook changes and isolates shared slide relationships, chart parts and workbook parts. Source-aware relationship target edits retain aliases and opaque payload. Case-equivalent occupied package names are respected. Candidate publication uses the established owner provider.

Word relative insertion at `crates/rdocx/src/document.rs:23474` resolves the direct body paragraph's physical position, assembles on a candidate and validates before commit. The binding at `crates/rdocx-py/src/document.rs:6387` refuses foreign, retired and unsupported story owners and allocates its returned Paragraph before publication. PPT creation at `crates/rpptx-py/src/shape.rs:1856` prepares and allocates its returned owned Shape before publication. Live Chart replacement uses the established current owner and native transaction. Category/series snapshots and concrete Python data objects remain detached from package mutation. Current exports, signatures, stubs and the six HLD edits describe these mechanisms without claiming completed Office or final gates.

## Actual evidence authenticated

Checkpoint SHA-256 is `2ca06571c784bc7364dd93ccfb771abf008488a835fd488669249ceb72e59c02`. Every named current source hash and complete patch match. The inherited b3034f22 review claim is withdrawn and receives no independent review credit. This is the first genuine review of the native implementation as well as the public phase.

Authenticated actual receipt and raw log hashes for current installed runtime, final typing, returned allocation, pinned oracle, composed-provider native controls, ordinary-stack Word control and strict scoped Clippy. Installed successor receipt `ff033d0cea895712ba1948725595737f02e3f3566ef1c70f208da89c54b0d8da` records the exact three Word and four PPT nodes passing. Final typing receipt `92a586cd1449c7c89163eb7671920b4b53e56749f45a1ace6415bc28181e708e` records both strict mypy and both complete stubtest passes. Setup and prior stub-export failures remain retained.

Independently checked both current wheels, all members, RECORD digests and lengths, current Python/stub bytes and equality of the archived native extensions with retained built targets. Word wheel SHA-256 is `dda00e5bfa8aca0df3a69eba7daee42c7fb533331f2b0f73fee1c3ad68f03fb5`, twelve members. PPT wheel SHA-256 is `b89259f50fb2b364a03dc067b5dc5fcbc739142a35efcfdfce1595d34d96577b`, fifteen members. These are isolated fresh-wheel results, not a managed shared CP publication claim.

Inspected the process-isolated C allocator and Python control. The helper temporarily replaces only the requested returned Paragraph or Shape type's tp_alloc, calls the real authoring method, restores the allocator and returns its actual MemoryError. Receipt `16941e13fe5bf13d04190da591c23b081728c43ee145644a5d4d1947992e3e09` records one observed refusal per family, unchanged complete package and held witness, then successful ordinary authoring after restoration. The causal path proves allocation-before-publication, without a production fault switch or shared interpreter mutation.

Pinned Python oracle receipt `b6d6d84ae0872ab34c43a0931bf1fa24f01fec2d3618894e7f0a631fc1dfab61` records five actual normalized structural rows: both family bar/line outputs and shared-X scatter. python-docx checks surrounding Word ownership, not a nonexistent chart API. Existing authenticated corpus outputs retain actual cache/formula/worksheet checks, but do not resolve D1's different namespace setup.

Current composed-provider native controls pass10 and the Word after-insertion control passes1 on the ordinary2MiB stack, receipt `6cdd1a1fe363d95138373c0dce2e6ca42307934aa1cb57bae594cdb9a54dd967`. PPT native controls retain their explicitly declared8MiB setting and the existing constructor's genuine default-stack failure is not hidden. Strict changed six-crate all-target/all-feature Clippy passes, receipt `344549c7d69e4e7c00b18bce57ead9225ff49b198eddfbe30a8daf66be6d894d`. Earlier shared qualification uses guarded actual evidence, with the documented early callback limitation preserved.

## Remaining gates

D1 needs separate remediation and re-review. Native Word/PowerPoint open, Edit Data, save/reopen and deterministic value-specific renders remain due, including stale-value negatives. Root managed CP, four verified archives/current measurements, current WASM/hash/policy and remaining scoped gates and prepared handoff remain due. This implementation review does not grant completion, sprint closure or release publication.

## Review integrity

Read canonical workflow/review rules, approved plan, referenced HLD, progress and in-progress sprint status. All3,205 tracked file hashes, sizes and modes were compared before and after the review. Canonical sorted-map aggregate remains `fec41122266bdf8f54b730455cc5c881bdc654a84619d65c09ea6e790a6fd3fb`. Full binary diff remains `a21f27b527702c76d691d22424453ca4ba83a0e4c82d98043a927a85c99a3b3e`. Checkpoint indexSHA is the textual git ls-files -s digest, `af79b53d8878c28ad5bdf271aded0e77721de17636314579e39b1e59601e8a3a`. Additional logical NUL-delimited index remains `3e55920e60cde135bb08358e9c9b71be1e035f8e11dfdeb9ad888c635062da75`, physical index `4c543b6dbeaac6addb738ef83a3986b58c2f764b1a504a0006fe395ca896f545`. HEAD and branch remain exact.

Only this review file was written. No source, plan, progress, HLD, test, ledger, runtime, Cargo, Git mutation, CP or UI changes occurred. This review ends and returns control and the writer lease to the orchestrator for separate remediation.
