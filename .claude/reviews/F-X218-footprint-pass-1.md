# F-X218, Word footprint supplement, pass 1

**Reviewed**: Independent bounded supplement on `work/f-x218-codex`, HEAD `a86e1f9142e602c6e34d4801136c8b33efb652d1`. Two files, two additions and two deletions since the clean whole-feature review. Only the Word archive measurement row in root README and its matching policy tuple changed. This does not repeat or replace the whole-feature review.

**Verdict**: Zero defects, zero smells, zero nitpicks in this supplement. Native PowerPoint source authority and full-feature completion remain pending.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Aspects checked

- Correctness: zero findings. `README.md:41` and `scripts/readme_doctests.py:405` agree on3,650,023 compressed bytes,51,457,596 normalized member bytes and37 members. Independently read both retained archives, authenticated every member hash and size and reproduced the existing normalization at `scripts/readme_doctests.py:1292`. Pre-edit actual archive is3,650,023/51,457,596/37. Final actual archive is3,650,036/51,457,596/37. The final13-byte compressed difference is within the unchanged64-byte tolerance at `scripts/readme_doctests.py:394`. Exact normalized size and count match and the archive remains below10MiB.
- Contract: zero findings. This is the explicitly authorized root Word metadata reconciliation for the admitted F218 dependency graph. Every ordinary final archive member equals its actual current source, including root README and Cargo.toml.orig. The packaged rpptx Cargo.lock record equals the current admitted workspace record. No Word production source or dependency is changed by the supplement. The recorded build mode remains package metadata measurement without compile verification or publication. No authenticated Base archive comparison is claimed.
- Panics: zero findings. The sole Python change is a numeric tuple. No indexing, arithmetic, parsing, allocation or error behavior changes. Existing archive validation remains unchanged at `scripts/readme_doctests.py:1346`.
- OOXML: zero findings. No XML parser, serializer, relationship, model or package payload implementation changes. Ordinary packaged source members remain byte-exact. Generated VCS metadata identifies the actual claim HEAD and dirty state, and the existing measurement rule removes only the dirty marker and neutralizes the VCS SHA for normalized byte accounting.
- Tests: zero findings. Authenticated actual unchanged policy successor3 runs `test_readme_depth_footprint_and_speed_claims_are_evidence_backed`, exits zero and logs one test passing plus the actual README compilation/inventory output. Final successor2 authenticates all37 current archive members and observed measurement agreement, followed by passing prose, adapter sync and diff checks. Existing normalized-size/count requirements, compressed tolerance and10MiB bound are retained. Original pre-edit policy failures and inventory setup failures remain separate failed evidence. No new test assertion or source gate is weakened.
- Structure: zero findings. Exactly two existing measurement carriers change. No product file, module, crate, dependency, generic, trait, wrapper or feature flag is added. Sprint records, plan, HLD, production carriers and previously reviewed tests remain unchanged.

## Authentication and retained evidence

Supplement checkpoint SHA-256 is `8162ca856ad13367bef41f9fb0bb8fbc114a03b560d20d08625cf8db59a6b4bc`. Exact supplement patch SHA-256 is `b969a7d31953765d864b41091fc8d1e2369246a6aae7ccc75ecb9bc9dde4522d`. Every checkpoint-listed evidence file hash authenticates. Comparing all3,203 tracked source hashes against the clean whole-review source snapshot confirms only README.md and scripts/readme_doctests.py changed.

Final archive SHA-256 is `51d16cf0fef5dec6bf9d14a1656ad79710c7393c74553c5a9d0f4e4176f0a6fb`, inventory SHA-256 `764ac9324050d10c79da4d75cf6ce699acff278943646ced49c261b6ee18d7e7`. Independently inspected all37 actual archive members and matched inventory hashes/sizes, ordinary source bytes, generated manifest identity, admitted lock record and VCS identity. The pre-edit archive SHA-256 is `6d398fa3140c5b562ac53b7f2d3647c7c6dc52b7e560c806fcab161d8e5e2525`, its independently reproduced measurement is retained without claiming an older Base comparison.

Policy successor3 receipt SHA-256 is `662b78d91048d469f00203384b3256f2cd7cdc3fdccfd97a3fdc4dfddefcf46c`, actual log SHA-256 `31fd92aefb3f061d609ddcb5c6a11df267771104fece3772fead682a8804a080`. Final successor2 receipt SHA-256 is `8643792fee01a894acb672a6b26d1140c9b3ae74bcbd8061360ae9f3ea04a678`. Their deciding rows retain source guards, terminal groups and reaped leaders. Earlier policy successors1/2 and final inventory attempt1 retain actual nonzero exits and are not credited as passes. Original full policy and immutable setup failures remain retained alongside the separately authenticated immutable successor.

## Qualification boundary and review integrity

The supplement resolves the measured Word footprint metadata gap only. Actual PowerPoint semantic source authority and normal save/reopen, remaining scoped completion preconditions and the final integrated sprint gate remain due. No ready handoff, feature completion, Office acceptance, merge, tag, release or publication is authorized by this report.

Read canonical CLAUDE/workflow/microscope, approved plan and cited packaging/test contract, progress and the prior clean whole-feature review. HEAD, branch, physical index, null-delimited logical index, full tracked diff, all3,203 source hashes and every listed evidence hash authenticate before and after this report. Physical index SHA-256 is `74d5287ceed23c1aac004126d4332e20bbc83920338bee52d0dbdc386546b5e6`. Logical index SHA-256 is `ce0be2590c52a95f9c0101559ccbbcbab4becc608667c1ab5fa8e67909e0b8ac`. Full diff SHA-256 is `7d4d4655ec21a9451913d6793bf44a7ff1c1c883263670e69f26c1578c21225c`.

Only this review file was written. No source, plan, progress, HLD, test, runtime, Cargo, Git mutation, CP, Office or shared ledger changes occurred. Review stops here and returns the writer lease to the orchestrator.
