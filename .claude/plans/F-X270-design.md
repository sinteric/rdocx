# F-X270, Prepare Word 0.16.4 and PowerPoint 0.14.4 patch releases

**Status**: completed
**Sprint**: S95.4
**Size**: L
**Depends on**: F-X263
**Base**: 4f5c4ae20fbc25a399af819aa2b20c84c67b39a1
**Design date**: 2026-10-27

## Problem

The user requested publication preparation after the product stories and a hold for login and final approval. The current0.16.3/0.14.3 carriers and tags were already published from S95.3 at a57be365. They cannot identify a new release. Existing S95.4 additions were incorrectly appended to those published changelog sections. Current product qualification must not be mistaken for new registry absence or publication readiness.

## Spec reference

- `docs/hld/10-bindings-spec.md`, Packaging and CI, exact distribution metadata and installed typed consumers.
- `docs/hld/15-build-and-toolchain.md`, Packaging, Publishing, Release process and CI job matrix, exact artifacts and final approval.
- `docs/hld/14-development-backlog.md`, F-X270 and completed F-X263 release machinery.
- `.claude/commands/release.md`, Family contract, Preconditions and Final approval.

## Dependency readiness

F-X263 is done in BACKLOG and its actual S95.3 preparation/publication records exist. It supplies the unchanged release workflow and validators. All eight S95.4 product stories are already integrated, so no unavailable public contract is assumed. Their pending final acceptance remains required by A8 and is not converted into a fictitious completed dependency. This preparation can align metadata while their final gate remains pending. No release can proceed until the whole integrated union and sprint close pass.

## Approach and compatibility

Prepare Word0.16.4 across exactly seven published Rust crates, its nonpublished binding crate and Python project. Prepare shared/PowerPoint0.14.4 across exactly fifteen published Rust crates, its nonpublished binding crate and Python project. Retain the separate rpptx-wasm0.12.1 boundary and all historical version controls. Align the Word workspace inherited version and every applicable internal dependency pin and lockfile record. Do not change external dependencies, feature sets, source algorithms or public signatures.

The retained0.16.x/0.14.x lines come from the user's recorded F-X263 choice. Q9 already approves the bounded Arc Rust source break. Both new notes must prominently preserve that explicit source-compatibility exception and concrete migration. The patch labels do not promise Rust source compatibility. Python API signatures remain unchanged. Patch-only changes preserve the existing major/minor Word AppVersion stamp, subject to actual call-site inspection. Inspect all version consumers before asserting product evidence applicability. Keep the actual older producer labels when version-only source reuse is justified. A new hosted artifact has its own producer and build identity.

Restore the published changelog suffix from a57be365 exactly. Extract only the S95.4 additions into new0.16.4 and0.14.4 sections with Highlights, Added, Fixed, Compatibility and Contributors. Preserve all relevant reporter and contributor credit, native limitations, original404 unavailability and reporter retest note. Do not claim unreduced-case verification, universal flat cost, full Word database support, soft-wipe PDF parity or held-shadow/wipe qualification.

Use the existing sole-writer canonical preparation path established by F-X263. Record actual source and Base without inventing a worker squash. No delegated implementation or new Grok task. A validated external-ci-source-availability checkpoint may push only the clean reviewed sprint ref after applicable local checks and independent microscope pass. Dispatch only wheels.yml workflow_dispatch, authenticate actual run/jobs/source and exact artifacts, and require all publication jobs skipped. No release tag, registry upload or GitHub release.

### Explicit canonical source-availability exception

Root recommends and approves the narrow canonical exception under the user's explicit release-preparation and select-recommended-without-questions instructions. The ordinary new-workflow/worker-handoff checkpoint wording does not literally fit this sole-writer preparation using unchanged wheels.yml. Only exact reviewed new-version source availability is excepted. Root must authenticate the clean canonical sprint ref, remote, SHA/ancestry, actual local scoped evidence and zero-finding independent microscope, and retain hosted/product debt explicitly pending in an actual canonical evidence packet. No fictitious worker squash or consumed worker handoff is recorded. A non-force push of only that exact sprint ref and manually dispatched build-only workflow is allowed. No main merge, release tag, registry upload or GitHub release is allowed. All other checkpoint restrictions and full acceptance remain binding. This exception is not publication approval.

### Pinned hosted inventories

The unchanged workflow's WHEEL_TEST_NODES maps42Word and44PowerPoint nodes. The complete exact names and hash are pinned in /private/tmp/s954-next-patch-priority-pin.json, nodes SHA256 `84fa609632ef1e871a0286f8d3c2283ab7d543ea0dc01c5fdfd7c742b6e31876`, workflow SHA256 `a95b59d51e6bc2d5e12dcc28915d99334d1de95d5dd1272f74173af01b665df1`. Run both maps on each Python3.9/3.12 host install,172calls total, plus strict3.12 mypy/stub checks. Required31hosted jobs comprise12wheel,2sdist,12CLI and1CLI-aggregate successes, with4publication jobs skipped. Authenticate26unique raw payloads:12wheels,2sdists and12CLIs. Aggregated copies add no subjects. The source-availability exception does not waive any inventory.

## Consumer map

| Consumer | Input and observable contract | Evidence |
|---|---|---|
| Cargo users and archive verification | Exact22local-source versions and internal pins, unchanged algorithms | A1,A4,A8 |
| Python metadata and imports | Exact project/wheel/sdist identity and installed version, unchanged typed APIs | A1,A5,A6 |
| CLI users | Exact host CLI versions and six target inventories per family | A5,A6 |
| Release validators and maintainers | Published history preserved, only new family notes, exact credit and constraints | A2,A3 |
| Existing product/native/performance consumers | Existing source-bound observations plus explicit version-consumer exclusion proof | A7,A8 |
| Publication workflow and user | Prepared clean main SHA, fresh final mutable preflights and separate approval | A9 |

## Acceptance criteria

| ID | Input and expected result | Local/final evidence |
|---|---|---|
| A1 | Candidate0.16.4/0.14.4 absent in exact22Rust entries,2PyPI entries and remote tags, and all current version carriers/pins agree | Fresh read-only receipt and current family/version policy controls. Final publication rechecks mutable absence |
| A2 | Published changelog from v0.16.3 onward remains byte-exact to a57be365. New family notes contain only S95.4 deltas and unchanged relevant caveats | Actual historical-integrity RED/GREEN and independent note review |
| A3 | Word new notes cite issues402,404,405,406,407,408 and PR409 through422. RP notes cite issue403 and PR413,416, all with hadim credit. Shared work and adapted inputs are attributed accurately | Exact current inventory regression, historical sections separately retained |
| A4 | Actual current-source22verified archives are below10MiB, contain correct READMEs/metadata, match exact normalized member bytes/counts and existing64byte compression tolerance. All README examples compile | One final integrated packaging/README qualification after metadata alignment, retained verified archives |
| A5 | Actual build-only workflow has exactly31jobs:27successful deciding jobs and4skipped publication jobs. Downloaded exact sixCLI, sixcp39abi3wheels and one sdist per family have authenticated source/artifact identity, correct versions and complete metadata | Actual hosted run and artifact inventories/digests, no substitute old artifacts |
| A6 | Downloaded host CLI reports new version. Fresh Python3.9/3.12 installs of both host wheels pass the pinned42Word and44PowerPoint priority calls per interpreter,172calls total, and3.12 strict typing/stub checks, and report exact installed distribution versions and authenticated imported module paths/hashes | Actual installed artifacts, commands and outcome receipts. No PyO3 Rust test binary |
| A7 | Version-only edits do not silently invalidate product output, performance or native evidence. Trace every changed version consumer and actual cfg/profile/dependencies, retain actual original producers and all prior failures | Explicit independent applicability assessment, targeted observable version controls, actual hash gate if inputs affect outputs |
| A8 | Complete S95.4 union passes all eight existing product contracts, including Cell.add_picture. Refresh only checks invalidated by this metadata delta, keeping all actual pending/failed results | Final integrated source-bound full record and affected policies, no repeated unaffected runtime/native/RSS suites |
| A9 | Preparation completes on reviewed clean sprint source, with a handoff that authenticates the exact main merge and tree after sprint close. Publication waits separate exact-main approval and any user login. Shared/PowerPoint family precedes Word | Final handoff and canonical close/release preflights. No publication in this feature |

## Test matrix and discriminators

| State | Expected discriminator |
|---|---|
| Existing published0.16.3/0.14.3 | Fresh registry200 and existing tags reject them as new candidates |
| Fresh proposed0.16.4/0.14.4 | Actual24queries404 and no matching remote tag establish candidate selection, not perpetual absence |
| Changed published notes or missing new-only scope | Integrity/inventory assertion fails even if the generic release-note parser accepts the text |
| One old carrier or internal pin | Family policy fails while historical mutation fixtures stay unchanged |
| Wrong version/platform/metadata, duplicate artifact or wrong producer | Existing exact archive/wheel/CLI validators refuse |
| Old wheel substituted for new current artifact | Actual imported path/hash/version and downloaded-artifact digest refuse |
| Cell gate still failed or hosted job incomplete | Preparation remains in progress, no final completion/close/publication claim |

## Test plan

Local: establish actual new-target carrier and historical-note regression failures against Base, then batch affected version-family, inventory, note and metadata policy cases. Preserve original200 registry responses and all actual failure logs. Existing policy machinery is reused, not weakened. No new product test module or source file.

Final integrated union: actual22package dryrun and README inventory/examples at final source, actual hosted build-only artifacts, fresh downloaded-wheel Python3.9/3.12 priority and3.12 typing/stub checks, host CLI version checks, exact artifact inventories, source-applicability assessment and all pending product8 criteria. Reuse applicable full lint/WASM/native/performance/memory/supply-chain observations with explicit input and version-consumer proof. Run only invalidated checks, never blanket new normal LTO/RSS/native suites for patch metadata. Publication and mutable publisher/owner checks stay under release at reviewed main.

## HLD impact

- `docs/hld/14-development-backlog.md`, F-X270 and current prepared-version wording in F-X267.
- `docs/hld/15-build-and-toolchain.md`, Packaging and Release process current prepared family references if affected by actual version inventory. No mechanism change.

## Risk routing

Release scripting/version strings: exact manifests, internal pins, lockfile, README and family assertions audited. Require clean final gate and separate final approval. Python/package metadata: actual installed identity and typed smoke on downloaded wheels. Public Rust compatibility: preserve approved Arc exception, no new API or semver promise. Completion/evidence adoption: affected live-owner/status policies after ledgers. No XML/parser/render/font/cache algorithm changes are authorized. Product baseline changes are not authorized by this plan.

## Hash harness

Expected unchanged49hashes and existing deterministic pixel buffers. Version-consumer inspection must confirm patch metadata does not alter output inputs. An actual unexplained delta blocks acceptance, with no baseline rerecording.

## Exclusive resources

Existing Cargo.toml carriers and Cargo.lock, both Python pyproject metadata, current README version/examples/measurement rows, CLAUDE prepared-family prose, existing CHANGELOG.md, scripts/readme_doctests.py dictionaries and scripts/test_sprint_workflow.py current policy assertions, the existing version-carrier test assertions in rdocx-wasm/src/lib.rs, rpptx-wasm/src/lib.rs, oxml-chart/src/lib.rs, oxml-drawing/src/lib.rs, rpptx-render/src/lib.rs, rpptx-oxml/tests/integration.rs and rpptx/tests/integration.rs, and the two named HLD files. Binding initializers have no independent version constant and remain unchanged. The existing .github/workflows/ci.yml lease is limited to its rdocx-wasm current package-version literal. No new source module, crate, trait, test binary, dependency or workflow is needed. The new canonical plan/progress/handoff/review records are required workflow artifacts for the user's requested release preparation.

## Implementation checklist

- [x] Confirm completed provider, fresh candidate absence and exact current carrier map.
- [x] Record actual carrier/history RED and retain historical controls.
- [x] Align current versions and pins, restore published notes and add only new family outcomes/credits.
- [x] Complete named HLD intent and obtain clean independent microscope.
- [x] Qualify actual final archives/README and applicable local policies.
- [x] Complete reviewed source-availability checkpoint and actual hosted build-only artifact qualification.
- [x] Qualify downloaded host CLI and installed Python3.9/3.12 plus typing/stubs.
- [x] Accept all product8 criteria and explicit version-only source applicability.
- [x] Finalize delivery/metadata and hold publication for exact-main approval.

## Open questions and authority

No new product API or version-line decision. The user explicitly requested release after the stories, preparation while asleep and separate login/approval later, and subsequently instructed no questions and selection of recommended options. Root recommends the next actually absent retained-line patches0.16.4/0.14.4 and applies that instruction to preparation only. Actual candidate receipt is /private/tmp/s954-next-patch-absence/receipt.json. Existing0.16.3/0.14.3 receipts are /private/tmp/s954-current-registry-readonly. This is not publication approval. Independent corrected design review is zero-finding in F-X268-final-performance-pass-30. Root approves this bounded plan at4f5c4ae2 plus the recorded intake on2026-10-28 under the user authority above. The earlier draft and its four corrected clarifications remain in the actual review history. The existing eight designs and every pending product gate remain binding.
