# F-X268, final performance repair, pass 30

**Reviewed**: Working diff on `sprint/s95.4` at `7ca9adb950a82aa6094fe3da897972651c98338f`. Eight tracked modified files, 411 additions and 51 deletions, plus the existing 86-line memory rider review. This is the incremental debug-reference, stale-expectation and final-evidence repair, not a new whole-feature review.

**Verdict**: 0 defects, 0 smells, 0 nitpicks in this bounded repair.

## Contract and source

I read the canonical repository guidance, workflow and microscope command, F-X268's approved contract and amendment, the applicable F-288 retained-marker contract, and F-X267 A15. F-X268 remains in progress in both shared delivery records. The testing HLD amendment describes the actual debug policy and stays within the existing HLD impact list. The README reference correction is explicitly included in the plan extension.

The six auxiliary counters at `crates/rdocx/src/document.rs:7606` through the namespace-span unit now honor the existing staging pause. They do not create another pause boundary or suppress actual unpaused production work. The fresh probe uses real frame, owner, namespace-span and relationship machinery. Its deliberately refused closed-body proof also counts outside the pause. Positive observations before and after the paused call reject a permanently disabled counter, and the zero vector during the pause rejects the original defect.

The reference clone at `crates/rdocx/src/document.rs:22529` retains ordinary typed and package source state but discards Parsed and MainPartBlocks and gives MainBodyScans a new mutex and allocation. The four consumers are reference save comparison, whole drawing canonicalization, save-equivalence fallback and the independent ZIP reopen route. Their remaining staging-clone fields are ordinary source data, independently copied state or the existing immutable package context. The repaired paths do not retain the mutable main-part proof roots being used as their own reference.

The helper at `crates/rdocx/src/document.rs:7874` reads `OnceLock::get`. A genuinely cold slot stays cold. An initialized failed parse is still compared and rejected when a fresh parse matches the current model. Fresh-model agreement remains unconditional, including when no retained slot exists. Both production callers remain inside their existing debug-only agreement checks.

No production namespace guard, retained-owner admission rule, error ordering, cache invalidation rule or public signature is relaxed. The compatible fixture correction preserves the existing refusal boundary rather than modifying it.

## Discriminating tests and affected evidence

The three new tests begin at `crates/rdocx/src/document.rs:54118`. They distinguish actual paused proof work, readonly parse state and independent reference ownership. The parse test covers an empty slot, stored failure, matching stored model and changed current model with no retained slot.

The rich fixture enters through actual saved bytes, including a single-quoted root namespace. It contains an imported qualified legacy begin marker, distinct current and default values, its physical bookmark pair, W15 repeating owners, a header and an exact foreign RawXml sibling. It explicitly verifies that the foreign sibling is parsed as RawXml and that its full bytes survive save. This is a compatible unknown-property control. The earlier pPr attribute fixtures exercised an intentional unsupported namespace-owner boundary and remain preserved as failed setup attempts.

The isolation test checks both cold and deliberately warmed child facts. Snapshots include all eight child proof/staged occupancies, six frame proof occupancies, retained part allocation identity and bytes, root/head/tail, Parsed slot occupancy, shared scan child identities, frame facts and serialized record. Save, ZIP reopen and a whole-proof-disabled save must preserve these original snapshots. The reference must have no retained Parsed or MainPartBlocks and a different empty MainBodyScans allocation. Reopened legacy current/default projections and typed document XML agree. These assertions reject merely pausing counters while still warming or overwriting the original shared proof roots.

Actual scoped execution is retained in `/private/tmp/s954-debug-reference-isolation/execution/resume-receipt.json`. The original four native failures have passing affected successors. Sixteen of the seventeen selected rdocx cases passed in that recorded batch. The remaining rich isolation case initially failed during fixture setup. Its final compatible successor passed the fully qualified exact test in `isolation-compatible-opaque.log`, one actual case, zero ignored, in 0.30 seconds. The intervening zero-case command and both setup failures are not passes. `clippy-after-fixture.log` records the final affected rdocx clippy success.

The other selected tests cover actual staging, source changes, reopen fallback, physical owner publication, retained pictures, hyperlink publication, scoped normalization and the three additionally affected block-read assertions. The final fixture-only amendment does not change those callers or their test bodies. Their prior successes remain applicable without rebuilding each one again.

The long body-lookup linearity test measures A9 lookup units already excluded from old debug-reference accounting. It does not measure the six repaired auxiliary counters. MAIN_BODY_BLOCK_SCANS consumers are actual owner/layout reads, which these save, canonicalize and ZIP reference paths do not invoke. The independent reference change therefore does not invalidate that test's measured lookup facts. Repeating its 24.5-minute execution solely for this repair is not required.

The batch also records successful affected debug WASM, clippy, formatting, prose, adapter and diff checks. These are actual scoped observations, not a replacement full-suite pass. The receipt correctly retains its initial failed rich-case status. The separate final exact log supplies that case's successor rather than rewriting the old aggregate.

## Existing expectation corrections

The complex-field assertion at `crates/rdocx-oxml/src/text.rs:19032` now expects the retained `q:fldChar` element marker from its imported alias input. The existing assertions continue to require the changed canonical dirty attribute exactly once and preservation of unknown producer content. This corrects the expectation to the retained-marker contract and changes no parser or writer. The complete affected test passed.

The SmartArt assertion at `crates/rpptx/tests/integration.rs:4075` now expects 15.24 points, as required by approved A15's native list body insets. The native 193548 EMU value divided by 12700 is 15.24. Group scaling does not change this stored local inset. The complete test passed in 1.28 seconds, including its later font, fill, geometry, edit and rendering assertions. No geometry or SSIM threshold was weakened.

## Release inventory and official reference

The exact set comparisons at `scripts/test_sprint_workflow.py:53` now cover the actual S95.4 release-note inventory, separately for Word and PowerPoint. They retain contributor credit checks and the partial/private-reproduction and compatibility caveats. The actual release-note inventory command passed. This is evidence about the prepared notes, not permission to publish or evidence that every included issue is fully resolved.

README and the existing comparison-evidence list use the same immutable first-party docx4j module commit. `/private/tmp/s954-docx4j-link-correction/receipt.json` preserves the original 404, authenticated commit response, corrected module HTTP 200 and captured module README HTTP 200. The README still supplies the bounded Markdown importer/exporter comparison. The unchanged affected link checker actually passed, recorded in `affected-check.json`, with nine unaffected original successes retained. The comparison claim was not expanded.

## Artifact applicability and exclusions

All changed runtime branches in document.rs are guarded by `debug_assertions`. Both new helpers and their four clone consumers and two readonly consumers are debug-only. The six counters retain their prior noassert implementation. All new tests are debug-only. The OXML and PowerPoint changes alter test expectations only. No production dependency, feature, binding, crate manifest, build script or product resource changes in this batch.

Consequently the normal release and instrumented release runtime paths remain source-equivalent to their frozen `fc670a18` and `7ca9adb9` producers for the qualified workloads. Their original source/build/artifact labels must remain. The opt-level 1 noassert A9 artifact likewise excludes the changed code, so its 28 vectors and 870 comparisons remain applicable without a new A9 build. This is an explicit source applicability assessment, not a claim that an old artifact was compiled from the working tree.

Ordinary debug native artifacts do include the repair. The refreshed affected debug executions qualify the changed callers and counters. Unaffected prior native results require the caller argument above, not blanket adoption of every old debug executable. In particular, the old private opt-level 2 debug history numbers are not adopted as execution on the repaired artifact. The same immutable S2 history case with root debug assertions disabled is a separate pending actual execution and assessment at this review boundary. Ordinary history assertions remain active in that configuration.

The root README URL also changes an eventual source-package input even though neither Python project README nor runtime product code changed. Packaging and data-footprint observations that depend on final source contents need their own bounded actual evidence. This review does not adopt unfinished packaging measurements.

## A7 context and remaining gates

The existing `.claude/reviews/S95.4-memory-rider-pass-1.md` separately reviews the 84 RSS children, including 60 historical children and 24 current children, and their original producer labels, calibration and pressure limits. It distinguishes the repeated 100-value RSS loop from the independent distinct 100/400 history case, lazy activation from the font workload, actual native clone from Python's unavailable equivalent, and separately reported parsed-model pressure context. No layer deltas are summed, no hard RSS cap is invented and no successor debug history is silently inferred.

That report also reviews six genuine Word fallback cases with 18 raw fresh-target timings and actual correctness checks. The signed package and embedded cases retain genuine package-signature validation and distinguish opaque attached VBA evidence. PowerPoint timings are not substituted. These disclosure observations are applicable to unchanged normal release code, with their documented workload limits and no invented fallback timing ceiling.

Cell.add_picture's original performance failure remains open. The post-RSS load observation of 18.9521 did not meet the predeclared condition below 18, so no successor timing was executed. This pass supplies neither that missing outcome nor whole-sprint acceptance, final packaging evidence, publication readiness or release approval.

## Defects

None found in the bounded source and evidence scope.

## Smells

None found.

## Nitpicks

None found.

## Not found

Correctness, contract, panics, OOXML preservation, test discrimination and structure produced no findings. No hidden counter suppression, cache-sharing reference oracle, warmed readonly getter, production refusal relaxation, threshold weakening or artifact producer relabel was found. This review executed no build, test, product import, benchmark or native application. Only this review file was written. The pass ends here and returns control to the orchestrator.

## Supplemental final archive inventory review

**Supplemental verdict**: zero defects, zero smells and zero nitpicks in the bounded archive-row delta since the original pass. The complete working diff now spans 15 tracked modified files, with 437 additions and 75 deletions. The additional scope is eight existing README measurement rows and dates, their two existing script dictionaries, and the explicit final inventory lease in the existing F-X268 plan. Product source and test changes remain those reviewed above.

I read `/private/tmp/s954-final-archive-measurements-round1.log`, verified its SHA-256 `5719a9f467f88fbd963d3d7929c80669cfcffb8539fb04de0afc2b044ac18163` against `/private/tmp/s954-final-archive-inventory-update.json`, and compared every changed tuple to the actual log, script constants and README row. The eight packages are oxml-layout, oxml-pdf, rdocx, rdocx-layout, rdocx-oxml, rpptx, rpptx-layout and rpptx-render. Each date changes to 2026-10-27. The raw measurement output still displays the old date metadata that preceded the dictionary update. That is not represented as a separate earlier execution of the newly observed values.

As an independent static source check, I inspected the eight existing generated archives under `target/package` without invoking Cargo or importing product code. Their VCS entries identify source `7ca9adb9`. Each archive's normalized member-byte total and member count equals its updated tuple. At this observation, compressed lengths either equal the logged values or differ by one or nine bytes, within the unchanged tolerance. Source members match current source files, except README rows in five archives that the concurrently running validation had not yet regenerated. Those differences are the intended measurement row and date refresh, not runtime source differences. These observations do not assert completion of that concurrent validation.

The older `/private/tmp/s954-final-archive-inventory-before-readme-refresh.json` and retained archive set identify source `665b3e34`. They are not used as authentication for the new measurements or relabelled as current-source execution.

The row update can change its containing README member's length. In particular, rpptx-layout's compressed value changes from five digits to six, adding one byte to its own row. My initial static archive sample still contained that package's old README and therefore did not establish a stable self-reference size. The actual final validator subsequently detected exactly this one-byte difference, preserving the failed first outcome. Root corrected the recorded normalized member total from 594081 to 594082 in the script and that README. Its actual affected validator then passed. Compression can also change with content even at the same width. The existing 64-byte tolerance remains. The normalization mechanism at `scripts/readme_doctests.py:1291` remains unchanged: only VCS dirty state and the fixed-width source SHA are normalized for member-byte accounting. Member count and normalized member bytes remain exact requirements. The compressed-size ceiling and toolchain-sensitive comparison are also unchanged. No runtime throughput, native parity, compatibility, install-size or behavioral claim is expanded by these archive rows.

The plan's bounded inventory lease names the same eight packages and restricts reconciliation to observed bytes, member counts and dates. This is consistent with the delta. Archive inventory measurement uses the existing no-verify Cargo packaging route with local source patches. It is not a successful workspace publication dry run, registry qualification or hosted-CI result.

### Current source and evidence applicability

Only `crates/rdocx-oxml/src/lib.rs:1` and `crates/rdocx-layout/src/lib.rs:1` include the affected README files through crate-level `doc = include_str!`. For those libraries the updated included text changes documentation. The other refreshed README pages are packaging and documentation inputs, with no runtime inclusion at these library roots. No runtime logic, API, dependency, feature, manifest, build script, font, rendering baseline or other product resource changes in this supplement.

Combined with the debug and test exclusions established in the main pass, the current delta preserves applicability of the frozen normal and instrumented release observations with their actual `fc670a18` and `7ca9adb9` producer identities. The original opt-level 1 noassert A9 observations remain applicable because the changed runtime branches are excluded and these new README values are documentation, not executable algorithm inputs. This is source-based evidence reuse, not an execution at a new source SHA or a claim that crate archives or rebuilt binaries are byte-identical. Packaging inputs changed and therefore retain their own actual validation boundary.

The current reviewed file identities are:

| File | SHA-256 |
|---|---|
| README.md | `e9c73de7c18877a87f7ea7622a9cc33d442d462f7d603eabd17d022f3dd7dbd4` |
| crates/oxml-layout/README.md | `c8f1d01d0e36075a95b96ae934cc0a2479f72cc35a76cbc2c8066a8e761dedd5` |
| crates/oxml-pdf/README.md | `710e4c3e360dc70e1d413b0edbb9486a650eed90d27d65523b76930da9fa4ed2` |
| crates/rdocx-layout/README.md | `1963e15eed59b87b1459ddc6bad1969be7d0497087b79448b24739e9573fa4b3` |
| crates/rdocx-oxml/README.md | `00788e529e2233c667a0d032e939be1a1491b451201c44f0c755a4322af6b9e8` |
| crates/rpptx/README.md | `a5cf743a9b85bbb876ab53f5283a8106b46fb952786735e342512c8ec5f9f19f` |
| crates/rpptx-layout/README.md | `be1d0795b9a279cc590eaef080e51ea98e47b7e83843bd8b070d2a309a7b5612` |
| crates/rpptx-render/README.md | `9312dae570b55fedf8fa73ea5345506e4918eeaa811fddfa181cccc617f48272` |
| scripts/readme_doctests.py | `be6e9a60c430a06a19cecff773eef9a7710ebed85481aa536f9fcbcdfb167bb7` |
| .claude/plans/F-X268-design.md | `3fbf0eb9fda2a4753e78546e808397906dc3c83e9fd07c72457af7927ab560be` |
| crates/rdocx/src/document.rs | `f7755982ae2fcb4ad8a995fdf8eec6461e11b0933d7d0e3bc103269d0468ebdc` |
| crates/rdocx-oxml/src/text.rs | `d3d405dccd93e9c8d62b3d51ab2fd1b54677e0ca4be61b84cedf09838d0fd853` |
| crates/rpptx/tests/integration.rs | `aad21923353dff5fc7832887cc34c73581ca0fd98bdf6637b884a6bace6e8939` |

Full inventory validation was still running at this supplement's boundary. Cell performance, package publication dry runs, hosted CI and whole-sprint acceptance are not passed by this review. No compiler, test, product import or measurement process was launched. This supplement used static source, log and archive reads, appended only this report and now returns the write lease.

### Actual one-byte inventory correction

The preceding source hash table records the first supplement's boundary. Its two changed identities after the one-byte correction are `scripts/readme_doctests.py`, SHA-256 `4c39001b9e6fea9440dc7065d7bbdd5c4ab25b92818abe4ee63180e270f024cd`, and `crates/rpptx-layout/README.md`, SHA-256 `c5a44b5274b230a44b24495c021f318a551a75cf8ccba18e53ac768b2504122b`.

`/private/tmp/s954-final-readme-batch/receipt.json` records that the original permitted loop passed 21 archives and failed only rpptx-layout's exact normalized total. The affected correction then passed the actual `validate_package_archive` call in 1.38848 seconds. Actual archive SHA-256 is `f06198151d357a39176404927e1f8ea4da2a51fa608c7c657886a5b3404f67dc`, observed tuple 103942 compressed bytes, 594082 normalized member bytes and 11 members. The recorded compressed value remains 103931, within the unchanged 64-byte tolerance. The original full-loop failure is retained, not rewritten as a successful command. This factual correction supersedes my inaccurate same-width explanation. It changes neither the runtime applicability assessment nor the pending README example, publication dry-run, Cell or full-sprint outcomes.

## Supplemental pre-H plan reconciliation

**Verdict**: zero defects, zero smells and zero nitpicks in the eight-plan execution-boundary reconciliation. I read each current plan delta. The common boundary distinguishes historical approval and provider-prefix snapshots from available source-bound execution evidence, retains all actual producers and failures, and keeps Status approved until required acceptance. It explicitly preserves APIs, refusal and preservation rules, thresholds and the final full-union gate. The eventual AS_BUILT entry is authoritative for completed delivery, not a substitute for missing evidence.

The F-X268 readiness paragraph now correctly describes its integrated implementation, clean bounded review and accepted A7 evidence, with full acceptance still pending. F-288, F-X264 and F-X266 checklist wording now describes positive eventual observations instead of asking a later checkmark to certify that evidence stays pending. F-X264 A10 remains an unsupported limitation outside the gate. No new API, acceptance waiver, reporter-input substitute or threshold change appears.

The hosted-wheel sentence agrees with `.claude/commands/release.md:82`: the manually dispatched build-only run and its artifact qualification remain required release preparation before final approval. Neither the eight sprint designs nor `/verify` make it an additional sprint-close criterion. This reconciliation does not remove that release requirement or approve publication.

The dated boundary's active README batch describes its writing checkpoint. The actual batch has since completed. `/private/tmp/s954-final-readme-batch/receipt.json`, SHA-256 `147a0e31974999074da98adbe452689c9b04f5dff4f36313124c8900997471ef`, now records all 22 steps passing: the affected archive and 21 README example cases. I checked its recorded current source-file hashes against disk. The five affected measurement policy tests also pass in `/private/tmp/s954-final-measurement-policy.log`, SHA-256 `4a5698c5069f8e68ffef1e84d901a1aef4436529eda6c884fcb5d192de3528fb`. These are actual observations, not a repeat of the full inventory loop or a clean publication dry run. Cell, clean publication verification, final full-union recording and delivery/sprint review remain pending.

Reviewed plan SHA-256 identities are F-288 `56dd12ad9403d100b82c7fb4c7ae0cbcb338e97dcd0c860bb3c3595e067350ca`, F-289 `0ab29e28649f3e3d77e22eaa5e9b56d3c22de8a52b0761c775f86020218ee454`, F-290 `044e5b725e8d7e1bcdcf4254d54c4dd2cd654db2e6e1b8dc3cfdf3c931d7cf55`, F-X264 `d3fd843f713e8e0b29c835a419e4a7ffb69c44cb11f62155edb3330c055c2f0b`, F-X265 `d03e4445655aa5ae92eacac6f8c44570af3b3fb739acbe5dd1e39216dc26401a`, F-X266 `103e9b15934e6011cc210ddc88587cbeefa81fc85bb5dbf0bc2267852aed0da6`, F-X267 `527a35a29e697659ddf869a2eafe1599b63de033701a1b2aa56470821a815c15`, and F-X268 `0adb741d2409acf7a964d6cf7c39f50c48e603b0a14dab42e5a42f8130557dba`.

No additional missing product gate was found by this bounded closure-contract audit beyond the explicitly retained acceptance debt and final metadata/review workflow. This is not the final sprint review. No build, test, runtime, product import or measurement was executed. Only this report was appended, and the write lease is returned.

## Supplemental F-X270 independent design assessment

**Scope**: The corrected draft `.claude/plans/F-X270-design.md`, SHA-256 `619cea911939d9a5d0d0c35ea5f9e7283addc20b134bba82b74cdc34db11a4e4`, and its bounded ninth-story intake in the existing HLD14 and sprint records. This is a design assessment, not an implementation microscope or completed preparation claim.

**Verdict**: zero remaining design defects, smells or nitpicks. The four initial clarifications are resolved. A9 completes preparation on reviewed sprint source before close and leaves exact main/tree authentication and final approval after close. A6 uses installed distribution metadata and imported native path/hash rather than requiring an unavailable native version getter. A5 and the pinned inventory require 31 jobs, 27 successes, four publication skips and 26 unique raw payloads. The actual unchanged workflow supplies 42 Word and 44 PowerPoint priority nodes, required on both interpreters, with strict 3.12 typing and stub checks. Empty or substituted selections do not satisfy that contract.

The plan explicitly identifies the ordinary source-availability checkpoint's literal new-workflow and worker-handoff mismatch. Its narrow Root-approved canonical exception allows only the exact clean reviewed new-version sprint ref and manual build-only execution, after local scoped checks and independent microscope, with real canonical evidence and pending gates. It invents neither a worker nor consumed handoff and preserves all main, tag and publication restrictions. The user's requested preparation and recommended-choice authorization are distinguished from the later mandatory exact-main publication approval.

The retained 0.16.x and 0.14.x lines follow the recorded F-X263 choice. The next patch candidates avoid the already published versions while preserving the approved Arc compatibility exception and migration. I checked all 24 unique stored candidate responses and their hashes against `/private/tmp/s954-next-patch-absence/receipt.json`, SHA-256 `f42a1c05cd2f36deb8e3d479b217a758ad14be7f76bb24da53e0727ad1120397`. They record HTTP 404, successful empty remote-tag output and selection-only absence. Publication must recheck mutable state. This evidence does not reserve those versions.

The contract preserves the published changelog suffix byte-exactly and requires new-only family contribution inventories, hadim credit, unavailable original 404 input and native/performance limitations. Its actual carrier/history RED, negative artifact controls, verified 22-package dry run, README qualification, downloaded metadata and installed public calls are discriminating acceptance observations. The seven additional existing version-test file leases are bounded to current carrier assertions. Historical controls, separate WASM policy, APIs, algorithms, build profiles and dependencies remain protected.

Completed F-X263 supplies existing machinery without falsely marking the eight integrated product stories finally accepted. A8 retains their complete union criteria, including the failed Cell gate. Version-consumer analysis is an explicit prerequisite for source reuse and preserves actual producers. No blanket new native, timing or RSS execution is invented, and no old artifact is relabelled as a new-version build. HLD impact and exclusive files fit the requested metadata preparation. Public signatures, XML grammar and lifetime changes are outside this story, so no additional API contract is required.

This assessment permits Root's design approval decision and does not itself change draft status, start implementation, pass a gate or authorize publication. No compiler, test, product import, runtime, measurement or external mutation was executed. Only this review supplement was written. The review ends and returns the lease.
