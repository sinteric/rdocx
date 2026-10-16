# F-X248, packaging, pass 2

**Reviewed**: Bounded numeric amendment only, four replacement lines across scripts/readme_doctests.py, README.md and crates/rdocx-cli/README.md. Before/after bytes authenticate against /private/tmp/S95-F-X248-numeric-reconciliation-2/receipt.json and its before copies. The six production/test/HLD files from correctness pass1 remain byte-identical to their frozen-source.json digests.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Independent archive observations

- rdocx: compressed 3,752,073 bytes, normalized member bytes 52,059,111, 37 members. Actual archive SHA256 is 812bf27120bc0410e7cb6fc540321e7a52fc2cdb0b5570931c3ca7fc7ac4b01b.
- rdocx-cli: compressed 108,827 bytes, normalized member bytes 503,176, 8 members. Actual archive SHA256 is 5742c9c08a79ddadb9a1341ac544105cd1898fdde204664b0694d390495b6008.

Both actual archives are below 10 MiB. Their resolved target paths and complete file hashes match the root receipt. The reviewer independently calculated both tuples with the unchanged archive_measurement function, executing only that pure function with explicit imports. The edited catalog tuples and corresponding README rows match those observations exactly.

## Not found

Correctness, contract, panics, OOXML, tests and structure produce no finding within this numeric amendment. The script differs from its before copy only in the two declared archive tuple literals. Both README files differ only in the corresponding numeric values. Measurement/validation function ASTs remain unchanged. Compression tolerance, assertion logic, measurement dates, performance rows and unrelated catalog records are unchanged. The earlier six-file implementation audit is not repeated or widened by this pass.

## Boundary

These are pre-regeneration archive observations. Editing packaged README content changes subsequent archive bytes, so this review does not claim that regenerated packages or affected policy/publication checks have passed. The worker must rerun the affected checks, verify regenerated measurements against the unchanged normalization and tolerance rules, and retain actual resulting evidence before completion.

No Cargo, build, test, install, cache, source or Git mutation was performed. Only this review file was written. Return the worktree lease to the orchestrator.
