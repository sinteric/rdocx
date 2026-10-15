# F-X209, installed correction, pass 18

**Reviewer**: fonts_resume, independent read-only review.
**Reviewed**: Frozen installed-correction-proposal receipt d777d28ec216187b6ee49773c40e12fb35d4f35f0adbd03667408958b822293d. Four existing carrier before/after pairs and the 6,526-byte incremental diff. Scope is master/layout placeholders, the nine reported stub mismatches, and the exact stale-exception fixture correction. The separate six native integration regressions are excluded.
**Verdict**: 0 defects, 0 smells, 0 nitpicks. The bounded proposal is admissible, with successor execution still due.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Aspect results

Correctness: 0. Contract: 0. Panics: 0. OOXML preservation: 0. Tests: 0. Structure: 0.

## Source qualification

The added SlideLayout and SlideMaster getters validate the existing owner handle and construct the same PlaceholderCollection with its existing ContentPath. Lookup now resolves ShapeOwner from the current path, enumerates that physical owner's immediate children using part_shape_at, and returns through the existing checked ShapeCollection.item capture. This extends the existing Slide path instead of introducing detached placeholder models, new identity counters, or a separate mutation route. The old expect asserting an immediate slide shape is removed.

The existing native show_master_shapes implementation is reflected in the stub with its optional boolean getter/setter. Layout duplicate/remove parameter names are aligned to runtime layout. TextStyleList and ThemeColors special-method parameters are made positional-only, matching the retained stubtest diagnostics. Master/layout placeholder properties already existed in the public stub and are supplied at runtime by this proposal.

The stale fixture now expects StaleElementError, the exact exception in the preserved failed log, while retaining the operation-name match and surviving-owner assertions. The additive placeholder test checks physical shape ID correspondence, write visibility through owner.shapes, missing-index refusal, save/reopen persistence and repeated bytes. Its execution is pending. These changes neither loosen ownership invalidation nor alter package serialization directly.

## Evidence and limits

All nine local artifact records and eleven retained external reference records were authenticated by exact size and SHA256. Before carriers are bound to source-production-65. The original installed attempt remains 20 passed and 1 failed. The failed assertion expected ReferenceError but received the documented StaleElementError naming SlideLayoutCollection.remove. Its strict mypy row exited 0. The original nine stubtest diagnostics are retained and each has a corresponding proposal change. Before/after/current origin records and the complete installed receipt are authenticated, not replaced with a fresh execution claim.

This proposal is unadopted and unrun. No native, installed placeholder, fresh-wheel, strict typing successor, Office paint, whole-feature acceptance or six-regression correction credit is granted here. Earlier whole and incremental reviews remain independent. No source, runtime or canonical record was changed. This review ends at this report.
