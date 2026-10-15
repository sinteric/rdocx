# F-X221 integrated gate remediation, pass 2

**Reviewed**: Frozen working diff on canonical HEAD `c48624f58122d4c22026434acc1496accb061342`, 13 modified carriers, 534 insertions and 139 deletions. This pass reviews the distinct relationship remediation, prose correction, experimental-storage reversion and the unchanged behavior/test repairs inspected in pass 1. Only this report was written by the reviewer.

**Verdict**: Zero defects, zero smells, zero nitpicks.

## Contract correction

The actual approved contract at `.claude/plans/F-X216-design.md:59` and its correction at line140 treats the default 2 MiB constructor overflow as diagnostic evidence, not a new acceptance requirement. `docs/hld/15-build-and-toolchain.md:952` pins `RUST_MIN_STACK=8388608` for the broad corpus-heavy Test and MSRV jobs. My previous summary incorrectly treated default-stack qualification as a final broad requirement. That statement is superseded here. The current diff removes all experimental parser and private-storage changes. `crates/rpptx/src/lib.rs` retains only physical fallback-child reads and the proved notes-master registration guard. No new default-stack acceptance or product-runtime setting is introduced.

## Prior finding disposition

D1 is resolved by the current frozen source. `crates/oxml-opc/src/relationship.rs:324` admits map keys belonging to either retained raw records or current items. At line333, a historical owner absent from current IDs and lacking an explicit retained-ID mapping causes refusal when the proposal would emit an unmatched new row. This preserves the explicit retained-ID ownership route and rejects the destructive current-ID rename example before a result is published. It does not guess ownership from position or target. Legitimate removals remain supported, and a separately staged removal establishes an unambiguous source for later authored rows. The existing-module tests beginning at lines1464,1501 and1562 cover appended rows, renamed retained opaque owners, input immutability, explicit ownership, ambiguous additions and staged removal. The deciding predecessor failure remains retained. No new test execution was performed by this reviewer, and the concurrent broader OPC run is not credited by this static verdict.

D2 is resolved at `.claude/reviews/F-X221-wasm-manifest-oracle-pass-1.md:8` by replacing the prose semicolon with a sentence boundary without changing the retained failed-run qualification.

## Remaining aspects

Zero additional correctness or panic findings. Zero additional OOXML namespace, schema-order, opaque-preservation or atomicity findings. Zero additional lifecycle, public API, test-strength or structure findings. The unchanged chart namespace, checked paragraph-source route, promoted physical-owner capture, fallback reads, notes-registration guard and old-oracle repairs retain the pass 1 review coverage. The updated relationship logic is scoped to existing concrete records and introduces no framework or public API.

No fresh wheel, installed full suite, full integrated gate or native appearance acceptance is claimed. Original failed runs and diagnostic predecessors remain preserved. This review ends here and releases the review-file write lease to the orchestrator.
