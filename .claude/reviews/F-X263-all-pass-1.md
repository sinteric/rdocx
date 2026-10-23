# F-X263, all aspects, pass 1

**Reviewed**: Complete change from Base `2ddbb13e18e93111dd0ea6e66328402805c9c1a6` through HEAD `94816e4e929e52fb8d3b2412830a8ee8a739e730` and the frozen working diff on `sprint/s95.3`. The combined diff contains 76 files, 471 insertions and 273 deletions. The committed process refinement contains seven files, 34 insertions and five deletions. The working diff contains 69 files, 437 insertions and 268 deletions. Counts exclude this report.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Clean feature implementation review at the inspected source. This does not complete F-X263, accept external CI, authorize a push or approve publication. Remaining local checks still gate the source-availability checkpoint.

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Scope and contracts

Read `CLAUDE.md`, `.claude/WORKFLOW.md`, `.claude/commands/microscope.md`, the approved F-X263 design, its existing progress record, HLD10 Packaging and CI, HLD15 Packaging, Publishing, Release process and CI contracts, and the actual `/release` and `/verify` commands. BACKLOG and CURRENT_SPRINT retain F-X263 as in-progress with all seven providers done. The provider-plan edits distinguish historical checkpoints from accepted qualification without deleting the earlier failures or unavailable-input limitations.

The reviewed delta includes all manifest, lock, Python metadata, README, measurement catalog, CI/WASM assertion, policy-test, changelog, HLD and delivery-state changes. It also includes the separately committed binding-process instructions and all three generated adapters. Their recorded source digests match the canonical files. The progress record acknowledges the process revision and retains the failed typing observations. No gate or publication authority is removed by that refinement.

The release-note comparison uses published baseline `57f4d7438483e109cb8e45097f3498fe1c223b23`, independently confirmed as the peeled local identity of both `v0.16.2` and `rpptx-v0.14.2`. The intervening history includes S95.11, S95.2 and S95.3. This is broader than the implementation Base and is the appropriate notes boundary. The complete changelog tail beginning with `v0.16.2` remains byte-identical to the Base copy.

## Carrier and release-note review

Parsed all 27 workspace manifests, the 24 internal dependency pins and the lockfile. The exact seven publishable Word crates resolve to 0.16.3. The exact fifteen publishable shared/PowerPoint crates resolve to 0.14.3. Ten inherited carriers, sixteen explicit target carriers and 26 changed lock records align. Both Python projects match their family. Binding/support/WASM publication exclusions remain intact. `rpptx-wasm` remains 0.12.1. Every registry-sourced lock record is unchanged, including external `gif` 0.14.2.

Inspected the actual family validator at `scripts/sprint_workflow.py:820`, rather than treating target-string assertions as sufficient evidence. It checks the selected publishable set, effective manifest versions, internal pins, lock membership and Python metadata. The target regression at `scripts/test_sprint_workflow.py:31` rejects the old prepared source. The retained target RED reports 0.16.2 versus 0.16.3, and the final target GREEN is retained separately.

The mutation controls remain meaningful. `scripts/test_sprint_workflow.py:6501` independently changes Python metadata, publication eligibility and lock membership and expects distinct failures. WASM version/tag mutations at line 6123 assert that the mutation actually changes the input. The current archive-version mutation at line 5640 changes the present 0.16.3 measurement row, while the historical performance-version control remains 0.14.0. Existing exact historical contribution inventories remain separate from the new family inventories. A matching literal alone is not used to establish runtime or artifact compatibility.

Compared the family notes and exact contribution sets with `.claude/scratch/S95.3-fx263-contribution-inventory.json`, the prior-release contribution intake, GitHub issue intake and pinned PR intake. Word includes Issues 360, 385, 387, 388, 389, 390, 391, 398 and 400, and PRs 393, 395, 396, 397, 399 and 401. PowerPoint includes Issues 385, 388, 389 and 392, and PRs 393, 394 and 397. The receipts authenticate hadim as the reporter/contributor and retain the candidate heads and functional-parent distinctions. Notes credit reviewed adaptations without claiming unchanged candidate merges or counting archive-only commits as features.

The five required sections exist for each family. Compatibility text names boxed BodyContent constructors, nullable CommentRef resolution, hidden ResolvedNumbering metadata, exhaustive ValidationIssue matches, placeholder iteration and percentage rounding. It preserves the native repeat/last-item differences, embedded-font limits, private unreduced Issue 390 exclusion, later form boundaries and advisory SSIM miss. No full API compatibility, native default parity, private-input closure or advisory-target success is claimed. Earlier shipped APIs are not recounted as new outcomes.

## Installed binding review

Traced native registration, package re-export and both export lists for ContentControlTypePayload. The added stub export at `crates/rdocx-py/python/rdocx/_rdocx.pyi:76` matches the existing runtime path. The range declaration at line 1410 matches the existing callable implementation at `crates/rdocx-py/src/document.rs:11929`, including its integer result and half-open range behavior. No runtime range body changed.

The private Never argument and class return at `_rdocx.pyi:2210` preserve factory-only construction while allowing static factories to retain their actual type. The public typed use at `crates/rdocx-py/tests/typing_smoke.py:644` calls the date factory, asserts the concrete result and passes it to the actual public setter. It also checks omitted and explicit schema sequences and range-removal typing. The retained old declaration fails with the concrete callable-without-date diagnostic, so this discriminator does not merely compare stub names.

The PyO3 edit at `crates/rdocx-py/src/document.rs:8630` changes the visible text signature to an empty tuple while retaining `Vec::new()`, argument conversion and the entire mutation/refusal body. The runtime assertion at `crates/rdocx-py/tests/test_core.py:10086` rejects Ellipsis. The retained old-wheel failure shows that exact assertion failing. Existing store tests retain explicit schema values, malformed XML and atomic refusal checks.

The PowerPoint smoke at `crates/rpptx-py/tests/typing_smoke.py:680` requires float, Length or None and exercises percentage, point and clearing assignments. Actual getter/setter branches in `crates/rpptx-py/src/text.rs:1362` agree. The installed suite also contains discriminating value/XML checks at `crates/rpptx-py/tests/test_documented_examples.py:11717`: point values return Length and store buSzPts, percentages store buSzPct, clearing returns None, and invalid values/booleans refuse without mutation. This is stronger than accepting a broadened annotation alone.

## Evidence authentication and acceptance applicability

Consumed observations read-only. No Cargo command, pytest run, large gate or native UI action was performed for this review.

Independently rehashed all 325 entries in `.claude/scratch/S95.3-fx263-qualified-source.json`, with zero drift. Its SHA-256 is `58701d3b75422a7e84680951b8c2de20999bc5908a64a2bc1409f51202213ec7`. It explicitly identifies working source, not a clean Git tree. Rehashed all 53 copied observations against `release-preparation/checkpoint-artifacts.json`, with zero mismatches.

Authenticated `/tmp/s95.3-fx263-local-artifacts.json` against both actual wheel files, their complete hashed RECORD entries and packaged Python source/stub bytes. Word wheel SHA-256 is `62b01010aa1855424ac8cc55d927354571168eba6ac2c44c6ad4c11666ac171c`. PowerPoint wheel SHA-256 is `0ef39e438f4ee32d1e1f1950a279f7bc4020e4b2c7694faf5a47f222e42f60fd`. The authentication script checks metadata, Markdown MIME parameters, README bodies and cp39-abi3 macOS arm64 tags.

Rehashed the imported native files referenced by all eight full, priority and typing receipts. Word native SHA-256 is `7cea402715b1a03aa6720803c5d15e75474a78b67bac6f82e4addc9ee5cd801d`. PowerPoint native SHA-256 is `b2526f8628cabad38faa1673d87ee464210d4d1f3c294fa72d327964eda5ce06`. Both interpreter installations retain valid complete hashed RECORD entries, 17 Word and 19 PowerPoint entries each. Imported paths are inside the fresh environments. The priority inventories contain exactly 42 distinct Word and 44 distinct PowerPoint nodes, matching the workflow inventory and actual receipt commands.

| Criterion | Inspected observation and current disposition |
|---|---|
| A1 | Actual target RED and final GREEN retained. Parsed current carriers, exact family validators and meaningful negative controls agree. |
| A2 | Baseline-to-current notes and authenticated family inventories reviewed. Historical notes unchanged. Future rendered release-body and notification checks remain release obligations. |
| A3 | Both 22-package no-verify measurement logs and the README inventory observation support the catalog only. Genuine clean-tree, locked, locally patched publish dry run with 22 successful Verifying records remains pending. |
| A4 | Fresh ordinary local wheels and CLI build/version receipts retained. Full installed suites report Word 1063 and PowerPoint 1028 passed, both child exits zero. Exact priority suites pass on Python 3.9.25 and 3.12.13. Strict mypy reports 9 Word and 11 PowerPoint files passed, and installed stubtest reports 8 and 10 modules passed. Source-package typing bytes match the installed wheel bytes. No typing gate is waived. |
| A5 | Actual hosted build-only run remains pending. Required outcome is 27 deciding jobs passed, four publication jobs skipped and 26 unique raw payloads at the reviewed pushed SHA. |
| A6 | Downloaded hosted artifact validation and fresh installed qualification remain pending. Local wheel evidence does not substitute for it. |
| A7 | Latest hash log reports 49 matches. Latest deterministic pixel log reports seven buffers matched with pinned Poppler 26.01.0 at 150 DPI. Both WASM configurations and downstream native checks remain pending. |
| A8 | Feature remains approved/in-progress. No push, merge, tag, upload, release or publication approval is recorded or performed by this review. |

The final policy log reports `Ran 192 tests in 142.314s` and `OK (skipped=2)`. Its expected negative-mutation diagnostics do not change that overall outcome. README compilation and 22-archive inventory pass. Retained deny, formatting, prose and adapter observations are separate from the pending full acceptance result.

The first workspace log remains a failed attempt caused by pinned soffice being absent from PATH. The corrected viewer attempt has its own `/tmp/s95.3-fx263-workspace-viewers.log`. It was still emitting successful test cases at the final inspection and had no overall completion result. Workspace acceptance, downstream clippy, minimal layout, WASM and docs are therefore not called passed. Earlier signature, typing, release-note guard and metadata-probe failures remain distinct from their corrected observations.

## Pending obligations and exclusions

Finish and inspect the actual native batch outcomes, retain any failures, and complete the genuine 22-package verified dry run at the required clean checkpoint. Source availability requires all applicable local checks and this reviewed source to be reconciled into a clean canonical sprint ref. This report does not execute that handoff or push.

Hosted build-only execution, full family payload validation, downloaded-wheel qualification and subsequent completion metadata remain required. They block feature completion and publication. Honest pending final or external riders are not implementation defects at this review boundary. Final integrated verification, authorized independent sprint review, close-sprint provenance, mutable registry/publisher checks and separate approval per family at reviewed clean main remain due.

Excluded original provider product implementation audits, rerunning native authoring, original reporter private input, historical review rewrites, final sprint review pass 4, external CI acceptance and publication. Historical native and source receipts were consumed within their stated limits. This review does not extend their coverage.

## Not found

Correctness, contract, panics, OOXML, tests and structure were checked with no findings in the complete F-X263 delta. No new runtime trait, generic, wrapper, module, dependency, XML writer behavior or panic path was introduced. The only binding implementation edit preserves the underlying default and mutation body. No unexplained output delta or weakened acceptance gate was found.
