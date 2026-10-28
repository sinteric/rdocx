# F-X268, source piece integration, pass 5

**Reviewed**: Documentation-only D1 remediation after pass 4 in `work/f-x268-codex`. The two inspected corrections are `docs/hld/04-opc-and-packaging.md:1947` and `docs/hld/12-testing-strategy.md:4455`. The unchanged source SHA-256 remains `af0cc88b3bb4e45dcf757bfc81e590e44a012359067e14a0be9d0b7f53be1388` for `crates/rdocx/src/document.rs`.
**Reviewer**: Independent Codex agent `/root/review_source_pieces`. The reviewer did not implement the corrections, modify source or run builds or tests.
**Verdict**: 0 defects, 0 smells, 0 nitpicks. D1 is resolved in this scope.

## Defects

None. D1 is closed.

## Smells

None.

## Nitpicks

None.

## Resolved finding

D1 concerned the claim that clearing the opening record alone universally restored the full reader and canonicalization. The packaging section now distinguishes loss of trusted reread provenance, which restores ZIP staging, from disabling independent child proofs and removing relevant carried records for a whole-reader or whole-writer control.

The testing section qualifies the growing-work observation as the pinned opening-cache-off fixture's state without a reusable child record. It explicitly states that clearing an opening alone does not disable independent child proofs. The child-source and writer controls use the explicit force-off path with the relevant carried records unavailable.

The distinction matches the source. `block_proofs_enabled` and `offered_canonical_blocks` govern eligible child serialization, canonical scanner and source-reconstruction proofs. They are not a universal switch for every opening, address, identifier or related-part cache. `source_blocks` checks that switch before either retained or canonical reconstruction. The source reference tests turn it off explicitly. The broader native staging references also clear their current opening and carried child/address records as required by their consumers. The documented instructions no longer equate those different controls.

## Not found

Documentation accuracy, contract, evidence provenance and source-switch scope produced no remaining finding in the D1 repair. Pass 4's source-safety result is unchanged. The initial opening cache still stores an actual exact-byte parse rather than a staged typed model. Its cold-memory effect remains covered by the pending Q22 measurements.

No native or Python result was rerun or newly claimed by this documentation review. Mechanism 7 qualification, drawing-ID callers, mechanism 8, A3 and A5 composition, HLD completion for subsequent implementation, release equality and error order, memory, performance, native Word, hash and full integrated verification remain pending. This is not whole-feature or final sprint acceptance.
