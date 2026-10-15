# F-X195, Python and SVG, pass 17

**Reviewed**: R7-to-R8 remediation, four existing paths, 4814-byte delta.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Authority and identity

Read CLAUDE, WORKFLOW, microscope, the approved F-X195 plan, cited rendering,
binding, architecture and toolchain contracts, progress and pass16. Worker
work/f-x195-codex remains in-progress, HEAD
2a9bf14072231e46fcb34df70f88cf7aa79fbd79.

Authenticated receipt SHA256
c3618dc416811fe6acda87db2c7e6fceab3c674f84b5f74211ba4c53f16c4a23,
317 current source byte, size and mode guards, source manifest SHA256
27cf7d5c643ec03e24f25ae179a557ebdc5acb48f65d593a1bd078af934485c8,
and the predecessor manifest. Exactly four inputs changed and 313 remain
unchanged. Reconstructed the exact supplemental diff from retained before
copies and current files, SHA256
ba8efca4f4ae859a1b210166cb3efa8871ac0d7e2de1b8a07c039ac9e163453c.
The current complete git diff matches retained full-working.diff SHA256
45df3a370332e235dc6b369e131e2ab79ad805ad3e4576e05004e0cb470307e3.
SVG and native document sources remain exact R7.

## Defects

### D1, page-zero metadata cannot discriminate selected font metrics

`crates/rdocx-py/tests/test_rendering_threads.py:804`
`crates/rdocx-py/src/document.rs:363`
`crates/rdocx-py/src/document.rs:3550`

The new fallback control requires layout_page(0) with the actual normal face
to differ from ordinary layout_page(0). LayoutPage equality compares only
physical and displayed page numbers and page width and height. It contains
no fragments, glyphs or font identity. Both layouts of this unchanged,
single-section fixture have a first page numbered one with the same section
page dimensions. Changing Arial metrics can change fragments and pagination,
but cannot change these first-page fields. Consequently this assertion fails
even when empty-input fallback is correctly implemented, preventing the
subsequent empty-input and invalid-path controls from executing.

Keep a genuine unequal measurement guard using the existing fragment result.
For layout_page, use a genuinely font-sensitive page-existence boundary if
independently established by the fixture, or retain first-page empty-input
equality without claiming its metadata discriminates physical font metrics.
Do not change the public LayoutPage result merely to satisfy the test.

## Smells

0.

## Nitpicks

0.

## Aspects checked

Correctness and contract: the two collector changes resolve pass16 D1. None
is returned only after every supplied path has passed metadata validation and
strict detached directory loading, and only if the final loaded collection
is empty. Nonempty explicit fonts and ordered directory selection retain
existing behavior. Empty fonts cannot bypass an invalid supplied directory.
The thirteen existing omission branches remain unchanged.

Tests: D1 above. The normal SVG extraction uses the actual emitted selected
font bytes and the fragment inequality is a meaningful positive fixture guard,
with host discrimination still requiring actual runtime. The permission
controls resolve pass16 D2 without accepting arbitrary exceptions. Word
requires PermissionError or public LayoutError, and PPT requires
PermissionError or public RpptxError. Both require the exact denied path and
permission-denied text, restore owned permissions in finally, and retain
source-byte atomicity. Existing loader error mapping remains unchanged.

Panics, OOXML and structure: zero findings in the incremental scope. Both
collectors remain concrete operations in existing carriers, with no new
module, trait, generic or public API. Source mutation is absent. SVG
correctness, static variable oracles and synthetic paint controls were not
changed and retain their prior review qualification.

## Limits and handback

No Cargo, installed import, test execution, native UI or baseline update ran.
This is a source review, not runtime or feature completion credit. Original
pass16 and failed runtime evidence remain retained. Only this review file
was written. Review phase ends here.
