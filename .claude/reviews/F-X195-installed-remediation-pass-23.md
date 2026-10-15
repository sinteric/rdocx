# F-X195, installed remediation, pass 23

**Reviewed**: R13 to R14 narrow two-carrier delta, 10006 bytes. This is
incremental admission review, not the full working feature review.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authority and source

Read canonical CLAUDE, WORKFLOW and microscope, the approved caller-font
ordering and measurement contract, cited rendering and binding surfaces,
previous pass22 and current progress. Confirmed work/f-x195-codex at
2a9bf14072231e46fcb34df70f88cf7aa79fbd79 and in-progress CURRENT/BACKLOG rows.
Authenticated actual receipt, both complete source maps, all317 current
source bytes, sizes and modes, two changed carriers and315 unchanged inputs.
Read the complete narrow delta and both complete changed implementation regions
with their loading and measurement consumers. Original installed failure logs
and probe receipts were authenticated separately. No execution result is
inferred from syntax checks or source review.

Current317 map SHA256:
e9ef0b359a38a2a0d8940f4d91596d84817e013c00d45ab819f12499bfd2acc7.
Delta SHA256:
e4f2f664688f49fbcf662203d36df499ec88939a3c339cbbe7154893d6545a29.
Receipt SHA256:
85b537e06a4689b527e38a2463048ee75c99c56b4a1635f2dcde1cbf48b7d7ba.

## Assessment

The additional-font reload first clones the original constructor maps and
appends newly loaded IDs. At crates/oxml-layout/src/font.rs:885 it reverses
only the constructor prefix and moves it behind additional candidates. The
reversed query iteration at :2562 therefore visits original constructor
candidates first in their original order, followed by additional candidates
in reverse load order. Constructor-only active maps reverse at :934 while
the stored base maps retain insertion order. Every reload starts from those
base maps, so repeated replacements do not rotate an already rotated map.
The prefix length is bounded by the just-cloned base vector and append-only
loader at :956. No untrusted indexing assumption is introduced.

All candidates still participate in the unchanged fontdb stretch, style and
weight query. Provenance decides equal-ranked ties, rather than outranking a
genuinely better CSS face. Default and deterministic managers have empty
constructor maps. Document embedded and explicit caller inputs share the
additional tier, so later explicit inputs win there. Existing constructor
priority and alias controls remain intact. The new inline test at :3297
checks actual original bytes, both request aliases, cache identity for an
unchanged set, reversed input ordering, genuine bold over a later regular,
and genuine bold over regular constructor candidates. These controls would
reject the previous additional ordering and a blanket constructor reversal.
They remain unexecuted.

At crates/rdocx-py/tests/test_rendering_threads.py:616 the selected-page
control now uses unique reverse order, exact TIFF frame-to-PNG pixels and an
explicit duplicate LayoutError with unchanged document bytes. It preserves
existing native refusal instead of expanding product behavior to satisfy an
incorrect new fixture. The independently qualified installed probe confirms
that pre-existing behavior, not acceptance of the unbuilt R14 selector.

At :798 the ordinary fallback control uses a selected physical normal face
with a demonstrated18 versus20 page boundary. The test still requires one
real SVG font payload and a real pagination difference before comparing empty
input routes. Fragment physical_page is one-based, as emitted by
crates/rdocx-layout/src/paginator.rs:390 and :434. layout_page indexes the
zero-based pages vector at crates/rdocx-py/src/document.rs:3556. Therefore the
minimum final physical page is the first absent index for the shorter layout
and remains present in the longer one. The new boundary assertions are
consistent with these actual consumers. No existing paint or ordering
inequality is removed to hide a product defect.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Not found

Correctness, contract, panics, OOXML preservation, tests and structure produced
no findings in this narrow delta. No production file, module, dependency,
public API, font asset or baseline is added. The original installed19 results
remain3 passed and16 failed. Current native controls, both fresh wheels,
installed19, remaining riders, HLD and complete working review are still due.
This pass writes only this review artifact and ends here.
