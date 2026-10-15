# F-X211, shared local publication, pass 1

**Reviewed**: Root-serialized shared prerequisite in crates/oxml-py-support/src/lib.rs against its actual preserved pre-edit bytes. One file,265 insertions and5 deletions, consisting of27 production insertions/5 deletions and238 test insertions. The four local-publication tests are new. Existing tests are unchanged.
**Verdict**: 0 defects,0 smells,0 nitpicks within this shared prerequisite. No child or whole-feature acceptance is certified.

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Aspects checked

- Correctness: zero findings. `crates/oxml-py-support/src/lib.rs:394` selects local mode while ordinary prepare_scopes at line389 selects global mode. At line414 only allocated successors and births count against the generation allocator. In local mode, the reserved Document slot is cloned at line443 without allocating a generation. Commit skips only that slot at line494 and still advances the same serial exactly once. Explicit affected scopes retain their own successor ordering and first-cause operation/serial.
- Contract: zero findings. `crates/oxml-py-support/src/lib.rs:479` retains the current Document identity check, while expected serial/generation checks reject stale local preparations even when the Document identity has not changed. Global mode and legacy bump at line515 retain their previous behavior. successor(index) still offsets by1 at line230, born(index) remains independent, and capture_prepared at line376 still captures slot0. Foreign authority and constructed numeric-path semantics are unchanged.
- Panics: zero findings. `crates/oxml-py-support/src/lib.rs:410` preflights serial and generation arithmetic before state publication. The private PreparedRevision construction preserves nonempty slot0 and preallocates retired/successor/birth vectors. `commit_scopes:489` has no new allocation or fallible action after the first retirement. Existing private-invariant expect remains prevalidated. The boolean mode cannot be forged by external callers because the prepared fields remain private.
- OOXML: zero findings. This file changes no XML parsing, serialization, namespace, relationship, schema-order or opaque-byte behavior. It supplies retirement authority only. A binding still must prove actual physical owner survival and finish native candidate validation before calling commit.
- Tests: zero findings. `crates/oxml-py-support/src/lib.rs:747` requires retained Document/ancestor authority, retired descendant authority, future capture validity and later conservative global retirement. At line806, dropping preparation releases uncommitted successor/birth states and restores Document Arc count, while foreign/duplicate/retired/stale refusals remain atomic. At line876, overflow refuses before mutation and empty local preparation at an exhausted generation allocator succeeds without minting a generation. At line908, successor/birth indexes, foreign/stale future captures, same-serial causes, reversed state order and explicit numeric selector refusal are checked. The deciding ancestor-survival expectation fails under original global retirement, rather than merely testing method presence.
- Structure: zero findings. The existing RevisionCounter, PreparedRevision, RevisionState and ContentPath are reused. One explicit mode field implements the root-admitted local versus global publication distinction for the five concrete Word XML owners. No new type, framework, trait, generic, registry, file, module, dependency or flag is introduced.

## Actual failure and successor evidence

Actual pre-edit lib.rs SHA is `ec8712df5d1843409b18f4f47a727ae6ae51ebef9a1f0185eba07257c0ac807c`. The exact actual pre-edit/current textual diff was inspected independently.

Original deciding RED receipt SHA `bf451ccab87c79441c5a3921eebfd12f7aa42c24fb4cca78e19c9d61e70dde15` authenticates exit101, no timeout and safe owned cleanup. Its raw log SHA `f7d05e1d8cdf13a4b28ccab0a94e11f60709888f665f9496e2a3c09aa106e7d8` authenticates an executed test failure on ancestor.validate_scopes after global retirement, not a compiler failure or missing API. Original evidence is retained and not relabeled passed.

Root confirmed session78372 terminal0. Green receipt SHA `616df115c87cd9d8902882d79825ad3d1a87579793e59b4b43ccc51aee9a577b` authenticates four actual exit0, qualified, non-timeout rows: locked/offline all-target/all-feature check, all support unit tests, strict all-target/all-feature Clippy and workspace formatting. Every row has reaped leader, gone owned group and safe lease release. All four raw log hashes were independently checked. Unit result is17 passed,0 failed,0 ignored. That includes all four new controls and all unchanged fx206/legacy tests. The successful doctest command contains0 cases and receives no invented behavioral credit. Driver configured8388608-byte stack, so this evidence is not an ordinary2MiB native consumer claim. No test or runtime was run by this reviewer.

## Source integrity and acceptance boundary

Current reviewed source SHA is `b189bb464b75938653441d76d5b237bc59ca9d579293875235c5dffb0cc955e2`, matching the actual green driver guard. HEAD remains0a4f9259c6d98f281075244cdc6b7b39da8edd83. NUL-delimited index SHA is `d1c274930f3047dc8ae8da4d21c9512eda688892ebb8b97a5c7624fa7f9034ef`. Reviewed carrier binary-diff SHA is `d27da6c6a2f96ac20f00654069198c9dca2eb867aa2bd0adfcc8ec063c64a327`. Source, HEAD, index and reviewed diff authenticate unchanged before and after writing this report.

The canonical workflow/microscope, parent contract and approved childa/b/c ownership boundaries were read. This review authorizes only the shared prerequisite's correctness. Captured Document survival does not establish physical StoryItem, hyperlink, paragraph, table, cell or section survival by itself. Childa must attach and retire actual owner scopes and stage allocation-safe publication. Childb/c remain separate owners. Existing global Word append retirement must remain conservative.

No hash-harness, WASM, MSRV, package footprint, installed binding, XML-owner, native Office, child scoped-verification or whole-feature completion claim is made. Maximal85 and the separate native duplicate raw-content semantic correction remain outside this pass. Only this review file is written, then control returns to root.
