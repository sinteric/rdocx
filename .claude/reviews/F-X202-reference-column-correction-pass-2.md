# F-X202, reference column correction, pass 2

**Reviewed**: Bounded topology pass1 D1 correction. Four complete before/after context carriers, 3284-byte isolated correction diff and one selected reference-column control. Independent paired implementation and descendant-cell controls are excluded.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Evidence

Authenticated all 19 capsule manifest artifacts. Capsule receipt SHA256 is `0b20c1aa87c077f536b6fd947ed650c3d152f22a2be6f20b9a656e7d31f44509`. The three production after carriers exactly match actual batch16 source map `bb4767f715a6e9a8714d0655d0575fdc0668b12216a52af9635a5357ddfbdd8d`.

The retained `paired-descendant-fixture-before.rs` hashes to `39a4cee13b56b390a765febf868602495db98ad9ced9ac807df7bc106872394a`, exactly matching runtime16 regression source. The selected reference control occurs once, byte for byte. The later regression carrier differs by the disclosed descendant fixture string delimiter and xml:space correction, outside this review.

Actual reference selector log SHA256 `84722eceab7e900ccae42eda9444a88ceb47fa082a350115f9aa9925d78acb7c` records one pass, zero failures and zero ignored. Check exits zero. The combined batch16 remains failed because its independent descendant fixture failed. No installed binding, native Office, whole-feature review or full gate credit is awarded.

## Defects

None. D1 is resolved: `crates/rdocx/src/table.rs:973` keeps the selected reference separately from its insertion boundary. Width and new-cell properties come from that reference. An omitted reference extends the correct omission rather than creating a physical cell. The main wrapper at `crates/rdocx/src/document.rs:21786` and existing binding call preserve the same operand.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness, contract, panics, OOXML, tests and structure produced no finding within this correction. Existing topology validation bounds cell ranges before indexing. Mutations occur on a candidate, validate topology and reopen before publication. The selected control covers heterogeneous widths and formatting, before/after insertion, both omissions, invalid-reference byte preservation and reopen. No new trait, generic, module or dependency appears in this bounded correction.

This pass ends here and returns to sprint orchestration. Remaining F-X202 acceptance is still due.
