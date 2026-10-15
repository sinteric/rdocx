# F-X221, Prepare Word 0.16.1 and PowerPoint 0.14.1 patch releases

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: F-X193, F-X194, F-X195, F-X196, F-X197, F-X198, F-X199, F-X200, F-X201, F-X202, F-X203, F-X204, F-X205, F-X206, F-X207, F-X208, F-X209, F-X210, F-X211, F-X212, F-X213, F-X214, F-X215, F-X216, F-X217, F-X218, F-X220, F-X222
**Test gate**: release regression
**Implementation**: local preparation, full verification and the bare-wheel routing amendment are qualified. Final integrated successor review is clean. Hosted rehearsal, closure and publication retain their separate gates.

## Problem

The user requests patch Rust crate and Python package publication in the existing Word0.16.x and shared/PowerPoint0.14.x lines after S91. Current stable workspace and Word Python versions are0.16.0, with PowerPoint/shared crates and Python at0.14.0. These exact versions are already published in both registries and tagged. S91 repairs require new immutable versions, reviewed family notes and fresh matching artifacts.

Read-only registry/tag receipt /private/tmp/S91-intake-admission-360-361/registry-tags-receipt.json records actual2026-10-12 availability: rdocx0.17.0 and rpptx0.15.0 are absent from both project registries, and v0.17.0/rpptx-v0.15.0 tags are absent. Recheck every selected crate/version and tag at final release, because these external facts can change. The initial hostname-only tag query failure remains retained separately. Its successful intake requests are not replayed or relabelled. The earlier0.17.0/0.15.0 target proposal is superseded by the user's patch-line correction. Current primary registry histories and GitHub CLI receipt /private/tmp/S91-release-readonly-preflight-1/corrected-network-successor-2/receipt.json (SHA49e1f3fba1fd58a3999d5010c0ddafecfcc6426295d5d394061883666ea746a8) verify that all seven Word0.16.1 and fifteen shared/PowerPoint0.14.1 crate versions, both matching PyPI versions and both remote release tags are absent. Current GitHub repository permissions and the pypi environment are verified. Read-only PyPI UI observation on2026-10-14 verifies Owner mantissaman and GitHub trusted publishers for both projects, repository tensorbee/rdocx, workflow wheels.yml and environment pypi, at receipt56f95fbec97770274f2ea7967e89d89185e079ccc0c113c5a10672bdb26a2539. These observations grant no publication credit and require the final release preflight. Recheck every final selected version and tag before publication.

## Spec reference

- docs/hld/14-development-backlog.md, F-X221 and completed F-X189 preparation authority.
- docs/hld/15-build-and-toolchain.md, Packaging, Publishing, Release process and CI job matrix.
- docs/hld/10-bindings-spec.md, Packaging and CI.
- .claude/commands/release-notes.md, Evidence and Validate and review.
- .claude/commands/release.md, Family contract, Preconditions and Final approval.
- .claude/commands/close-sprint.md, clean reviewed sprint merge provenance.

## Admission and dependency barrier

Design uses the run-sprint batch route. The user explicitly requests both patch families and matching Python packages. No material version question remains after current registry/tag verification. All28 included repair prerequisites must be completed through their full approved scoped lifecycles before claim. Font embedding F220 and the Issue362/PR363 inventory repair F222 remain included. Bibliography F192, navigation F283, standalone authoring carryovers and full-range watermark F219 remain in S92 and are not release blockers or completion claims.

Contribution intake is bounded to the user's latest explicit fold through Issue362/PR363:40 included issues and30 PRs, excluding discussion264/281. Updated PR357 and359 are reconciled incrementally with their existing F216/F217 owners, without repeating accepted provider implementation. Do not discover or silently add later records. Native/binding source must be final before version changes and fresh final wheel builds. Keep one canonical tracker, backlog, design and review set.

## Approach

Prepare exactly seven Word crates rdocx-opc, rdocx-oxml, rdocx-layout, rdocx-html, rdocx-pdf, rdocx and rdocx-cli at0.16.1, with rdocx-py and its Python project version0.16.1. Prepare exactly fifteen shared/PowerPoint crates oxml-core, oxml-opc, oxml-media, oxml-layout, oxml-drawing, oxml-pdf, oxml-sml, oxml-cli-support, oxml-chart, rpptx-oxml, rpptx-chart, rpptx-layout, rpptx-render, rpptx and rpptx-cli at0.14.1, with rpptx-py and Python version0.14.1. Inspect existing oxml-py-support and WASM version carriers explicitly, preserving publish=false and the separate npm boundary. Align exact workspace pins, selected manifests, Cargo.lock, pyproject.toml and current-version examples/assertions. Do not replace unrelated external tool versions or rewrite historical release evidence.

Reuse completed F189 unified family preparation, source inventory, policy tests and wheels.yml rather than inventing another release system. Inspect the workflow and existing exact version/allowlist tests against the new source. Extend the existing scripts/test_sprint_workflow.py entrypoint with s91_release_families_match_reviewed_versions, proving real old-version Base failure then both prepared family validations and rendered notes. No new product module, file, runtime dependency, feature flag or generic is needed.

Use existing release-notes commands to write meaningful CHANGELOG.md sections for v0.16.1 and rpptx-v0.14.1. Derive every claim and compatibility statement from completed AS_BUILT, approved contracts and reviewed commits. Include every authenticated issue/PR in its applicable family, with direct URLs and exact external author handles. Distinguish hardened equivalents from direct donor adoption. Preserve earlier history and clarify physical-picture migration, safe ownership/stale handles, actual watermark0.5 restriction, supported import/export subsets and named unsupported boundaries. Do not claim the deferred bibliography, navigation, full-opacity interoperability or unimplemented external skills. Record the concrete downstream skill patch separately until its actual submission/acceptance.

Build one contribution inventory per family in existing ignored evidence, not a new tracked delivery record. Authenticate each reporter/contributor from actual GitHub records. Preserve the current exclusion of discussion264/281 and do not send release-bound comments from this preparation. Publication workflow owns approved external notifications later.

Qualify locally patched verified cargo publication dry run for the exact22-package union, with actual current-source archives, exact member inventories and10MiB ceiling. Record final measured README archive metadata without importing donor sizes. Run ordinary scoped policy, formatting/prose/adapters and dependency/MSRV/deny controls, including the approved CommonMark dependency graph. Version preparation alone must not alter any deterministic output.

Build fresh final-source matching Word/PowerPoint bindings under exclusive target and CP leases. Authenticate wheel/member/RECORD/native identities and metadata. Test installed runtime priority and compatibility nodes on clean Python3.9 and3.12, with exact node collection, strict mypy/stubtest under3.12 and both WASM graphs. Do not relabel a preceding0.16/0.14 wheel with a new version. Earlier product controls can be referenced by exact unchanged source where valid, but current install/metadata qualification and final full gate remain mandatory.

Prepare the existing build-only wheels.yml rehearsal for the reviewed and pushed sprint SHA. It builds both projects without registry publication, tags or GitHub release creation. Authenticate twelve cp39-abi3 wheels and two sdists, selected per-family six CLI archives, metadata/name/version/description/classifiers/project links and exact source provenance. Each final release has six CLI archives, six wheels, one sdist and SHA256SUMS, fourteen GitHub assets total. A source-only or locally tagged artifact is not hosted build-only evidence. If hosted authorization or trusted publisher state needs action, complete the concrete preflight first and request only the missing specific approval.

Prepare shared/PowerPoint0.14.1 publication before Word0.16.1, because Word consumes the newly versioned shared graph. This story ends with reviewed local preparation and explicit post-integration gates. It never tags, pushes a release tag, publishes, merges main or creates a sprint tag. Run-sprint ends only after final integrated verify --full, clean sprint-review and close-preflight. The final hosted build-only rehearsal is bound to that reviewed source and completes before close.

After close-sprint produces the clean reviewed main merge, release from that exact SHA through /release rpptx-v0.14.1 then /release v0.16.1. The user explicitly authorizes both corrected patch families, sprint closure, publication and final addressed issue and PR reconciliation after successful publication. Bind that authority to the exact reviewed SHA, assets, allowlists, verified roles and trusted publisher registrations, fresh registry absence, rendered notes and contribution notifications. Preserve all canonical release preconditions, main verification and exact reviewed-tree comparison. No publication or remote closure is credited by preparation.

## Rejected alternatives

- Reuse existing registry versions or mutate old tags, which are immutable.
- Python-only or mixed-family tags abandon the unified crate/CLI/Python contract.
- Rename old wheels instead of rebuilding current-source metadata/native code.
- Publish from a worker or unclosed sprint before the clean main merge and final approval.
- Reimplement the release workflow instead of qualifying its existing exact allowlists.

## Test plan

**Test gate**: release regression, valid existing version carriers against the requested new family contract.

| Category | Test | Asserts |
|---|---|---|
| release regression | s91_release_families_match_reviewed_versions | Real old-version Base fails, both exact selected crate and Python versions/pins/locks, notes and new current literals pass |
| package | actual locally patched workspace publish dry run | All22 exact archives verify, source/member provenance, actual measurements and10MiB ceilings |
| Python | clean current wheels on Python3.9 and3.12 | Native origin/member/RECORD and metadata, actual priority/compatibility cases, strict mypy/stubtest under3.12 |
| integration | version graph, policy, README, workflow and WASM | Published allowlists remain7/15, bindings/WASM excluded, external tools and historical records remain intact |
| post-integration artifact | exact reviewed-SHA build-only rehearsal | Both families' twelve wheels/two sdists and CLI payloads are provenance checked without publication |
| post-integration release | final full sprint and clean-main gates | Clean review/close provenance, fresh registry/tag/owner/trusted-publisher facts, separate immediate family approvals |

No absent method/command compilation is Base failure. Preserve original failed commands and exact successors. Local microscope must reach zero defects and smells before prepare. Final integrated full verification and sprint review run once over the finished source, with actual failed checks retaining their state. Hosted artifact and publication are not inferred from worker completion.

## HLD impact

- docs/hld/03-architecture.md, correct only the older optional-codec paragraph to match approved F218 unconditional bounded-media validation.
- docs/hld/10-bindings-spec.md
- docs/hld/14-development-backlog.md
- docs/hld/15-build-and-toolchain.md

## Risk routing

Release/version strings: canonical release and HLD15, inspect every manifest/pin/lock/README/policy diff, exact family preflights, final full gate and separate final approvals. Published API: HLD10 and structural rules, actual pre1.0 minor compatibility notes, verified current archives below10MiB. Graph: HLD03, existing22-package union and shared-first order, no new facade dependency. PyO3/WASM: fresh paired source/version provenance, clean3.9/3.12 runtime/typing and both WASM graphs. No rendering change, new product file/module/crate/dependency/trait/generic is introduced by preparation. All ordinary scoped checks remain mandatory.

## Hash harness

All49 deterministic outputs must remain unchanged by version preparation. Preserve prior attributed approved sprint deltas and goldens. Do not replace the baseline or discover host fonts.

## Implementation checklist

- [x] Complete all28 repair prerequisites and obtain exclusive preparation claim.
- [x] Recheck version/tag facts and genuine old-version Base failure.
- [x] Update exact family carriers and authenticated notes/inventories.
- [x] Pass actual package, policy, wheel/metadata/runtime/typing/WASM controls.
- [x] Obtain clean microscope, update listed HLD files and prepare handoff.

## Post-integration gates

- [x] Route native and musl bare-wheel checks through the authenticated existing78-node priority inventory, with exact selector controls and actual bare-environment acceptance.

- [x] Final integrated full verification and clean sprint review at the current SHA.
- [ ] Actual build-only hosted rehearsal at that exact reviewed pushed source.
- [ ] Close-preflight and close-sprint exact main merge provenance.
- [ ] Separate /release final approval and verified publication for each family.

## Open questions

No blocking design question remains. The user requests patch crate and matching Python releases in the existing version lines. Fresh checked registry/tag facts support0.16.1 and0.14.1. The user explicitly authorizes publication and closure in these corrected lines after all gates. Exact main provenance, verified roles and publisher registrations, assets and final registry checks remain mandatory.


## Scoped qualification oracle corrections

The release checks retain their actual failed predecessors. The exact shared
stale-error helper now includes the approved `stale ` prefix without changing
exception, revisions, first cause, recovery or byte-atomicity assertions.
Historical S90 notes remain fixed while present carriers follow current
versions. CLI rehearsal job guards admit only the approved manual-or-push
builds, with strict push-only publication guards and negative mutations.
The WASM manifest control now checks the approved F218 unconditional facade
image-validation codecs separately from optional rendering dependencies.
This changes only the existing test oracle and the contradictory older HLD
paragraph, preserving native defaults, WASM isolation and product dependency
edges. Frozen compatibility bodies and retained release artifacts are unchanged.

Binding CI prepares independent licensed-font and synthetic-axis controls from source before pytest. Pinned HarfBuzz14.4.0, authenticated Poppler and the physical compiler with an isolated dependency closure qualify the reference renderer. The104 cases,0.99 paint thresholds, complete Python suites and separate ROOT native-eight gate retain their acceptance. The current full gate passes with unchanged production bytes from the genuine paired-wheel build. Final sprint review, hosted rehearsal, close and publication retain their separate post-integration gates.

The bare-wheel amendment uses the existing37 Word and41 PowerPoint priority/compatibility node IDs. Native and musl jobs pass exact argument arrays without shell splitting. Complete-directory Python,104 font and native-eight acceptance remain in the equipped gates. No missing oracle becomes a skip, and no runtime acceptance is removed. Original clean reviewpass2 remains historical. The changed release workflow has a clean independent working review and actual bare78 acceptance. Its final integrated successor review is clean. Actual hosted rehearsal remains required before closure.
