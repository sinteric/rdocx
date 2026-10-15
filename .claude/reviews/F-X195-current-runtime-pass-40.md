# F-X195, current runtime, pass40

**Reviewed**: Bibliography-authored R2 to R3 artifact-schema correction, 3,927-byte incremental diff, and the complete new 8,467-byte final-producer helper with its manifest and admission template. Prior pass38 and pass39 closures were checked for retention. I authored the original R1 preparation. This pass independently reviews the later R3 correction and new helper, not my original authorship.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication

Packet: `/private/tmp/S91-F-X195-current-r29-runtime-r3-prepared`.
Receipt SHA-256: `56566cd13e6ac6698d4ec36296701c739497a6639b831495d59db4a1a863c148`.
Runner SHA-256: `d285b99abec9ca878f64fc3f3287e30f905a99f3f89ddfcde1643fc50d55a298`.
Incremental diff SHA-256: `33f1042e4a837f3e822afb516ccace038a4c38e2ec82ffada3a91ea0ee3ff805`.
Final-producer helper SHA-256: `1697a237200020c8bde2f235298d2497ea358ea45ca194f5aeb8319f14aa979a`.
Final-producer manifest SHA-256: `9579d5d8f208c76199159555dd3aba932c8b6351deaf551bbb12b1f3b2325af6`.

All eight receipt-listed files match their actual hashes, sizes and modes. Both helper ASTs parse. The queue is byte-identical to R2 and preserves all command, family and selector identities. All2,679 current F195 tracked guards and317 source guards match. Its branch is work/f-x195-codex, HEAD is2a9bf14072231e46fcb34df70f88cf7aa79fbd79 and semantic index hash is c964c1c38e50c6b8a84c8ea0f8cf0f5f38bd230cd96a8d542ed4c30272c83db1. Immutable oracle inputs match their hashes.

All2,714 F200 final-producer source guards match. Its actual HEAD and semantic index match the manifest. The source-map hash is0e79e818962fd14c429f5e3830e4ab313f91890c32860d2505f1898a19c2e9ba. The actual prior two-ODT receipt hash is ace9cbdf43048658d9f9e1791b825e0b947c733bab3c11f5afa2941fe400d454. It contains two exact named test commands, each actual exit0 and1 passed with0 failed/ignored. Both retained log hashes match. This remains bounded check/two-test provenance, not whole F200 acceptance. Final-producer tool paths, resolved paths and hashes match.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Aspects and prior findings

- correctness: Zero findings. Pass39 D1 is resolved at the candidate run.py:280-312. The four static owners require their exact immutable static font paths and hashes. The four genuine or synthetic owners require null oracleFont fields and authenticate the actual original bytes, face0 and exactly one matching embedded SVG font declaration. No absent static font is invented.
- contract: Zero findings. The null-field route matches the actual producer at crates/rdocx-py/tests/test_rendering_threads.py:1570-1583 and native consumer at crates/rdocx/src/svg.rs:3592-3611 and :3711. Genuine italic is bound to the guarded bundled Carlito-Italic bytes. Synthetic owners are bound to the immutable none.ttf control. Mandatory actual SVG and quadratic/cubic references remain required for all eight owners. Variable owners retain expectedSVG and pathProof requirements. The threshold remains0.99.
- panics and failure handling: Zero findings. The new final-producer helper places spawning, active receipt publication, waiting and post-spawn observations inside its cleanup boundary at final-producer-prepared/run.py:80-130. Timeout, receipt failure, wait exceptions and guard failure enter bounded owned-group TERM/KILL, group disappearance and leader reaping. Unexpected surviving descendants refuse qualification. Unproved cleanup retains doNotReleaseLease. Original exceptions and actual child exits remain distinct from wrapper qualification.
- ooxml: Zero findings. These changes introduce no product or XML mutation. Actual emitted font declarations are read and authenticated rather than rewritten. Native owner, coordinate and reference checks remain unchanged.
- tests: Zero findings. Word56/PPT48 exact membership, fresh paired wheels, installation provenance, typing/stubs and the mandatory exact ignored native SVG test retain their original stage identities. Pass38 conditional SVG/PNG handling, whole-group cleanup and consumed transition timestamp/inode checks remain intact. The final-producer command is the exact guarded rdocx all-features/all-targets check. It consumes the authenticated actual two-ODT terminal without relabelling it as a later compile result.
- structure: Zero findings. The correction is confined to external orchestration. No product module, dependency, additional target or public API is introduced.

## Admission and handback

Both admission templates retain authorization false. Latest-producer fields remain pending until ROOT executes and authenticates the separate final-producer stage. A successful future final check must produce its own actual argv, cwd, HEAD, index, source-map, before/after guard, log and cleanup records. It cannot be inferred from preparation, the earlier two tests or a truthy admission flag. Prior failed R1/R2 preparation and runtime evidence remains separate.

This pass executed only read-only metadata authentication and Python AST parsing. No helper, product import, compiler, Cargo, wheel build/install, painter, test, UI action, source write, cache change or timestamp change occurred. No full104, native SVG or full-feature acceptance is claimed. Only this review report was written. The review ends here and returns the canonical reviews writer lease.
