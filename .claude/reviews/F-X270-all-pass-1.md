# F-X270, all aspects, pass 1

**Reviewed**: Working implementation against Base `4e7c4517` on `sprint/s95.4`, 59 existing files, 296 additions and 260 deletions. Approved plan SHA-256 `15c036b1576c5e23cc9d00f244cae6f5b5c907a1fe6c255c0f98010e4b96fefe`. This is the bounded carrier, notes, test and source-applicability review. Final archive, hosted and installed-artifact outcomes remain pending.

**Verdict**: 0 defects, 0 smells, 0 nitpicks in the reviewed implementation.

## Contract and scope

Read canonical repository guidance, workflow and microscope, F-X270's approved contract, its prior independent design assessment, current readiness/progress and the named HLD impact. F-X270 is in progress in both delivery records. Its completed formal provider is F-X263. The eight integrated product stories retain their complete acceptance criteria, with no fictitious dependency completion or new product API.

The plan's bounded CI lease adds only the actual rdocx-wasm package-version consumer. `.github/workflows/ci.yml:468` changes that literal to 0.16.4 and retains the separate rpptx-wasm 0.12.1 boundary. No job, routing, permission, artifact set, feature or publication rule changes. The actual canonical sole-writer source-availability exception remains explicit, with local evidence and independent review required before any sprint-ref push. This report does not execute or approve such a checkpoint by itself.

## Carrier and dependency correctness

The Word workspace family and Python project now select 0.16.4. The explicit shared/PowerPoint family, binding and Python project select 0.14.4. The seven and fifteen published allowlists remain unchanged. Binding/support packages remain unpublished where previously required. The independent WASM boundary remains unchanged.

I compared parsed before/after manifests independently. All non-version fields agree, including feature sets, profiles, external dependencies, target settings and publication configuration. The workspace's 24 internal dependency pins align with the new family versions. I also compared all lockfile packages in order. Exactly 26 local package entries change only their patch versions. No source, checksum, dependency list or external package changes. This is not an external dependency upgrade.

The seven modified Rust files change existing manifest assertions inside test code only. The WASM link fixtures retain their previous feature and target choices. Test literals now select the actual prepared dependency versions. No runtime method, parser, renderer, owner, cache, font, public signature or production guard changes in this implementation.

## Published history and new contribution scope

I independently compared the current CHANGELOG suffix beginning `## v0.16.3` byte-for-byte against `a57be365`. It agrees exactly, SHA-256 `dd636f9d967ef1a09cdd4c3e47b5a7096b77ba01700a5cf33741785a3d672e11`. The new invariant at `scripts/test_sprint_workflow.py:79` pins that suffix rather than silently rewriting published releases.

The new Word and PowerPoint sections contain S95.4 outcomes separately from that published history. The exact current contribution assertions at `scripts/test_sprint_workflow.py:53` require Word Issues 402, 404 through 408 and PRs 409 through 422, and PowerPoint Issue 403 and PRs 413 and 416. Each record remains linked and credited to hadim, with shared Word applicability and adapted contributor inputs distinguished from unchanged PR merges. Earlier published contribution inventories remain in the exact historical suffix.

Both new compatibility sections prominently disclose Arc storage and removal results, the Word re-export, accepted Vec inputs, sharing, explicit owned cloning and copy-on-write migration. The retained version-line exception is not described as unrestricted Rust compatibility. Python signatures remain unchanged. The new notes retain metadata-only offline mail-merge boundaries, the CPython 3.9 constructor-metadata limitation, unavailable original 404 input and reporter retest, qualified performance and memory limitations, soft-wipe PDF limits, group fallback and unqualified held-wipe/shadow interactions. Retaining relevant existing limitations is not recounted as new delivery credit.

HLD14's current prepared-family wording and HLD15's current carriers, prior complete releases and immutable release SHAs agree with the implementation. Historical version examples, benchmark observations and earlier package contracts are not globally replaced. Current README examples and archive row version labels are updated. Their final measured bytes and compilation remain separate actual gates.

## Discriminating local evidence

The original three-case RED is retained at `/private/tmp/s954-fx270-target-history-red.log`: two assertion failures for the old target version and changed published history, plus the missing-new-notes error. This is not described as a compiler or missing-API semantic failure. The current target and historical tests reject a stale family carrier, missing new notes, leaked earlier contribution scope and altered published suffix independently.

The source alignment receipt `/private/tmp/s954-fx270-alignment/receipt.json`, SHA-256 `6b2b1698cb72268f74c5b90e15ab9c55d178d21d3513524ba91e2ae5508dd381`, identifies 57 alignment changes and the 13 existing current policy methods. I authenticated their recorded resulting file hashes against disk. The remaining plan lease and single CI literal were inspected directly. Existing wrong-version, wrong-family, stale-example and metadata mutation assertions remain active.

`/private/tmp/s954-fx270-policy-green.log`, SHA-256 `59a513321627285de0ebbbddd1e932a31f6df78086f550c049cfaa88c18adb1c`, records 17 tests passing. Its printed negative-mutation diagnostics are expected test observations, not failed acceptance. The seven-case native/WASM/clippy batch was still running at this review boundary. I did not adopt an uncompleted batch, invoke another compiler or wait for a confirmation build. This is not scoped feature completion.

## Explicit existing-artifact source applicability

The only direct package-version-dependent production output found in Rust source is Word's AppVersion construction at `crates/rdocx/src/document.rs:8118`, which reads major and minor components. Patch selection leaves both unchanged. The two CLI roots use Clap's generated version flag at `crates/rdocx-cli/src/main.rs:14` and `crates/rpptx-cli/src/main.rs:12`. Their displayed package versions intentionally change and require actual newly built/downloaded CLI verification. Neither binding exports a native version getter. Distribution metadata and imported native module path/hash, not an invented getter, must qualify the new wheels.

Other changed version consumers are package/distribution metadata, dependency selection, documentation, current policy assertions and the existing manifest tests. Internal lock resolution, external dependencies, runtime algorithms, Rust target/features and build profiles are otherwise unchanged. Consequently existing normal release, instrumented release, opt-level 1 noassert A9, native functionality, native Office comparisons, geometry/render, timing and RSS/lifetime observations remain source-applicable to the unchanged product behavior within their already reviewed workload and configuration boundaries. The previous debug-only repair and its actual affected tests remain separately qualified by pass 30. This preparation does not introduce another debug or noassert runtime change.

This applicability retains every actual producer: normal Word `fc670a18`, integration/instrumentation `7ca9adb9`, the preserved PowerPoint producer and its explicit F48/7ca joins, and the independently qualified noassert A9 and lifetime producers. It does not claim that those binaries were rebuilt as 0.16.4 or 0.14.4. Nor does it establish the identities of future hosted artifacts. New artifact receipts must authenticate actual prepared-source builds, versions, configuration, complete metadata, downloaded bytes and installed modules. Old artifacts cannot satisfy those requirements.

Patch metadata and test changes do not invalidate owner-scan/count assertions, memory graph lifetime facts, independent Office source/output comparisons or the actual qualified public-call timing algorithm. Repeating complete native, performance or RSS suites solely because the package patch numbers changed is not required. Actual changed version outputs, package contents and hosted/installed artifact checks remain required. Any subsequently changed runtime source, feature, configuration, input or dependency graph requires a new applicability decision.

## Actual Cell successor and retained original failure

The separately recorded same-product Cell successor has completed. `/private/tmp/s954-cell-admitted-run/receipt.json`, SHA-256 `284db74da3f48379bd4769ed1384b0e06b245e47e392b053922772ba8a7cc178`, records one actual attempt admitted at load 16.7715 and 29.0254 with no compiler, after fourteen rejected observations. Its original frozen driver/module and 7ca workload identity remain attached. This is not timing on a newly prepared-version artifact.

The actual sample record has small values 62.612042, 25.265625 and 24.889375 ms and large values 142.893667, 40.835583 and 39.790875 ms. The pinned minima give 39.790875 ms against the unchanged `1.5 * 24.889375 + 5` limit of 42.334062 ms. Cold first samples remain visible. Summary reports one required measurement, no errors and no mandatory failure. The old full-catalogue failure remains preserved. This bounded admitted successor supplies the affected passing observation, rather than rewriting the original full run or selecting away unfavorable samples.

## Pending acceptance and publication boundary

New-version verified 22-package archives, actual archive measurement updates, README compilation/inventory, hosted build-only jobs, 26 unique downloaded payloads, both host CLIs, 172 installed priority calls and required typing/stub checks remain pending. No old-version archive or installed wheel is accepted as a new-version artifact. Final full-union recording, delivery reconciliation and independent sprint review also remain due.

This clean source review permits continuation toward those actual observations. It does not complete F-X270, pass the sprint, merge main, authorize a tag or approve publication. Final mutable absence, publisher/owner and exact-main approval checks remain mandatory under the separate release command.

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Not found

Correctness, contract, panics, OOXML/output preservation, test discrimination, structure, historical integrity and artifact/source labeling produced no findings in this bounded diff. The review used static source, stored receipts and archive/metadata evidence only. It executed no compiler, test, benchmark, product import, native application or external mutation. Only this review file was written. The pass ends here and returns control to the orchestrator.

## Supplemental final inventory data and scoped outcomes

**Verdict**: zero defects, zero smells and zero nitpicks in the additional observed inventory data and completed scoped evidence. This supplement reviews new material rather than confirming unchanged product implementation.

The delta after the alignment hashes is confined to observed compressed-byte rows, their measurement dates, the existing dictionaries and generated backlog counts for the running preparation story. I checked every alignment identity against disk. The 23 changed identities are within the allowed README/data/count set. Runtime, manifests, dependency graph, CI, algorithms and seven native test sources remain those reviewed above. Backlog totals move one cross-cutting story from pending to in progress without changing total or completed counts.

`/private/tmp/s954-fx270-final-inventory-update.json`, SHA-256 `18d6052e1c01f390bf87d0fc4fb06ab4efd89ec47d9dc5ddadc1b30d015a5a9e`, pins the actual round-one log SHA, old and observed tuples, 22 pages and date 2026-10-28. I checked all observed values against that log and the parsed dictionaries. All normalized member-byte totals and member counts stay unchanged. All old compressed values were already within 64 bytes of the observations. Neither tolerance, normalization, size ceiling nor comparison policy changes. This is fresh data disclosure, not a weakened acceptance limit.

For this particular update I verified each old/new compressed value's formatted width and the unchanged remaining tuple fields. The dates also retain their width. Unlike the earlier rpptx-layout digit-growth incident recorded in pass 30, no measurement-row member-length change occurs here. I independently inspected all 22 current new-version archives as static tar data. Every normalized member-byte total and count matches exactly, and each actual compressed length is within the original 64-byte tolerance of its recorded value. No product code or archive generator was executed by this review.

The actual inventory validator log `/private/tmp/s954-fx270-final-inventory.log`, SHA-256 `1035c808a6102781519afddf51640bd67f02620af973ea5549505f0c92ee9bc3`, records 27 distinct workspace README and 22 publishable inventory validations. The five measurement policy tests pass in `/private/tmp/s954-fx270-final-measurement-policy.log`, SHA-256 `68654c3017bfe7a0643f65394a3b19f32e7744d07778cc8ae93982b3b5f78c61`. Inventory validation is not README example compilation or a verified publication dry run.

The completed scoped receipt `/private/tmp/s954-fx270-native-batch/receipt.json`, SHA-256 `9e079730c5df99f96c735e6a0a761a27e8d243a086559ceb45f4d7d83213cbc4`, records all 12 steps exiting zero. They comprise two build batches, seven exact version-consumer cases, both-binding WASM check, affected seven-package all-target/all-feature clippy with warnings denied, and formatting. I authenticated every recorded log and all seven source hashes. Every selected native case actually reports one passed, zero failed and zero ignored, rather than an empty filtered invocation. Builds use the recorded ordinary test profiles in the isolated target, with no release LTO or normal product rebuild. This supplies the previously pending affected version-case outcomes, not a repeated full native suite.

Current source identities include README.md `d46153f87c9d3fcf5fcd38052243dd14251c8bacf6755ed20fd7864ac9a72d1a`, scripts/readme_doctests.py `b7b920382704de151ee2f09461bc7cdcb38c6eb4ce6f748bddc2cb60d106290c`, and docs/sprints/BACKLOG.md `08a7929de1f09b30faebb05cb72eda2317777a7ebea69910517b828eeb43e427`. The full source map over the 22 paths in the update receipt plus that script and backlog has SHA-256 `9235e6e3d45865d11a0b30a012a89f2a2fbc17f375f1c3c3383634c37b637b90`, using sorted path keys, SHA-256 hex values and compact JSON separators.

The old product/native/performance/RSS/normal/instrument/noassert applicability assessment remains unchanged. Included documentation values are not runtime algorithm inputs, and no actual producer is relabelled. New-version README example compilation, clean verified publication, hosted build-only execution, downloaded artifacts and installed checks remain pending at this boundary. The complete final source-availability checkpoint still requires its authenticated clean ref, evidence packet and explicit routing. This supplement neither executes that checkpoint nor approves publication or sprint closure. Only this review file was appended, no runtime or build was launched, and the write lease is returned.
