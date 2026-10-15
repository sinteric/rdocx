# F-X216, public batch normalization, pass 1

**Reviewed**: Frozen capsule `public-batch-normalization-frozen-1/manifest.json`, SHA-256 `4c4559d3958da0e367a61724a37bd8ed5b875fcaebbf01e0b08c0aa9cf74c886`. Exact predecessor-to-current production delta is three added lines in `crates/rpptx/src/lib.rs`, plus the defining existing-entrypoint regression in `crates/rpptx/tests/integration.rs`. This is a bounded normalization review, not whole-feature acceptance.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: zero findings. `crates/rpptx/src/lib.rs:9310` clones the source main relationship scope. At9316 the existing preservation operation stages every actual current record with an empty ID map before the later selected mapping. Authored IDs consequently exist in the retained physical scope when selected keys are validated. Selection and target matching at9331 remain unchanged.
- Contract: zero findings. The normalization is restricted to the facade import reconciliation. `crates/rpptx/src/lib.rs:8108` invokes it on the staged destination before `commit_candidate` at8115. Neither original presentation nor the ordinary OPC serializer is mutated. Single-import defaults and graph-selection rules receive no changes.
- Panics: zero findings. The new call propagates errors. Existing `crates/oxml-opc/src/relationship.rs:236` validates proposed IDs and record values before edits. Explicit mapping keys must refer to physical records at333. No new unchecked access or arithmetic is introduced.
- OOXML: zero findings. `crates/oxml-opc/src/relationship.rs:254` uses retained source XML when present. Owned attribute spans are edited at296, added authored records use the actual root prefix at336, and existing opaque bytes stay outside those edits. Unchanged records retain their order and metadata. Reparsing and exact proposed-item comparison at375 refuse incoherent projections. The empty normalization map neither renames retained IDs nor merges foreign scopes.
- Tests: zero findings in this bounded scope. `crates/rpptx/tests/integration.rs:34787` builds an actual fresh three-slide source and imports indexes2,0 before an existing slide. Assertions at34812 cover returned indexes, empty diagnostics, untouched destination slide, saved/reopened validity, requested text order and byte-identical source. Genuine RED2 executes that named test and fails `Package(InvalidRelationship)`. The successor executes the identical selector and passes1/1. RED1 is an API fixture compile failure and earns no behavioral credit.
- Structure: zero findings. The three added lines reuse the existing concrete relationship operation without new types, modules, dependencies or forwarding helpers.

## Authentication and qualification limits

The predecessor SHA-256 `02ba10adfc7aaac39951b3873c38fda4da624645d9ca5cea11168bd2c4c1b1b4`, frozen/live library `2229b55af11da335976d4f997d16e8f37611d5b79bad95909b9cb82371407d8e`, test `70accf370f08fad085a1556fd8d13c88d873777021d68d159bdb387911ac75e2`, source map `f87701d73c47b04a809f00e77c258a7a2ac4bde156a71fbcfdb4fe2c5c2fab38`, all three receipt files and their raw log hashes authenticate exactly. Library, test and reused relationship provider match the source-map body, size and mode before and after audit. The implementing writer confirmed the freeze.

RED2 receipt `eca496dab2765d5c075fffb3360855129547fc722dc77f2a5660ff3fc4d77e40` preserves actual exit101 and named0/1. Successor receipt `6b0d7f1f0f2af698333901b5e2e582f0868047635a4fca7c59953c250121d401` preserves exit0, named1/0, no timeout and safe natural process cleanup. Its stack is the documented8MiB corpus-test environment. Existing opaque relationship preservation is assessed from the unchanged provider and reconciliation logic, not represented as a newly executed opaque runtime control.

The failed installed seven-case receipt `f2f9f485216d6ab77019f00c5aecafba496da35dac905dd2bc456d7583cbda5b` remains historical failure evidence. No fresh wheel, installed successor, typing, native Office, broad regression, package, full sprint or release qualification is credited here. Subsequent notes-master registration remediation is outside this frozen delta. This review ran no tests and changed only this review file.
