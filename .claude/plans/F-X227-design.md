# F-X227, Execute the unchanged first-draft compatibility corpus in owner CI

**Status**: completed
**Sprint**: S91.1
**Size**: L
**Depends on**: F-X211, F-X221
**Test gate**: regression

## Problem

Issue316 requires actual first-draft corpus execution with per-area rates in
CI. PR333 supplies that scope and Issue314 includes it. Current CI invokes
synthetic report-contract tests, while the approved parent and child plans
still require execution. The historical95/100 result and current selected10
and documentation24 qualifications do not satisfy this remaining criterion.
The independent closure audit is retained at
`/private/tmp/S91.1-closure-readiness-audit-1/audit.md`.

Two exact original native PowerPoint inputs are missing. They affect three
positive cases. All100 script bodies, setup, assertions and support remain
intact. Exact installed Office fonts and media are available on the owner
machine, with no authenticated redistribution route to hosted runners.

## Spec reference

- docs/hld/10-bindings-spec.md, Python compatibility and owner lifetime contracts.
- docs/hld/12-testing-strategy.md, Binding tests and What CI runs.
- docs/hld/15-build-and-toolchain.md, CI job matrix and pinned external tools.
- .claude/plans/F-X211-design.md, Frozen first-draft corpus and rate.
- .claude/plans/F-X211c-design.md, immutable100 execution and CI/reporting.

## Approach

Use the existing workflow, reporting script and test entrypoints. Add no new
product module, file, dependency, feature or public API. Keep the100 frozen
script, setup, body, assertion and support hashes unchanged. Preserve the
existing17/7 documentation subset, per-area allocation, denominator and five
approved no-credit non-successes. Retain every original failed attempt and
the historical95/100 score separately.

The user approved the revised fixtures and one-job runner proposal on
2026-10-15. Create a separately reviewed external fixture
revision for the two missing native inputs. Do not overwrite the original
manifest or claim recovery of the old hashes. A retained genuine notes model
can seed a fresh native PowerPoint16.113.4 save and normal reopen. The original
transition/audio derivation is retained. Qualify actual source ownership,
notes, theme, transition, audio and preserved opaque facts before admitting
the new inputs. No substitute fonts or media, changed assertion, hidden
refetch or retry-derived best score is allowed.

Run all100 unchanged scripts on the exact reviewed source in an owner-only,
manually dispatched CI job. The original issue does not require this private
native-input job on every public PR. Existing hosted binding suites remain
mandatory. Authenticate both installed projects, source/build identities,
interpreter, tools and external inputs before and after execution. Preserve
finite process-group supervision and save/reopen assertions. Report actual
attempted, executed, qualified correct-file, deliberate divergence and
unexpected failure counts per area. Assertion exit0 alone does not establish
correct-file or required native/render qualification. Preserve all42 hard
and51 native conditions, independently requalifying changed input/output
facts and authenticating any unchanged-byte evidence that is reused.

Worker preparation qualifies the source and scoped local controls for ROOT
integration. Actual owner100 execution and its42/51 qualification at the
reviewed integrated source remain delivery gates. ROOT may publish the reviewed
sprint branch after local gates and independent source review so that the
approved one-job CI can access that source. This is an explicit integration
checkpoint ordering exception, not main merge, tagging, publication or feature
completion. F-X227 stays in progress until actual CI acceptance passes.

The existing-file implementation carriers are:

- `.github/workflows/ci.yml`: the owner-only manual job, reviewed source
  selection, paired installed bindings, guarded execution and report upload.
  Preserve every existing hosted test job.
- `scripts/sprint_workflow.py`: the actual corpus executor and CLI routing,
  immutable script/support and runtime-input authentication, finite process
  supervision, source/install custody, attempt records and per-area report.
  Read the retained external corpus rather than embedding rewritten examples
  or invoking its historical draft builder.
- `scripts/test_sprint_workflow.py`: executor and provenance controls,
  report contracts, workflow admission, upload restrictions and dev-profile
  negative controls. Synthetic controls remain unit evidence only.

The external corpus remains `/private/tmp/S91-F-X211-frozen-corpus-r2`.
Authenticate its original manifest, body manifest and executable closure.
Copy exact script/support bytes into a task-owned execution directory with
the separately approved runtime-input overlay. Keep the overlay manifest
external and distinct, with old and new identities for the two native
fixtures and actual native producer evidence. Do not change the original
manifest or recreate missing historical logs. Missing non-runtime history
must be reported separately from the executable closure.

The reporter currently admits only a dirty working-copy source. Add truthful
clean-Git admission for the actual CI commit, tree, index, tracked paths and
installed build identity. Preserve historical dirty-candidate support and
its evidence. Never mark a clean checkout dirty to satisfy the old branch.
Test both valid source kinds and rejection of mismatched commits, trees,
tracked bytes, installed products and inconsistent dirty-state claims.

Upload only the reviewed report and permitted logs. Do not upload installed
Office fonts, media or saved documents. The approved one-job local runner
still needs a demonstrated task-scoped execution boundary before launch.
Clean HOME, random labels and ephemeral registration alone do not provide
process isolation. The failed harmless sandbox canaries remain failures.
Do not register a runner until its exact trusted-job admission or isolation
mechanism is reviewed and qualified. Remove the owned runner after its one
job, with finite cleanup and no unrelated process or service changes.

For the CPU-heavy hosted binding suites, consider only job-local dev
optimization level2 with debug assertions and overflow checks explicitly
enabled. Preserve maturin develop, complete pytest directories, actual debug
save assertions and current native/WASM budgets. No release-profile switch,
suppressed failure or unmeasured speedup claim is allowed.

The original main CI Word suite is now terminal:778 passes in5982.92
seconds, with131 seconds building the extension and322 seconds preparing
font inputs. Dot-only logs do not identify individual test durations.
Retain the authenticated timing receipt at
`/private/tmp/S91-main-CI-38662802341-readonly/Word-terminal-timing-diagnosis.json`.
These observations motivate the proposal but do not qualify its performance.

## Test plan

**Test gate**: regression.

| Category | Existing carrier or control | Asserts |
|---|---|---|
| regression | Existing workflow policy tests | Actual owner/manual/ref/SHA routing, no weakening of public PR jobs, no resource upload, and rejecting wrong or missing admission guards |
| regression | Existing compatibility report tests | Exact100 identity/body hashes and per-area denominators, honest divergences, no missing assertions or synthetic acceptance |
| integration | Actual owner CI100 execution | Real reviewed paired artifacts, unchanged scripts, exact source/input custody, all required saved/render/native qualifications and actual per-area rates |
| differential | Three cases using the two revised native inputs | Separately approved source/native provenance, preserved model facts and current producer save/reopen evidence |
| regression | Hosted dev-profile controls | Explicit opt2 plus enabled debug assertions and overflow checks, refusing release substitution, removed guards or swallowed full-suite failures |
| integration | Current complete binding and repository gates | Original full suites, typing, policy, verified archives and unchanged49 hashes remain controlling |

Do not rerun green unrelated checks without a source change or an unresolved
concern. Current local qualification atb3b2163482ae42f68a66594ab2064481bd7b406b
is retained at `/private/tmp/S91.1-current-local-verification-summary-1/receipt.json`.
It is not execution of this proposed CI job or a new100 score. Final closure
still requires current full verification, clean review, primary hosted CI,
exact-SHA build-only rehearsal and the normal closed-main release gate.

## HLD impact

- docs/hld/12-testing-strategy.md
- docs/hld/15-build-and-toolchain.md

Replace the report-only CI description only after actual execution is
implemented and qualified. Keep the historical result and new execution
identity distinct. Story registration in HLD14 belongs to ROOT.

## Risk routing

External oracle comparison requires the canonical differential-testing
instructions, exact producer/tool/input identity and no fabricated original
assets. Python binding consumers require HLD10, installed source/RECORD and
typing controls. CI and release scripting require HLD15 and the existing
release boundaries. No production parser, serialization, font-selection,
unit-conversion or dependency change is proposed. Native fixture revision
and one-job local runner authority are approved. Actual runner admission
remains a technical qualification gate before registration or launch.

## Hash harness

All49 accepted outputs remain unchanged. No baseline update is authorized.

## Implementation checklist

- [x] Resolve user approval for fixture revision and the one-job runner proposal.
- [x] Approve this design and concrete existing-file implementation carriers.
- [x] Qualify new native input identities without rewriting old evidence.
- [x] Implement actual100 CI execution and per-area reporting.
- [x] Prove the local owner admission, finite cleanup and no resource upload controls.
- [x] Preserve full hosted debug assertions while qualifying any opt2 change.
- [x] Pass local scoped checks and a zero-finding independent microscope.
- [x] Qualify the current integrated product and external CI result for scoped completion. Final reviewed-commit full verification and sprint review remain mandatory close gates.

## Source availability checkpoint

Reviewed source215715b3b733307832202147243738f5b7b6279d is available
on the canonical sprint ref. Actual owner CI38702668217 and its aggregate
gate passed. Its unchanged100 first attempts, authenticated source and
paired providers,114 byte-identical outputs, three fresh revised-input
packages and separate ROOT admissions yield95 qualified files, all42 hard
gates and51 native clauses, five no-credit cases and no unexpected failure.
The successful one-job runner was removed. Three preceding refused
assignments ran no corpus cases and remain retained as failures.

Corrected primary CI38708235000 succeeded at
6d7c7da729e0ecb34252e821341f22844e37a9fd. Its full native, installed binding,
MSRV, animation, raster and fidelity consumers qualify the corrected source.
Original primary CI38696954013 remains failed. F-X224's independently reviewed
three-value propagation repairs the omitted F-X210 raster expectations without
changing product hashes or comparison thresholds.

Authenticated current integrated product qualification, actual owner CI and
corrected primary acceptance permit ordinary scoped feature completion.
This clarifies sequencing, not the acceptance criteria. Final full verification
of the reviewed sprint commit and the clean sprint review are mandatory
sprint-close gates. Exact-main full verification, publication and remote
closure remain separate later gates. Hosted debug assertions and overflow
checks remain enabled. No earlier failed result becomes a passing execution.

- Actual owner CI receipt: `/private/tmp/S91.1-F-X227-owner-ci/actual-CI38702668217-success/receipt.json`, SHA364fc8df596ac143943de2d33ac0dc30933c273621ba2692fbad60b7091e1140.
- ROOT qualification decision: `/private/tmp/S91.1-F227-qualification-routing-preparation-1/root-CI38702668217-adoption-1/root-decision.json`, SHA7a2c28c8f82e0f88f567d384177484ee4c4cde374a0f92725af3a830ded364df.
- Actual execution/source/attempt identities and historical native provenance remain distinct in the admitted packet. No historical score is relabeled as this CI run.

## Open questions

No material user question remains for this story. The user approved the
separately qualified native input revision and one-job local CI runner
proposal. Implementation may proceed. Registration and launch remain
conditional on the reviewed admission or isolation boundary actually passing
qualification. Approval does not turn failed canaries into passing evidence.
Issue360 is approved for closure after publication, with its upstream
follow-up recorded. This does not change the corpus contract. PyPI ownership
and the two trusted publishers have been verified after authentication was
restored.

Corrected primary terminal receipt: `/private/tmp/S91.1-primary-corrected-raster-acceptance-1/terminal-receipt.json`, SHAad03f29ef28fe9a5174cdb7cb30a8811c4cb25fa237eac4df1b8a0d63d388792.
