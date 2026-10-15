# F-X195, SVG owner qualification, pass 33

**Reviewed**: External two-carrier test-only proposal, 21551-byte diff.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in this incremental proposal.

## Contract and evidence

Reviewed the approved variable defaults and SVG contract, mandatory installed
gate, current progress, pass32, complete proposal and retained raw captures.
F-X195 remains in progress on work/f-x195-codex. The source author is separate
from this review. Authenticated current two carrier hashes, every candidate
artifact and all four original variable/static raw SVG hashes.

The approved plan at .claude/plans/F-X195-design.md:124 preserves the physical
source default for nonbold. Selected nondefault coordinates must remain
explicit at:138. Actual default400 has no style while all selected700 owners
have explicit weight. Both actual weights have five positioned Thai owners,
which the old whole-string owner search at
crates/rdocx-py/tests/test_rendering_threads.py:867 cannot find.

The proposed qualifier replaces that unsupported assumption with all five
exact ordered texts, joined source text, physical family/source hashes and
per-owner selected coordinates. Its bounded fvar reader authenticates defaults
400 and100 from the exact embedded original source before admitting absent
style for normal400. Every700 owner still requires explicit weight700.
Static reference font bytes remain independently selected and authenticated.
No production coordinate policy changes. Complete geometry comparison retains
all positions, grouping, child order and nonrepresentation attributes.

The existing native gate at crates/rdocx/src/svg.rs:3441 retains actual104
success, paired wheel/source admission, eight distinct cases and threshold0.99.
The proposal independently reads every positioned owner and ttf-parser source
defaults before selecting the independently generated static resolver font.
Other style selectors and existing independent paint negatives remain exact.
Last-owner wrong-family, wrong-coordinate, missing-owner and missing explicit
700 selection controls prevent a first-owner or global-substring substitute.

Proposal SHA is
e624b1c264d0aa487c2f61f35ee874371f9e2d7cee9d1c34400e78ee9f2ad151.
Python candidate SHA is
1bb71eb69280fab5c47cfebb469eafe8495ffe9c496eec749f885e2b98d64a7f.
Native candidate SHA is
65b45be75daa9803cad8e9ecd8613cd2e4f0e2f7978c5b9090114c5b73506282.
Native production prefix and both product providers remain unchanged.

## Defects

0 defects.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Not found and limits

Correctness, contract, panics, OOXML, tests and structure produced no finding
in this finite fixture qualifier. This is not a general SVG or SFNT parser.
Original full104 failures remain retained. No runtime, compilation, paint,
source adoption, formatting, cache, UI or feature completion occurred here.
Actual104 then the explicit ignored native test remain mandatory acceptance.
Only this report was written. The review pass ends and returns control to ROOT.
