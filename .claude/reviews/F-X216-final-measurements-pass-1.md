# F-X216, final measurements, pass 1

**Reviewed**: Five existing README archive rows and their existing measurement catalog, plus the narrow plan and HLD12 appearance-routing addendum. Final measurement manifest SHA-256 is `790bf7d2173a5932737a4c1ff778d48a0e3ffddf8e63c85346f731b7fd00d951`.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Measurement scope and actual artifacts

Authenticated all six before and current metadata carrier hashes and the
retained metadata patch. Changes update only the five affected archive tuples,
measurement dates and their verified build-mode descriptions. The other
seventeen archive rows are byte-equal under the row generator. The compression
tolerance remains the existing 64 bytes. All row regeneration commands retain
the existing recorder command. That command describes metadata regeneration,
not a claim that the recorder performed these verified package builds.

Opened and independently authenticated every final archive, including its
actual compressed size, all regular member paths, bytes, sizes and modes,
unique inventory and normalized byte total. All 65 ordinary members match
actual current tracked source bodies. Generated manifests, package locks and
VCS metadata remain separately classified. The VCS records identify the dirty
worker HEAD and correct crate path. Their documented normalized sizes remove
the dirty marker and replace only the commit identity with the fixed-width
measurement placeholder. No ordinary source member is normalized.

| Package | Actual compressed bytes | Normalized member bytes | Members |
|---|---|---|---|
| oxml-opc | 106549 | 426524 | 12 |
| oxml-drawing | 225393 | 1487642 | 24 |
| rpptx-oxml | 216919 | 1377707 | 20 |
| rpptx-cli | 62353 | 287324 | 8 |
| rpptx | 713016 | 3816760 | 16 |

The rpptx catalog records 713015 compressed bytes. Its one-byte difference
from the final verified archive is inside the unchanged 64-byte tolerance.
Member bytes and count match exactly. Every archive is below 10 MiB.

Final rpptx archive SHA-256 is
`2cbca9f232ac3c5c410f22cf9f7b3e6f5391088eb044bb12cc6e34a220b651e9`.
Final verification receipt is
`dfdb81faa76e7f77e93da6fa7d4ed600b162561ad857ff380d01dd427401997a`.
The other four final archives remain those authenticated by receipt
`6b04e5e727613acddead7d80c6969d6c0089d97b8e16bde308eb610f10664cbe`.
The older rpptx member snapshot is historical after its two test-routing
attributes changed, and is not substituted for the final archive.

Authenticated both receipts, every recipe/configuration/source-map pin and
their raw logs. Actual package commands carry all 22 reviewed local patches,
`--allow-dirty --locked`, default verification and no `--no-verify`. Raw logs
contain successful Packaging and Verifying stages. All commands exit zero
without timeout and have safe terminal cleanup. These are package checks,
with no publication, upload or release-version change.

## Existing metadata validator

Read the retained checkpoint's unsupported `--metadata-only` invocation as a
setup failure, not verification credit. No such CLI flag was introduced.
Independently invoked the existing `validate_measurement_evidence` function
with actual locked offline Cargo metadata. It passes for current README rows.
Python bytecode writing was disabled, and no build or recorder execution was
started by this read-only audit.

## Routing addendum

`docs/hld/12-testing-strategy.md:4044` and
`.claude/plans/F-X216-design.md:89` add only the admitted focused-rider boundary.
Removing their exact added prose reconstructs the previously reviewed HLD12
and plan hashes. Both task-input appearance nodes remain mandatory against
the final integrated source with exact selectors, `--ignored` and admitted
input directories. The plan names `FX216_NEGATIVE_INPUT_DIR` and
`FX216_CURRENT_NOTES_DIR`. Missing required focused inputs fail qualification.
No private temporary path or new CI setting is added to tracked prose.

The annotations' exact inverses and producer4 production-prefix inverses
remain those qualified by appearance-routing pass 1. Assertions, input hashes,
font-byte and painter facts, negative discrimination and notes language
observations are unchanged. Ordinary ignored counts supply no execution
credit and do not reduce native or integrated acceptance.

## Not found

No correctness, metadata-authority, archive-source, scope, routing-contract or
structural finding in this bounded update. No source fixes, worker writes,
Git mutations, builds, package commands or native application actions occurred
in this review. Only this report was written. Extra-fixture native diagnosis,
whole native acceptance, complete feature verification and sprint or release
readiness remain separate and uncredited here.
