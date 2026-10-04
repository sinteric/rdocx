# F-X167, all, pass 2

**Reviewed**: working diff against 5e760049, 22 files, 747 added and 123 removed lines
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, JSON keeps a deleted-mark paragraph as a separate record
`crates/rdocx-cli/src/commands.rs:152`

`text --json` writes one record for each body paragraph without checking
`accepted_paragraph_joins_next`. A paragraph with a deleted mark and retained
text therefore remains a separate JSON paragraph, while `accept_all()` merges
its runs into the next paragraph. The plan requires the accepted JSON output
to agree after save and reopen. Join the records or project the joined
paragraph before serializing, and compare the normalized records with
`accept_all()` in a regression test.

## Smells

None.

## Nitpicks

None.

## Not found

No further correctness, contract, panic, OOXML ordering, test-gate or
structure finding in the reviewed diff. The previous control-owned row
finding is fixed. The changed-crate suite, Python bindings, hash harness,
policy gate and package dry run passed before this review.
