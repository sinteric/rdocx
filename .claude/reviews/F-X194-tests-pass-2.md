# F-X194, tests, pass 2

**Reviewed**: Supplemental five-line fixture-only change in the frozen seventeen-file working diff against Base400e52b09c60f44ea20f0de7cd05240ed6952b5f.
**Diff SHA-256**: 7648e16cd6eddf80dd9db848c7f20b47c10b89c9c8cfe56fcecef859df0feae9
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

The full approved contract and prior all-aspect pass1 were read. The only
changed reviewed bytes are the canonical p/a/r and package-relationship
namespace registrations in the imported grouped-cell fixture at
`crates/rpptx-py/tests/test_documented_examples.py:4924`. The URI identities,
group and grid-cell ownership, relationship target and positive read assertion
remain unchanged. The native shared parser currently extracts literal r:id
at `crates/oxml-drawing/src/text/paragraph.rs:500`. Its pre-existing alias
loss was reproduced and retained separately with original exact XML.
Canonical serialization exercises the existing native-reader surface while
keeping the new cell-body resolver regression meaningful.

The alias defect remains explicitly assigned to F-X211 with namespace-aware
reader and exact preservation controls. This review does not claim that the
defect is repaired or that aliased inputs pass. No positive hyperlink
assertion was removed or relaxed. All sixteen other source/HLD hashes and
all production bytes match the independently reviewed pass1 identity.
Correctness, contract, panics, OOXML and structure retain that unchanged
source review. The supplemental tests aspect has zero findings.

## Verification boundary

The fresh wheel production source is unchanged. Updated focused tests and
full scoped, package, hash and integration gates remain required on the
corrected carrier. Earlier valid-alias RED evidence remains a failure under
its separately tracked owner. This review ran no Cargo, installed tests or
formatter and wrote only this review file.
