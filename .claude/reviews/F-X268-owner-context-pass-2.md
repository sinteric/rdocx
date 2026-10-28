# F-X268, owner context, pass 2

**Reviewed**: Focused D1 remediation in `WordPieces::assemble`, binding source SHA-256 `8937789813ae33ec9381de538f655aad51ea77428be42fe37355ebea3a1eb2ef`. Root independently inspected the changed predicate, returned inventory and authoritative caller fallback.

**Verdict**: 0 defects, 0 smells, 0 nitpicks within this boundary

## Defects

None. D1 is resolved by inspection. A different existing `Some` context now returns `None` inventory. Callers select `word_owner_locations` before publication. Fresh pieces can remain available for the next stable-context call. Initial construction without a cached context remains distinct. Stable-context admission and the existing refusal policy are unchanged.

## Smells

None found in this focused pass.

## Nitpicks

None found in this focused pass.

## Evidence and boundary

Scoped clippy for the three affected crates and all targets exited 0 in 18.93 seconds. The fresh development build exited 0 in 15.06 seconds. Extension SHA-256 is `b965f19d8e3806b525b8ce6802ddc2819f124b734803fc8fca08c93ef54a8046`. The stable-context run case and four splice cases passed in 6.84 seconds. Prose check reported zero violations. The two native test lint changes name a concrete tuple and use `Box::default`, without changing assertions.

An actual runtime context change was not exercised. That named A3 coverage remains pending. This focused closure does not claim it passed, complete the broader provenance matrix, or approve the whole feature. The prior warm source-equivalence, counted-work and cache-off observations remain applicable to their unchanged paths. Mechanisms 3 through 8, remaining binding publication work and final release, memory, Word, hash and integrated checks stay pending.
