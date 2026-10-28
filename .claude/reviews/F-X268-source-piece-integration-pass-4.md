# F-X268, source piece integration, pass 4

**Reviewed**: Opening-parse retention and the five approved HLD files after pass 3 in `work/f-x268-codex`, parent `5759559029d2af5c61ad5e69e3ebc3af8a8557fe`. The surrounding nine-file diff contains 6823 added and 222 removed lines. Source SHA-256 is `af0cc88b3bb4e45dcf757bfc81e590e44a012359067e14a0be9d0b7f53be1388` for `crates/rdocx/src/document.rs`.
**Reviewer**: Independent Codex agent `/root/review_source_pieces`. The reviewer did not implement the increment, modify source or run builds or tests.
**Verdict**: 1 documentation defect, 0 source defects, 0 smells, 0 nitpicks.

## Defects

### D1, opening-record removal is described as a universal whole reference

`docs/hld/04-opc-and-packaging.md:1947`
`docs/hld/12-testing-strategy.md:4455`

The packaging section says that clearing the opening record restores the full parse, canonicalization and ZIP path. The testing section similarly generalizes that a cleared opening record grows all four byte units with body size. Those statements no longer describe every current state. A document can retain qualified `MainPartBlocks` after `parsed` is cleared. `flush_document_to_package` then reconstructs through `blocks.proved_document` at `crates/rdocx/src/document.rs:23705`, and canonicalization can still use current child proofs. Removing the opening record still removes the trusted byte-opened route, but does not itself disable those independent caches.

Qualify the recorded parsed-none fixture as that fixture's unproved-record case. Describe a complete whole-reference control using the explicit proof-off and record-clearing mechanisms appropriate to the tested consumer. This is documentation drift, not a product safety failure or a request to weaken the existing fallback.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness, panics, namespace preservation, parse provenance, source error order and structure produced no source finding in the inspected increment.

`from_package_reusing` obtains `document` from one of three sources: `MainPartBlocks::proved_document` bound to exact current bytes, a retained actual parse qualified by part name, bytes and namespace scopes, or a fresh successful `CT_Document::from_xml`. It does not accept a staged typed model as a parse. The immutable local `document` is not rewritten before its clone is stored in the new opening record. Retaining that parse unconditionally avoids a later duplicate read, rather than changing which bytes were parsed. Inspected paths are `crates/rdocx/src/document.rs:22254`, `crates/rdocx/src/document.rs:22286` and `crates/rdocx/src/document.rs:22481`.

The actual serialization-success proof remains independent. A parse whose serialization failed does not gain that proof from the cached model. Trusted byte-opened eligibility, literal unbounded limits and related-part checks are not relaxed by this assignment. Retaining a clone on the first open changes cold memory use, so the existing Q22 memory-at-open, per-layer and combined-footprint measurements remain required. No memory bound or release result is established here.

The five HLD files remain within the approved impact list. Apart from D1, their new descriptions distinguish address scans, current serialization pieces, retained source reads and writer eligibility. They retain the checked frame and whole fallbacks, the six counter units and explicit pending status. The backlog still says in progress and not delivered. Mechanism 8 and the full qualification remain pending. Historical approvals remain historical, while current ownership is Codex.

## Evidence examined

`/private/tmp/fx268-codex-producer-frame-python-test.log` is the actual pre-correction Python result, 9 passed and 1 failed in 183.11 seconds. Fragment work was flat at `[1214, 607, 607, 0, 2, 1]` at both sizes, but its reopen main-parse count was 2 rather than the required 1. That failure is not relabelled as an acceptance result.

`/private/tmp/fx268-codex-opened-parse-native.log` records 13 selected and 13 passed tests in 11.97 seconds. Its cases cover the eight source controls, new, Flat OPC, limited and hook reopen paths, changed related-part errors, identifier error order, the parsed-none growing reference fixture and successful serialization requirements. Their assertions and the supplying parse branches were inspected. No build or test was rerun in this review. The new Python artifact's qualification remains pending at this boundary.

## Scope and pending work

This pass does not complete mechanism 7, the F-ID or the sprint. D1 requires a separate documentation remediation and review. Drawing-ID callers, remaining mechanism qualification, mechanism 8, A3 and A5 composition, release equality and error order, memory, performance, native Word, hash and full integrated verification remain pending. No ready-to-close or release state is supplied.
