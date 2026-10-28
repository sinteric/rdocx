# F-289, whole feature, pass 1

**Reviewed**: Codex root independently reviewed the working implementation against the approved form and Python contracts and the five drafted HLD files. Worker Base is c0b58c7fa8e4e919b9bfe5c02932f74827cea36b. The tracked working diff is 17 files, 7595 insertions and 77 deletions. Earlier native passes remain bounded historical reviews.
**Verdict**: 3 defects, 0 smells, 0 nitpicks

## Defects

### D1, owned display projection and replacement omit supported visible content

`crates/rdocx/src/content_control.rs:4528`, `crates/rdocx/src/content_control.rs:4569`, `crates/rdocx/src/content_control.rs:5812`, `crates/rdocx/src/field.rs:1822`

The high-level form reader and rewriter enumerate only Word text elements. Their ancestor test delegates to the drawing-discovery grammar, which explicitly excludes fldSimple. Existing native run text includes tabs, breaks and projected field results at `crates/rdocx-oxml/src/text.rs:3340`. A valid rich control containing A, a tab and B is filled with X while its tab remains. A control whose visible text is a simple field cache is instead read as empty, and replacement inserts new text beside that retained cache. Validation, bound value and rendered text can disagree. Read and replace the accepted owned semantic display while retaining field instructions, source metadata, markers and independent nested controls and text boxes. Preserve the old body-only API behavior. Add distinct tab, break and simple-field cases with actual RED provenance and native, reopened and rendered assertions. This is a source finding, not a claim that those new cases have run.

### D2, protection removal allocates its Python result after publication

`crates/rdocx-py/src/document.rs:10948`, `crates/rdocx-py/src/document.rs:10953`, `crates/rdocx-py/src/document.rs:10961`

The snapshot is a Rust PyDocumentProtection value. The return type leaves PyO3 to allocate its Python object after the method has published the candidate. An allocation failure can therefore report failure after removing protection. The approved contract explicitly requires every result PyObject to be allocated before publication. Allocate the frozen Python object before commit and return that allocated object, keeping the public return type and owned old-value semantics. A normal old-value runtime test does not prove allocation-failure atomicity. Use an existing fault mechanism if available, otherwise record the source counterfactual honestly without inventing a semantic RED.

### D3, HLD publication descriptions contradict the read-only operation and retained epochs

`docs/hld/10-bindings-spec.md:4308`, `docs/hld/12-testing-strategy.md:251`

The bindings specification lists validate_form among operations publishing ModernFormValues, although `crates/rdocx-py/src/document.rs:10842` only reads and returns issues. The test taxonomy retains the old blanket rule that run and global snapshot epochs retire on actual form mutation, despite the new modern publication preserving proved runs and exact snapshots. Describe the read-only operation separately and reconcile the taxonomy with the modern versus existing publication policies. Do not change existing binding, legacy setter or signing behavior.

## Smells

Zero found in the examined source and contract paths.

## Nitpicks

Zero.

## Coverage and limits

Examined the new carrier factories, sequence and numeric conversion, exports and stubs, four form calls, protection and permission composition, result preparation, modern related-part certificates, local epoch commit, physical display ownership, current-value validation and reset, and the five HLD deltas. Consumed the recorded native 87 and Python 13 results after inspecting their public dense profile and relevant owner tests. Those results do not cover D1's new representations or prove D2 allocation ordering. The earlier clean native pass is not a whole-feature verdict on this later diff. No additional panic, structure or namespace defect was found in the examined paths. No new trait, generic, crate or dependency was introduced.

Word 16.114 A7 and the final integrated hash, full suite, WASM, publish dry-run, strict installed mypy and stubtest remain unrun. Local deterministic PDF/A diagnostics do not replace Word qualification. DOCX-056's bounded capability row does not imply native parity or feature completion. The revised canonical HLD coverage is acknowledged by the worker. This review changes no product or planning file and returns control to run-sprint for separate remediation.
