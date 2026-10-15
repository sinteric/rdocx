# F-X211c, column ancestor runtime failure, pass 1

**Reviewed**: Frozen f2 binding source, actual failed held-column companion, approved parent/c grid-ancestor contract and existing row/column controls.
**Verdict**: 1 product defect, 1 conflicting test defect, 0 smells, 0 nitpicks.

## Defects

### D1, collection ancestors incorrectly capture the retired grid scope

`/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/src/table.rs:344`

`Table.columns` constructs ColumnCollection with the surviving physical table
path but explicitly adds RevisionScope::Grid. `Table.rows` does the same at
`table.rs:363`. Neither collection names an indexed member. Its length resolves
the current table after validating that captured path at `table.rs:400` for
columns and `table.rs:614` for rows.

`ColumnCollection.add_column` stages a native grid edit at `table.rs:480`.
The common restructure route calls grid_changed_owners at `table.rs:80`,
which preserves the table owner while advancing unsafe grid descendants.
The collection's captured Grid state therefore retires with those descendants.
The actual operation succeeds but the held collection immediately fails its
next len read. RowCollection.add_row has the same defect at `table.rs:693`.

This violates the approved table-grid ancestor stability in
`.claude/plans/F-X211-design.md:69` and the c residual ownership contract at
`.claude/plans/F-X211c-design.md:27`. The frozen P-LIFE-05 native clauses require
row/column ancestor survival separately from unsafe descendants. The original
script's fresh table.rows lookup cannot substitute for that held-ancestor proof.

A normal remediation must distinguish these table-owned collection ancestors
from indexed row, column and cell views. Existing capture logic at
`crates/rpptx-py/src/presentation.rs:3104` already adds Grid to explicit Grid
captures, cells and Row-segment paths. Returned indexed members at
`table.rs:493` and `table.rs:706` must keep their prepared Grid authority.
The discovery of this collection defect does not justify preserving old
unsafe cells, reusing deleted indexed members or bypassing owner/foreign
validation. No new scope registry is required by this diagnosis.

### D2, an existing regression requires the contract-violating row collection retirement

`/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/tests/test_documented_examples.py:5550`

The existing grid-change test correctly requires the held cell to retire with
RowCollection.add_row at lines5548 to5549, then also requires held_rows length
to raise. That latter assertion encodes the collection-ancestor defect and
contradicts the now-approved parent/c acceptance. It needs normal correction
to assert actual held collection survival while retaining the precise unsafe
cell retirement, first cause and unchanged sibling/table observations.

The existing entrypoint should gain deciding held row and held column
collection controls covering their own and opposite-axis topology changes,
saved/reopened dimensions/content, unsafe member retirement, no-op/failure
atomicity and actual removed-table authority. The original allocated scripts
and companion operation must remain unchanged. Renaming or replacing the
failed add_column operation would not resolve the acceptance gap.

## Smells

None.

## Nitpicks

None.

## Actual evidence and limits

Original receipt
`/private/tmp/S91-F-X211c-final-typing-paired-producer-1/actual-column-topology-companion-1/receipt.json`
SHA2eaa9ef2473bcf6eba27bd22da215538a851a98c6fd3aa620562000ddd09c012
retains actual exit1, failed qualification and safe cleanup. The raw log
SHAc41e576f3f3cd35798d382d88f68f77f9a681b8dca192acf4079acee373938fb
fails at companion line40, len(columns), with first cause
ColumnCollection.add_column and document revision5 to6. Later cell-refusal,
save/reopen and saved dimensions assertions were not reached and receive no
success credit. The original first100 f2 wave remains unchanged.

This is distinct from the prior cache-fixture diagnosis. No parser/cache,
collection scope, test expectation, harness, source, runtime, target, CP, UI,
ledger, network or Git state was changed. Only this review file was written.
The review ends and releases the canonical review-write lease before remediation.
