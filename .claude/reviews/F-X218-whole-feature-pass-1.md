# F-X218, whole implementation, pass 1

**Reviewed**: Frozen implementation on `work/f-x218-codex`, HEAD `a86e1f9142e602c6e34d4801136c8b33efb652d1`. Twenty-five tracked carriers, 3,200 additions and48 deletions, covering native projection, Python, CLI, tests, seven listed HLD files and affected archive measurement documentation. This reviews the complete current code while keeping unfinished acceptance explicit.

**Verdict**: Zero defects, zero smells, zero nitpicks in the reviewed implementation. Full-feature acceptance and a completion handoff remain blocked on the qualification boundary below.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Aspects checked

- Correctness: zero findings. Native source-order projection, slide/title identity, nested lists, inherited run formatting, tables, notes, hidden selection, owned images and hyperlink semantics retain the clean native pass2 implementation. Python methods at `crates/rpptx-py/src/presentation.rs:1386` through the save/export counterparts call that same native route. Detached results at `crates/rpptx-py/src/presentation.rs:823` own text, tuple image bytes and diagnostics. CLI semantic dispatch at `crates/rpptx-cli/src/main.rs:476` precedes all font/layout/rendering work. Native options and caller format determine output rather than a pixel export.
- Contract: zero findings. Python keyword defaults, string return shape, detached-mode refusal, explicit export-result companions and save diagnostic tuples match the approved plan and stubs. Export uses shared immutable presentation access and does not increment revisions or invalidate held owners. CLI rejects every supplied rendering option, including explicit default DPI/quality, and rejects semantic flags on other formats. Raster defaults remain150 DPI and90 JPEG quality. Resource outputs use the whole-set input/no-overwrite/force policy at `crates/rpptx-cli/src/commands.rs:403` and the existing staged publication at `crates/rpptx-cli/src/commands.rs:1239`. The seven HLD changes describe this current contract and preserve the semantic versus pixel-layout boundary.
- Panics: zero findings. The unchanged native bounded image validation and checked traversal retain the clean native microscope conclusions. New binding getters construct owned immutable Python values, option types are checked before output and native errors are mapped through the existing family converter. CLI image outputs use generated digest filenames and checked publication paths. No new unchecked model indexing or allocation from caller dimensions is introduced in these surfaces.
- OOXML: zero findings. Export reads actual slide/resource owners, uses normalized case-insensitive OPC slide target identity and retains byte-exact image resources. It does not serialize a modified source package or reinterpret opaque source payloads. Native completeness validation runs before image destination or asset creation. Both public surfaces consume this established native result rather than reconstructing another semantic model.
- Tests: zero code findings. The genuine Base public probe, native distinguishing RED/GREEN controls, HTML recovery negatives, official pinned cmark records, fresh installed two-format Python control and actual CLI integration qualify the implemented paths. `crates/rpptx-py/tests/test_documented_examples.py:9251` checks detached ownership, bad option types, repeated picture deduplication, atomic collision refusal and held owner/source equality. The publication-contract assertion at `crates/rpptx/tests/integration.rs:21225` now distinguishes admitted unconditional decoder edges from optional render edges. Original broad failures, absent historical fixtures and pending corpus/native Office qualification are retained separately, not treated as passing or fabricated code defects.
- Structure: zero findings. Concrete native options/results remain in existing carriers. Python uses one frozen result class and direct existing methods, while CLI delegates projection and reuses the existing output publisher. No new product module/file/crate, trait, generic, feature flag or cross-family shared facade helper is introduced. Already-admitted existing workspace decoder edges and the two Cargo.lock edge additions are unchanged from native pass2. HLD03 describes those actual dependencies.

## Authenticated source and native continuity

Whole review checkpoint SHA-256 is `af0e124bc265393b6f431a8426f1ce03cb82bca822a8dc534e359689be6ee74a`. Full frozen source snapshot SHA-256 is `fc5b9896d01108dd201fc57cb86209d70bec9a0956782d90ade970bce86e77d1`. Complete tracked diff SHA-256 is `c5d0d755167e141126e06fb43779925124782acd9d878a4a86cb34894766aed1`.

Native product html.rs/lib.rs, manifest and lock bytes exactly match the clean native pass2 source checkpoint `b96e9c2cd9439f9bd620f17be903b9afe9ffa9204084b34c48b6a34cec8bb79f`. Its actual21 controls, genuine original three RED controls, stronger missing-JPEG-EOI RED, official cmark seven inputs/three grammar negatives and bounded PNG/JPEG/GIF/TIFF/BMP validation remain authoritative. The new publication assertion changes only test expectations for the already-approved graph. It does not remove optional rendering coverage or widen native acceptance.

Every checkpoint-listed evidence file and actual row log hash authenticates. All inspected execution rows, including failed rows, retain no timeout, owned group gone and leader reaped. This audit performed no Cargo, product, oracle, CP or Office execution.

## Public and regression evidence

Fresh installed PPT receipt `66927ec7a5aed1afb95dd4125114f9dd35f17c8abf32c5f95f0ea8471ea93c68` records eight actual terminal zero rows: complete paired inventories, isolated reviewed PPT installation, exact collection of HTML/Markdown parameter nodes, two actual public passes, strict mypy, complete stubtest and after-inventories. The installed Word distribution remains exact in that retained paired evidence. This is the requested current Python Rust surface, not a source import masquerading as installed acceptance.

Wheel SHA-256 `091bb9adebfc4cc52e10ad551a7ae2dccd99e9d9b314fbe658f74aaed0276532` authenticates20,834,189 bytes and15 members. Independently verified every member hash and byte size, complete RECORD digests and exact current Python/stub source bytes. Retained wheel inventory also authenticates arm64 native target equality. No environment was modified during this review.

Public CLI receipt `24ee89f0aafbfcae5b40fd1525ef3b7c54202877dcf77e14a1f6220f552a7ed8` records deciding CLI and changed public-crate all-target strict Clippy success. Public regression receipt `e68b448b5627ff6c6d33fcca22e9a1358edc1617e70063df52a29356f1c6d6dc` records all49 CLI tests passing and both actual WASM graphs passing. The six pinned broader corpus inputs produce twelve successful CLI semantic exports across32 source slides with exact unchanged inputs and actual OPC slide identities. This is authenticated breadth and not a substitute for independently expected mixed semantics or native PowerPoint source authority.

Contract successor receipt `be7826a776654203596aed69a3ac91990f9b35fe1d736b5b0d365972261ff770` records the corrected publication assertion, formatting, prose, skill sync and all49 deterministic hashes passing. Final metadata receipt `583577358d901268de817aa4014eeb4794f51ac5e97007675b5fbfc813e48e1e` records affected archive measurement checks and repeated formatting/prose/sync success after final documentation updates.

## Verified affected archives

Final verified dry-run receipt `8408ac8d367c19953719f9794e9f299d68e8724e52e3d3f2549d63369534945e` records actual Cargo verification and no publication. The inventory points at the verified tmp-registry artifacts rather than an older package copy. Independently verified archive hashes, all member hashes/sizes and every nongenerated member against current sources, mapping Cargo.toml.orig to its actual source manifest. Generated VCS metadata records the claim HEAD and working diff. Normalized member sizes independently reproduce the existing measurement rule, which removes the dirty marker and neutralizes the VCS SHA.

| Crate | Compressed bytes | Normalized member bytes | Members |
|---|---:|---:|---:|
| rpptx | 617,784 | 3,251,084 | 16 |
| rpptx-cli | 56,773 | 256,289 | 8 |

Both archives are below10MiB. The recorded PPT compressed tuple is617,783, one byte below the final artifact, within the existing64-byte compressed tolerance. Exact normalized bytes/member counts and current payload hashes match. Archive inventory SHA-256 is `6fd2bebaf97b392d3a27d570523774953f0ad578a65e9fe7d32273a598a95aa3`.

## Qualification boundary

The original broad native result remains327 passed, seven failed and24 ignored, actual exit101, receipt `3ecabbed79390ffd4a7bf0b26e5bb2e22a62f28d3c5f4bad3ffb36f245ed6866`. One failure was the now-corrected publication expectation and has its exact passing successor. Two python-pptx corpus comparisons failed during UV cache setup and still need authenticated successors. Four historical timeline/SmartArt oracle controls lack their genuine files and remain explicitly skipped/uncredited under the root's stated story scope. No broad-suite pass is inferred.

The independent policy receipt `75a34ae9eb98fcb4ad231ad06b17c4d427cc56772fc4d126806d3aa08311b9a7` retains a failed immutable-registry policy setup row. Its deciding successor is still due. The Word footprint mismatch is documented as a23-byte workspace lock edge delta, with Word source carriers unchanged. Its analysis explicitly does not claim an authenticated Base archive comparison. Integrated root-owned metadata reconciliation remains due and is not an inferred product fix or a reason to alter Word sources in this worker.

Native PowerPoint source title/body/level/table/alt/notes/hidden authority and actual unchanged native save/reopen remain pending explicit activation permission. Raw OPC expectations and producer-labelled corpus input cannot substitute for that use. Final integrated scoped qualification, resolution of these required gate gaps and a valid handoff remain due. This zero-finding implementation review does not waive them or authorize sprint closure/publication.

## Review integrity

Read canonical workflow/microscope, approved plan, prior native reviews, progress and frozen public/final checkpoints. All3,203 frozen source file hashes, HEAD, branch, physical index, logical index and complete tracked diff match before and after writing. Physical index remains `74d5287ceed23c1aac004126d4332e20bbc83920338bee52d0dbdc386546b5e6`. Logical index remains `ce0be2590c52a95f9c0101559ccbbcbab4becc608667c1ab5fa8e67909e0b8ac`.

Only this review file was written. No source fixes, test runs, runtime, Cargo, Git mutation, CP, Office, intake, plan or shared ledger changes occurred. Review stops here and returns control and the writer lease to the orchestrator.
