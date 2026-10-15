# F-X209, Edit PowerPoint masters layouts and themes

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: F-X205, F-X206, F-X208

## Problem

Issue312 requests persisted part-owned master/layout/theme editing.
`crates/rpptx/src/lib.rs:631` retains layout records, while Presentation at
:607 has no editable master record collection. ShapesMut at :7193 addresses
one slide and group path. ShapeMut at :7822 already borrows the actual child
without a slide index, so do not add redundant ownership to that handle.
Layout and master shape authoring needs the correct part relationship owner.

## Spec reference

- `docs/hld/06-presentationml-model.md`, Public facade, Parts, shape tree,
  placeholders, preservation strategy and relationship remapping.
- `docs/hld/10-bindings-spec.md`, Python API shape and handle invalidation.
- `docs/hld/05-drawingml-model.md`, theme colours/fonts and format schemes.
- `docs/hld/08-rendering-spec.md`, Text in a shape and inherited rendering.
- `docs/hld/14-development-backlog.md`, F-X209 and full Issue312 acceptance.

## Approach

Extend existing concrete part records and shape authoring in lib.rs. A small
concrete owner discriminator addresses slide, layout or master collections.
Use the existing borrowed ShapeMut for in-place properties. Part-owned picture
insertion creates relationships on the actual owning part and content-hash
media deduplication keeps its current semantics. No parallel mirror model,
new trait, generic parameter, source module or crate.

Load and persist masters and themes with existing qualified OOXML models,
explicit dirty state and ordered writes. Add live Python slide_masters,
slide_master, SlideMaster and SlideLayout properties in existing modules.
Expose shapes, placeholders, background, show_master_shapes, text_styles,
layout name and usage. Deleting a used layout refuses before any part,
relationship, vector or handle mutation. Duplication remaps owning-part
relationships, media and shape IDs with existing copying rules.

Add solid/gradient/picture backgrounds, master title/body levels1 to5/other
font, size, colour and bullet properties, and theme colour/font mutation.
F-X205 owns common fill property wrappers and F-X208 owns theme read, so reuse
those actual integrated surfaces. Existing slide rendering must consume
persisted master/layout changes through its existing inheritance chain.

apply_theme imports colours, fonts and format schemes from pptx, potx or thmx,
with optional master/layout import. Stage the complete import and refuse
unresolved dependencies, malformed sources and unsafe relationship targets
atomically. Keep all existing slide content, opaque XML and unrelated parts.
Do not silently copy executable content with a theme. Exact selective import
and preservation policy needs resolution before approval.

## Rejected alternatives

- Repeat logos on each slide, which does not provide master editing.
- Add a slide index to ShapeMut, which already borrows its exact child.
- Rewrite all template XML when a specific part property changes.

## Test plan

**Test gate**: regression.

| Category | Test | Asserts |
|---|---|---|
| regression | master_and_layout_pictures_use_own_part_relationships | Master logo appears on every admitted slide, hidden title layout and alternate layout logo agree |
| round-trip | master_background_and_text_style_edits_follow_inheritance | Native PDF/PNG and saved PowerPoint reflect level properties and showMasterSp |
| regression | used_layout_delete_and_malformed_theme_import_are_atomic | Complete package and revisions unchanged on refusal |
| round-trip | theme_apply_preserves_existing_slide_content | Deck, potx and thmx sources apply colours/fonts/format scheme, optional master/layout import retains content |
| regression | live_master_and_layout_handles_keep_checked_identity | Related and unrelated edits have deliberate success-only invalidation |

Use existing native and Python test entrypoints. Run all three Issue312 scripts
from a fresh installed extension, strict mypy and stubtest, deterministic
native PDF/PNG, actual PowerPoint no-repair open and pinned LibreOffice
round-trip. Preserve exact unknown subtrees and schema order. Google Slides
import is best effort with explicit evidence limits.

## HLD impact

- `docs/hld/02-scope-and-non-goals.md`
- `docs/hld/05-drawingml-model.md`
- `docs/hld/06-presentationml-model.md`
- `docs/hld/08-rendering-spec.md`
- `docs/hld/10-bindings-spec.md`
- `docs/hld/12-testing-strategy.md`
- `docs/hld/14-development-backlog.md`

## Risk routing

Parser/serializer and package relationships require qualified reads, schema
sequence, atomic import and exact raw retention. Published API additions
require local-source packaging dry-run and actual affected archive inventory.
PyO3 requires installed isolated tests, typing/stubtest and both WASM checks.
Theme operations retain the deliberately pinned Word tint/shade convention.
Layout comparisons use deterministic fonts. Pin PowerPoint and LibreOffice
oracles and retain compared inputs/results. Audit dependency direction before
any cross-family use. No new source module, crate or dependency is approved.

## Hash harness

Existing immutable deck output should remain unchanged until callers edit
master/layout/theme properties. Measure changed authored output separately.
Any observed fixture delta requires an approved explicit revision before
baseline recording. No baseline movement is authorized by this design.

## Exclusive file claims

rpptx lib.rs, existing slide_parts/theme models where required, existing
rpptx-py layout/shape/dml/presentation/slide modules and stubs, existing native
and Python test entrypoints, and the HLD sections above. Source conflicts
serialize after X205, X206 and X208 completion. No worktree is claimed yet.

## Implementation checklist

- [x] Resolve exact owner record and live handle design after prerequisites.
- [x] Resolve selective theme import and preserve/refuse policy.
- [x] Approve source claims, riders and expected output behavior.
- [x] Reproduce missing public scripts before implementation.
- [x] Implement and verify every Issue312 item with native oracle evidence.
- [x] Obtain clean independent review and validated prepared handoff.

## Open questions

Resolved by the part-owned records, live proxy semantics and selective import
contract below. Apply_theme has explicit source/master selection and layout
mapping, a preflighted supported relationship closure, preserved unrelated
parts and atomic refusal of unsupported selected dependencies. One existing
shape owner/provider serves slide, layout and master consumers. X208 owns
its single theme provider, extended here rather than duplicated. Prerequisite
completion is a source-claim barrier. It does not prevent approval of these
resolved design contracts. Implementation still waits for every formal edge,
the global design barrier and exclusive existing-file claims.

## Resolved part-owner and selective import contract

## 1. One part-owned model and shape authoring path

Presentation currently stores layouts/slides at crates/rpptx/src/lib.rs:626. LayoutRecord at631 has part_name and CT_SlideLayout only. layout_and_master at2142 reparses a master from package bytes, so a second mutable detached master cache would fail to affect render/inheritance until write-back. Add private MasterRecord { id:Option<u32>, part_name:String, master:CT_SlideMaster, dirty:bool } and ThemeRecord { part_name:String, theme:CT_OfficeStyleSheet, dirty:bool } in that existing file. Extend existing LayoutRecord with dirty:bool. Master ID is the existing optional presentation master-list ID (rpptx-oxml/src/presentation.rs:85/:87), not an invented new serialized identifier. Preserve imported absence on reads and value edits. Only insertion allocates a fresh schema-valid ID before commit. Theme records are unique by canonical existing package part identity, because multiple masters may share one theme. Resolve layout-to-master and master-to-theme via actual part relationships instead of storing duplicate mutable indexes. If X208 already supplies the theme record/provider, extend that exact implementation once and do not introduce another record collection.

Loader uses presentation master order, relationship type/internal checks, and the actual master layout-ID-list order. Existing resolve_layouts at6471 scans master relationship order, not typed sldLayoutIdLst ownership. Preserve the current public flat ordering until an explicit reviewed migration if source behavior depends on it, while master.slide_layouts exposes its actual master list order. Detect duplicate/dangling IDs, repeated relationships and cross-master mismatch with named errors before mutation.

Use one concrete owner discriminator in the existing native file: ShapeOwner::Slide(usize), Layout(usize), Master(usize). Two new actual consumers, master/layout authoring, justify extending the existing slide authoring route. Change ShapesMut at7193 from slide_index to this checked owner plus existing group path. Keep current slide constructors/API signatures delegating to that actual route. Proposed additive entry point shapes_mut_at(&mut self, owner:ShapeOwner)->Result<ShapesMut<'_>> and shape_at(&self,owner:ShapeOwner,path:&[usize])->Result<ShapeRef<'_>> / shape_mut_at(&mut self,owner:ShapeOwner,path:&[usize])->Result<ShapeMut<'_>>. Missing owner/path is a checked error. These own actual lookup, dirty/source comparison and validation work, not a forwarding facade layer.

Reuse the actual claim provider representation. The integrated F-X206 provider already uses ShapeMutOwner::Presentation with the owning Presentation, slide selection and group path, or its existing Child variant. Extend that existing selection to ShapeOwner for slide, layout and master scopes. The donor-era direct-child-borrow description does not describe the claimed provider. Do not add a second mutable representation or redundant owner state. ShapesMut::append at7284 selects owner tree and uses existing append_new_member, retaining per-part shape-ID allocation and group geometry. IDs are unique within the owning part, not globally across the deck. Layout cloning into a fresh part can retain its internally unique IDs and connector endpoints, since they occupy a new scope. Duplicating children within an existing part allocates fresh IDs and remaps local connector references. Never indiscriminately renumber a new part's opaque raw IDs.

Part-aware picture insertion must use owning master/layout/slide relationships, existing MediaStore byte deduplication, and owner-aware shape tree selection, not add_picture's slide-only append_picture at3385. Extend the real implementation, stage relationships and child together, then publish. X205's set_picture_fill and qualified FillOwnerLocation reuse this SAME owner discriminator and checked shape/cell paths. Gradient and background properties reuse X205 fill classes. The borrowed ShapeMut needs no picture relationship setter that guesses a slide.

Minimal read accessors: master_count(&self)->usize, master(&self,index:usize)->Option<&CT_SlideMaster>, layout(&self,index:usize)->Option<&CT_SlideLayout>, master_layout_indices(&self,index:usize)->Result<Vec<usize>>, layout_master_index(&self,index:usize)->Result<usize>, layout_used_by_slides(&self,index:usize)->Result<Vec<usize>>. Expose no public private-record fields or mirror MasterRef wrapper that only forwards. Return ordered owned index facts where a collection must join existing flat APIs.

## 2. Write-back and current render input

CT_SlideMaster and CT_SlideLayout already preserve raw_attributes/raw_children at crates/rpptx-oxml/src/slide_parts.rs:213/:226. CT_OfficeStyleSheet and theme elements retain raw attributes/children at crates/oxml-drawing/src/theme.rs:349/:358. Clone these actual models and edit named fields, preserving wrappers, schema order and opaque bytes. Master text styles are existing CT_MasterTextStyles title/body/other at slide_parts.rs:161. Use their existing CT_TextListStyle levels, checked1..5 body level addressing, integrated X197 colour and X205 text/font/bullet editing. Do not implement another theme colour conversion or generic style engine.

staged_package at lib.rs:1336 must persist actually modified master/layout/theme models before returning save/render packages. Current render_input uses staged_package(false) at1109, so all existing inheritance/resource consumers must receive the edited owners, including layout_and_master and current_part_xml. Never render from original package bytes after a live owner edit. Dirty means eligible comparison/write-back. An unchanged cloned model or absent-clear operation must preserve the original part bytes rather than force normalization. Exact original bytes for unaffected parts are required for selective apply_theme.

The existing commit_candidate at2553 intentionally serializes staged_package(false), writes a temporary package and reparses before replacing self. Preserve its failure-before-publication transaction discipline, but X209 theme/layout lifecycle operations must prepare with preserve_unchanged_modelled_parts=true so unrelated slides/layouts are not reserialized simply for importing a theme. Implement that concrete publication within the operation or explicitly refine the existing transaction helper with reviewed preservation semantics for its real consumers. Do not add a forwarding-only transaction wrapper. Verify byte preservation for existing opaque slide XML, relationships, properties and unrelated parts, not just typed equality.

## 3. Live Python owners and lifecycle

Extend existing slide.py/slide.rs classes and existing stubs. PySlideLayout presently stores index+ContentPath at crates/rpptx-py/src/slide.rs:25 and validate_path at33. X206's dedicated layout inventory generation must remain the provider until X211's reviewed stable owner identity contract replaces/coalesces it. Do not quietly invent a parallel master/layout revision map in this feature.

Master and theme proxies bind canonical owning part identity in the same PyPresentation and resolve that identity on each operation. Theme proxy is the actual X208 master-part-bound proxy: resolve its current master-to-theme relationship on every read/write. Two masters sharing one theme see the same edited theme. Changing a shared theme modifies its actual resource and all sharers by design. Theme colour/font value edits preserve all owner, shape and text generations. Removing or replacing the master makes its old proxy loudly stale. Reusing a freed part filename must not resurrect a removed owner under X211. Until stable IDs are integrated, success-only master/layout inventory generations give conservative loud invalidation, with explicit error naming the first invalidating operation and retrieval expression.

Collection remains live. Property edits rename/background/showMasterSp/text styles/theme retain existing owner inventory generations. Existing-layout add_slide does not stale layouts, already X206 acceptance. Layout removal/import/reorder updates the existing dedicated layout generation once after success, never on refusal. Appending a new master/layout currently follows that conservative generation contract. X211 later requires unrelated owner handles to remain stable using its single reviewed logical-owner provider. Write causal mutation hooks now with actual operation names so X211 can refine identity without duplicate counters or incompatible proxy types.

Layout name setter changes cSld.name only. Follow-master-background=true removes explicit local background, false materializes current effective background from its actual master with owner-aware image relation closure. Missing effective background becomes the documented existing native no-background/default result, not an arbitrary white fill. show_master_shapes edits existing optional attribute and None clears if exposed as Optional. Master layout usage is actual internal slide-layout relationships, not name matching or a cached index list.

Used-layout delete validates full current package ownership graph first. Refuse if ANY slide relationship uses that part, including hidden slides and relationships outside the cached admitted model. Refuse ambiguous foreign/raw master-list routes or unsupported inbound dependencies. For unused layout, stage removing the exact owning master ID-list entry, corresponding master relationship, layout part/rels/content-type override and flat record. Prune closure only if provably unreferenced from remaining owners. Shared images/themes survive. Publish one reopen-validated candidate. No revision, collection change or media deletion on error.

Layout duplication stages a new collision-free part, exact cloned model, remapped internal relationship targets and fresh master layout-list ID, adds that relationship/list entry atomically, then returns a new captured handle. Retain same owning master, source appearance and opaque content. External hyperlinks retain exact targets without network access. Unsupported internal payload closure refuses rather than dropping it. Duplicating an unused name is allowed, identity never equals name. Cross-master duplication is not implicitly performed by the default collection API.

Concrete low-level gap: master sldLayoutIdLst is currently raw, recognized at slide_parts.rs:279 and emitted at794. Add the smallest typed layout-ID-list editing operation in that existing model/parser/writer, retaining unknown list/entry attributes and children and schema position. Do not string-replace raw IDs or generate a parallel whole-master XML serializer. Required checked native public lifecycle operations in existing lib.rs: set_layout_name(&mut self,index:usize,name:Option<&str>)->Result<()>, duplicate_layout(&mut self,index:usize)->Result<usize>, remove_layout(&mut self,index:usize)->Result<()>. Existing native set_slide_layout at2054 remains the provider for explicit remapping.

## 4. Exact selective apply_theme contract

Proposed native signature in existing lib.rs:
apply_theme(&mut self, source:&OpcPackage, source_master:Option<usize>, import_masters:bool, layout_map:&[(usize,usize)])->Result<()>

Python: apply_theme(source, *, source_master=None, import_masters=False, layout_map=None)->None. Input path or bytes is loaded as one local OPC source, never executes content or downloads external targets. Map pairs name current destination flat layout index and source selected-master layout index. Mapping keys unique, ranges validated, explicit mapping requires import_masters=True. Empty mapping keeps ALL existing slide-layout relationships. No automatic layout-by-name remap: existing import_slide at2877 has documented first-name match behavior, but that ambiguity is inappropriate for destructive theme adoption.

PPTX/POTX source: validate package main relationship/content type, resolve selected source master and its single internal theme relationship. None selects the sole source master, multiple masters require explicit source_master with a named error. Do not import source slides, notes, handouts, comments, presentation metadata, custom shows, signatures or unrelated deck resources. Main extension alone is not trusted. Source package may contain unrelated executable parts. Ignore those outside selected closure, but refuse if selected import closure depends on executable/embedded/unknown unsafe payload.

THMX source: resolve package-level THEME relationship and theme content type, require one selected theme and well-formed complete colour/font/format schemes. It has no presentation master selection or imported master/layout inventory: source_master must be None, import_masters=False and mapping empty, otherwise explicit error. Do not assume /ppt/theme/theme1.xml or treat a THMX as a Presentation. The existing CT_OfficeStyleSheet parser/writer is the provider, theme content-type constant exists at crates/oxml-opc/src/content_types.rs:23. Source without legal qualifying relationship refuses instead of guessing filename.

Default colours/fonts/format scheme application: for every distinct destination master theme resource, clone latest destination theme and replace ONLY theme_elements.color_scheme/font_scheme/format_scheme with the selected source's full actual models. Preserve destination theme root name/objectDefaults/extraClrScheme/extLst and unrelated unknown XML unless caller explicitly edits name later. The full selected source scheme models retain their own unknown attributes/children. CT_FontCollection at theme.rs:438 includes latin/east_asian/complex_script and supplemental_fonts: do not apply Latin only. CT_StyleMatrix at470 includes fills/lines/effects/background fills and private preserved source: copying only colours/fonts is incomplete.

Import any internal images referenced by the copied format scheme. Compute relationship closure for actual retained source relationship references, validate namespace-qualified r:id/embed/link references including opaque retained source, allocate collision-free destination scope IDs, deduplicate image bytes only when existing content type/bytes agree, rewrite exact qualified references through the existing rewrite_exact_rel_ids provider used at lib.rs:3146 and10135. Do not rewrite arbitrary text or same-spelled foreign attributes. External image relationships are not fetched and cannot silently imply rendered imported appearance. Refuse unsupported external resources when retained imported appearance depends on them. Safe external hyperlink targets may remain exact in imported master/layout shapes without network fetch.

With import_masters=True, copy ONLY selected master, its actual ordered layouts, selected theme and their supported dependency closure into fresh destination part names. Add master ID/relationship and complete legal layout-list links, rewrite layout back-links to the NEW imported master. Existing masters/layouts remain. Default empty layout_map merely imports reusable branding layouts, preserving all current slide links/content while applying selected schemes to existing themes. For a provided map, each destination layout pair means every existing slide currently using that exact old layout is rebound to the chosen imported source layout, using existing checked native semantics. Unmapped layouts/slides retain relationships. Direct slide shapes/text/notes/order/IDs/hidden state/transitions remain byte-identical apart from the deliberately edited slide .rels layout target. Existing placeholders retain authored content and own explicit properties. Changes in inherited geometry/background are intentional appearance of explicit mapping, not an automatic content deletion or rewriting of shape coordinates.

Preflight complete selected closure, content types, internal targets, relationship-reference remapping, mapping and every affected destination owner before publication. Master/layout cycles form a known relationship graph, visited by canonical part identity. Allow understood master/layout/theme/image/hyperlink edges. Required unsupported chart/SmartArt/OLE/media/extension dependency must refuse with owning part+relationship+type diagnostic unless its exact safe copier/preservation route is proved in this feature. Never drop an opaque subtree to avoid importing a dependency. Unrelated destination executable payload remains preserved by existing package-class rules at lib.rs:1003. Selected source executable dependencies are refused, not silently copied. This meets selective no-content-loss by safe refusal where dependency import is not established.

## 5. Required concrete gates before approval/completion

Installed Base reproductions for each of the three Issue312 scripts, every new collection/property/lifecycle operation and both theme import modes. Native/Python tests in existing entrypoints, no new file/module. Test two masters sharing a theme and two distinct themes, layout order vs relationship order, relocated part names, duplicate names, unknown source XML, alias-qualified relationship refs and foreign lookalikes, missing/ambiguous theme/master, used/hidden-slide layout deletion, malformed unknown inbound edges, late copy/reopen failure and ID exhaustion. Error tests compare exact serialized package bytes, relationships, inventory and all affected/unaffected generations before and after. Reinsert-part-name controls must prove no stale owner resurrection under X211.

Positive apply_theme matrix PPTX/POTX/THMX, colours/fonts/four format scheme lists, images in scheme backgrounds, optional imported master/layout closure, no mapping versus explicit partial mapping, preserved destination macros outside scope and refused selected executable dependencies. Do not infer closure correctness from a simple RGB theme. Assert existing direct slide content and all unrelated original part bytes, plus actual requested inherited render changes using deterministic caller-font provider X195. Authenticate current PowerPoint no-repair normal reopen and native PDF/PNG, pinned LibreOffice round-trip and all three installed Python scripts. Preserve limits of Google Slides best effort without an unsupported compatibility claim.

Run exact X205 fill and X206 group/owner/handle regression controls alongside master/layout changes. Fonts/themes consume X208/X197 one source, no Word tint/shade behavior change. Scope riders: qualified OOXML preservation/order, native archive/size actual measurements and source package dry-run, installed PyO3 typing/stubtest, both WASM checks, independent microscope and current scoped hash harness. No baseline move is assumed. The design is approved with existing-file source claims serialized against its providers. Actual prerequisite completion, installed Base reproductions and native oracle acceptance remain implementation/completion gates. No local source or test pass is claimed by this appendix.

## PR327 immutable contribution assessment

PR327 closes Issue312 at3231c0c32ec42be781192fef07cbff52dee4394f. Its
exact ancestor is refreshed PR324 head232b5dbb63a29cfe72462a1716d2b17d79a7d252.
Retained capture /private/tmp/S91-PR327-intake authenticates39 cumulative
changed files and the24-file, seven-commit feature-only delta. Full trees,
base/head blobs, patches and empty paginated discussions are retained.
Do not reimplement or recount inherited F-X208 work under this F-ID.

Reuse concrete owner-aware mutation, dirty master/layout records, live theme
lookup, typed layout lists and staged copy/reopen scaffolds in existing files.
The contributor's new masters.rs is not authorization for a new module.
Source masters.rs:1148 chooses the first master, :1262 replaces existing
masters/layouts and :1215 overwrites complete themes. Those choices conflict
with the explicit selection, fresh import and selective scheme contract above.
Its lib.rs:2723 serializes with preserve_unchanged_modelled_parts=false.
Retain exact unaffected modeled bytes. Cached slide-index usage checks and
unqualified external payload retention cannot replace complete package graph
checks and explicit unsupported-dependency refusals. Index/global-revision
Python proxies must consume F-X206's causal owner provider. Contributor native
and rendering claims remain unverified local acceptance gates.

## Qualified hidden-picture round-trip boundary

The actual pinned LibreOffice save transports master visibility into a qualified cNvPr hidden attribute on retained layout pictures. The existing CT_Picture receives an additive hidden getter over its preserved qualified drawing owner. The existing picture resolver omits only explicit true or 1 owners. Missing and false flags remain visible, foreign or rebound lookalikes do not supply the flag, and retained raw XML is unchanged. This closes the original branded-logo round-trip predicate without expanding unrelated shape kinds or changing existing public struct fields.
