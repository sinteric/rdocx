# F-X224 golden raster correction, pass 3

**Verdict: 0 blocking findings, 0 structural smells, 0 nits in the bounded three-value correction.** Reviewed frozen `scripts/golden_pixel_manifest.json` SHA `bd4c48a75d081b35199ce6aa1da244d935e5cef424d3d2dfd0f7371f3c028dba`, its F-X224 plan amendment and HLD15 clarification. No source, baseline, test, tool or other artifact was changed or executed. Only this review file was written.

## Attribution and exact change bounds

The diff at `scripts/golden_pixel_manifest.json:7` through41 changes the attribution reason and only contract, proposal and report decoded RGBA hashes. All four unrelated sample records, all seven1275x1650 dimensions,150DPI and pdftoppm26.01.0 identity remain byte-equivalent as JSON values. The generator and exact comparator are unchanged.

`.claude/plans/F-X224-design.md:169` through196 explicitly bounds this correction to the already approved F-X210 Georgia-to-Gelasio behavior and preserves all49 accepted product hashes. The original contract at `.claude/plans/F-X210-design.md:80` permits measured affected entries only. Its PR319 section at271 through287 specifically requires independent recomputation of these three external raster values from the integrated approved font graph, rather than copying contributed expectations or treating pixels as native metric parity. The HLD clarification at `docs/hld/15-build-and-toolchain.md:818` through825 accurately keeps external raster and product manifests separate while requiring both to receive an attributed measured behavior delta.

The causal successor receipt `/private/tmp/S91.1-primary-golden-failure-38696954013/F210-causal-diagnosis-successor.json` physically matches `e8a16487d8cec498731b1242e3fdd80461c19fd7aa39f4933621438ed62b0c74`. The original reviewed twelve-key attribution receipt matches its `c7bec6249ad56eaa34df7504cc37ae38343ce77c54173515136c71d20952f59e` pin. The actual current contract/proposal/report PDF hashes equal the three reviewed new F-X210 PDF-byte hashes. Thus this is propagation of the existing labelled behavior delta, with no newly inferred product change.

The current diagnosis `/private/tmp/S91.1-golden-PNG-current215-diagnosis-1/diagnosis-and-minimal-proposal.json` matches `297b944d109738715caafc8e41864f1b58456b89ef13c89ed819c284e62e0a05`. All seven referenced current PDFs and PNGs physically match their recorded hashes. The three candidate values exactly equal both current observed RGBA digests and the actual hosted mismatch digests at `/private/tmp/S91.1-primary-golden-failure-38696954013/Test-job.log:23166` through23168. The four other current samples equal their original expectations. Cross-host equality supports the bounded correction and supplies no reason to alter FreeType or rasterization policy.

## Actual controls and source-location custody

`/private/tmp/S91.1-golden-PNG-current215-diagnosis-1/actual-three-value-correction-2/receipt.json` physically matches `1904a52c085166e37eda2e4c1c60ddbb71686aee609c86545661214c6ab3fcaa`. All four referenced command log hashes were authenticated. The actual original updater exits0, the seven-sample check exits0, one-pixel injection exits1 with the expected contract digest mismatch, and four harness self-tests pass. The check log's final line records seven exact page-one buffers at150DPI.

The measured candidate file physically matches the canonical frozen candidate. The retained original manifest matches its old pin and the clean-source215 clone's restored manifest. `source-location-and-restoration.json:2` through9 explicitly identifies the clone where the cached generator's compiled CARGO_MANIFEST_DIR writes samples. The successor ran the original harness at that actual source location. The first preparation failure remains a separate setup failure, not a passed test or a product regression. No expectation or comparator was weakened to conceal it.

## Scope and handoff

This pass does not repeat whole-F-ID acceptance, native font metric qualification or full sprint verification. Primary run38696954013 remains failed. The plan correctly leaves actual hosted acceptance pending at `.claude/plans/F-X224-design.md:207`. The reviewed correction is ready for the separately labelled integration and actual hosted check under the existing exclusive baseline lease.

No remaining finding was established in the assigned scope. The review ends on writing this file and returns the canonical writer lease to ROOT.
