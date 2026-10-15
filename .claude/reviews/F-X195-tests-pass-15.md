# F-X195, native handout fixture, pass 15

**Reviewed**: Frozen R5-to-R6 test-only delta on work/f-x195-codex,
HEAD2a9bf14072231e46fcb34df70f88cf7aa79fbd79. One existing function in
crates/rpptx/src/lib.rs, 92 additions and no removals, 5843-byte patch
c696481d543b0f23dab2d9ea7dbc43f25449896a6a5e8d4ddaa3efcccd244423.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Contract and identity

Reviewed the approved F-X195 plan, native acceptance requirements, guidance,
microscope contract, progress and prior native/test passes. F-X195 remains
in-progress. The fixture author is independent of this reviewer. This pass
covers only the genuine handout-master precondition and its paint assertion.

Authenticated R6 receipt
a0026e6cc7c760e7fef6c097eb83db8040a3ca20a352b6ceeeb4710e7950c689,
R5 manifest and the complete retained lib.rs before copy. All317 current
input hashes, sizes and modes match. Exactly one differs from R5. All other316
inputs and190 other Rust inputs are unchanged. Reconstructed the entire
5843-byte patch from actual before/current files and matched it exactly.
The complete prefix and suffix outside this single existing test are
byte-identical, including production and all other tests. No dependency,
font input, baseline or rendering code changed.

## Assessment

The default deck supplies no handout master, so exercising a required-master
consumer without authoring one was an invalid fixture precondition. The
repair at crates/rpptx/src/lib.rs:12402 stages the existing package and resolves
an actual notes-master theme relationship with exact theme content type.
It does not suppress the missing-relationship error or reinterpret a slide
PDF as a handout.

The authored part at lib.rs:12412 contains typed handoutMaster/cSld/spTree,
valid nonvisual group properties, distinct cNvPr IDs1 and2, complete shape
transform/geometry, txBody and an explicit caller-family run. Required
clrMap follows cSld. The family interpolation is a fixed controlled string,
not untrusted input. CT_HandoutMaster parsing is exercised before insertion.
The content-type override and internal master-to-theme and
presentation-to-master relationships at:12418 to:12426 form the graph the
actual handout renderer requires at lib.rs:10407 to:10423.

The presentation handoutMasterIdLst is inserted immediately before the sole
sldIdLst at:12436, after prior notes-master ownership and before slide IDs.
This matches the existing presentation schema boundary at
crates/rpptx-oxml/src/presentation.rs:2159. No existing presentation children
are removed. The exact handout related-part resolver then checks the newly
authored content type and target. Package serialization and native reopen
at lib.rs:12455 assert the actual resolved master part and typed owner,
using the real relationship resolver at lib.rs:7029. This is a saved/reopened
fixture, not an in-memory synthetic paint object.

The marker FX195HANDOUT identifies text contributed by the handout master,
separate from slide thumbnail text. At lib.rs:12523 the walker accepts actual
Text or MultilingualText output, requires nonempty glyph IDs, resolves its
FontId in the handout layout's font inventory and asserts the exact supplied
font bytes and face0. At:12549 at least one such run must contribute. Merely
retaining a caller font in an otherwise unused inventory cannot pass.
The controlled Liberation Mono bytes cover the authored ASCII marker. These
assertions prove actual caller selection and routing, not independent native
PowerPoint geometry or a font-family-label-only oracle.

All prior slide, notes, handout PDF/PNG, PDF/A routing, fit layout, timeline,
animation, repeated source-byte and empty-font fallback assertions remain.
The added graph is captured before the unchanged final source-preservation
snapshot. Backend equality comparisons retain their wiring role. Separately
reviewed physical-font and independent raster controls remain intact and are
not replaced by this fixture repair.

## Defects

0 defects.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Not found

Correctness, contract, panics, OOXML, tests and structure were checked, with
no findings in all six aspects. Unwraps are bounded controlled fixture
preconditions and expose malformed setup rather than accepting it. No public
API, module, binary, generic, trait or production behavior is added.

## Verification qualification

The original R5 failure remains immutable: four checks and prior layout/Word
tests completed0, the PPT child returned101 on the missing-master fixture
precondition, and six later commands were unrun. This source review executes
no Cargo, runtime, native UI or formatting and supplies no fresh test pass.
The repaired handout fixture and remaining acceptance queue must still run.
The review ends with this file and returns the writer lease.
