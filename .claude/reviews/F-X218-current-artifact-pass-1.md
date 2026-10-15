# F-X218, current artifact addendum, pass 1

**Reviewed**: Frozen current-artifact capsule `3b06283e3320b6ca5ee05088fc791f7ec8f5a88ce59e543eddcd4a0052bfbf04` on worker `work/f-x218-codex`, HEAD `a86e1f9142e602c6e34d4801136c8b33efb652d1`. Exactly two metadata carriers differ from the prior zero-finding completion composition, two additions and two deletions. Production sources and the seven HLD carriers remain unchanged. This pass reviews the current artifact delta, not the integrated sprint.

**Verdict**: 1 defect, 0 smells, 0 nitpicks. Current archive, wheel, installed and native semantic evidence authenticate. The remaining defect is the recorded measurement date.

## Defects

### D1, refreshed archive tuple retains the old measurement date

`crates/rpptx/README.md:31` and `scripts/readme_doctests.py:382`

The new617,701/3,250,880/16 archive tuple is recorded as measured on2026-10-12. This current artifact was produced and measured on2026-10-13. The actual before-metadata measurement record has filesystem time2026-10-13T13:16:00 UTC, the final default-verified receipt2026-10-13T13:16:25 UTC and the retained final archive2026-10-13T13:17:11 UTC. The prior archive has a different tuple and cannot support the new row's older date. Reconcile the existing README date and matching `ARCHIVE_REMEASUREMENT_DATES` entry with the actual current measurement. This is a metadata defect, not a failed package or runtime control.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Not found

- Correctness: zero additional findings. All3,203 current source bodies authenticate against map `b356ee21cb1acbdea972ca823309b20918ca17a22157acf8899f998313c6d2ed`. Only the README archive tuple and matching policy tuple differ from the previously reviewed completion map. The independently reviewed concrete validator relocation and all production bodies remain exact.
- Contract: zero additional findings. Current PPT-only installation retains the complete16-member Word215 distribution, including recorded origins, and replaces only the reviewed PPT provider without dependencies. Prior sibling and integrated union qualifications are explicitly historical. No later F215PPT, F211a, F211b or F220 installed union is claimed.
- Panics: zero findings in the metadata delta. Only numeric tuple values change. No production parsing, indexing, arithmetic or allocation changes appear.
- OOXML: zero findings. No XML or package implementation changes occur in the delta. The existing native source-semantic qualification remains distinct from exported HTML and Markdown validation.
- Tests: zero additional findings. Recorded actual collection and verbose execution contain exactly the HTML and Markdown F218 nodes. Both pass, strict PPT mypy reports success and complete stubtest exits zero. All eight installed rows authenticate with exact raw-log hashes, natural terminal groups and reaped leaders. Complete paired inventories after controls equal the post-install inventory, and Word215 remains exact.
- Structure: zero findings. No product file, module, trait, generic, dependency, wrapper or feature flag is introduced. The measured source stays in existing carriers and the two metadata edits retain the original policy gates.

## Actual archive authentication

Independently inspected all16 regular members of archive `46d5f12379eb36ffac37391b2f21f7bcc5969b8100a2c1cab51698c0c858e62a`, checked every recorded hash and size, and compared all ordinary members with current source, including README and Cargo.toml.orig. Generated Cargo manifest/lock and VCS carriers remain explicit. VCS identifies the actual worker HEAD and dirty source state.

Actual compressed bytes617,701, normalized member bytes3,250,880 and member count16 exactly match the current README and policy tuple. Raw members total3,250,899 bytes. The19-byte difference follows the existing dirty-marker removal and neutralized VCS SHA rule. The archive remains below10MiB. Actual default-verified offline packaging receipt `b70b3ccfbe369a59073cf4f6d9f0b36ac2487313afa801c86e6c498f88f80ca3` authenticates natural terminal execution. Inventory `91e3550509f86bf43039c1f9b536a0ba465f146b453e2c0b320b1122fd2affc4` and every member authenticate. No publication is claimed.

## Current wheel and finite installation

Wheel `a83dbf92b78ab9a46b84a3344653afa616db7c6a5adcb2c90dbd7482121ca9e7` contains15 unique members and20,834,588 bytes. Independently verified all member hashes/sizes, complete RECORD digests and sizes, local Python/stub bytes, arm64 Mach-O identity and native target equality. Native SHA is `e1daa380eb5ddf60196a48a4278571f756fd0166bda77071dc15dce0e3b37c61`. Producer receipt `e6c716b81beb8d7adea968559844af227c0d623e6823ffe907dc216f3a8876ac` authenticates four actual rows and unchanged complete paired read-only inventories. The wheel production bodies remain identical across the later two numeric metadata edits.

Installed receipt `f1dcbcff8ad8bdc058be6f1b099a1167793ce68f229e7df8a0c04feb485042b8` authenticates all eight actual rows and raw logs. Every installed ordinary wheel member equals the reviewed wheel. Exact collected and executed nodes are `test_fx218_detached_semantic_export_and_atomic_publication[html]` and `[markdown]`. Complete Word215 hash remains `74e74b3c0a6e384988eb978069287f4330a127f71513b89847a2b20bce265851`. The final current PPT distribution hash is `aeb37bd23e738d4a9cb2b8acd08f696a803627605c685fb4533d5f7901fd7828`. This evidence qualifies the recorded current feature pair only.

## Native semantic authority and boundary

Native assessment `f39deedfb19a28394c949703d87ac55c4f10e5cc1c26002b3ae9e37ad7b1b9bc` and every listed body/hash authenticate. It retains actual source order/IDs/hidden flags, title/body levels, tables, picture metadata/media and notes across two normal native source/save/reopen cycles. First/reopened words and raster hashes agree. Two layout bounding-box records differ by a maximum0.0218 point. Exact bounding-box comparison remains false. No tolerance promotes that difference, and the tiny base picture is not credited as visually discernible. HTML pixel parity and integrated union acceptance remain outside scope.

Read canonical workflow/microscope, approved plan, current in-progress status, progress and prior completion review `00b74c0c5127903719926c3c1d930164c8587de1d73a9bfccab47ad0e2f92b62`. Full diff remains `a7a2a4563183f05d10dfaaa7d0e1fcd5868738212769e6789b9e91ac901ca89c`. Worker HEAD, branch, physical/logical indices and every mapped source body remain unchanged before and after this review. Only this canonical report was written. No source fix, progress edit, runtime, CP, Cargo, UI, index, Git reference, ledger or baseline mutation occurred. End this review pass and return to the orchestrator for separate metadata remediation.
