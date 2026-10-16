# F-X251, packaging, pass 2

**Reviewed**: Bounded amendment only, exactly four changed numeric lines in scripts/readme_doctests.py, README.md and crates/rdocx-cli/README.md. Original correctness pass 1 is not repeated.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Authentication and boundary

Authenticated numeric reconciliation receipt SHA256 ab636603aa9e67dd4f4403169eec29bdb02466857dd4d0ea7e6f89cb80c6dd7d, its before copies and patch. Root's independent all22 inventory receipt authenticates as a3230891dae0dda8a8c87aa9b24f58c6f030ef8424933d362891db7bb5973b85. Actual retained archives were independently read and measured using the unchanged pure repository archive_measurement helper isolated from module execution:

- /private/tmp/S95-F-X251-worker-1/inventory-archives/rdocx-0.16.1.crate, SHA256 372ec8e33e4c7c7b39f95dbe29e621abda7cbe56d568434cfe2cbb284d8e2dee, tuple (3754589, 52071185, 37).
- /private/tmp/S95-F-X251-worker-1/inventory-archives/rdocx-cli-0.16.1.crate, SHA256 4c06f983e57e14982cdc9b7b3b2c11bf64f4baa90d1dd8c721a491f83cf763e6, tuple (109949, 510634, 8).

Both actual archive paths resolve to the stated retained paths and both compressed sizes are strictly below 10 MiB. Their exact observations match the two catalog tuples and corresponding README rows. Full-file reconstruction from authenticated before copies permits only those four numeric replacements. All other bytes remain unchanged, including dates, performance observations, compression tolerance, normalizer, validator and assertions. Function ASTs are unchanged.

All eleven original implementation, test and HLD hashes still equal frozen-source.json, SHA256 fdc82c538343e0c4723a7947de1554ae9593b3e6eff819dc9ca2a247c3bc46a1, before and after the audit. No correctness confirmation pass or product remediation was performed.

These observations are pre-regeneration measurements. The affected full 183-test policy run and genuine two-package publication rerun remain pending. Earlier archive authentication and this review do not establish that those later deciding checks passed. No Cargo, build, product test, cache, UI, source or private proof mutation occurred. Only this report was written. Return the sole worktree lease to the orchestrator.
