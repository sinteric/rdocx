# F-X178, all, pass 1

**Reviewed**: Frozen working-tree diff on `work/f-x178-codex` against its index,
3 files, 121 insertions and 2 deletions. Files are `crates/rdocx/src/run.rs`,
`crates/rdocx/tests/integration_test.rs` and `docs/hld/10-bindings-spec.md`.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- **Correctness**: Both nullable methods return before materializing properties
  for an absent-source clear. Otherwise each assigns only its selected field.
  Zero width, signed position and explicit baseline values are preserved.
- **Contract**: The approved plan and F-X178 story were read. Existing setters
  delegate with Some, builders retain their existing setter delegation, and no
  validation, unit reinterpretation or Python surface was added. The named HLD
  section documents the additive native methods and inheritance reset behavior.
- **Panics**: No new production indexing, arithmetic, unwrap or expect was added.
  The existing property materializer establishes its Some invariant before its
  expect. New test unwraps apply only to constructed fixtures and asserted shape.
- **OOXML**: The existing serializer remains responsible for ordered `w:w` and
  `w:position` output. Its Option checks omit cleared elements. Absent-source
  clears bypass property insertion and raw-position shifting. Unrelated modeled
  and opaque property state is retained.
- **Tests**: The two unit tests cover absent properties, zero, negative position,
  maximum u32 width, independent clears and exact restoration of unrelated
  property state. The integration test covers repeated save and reopen, both
  clears, explicit zero position, explicit false bold and exact opaque payload.
  Its plain-run control requires no serialized `w:rPr`. Reverting the new methods
  makes these tests fail compilation, matching the caller's reported RED check.
- **Structure**: Two concrete nullable methods match neighboring setter style.
  No new trait, generic, forwarding-only type, module, file or dependency was
  introduced by the implementation. Changes remain within the approved scope.

## Evidence boundary

This independent pass inspected the exact diff, approved plan, cited formatting
and test-taxonomy sections, feature status, existing property materializer and
run-property parser and serializer. `git diff --check` passed. The caller's green
test compilation was still running when review began. This review does not claim
that compilation, tests, scoped verification, harness or packaging riders passed.
Those remain the implementing phase's completion gates.
