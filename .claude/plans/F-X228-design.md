# F-X228, Qualify faster CI without weakening gates

**Status**: completed
**Sprint**: S92
**Size**: M
**Depends on**: none

## Problem

Test and MSRV run broad workspace suites with an unoptimized development
profile at `.github/workflows/ci.yml:160` and
`.github/workflows/ci.yml:710`. The presentation corpus invokes `cargo run`
without `--release` at `scripts/pptx_ssim_harness.py:135`. Presentation routing
selects every crate at `.github/workflows/ci.yml:107`, including crates outside
the renderer's dependency closure.

PR376 by Hadrien Mary proposes job-local development optimization, release
corpus rendering and narrower routing. Its independently refreshed head on
2026-10-15 is `4cd93af075ddfd2ad6ccd394d7d91fc26e1fa571`. Donor
run38631217425 has successful Test, MSRV and Presentation fidelity jobs, but
failed Python bindings (rdocx) and CI gate. Donor measurements and individual
successful jobs do not qualify the integrated sprint source.

## Spec reference

- `docs/hld/14-development-backlog.md`, "F-X228, Qualify faster CI without weakening gates".
- `docs/hld/12-testing-strategy.md`, "What CI runs", including filtered job selection and aggregate failure propagation.
- `docs/hld/15-build-and-toolchain.md`, "CI job matrix", specifically pull-request Python bindings, owner-only compatibility, pinned native font setup, corpus-test runtime and Presentation fidelity.
- `docs/hld/15-build-and-toolchain.md`, "Deterministic rendering".

## Approach

1. Reconcile the three-file PR376 patch onto current sprint source, preserving
   attribution to Hadrien Mary. Do not import its base wholesale.
2. Add job-local `CARGO_PROFILE_DEV_OPT_LEVEL: "1"`,
   `CARGO_PROFILE_DEV_DEBUG_ASSERTIONS: "true"` and
   `CARGO_PROFILE_DEV_OVERFLOW_CHECKS: "true"` to Test and MSRV. Preserve
   existing steps, commands, flags, test filters, native installers and pins.
   Do not change workspace-wide Cargo profiles.
3. Add `--release` to the existing
   `run_rust_renderer(corpus: Path, output: Path, manifest: Path) -> None`
   command. Preserve renderer arguments, deterministic font selection, corpus
   verification, independent oracle and failure propagation.
4. Narrow presentation crate paths against the actual current Cargo graph and
   render example. The current local dependency and dev-dependency closure is
   `oxml-chart`, `oxml-core`, `oxml-drawing`, `oxml-layout`, `oxml-media`,
   `oxml-opc`, `oxml-pdf`, `oxml-sml`, `rpptx`, `rpptx-layout`, `rpptx-oxml`
   and `rpptx-render`. Route each corresponding `crates/<name>/**` path.
   Preserve workflow, Cargo, lockfile, toolchain, configuration and existing
   corpus/tool routes. Add `scripts/golden_png_harness.py`, imported by
   `scripts/pptx_ssim_harness.py:19`. Preserve the corpus manifest included
   directly by `crates/rpptx/examples/render_deck.rs:15`.
5. Extend existing regression carriers only. Assert profile isolation,
   preserved debug checks, release command construction, dependency routing
   and irrelevant-path skips. Mutations must reject missing required routes
   and weakened profile safety settings. The profile regression must reject
   the old workflow, and the routing regression must reject the old broad
   presentation filter.
6. Derive the required local dependency set from current manifests in the
   existing regression carrier, accounting conservatively for optional,
   default and example dev dependencies. Assert that every member remains
   routed. A new renderer dependency must fail until its route is included.
7. Preserve binding optimization2, both complete binding suites, all104 font
   cases, independent inputs, HarfBuzz14.4.0, Poppler and LibreOffice pins,
   animation stack settings, owner-only identity conditions and fail-closed
   aggregation. No suite becomes compile-only, filtered, ignored or optional.
8. Measure same-source dev and release rendering on the authenticated corpus
   with deterministic fonts. Compare every PNG and render manifest byte for
   byte. Retain source and corpus identity, command, profile, wall time,
   output hashes and runner conditions. This profile-equivalence evidence is
   distinct from the independent LibreOffice comparison.
9. Obtain actual hosted execution before completion. After all local checks
   and independent microscope pass, use the canonical external CI source
   availability checkpoint. Keep the feature approved and in progress until
   GitHub evidence binds the reviewed canonical sprint SHA. The checkpoint
   grants no owner-only runner admission, main merge, tag or publication.
10. Record hosted job and relevant step durations, separating compilation
    and execution when logs permit. Compare with a comparable actual
    predecessor run and state source, cache and runner limitations. Do not
    infer speedup from settings or donor claims.

## Rejected alternatives

- Import PR376 wholesale. Its base does not establish current integrated controls.
- Replace MSRV execution with compile-only checks. This removes required runtime coverage.
- Change global profiles. This story authorizes job-local optimization and preserves binding optimization2.
- Claim broad `oxml-*` and `rpptx*` patterns are the exact closure. They also include unrelated support, CLI and binding crates.
- Relax performance limits or remove failures. Optimization must retain every existing gate.

## Test plan

**Test gate**: regression.

| Category | Test | Asserts |
|---|---|---|
| regression | `test_native_ci_optimization_preserves_debug_checks_and_binding_profile` in `scripts/test_sprint_workflow.py` | Test and MSRV have optimization1 and explicit debug/overflow safety. Bindings retain optimization2. Old profile and weakening mutations fail |
| regression | `test_presentation_fidelity_routes_the_renderer_dependency_closure` in the existing policy module | All current renderer, example, root configuration and shared harness inputs select presentation fidelity. Unrelated Word, binding and CLI paths do not. Old broad routing and missing dependency mutations fail |
| regression | Existing aggregate, native-tool, binding and owner-job mutations | Missing prerequisites, swallowed failures, weakened owner identity and changed binding contracts remain rejected |
| regression | `test_rust_renderer_uses_release_without_changing_arguments` in `scripts/pptx_ssim_harness.py` | Mocked subprocess gets release Cargo command, exact example arguments and `check=True`. Old dev command fails |
| regression | `python3 -m unittest scripts.test_sprint_workflow` | Complete repository workflow and release policy module passes |
| regression | `python3 scripts/pptx_ssim_harness.py --self-test` | Existing harness self-tests and new command regression pass |
| golden | Same-source deterministic dev/release corpus comparison | Every PNG and render manifest is byte-identical across the pinned50-deck corpus |
| integration | Current reviewed-SHA hosted Test and MSRV | Both full workspace suites execute successfully with pinned native tools and8MiB stack |
| integration | Current reviewed-SHA hosted Python bindings (rdocx) and Python bindings (rpptx) | Both complete isolated suites pass with original optimization2 and native/oracle controls |
| differential | Current reviewed-SHA hosted Presentation fidelity and Word fidelity | Existing independent oracle, corpus completeness and unchanged thresholds pass |
| regression | Current reviewed-SHA hosted CI gate | Every selected required job succeeds and owner-only compatibility stays skipped on the ordinary PR route |

Run `/verify --scoped F-X228`, including formatting, unchanged hash harness,
prose, generated-adapter drift and the complete policy module. This
workflow/script change has no changed Rust crate requiring a separate crate
test run. Actual hosted Test/MSRV execution is a declared feature gate,
distinct from the final integrated sprint `/verify --full`.

Hosted evidence retains workflow run ID, job IDs, checked source SHA,
conclusion, timing and artifact references. Donor results, older SHAs,
local mocks, skipped selected jobs and pending native facts supply no current
success credit. Preserve every other selected suite in the existing CI gate.

## HLD impact

- `docs/hld/12-testing-strategy.md`
- `docs/hld/15-build-and-toolchain.md`

## Risk routing

- External oracle comparison. Read `.claude/skills/differential-testing.md`.
  Preserve exact LibreOffice26.2.5.2 build identity, Poppler26.01.0 and corpus
  pins. Execute the actual hosted fidelity gates at their existing150DPI and
  existing declared metrics. Presentation retains its0.95-on80-percent trend
  and independent completeness failure contract. No threshold changes or
  baseline regeneration are authorized. Our render side uses deterministic
  bundled fonts. Record and triage any oracle disagreement rather than
  masking it. Do not add an oracle to published dependencies.
- Same-source dev/release output comparison is golden evidence, not an
  external correctness oracle. It requires deterministic fonts and exact PNG
  and manifest equality. An unexplained difference stops acceptance.
- No parser, public API, dependency edge, feature flag, version, new product
  file or layout implementation is introduced. Reading the existing graph
  does not change the dependency graph. The normal scoped floor still runs.

## Hash harness

Expected to be unchanged. Run `python3 scripts/hash_harness.py --check`.
No baseline update is authorized. Presentation dev/release PNGs and manifest
must also remain byte-identical. An unexplained profile-dependent output
delta blocks completion.

## Implementation checklist

- [x] Refresh PR376 head, patch, comments and checks before extraction.
- [x] Retain contributor attribution and separate donor provenance from acceptance.
- [x] Add job-local optimization1 and explicit debug/overflow safety.
- [x] Switch the existing corpus command to release.
- [x] Narrow presentation routes to the authenticated existing closure.
- [x] Route the imported golden PNG helper.
- [x] Add regressions and weakening mutations in existing carriers.
- [x] Preserve all binding, native-tool, animation and owner-dispatch controls.
- [x] Complete deterministic dev/release output and timing comparison.
- [x] Pass local scoped verification and independent zero-finding microscope.
- [x] Use the authorized source availability checkpoint for actual hosted CI.
- [x] Bind successful required jobs and measured durations to reviewed source.
- [x] Update exactly the two named HLD files.
- [x] Complete the normal lifecycle after actual external CI acceptance.

## Open questions

No material product question remains. Operational assumption: use the
canonical sprint source availability checkpoint and the existing ordinary workflow_dispatch route with owner_compatibility=false
for the reviewed workflow. Owner-only native runner admission is outside this
story. The checkpoint must follow the run-sprint admission rules.

Timing assumption: acceptance requires measured current-head results and
unchanged correctness. No minimum percentage speedup is specified. Report
observed results and comparison limits without inventing a threshold.

## CI routing

Use the existing ci.yml workflow_dispatch at the reviewed sprint/s92 ref with
owner_compatibility=false. Bind source_sha and workflow_sha to the authenticated
remote sprint SHA and source_ref to refs/heads/sprint/s92. Required owner-only
runner_label and config_sha inputs use unused-public-hosted-ci sentinels.
The owner corpus must remain skipped. No PR exists for sprint/s92 at design time.
Authenticate the actual checkout SHA, selected jobs and aggregate result.

## Local source availability evidence

Independent review `.claude/reviews/F-X228-all-pass-1.md` reports0 defects,
0 smells and0 nitpicks. All ordinary scoped checks passed: formatting,
prose,26 generated adapters, the unchanged49-entry hash harness and the full
policy module,182 tests in130.566s with180 passes and two platform skips.
Harness self-tests passed eight cases with two existing corpus-evidence-only
skips. Those skips grant no corpus or external acceptance credit.

The independently inspected bounded four-process profile driver executed all
50 pinned decks and421 source slides with the actual worker-built dev and
release examples. Every PNG and the canonically assembled render manifest
are byte-identical. Each per-deck row, slide count, output and zero-drop field
was checked. All166 tracked renderer inputs and all50 corpus hashes were
verified before and after. The example creates an independent package and
render context for each deck, so bounded per-deck scheduling changes no
cross-deck state or coverage. The earlier interrupted sequential attempt is
retained as partial and uncredited.

Evidence directory:
`/private/tmp/S92-F-X228-profile-parallel-evidence`.
Receipt SHA-256:
`6b6f4d7ccdcd56d3bcafed037eedd52825ff5b1d927f3aff2990ca8670363ae5`.
Driver SHA-256:
`de5de6c67a8b0ff02ac09c7ea9c7887305780f486c123b5637bf34fb36ad66de`.
Both render manifests SHA-256:
`34f0c7913bf69c49d9ac32d7396cd945e62310a7b6ae8b5e6630cc469396575f`.
Actual build commands, toolchain identities, binary hashes, per-deck commands,
per-profile PNGs and logs are retained there. Diagnostic parallel wall times
were568.141s dev and6.802s release. Concurrent workloads and cache conditions
prevent an isolated or hosted speedup claim from these observations.

The full policy log is `/private/tmp/S92-F-X228-policy-final.log`, SHA-256
`ea9f20512a2dd08a8a0ee15554733abf2218545b756e12e684dc0871c0f5c3b0`.
The hash log is `/private/tmp/S92-F-X228-hash.log`, SHA-256
`88f62f8bfd75b45c4c881173a5f4c7d842165e422faf1f1ae9857262363ed52f`.
Prior sandbox/offline attempts remain failed or interrupted in progress notes.
The one-line roadmap expectation correction mirrors the parent-owned S92
opening scaffold fix and preserves exact-set and one-placement coverage.

Local checks and actual hosted prefix acceptance pass. The plan stays approved
and the feature stays in progress until normal integrated completion records.
This hosted result qualifies its exact source, not later integrated changes.

## Actual hosted acceptance

[Run38755615219](https://github.com/tensorbee/rdocx/actions/runs/38755615219)
completed successfully at exact reviewed source
`638cf3e4d8580fd45195f065531328f4891cabd1`, ordinary workflow_dispatch,
with owner_compatibility=false. All18 required jobs succeeded. CI gate
`116522544812` succeeded. Owner corpus `116493120658` was skipped. All9
change filters selected their jobs and17 actual checkout logs print the
reviewed SHA. ROOT independently authenticated the terminal server result.

Test and MSRV each record6465 passed,0 failed and71 ignored in their complete
workspace command. Full Python suites record778 Word and854 PowerPoint passes.
Native job environments retain optimization1 with explicit debug assertions
and overflow checks. Binding jobs retain optimization2 with those checks.
Ignored cases and the skipped private owner job receive no new credit.

All fidelity artifact members are byte-identical to primary main predecessor
run38722351979 at80117123. Presentation covers50 decks and421 slides with0
dropped shapes. Its12.59percent diagnostic trend coverage is unchanged and its
trend target remains false. Word broader trend remains0percent while its
mandatory multiscript gate passes5/5. Configured gate success does not claim
either broader trend target was met.

| Job | Current job ID | Main predecessor seconds | Current seconds |
|---|---|---|---|
| Test | 116493270159 | 4281 | 3417 |
| MSRV1.93 | 116493269978 | 2678 | 1888 |
| Presentation fidelity | 116493270203 | 2459 | 1594 |
| Python bindings rdocx | 116493269887 | 2570 | 1620 |
| Python bindings rpptx | 116493270035 | 1548 | 1190 |

The compared sources have identical product crates and Cargo inputs. Workflow
event, runner instances, cache restoration and queues differ. These are actual
observed durations, not isolated causal speedups or donor timing claims.

Raw metadata, full logs, checkout proofs, matching-step timings, cache records
and artifacts remain under `/private/tmp/S92-F-X228-hosted-38755615219`.
Authenticated acceptance SHA-256:
`31071cb1dab5779d17b0016a116e537ac21da2043bfeeb5594793b92198a66d1`.
Terminal receipt SHA-256:
`f16776b03457be4661fbffb3c2de86d4e7eefbd84a260ebccefcfb6d8e7b79f5`.
Timing comparison SHA-256:
`1f40d0f866801327f28ca0f82ce255d2bc015e8c62599b9b95807da750b1047d`.
Evidence index SHA-256:
`17a6abb7dda04ff295e3d9957da018eb758565aafa071c0d773b25b4f1b8d746`.
Final integrated S92 full verification and sprint review remain due.
