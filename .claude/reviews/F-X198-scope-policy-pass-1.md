# F-X198, scope policy, pass 1

**Reviewed**: Supplemental archive and roadmap policy review of worker `/private/tmp/rdocx-f-x198-codex` at HEAD df3b6287ec42e9392479a6ab338f7c5069b57367 against Base1f163e4aff8d9cbdbad4fad53c8d2c67fc4ae811. Two uncommitted archive carrier files, 2 additions and 2 deletions. Separately reviewed roadmap cherry-pick, one file, 1 addition and 1 deletion, plus its copied review. Prior clean production ALL pass2 is not reopened.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Evidence

The approved archive carrier contract at `.claude/plans/F-X198-design.md:51` permits only the observed rdocx README row and measurement tuple, with unchanged inventory and tolerance. The manifest, lockfile, version and publication constraints remain explicit at `F-X198-design.md:100`. The actual delta is confined to `README.md:42` and `scripts/readme_doctests.py:405`, both recording 3528332 compressed bytes, 50780709 normalized member bytes and 37 members. No other measurement row or script helper changes.

`/private/tmp/S91-F-X198-archive-measurement-receipt.json:3` and its canonical log retain two exact measurements of that tuple, associated with historical archive SHA-256 `05ebe1333014324eb742bd2ebcc7ce07de32b1cd0bb977ff7941c11370478948`. The exact argv matches the existing package helper at `scripts/readme_doctests.py:1257`, including locked, allow-dirty, no-verify and the local publishable patches. The earlier failed worker-target lookup is retained separately and is not passing evidence. The original measured gzip has subsequently been overwritten by the active policy run. This review does not claim to have independently rehashed or retained that original binary.

A read-only inspection of the current archive identifies it separately as SHA-256 `9e2ce53d197bd2f0a539c49938652537dbb5e82ddfdd513808238f951feb7066`. It has 3528330 compressed bytes, exactly 50780709 normalized member bytes and 37 members. Its two-byte compressed difference is within the unchanged tolerance at `scripts/readme_doctests.py:394`. Its single packaged README exactly matches the reviewed worker README. Packaged document.rs, regression_test.rs and Cargo.toml.orig also match their actual worker source bytes. The archive is not relabeled as the original measurement output.

Exact normalized member bytes and member count remain required at `scripts/readme_doctests.py:1348`. The compressed 10 MiB ceiling and pinned-toolchain tolerance remain at `readme_doctests.py:1350` and `readme_doctests.py:1354`. Generated VCS normalization remains at `readme_doctests.py:1291`, with no payload exemption added. The exactly-one-README and source-byte comparison remain at `readme_doctests.py:1385` and `readme_doctests.py:1393`. These preserve the HLD15 contract at `docs/hld/15-build-and-toolchain.md:807`.

No workspace manifest, crate manifest or Cargo.lock differs from Base. `crates/rdocx/Cargo.toml:13` still selects the root README, and release metadata at `Cargo.toml:15` is unchanged. Read-only TOML inspection confirms 27 workspace packages and 22 publishable members. The existing LOCAL_PATCHES set equals exactly those publishable name/path pairs, preserving the strict metadata comparison at `scripts/readme_doctests.py:1238`. The normalized packaged metadata retains rdocx 0.16.0, edition 2024, MSRV 1.93 and the existing release metadata. Packaged dependency entries have no local path override. No publish membership, asset inventory, dependency, feature or release boundary changes.

The roadmap file matches the separately reviewed e211fe6e source byte-for-byte. Cherry-pick df3b6287 changes only `scripts/test_sprint_workflow.py:7947`, extending the occupied sprint expectation through S95. The copied `.claude/reviews/S91-roadmap-policy-pass-1.md:9` explains the inserted sprint boundary and the audited story placements. Unique placements at `test_sprint_workflow.py:7943`, unique backlog rows at `test_sprint_workflow.py:7964`, cross-record equality at `test_sprint_workflow.py:7976` and archived exclusion at `test_sprint_workflow.py:7978` remain unchanged. No unrelated policy assertion is weakened.

All eight previously frozen source, plan, HLD and review hashes match the existing microscope freeze receipt. This is identity evidence, not a new production correctness review. Current carrier hashes are README `a0a66ef817fc393abdf44ccb48747bd9727f2e09a1550fa5c77deeea70c8515b` and measurement script `13ba407ec6eb02ba091c746985f6139d3717cae5edec351096e44754744a1334`.

## Not found

Correctness, approved scope, package metadata, inventory strictness, archive measurement policy and roadmap assertion strength produced zero findings. No new panic path, OOXML behavior, structural abstraction or public API is introduced by this supplemental delta.

No Cargo, test suite, native UI, package regeneration, source fix or publication action ran during this review. Existing native receipts retain the original 300-second consumer timeout failure separately from its unchanged exact 11.62-second passing rerun. Installed Python, final focused native controls, both WASM checks and all 49 hash matches are prior receipts, not executions by this review. The active full repository policy result, subsequent verified 22-package dry run and final scoped completion records remain outstanding at review time. This clean supplemental review does not claim those gates passed or complete the feature handoff.
