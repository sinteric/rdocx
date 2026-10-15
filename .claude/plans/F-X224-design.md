# F-X224, Repair hosted release tool and stack routing

**Status**: completed
**Sprint**: S91.1
**Size**: M
**Depends on**: F-X221

## Problem
Actual main CI38662802341 exposed an animation golden invoked without the approved RUST_MIN_STACK=8388608 and an MSRV font consumer without exact hb-subset14.4.0. The current fractional sprint grammar is accepted by scripts/sprint_workflow.py:76 but rejected by the release-assets guard in .github/workflows/wheels.yml:568. No release has been published. The closed S91 tag cannot be moved to repair these new source bytes.

## Spec reference
- docs/hld/15-build-and-toolchain.md, "CI job matrix", "Toolchain pinning", and "Release process": native stack budgets, exact independent tools and closed-main publication provenance.
- docs/hld/12-testing-strategy.md, "What CI runs" and "Binding tests": mandatory source/oracle qualification, failed-attempt retention and full integrated gates.
- .claude/commands/release.md, mandatory exact-main preflight and immutable tag boundaries.

## Approach
Amend only existing workflow/tool/test carriers. Supply the existing8MiB contract to the actual animation command without globally masking WASM stack defects. Reuse the already approved HarfBuzz14.4 source/hash/build protocol through an existing carrier, authenticating actual tools before the MSRV consumer executes. Do not trust an unpinned package-manager version or fabricate portable binary reproducibility. Allow exactly ^s[0-9]+(\.[0-9]+)?$ in the release sprint-tag check, retaining exact current origin/main SHA, exactly-one tag and annotated object guards. Keep all family/tag publishers unchanged.

After F-X225 integration, root requalifies existing0.14.1/0.16.1 metadata and source-built artifacts, notes/current-contract accuracy, selected family manifests and dependency order. Source changes require fresh affected wheel/native/package custody. Do not carry old S91 execution across changed production bytes. Existing release commands, clean review, hosted exact-SHA build-only rehearsal, close and final-main gates remain controlling.

## Concrete existing-carrier HarfBuzz route

Extend only the existing private `_fx195_prepare_inputs` in
`crates/rdocx-py/tests/test_rendering_threads.py:2047` with a named
`native_tools_only=False` option. Its two new consumers are the existing
native Test and MSRV jobs. Keep the current binding callers and their full
independent input preparation unchanged.

The native branch returns immediately after the existing source-qualified
HarfBuzz producer and tool checks at lines2088 through2135. That existing
producer authenticates archive SHA256
2357ed966c6ced7bfa720b0640c0231065af01158fbea215093ffa15aed44371,
bounded extraction and the reviewed documentation-link exception, Meson
release/static options, the actual four-job build and all three actual
`hb-shape`, `hb-subset` and `hb-view`14.4.0 identities. Reuse that sequence
without copying it or adding another producer function.

Before the early return, rehash the three tool binaries and the existing
tracked source body/mode map. Write the same owned output directory's
`receipt.json` with an explicit native-tools-only status, source guards,
archive identity, actual command exits and stdout/stderr hashes, actual tool
paths and binary hashes. The receipt must not claim generated fonts, binding
installation, CLI discovery, Rust compiler qualification, path-renderer
qualification or104 acceptance. Publish only the qualified existing
`harfbuzz-build/util` directory through `GITHUB_PATH` when present, so later
native `Command::new("hb-subset")` resolves the actual qualified producer.
No package-manager HarfBuzz or PATH shim can substitute for that binary.

Expose this mode through an existing-carrier `--native-tools-only` CLI flag.
Keep `--prepare-fx195-inputs OUTPUT` required in both modes. Binding mode still
requires exactly one existing `--family rdocx|rpptx`. Native-tools-only mode
requires no family and refuses an accompanying family, avoiding a mixed-scope
receipt. Preserve `--harfbuzz-archive` as the existing hash-checked input
option. The early return precedes bundled-font generation and the Cargo CLI,
rdocx development-artifact and rustc1.97.1 checks at lines2298 onward. Thus
MSRV1.93 remains the actual native consumer compiler, with no installed Python
binding prerequisite or silent upgrade to the current Rust compiler.

In each existing Test and MSRV job, prepare a separate owned Python venv and
install exact pytest9.1.1 and Meson1.9.1, which the existing carrier imports or
executes. No product package or Pillow is needed for this early branch. Use
the existing pinned Python setup action with3.12.9 if an interpreter is made
explicit, rather than inheriting a new unreviewed action. Add only the Linux
Cairo and GLib development prerequisites needed by the unchanged Meson recipe
to the current native-tool apt setup. Existing freetype, compiler, pkg-config
and Ninja prerequisites are retained. Keep this before every native font
consumer, not only before the MSRV full suite.

The concrete native invocation in both jobs is:

```bash
native_venv="${RUNNER_TEMP}/native-font-tools-venv"
python3 -m venv "$native_venv"
"$native_venv/bin/python" -m pip install pytest==9.1.1 meson==1.9.1
export PATH="$native_venv/bin:$PATH"
"$native_venv/bin/python" -I -B crates/rdocx-py/tests/test_rendering_threads.py \
  --prepare-fx195-inputs "${RUNNER_TEMP}/native-font-tools" \
  --native-tools-only
```

The existing source-faithful native control is
`rdocx::svg::tests::fx195_variable_svg_paint_matches_independent_static_instances`
at `crates/rdocx/src/svg.rs:2233`. It requires actual14.4.0 and independently
checks the original source and static font data. Qualify this exact original
control on both current Rust and1.93 before claiming the tool route fixed.
Do not remove or ignore it. Retain the full Test and MSRV suites afterward.

## Exact native stack and release-tag scope

Set `RUST_MIN_STACK: "8388608"` on both existing named deterministic animation
commands in the Test job and Presentation fidelity job. Their full selectors,
expected two-machine manifest, rendering thresholds and follow-on golden-PNG
and all-slide SSIM gates remain unchanged. Keep the already-present8MiB
workspace-suite environment. This native test budget does not set Node flags,
WASM linker flags, a global runtime stack minimum or any F-X225 parser policy.

Only the existing release sprint-tag shape check changes from
`^s[0-9]+$` to `^s[0-9]+(\.[0-9]+)?$`. Keep exact current
`GITHUB_SHA == origin/main`, `git tag --points-at` collection, exactly one
nonempty sprint tag and `git cat-file -t ... == tag` before publication
archives. Add executable existing-entrypoint controls using temporary local
Git repositories for accepted annotated `s91` and `s91.1`, and refused empty,
uppercase, suffix, nested-fraction, duplicate matching tags, lightweight tag
and wrong-main cases. Do not merely test a second independent regex that
could drift from the workflow shell. Keep existing mutation controls and
extend them for removed count, annotation and main guards.

## Deciding scoped controls

- Preserve the actual main animation stack overflow and MSRV hb-subset ENOENT
  as distinct defining failures, with the original exact logs.
- Exercise native-only preparation without installed rdocx or rpptx, requiring
  actual source/tools and source-before/after qualification. Prove early exit
  does not invoke Cargo, renderer compilation or product imports.
- Existing policy entrypoint rejects missing, commented, moved-after-consumer
  or failure-swallowed tool setup, wrong HB source/version and absent native
  golden8MiB routing. Binding mode missing family still refuses, and all full
  binding preparation/compiler/input assertions remain in place.
- Run the exact original animation golden with8MiB and the native variable SVG
  control under both compiler lines. Do not narrow expectations or claim a
  failed wrapper passed. Then run affected full policy and required native,
  binding104 and native-eight gates at the integrated current source.
- All current source/RECORD, package, full verification, clean review,
  build-only rehearsal and publication preconditions remain mandatory. F-X225
  owns any actual changed WASM production source and its own proof.

## Rejected alternatives
- Editing main, retagging s91, or repurposing the closed S91 runstate.
- Installing an arbitrary HarfBuzz version, skipping its exact native consumer, or loosening glyph/paint or package thresholds.
- Broadening tag grammar to arbitrary names or removing SHA/count/annotation checks.

## Test plan

**Test gate**: regression.

| Category | Existing entrypoint or control | Asserts |
|---|---|---|
| regression | Actual38662802341 animation/MSRV logs | Original failures remain immutable and correctly attributed |
| regression | Existing deterministic animation golden | Same expected frames/manifest pass with approved8MiB budget |
| regression | Existing failing hb-subset native control and full MSRV suite | Actual14.4.0 source/tool identity, independent glyph facts, no skipped acceptance |
| regression | scripts/test_sprint_workflow.py | Exact guard accepts s91/s91.1 and rejects empty, nested fractions, suffixes, uppercase, duplicate, lightweight and wrong-main cases |
| regression | Existing complete policy suite | Operative setup precedes consumers, missing/commented/swallowed steps rejected |
| integration | Existing family/package/typing/full verification and hosted rehearsal | Current source-qualified artifacts and original acceptance remain mandatory |

## HLD impact
- docs/hld/15-build-and-toolchain.md

## Risk routing
Apply release scripting/version routing: inspect existing family carriers and require reviewed clean main/final approvals. Apply external oracle routing: exact HB14.4 source/tool provenance and existing unchanged metrics. If sharing the HB producer entails existing Python binding-carrier edits, retain actual104 and standalone compiler/transitive closure assertions. No new file/module/crate/dependency is authorized. Normal scoped checks and final union remain mandatory.

## Parallel custody
Own .github/workflows/ci.yml, .github/workflows/wheels.yml, scripts/test_sprint_workflow.py and only the existing independent-tool preparation carrier if required. No F-X225 source or tests. Root owns shared scheduling/ledger/HLD14 entries and final source/version/archive integration. No worker writes sprint totals.

## Open questions
None. ROOT approved the concrete existing-carrier native-tools-only route and authenticated the original failure logs. No API, acceptance or publication approval is expanded.

## Boundaries
S91 and its s91 tag remain immutable completed history. This is S91.1 only, with no new contributor intake. Issues264 and281 remain discussions and open. S92 bibliography, navigation, authoring, multi-face/full-range followups and missing historical oracles remain deferred. Both still-unpublished prepared version families are requalified from the new exact reviewed main merge. No release tag, publication, remote closure or acceptance metric is inferred from this plan.


## Original failed evidence

- Presentation fidelity log: `/private/tmp/S91-main-CI-38662802341-readonly/Presentation-fidelity-failed-job-successor.log`, SHA256 fdad0f54f966f57284f4b2f714dd8c6213aa160d492c8d19f6baa5904b631527.
- MSRV log: `/private/tmp/S91-main-CI-38662802341-readonly/MSRV-failed-job.log`, SHA256 3b230b3c49f1c4fcb99f98e0a92ed29c5bee61c5281f6c19f4f21ec18bc7b5e4.
- The separate WASM memory trap remains F-X225 diagnosis, not a native tool or golden waiver.

## Hash harness

All49 accepted product hashes remain unchanged. The separate external PDF
raster manifest has one narrowly attributed correction for the already
approved F-X210 behavior. Current source215 retains exactly the reviewed
F-X210 PDF bytes. Its Georgia requests resolve to approved Gelasio instead
of Caladea. The labelled behavior commit7363ce342ee07897ded52cc422db34492738931a
updated12 product hashes but omitted these three external raster values.

Recompute only these decoded RGBA expectations in
`scripts/golden_pixel_manifest.json` with the original deterministic generator
and pinned Poppler26.01.0 at150DPI:

- contract: `06421718e6dbad9c622c8f7cfdc474e76620f32045dd46b06d678130a2d5b478`.
- proposal: `08ec5c7046b024447f9ebad03ff49823361a30e61f99e9a5436bb8ff6fe63945`.
- report: `8e1f83d71ad0e1a2aa58ba0d257a31be42049892918fbf69dcf11c4097d5f49b`.

All seven dimensions1275x1650, the four unrelated sample hashes, exact
comparison, DPI and rasterizer version remain unchanged. Local actual
FreeType2.14.3 and hosted2.13.2 produce these same three RGBA digests, so no
FreeType change is justified. This repairs propagation of the existing
approved output delta. It introduces no product, font, layout or oracle
algorithm change. The original failed CI run remains a failure.

The existing F-X210 design's Hash harness section and its PR319 incremental
golden-pixel refresh explicitly require this attributed measured recomputation.
Do not copy contributor expectations. Root's exclusive baseline lease uses
actual current producer bytes, the retained original12-key review and current
independent raster observations. Require an independent zero-finding review,
actual seven-sample check, one-pixel rejection and actual hosted acceptance.

## Implementation checklist

- [x] Reuse the exact source-qualified native-only HB producer before both native consumers.
- [x] Route the approved native stack to both original animation commands.
- [x] Execute the actual integer/fractional tag guard and deciding negative controls.
- [x] Pass scoped regression, policy and actual native controls without reducing binding104/native-eight.
- [x] Obtain a zero-finding microscope and update the listed HLD mechanism.
- [x] Recompute and independently review the three omitted F-X210 external raster expectations.
- [x] Pass original seven-sample exact comparison and one-pixel rejection.
- [x] Qualify actual hosted acceptance after the attributed raster correction.

- Native Test log: `/private/tmp/S91-main-CI-38662802341-readonly/Test-failed-job.log`, SHA256 e87dd5909a3c80c76043e9fbe54098acbf96d60337431418742663f299dfa7b6. The same missing exact hb-subset consumer fails after the performance and authoring gates pass.

## Scoped qualification

Local worker preparation is complete. The integrated full gate, binding104,
native-eight, primary hosted CI, clean main review and publication conditions
remain mandatory at the integrated source.

- Defining regression: `/private/tmp/F-X224-defining-red.log`, actual missing native setup and fractional tag refusal before implementation.
- Native-only producer: `/private/tmp/F-X224-native-tools-actual-1/receipt.json`, actual five successful commands, original archive and three14.4.0 binaries, no product bindings and stable source guards.
- Original SVG control: `/private/tmp/F-X224-native-svg-current.log` and `/private/tmp/F-X224-native-svg-msrv.log`, one pass each under current Rust and1.93.0.
- Original animation golden: `/private/tmp/F-X224-animation-golden.log`, one pass with the same manifest and8MiB.
- Policy: `/private/tmp/F-X224-policy-full-successor-2.log`,164 tests, zero failures, two existing skips. The first failed attempt remains `/private/tmp/F-X224-policy-full-1.log`. Its network/archive readiness and inherited mutation-target/count failures were corrected without relaxing a gate.
- Source-qualified retained22 package archives: `/private/tmp/F-X224-current22-materialization.json`.
- Hash harness: `/private/tmp/F-X224-hash49.log`,49 unchanged.
- Format, prose and generated adapters: actual exit0. No Rust production or published package source changed, so no changed-crate compile or Clippy gate is added beyond the original native controls.
- Independent whole-source review and test-only follow-up: `.claude/reviews/F-X224-all-pass-1.md` and `.claude/reviews/F-X224-all-pass-2.md`, zero defects, smells and nitpicks.

## Golden raster correction evidence

Primary CI38696954013 passed the actual full Rust, binding, MSRV, animation
and fidelity consumers but failed the separate golden-PNG comparison for
contract, proposal and report. It remains failed. The retained original
F-X210 reviewed PDF bytes and old/new Caladea/Gelasio inventories identify
the omitted baseline propagation, independently reproduced on macOS and Linux.

- `/private/tmp/S91.1-primary-golden-failure-38696954013/F210-causal-diagnosis-successor.json`, SHAe8a16487d8cec498731b1242e3fdd80461c19fd7aa39f4933621438ed62b0c74.
- `/private/tmp/S91.1-golden-PNG-current215-diagnosis-1/diagnosis-and-minimal-proposal.json`, SHA297b944d109738715caafc8e41864f1b58456b89ef13c89ed819c284e62e0a05.
- The reviewed original F-X210 twelve-key delta is recorded in `.claude/reviews/F-X210-hash-delta-pass-25.md`. Native metric compatibility is not inferred from raster baseline adoption.

Local correction qualification: original updater, seven-sample check,
one-pixel rejection and four self-tests pass, receipt1904a52c085166e37eda2e4c1c60ddbb71686aee609c86545661214c6ab3fcaa.
Bounded independent `.claude/reviews/F-X224-golden-pass-3.md` is clean.
The existing operative CI golden-gate mutation control passes. Prose has
zero violations,26 generated adapters match, and diff checks pass. Actual
hosted acceptance passed in CI38708235000 at6d7c7da729e0ecb34252e821341f22844e37a9fd.
Terminal receipt `/private/tmp/S91.1-primary-corrected-raster-acceptance-1/terminal-receipt.json`, SHAad03f29ef28fe9a5174cdb7cb30a8811c4cb25fa237eac4df1b8a0d63d388792.
Ordinary completion records its scoped evidence. Final reviewed-commit full
verification, sprint review and exact-main publication gates remain due.
