# S91 sprint review, pass 1

**Reviewed**: `sprint/s91` at `2c8e9c8bfb73a78e8b04a33c7cd860976cb94001` against merge base `bed9097dd73c3007eff01d80470bf7f165ba75c4`, 993 files, 234794 insertions and 46573 deletions. The delta touches 27 crates: oxml-chart, oxml-cli-support, oxml-core, oxml-drawing, oxml-layout, oxml-media, oxml-opc, oxml-pdf, oxml-py-support, oxml-sml, rdocx, rdocx-cli, rdocx-html, rdocx-layout, rdocx-opc, rdocx-oxml, rdocx-pdf, rdocx-py, rdocx-wasm, rpptx, rpptx-chart, rpptx-cli, rpptx-layout, rpptx-oxml, rpptx-py, rpptx-render and rpptx-wasm. Existing independent feature, composition and integration reviews remain authoritative for unchanged component bodies. This pass audits their integrated contracts and gate routing. No test, build, source, index or ledger mutation was performed by this review.

**Verdict**: 1 blocking, 0 should-fix, 0 nice-to-have. Return to distinct normal remediation before another review pass. S91 is not ready to close.

## Blocking

### B1, Clean CI cannot supply the mandatory binding acceptance inputs

`.github/workflows/ci.yml:263` creates an isolated environment containing maturin, pytest and the package oracle. The job builds the extension and runs its complete pytest directory at `.github/workflows/ci.yml:281`. Neither this job nor a committed conftest or preparation hook supplies the new required acceptance inputs. `crates/rdocx-py/tests/test_formatting_tables.py:1692` requires a coordinator-built `RDOCX_CLI`. `crates/rdocx-py/tests/test_rendering_threads.py:1063` requires `F_X195_VARIABLE_ORACLES`, and lines1193 and1198 require source-bound static-family guards and synthetic-axis inputs. The clean Word environment also lacks the Pillow dependency imported by the native paint companion at `crates/rdocx-py/tests/test_formatting_tables.py:1686`. These are hard failures, not optional corpora or skipped application checks. The PowerPoint variable consumers require the same external oracle setup at `crates/rpptx-py/tests/test_documented_examples.py:6590`.

Setting paths alone is insufficient. `crates/rdocx-py/tests/test_rendering_threads.py:1414` and `crates/rpptx-py/tests/test_documented_examples.py:7150` require the exact local Mach-O Poppler digest `d813ee83025433b811b5b7e982dc061751a1ea6cf03773107c398d2a597b70d9`. In contrast, the committed installer authenticates source archive `1cb944a4b88847f5fb6551683bc799db59f04990f5d8be07aba2acbf38601089` at `scripts/install_pinned_poppler.py:19` and builds on the current host. The workflow establishes neither that historical executable nor a portable source-build receipt admissible to these consumers. Exact version output alone cannot prove binary identity. The standalone path-reference consumer also requires a genuine compiler and transitive-artifact qualification at `crates/rdocx-py/tests/test_rendering_threads.py:1699`.

This contradicts the clean complete-directory CI obligation in `docs/hld/15-build-and-toolchain.md:924`. The approved independent controls are mandatory: `.claude/plans/F-X195-design.md:230` requires the existing entrypoints, lines240 through250 authorize independent static and synthetic controls from reviewed sources, and lines259 through285 retain all104 installed cases, the0.99 comparisons and separately qualified standalone renderer. Local temporary receipts do not make that preparation reproducible on a clean runner.

Remediation must prepare authenticated inputs before pytest, build and bind the actual CLI, install the qualified Python test dependencies, and provide portable actual compiler/tool provenance without weakening the historical binary route or accepting a version string as identity. The existing test carrier already embeds the standalone renderer source and independent contour authoring. Reviewed synthetic SFNT literals and licensed bundled Noto sources provide a bounded existing-file preparation route. Preserve independent HarfBuzz static controls, original-font paint, all104 cases, negative controls and the separate native-eight stage. Do not skip the affected cases or use product output as the oracle. Qualify the preparation on fresh task-owned inputs and prove the full CI command can consume them.

## Should-fix

Zero findings.

## Nice-to-have

Zero findings.

## Gate evidence and limits

The actual current core observation is `/private/tmp/S91-final-integrated-verification-1/approved-runtime-clean-core-gate-1/root-core-component-observation.json`, SHA-256 `999826f14204ce0233432da1caf50bffbba80ce766ae89163143774344e9e8a4`. Its13 commands exit0 at the reviewed HEAD. The raw native totals are6459 passes, zero failures and71 ignores. Four named historical early-return cases remain explicitly uncredited in S92. These facts do not constitute the full integrated gate.

The final Python attempt remains failed. `/private/tmp/S91-final-runtime-repair-paired-wheels-1/final-full-runtime-1/terminal-failure-observation.json`, SHA-256 `6553161a2d364bb47eddabf427828c9cafe9157f7d0ef00d117fefd8ade4b213`, records PowerPoint854 passes and Word736 passes with42 missing-external-input failures. Only14of56 Word and48of48 PowerPoint font cases passed. No104-case or native-eight admission follows from that result. Its source guard also failed when45 captured tracked files were absent. Recovery receipt `exact45-missing-tracked-recovery.json` in that directory, SHA-256 `17d2e0bf0b96f3b568e87fc22fe74c523f2fd71d1d6f6d6531e10f1e020ea5f7`, authenticates exact restoration from capture and Git. The reviewed tree is clean again. The failure remains failed, uninterrupted file presence is not claimed, and no actor is attributed.

The main-to-sprint baseline has16 distinct changed keys, not zero. The approved changes are the three F-X212 visibility keys, twelve F-X210 Georgia-to-Gelasio paint keys and two F-X200 namespace-transport XML keys, with an overlapping feature_showcase XML key. Their labelled baseline commits are3125cf58, eff7af54 and f92610f8. The corresponding plans and append-only AS_BUILT entries attribute the intentional behavior. The current core harness passes all49 accepted entries. That means unchanged against the approved sprint baseline, not byte identity with main.

Fresh final packaging and README qualification, restored current104 font acceptance, native-eight, current compatibility evidence custody and final full verification remain due. Missing historical oracle assets and missing frozen-body custody earn no new acceptance. Hosted exact-reviewed-SHA rehearsal, main merge, release tags, both publications and remote issue/PR reconciliation are not met by local preparation. The70 humanized notification drafts remain unposted. F-X221 is locally prepared but has not completed its formal sprint ledger and final gates.

## Milestone gate

The M24 gate at `docs/hld/14-development-backlog.md:2605` requires that "the approved capability matrix has no unexplained partial row" and the broad authoring and pinned-corpus acceptance described there. This review does not declare M24 closed. S91 is the bounded repair sprint, while its approved S92 carryovers retain their full contracts. The sprint definition of done at `docs/sprints/CURRENT_SPRINT.md:181` requires actual final integrated verification and review. B1 and the incomplete gate evidence prevent S91 closure now. Intake remains40 issues and30 PRs through362/363, with discussions264 and281 excluded and open. No new intake or deferred bibliography implementation is admitted.

## Not found

No additional interaction, duplication, layering, harness-attribution, dependency-consumer, public-surface or HLD-contract findings. In particular, the shared format-neutral manifests add no reverse family dependency, the pinned CommonMark parser and bounded-media dependencies have approved current consumers, and publication jobs retain their push-only boundary. The separate known external gates above are not relabelled as product defects or silently credited. This pass ends on writing this report and releases the review-only lease.
