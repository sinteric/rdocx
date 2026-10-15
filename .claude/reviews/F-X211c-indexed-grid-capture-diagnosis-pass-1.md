# F-X211c indexed grid capture diagnosis, pass1

## Audited state

Worker `work/f-x211c-codex` remains frozen. The source map at
`/private/tmp/S91-F-X211c-collection-owner-remediation-1/source.json` has
SHA-256 `6079cc5f80f5df5d4e7987ff8298fa50f4a2c2aa276af3e2b6db64fba8ede489`.
All3366 entries match actual worker bytes, sizes and modes.
The actual installed20-control log at
`/private/tmp/S91-F-X211c-f3-paired-producer-1/actual-installed-1/actual-current-PPT20-controls.log`
has SHA-256 `4d6d8b61321f82b79817f4628ff03dbb920171708e48687cd02455b6d6d29a76`.
It records18 passes and2 failures. Neither failed installer nor the unexecuted
later assertions receives success credit. No runtime probe or test rerun occurred.

## Finding

**D1, indexed columns lack grid authority.**
`/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/src/table.rs:421`
captures an indexed column with `None` after adding only `PathSeg::Cell(index)`.
The shared capture implementation attaches the table Grid state only for an
explicit Grid request, a real Row+Cell path, or a Row segment at
`/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/src/presentation.rs:3104`.
A lone column Cell segment satisfies none of those predicates. It therefore
captures the surviving table owner without the retiring grid state.

Topology publication correctly replaces Grid at
`/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/src/presentation.rs:3278`
and marks the surviving table grid changed at
`/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/src/presentation.rs:3909`.
The missing capture permits old Column.width reads and writes, whose existing
validation is at
`/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/src/table.rs:583` and
`/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/src/table.rs:595`, to address a
current numeric slot after topology changed. Column removal also validates
this insufficient capture before selecting its current index at
`/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/src/table.rs:517`.

Integer indexing, slicing and iteration all share this faulty item route at
`/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/src/table.rs:442`,
`/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/src/table.rs:451` and
`/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/src/table.rs:569`.
The distinct returned add_column member already captures prepared Grid at
`/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/src/table.rs:493`.
Rows automatically acquire Grid through their Row segment at
`/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/src/table.rs:634`.
Table.cell and Row.cells use genuine Row+Cell paths. No second capture defect
is established in those routes.

The actual two failed axis controls stop in the old_row, old_column, old_cell
loop at
`/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/tests/test_documented_examples.py:10431`.
The old_row refusal succeeds before the old_column setter fails to raise.
This is a product authority gap, not a faulty test expectation. The explicit
indexed retirement contract is documented at
`/private/tmp/rdocx-f-x211c-codex/crates/rpptx-py/src/table.rs:54`, and the approved
ancestor/grid acceptance is at
`/private/tmp/rdocx-f-x211c-codex/.claude/plans/F-X211c-design.md:27`.

The smallest normal remediation is an explicit Grid request in the ordinary
Column item capture, retaining ancestor-only collection captures and existing
prepared add routes. Meaningful qualification must keep both axis variants,
first-cause errors and atomic bytes, and exercise integer, slice and iterator
members plus returned additions after a later topology edit. The first failed
loop leaves remove, roundtrip and parent-removal gates unreached, so they remain
pending until an actual successor completes them.

## Other categories and disposition

Zero additional findings in row/cell capture, ancestor collection survival,
prepared returned-member routes, no-op/refusal publication, test contract or
source preservation. These zero findings do not substitute for the failed
installed runtime gate. No source, test, runtime, target or other review file
was modified. This review ends here and releases the review-file write lease.
