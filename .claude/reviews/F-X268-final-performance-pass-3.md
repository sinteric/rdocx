# F-X268 final performance remediation, pass 3

**Reviewer:** Independent Codex agent `/root/review_source_pieces`.
**Scope:** Frozen canonical diff against `f486efa1e56f2ade93e683bffbb31a194af07d27`. Three files contain 180 added and five removed lines: `crates/rdocx/src/document.rs` and the architecture and packaging HLD files. Review covers query-binding equivalence, actual file-opening provenance and checked story-hyperlink provenance adoption. Prior clean pass 2 supplies the unchanged producer/source/cache assessment.
**Method:** Read-only source, runtime consumer, HLD and existing execution-log inspection. No implementation, builds, tests or delegation.
**Verdict:** Zero defects, zero smells and zero nitpicks in this bounded common-path batch. The implementation is ready for affected release qualification. The six existing release timing failures remain failed until actually remeasured.

## Source identity

| File | SHA-256 |
|---|---|
| `crates/rdocx/src/document.rs` | `0385bd3cd8b4993451ea33e1fa646fd72cfd4f26f56d667e4b05f005985ee278` |
| `docs/hld/03-architecture.md` | `aa6e61f156868f946beaa053217ed747325f917ace2b59935c5e0ec6069bc2b5` |
| `docs/hld/04-opc-and-packaging.md` | `8988e3f2bbaff784e3bed5e59e64d64d3bfa4cf9a81f4b91fd763432fda5c136` |

## Query bindings versus proof identity

`same_query_namespace_bindings` compares exact name/value pairs with their multiplicity, accepting only their reordered multiset (`crates/rdocx/src/document.rs:8558`). It is used at precisely the two nonempty query-argument eligibility checks in `canonical_child_facts` (`crates/rdocx/src/document.rs:9053`, `crates/rdocx/src/document.rs:9072`). It does not normalize stored source scopes, byte partitions, frame identities or cached proof dependencies.

The authoritative scanner checks inherited relationship declarations using an order-independent `any` predicate on exact declaration name and URI (`crates/rdocx/src/document.rs:6360`). Thus accepting reordered equivalent queries does not change relationship resolution. Changed URIs, omitted entries, extra entries and duplicate substitutions cannot match the canonical frame's unique declaration set. Empty query behavior is unchanged.

The new regression constructs a real canonical record with a header relationship, reverses its binding query and compares the proof result with the authoritative scanner. It also requires changed-value and duplicate-prefix variants to decline (`crates/rdocx/src/document.rs:48571`). This directly discriminates the observed query-order eligibility failure. The previously proposed replayed-source reconstruction is absent from this diff. The existing replay-byte equality guard remains unchanged.

## Actual file-opening provenance

`Document::open` now marks the successfully opened package as read from bytes (`crates/rdocx/src/document.rs:22670`). This is actual provenance: `OpcPackage::open` opens a file and calls `from_reader`, which uses the checked ZIP reader (`crates/oxml-opc/src/package.rs:93`). It does not treat an arbitrary in-memory package as a byte opening. `Document::from_package` still passes false (`crates/rdocx/src/document.rs:22792`). New/Flat OPC eligibility, literal UNBOUNDED limits and the other reopen guards are unchanged.

No parsed model or checked package is fabricated by changing the flag. The successful file read has already established the same relevant package-read provenance as an actual byte read. Subsequent reuse remains subject to current package validation and the existing exact main/context dependencies.

## Checked hyperlink adoption

The hyperlink method still prepares its private candidate and successfully validates it through the existing checked reopen before any publication. It now retains that reopened result rather than discarding it (`crates/rdocx/src/document.rs:27954`). Adoption requires matching main-part identity, raw current main bytes and all three stored root/body scope vectors (`crates/rdocx/src/document.rs:27955`).

The candidate retains its authored typed document. Only the actual validated package, opening record and identifier scan cache are adopted. The actual reopened `Parsed` AST is not replaced with the authored model. Authored relationship/drawing bookkeeping and other identifier state are not overwritten wholesale. Carried block facts pass the existing installation checks for actual source bytes, successful source parsing and normalized model output (`crates/rdocx/src/document.rs:23993`). A guard miss retains the former validated-candidate publication path. A reopen error still occurs before publication.

The new native test covers new documents, actual byte openings and actual file openings, with three distinct hyperlink additions each (`crates/rdocx/src/document.rs:48599`). It requires the new-document first commit to write one checked ZIP and subsequent warm commits to write none. It verifies the authored local relationship namespace attribute survives, the retained parsed AST equals a fresh parse of actual current source, and saved/reopened URLs equal the complete expected set. An invalid unrelated package-root relationship causes refusal while model bytes, stored parts and the original bad relationship remain unchanged. These assertions distinguish validated provenance adoption from either committing a normalized replacement model or merely setting an eligibility flag.

## HLD and evidence

The architecture and packaging additions describe actual ZIP-file provenance, preservation of the authored hyperlink body, actual reopened parse ownership and order-independent query arguments while retaining exact stored proof dependencies (`docs/hld/03-architecture.md:62`, `docs/hld/04-opc-and-packaging.md:1946`). They make no new timing or whole-call constant-cost claim.

- `/private/tmp/s954-binding-order-red.log`: actual reversed-equivalent-binding regression failed before the fix, returning no proof instead of the authoritative relationship result.
- `/private/tmp/s954-binding-order-and-opening-green.log`: seven focused controls passed in 11.74 seconds.
- `/private/tmp/s954-binding-order-opening-final-controls.log`: after the final URL assertion, the same seven controls passed in 12.14 seconds.
- `/private/tmp/s954-binding-order-opening-clippy.log`: applicable scoped clippy succeeded in 22.37 seconds.
- The temporary simple-fixture test failed its own premise that replay differed from writer bytes. It is not semantic RED evidence and was removed. The actual pinned diagnostic established the query-order cause, with replay equality and positive records preserved. The archived diagnostic is not part of the product diff.

No additional correctness, namespace, panic, resource, structural-smell or nitpick finding was established. Prior normalization-cache/source-read safeguards remain applicable and unchanged. The old pinned release summary and the repaired summary with six required misses retain their recorded outcomes. A current release artifact and affected timing remeasurement are still required. Memory qualification, native Word approval/execution, missing external PowerPoint resources and all remaining final riders stay pending. No final feature, sprint or release acceptance is claimed.
