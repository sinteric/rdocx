# F-X206, Complete PowerPoint shape authoring and live handles

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: F-X194, F-X197
**Issue**: 309 and 323, captured in S91-intake-issues.json on 2026-10-09
**Prerequisites**: F-X194 and F-X197
**Implementation**: owned implementation on accepted provider history with source-qualified scoped evidence

## Problem

Issue309 requires shape operations missing in native Rust as well as Python,
plus two concrete ancestor-handle defects. Complete acceptance is:

- Shape, picture, group and table alt_text and alt_title in p:cNvPr, plus
  the a16:decorative extension through shape.decorative.
- shape.flip_h and flip_v through DrawingML transform flags.
- shapes.group(existing_shapes) and group.ungroup, preserving slide positions
  through nested coordinate systems, rotation and flips.
- connector.begin_connect(shape, site) and end_connect, storing stCxn/endCxn
  and moving endpoints to preset-geometry connection sites.
- placeholder.insert_picture(path), cropping the picture to fill the existing
  placeholder rectangle.
- cell.vertical_anchor through a:tcPr anchor.
- Internal run slide hyperlinks with ppaction://hlinksldjump.
- Run highlight and small caps through a:highlight and cap="small".
- Native align/distribute helpers for horizontal/vertical positions relative
  to the slide or selection, usable by Python, CLI and WASM consumers.
- Native placeholder names matching PowerPoint, such as Title1 and Content
  Placeholder2, rather than generic PlaceholderN.
- Slide layout handles remain live after add_slide.
- Slide handles remain live after replacing the title TextFrame.text, while
  unsafe descendant paths still fail loudly.
- Every item saves/reopens, has a typed stub, and saved files open in
  PowerPoint without repair. Flips, groups, connections and crop require
  actual render evidence.

Google Slides import remains best effort. Descr, flips, groups, glued
connections, picture placeholder crops and cell anchor should use standard
OOXML. Highlight is text background, not a paragraph or shape fill. Title
and decorative import differences are documented without claiming CI support.
Issues264 and281 remain excluded.

## Spec reference

- `docs/hld/10-bindings-spec.md`, Python API shape and checked handle invalidation.
- `docs/hld/02-scope-and-non-goals.md`, native and Python capability rows.
- `docs/hld/04-opc-and-packaging.md`, The package and owned relationships.
- `docs/hld/08-rendering-spec.md`, Text in a shape, Tables and deterministic output.
- `docs/hld/14-development-backlog.md`, F-X206 and the full linked issue acceptance.

## Approach

Native ShapeRef and ShapeMut are rpptx/src/lib.rs:7805 and :7822. ShapeMut
already routes set_position:7855, set_size:7861, set_rotation:7870,
set_name:7876, set_fill:7891, set_line:7897 and set_crop:7912 through
concrete typed child kinds. AlternateContent is read-only for existing
mutations. Retain that boundary and atomic unsupported errors for new ones.

Add concrete alt text/title/decorative getters and checked native setters
over existing shape/picture/group/graphic-frame non-visual property handling.
Do not parse every cNvPr operation in the Python wrapper. Existing model
files are rpptx-oxml/src/shape_tree.rs, picture.rs, graphic_frame.rs and
connector.rs. Imported unknown extensions and child positions must retain
exact bytes. Decorative must update its actual namespaced extension without
discarding unrelated extLst entries or duplicating its URI. Foreign and
rebound a16 prefixes remain preservation controls, not decorative flags.

CT_Transform2D already has flip_horizontal and flip_vertical in
oxml-drawing/src/xfrm.rs:107 and :108. Add native facade accessors preserving
the other transform fields. Boolean false versus missing flags must be
documented. Support each actual transform-bearing shape kind, including
groups and graphic frames, with checked unsupported errors.

Existing ShapesMut at native lib.rs:7193 and native group creation at :7241
are the insertion points for group/ungroup. Existing group child mutation
at :7839/:7847 and coordinate helpers at :7480 through :7585 provide current
mechanics. They do not provide an operation to group an existing selection.
Stage the whole selection mutation, validate same owner, duplicate selection,
ancestor/descendant collisions, actual child kind, transform bounds and IDs,
then commit once. Preserve z-order, relationships, IDs and unsupported raw
nodes. Ungroup converts children into the parent coordinate space rather
than copying offsets or dropping rotation. Do not silently flatten opaque
AlternateContent children or approximate unsupported transforms.

The existing pivot_shift documentation at lib.rs:7556 explicitly states that
PowerPoint flips then rotates, while the renderer rotates then flips. They
differ for groups that are both rotated and flipped. This earns an actual
combined-transform oracle and an explicit decision on rendering semantics.
No assumption that grouping is an output-neutral wrapper is justified.
Reject unsupported decomposition atomically or implement and review the
actual affine transformation needed by the issue. Existing unit constructors
still truncate. Geometry algorithms must document any independently required
rounding and may not change the constructors to get a desired picture.

Connectors already model start_connection and end_connection at
rpptx-oxml/src/connector.rs:39 and :40. CT_Connection serialization respects
schema order and retained connector property bytes. Public glue and endpoint
site resolution are missing. Add native owner-aware operations using actual
shape IDs and preset geometry connection sites. Validate the site index and
geometry before editing connector or target. Define the behaviour when the
target later moves, changes geometry, is removed or is regrouped. A stored
stCxn alone does not satisfy the endpoint requirement.

Picture placeholder filling must replace the selected placeholder tree child
with a picture carrying its placeholder identity and metadata, allocate its
image part/relationship through the native owner, and crop using image aspect
ratio and inherited placeholder rectangle. Existing ShapeMut.set_crop:7912
and Presentation image operations :3358/:3489 are reusable. A generic picture
insert followed by deleting the placeholder is not equivalent without
preserving order, identity, relationships and inherited geometry atomically.

Cell anchor is a:tcPr anchor. X194 TextFrame.vertical_anchor belongs to
a:bodyPr, and they are different sources. Native TextFrame.set_vertical_anchor
at lib.rs:9079 cannot substitute for the cell property. Add checked
TableCellRef/Mut cell-anchor accessors on the existing typed cell properties,
then expose them in current rpptx-py/table.rs. Formatting changes preserve
cell/grid handles and nested text topology.

Presentation.set_run_hyperlink at native lib.rs:3967 handles external address
relationships. Existing shape_target_slide:3742 and set_shape_hyperlink:3787
provide internal-link mechanics to inspect and reuse. Extend the native run
contract for a checked target slide and action, respecting owner slide and
relationship pruning. Bind it through the existing text.rs Hyperlink class.
Retain X194's explicit cell-run hyperlink refusal until a real cell native
owner path is separately approved. Test refusal before relationship mutation.

CT_TextCharacterProperties at oxml-drawing/src/text/paragraph.rs:610 has
all_caps Option<bool>, but no typed small_caps or highlight. The existing
test character_all_caps_round_trips_while_small_caps_stays_unmodelled at
:2285 intentionally preserves cap="small" raw. Promote only the requested
model fields, retain existing public all_caps behaviour, update raw ownership
without duplicate cap attributes, and preserve unsupported siblings in their
ordered slots. Then update native resolution/rendering where necessary.
Python-only setters over raw attributes would leave native consumers behind.

Align/distribute belong in existing native ShapesMut/Presentation operations,
with Python wrappers consuming checked same-owner selections. Use slide
coordinates, actual transformed bounds and an explicit reference rectangle.
Validate spacing/selection cardinality before writes. Final design must
state whether visual rotated bounds or unrotated geometry defines alignment,
and prove that choice against PowerPoint. Do not add unrelated CLI commands
or WASM exports without an approved signature scope.

Placeholder naming belongs in native add_slide/template-derived placeholder
creation, not only in Python Shape.name. Preserve explicit imported names.
Source-build all supported layout kinds and compare with pinned PowerPoint
names and numbering rules, including duplicate shape IDs/name collisions.

## Concrete handle partition proposal

The current source has one RevisionCounter in PyPresentation at
rpptx-py/presentation.rs:151. PySlideLayout and its collection at slide.rs:25
and :67 store ContentPath and validate the same global revision as all
structural descendants. add_slide bumps it at slide.rs:586. TextFrame.text
at text.rs:223 and Paragraph.text at :556 also bump it. This is why title
text invalidates Slide, and adding a slide invalidates an unchanged layout.
Removing revision checks globally would let indexed handles silently bind to
different objects and is rejected.

Preferred smallest direction for design review:

1. Keep the existing structural revision for indexed shape/slide topology.
   Successful shape insertion/removal/move/grouping and slide removal/reorder
   retain loud stale-path behaviour. Formatting does not bump it.
2. Generalize the proposed X194 cell-text generation into one concrete text
   topology RevisionCounter in PyPresentation and one optional text revision
   field on existing ContentPath. The current approved X194 field is named
   text_revision with PyPresentation.text_revisions, reconciled in F-X194
   before either story is claimed. F-X194 initially captures Some for cell
   text only. This story adds actual shape-text consumers to the same fields,
   without a second counter or another field migration.
3. Shape-backed and cell-backed TextFrame paragraph/run/font consumers
   capture that optional generation directly and inherit it into descendants.
   Text replacement, paragraph/run insertion and cell replacement/merge/split
   bump it only after success. Structural revision validation remains first.
   Shape/Slide/Table/Cell identities without text generation remain live
   across text-only topology changes. Conservative invalidation of all
   presentation text descendants is explicit and acceptable only if tested
   and documented. No per-cell map or generic revision policy framework.
4. Give the actual layout handle kind a separate concrete layout-inventory
   generation in PyPresentation. PySlideLayout/Collection check that captured
   generation and native bounds, rather than unrelated shape or text changes.
   Existing-layout add_slide does not change it. Any operation that replaces,
   removes, reorders or imports the layout inventory updates it after success.
   Audit all present native layout mutation paths before claiming this.

The text counter serves concrete shape and cell text consumers today. The
layout counter serves actual SlideLayout and SlideLayoutCollection consumers
today. Both use existing RevisionCounter, not a new trait, map, builder or
dynamic framework. The partition is an explicit contract migration and must
retain meaningful stale checks and exact recovery messages.

Checked stable-owner IDs are a possible narrower alternative if the layout
inventory never reorders. The current native facade exposes SlideRef.id at lib.rs:7674 and
ShapeRef.non_visual_id at lib.rs:9636, but no indexed stable-ID resolver or layout owner-part
projection was found. Do not invent an existing API or blindly resolve reused
IDs. Any stable-owner approach first needs minimal native lookup, owner-kind
checks, uniqueness/non-reuse reasoning and removed-owner tests. A stable
slide handle is not required to fix title-only invalidation because the index
is unchanged. Prefer the counter partition over unrelated stable-ID machinery
unless a concrete accepted operation requires handle survival across reorder.

Alternative retaining X194's cell-only field would need a second shape-text
field/counter and duplicate inheritance logic. That increases cases readers
must consider. Do not choose it merely because the earlier draft existed.
The field names are reconciled with F-X194. Complete the mutation and
constructor audit before implementing the additional shape partition.

## Ownership and affected paths

X194 owns cell text traversal and the initial cell topology invalidation
contract. X197 owns shared colour validation and colour formats, used for
highlight input without another extractor. X205 owns the fresh 16:9 template
and metadata, so placeholder names and inherited geometry must be reconciled
with its authenticated template update. X206 owns shape mutation and the
ancestor-handle partition after explicit plan reconciliation.

Likely exclusive files are native rpptx/lib.rs, current rpptx-oxml shape,
picture, connector and graphic-frame files, oxml-drawing/xfrm.rs and current
text model/resolution files only where required, existing rpptx-py
presentation.rs, slide.rs, shape.rs, text.rs and table.rs, shared
oxml-py-support/src/lib.rs for the coordinated ContentPath field, current
stub/enum files and current tests. These overlap prerequisites, so claim
after integration or agree serialized ownership. No new file is prescribed.

## Rejected alternatives

- Duplicate prerequisite colour/table APIs instead of reusing their actual result.
- Treat the issue's claimed native parity as proof where source inspection shows gaps.
- Drop unknown source or stale-path checks to make the missing API appear usable.

## Test plan

**Test gate**: regression.


Reproduce each requested operation through installed Python on the claimed
Base, and use native public source-built regressions for operations missing
in Rust. Preserve two exact current stale-handle failures as baseline
evidence before changing revision semantics.

| Test | Required evidence |
|---|---|
| test_shape_accessibility_and_flips_round_trip | All named kinds, true/false/None policy and foreign extension preservation |
| group_existing_shapes_preserves_transformed_slide_positions | Rotated/flipped/nested geometry, z-order, IDs and exact opaque nodes |
| ungroup_preserves_combined_transform_and_relationship_ownership | Inverse transform, invalid/opaque atomic refusals and native render oracle |
| glued_connector_endpoints_follow_preset_connection_sites | Endpoint facts, invalid sites, move/delete/regroup behaviour |
| test_picture_placeholder_fill_keeps_placeholder_and_crops_to_fill | Landscape/portrait images, inherited geometry, package ownership and rendering |
| test_cell_anchor_is_distinct_from_text_frame_anchor | Actual tcPr/bodyPr values, save/reopen and unchanged text handles |
| test_internal_run_slide_hyperlinks_prune_owner_relationships | Same/cross-owner targets, deletion and no-orphan rejected cell links |
| test_run_highlight_and_small_caps_preserve_other_caps_metadata | Typed promotion, all/none/small, unrelated raw attributes/children and render |
| align_and_distribute_use_checked_transformed_selections | Every direction/reference, cardinality, bounds and atomic refusal |
| test_native_placeholder_names_match_powerpoint | All supported layouts, duplicate names and imported-name preservation |
| test_layout_handles_survive_add_slide_but_detect_layout_change | Collection/item identity, actual inventory mutation and exact stale recovery |
| test_slide_survives_title_text_while_descendants_stale | Slide/Shape live, old paragraph/run/font stale, fresh paths usable |
| test_text_topology_generations_cover_shape_and_cell_consumers | Coordinated X194 replacement/merge/split/add operations, no-op and failure |
| test_true_structural_mutations_still_reject_old_index_paths | Remove/reorder/group boundaries and no silently rebound handles |

Existing Python test_documented_examples.py contains staleness cases at :215,
:314 and :359, add_run at :1553, fill/line :2328, table :2412, picture crop
:3078, shape/slide reorder :3137/:3322 and shape slide links :4049. Update
only the expected contract cases, retaining old unsafe-path controls. Use
current typing_smoke.py and handwritten stub for strict mypy/stubtest.
Native tests stay in existing lib.rs/model modules and the current integration
entrypoint. No new integration binary.

Round-trip twice, with alternate/default namespace aliases, rebound and
foreign names, unknown extensions and exact unsupported bytes. Atomic
failures retain package bytes, relationships and both topology generations.
Getter reads do not create properties or advance revisions. Renderer claims
need deterministic fonts and actual native PowerPoint evidence for combined
rotation/flip, glue routing and placeholder fill. PowerPoint open-without-
repair must be separately observed, not inferred from library parsing.

## Risk routing

Matched rows are unit conversion, parser/serializer, rendering/layout,
published native API, PyO3, dependency edges, colour/theme and external
oracles. Read HLD01/03/04/05/06/07/08/10 and HLD15 packaging, plus the
canonical differential-testing skill. Preserve canonical truncation, child
order, exact raw XML, retained colour transforms and the Word tint routine.
Earned riders include actual published archive dry-run/size/measurements,
WASM compilation, binding exclusions, pinned deterministic render evidence,
normal scoped verification and independent microscope.

Expected wrapper/handle-only default harness delta is none. Placeholder-name
defaults, promoted native render semantics and a combined group transform
correction can intentionally change output. Each actual default change needs
its own labelled reviewed commit and exact delta. Root owns the exclusive
baseline, coordinated with X205's template change. No unexplained delta may
be waived as a harmless binding change.

## HLD impact

- docs/hld/02-scope-and-non-goals.md, actual native/Python capability rows.
- docs/hld/03-architecture.md, concrete shared path/counter ownership if changed.
- docs/hld/05-drawingml-model.md, caps/highlight, transforms and connector sites.
- docs/hld/06-presentationml-model.md, accessibility, group/placeholder/link ownership.
- docs/hld/07-inheritance-and-resolution.md, geometry and placeholder resolution if changed.
- docs/hld/08-rendering-spec.md, actual transform/highlight support and deterministic deltas.
- docs/hld/10-bindings-spec.md, exact methods, public types and revised invalidation contract.
- docs/hld/14-development-backlog.md, complete delivered acceptance and retained boundaries.

## Hash harness

Wrapper exposure should preserve existing defaults. Actual native semantic
changes require measured declared deltas and separate labelled commits before
any exclusive baseline update. The risk discussion above identifies those
possible changes. No baseline movement is preapproved. Implementation requires completed dependencies and exclusive source claims.

## Implementation checklist

- [x] Resolve technical contracts and actual native gaps against prerequisites.
- [x] Approve complete issue acceptance, source claims and measured output policy.
- [x] Claim only after actual prerequisites and file conflicts clear.
- [x] Prove installed public Base regressions before production changes.
- [x] Implement every requested capability, stubs and existing-entrypoint tests.
- [x] Run scoped gates and exact applicable oracle/risk riders.
- [x] Obtain independent zero-defect, zero-smell review and prepare handoff.

## Open questions

Resolved by the bounded causal provider, geometry, connector and rendering
contracts below. F-X194 remains initial numeric cell-generation owner, and
F-X206 extends it once after completed F-X194/F-X197. Word/PPT captured-token
migration belongs here, while F-X211 audits residuals and the final corpus.
Existing-file ownership and dependency completion gate the actual claim.
Issue323's native/render control, source-built failures, no-op/late-refusal
atomicity and labelled output deltas remain completion tests.

## PR317 cell anchor reuse

Reuse the actual a:tcPr anchor slice from immutable
3d056f46e97e9cc1caf2969b5096eeb6775a5fa0: contributor oxml-drawing/table.rs:110
and1752, native rpptx/lib.rs:8566, binding table.rs:662 and layout
context.rs:2356. The independent context test at6563 distinguishes tcPr
from bodyPr and must be reproduced locally. F-X194 retains the reused
TextFrame bodyPr surface. One owner implements actual cell anchoring.
Do not adopt unrelated colour, accessor-read mutation, archive or hash
records as part of this slice.

## Authenticated combined group transform

## Native method

Microsoft PowerPoint 16.113.3, build 16.113.26092714, was operated through CUA only. An independent copy of the prior native blank template received a generated slide with twelve small filled square markers. The input opened without a repair prompt. Native Save As wrote a separate local copy. The PDF was exported using explicitly selected Best for printing, without an online export service. The saved generated deck was closed, normally reopened from its exact local path, saved, and exported again using the same local option.

The input generator writes correctly ordered DrawingML group and shape XML and records every transform, center and independent numerical prediction. It does not write an expected output or modify any saved native package. Prediction order is tested independently using both candidate linear transforms, with screen coordinates where positive y points down and positive rotation is clockwise.

For a point relative to the group's extent center, the accepted order is:

```text
local child-coordinate mapping to group extent
horizontal/vertical flip about group extent center
clockwise rotation about the same center
translation to the parent coordinate system
```

The flip/rotation comparison computes `R * F` and `F * R` directly. Nested transforms compose child first, then parent. The PDF content stream provides each colored square's four actual vertices. Their arithmetic mean gives its native center. The y coordinate is converted from the PDF's bottom origin using the actual 540-point slide height. This uses vector coordinates, rather than reading positions from a screenshot.

## Controls and results

All controls use asymmetric red, green and blue marker locations, so a flip cannot be mistaken for unchanged symmetric contents. Group extents are3500000 by2000000 EMU. The ordinary H control rotates30 degrees with flipH. The V control rotates35 degrees with flipV. A nested control has a25-degree flipH outer group and a minus40-degree flipV inner group with its own offset and smaller extent. The fourth control rotates40 degrees without flips, which should agree under both candidate orders.

All twelve native marker centers match flip then rotate. The maximum distance is0.00010007642342126402 points, consistent with the decimal precision of the emitted PDF path coordinates. The nine markers in groups with combined flip/rotation differ from rotate then flip by43.30707598425451 to143.46078135286186 points. The three no-flip markers agree under both orders. This directly confirms the `pivot_shift` comment's PowerPoint order and supplies concrete counterexamples to the current renderer's opposite combined order.

The first and reopened page-one PDF content streams are byte identical. Native slide1.xml is byte identical across the normal reopen/save. Input package bytes retain their recorded initial SHA-256. Native changes during initial Save As are retained in separate original and saved owner XML, so serializer normalization is not confused with post-reopen geometry drift.

Pinned Poppler26.01.0 rasterized the reopened page, and the image was visually inspected. All twelve markers are visible in their expected quadrants, with the nested case visibly asymmetric. The native template has a second blank slide, which was preserved. Only page one is the affine control and only page one is used for the position comparison.

## Evidence

`/private/tmp/S91-F-X206-native-affine` contains `create_controls.py`, `input-receipt.json`, the unchanged input, native before/after reopen packages, both native PDFs, exact extracted slide XML, PDF path streams, per-marker comparisons, a rendered page PNG, `audit.py` and `capture-receipt.json`. The receipt contains file sizes and SHA-256 values, native application version/build, all marker vertices, both candidate errors, and the byte-identity assertions. The complete numerical comparison can be rerun without opening PowerPoint.

The generated native deck was closed only after its exact generated URL and Saved to my Mac state were visible. PowerPoint returned to its new/recent-files window without any remaining document. The UI lease is released. No user document, native application preference, tracked repository file, Cargo artifact or font was modified.

## Bounded conclusions and remaining acceptance

This establishes combined flip/rotation order for ordinary and nested groups with unit axis scale. It does not establish nonuniform group scaling combined with child rotation, skew decomposition, unsupported opaque nodes, group/ungroup position preservation, ID and relationship lifecycle, connectors, or stable handle generation. The initial group inputs were not modified through a native group or ungroup operation. Those acceptance cases still need their own operation-specific implementation and controls.

Native align/distribute semantics were not exercised in this observation. Visual rotated bounds versus unrotated frame geometry remains an explicit separate decision. The affine result does not imply one of those policies.

## Authenticated native selection alignment

## Controlled input and native actions

Microsoft PowerPoint16.113.3, build16.113.26092714, was operated only through CUA. A copy of the prior native blank template received three generated correctly ordered DrawingML rectangular shapes on its first slide. The input package hash and every original transform are in `/private/tmp/S91-F-X206-native-alignment/input-receipt.json`. No output position was written into the input. The second existing template slide was preserved and is outside this control.

| Name | Original off x,y | Extent cx,cy | Rotation |
| --- | --- | --- | --- |
| RotatedRed | 1600000,900000 | 2300000,800000 | 45 degrees |
| PlainGreen | 4500000,2200000 | 1200000,1000000 | 0 degrees |
| RotatedBlue | 7200000,4000000 | 900000,1800000 | 30 degrees |

The input opened without repair. Native Save As created `alignment-native.pptx` separately. A click on the visible red shape followed by Command-A selected all three slide shapes. A native screenshot showed all three distinct sets of selection handles, so this was not text selection or thumbnail selection.

Home Arrange > Align explicitly showed Align Selected Objects checked. Align to Left was clicked. The native toolbar Save was then clicked after the menu had closed. The saved package was copied to `alignment-left-native.pptx`. The same selected objects then received Arrange > Align > Align to Top, again with Align Selected Objects checked. An explicit subsequent toolbar Save persisted that operation. Those actions changed native xfrm offsets. No saved native package was patched by a script.

An early Command-S in the same call as the first menu selection had not persisted the changed positions. That intermediate read was not used as evidence. The explicit Save after the freshly observed editor state produced the actual changed native package, which is the retained Left result.

## Native saved coordinates

| Name | After Align Left off x,y | After Align Top off x,y |
| --- | --- | --- |
| RotatedRed | 1600000,900000 | 1600000,900000 |
| PlainGreen | 1653984,2200000 | 1653984,203984 |
| RotatedBlue | 2043696,4000000 | 2043696,308407 |

The three raw x offsets differ after Left, and the three raw y offsets differ after Top. The extents and rotations remain unchanged. The minimum original rotated visual left is1653984.4891608513 EMU. The minimum original rotated visual top is203984.48916085134 EMU. Every saved shape's corresponding visual edge equals those reference edges within2 EMU. The reference is the original selection's outer visual edge, not the slide edge or raw unrotated off coordinate.

Independent numerical comparison uses the axis-aligned bounds of the four rotated frame corners. Equivalently, a rectangle's rotated bound width is `abs(w*cos(theta)) + abs(h*sin(theta))`, with the corresponding height formula. Rotation occurs about the frame center. The native PDF provides a separate actual vertex measurement rather than only agreeing with an XML formula.

## Export and normal reopen

Local Best for printing PDF was explicitly selected, avoiding online export. The saved generated deck was closed only after its exact generated URL and Saved to my Mac state were visible. Its observed recent-file entry pointed to the same generated directory and was normally reopened. The native toolbar Save was clicked without any edit, and the same local PDF option produced a second export.

Both native PDFs' page-one vector streams are byte identical. Saved slide1.xml is byte identical across the normal reopen/save. Original input package bytes retain the initial recorded SHA-256.

All three emitted polygons have left130.235 points. Red's top is16.0618 points, while green and blue top is16.0617 points, within emitted path precision. The actual vector vertices therefore establish shared visible edges after both operations. Pinned Poppler26.01.0 rasterized the reopened PDF, and the page was visually inspected. The shapes overlap because both their left and top edges were aligned, with original z-order preserved.

The final generated deck was safely closed, and PowerPoint returned to its new/recent-files window with no remaining document. No user document, application preference, font, production source, tracked repository file or Cargo artifact was modified. The PPT UI lease is released.

## Evidence and bounded conclusion

`/private/tmp/S91-F-X206-native-alignment` retains the original input, original transform receipt, native Left snapshot, native Left-plus-Top before/after reopen packages, two native PDFs, rendered PNG, extracted owner XML, PDF path streams, `audit.py` and `capture-receipt.json`. The receipt records file sizes and SHA-256 values, exact native action sequence, every original/final transform, numerical bounds and actual emitted PDF vertices. Running `audit.py` independently validates all assertions without opening PowerPoint.

This answers the core native selection-alignment question: Align Left and Align Top align the transformed visual bounding boxes of these differently rotated rectangles. It does not prove behavior for slide-relative alignment, distribution spacing, nested-group selections, nonuniform scaling, lines/connectors, arbitrary freeform paths, cropped pictures, shadows, stroke extents or unsupported shapes. Root may choose the measured visual frame-bound policy for the declared API and retain those operation-specific cases in its acceptance tests. No claim about arbitrary painted-ink bounds or automatic generalization to distribution is made.


## Rolling Issue323 connector-in-group defect

[Issue323](https://github.com/tensorbee/rdocx/issues/323) is included in this
existing shape/connector story. Its reported reproduction adds a connector
inside a group rotated30 degrees, horizontally flipped and scaled2x through
unequal a:chExt/a:ext. Rendering or PDF conversion then drops every member.
LibreOffice is reported to retain the group. This is additional acceptance
for the existing nested transform and connector work, not a new F-ID.

Capture a real Base failure through the installed public operation. Compare
the actual connector and retained rectangle positions with pinned
LibreOffice output and an authenticated PowerPoint control. A member that
cannot be resolved must produce an owner-specific render diagnostic and
must not suppress resolvable siblings. Test nonuniform/unequal child extents,
rotation and flips together, not only the existing unit-scale markers.
Retain member IDs, z-order, opaque XML and save/reopen evidence. Reuse the
shared group transform and connector implementation once. This intake has
not reproduced the defect or changed production source.

## Complete PR326 and Issue323 reconciliation

PR326 at a81d94683688d5e4d144e0f2dafd9c876548a293 closes Issue309 and
attaches here. Its36-file cumulative capture and71 base/head Git blobs are
retained in /private/tmp/S91-PR326-intake, receiptSHA
47b21f03bb8b454f0e85359590b986d010e0373a6116ce65a03c01d599c65c8c.
Affine order and visual-bound work are useful bounded contribution input.
Nonuniform rotated-child ungroup approximation must become supported exact
behavior or atomic refusal, not silent drift. Reject failed alignment before
live geometry materialization. Preserve qualified decorative/highlight raw
extensions, glued-connector lifecycle and placeholder relationship closure.
The contributor explicitly leaves Issue323 unresolved. Retain resolvable
siblings and owner-specific diagnostics for unresolved grouped connector
members. Its global kind-counter history is not the selected X211 causal
owner contract. New and later PRs remain under this owner, without a new F-ID.

## Approved shared causal scope and operations

## Single implementation ownership, without dependency cycle

F-X194 owns initial cell resolver/text generation. F-X206 owns extending that same generation to shape text and notes, distinct layout/master inventory scopes, PPT ancestor survival, native affine/connector/arrange/caps/highlight and cell-anchor work. Assign the shared bounded causal-state foundation and minimal existing Word/PPT consumer migration to F-X206 once in oxml-py-support and existing binding files under explicit root leases. Preserve public numeric constructor behavior from the final causal memo. X202 owns Word hyperlink physical identity and related picture/table operations. X201 owns sections. X209 owns master/layout/theme part identities. F-X201/F-X202/F-X205/F-X208 formally depend on completed F-X206. F-X209 already depends on it. F-X194 does not depend on F-X206 and retains its initial numeric text-revision extension. None depends on211.

X211 follows integrated providers. It audits/refines residual Word table/grid ancestors, Word hyperlinks and other uncovered consumers without reimplementing provider lifetimes, adds checked fourteen-owner XML operations, actionable errors and the final corpus. If root assigns a residual fix to a provider,211 retains only its cross-story acceptance. Formal211 depends on200..210 for final corpus. Do not make194/206/202/209 wait for211 approval or completion. Design approval is conditional on fixed contracts, claims wait for actual dependencies and file leases. Shared consumer migration is a declared scope amendment, not silently expanded206 source ownership.

Use actual current model owner inventory, not a generic stable-handle registry. Shape/Slide/Table/TextFrame ancestor identity survives descendant text edits. Layout/master proxies are keyed by checked originating part ownership and separate inventory tokens, not global shape revision. Existing shapes can survive unrelated insert/remove only when provider proves their original owner token, ID uniqueness and non-rebinding. Indexed Row/Column/Cell and paragraph/run/font descendants still retire for affected grid/text topology. Removed/recreated native IDs never resurrect tokens. Layout import/reorder/removal retires only affected inventories. Notes ancestor behavior uses the same text-body partition, not an independent counter. Do not promise live cell descendants across grid edits.

Common top-level revision remains committed invalidating-transaction serial, distinct from local generations. One relevant successful commit advances it once, preflight overflow/all successor allocation before publication, no fallible state changes afterward. A handle holds originating document/owner and relevant Arc states. Validate foreign identity before lookup, then retired states, choosing minimum common committed retirement serial across relevant scopes. Same-transaction ties use one real public label and fixed scope order. OnceLock records first cause once, no next links/history/last-operation substitution. Manual revision-u64 selectors remain unbound/unknown cause, preserving visible equality and legacy missing-owner ordering. Manual Item with bound Story inherits only its Story owner constraint. Errors/no-ops retain bytes, all state identities and serials. Existing setters are not reclassified as no-op merely because displayed text matches.

## Geometry and operations

Adopt flip then rotate about frame center, composed with actual group child-coordinate offset/scale and all ancestor transforms. Unit-scale native controls are exact evidence for that order, not nonuniform decomposition proof. Preserve IDs/relationships and relative z-order during group/ungroup. Noncontiguous grouping cannot preserve interleaving against unselected siblings, so refuse it atomically, rather than claim total z-order preservation. Exact affine child conversion is supported only when representable by DrawingML rotation/flip/scale without skew. Nonuniform rotated-child skew or opaque decomposition refuses before candidate publication. no heuristic axes/position approximation. Bounds and checked integer publication use one documented rounding rule local to geometry, leaving existing unit-constructor truncation unchanged.

Keep contributed enum/signature vocabulary in existing native lib.rs, no arrange.rs module: ShapesMut::align(indices:&[usize],alignment:ShapeAlignment,reference:ArrangeReference)->Result<()>. distribute(indices:&[usize],direction:DistributeDirection,reference:ArrangeReference)->Result<()>. ShapeAlignment is Left/Center/Right/Top/Middle/Bottom, reference Selection/Slide, direction Horizontal/Vertical. Public Python takes actual same-owner Shape handles, validates bound origin and converts current owner paths only after checks. No duplicate selections or ancestor/descendant mixture. Alignment requires>=1 slide-relative and>=2 selection-relative. Distribution requires>=1 slide-relative,>=3 selection-relative. All validation and geometry materialization occurs in a candidate, never the live tree.

Use axis-aligned bounds of transformed frame corners in slide coordinates, excluding shadows, stroke/painted ink and cropped picture contents. Native Left/Top controls establish this choice for rotated rectangles. Selection reference is union of those bounds, slide reference is full slide rectangle. Map slide-space translations through inverse parent affine for nested same-parent children. Refuse singular or unrepresentable transform, not all nested slide-relative operations indiscriminately. Align selected visual edges/centers. Distribute equal signed edge-to-edge gaps, preserving selection extrema. negative gaps are permitted and explicit, not clamped. Slide-relative uses both slide edges, one member is centered. Sort by leading bound then existing owner z-order for ties. preserve z-order, sizes, rotation and IDs. Precompute all translations/check overflow before any mutation. Distribution is a selected deterministic product contract, not a falsely claimed native measurement.

Connectors: adopt owner-aware Presentation::connect_connector(slide_index,connector_path:&[usize],end:ConnectorEnd,target_shape_id:u32,site:u32)->Result<()> and set_connector_endpoint with the contributor's existing enum vocabulary, but bind checked physical owner paths. Site indices are zero-based actual preset geometry connection-site inventory after guide evaluation, not guessed rectangle cardinal sites for unsupported shapes. Support same-slide nested targets and inverse-transform endpoint into connector's owner coordinates. Reject self, foreign/detached/duplicate target, invalid site, custom geometry lacking modeled sites, singular transform or read-only AlternateContent before allocation/publication. Unsupported native sites refuse authoring rather than silently substitute four sites.

Glue remains live: target position/size/flip/rotation/preset edits and exact regroup/ungroup recompute attached endpoint coordinates in the same checked candidate. Native render assembly resolves retained imported attachment IDs against current actual owner/site geometry and emits owner-specific diagnostics for unsupported attachments. Deleting a glued target detaches the relevant stCxn/endCxn while preserving the last resolved endpoint, other end and connector. do not leave a dangling target or delete the connector. Unsupported late reroute causes whole authored operation refusal. No auto-routing promise for elbow bends beyond supported connector geometry. Save/reopen preserves attachment semantics. Member-level renderer failures must not suppress valid siblings.

Issue323 remains required: fresh installed Base repro adding a connector to rotated30-degree/flipH/2x unequal chExt/ext group, plus nonuniform partner. Compare actual rectangle/connector positions with pinned native and LibreOffice, verify member IDs/z-order/opaque bytes and save/reopen. Current unit-scale markers and contributor326 do not satisfy this case. One shared transform/connector fix handles it, no second F-ID.


## Caps, highlight and XML decisions

Keep existing all_caps:Option<bool>, add minimal typed small_caps:Option<bool> and highlight:Option<ColorChoice> within current model file. They own exactly one qualified cap attribute and one DrawingML highlight child. Parse all/none/small distinctly. unsupported/foreign cap metadata remains raw. Checked setters select one cap state, clear conflicting true state, and never emit duplicate cap. Simultaneous true values in supplied native property values refuse before publication, not arbitrary precedence. None clears the requested owned flag without destroying another explicit mode, false explicitly selects cap=none. Preserve legacy all_caps getter semantics for imported small. Namespace URI qualification is mandatory, including raw-child replacement. foreign highlight remains opaque. X197 owns RGB/theme extraction. Getters do not promote defaults or mutate source.

Render actual all-caps and small-caps with original text/cluster identity preserved. Prefer OpenType smcp when available. Select a deterministic synthesized fallback when the font lacks smcp: lowercase cased clusters display their Unicode uppercase mapping at80% of the original run size, while original uppercase and uncased clusters keep full size. Preserve original logical text, cluster source mapping and font face, including multi-codepoint uppercase expansions. This80% ratio is an explicit product policy, not measured PowerPoint equivalence. Declare its expected pixel/width delta and test font-feature and fallback controls before the behavior commit. Root may adopt this precise choice now rather than leave an unbounded render-policy question. Highlight paints its resolved colour behind the actual run glyph area with alpha, preserving other effects. A typed save/reopen alone is not rendering acceptance.


The neutral-state scope amendment owns ContentPath/RevisionCounter in existing
oxml-py-support/lib.rs and actual current Word/PPT capture, validation and
publication consumers in their existing binding files. Initial Word migration
preserves current conservative global invalidation and manual numeric selector
semantics. It does not preimplement X201 section authoring or X202 physical
hyperlinks. New consumer handles use this completed provider. Removing Copy,
hidden owner tokens and common serial changes require both affected native
families, installed runtime/typing/stubtest, WASM, packaging, Base failure,
no-op/overflow/first-cause and bounded state-lifetime checks. Every invoked
public mutation label must be accurate. No new file/module/trait/generic.

## PR335 reconciliation and member resolution policy

[PR335](https://github.com/tensorbee/rdocx/pull/335), immutable head
e2875c7ece140439286c6e6948a378ac2a343250, closes the already assigned
Issue323. Its complete source assessment is retained under
/private/tmp/S91-PR335-e2875c7-intake. Both group transform function bodies
are byte-identical to the assessed PR326 versions. Adopt that correction
once. Reuse its equal-axis2x connector fixture without claiming unexecuted
LibreOffice/native evidence or satisfying the required nonuniform partner.
Installed Base, actual native positions, member IDs/z-order/opaque bytes and
save/reopen remain separate required gates.

Select member-local resolution recovery for the existing semantic
ResolveError variants: StyleIndexOutOfRange, UnresolvedPlaceholderColor
and ConcreteValue returned while resolving one member. Retain valid
siblings in authored draw order. A failed member with trustworthy finite
bounds uses the existing unresolved-shape bounds-outline fallback under
its actual ancestor transform. Without trustworthy bounds it is omitted
from drawing with a named omission diagnostic. Increment unresolved count
before deciding whether bounds permit drawing, never afterward.

Use one owned unresolved_member_count in the existing ResolvedSlide,
initialized to0 and incremented once per failed member. The strict corpus
gate consumes that count independently of drawable fallback shapes. Native
render reporting also consumes it to identify incomplete slide output,
alongside ordered owner-specific diagnostic messages. This additional
field is a pre-1.0 Rust source change for struct literals. Update all actual
existing literal/renderer consumers and archive/API evidence. No new
public type, module, dependency or generic recovery framework is proposed.

The recoverable render/convert may return successful bytes with these
explicit diagnostics. Existing byte return types remain unchanged, while
the approved same-pass companions and CLI stderr expose the actual facts.
Identify failed owner/ID/name and reason once. Do not represent partial
failed-member font/paint facts as successfully drawn output. Other valid
sibling diagnostics remain ordered and intact. Fatal package/XML admission,
slide background/context setup, backend encoding or I/O failures still
return errors. Recovery does not turn a refused authored connector/group
operation into success or weaken its atomicity and source preservation.

Add malformed-style member and boundsless-member controls with valid
siblings, asserting actual pixels/order, owner diagnostics, unresolved
count and a failing strict corpus gate for both kinds of recovery. Include
fatal background/encoding controls and count0/unchanged output for valid
slides. The contributor gate scans emitted shapes and misses boundsless
omissions, so it cannot be adopted as proof that fallback hides no failure.
These rendering semantics require their own labelled behavior commit and
measured reviewed deltas, separately from the duplicated group correction,
handle migration and authoring wrappers. Root holds the baseline lease.

## Qualified deterministic package serialization deltas

The two existing SmartArt and F116 package controls retain their complete
validation, owner, rendering and reopen assertions. Their historical archives
were reproduced from exact Git source commits and recovered both original
whole-package SHA-256 values. The R39 comparison records every changed member.
SmartArt changes six of sixty members and F116 changes eleven of sixty.
Member order and ZIP writer options remain identical. All other member bytes
remain exact.

The approved native placeholder naming changes generated Title Slide names
from Placeholder 2 and Placeholder 3 to Title 1 and Subtitle 2. Namespace
closure adds canonical DrawingML and PresentationML bindings to text-body
owners, and preserves the inherited diagram binding on the SmartArt shape
tree. The modeled rtl property is emitted before the preserved defTabSz
attribute on nine default text-style levels. Expanded XML is unchanged apart
from the approved generated names. Text, geometry, relationships, media,
unknown attribute values and child order remain exact. No paint baseline is
changed by this package expectation proposal.

This is an explicitly attributed serialization behavior change. The labelled
behavior commit must retain the R39 original archives and comparison evidence.
Proposed package expectations are SmartArt
9f71406a506c91df5e91f75bee01c3e7d1f4487b44d99e8709c646c8368082ea
and F116
ef24e9477e6b3b54e1c3b27dfbda8082fce6baad5f773ebf37a2577319d2c247.
They are retained R27 current artifacts, not newly measured R38 outputs.
Independent review and exact current-source regeneration of both original
selectors are required before acceptance. Historical passing tests prove
original package identity only, not acceptance of the current implementation.

## Scoped completion evidence

The owned implementation preserves claim Base7b1e5ef29464e93ae30e68df1523ce09143578ff and accepted provider history22503882. Independent foundation53, presentation52 followed by picture-preservation54, HLD-preservation55, provider-composition56 and equivalent test-placement57 cover the final owned projection. Their retained findings and corrections remain separate. The original dirty union is retained as an authenticated rollback capsule and named stash. No borrowed F200 formatting implementation or baseline update is included.

Original public and native Base failures, strict fifty-deck inventory, native PowerPoint and pinned LibreOffice controls, source-font small-caps and contour oracles, full104 installed cases and native SVG controls remain retained with their exact input-qualified dispositions. The affected composed-source eleven-crate native check and Clippy, fifteen deciding native controls, WASM, eight installed controls, PPT strict typing and stubtest passed. Current deterministic hash49 matches. Eight affected verified archives and their README-only measurement successors are source-authenticated. The full policy run retains two original failures and two expected skips. Both exact policy successors passed, followed by prose, adapter and diff checks. Current SmartArt and F116 controls regenerate the reviewed9f714 and ef24 expectations. The SmartArt native child passed and its later wrapper missing-key failure remains retained and independently qualified from exact raw counts, not replayed or relabelled.

All scope receipts and source equivalence dispositions are indexed in the worker progress checkpoint and retained task evidence. Final sprint workspace verification remains the integrator responsibility.
