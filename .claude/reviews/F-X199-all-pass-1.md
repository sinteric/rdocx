# F-X199, all, pass 1

**Reviewed**: Frozen working diff against Base71435bae, five tracked files,
153 insertions and 5 deletions. Independent static review of the parser,
existing-entrypoint regressions, approved design plan and declared HLD changes.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: `crates/rdocx-oxml/src/table.rs:1782` admits only qualified
  empty tcMar with namespace declarations. The default at `:1792` has absent
  edge values, not explicit zeros. Existing scope resolution at
  `crates/rdocx-oxml/src/properties.rs:369` removes rebound prefixes and admits
  local aliases and default namespaces correctly.
- Contract: the production branch matches the exact PR302 head
  7ac2851cb61b5e7b26427004517cc0cd4db0dbd9 patch. Local additions strengthen
  preservation controls rather than broadening admission. The approved plan
  at `.claude/plans/F-X199-design.md:37` keeps actual generated archive records
  and remaining verification behind explicit completion barriers. HLD10 at
  `docs/hld/10-bindings-spec.md:1689` describes the same narrow reader contract.
- Panics: `crates/rdocx-oxml/src/table.rs:1785` propagates malformed attribute
  errors. No new production indexing, slicing, unwrap, expect or arithmetic
  appears. Test unwraps operate on deliberately constructed bounded inputs.
- OOXML: the existing empty-event dispatcher at
  `crates/rdocx-oxml/src/table.rs:1679` preserves rejected raw content. The
  writer at `:1868` retains the absolute margin schema boundary. Tests at
  `:3547` assert exact rejected bytes before and after reparse. The mutation
  control at `:3579` checks all surrounding typed and unsupported slots.
- Tests: `crates/rdocx-oxml/src/table.rs:3518` covers compact, spaced,
  expanded, alias and default-namespace spellings. Controls at `:3547` cover
  foreign names, rebound prefixes, real qualified and unqualified attributes.
  The public generated 7-by-3 table at
  `crates/rdocx/tests/integration_test.rs:535` checks all 21 cell texts and
  both reader flags before and after reopen. The recorded Base failures in
  `.claude/scratch/F-X199-progress.md:18` establish that the unit and public
  regressions detect the original omission. This review does not claim a
  post-change test pass.
- Structure: `crates/rdocx-oxml/src/table.rs:1782` extends one existing private
  parser branch. No new implementation file, module, trait, generic, wrapper,
  dependency or public signature appears.

## Verification boundary

Post-change Cargo checks, scoped verification, hash harness and actual local
archive measurements remain pending. No generated footprint records were in
this five-file review. Their eventual diff requires another independent pass
before preparation. No source, documentation, progress or formatting changes
were made during this review.
