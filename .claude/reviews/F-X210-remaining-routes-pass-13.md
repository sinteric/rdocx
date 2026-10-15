# F-X210, remaining export routes, pass 13

**Reviewed**: actual six-carrier remaining-route delta, 18694 bytes, and its complete surrounding output assembly and diagnostic lifecycle against the approved F-X210 companion policy.

**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Scope and authority

Worker branch is work/f-x210-codex. The source packet is `.claude/scratch/F-X210-remaining-routes-r1` in `/private/tmp/rdocx-f-x210-codex`. Receipt SHA256 is `48e8a7fae6a6ff70a970e771039d5794168096ed7a71470ca4986690f114e83a`. Delta SHA256 is `4ca099f99bf79717744742a499cd854b91b10c84a077c4dc773d42bb154a745c`. Authenticated current2909 byte, size and mode records against source map `cab15d416665fda2da09a56595ea909898a1397cc99c32a658509e071d62ab6c`, its exact six changed carriers and all2903 unchanged records against the formatted predecessor `8cbd9a66ede6e4e76dba54b54df15d12bd500be78c3e1e842f25e60b6bee8f45`. Approved assets are unchanged.

Read approved design, companion policy, current progress and prior independent provider and companion reviews. The reviewer authored earlier companion and HLD proposals. This pass independently audits the separately authored six-carrier remaining-route implementation. Unchanged HLD intent relies on the independent clean HLD pass11. This report does not claim an independent review of the reviewer's own HLD text or a whole-feature completion verdict.

## Defects

### D1, notes and handout surfaces return pre-layout diagnostics and lose their own font facts

`crates/rpptx/src/lib.rs:11001`, `crates/rpptx/src/lib.rs:11008`, `crates/rpptx/src/lib.rs:11020`, `crates/rpptx-render/src/lib.rs:337`.

`render_export_surface` clones resolved slide diagnostics before laying out the notes or handout surface. The layout then records committed font contributions and includes their approximation diagnostics in its LayoutResult, but the helper returns the earlier vector. Notes and handout assembly append that stale vector at `crates/rpptx/src/lib.rs:10385` and `crates/rpptx/src/lib.rs:10567`. Consequently, a presentation with only Calibri main-slide text and Aptos speaker-note text or Aptos handout-master text successfully returns rendered payload with no Aptos substitution fact from that surface. The new diagnostic companions expose this incomplete vector at `crates/rpptx/src/lib.rs:1279` and `crates/rpptx/src/lib.rs:1308`.

The new native fixture already has a contributing Aptos main-slide run at `crates/rpptx/src/lib.rs:13830`, before it introduces the Aptos notes and handout text. The initial main-slide diagnostics copied at `crates/rpptx/src/lib.rs:10345` and `crates/rpptx/src/lib.rs:10548` can satisfy the new message assertions without proving surface attribution. The installed fixture likewise puts Aptos in both its slide and notes at `crates/rpptx-py/tests/test_documented_examples.py:6876` through `_fx195_presentation` at `crates/rpptx-py/tests/test_documented_examples.py:5497`.

Return the actual surface-layout facts and add notes-only and handout-only request discriminators. Preserve the approved stable first-use deduplication across preview and export surfaces. Simply appending every surface vector would duplicate the same font warning because each surface begins a new layout lifecycle at `crates/rpptx-render/src/lib.rs:311`, clearing manager-local deduplication at `crates/oxml-layout/src/font.rs:1013`. Non-font paint warnings must retain their existing multiplicity and order. This is a source-backed finding, not an executed failure.

### D2, the timeline output never receives committed font approximation facts

`crates/rpptx/src/lib.rs:4977`, `crates/rpptx/src/lib.rs:5025`, `crates/rpptx/src/lib.rs:5037`, `crates/rpptx-render/src/timeline.rs:43`.

The timeline route shapes its incoming and optional outgoing content with the chosen font manager and lowers the actual timeline page. The manager records final Aptos substitutions, but timeline composition builds its diagnostic vector only from resolved-slide, timing-state, media and transition diagnostics. It never projects the committed font-warning sink. The timeline layout hook returns only a PageFrame, so those warnings have no other path into the returned frame.

A timeline with a visible Aptos run resolved to caller-labelled Carlito therefore returns no Aptos approximation diagnostic. The newly added native assertion at `crates/rpptx/src/lib.rs:13924` requires precisely that message, but this source has no producer-to-output route that could provide it. Project the timeline's actual accepted font facts from the same layout operation, preserving requested identity, stable first-use deduplication, outgoing/incoming contribution ownership and existing non-font diagnostics. Do not copy unrelated deck warnings or perform a second render. This obligation follows the output-owned same-pass policy at `.claude/plans/F-X210-design.md:33` and the new route's own assertion. No runtime reproduction ran during this review.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Six aspects

- Correctness: D1 and D2. No additional payload-selection, indexing or encoder logic finding.
- Contract: D1 and D2 violate output-owned same-pass diagnostic projection. Other new archival, notes and handout companion shapes match the approved additive policy.
- Panics: 0 findings in the new production paths. Existing Result propagation precedes returned payload and diagnostic allocation. Test-only unwraps have explicit fixture construction.
- OOXML: 0 separate findings. The handout fixture uses a typed master, required content type and related theme, and authentic notes runs. Render methods use staged package copies and do not mutate source owners or stored font codes.
- Tests: 0 separate findings beyond the masking fixture identified under D1 and the uncovered route exercised under D2. Word's archival counter and payload equality controls use the actual existing counter. Python and native error/source-atomicity checks retain real existing behavior. New assertions are unrun and earn no acceptance credit.
- Structure: 0 findings. Six existing carriers contain the implementation, with no new file, module, trait, generic or dependency. Native methods perform actual layout and encoding, and Python methods detach that work before creating owned bytes, lists and message tuples.

## Other audited boundaries

Word archival PDF uses one caller-configured deterministic layout, checked archival encoding and that bundle's diagnostics. PPT archival uses the existing one prepared slide assembly. Python notes defaults and keyword-only font intake match existing byte-only methods, retaining DPI150 and tuple[str,...] diagnostics. The stubs and typing controls match concrete method signatures. Source and payload equality are required before any new runtime credit. The old byte methods and their returns remain unchanged.

## Limits

No Cargo, product import, installed runtime, compiler, formatter, environment, target, source or native UI mutation occurred. Prior actual provider and companion GREEN remains historical unchanged-source evidence. The six new route changes require corrected source, independent review, current checks and actual native and installed controls. Licensed-font painted-CID acceptance, full scoped riders, deterministic hash, packages and feature completion are separate obligations. Only this canonical review artifact was written. Review phase ends here.
