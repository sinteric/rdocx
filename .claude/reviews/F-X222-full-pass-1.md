# F-X222 full pass 1

**Reviewed:** Working diff against completed F204 prefix `36cfafaa5999709a4899f74ad4ccef773981a876`, preserving original claim `f76bce4c654f862af26029416aa200e4182b1cc0`. Twelve tracked carriers, 1,017 insertions and 75 deletions, plus the existing bounded native review. Inherited F204 provider changes are excluded.

**Verdict:** Zero defects, zero smells, zero nitpicks. This is the feature microscope, not integrated sprint verification or release approval.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Implementation and contract

`crates/rdocx/src/document.rs:6593` retains one checked span list per physical owner. Namespace scopes are batched, each owner/control is scanned from its namespace-complete excerpt, and mapped coordinates use checked arithmetic and physical-source bounds at `crates/rdocx/src/document.rs:10760`. Empty qualified owners cannot scan into a following owner. Foreign scopes remain excluded. The source inventory at `crates/rdocx/src/document.rs:19072` preserves existing story identities, source order and de-duplication. Text-box cache projection at `crates/rdocx/src/document.rs:18804` shares physical source/span facts and selects the same narrowest enclosing owner.

Rich-comment namespace replay and existing picture/table mutation callers retain their original physical-location checks. Namespace declarations and opaque source are not flattened. `crates/rdocx/src/comments.rs:658` delegates range pairing to the private concrete paragraph helper. `crates/rdocx/src/field.rs:13225` reuses one inventory for pairing and bookmark-name counts. Pairing failure does not suppress independent name counting. No global cache, new public identity, revision axis, trait, generic, dependency or product file is introduced.

The existing inline reference at `crates/rdocx/src/document.rs:35463` retains the prior successful algorithm. Its all-story source distinguishes nested cells/controls, related owners, text boxes, namespace aliases/rebindings, fields/bookmarks and comment replay. Added counters and evaluator control at `crates/rdocx/src/field.rs:16905` distinguish repeated acquisition. Bounds/foreign/empty-owner controls start at `crates/rdocx/src/document.rs:35235`.

## Actual qualification

Authenticated retained row logs against recorded SHA-256 values and terminal cleanup receipts. Genuine native and installed Base failures remain retained. Compiler/setup mistakes and the old CLI invocation remain unqualified. Corrected native controls and exact old/new location/XML/cache-coordinate equivalence passed. Current all-feature library and integration results are retained, with four original LibreOffice failures preserved and four exact unattended successors passing. Word regression executed 2,396 passes with seven existing ignores. No new tests were executed during review.

Independently rehashed all 18 Base/result input/output packages. Every corresponding package is byte-identical. Independently recomputed all 15 warm medians from actual release logs, excluding sample zero. Recorded speedups match those medians, ranging from 23.29 to 365.46. Source inventory and field doubling are approximately proportional in measured cases. Whole TOC costs remain separate, including the scale-two-to-four ratio near 2.51. Fixed-input one-versus-two TOC costs have a 1.38 median ratio. No global linearity claim is made for remaining per-bookmark REF lookups, enclosure searches or single-location scans.

The public control at `crates/rdocx-py/tests/test_core.py:5594` uses an alias-qualified empty comment with opaque attributes, verifies records and held handles, runs no-op field/page/TOC updates, and checks exact bytes and reopened comments. Its source-built wheel evidence authenticates complete members, RECORD, architecture, Python source and origins. The corrected installed successor executed one public control plus strict mypy and stubtest, with complete Word/PPT distribution guards and unchanged PPT distribution. Earlier collection-path failure is not runtime credit. Fresh source-built CLI successors exercised fields and TOC and compared saved package members exactly.

Scoped format, all-target/all-feature Word Clippy, both WASM consumers and deterministic hash49 rows passed. Original policy setup failures remain retained. Final successor executed all 141 policy tests successfully with two existing skips. Source inventories show no production source change after current native/public qualification, only recorded test, plan, HLD and archive-carrier updates where applicable. Final archive source inventory matches all current tracked source exactly.

## Archive and documentation

Authenticated final verified dry-run archive SHA `739cf291ff6bba1ec14b7a402c7cad28f1d543a75cf3a794da40836f3b12b042` and all 37 member payload hashes. Origins distinguish owned source from Cargo-generated manifest, lock and VCS metadata. Packaging used local patches, did not upload and did not bypass verification. Final gzip size is 3,656,660 bytes, one byte above the recorded first observation. Normalized member bytes are exactly 51,495,139 and member count exactly 37, within existing measurement tolerance and archive ceiling. `README.md:42` and `scripts/readme_doctests.py:405` record the measured first observation rather than donor values.

The five approved HLD carriers describe operation-scoped ownership, empty-comment preservation, unchanged public identities and bounded performance claims. No bibliography/navigation expansion or sprint ledger edits are included. Remaining checklist, prepare and integration actions belong to the orchestrator and are not completed by this review.

## Not found

Correctness, contract, panics, OOXML, tests and structure produced zero defects and zero smells. No unmodelled source loss, namespace ownership regression, editable-coordinate substitution, unexplained successful-output delta, hidden public API expansion or unsupported performance claim was found.

## Review integrity

The first full-file inventory was accidentally printed beyond the tool output limit. Its truncated result is not credited as a complete guard. Corrected compact guard captures branch, HEAD, physical index, logical index, tracked binary diff and a SHA-256 aggregate over every tracked path/file hash. After that correction the native decision paths were re-read and complete guard equality is asserted after report publication. All 3,203 tracked source hashes aggregate to `a45c08729fdb68bf102d2a642e63452c9b9d8c8402f6414f4eeb42e654af8c67`. Binary diff SHA remains `70c89b8e6623d2e53bd647712e90835c83e142fc93cb2076b61b6d5d5b7ab0cb`, physical index SHA remains `1b560f19c60ceedacbcdf5bb37e4cdd6dedf600b6c3c765040e1a699b4bff1a2`.

Only this review file was written. No source, plan, progress, HLD, test, formatting, Cargo, CP or Office mutation occurred. This pass ends here and returns the F222 writer lease to ROOT.
