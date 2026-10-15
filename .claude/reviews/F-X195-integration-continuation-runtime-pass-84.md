# F-X195, integration continuation runtime, pass 84

**Reviewed**: independently authored R61 runtime renewal, its120-line runner delta against reviewed R59, complete current runner and prerequisite metadata. Incremental admission review against the approved F195 contract and pass83. No product execution occurred in this review.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, terminal receipt retains a stale latest compiled producer label

`/private/tmp/S91-F-X195-integration-continuation-r61-prepared/run.py:195`

The receipt initializes `latestCompiledProducer` to failed R58 and never changes it. This correctly describes the starting compiler prerequisite. However, line265 executes the new source-guarded PPT Cargo JSON build, line267 stores its actual executable metadata, and line276 can emit a completed R61 receipt still identifying R58 as the latest compiled producer. A subsequent cache handover consumer reading the explicit field receives a false final lineage label even though authentic new rows are retained. Rename the initial field to `startingLatestCompiledProducer`, or maintain separate truthful attempted and actual compilation evidence. Do not rewrite either failed receipt or infer compilation merely from a planned command.

## Smells

None.

## Nitpicks

None.

## Checked evidence and nonfindings

Receipt `5bfd926271eb5afbd3d7b74d0cebb1e8c80efd260726bee75330fa38776759ba`, runner `c81dfac41a30c1523926c2a19d67a2f3e8b76d51ac2b70107d67b885748eb6e9` and preparation `8678cac4e1c11a0efab76a4bcd8db461ed268bccc4d25dbae02416b0ce57234e` authenticated. All seven packet artifacts matched hashes and sizes. The exact runner delta reconstructed from R59. Current317 and2679 source bytes, sizes and modes matched their guarded maps, and retained Word binary metadata remained exact.

Pure metadata qualification independently reproduced the authentic R59 ODT child0 result with exact name, process-owned paths, raw log, four command rows, whole1 passed/0 failed/0 ignored/373 filtered summary and all four cleanup records. Eight mutations of name, owner process, summary, outcome, truncation, extra outcome, filtered count and warning were rejected. Source-built status conversion and temporary-path construction match integration_test.rs2284 and5092. The theme grammar is source-derived synthetic metadata only and earns no actual pass. R59 remains failed and its unqualified wrapper row is not relabelled.

The original R58 qualification still proves374 names,361 raw passing results,3 failed names,8 cited ignores and2 separately pinned tests. Removing only the first failed name from fresh retries corresponds to the independently qualified ODT result. Both remaining original Word bodies and both26.01 pinned bodies remain mandatory. Full PPT Cargo JSON artifact selection, binary listing and complete result membership remain mandatory. Original24 gate definitions and conditional missing-oracle qualifications remain intact.

Reviewed post-spawn owned-group cleanup is unchanged. Latest runtime R59 and starting compiled prerequisite R58 are distinguished in admission. Explicit ROOT review, lease, outside-sandbox approval and no-intervening-Cargo checks are required before execution. Future authorization is false and no completed continuation is asserted by this prepared packet.

## Aspect coverage

- **Correctness**: D1 above. No other findings in qualification or command membership.
- **Contract**: D1 above. Original failed history, pins and acceptance boundaries otherwise preserved.
- **Panics**: zero findings. Qualification fails closed and bounded cleanup remains mandatory after spawn.
- **OOXML**: zero findings. This runtime renewal introduces no XML or source mutation.
- **Tests**: zero additional findings. The raw positive and eight negative metadata checks passed independently. AST and metadata checks are not product runtime.
- **Structure**: zero findings. Existing lifecycle mechanisms are reused without a product API or module change.

Full integration, remaining riders, hash, verified publication, policy, HLD and completion remain pending actual evidence. This pass ends with the report and returns control for a distinct remediation phase.
