# F-X206, surfaces Base runtime, pass 41

**Reviewed**: Independent R3 producer-admission successor against clean pass39 R2 and the approved F-X206 surface contract. Complete runner and R2 delta, source-derived graph, five original controls, three temporary candidate records, templates and current2714 guards authenticated. Packet receipt is a0d143e91dddcbc50c6efb6efcbc4de618f53fa647a681c6a30395c1e73d85fd. No runtime, compiler or source mutation occurred.

**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Defects

### D1, transition selectors are not tied to actual producer Rust differences

`/private/tmp/S91-F-X206-surfaces-Base-runtime-r3-prepared/run.py:27`

The actual producer source is admitted separately, but only its manifest rows are compared with the prepared graph at line28. The graph's prospective producer snapshot, changed Rust inputs and eight selected roots are not checked against the actual producer map. A later producer with identical manifests and another changed Rust carrier can satisfy this admission while retaining selectors derived from a574. The new affected crate and its reverse consumers may then reuse foreign compiled bytes during the Base diagnostics.

Bind the actual producer map to the exact graph comparison snapshot, or recompute and authenticate the producer-to-temporary-consumer differing inputs and reverse closure before any timestamp/source mutation. Root manifests and lock/config inputs must retain their explicit qualification. A changed source snapshot requires a renewed graph, not merely a seal flag carrying the old graph hash.

### D2, accumulated compiled closure is never used to reject unknown cache targets

`/private/tmp/S91-F-X206-surfaces-Base-runtime-r3-prepared/run.py:44`

Each evidence row derives a closure and adds it to established, but nothing checks that accumulated set before execution begins at line46. The empty remainingUnknownCacheCrates assertion at line21 is not reconciled with the derived evidence or the selector coverage. For example, one genuine oxml-pdf check row can establish only its dependency subset while the rest of the twelve-crate consumer closure remains unproved. An empty asserted unknown list still admits execution with no conservative invalidation for untouched unknown targets.

Derive unresolved target state from the actual evidence and require every needed cache target either to have source-qualified producer evidence or to enter a conservative checked reverse-root invalidation closure. Reconcile that result with the seal's unknown list and admitted selectors. Preserve the distinction between captured source inputs and proof that a particular target compiled.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Not found

Panics, OOXML and structure produced no additional findings. Correctness, contract and test-evidence admission findings are recorded above. The original five command objects, semantic markers and three temporary candidate records are exact R2 identities. Current53debc source bytes, sizes and modes matched all2714 records. The original namespace, cell-anchor and naming discriminator scope is unchanged.

Pass39 lifecycle and restoration closures remain retained: whole owned-group disappearance and reaping precede restoration, unknown cleanup blocks handback, and the prospective temporary map must equal the actual attempted map. Actual build/test observations remain log-backed and separate from attempted source. Restored source is explicitly not claimed as the compiled producer. These are sound retained safeguards, not runtime credit.

Future producer receipt, source and seal remain null. No five-Base execution, semanticRED, feature acceptance or native completion is established. This pass ends without remediation.
