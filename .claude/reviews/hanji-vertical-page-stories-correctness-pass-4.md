# Hanji, vertical page stories, structure, pass 4

**Reviewed**: final working source paths and ten-file diff after vertical-only row refitting
**Verdict**: 0 defects, 1 smell, 0 nitpicks

## Defects

None.

## Smells

### S1, avoid numbering-state clones for tables without alternative measures

`crates/rdocx-layout/src/engine.rs:2251`

`NumberingState` includes the complete main-story source set. The new clone
runs for every body table, including horizontal tables and tables whose active
stories have only one measure. Repeated copies add document-size work to an
unaffected hot path. Compute finite alternative measures first, and clone the
preceding state only if one differs from the primary measure.

## Nitpicks

None.

## Not found

D3 is resolved by vertical-only row replacement/refitting at both transitions.
No additional conservation, geometry, source-identity, numbering or cache
correctness issue found. The affected native suites, strict Clippy, 139-test
policy gate with two existing skips, WASM checks and 49-entry hash harness
pass. The final archive refresh and final zero-finding review remain required.
