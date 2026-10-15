# F-X213, working implementation, pass1

**Reviewed**: Actual working diff on `work/f-x213-codex`, HEAD `3723dea217efdd1d1ee37201ec73b0fd1b5dab20`. Three existing carriers,1533 insertions and5 deletions. Read approved design, spec and risk references, source progress, prior proposal reviews and complete implementation/test diff. No self-authored F-X213 production is included in this review.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Source and evidence authentication

Adoption receipt SHA `9b346e17caa72676595c03e5572f59222eb3ba1d3f5190b4c7b0bc9151a870d8` binds current source map SHA `7df137d8fd05c92c8bcfd653dc63fda8ae7ae19d9f059ea5de7f45c08304f20c`. Independently checked all2852 tracked records against actual bytes, sizes and modes. Exactly the declared three records changed from the before map, with2849 others unchanged.

- `crates/rdocx/src/field.rs`: SHA `d368401571a362c6593acab9487e59d893b93ac92d3c41be5927e136f8135d3a`.
- `crates/rdocx/tests/regression_test.rs`: SHA `3e9aa0b476353cf21ee13f483b14de5bb62418930a4ade8e56d9aeb595773cc2`.
- `crates/rdocx-py/tests/test_core.py`: SHA `5c9a8a68a003ec8d683751da538bfedde4a06aca036f895586c9f6f1e51bcdd8`.

Actual Base receipt SHA `2bc0b4c6b113195814cfc83776241b5c817d1750613c784738238faf9ba4d284` and both raw log hashes match. The three-entry control actually failed on8 versus6 paragraphs. The single-entry control actually failed on the former paragraph-boundary refusal. Both children exited101 with0 passed,1 failed and0 ignored. These are retained Base observations, not corrected-source results.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Aspect results

- Correctness: zero findings. Reviewed `crates/rdocx/src/field.rs:9168` entry recognition and source spans, unchanged-count topology, count changes, one-to-many and many-to-one transitions, one/two-root zero-entry handling and subsequent positive rebuilding. End-root identity or opaque properties prevent unsafe root removal. Existing marker-only and protected-boundary generic topology remains available. The newly admitted single-paragraph case retains a named refusal when an overlay cannot safely be formed. Generated-source slicing is bounded by internally emitted canonical entries, while source paragraph offsets come from the qualified scanner.
- Contract: zero findings. The source-aware bounded repair uses existing rebuild APIs and resolved TOC entry styles. Original instructions, nested PAGEREF balance, deterministic page substitution, relationship ownership and unrelated package parts remain on the established transaction route. No bibliography or parked navigation implementation is adopted. `crates/rdocx/src/field.rs:2076` stages independently, and :2334 validates/reopens before publishing through commit_staged_mutation.
- Panics: zero findings. Inspected new direct paragraph indexing, source ranges, optional property-root access and generated entry slicing against scanner/generated-source invariants. Invalid namespace/property parse, duplicate properties and unsafe boundary topology return errors or the established generic path before live commit. This is static review, not a claim that panic tests executed.
- OOXML: zero findings. `crates/rdocx/src/field.rs:9378` qualifies properties by namespace URI. Duplicate owned leaves refuse. Complete started pStyle ranges survive retargeting and are fully removed for zero entries. :9649 overlays generated style/tab facts while retaining root attributes, opaque particles, entity spellings and other children. Tabs retain their source root and opaque siblings. :9864 closes each moved top-level retained root against its actual inherited bindings, covering paragraph-local aliases/default namespaces and foreign shadows. End-run/suffix moves and closing paragraph names preserve balanced XML. New generated pStyle/tabs insertion follows CT_PPr order.
- Tests: zero findings. The fifteen added native definitions retain genuine Base discriminators and cover count changes, old levels, content-control/nested PAGEREF, marker-only generic behavior, guarded boundaries, opaque source, alias/default/rebinding, adjacent fields, duplicate refusal, single-owner expansion and zero-to-positive save/reopen. The two installed definitions exercise the public API and preservation. Concrete source-built namespace controls at `crates/rdocx/tests/regression_test.rs:398579` check expanded identities and exact retained payload, beyond lexical prefix assertions. Tests are source inventory only until actual collection and execution. Existing update/layout, hash and risk gates remain required.
- Structure: zero findings. Three existing carriers only. No public method, dependency, module, trait, generic or product file is added. Private helpers perform the actual scan, source overlay and namespace closure operations without forwarding-only layers.

## Limits and handback

Corrected native GREEN, fresh installed tests, deterministic layout/hash49, changed-crate checks/tests/clippy, typing/stubs, both WASM graphs, verified publication and scoped policy remain runtime or completion obligations. HLD04/08/12/14 amendments and structured handoff remain separate work. This review does not grant those results or whole-sprint verification. Prior proposal review authority and its exact preservation resolutions remain intact.

Only this report was written. No source, formatting, Cargo, environment, cache, UI, baseline or publication action occurred during review. The pass ends and returns the worker review lease to ROOT.
