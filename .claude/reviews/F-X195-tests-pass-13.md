# F-X195, native Word acceptance tests, pass 13

**Reviewed**: The frozen R3-to-R4 test-only delta on work/f-x195-codex,
HEAD2a9bf14072231e46fcb34df70f88cf7aa79fbd79. One existing function in
crates/rdocx/src/document.rs,90 additions and22 deletions,7198-byte patch.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Contract and identity

Read CLAUDE, WORKFLOW, microscope, the approved195 plan, cited rendering,
native API and deterministic font sections, progress and reviews10/11/12.
The feature remains in-progress in both sprint records. The native author
is independent of this reviewer. This supplemental pass audits only the
new revision fixture and surrounding unchanged acceptance controls.

Authenticated R4 receipt
2541818ef06edefb01e62dd1975c347cc1123a923585090c2329e0190122559a,
current document hash
8fb456d5e07c5e54e51afdcea70b49f434a63d43a96279581a474c1d148deabe,
and the complete before/current reconstructed patch
521184bb6daffadfe5131446b831bb69ab12904ee5c20995d424a786679f7dc0.
All317 current input sizes, hashes and modes match. Exactly one differs
from the retained R3 inventory, the document test carrier. All other316
are exact. The entire file prefix before this test and suffix after it
are byte-identical to R3, including production and all other tests.
Prior review12 hash3a2f251c4634be22924270809ee44510854b9addf4e6e8365eafc318f9afbff3
is unchanged. No dependency, Python, baseline or product change is introduced.

## Assessment

The fixture at crates/rdocx/src/document.rs:33984 authors separate insertion
and deletion revisions with explicit caller-only family on every controlled
run. Parsing must yield exactly two typed revisions and their actual kinds
at:33988 to:33998. Accepted plain text is checked before layout. Serialized
source must retain both revision elements and markers at:34007. These are
real fixture preconditions, not an assumption that a raw unparsed fragment
will later exercise revision selection.

The accepted and tracked options are separately passed to normal caller-font
layout at:34014 to:34022. Page count and stored revision mode remain checked.
Every positioned text run on selected page1 must resolve its actual FontId
to exact supplied bytes, face0 and caller family at:34031 to:34043. Exact
concatenated shaped text at:34046 distinguishes accepted insertion-only text
from tracked insertion-plus-deletion text. The caller-only fixture helper
at:33955 also excludes matching bundled family or bytes. The existing
projection policy at crates/rdocx-layout/src/engine.rs:2254 includes both
revision kinds only for tracked view, so these authored expectations are
source-backed and do not derive from the renderer being tested.

Three selected raster formats retain exact facade/backend wiring comparison
and out-of-range rejection at document.rs:34068 to:34078. PNG and SVG page
variants retain their selected-bundle comparisons. SVG explicitly contains
the insertion marker and contains deletion only in tracked view at:34089.
Final accepted-versus-tracked comparisons require different PDF, PNG, SVG
and each of the three selected raster outputs at:34108 to:34128. Authored
text is an independent visibility expectation. The backend equality checks
prove caller-bundle and option routing, not independent glyph geometry.
The separately reviewed static and analytic SVG controls retain that role.
No comparison threshold or physical-font assertion was weakened.

Each view preserves both internal serialized OOXML and package bytes at
:34104 to:34105. This checks that rendering does not accept or delete source
revisions or mutate caller state. Native rendering here does not expose a
live Python handle, so no new binding lifetime claim is inferred. The
unchanged PDF/A profile forwarding and repeated normal PDF output checks
at:34136 to:34155 remain. Their byte comparisons prove routing and repeat
behavior, not independent archival conformance or complete revision parsing.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Other aspects

Correctness and contract: zero additional findings. Typed revision and
visible-content preconditions now discriminate the previously empty revision
option control. Panics: new unwraps and indexes are controlled test
preconditions, with two-page and two-revision assertions before access.
OOXML: no production parser or serializer change, explicit namespace fixture
and unchanged source checks. Structure: one existing test function, no new
module, dependency, API, trait or binary. Tests: exact caller identity and
all prior routing, range, repeat and source controls remain, with additional
independent content expectations. Zero findings in all six aspects.

## Runtime qualification

No Cargo, renderer, import, formatting or runtime test ran in this review.
The R4 test is uncompiled and unexecuted. Earlier accepted provider evidence
retains its original source identities and does not become R4 acceptance.
Actual native/CLI/SVG checks, remaining Python wiring, complete scoped risk
riders, HLD, archive and reviewed hash attribution remain required before
completion. This pass writes only this report and ends here.
