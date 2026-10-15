# F-X199, all, pass 2

**Reviewed**: Frozen working diff against Base71435bae, eight tracked files,
159 insertions and 11 deletions. Final scope includes actual rdocx and
rdocx-oxml archive records. Parser and regression source remain unchanged
from independent pass 1.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: `crates/rdocx-oxml/src/table.rs:1782` retains the qualified
  empty-container admission and namespace-only attribute boundary reviewed
  in pass 1. The default at `:1792` supplies no explicit margin values.
  Existing qualified scope resolution at
  `crates/rdocx-oxml/src/properties.rs:369` handles aliases, default namespaces
  and rebound prefixes without local-name admission.
- Contract: `.claude/plans/F-X199-design.md:37` requires actual local archive
  measurements. `README.md:42`, `crates/rdocx-oxml/README.md:20` and the two
  tuples at `scripts/readme_doctests.py:405` and `:411` agree. Independent
  read-only inspection observed rdocx (3525008, 50765013, 37) and rdocx-oxml
  (452312, 2809322, 32). Both pass unchanged measurement validation. The
  one-byte compression differences from adopted records are within the
  existing 64-byte tolerance. Exact member bytes and counts agree. Packaged
  integration_test.rs and table.rs bytes match this reviewed worker source.
  Both archives remain below 10 MiB. No manifest, lockfile, version, inventory,
  tolerance, historical performance or publication change appears.
- Panics: `crates/rdocx-oxml/src/table.rs:1785` propagates attribute errors.
  Production introduces no unwrap, expect, untrusted indexing or arithmetic.
  The added unwraps remain confined to controlled regression inputs.
- OOXML: the rejection path at `crates/rdocx-oxml/src/table.rs:1679` retains
  raw content. The margin writer at `:1868` retains schema order. Controls at
  `:3547` preserve exact foreign, rebound and attributed source through a
  second parse. The absolute-slot test at `:3579` retains unsupported sibling
  positions after typed mutation. HLD10 at
  `docs/hld/10-bindings-spec.md:1689` states the same narrow contract.
- Tests: `crates/rdocx-oxml/src/table.rs:3518` covers compact, spaced,
  expanded, alias and default-namespace forms. The existing public entrypoint
  at `crates/rdocx/tests/integration_test.rs:535` checks all 21 cells before
  and after reopen. Recorded real Base failures and subsequent 54 of 54 table
  tests plus the one public regression pass distinguish the fix from an
  always-green test. This review does not claim the remaining scoped gate.
- Structure: the existing private parser branch at
  `crates/rdocx-oxml/src/table.rs:1782` remains the complete production change.
  No new implementation file, module, public signature, trait, generic,
  dependency or policy mechanism appears. Only the two affected existing
  archive tuples and README rows changed after pass 1.

## Verification boundary

Scoped lint, full changed-crate tests, hash harness, repository policy suite
and verified package dry-runs remain pending. The approved plan retains its
completion barrier. This review only read existing archives and source.
It did not rebuild packages or modify any file outside .claude/reviews.
