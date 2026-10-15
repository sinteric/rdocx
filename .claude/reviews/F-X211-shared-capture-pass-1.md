# F-X211, shared capture, pass 1

**Reviewed**: One frozen shared carrier, source30, against the root-admitted local-publication prerequisite and source29 contract. Relative to the retained local-publication carrier, 113 additions and 0 deletions add two concrete accessors and two focused controls. Source29 to source30 adds59 lines. No publication algorithm changes.

**Verdict**: 0 defects, 0 smells, 0 nitpicks within this shared prerequisite. No Word public, child or whole-feature acceptance.

## Contract and authentication

Read the canonical workflow/microscope, approved parent and childa ownership/capture contract, HLD10 binding invalidation and prior `F-X211-shared-local-publication-pass-1.md`. The existing shared RevisionCounter, PreparedRevision, RevisionState and ContentPath remain the sole authority primitives. Physical-owner survival still requires explicit provider proof. A document epoch alone cannot validate an old snapshot ordinal.

Actual retained pre-source29 lib.rs SHA `b189bb464b75938653441d76d5b237bc59ca9d579293875235c5dffb0cc955e2` matches the previously clean prerequisite. Actual source29 preimage SHA `71c06861c93e83bf3659d9bca7ab3982a5c2c0d04614a8628f128c7ebe6846ec` and current retained/current source30 SHA `17ddb95a27f77ebefd234d928ad6024fc713ad5033677165bf50b6b56b276d99` authenticate. The shared carrier stayed frozen before and after this review. Disjoint native/Python worker changes were explicitly permitted and are not falsely treated as shared-carrier drift.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Aspects checked

- Correctness: zero findings. `crates/oxml-py-support/src/lib.rs:292` clones the actual current document state. `lib.rs:239` clones the actual prepared slot0 document state. In global mode that is the allocated successor. In local mode it is the retained current document state. Neither accessor increments a serial, mints a generation, retires a token or publishes a candidate.
- Contract: zero findings. Existing owner-only captures can remain valid across explicitly proven append operations, while Run and unsafe Story/Item snapshot captures combine owner authority with the conservative document epoch. A returned Run can combine prepared document/owner/text states before its Python allocation. `lib.rs:372` must still validate the same origin and current preparation before the caller combines prepared scopes. Accessor availability does not waive that caller obligation or allow an allocated handle to escape before publication.
- Panics: zero findings. `lib.rs:239` indexes the same private nonempty successor slot already consumed by capture_prepared and commit_scopes. The private prepare routine always seeds the document retired/successor slot before returning a PreparedRevision. Arc cloning introduces no fallible heap allocation or unchecked generation arithmetic. Capturing the document state before eager initialization preserves the existing strong-count initialization refusal at `lib.rs:314`, tested explicitly rather than bypassed.
- OOXML: zero findings. The diff changes no parser, serializer, namespace, relationship or opaque-byte admission. A physical paragraph/table/cell/section proof and snapshot-inventory retirement remain the binding provider's responsibility. These accessors cannot revive a removed physical owner or justify matching by value/index.
- Tests: zero findings. `lib.rs:758` qualifies actual old versus returned future Run authority, prepublication preparation validation, committed serial and later local text retirement. `lib.rs:812` distinguishes retained owner-only authority from conservative document-plus-text authority and preserves the earliest global cause after a later local retirement. It also checks foreign capture refusal and pre-initialization authority capture. The existing local/global preparation and scope ordering algorithms are unchanged.
- Structure: zero findings. The two accessors expose concrete existing document states for actual current/future combined capture consumers. No new type, registry, history map, next-state link, generic, trait, file, module or dependency is added. Existing successor(index), born(index) and slot0 conventions remain unchanged.

## Actual focused evidence and limitations

`document-scope-accessor-29-actual.json` SHA `14629b73d135cdaa181b8d39d34d2aab3f973d7f640553c80b10b192f58ef22d` records the source29 focused control, actual terminal exit0, 1 passed and17 filtered. It records a2097152-byte stack environment. `prepared-document-scope-30-actual.json` SHA `8134d7aae64458bb4fe4e5aa02a136c98a3428c7f31d82fbbc3319003a826daf` records the source30 focused control, actual exit0, 1 passed and18 filtered, direct tool chunkd33749. These are recorded direct console executions, not owned-driver/group cleanup receipts. Source30's record does not independently pin a stack environment or a current binary hash.

The historical source29 binary SHA `842ca877e83123d03a62f61df3c6f44540ff4c79cd951654ceb8c93fa3c7ff93` is recorded, but its Cargo filename now contains source30-era bytes SHA `bcbb51bb244d32f4ca845675b5d266b68b058a9d7e88f8f3433abf3f874cc517`. The old source29 executable was not separately retained at that path. This review does not claim to have rehashed the historical executable or authenticated a raw log that the focused records do not contain.

No complete current support suite, strict Clippy, formatting, hash, WASM, MSRV, package, installed binding or whole-feature gate is credited by these two focused runs. Broader current shared qualification and actual allocation-safe Word capture consumers remain separate obligations. Original85, immutable corpus divergences and public child acceptance remain outside this pass.

Only this canonical review file was written. No source, index, target, test, CP or UI operation was performed. This review ends and returns control to ROOT.
