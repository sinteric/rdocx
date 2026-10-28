# F-X268, final performance remediation, pass 28

**Reviewed**: Frozen working diff against `9fb396cbf9c42eb05efa6eedf15f57f0667aa34a`, six files, 497 insertions and 45 deletions. Scope is the plain-table inventory proof, empty complex-admission guard, existing inline tests and approved plan/HLD updates.

**Reviewer**: Codex, `/root/review_source_pieces`. I supplied an independent design assessment and static pre-audit, including two temporary preparation defects corrected before compilation. I did not author the mechanism, prototype, canonical implementation or tests. Earlier diagnostic profiling I prepared does not independently qualify release timings.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Identity and contract

I consumed the canonical workflow and microscope instructions, approved contract at `.claude/plans/F-X268-design.md:646`, current in-progress state and latest progress checkpoint. The six current hashes match `/private/tmp/s954-table-canonical-scoped/freeze.json`, SHA-256 `3f214d0fd9cecfe4557faa8186fdad686f5afa1dc717b5406f2015cc30c4a57c`. The current six-path diff exactly matches its `frozen.diff`. Product source `crates/rdocx/src/document.rs` is SHA-256 `f0bcdef5688ce1b19dcbd309116b18586fec5047c02b2487b9131850186ba3fb`.

I made no source changes and ran no builds, tests, imports, renderers, profiles or timing/RSS workloads during this review.

## Source assessment

**Read authority and provenance.** `crates/rdocx/src/document.rs:9187` retains the existing exact part-partition and immutable frame checks before collecting proof ranges. Each admitted block needs a populated successful actual source read, matching root/body declarations and bindings, lexical field exclusion and reusable private facts. It fills no read memo and uses no staged model as read authority. The existing frame-changing publishers invalidate child read facts. Their constructor-composed frame invariant remains required, as in pass24. The vector is ephemeral and ordered by the checked body partition, not a new retained cache.

**Complete table shape.** `crates/rdocx/src/document.rs:10832` checks table, row and cell containers, typed controls, root/raw sidecars, table-property exceptions, grid history/raw children, property revision/raw state, border raw attributes/source XML and shading extra attributes. Each cell content must be a paragraph satisfying the unchanged full plain-paragraph predicate. Nested tables and controls decline. The predicate complements `BodyContent::owner_facts_reusable_by_value`, which explicitly permits raw XML and therefore would be insufficient alone (`crates/rdocx-oxml/src/document.rs:750`).

I traced actual storage and parsing through `CT_Tbl`, `CT_Row` and `CT_Tc` at `crates/rdocx-oxml/src/table.rs:2684`, `table.rs:2379` and `table.rs:2088`. The actual cell paragraph branch calls the same CT_P parser family at `table.rs:2231`. Unknown owner children are retained separately and excluded by the new predicate. This is not a claim that every ignored property byte was modeled. The unchanged discovery grammar admits only the table/row/cell owner chain at `crates/rdocx/src/document.rs:11817`, with unsupported property descendants opaque through `document.rs:16561`. Consequently an ignored property descendant cannot supply an accepted TableCell paragraph merely because it falls inside a table byte range.

**Physical query and errors.** `crates/rdocx/src/document.rs:10892` retains exact direct-paragraph start/end equality. A table hit additionally requires a nonempty span and an actual TableCell owner, with both strictly inside the same certified table. Binary lookup uses the already sorted disjoint physical block ranges. Inverted, crossing, unrelated, whole-table and partial direct-paragraph queries decline. At `document.rs:30164`, main-part identity/root checks, actual owner and paragraph discovery, target namespace reconstruction and fragment closure precede the skip. Optional batch failure still restores the ordered original reader. The only skipped operation is redundant standalone paragraph parsing and its known-empty drawing projection. Related parts, unsupported tables and genuine drawing paragraphs keep the original path.

**Complex admission.** `crates/rdocx/src/document.rs:18135` adds a pure, allocation-free existence check. When any complex item exists, the original paragraph-major loop, candidate order, containment checks, typed admission, errors and private source-ID allocations run unchanged. With no complex item, the removed loop would only test false predicates. Final retention, sort and dedup remain outside the guard and unchanged.

**Structure and panic checks.** No new module, trait, generic, retained type/cache or public interface was added. New lookup arithmetic uses checked predecessor selection and checked range endpoints from the existing partition. No new untrusted slice or unchecked index was introduced. The proof remains optional and does not authorize writing XML.

## Evidence assessment

I verified the scoped receipt hash, all recorded check/build/native/clippy/fmt/prose/diff log hashes and the actual selected native binary hash. The canonical receipt records one check, one native build and fourteen unique passing cases on the final frozen source, followed by successful all-target/all-feature clippy and policy checks. No canonical failure or rerun is claimed.

The new matrix at `crates/rdocx/src/document.rs:51964` positively establishes source-read table ranges and exact direct-paragraph queries, physical owner containment, absent-read nonpopulation, stale bytes and real frame-publisher invalidation. Model mutations test predicate branches and are not represented as actual source certificates. The canonical additions parse genuine table, row and cell controls and assert their specific typed storage before refusal. `document.rs:52421` runs all eight existing field/private/opaque source fixtures both directly and inside tables. Existing selected cases retain force-off parser restoration, namespace and nested-owner grammar, MC pictures, malformed picture refusal, merged-cell atomicity, source-read versus writer authority and owned transaction/reopen behavior.

The temporary normal-cfg prototype result is SHA-256 `eb1795e03934189b2d5b9dda4e27a0eff2da19c80eed59f0997fd0682cad8ef1`. I verified its referenced evidence file hashes. It records 48 whole-public windows across ON and independent table/complex OFF controls, exact saved/reopened parts, media, relationships and physical occurrences, eight existing Python guards and actual MC table-picture positive controls. At Cell100 warm, 44 table paragraph skips reduce actual fragment parses from84 to40 while discovery/scope/closure and earlier source/normalization work remain visible. Empty-complex predicate visits change from19534 to0 with299 precheck visits. These are actual mechanism counts, not timing predictions.

Unlike the previous all-gap1 observations, canonical and aliased ordinary complex fixtures genuinely produce a complex Field item and enter successful typed admission. The temporary hook records one admission and the same original two candidate predicates in both modes, with exact items equal. This proves positive admission, not a new allocator-count measurement. The 24 guard keys run inside one diagnostic hook in each mode, not 24 separate Rust tests. Temporary patch-base, Cursor and diagnostic formatting setup failures remain preserved and are not semantic RED evidence.

## Documentation and remaining qualification

The plan and four HLD updates accurately distinguish exact direct spans from strict table-owner coverage, actual source reads from staged writer values, complete inventory from the parser skip, and source certificates from model-only predicate controls. See `docs/hld/03-architecture.md:71`, `docs/hld/04-opc-and-packaging.md:1963`, `docs/hld/12-testing-strategy.md:4474` and `docs/hld/14-development-backlog.md:8675`. No timing bound, public contract, native oracle criterion or final rider was removed.

Correctness, contract, panic, OOXML, tests, structure and evidence attribution produced zero findings in this bounded batch. The actual 9fb Document pass and Cell failure by approximately0.177ms retain their original artifact and outcome. This review supplies no new normal-artifact, performance, memory, final-union, completion or publication acceptance. The unchanged normal picture gates and remaining qualification still require actual execution.
