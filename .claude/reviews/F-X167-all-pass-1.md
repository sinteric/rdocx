# F-X167, all, pass 1

**Reviewed**: working diff against 5e760049, 22 files, 672 added and 115 removed lines
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, JSON omits rows and cells owned by content controls
`crates/rdocx-cli/src/commands.rs:541`

`collect_table_paragraphs` walks only direct `table.rows`, and
`collect_row_paragraphs` at `crates/rdocx-cli/src/commands.rs:557`
walks only direct cells. A retained row or cell inside a content control
therefore appears in accepted text and HTML but is absent from `rdocx text
--json`. The plan requires the accepted JSON projection to agree after
save and reopen, including content controls. Interleave the control-owned
rows and cells with their direct siblings, then test a surviving and a
deleted control-owned row.

## Smells

None.

## Nitpicks

None.

## Not found

No additional correctness, contract, panic, OOXML ordering, test-gate or
structure finding in the reviewed diff. The paragraph and row visibility
choices follow the approved plan, and the current changed-crate tests,
Python binding suite, hash harness and workflow policy gate pass.
