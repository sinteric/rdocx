# F-X195, native consumer and SVG review, pass 10

**Reviewed**: Frozen native source R1 in work/f-x195-codex at
2a9bf14072231e46fcb34df70f88cf7aa79fbd79. The native comparison against the
accepted canonical928d6c16 prerequisite covers12 source and dependency
carriers,1234 additions and75 deletions. It includes the three earlier SVG
FontData literal additions. Those additions and the earlier core/provider
implementation retain their prior independent reviews.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Identity and scope

Read CLAUDE, WORKFLOW, microscope, the complete approved195 design, cited
rendering/native API/dependency sections, progress, prior core/provider
reviews, native reconciliation proposal and the approved private SVG parser
edge amendment. CURRENT_SPRINT and BACKLOG both remain in-progress.

The actual native-source-r1-receipt SHA is
ccaa6b4e76f24cf6e7770ab75951b81eaf820e53093c66e32f889a6a2b79b9f7.
Its full27-carrier diff SHA is
d17a68d34bc0034409eff67e17d09dd03f7cb6a98e87e9038d3db883518b4476.
All27 bytes, modes and sizes match. All12 held Python hashes and the held
baseline match. The full diff includes the accepted eight-file194/212
prerequisite prefix and separately reviewed core/provider work. Those are
not presented as new native implementation or newly executed tests.

The new native author is independent of this reviewer. The earlier
PowerPoint test correction authored by this reviewer is excluded from this
pass and retains the independent provider-pass9 assessment.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Not found

Correctness and contract: normal Word additional-font layout and its owned
output bundle are used by the new PNG/SVG/selected/all-page methods at
crates/rdocx/src/document.rs:28614 and:28924. Revision options remain explicit.
Caller-only existing layout methods remain distinct from the added normal
base. Word CLI layout/convert/render load before output staging and consume a
caller-selected deterministic bundle at crates/rdocx-cli/src/commands.rs:411,
:707 and:2004. PowerPoint configuration enters the one prepared manager before
slide resolution at crates/rpptx/src/lib.rs:11280. Slide, notes, handout,
measurement and timeline paths share this preparation. Animation construction
passes the same configuration at crates/rpptx/src/animation.rs:212.
Normal PDF Result consumers use checked preparation before returning bytes,
including notes and handout routes. Existing archival checks stay separate.

Panics: collection extraction at crates/rdocx/src/svg.rs:760 checks selected
face lookup, counts, duplicate tags, table slices, checked sizes and relocation
conversions before allocation/copy. It removes invalidated signatures, zeros
head adjustment for checksums and recomputes the standalone checksum. The
selected-face parse is checked. Glyph bounds at:863 validate dimensions,
glyph/advance counts, selected glyph domain, variation coordinates and finite
geometry. New unwrapping is confined to existing test construction or writing
to infallible String sinks. Zero findings.

OOXML and source preservation: native render preparation uses staged packages
without authoring source. New Word and PowerPoint controls retain original
serialized bytes at crates/rdocx/src/document.rs:33977 and
crates/rpptx/src/lib.rs:12370. No new parser, serializer or schema child-order
mutation is introduced. SVG retains searchable text and ordered resource
selection. TTC face extraction retains selected unsigned tables. Synthetic
stroke width and baseline shear are explicit at crates/rdocx/src/svg.rs:316,
and browser synthesis is disabled for selected variation/synthetic runs.
Ink bounds include selected coordinates, offsets and stroke/shear while
preserving advances. Zero findings.

Tests: the added native bundle controls assert actual selected bytes/index,
output consumption, omission equivalence and unchanged source. PowerPoint
also compares configured metrics/timeline/animation against omission. The
TTC control at crates/rdocx/src/svg.rs:1802 checks face1 against distinct
source metrics, checksum, embedded bytes, searchable text and independently
loaded reference-face painting. The synthesis control at:1758 independently
computes the12-degree bound geometry. Both CLI controls preserve existing
output sentinels and input bytes on invalid directories/font files, then
require actual valid PNG output. These bounded source controls are meaningful
but unexecuted, and do not establish all entrypoint acceptance by themselves.
Zero findings in the reviewed native phase.

Structure and dependency policy: concrete configuration is passed through
existing native functions, with no new module, trait, generic or test binary.
The only native manifest edge is existing workspace ttf-parser at
crates/rdocx/Cargo.toml:41. Parsed Cargo.lock differs only by that entry in the
rdocx dependency list. All302 package identities, versions and checksums
remain unchanged. Python/support files and baseline are held. Zero findings.

## Runtime and completion limits

No Cargo, imports, external renderer, formatting or source correction was
performed by this pass. Native R1 is uncompiled and unexecuted. Earlier
layout9, PDF6, Word6, PPT5 and no-default9 controls retain their exact prior
qualified runtime records, including the original failures and bounded
historical reversals. This review does not repeat or widen that credit.

The complete plan still requires every CLI/native/Python entrypoint's actual
selection evidence, installed signatures, selected/revision controls,
independent variable/synthetic SVG output checks, full scoped riders,
measured publication carriers, declared hash attribution and listed HLD
updates before handoff. No Python, archive, baseline, feature-completion or
full-sprint verification claim is granted here. This ends the review pass.
