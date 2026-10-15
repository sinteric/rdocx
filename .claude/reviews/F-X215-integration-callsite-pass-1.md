# F-X215, integrated commit callsite, pass 1

**Reviewed**: One-line working production diff against canonical HEAD `6581e3a51dff3f1ba6edc176fb42c7d4cab0dba7`, adapting chart data replacement to the integrated provider's explicit optional XML-owner argument. This is a bounded integration remediation review, not whole-feature or sprint acceptance.

**Verdict**: Zero defects, zero smells, zero nitpicks in the bounded callsite change.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Provider and transaction authority

`crates/rpptx/src/lib.rs:10247` changes `staged.commit_owned_parts()` to `staged.commit_owned_parts(None)`. The provider at line5680 accepts `Option<ShapeOwner>`. Its Some branch is specifically for checked XML edits whose retained root attributes can change without rendering-model inequality. That route explicitly serializes and forces only the selected actual owner part. Checked XML replacement supplies Some at line5246. Ordinary modeled transactions elsewhere supply None.

Chart replacement at line10126 stages chart and workbook candidates, returns immediately for equivalent source bytes, and clones the presentation before mutation. It isolates shared chart/workbook parts, rewrites the selected frame's modeled relationship when multiple frames share an ID, and otherwise retargets the selected relationship. Chart/workbook candidate parts and relationship graphs are written into the staged package before this commit. This operation does not install a raw XML owner whose root-only changes need the provider's forced-owner branch.

`staged_package` at `crates/rpptx/src/lib.rs:3438` clones the current staged package, preserving those chart/workbook and relationship edits. Slide comparison at line3604 serializes the modeled slide when its selected graphic frame changes. An unchanged modeled slide retains its original source. None therefore preserves ordinary chart copy-on-write behavior and avoids needless forced slide serialization. The provider still stages, checks signature invalidation, writes and reopens the package before replacing its staged state at line5705. The live presentation is published only after the fallible commit succeeds at line10248. The optional argument does not bypass validation, reopen, failure rollback or source preservation.

## Actual evidence and boundary

The working source diff is exactly this one production line in the existing carrier. The original finite integrated precheck receipt `/private/tmp/S91-combined-prefix-precheck-1/actual-checks-1/receipt.json` and raw log were inspected. Raw log SHA `1cc303fbc96a49a507f7dbed6aa992a9e0f497efb64df7d213f7f10509a3e876` authenticates. The precheck genuinely failed with exit101 because the old chart call lacked the now-required optional argument. Its group was naturally gone and leader reaped. That failure remains retained.

The root-owned successor under `/private/tmp/S91-combined-prefix-precheck-2` is running. This static review assigns no successor runtime, full scope, whole feature or integrated union success before its actual terminal receipt. No runtime, Cargo, target, CP, UI, source, index, Git reference, ledger or baseline operation was performed by this audit. Root granted the writer lease for this review file alone. Review ends and returns that lease.
