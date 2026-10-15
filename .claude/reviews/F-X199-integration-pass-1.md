# F-X199, integration, pass 1

**Reviewed**: Frozen combined staged and unstaged diff against canonical
HEADd898b446a81c462457e4ee88d6263989d3642ede, 11 files, 321 insertions and
13 deletions. Approved F-X199 contract, cited HLD04/06/10/12/14 sections,
worker source/head, durable handoff, prior production reviews, scoped actual
receipts and both integrated package measurement stages. No Cargo, UI,
remediation or writes outside this review file.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness: 0 findings. The qualified branch at
`crates/rdocx-oxml/src/table.rs:1782` admits self-closing tcMar only after
namespace resolution. Attribute iteration at1785 propagates malformed input
errors and rejects genuine attributes before setting default typed margins.
Absent edges remain absent. The expanded parser and unrelated properties are
unchanged. Exact integrated table.rs and integration_test.rs bytes equal
reviewed worker source commit2d5c311d8629c9a21cdc9329fd88e194a4ea2dfc.

Contract: 0 findings. `.claude/plans/F-X199-design.md:3` remains approved
until formal completion. Its declared archive maintenance and final combined
measurement barrier at169 are retained. HLD10's native empty-margin paragraph
at `docs/hld/10-bindings-spec.md:1700` and HLD14's F-X199 section at6852
retain the narrow namespace/raw/schema contract and current contributor
7ac2851 provenance. F-X193 HTML and F-X198 document/regression source bytes
are identical to preintegration HEAD. The current CLI production source is
also unchanged. No worker sprint ledgers, manifest, lockfile, version or
inherited roadmap-policy delta enter this integration.

Panics: 0 findings. The production branch at
`crates/rdocx-oxml/src/table.rs:1782` introduces no unwrap, expect, unchecked
index, slice or arithmetic on document input. Added unwraps are in controlled
unit inputs at3518 and the generated public regression at
`crates/rdocx/tests/integration_test.rs:535`.

OOXML: 0 findings. The existing Empty dispatch resolves local aliases and
rebindings at `crates/rdocx-oxml/src/table.rs:1673` and captures rejected raw
subtrees at1679. Writer schema boundaries at1868 retain the established
margin slot. Unit controls at3518 distinguish compact/spaced, expanded,
alias and default namespace forms. Controls at3547 preserve exact foreign,
rebound and attributed source through serialization and reparse. The
absolute-slot control at3579 retains unsupported siblings after typed edits.

Tests: 0 findings. The public test at
`crates/rdocx/tests/integration_test.rs:535` checks all21 cell texts, typed
margins and unsupported-property flags before and after reopen. Retained
production reviews record actual Base failures and54 focused table passes.
The durable handoff at `.claude/handoffs/F-X199-ready.md:17` accurately
qualifies subsequent scoped evidence. It is byte-identical to the retained
worker handoff and extracted /private/tmp copy. Retained receipt log hashes
were independently checked for OXML, native facade, native recovery, hash
and policy. OXML639 units and1 doctest passed. Original full facade35529
exited101 after515 library and364 integration passes with one unchanged
300-second consumer timeout. That failed fail-fast command is not relabeled
successful. Unchanged exact consumer recovery passed separately, followed
by2325 regression passes with7 ignores and2 doctests under the documented
8MiB CI stack. Policy55950 has actual exit0 and terminal141 tests, OK with2
skips. The patched publication receipt has actual exit0,22 completed package
verifications and no oversized archives, preserving its earlier dirty
roadmap-HEAD source provenance. These are worker scoped facts, not final
integrated full sprint verification or sprint review.

Structure: 0 findings. The complete production change is the existing private
parser branch at `crates/rdocx-oxml/src/table.rs:1782`. No public signature,
trait, generic, dependency, crate, module, implementation file or test binary
is added. Tests remain in the existing inline module and public integration
entrypoint. HLD edits are confined to the approved semantic sections.

## Integrated measurement reconciliation

The script diff changes only rdocx and rdocx-oxml tuples at
`scripts/readme_doctests.py:405` and410. `README.md:42` and
`crates/rdocx-oxml/README.md:20` match actual before-row observations
3529814/50787392/37 and452312/2809322/32 respectively. The adjacent F-X196
CLI row at script406 remains76692/347960/8. Host provenance remains the
existing M5 platform, and compression tolerance at394 remains64 bytes.
Historical performance rows and inventory/validator code are unchanged.

Independent read-only authentication checked all eight retained archives
and package logs against SHA256 and remeasured normalized metadata/member
counts. Before-row network receipt:
/private/tmp/S91-F-X199-integrated-before-row-network/receipt.json.
Both observations repeat exactly. After-row receipt:
/private/tmp/S91-F-X199-integrated-after-row/receipt.json.
Both final rdocx archives measure3529815/50787392/37 and both final OXML
archives452311/2809322/32. Each differs by one compressed byte from its
carrier within unchanged tolerance. All34 ordinary rdocx members and29
ordinary OXML members, including both current README files, equal the actual
canonical source bytes. Both packages are below10MiB. Generated VCS/manifest/
lock metadata retain package-time provenance and are not misrepresented as
current repository files or a final integrated commit SHA. The earlier
worker measurements remain distinct historical evidence.

## Verification boundary

This review authenticates the frozen integration and actual retained scoped
and combined-package evidence. It does not run Cargo, full workspace gates,
final sprint review, publication or GitHub closure. Formal completion and
handoff consumption remain integrator actions. The pass ends with this file.
