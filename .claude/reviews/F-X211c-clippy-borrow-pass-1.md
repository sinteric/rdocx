# F-X211c, capture borrow correction, pass 1

**Reviewed**: One mechanical expression at crates/rdocx-py/src/document.rs:7996, frozen checkpointf6e1932a65c5db91af6889d3a85308ae5fff6a4eb0e212cded29e19dc69c8b35.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Not found

Correctness and contract: replaces a temporary single-element array containing a cloned owner with std::slice::from_ref of the same current owner. The shared capture_scopes implementation at crates/oxml-py-support/src/lib.rs:346 validates the same origin and first retirement cause, performs the same captured-state allocation and clones each input state into the result. It observes no transient owner strong count. The owner remains live in self.owners throughout the call. Captured tokens, operation order, failure paths, identities and persistent retention policy therefore remain equivalent.

Panics and structure: no new indexing, allocation, owner state, module, trait or dependency. OOXML: no native document or serializer change. Tests: the original actual strict Clippy failuread255678 remains failed. Current strict Clippy log82ab21cc ends in successful completion. Remaining MSRV, WASM, hash and formatting checks are still in flight and are not credited by this review.

## Provenance bridge

Root independently authenticates current/preimage/retained file hashes, checkpoint, source map, inverse proof, failed receipt and successful Clippy log. Exact replacement reproduces the complete current document.rs bytes, with no other source difference from the reviewed predecessor. Preimagea61bbb1b and current7bdd9c5e are distinct, not byte-identical. Installed Word native5f325f5e and focused installed receiptb5a9ac0e qualify the predecessor. The reviewed semantic equivalence above permits their bounded reuse for this mechanical correction, without calling the old wheel a newly built current artifact. Fresh release-version wheel production and final composed source acceptance remain due.

No runtime, corpus, build or source mutation ran in this review. Only this canonical review record was written.
