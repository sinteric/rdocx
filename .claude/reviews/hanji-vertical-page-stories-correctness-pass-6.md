# Hanji, vertical page stories, final continuation review, pass 6

**Reviewed**: renderer d1966e9915e3bb6bec0b4fa5e772f9680ca5bf72 against 873df08 and downstream Hanji 84b20899, focused continuation and pin review
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, split cells retain percentage spacing from the previous measure

`crates/rdocx-layout/src/paginator.rs:1109`

The fragment refit selects the template cell width but retains the pending
cell's left, right and remaining bottom margins. The table layout resolves
percentage cell spacing against the table width at
`crates/rdocx-layout/src/table.rs:599` and distributes half of it into every
cell margin. Therefore those margins are measure-dependent too.

The owned API reproduction uses a 100 percent table, 10 percent cell spacing,
zero authored cell margins and a long plain cell crossing a 120 pt first
header into a 12 pt default header. Page one correctly places body text
28.2 pt inside its 564 pt table measure. Page two restores the 648 pt table
measure but still uses 28.2 pt rather than the required 32.4 pt half-gap.
Every later page keeps that 4.2 pt error. Source-owned DOCX, rendered SVGs and
the independently derived spacing contract are saved under
`../hanji-vertical-docx-design-evidence/final-review/`.

The refit must copy selected-template side margins and the remaining bottom
margin for nonempty cells. The continuation's removed top margin and completed
cells' removed bottom margin must stay removed. A renderer regression must
derive spacing from the active story width while checking logical text and
source conservation plus cold/warm equality.

## Smells

None.

## Nitpicks

None.

## Not found

Logical Unicode, bidi and atomic-item cursor accounting remains consistent.
Paragraph continuations remove consumed markers and first-fragment anchors.
The selected story geometry uses the same reservation and transposition as
the finite prepared measures. Existing complex-row guards remain in place.
Numbering advances only in the primary table layout, with alternatives cloned
from preceding state. Source semantics identify AST blocks rather than lines.
Cache keys include width, revision view, provenance and grid, with unchanged
retained budgets. All five direct Hanji renderer pins and 16 lockfile packages
select one immutable git revision. The existing registry svg2pdf patch is
unchanged and no renderer path patch is present.

The three logical cursor and eight vertical layout tests passed at the reviewed
head. No full local suite was repeated. Native Word fidelity remains unverified.
