# F-X232, all aspects, pass 1

**Reviewed**: working diff against d4ef72c894b2b0b4926696a90e42f07c3ee4cc63,24files,1008inserted and100removed lines. Approved plan, actual revision-view and checked-owner contracts, complete source diff, tests, native producer receipts and current paint measurements read before review. Source stayed frozen.
**Verdict**: 2 defects, 1 smell, 0 nitpicks

## Defects

### D1, Scale regression counts decoration segments as complete strokes

`crates/rdocx/tests/regression_test.rs:403972`
`crates/rdocx/tests/regression_test.rs:403985`

The fixture contains four positioned text segments per size, but its collector
retains every horizontal line primitive and asserts six total. The actual
product produces24segment primitives, so the test fails before its native
geometry assertions or artifact exports. Aggregate identical y/width stroke
bands or use one positioned word per size, while preserving all three measured
sizes and the0.36point independent geometry bound. Do not change product geometry
or loosen the bound to satisfy the primitive-count assumption.

### D2, Required related-story and actual caller-font cache controls are absent

`.claude/plans/F-X232-design.md:92`
`crates/rdocx/tests/regression_test.rs:403897`
`crates/rdocx/tests/regression_test.rs:403915`
`crates/rdocx-layout/src/notes.rs:280`

The declared every-cache regression builds only body and table content and
always passes an empty caller-font list. It does not exercise revised headers,
footers, footnotes or endnotes, actual caller-provided fonts, or the new colored
note-range path. Existing related-story tests using only default plain style do
not verify this new state. Add the approved plain/legal alternation and
post-mutation comparison against fresh layout over related stories and real
caller-font inputs. Include a split note or equivalent fragment control so its
color remains attached to the correct visible revision range. No cache failure
is observed in the inspected source, but the named acceptance matrix is missing.

## Smells

### S1, Public note-range source break is absent from the compatibility inventory

`crates/rdocx-layout/src/notes.rs:56`
`crates/rdocx-layout/src/lib.rs:10`
`.claude/plans/F-X232-design.md:36`

NoteLayout is public through the public notes module. Its revision_ranges field
changes from Vec<Range<usize>> to Vec<(Range<usize>, Color)>, so existing callers
constructing or reading it must change. The approved plan explicitly inventories
only RenderOptions, LayoutInput and ParagraphBlock. The note tuple is a direct
continuation of colored paragraph bars across pagination and falls within the
approved pre-1.0 rendering-structure choice. Name this actual field-type change
in the plan and public compatibility documentation. No second approval or
parallel options or metadata hierarchy is required.

## Nitpicks

None found.

## Not found

- Correctness: legal colors and decorations remain transient. Projection retains source order and innermost revision color, while inherited decoration behavior remains unchanged. First-visible change-bar selection skips empty runs, then considers paragraph marks and property-only fallback.
- Cache identity: style participates in reusable-engine context comparison, paragraph and table keys and field-substitution inputs. Accepted/plain cache dispatch validates style before cache lookup. Caller-font builders carry the same validated options. Missing route-specific regression coverage is recorded as D2.
- Native and binding legality: native dispatch, CLI convert/render and all eight modified Python rendering methods reject legal/accepted combinations before output. Python stub signatures match keyword-only implementation arguments. Existing accepted/plain defaults remain intact.
- OOXML: additional namespace closure is confined to checked paragraph admission and replacement publication. It subtracts retained run-root bindings and closes only preserved drawing owners. Unchecked parsing and source snapshots retain their prior paths. Tests check local/ancestor declarations, foreign shadows, exact opaque interiors, reopen fixedpoints and atomic refusal. No new XML engine or family dependency appears.
- Native paint: source and exported PDF identities are separately recorded. Native colors and six decoration strokes agree with the product's legal scheme. The declared double-stroke rule fits the12/24/36point independent measurements within0.36points. A4/font differences and the existing strike-width divergence are recorded, without a full-page SSIM claim. Missing scale product export follows D1.
- Panics: changed production paths introduce no unchecked indexing or untrusted conversion panic. Checked parsing propagates XML and namespace errors through existing result types.
- Structure: shared underline_strokes replaces three duplicated formulas. No new trait, generic, wrapper, module, feature flag or production dependency. The additional public compatibility inventory is recorded as S1.

Scoped tests, native scale successor, installed bindings, publication riders and
final harness acceptance remain pending and are not claimed by this review.
The review pass ends here. Return control to sprint orchestration and the
separate remediation phase. No source was edited in this review.
