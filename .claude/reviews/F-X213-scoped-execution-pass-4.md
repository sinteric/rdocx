# F-X213, scoped execution, pass 4

**Reviewed**: The prepared current-R5 executor, queue and readiness document in `/private/tmp/S91-F-X213-hld-and-gates-prepared/current-r5-runtime`. The executor is 83 lines. All command objects and the reused lifecycle/archive implementations were read.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Independence and authority

This pass reviews the executor authored by fonts. I authored earlier F-X213 product proposals and the HLD proposal. This is not an independent whole-implementation or HLD review. No product import, Cargo command, installed test, environment change or source edit was performed.

The reviewed runner SHA is `b5ddb883d94a58ed9ac6858e5d42d2661c712666b4594eef305c8ee5c393aaaa`. The queue SHA is `cb2d31cbf6f84de7fdb69d4b1a5dfec5bcbeae85963ac4f878680b8ceea40c34`. The preparation receipt SHA is `e18e91bb941f216d8bccb56e19e47d4aaee9b04d090cafbd3cbab9ba20735ee9`.

Current source authentication matched all 2,852 tracked byte, size and mode records in source snapshot `235729b8feb0a8bc74bc27eeffc0a888819cb5c0762f6ce97e8f996a2120f31a`. HEAD is `3723dea217efdd1d1ee37201ec73b0fd1b5dab20`. The staged-entry digest is `4ef9a90dd6058f25eb6272953a93ba62a8a2732c9acf321059774b9f0f880ed0`. The snapshot and helper references match their recorded hashes.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Aspects checked

- Correctness: The queue preserves all 15 unique existing native selectors and both existing installed selectors. Native focused commands use exact selection, one-pass summaries and exact named outcomes through the reused helper. Full changed-crate partitions collect an actual filtered inventory before execution and compare complete result membership. Existing ignores receive no acceptance credit.
- Contract: Changed `rdocx` and `rdocx-py` checks and clippy, full native library/regression/integration partitions, both oracle-pin exceptions, doctests, fresh Word build/install, the two installed controls, strict typing/stubtest, both WASM graphs, deterministic hash, publication and policy remain present. No PPT wheel or full sprint test gate is invented for this bounded Word change.
- Panics and process ownership: Assertions fail closed. The reused command helper spawns inside its cleanup-protected block, records the active owned session, bounds waits and TERM/KILL cleanup, checks whole-group disappearance and leader reaping, and retains failed rows. No cache deletion or timestamp mutation is implemented by this executor.
- OOXML and archives: The executor does not alter OOXML. Publication uses verified dry-run rather than no-verify packaging. Capture authenticates current source members/modes, VCS HEAD, temporary archive copies, registry checksum and command-window freshness. Tuple differences are recorded as required carrier work and do not imply metadata acceptance.
- Tests and provenance: Selector declarations were checked statically against the current guarded sources. Build and install receipt references are source-bound and hash-bound. The installed probe compares the imported native extension against the fresh wheel, and the wheel records its complete member inventory. This pass does not claim actual collection or installed success.
- Structure: Reuse is localized to the existing reviewed lifecycle and archive functions. The packet adds no product file, module, trait, dependency or public API.

## Execution boundaries

Admission remains ROOT-owned and pending. Runner lines 15-21 require a hash-bound ROOT admission and actual latest-producer reference. The terminal and source-derived cache handover assertions are ROOT attestations, not an automatic producer-schema validator. ROOT must authenticate the genuine latest terminal and closure before writing those attestations. No prepared boolean or future terminal provides that authority.

Native partitions require real absolute Poppler identities and version outputs through runner lines 50-54. The inherited LibreOffice controls assert their pinned build in the actual tests. ROOT must supply the intended LibreOffice path in the admitted base PATH and retain the applicable oracle input authority. The installed stages require the actual fresh source-bound build and install receipts through lines 40-47, with the exclusive installed-environment lease retained.

All scoped execution results remain pending. A clean executor review does not turn previous compile failures, partial native successes, HLD adoption or this static inspection into gate success. Measured carrier changes require ROOT review and renewed source guards before final policy acceptance. This pass ends with this report.
