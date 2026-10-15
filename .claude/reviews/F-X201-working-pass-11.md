# F-X201 whole working implementation, pass 11

Reviewer: fonts_resume, independent of this F201 implementation.

**Reviewed:** immutable final-authoring-snapshot-r1, all28 changed existing carriers, including22 source/test/API carriers and6 HLD files. Complete diff has5858 added and716 removed lines. **Verdict: 1 defect, 0 smells, 0 nitpicks.** This first full feature review incorporates the earlier bounded independent source reviews. It is not feature completion or a new runtime qualification.

## Authentication and scope

Capsule `/private/tmp/S91-F-X201-final-authoring-snapshot-r1/receipt.json` SHA-256 `02f128f28dbaad001f9e9b751ed607e03f76cd4f9584c8e6919d0425c5fb4716`. Source map `32d07962a2ece54d03723d30413bc254ab2740231f9d03eb3dcbbf0bdfeec013` has3076 records. Complete working diff `beb77c79108542c57af4f4852e19ccbe00f728a494fe90353df6a6db8b4cc04a`. All45 manifest file references authenticated with zero mismatches. Captured worker HEAD `2d2f4b8f133e3a4c1a097154d39119552d8050ec`, indexSHA `3c6e655bc171159b300e85d478bd5ad879ba4998d89e67b5ce188bc992c4f518` are snapshot authority, not a claim about subsequently advancing worker state.

Reviewed the approved F-X201 design, including Issue304, PR328 advanced-head style contracts, PR343 physical story-picture reuse and DrawingML output policy, plus the six affected HLD contracts. Earlier focused passes1 through5 establish the independently inspected geometry/even-story, rich owner, stub, note-policy/nested-table and page-colour slices. This pass reads the final full diff and new seams, without relabeling historical runtime as current acceptance. Every citation below addresses copied `source/` files under the immutable capsule.

## Defect

### D1. SectionHandle setters falsely advertise nullable assignment

`crates/rdocx-py/python/rdocx/_rdocx.pyi:407`, `crates/rdocx-py/python/rdocx/_rdocx.pyi:443`, `crates/rdocx-py/src/document.rs:840`, `crates/rdocx-py/src/document.rs:998`, `crates/rdocx-py/src/document.rs:3487`.

Nine numeric setter signatures advertise int | None, and orientation advertises str | None. The actual numeric setters pass the supplied object through checked_section_length(Some(value)), whose integer extraction rejects Python None. The orientation setter requires a Rust &str, so None is also rejected. A caller accepted by these stubs therefore fails at the implemented public API. Optional getters correctly represent absent direct source values, but do not imply a clear operation.

The approved contract requires checked integer geometry operands and the existing checked orientation behavior, without a new nullable clearing policy. Make setter types match these implemented operands while retaining optional getter returns, and preserve None/bool/float/overflow refusal and source atomicity controls. The worker confirmed the intended integer-only and string-only setter contract during this review. No runtime or stubtest was run by this reviewer.

## Prior defects and final source disposition

Pass6 rotation D1 is resolved in this snapshot: CT_Anchor.shape_rotation_degrees at drawing.rs477 closes only projection input over retained inherited bindings, verifies the actual wp/graphicData/WPS/xfrm path and reads the unqualified rotation operand. Engine collect_anchored_drawings at11060 carries that fact, and paginator.rs2758 applies rotation about the resolved rectangle centre to actual text paint. Physical raw_xml remains untouched. The meaningful text watermark control checks315-degree versus unchanged real text and alpha, not only authored XML.

Pass6 theme-border D2 is resolved: border set_literal_color at borders.rs57 removes only expanded Word themeColor/themeTint/themeShade attribute keys. The qualified page-edge parser at rdocx-oxml/document.rs1437 admits real Word aliases, retains foreign attributes and complete opened-edge child source, and refuses duplicate recognized edges. Native set_page_border at document.rs4135 updates a copied selected edge after checked operands. The source control at regression_test.rs399566 includes theme selectors and foreign lookalikes.

Pass7 checksum D1 is resolved: image replacement at document.rs22349 retires only the already qualified removed owner and unreferenced relationship before allocating its replacement. The exact source fingerprint and changed-owner refusal remain. Direct picture output at22377 uses ordinary w:drawing rather than an unnecessary compatibility choice with no fallback. Actual repeat control reaches0,0.5,1 alpha, retained rotation, reopening, one live owning image relationship, removal and changed opaque-source refusal. The old failed checksum and native MC-image observations remain historical failures.

Earlier rich-class stub omissions are resolved in the final HeaderFooter and FootnoteEndnote declarations, apart from new D1 above. Existing source-qualified physical paragraph/table resolution, same-document checks, native main-story note host admission, and staged publication are retained. Related cell/nested-table authoring does not borrow main-body ordinals. No donor story module, packed owner registry or relationship-only picture proxy was introduced.

## Six aspects

- Correctness: 0 production findings. New geometry inheritance copies only resolved geometry, while story creation and even settings share a staged candidate. Rich related owners resolve latest physical source before typed edits. Watermark replacement and removal affect only qualified unchanged owners and preserve unrelated header content. Source-qualified image effect paths preserve ordinary no-effect rendering.
- Contract: D1. Zero additional findings. The final implementation includes live section and all six story-slot handles, ordinary rich paragraph/run/table/cell classes, checked note creation/deletion and policy, page/column breaks, page colour/display setting, per-edge page borders, and real DrawingML text/image watermarks. Legacy VML APIs remain accurately exposed. Incomplete section-size validation remains the shared native finding consumed by F203, with later cross-surface acceptance pending rather than fabricated.
- Panics and errors: 0 findings. Length, note start, style, owner, run-boundary and geometry checks precede publication. Required watermark lengths use Some inputs and successful integer extraction before unwrap. Related-table ambiguity and unsupported source return checked errors. Prepared revision allocation precedes committing candidate owners.
- OOXML: 0 findings. Qualified physical namespace ownership is respected, typed source snapshots close inherited context before parsing, and replacements splice the selected physical owner only. Border foreign attributes and opaque children survive. Text fill uses qualified Word2010 solid fill and alpha, and semantic style merging transports recognized facts without arbitrary extension copying. Actual text persists in both DrawingML and VML fallback representations. Picture alpha/rotation is taken from the qualified picture chain, not foreign local-name matches.
- Tests: D1 is not yet covered by a final fresh installed typing gate. Zero separate test defects. Meaningful geometry/even-story Base controls, alias/foreign/late-failure ownership controls, nested tables, styles, field/image relationship ownership, notes, page source preservation, actual affine text/image paint and checksum repeat/removal controls are present in existing entrypoints. Styled note control compares both streams, cold/warm output and signed explicit baseline shifts. Genuine suite, installed and native oracle obligations remain execution requirements, not source-review credits.
- Structure: 0 findings. All changes stay in existing carriers, with ordinary rich Python classes sharing concrete locations and resolvers. Generic read/edit helpers have multiple actual closure consumers. No new file, module, crate, trait, dependency, flag or speculative extension system appears in this diff.

## HLD and evidence limits

All six HLD candidates describe current intended live-section, owner, page appearance, note-reference and source-preserving behavior. Their additions retain existing provider contracts. Google import stays optional evidence with no compatibility guarantee. The note-reference superscript change is intentional and must retain its separately labeled behavior attribution and measured deterministic delta under normal completion.

Actual final-watermark receipt SHA-256 `172e91b77bcbabb3e90a8ec7771a0d0a5bbf5e8f9f2e50ae78a27750d58c5d9d` authenticates three child0 rows, each exact1PASS0FAIL0IGNORE: qualified picture effects, direct picture repeat, declared Calibri text. Raw logs and generated artifact hashes authenticated. All three owned groups naturally ended and are gone. This does not grant all earlier native rows current-source qualification or a completed whole gate.

ROOT native pass10 recorded the prior MC-choice image being omitted by pinned LibreOffice despite Word paint. The final direct-image source addresses that representation, but new Word/LibreOffice acceptance was pending at review time and is not inferred here. Full installed matrix on rebuilt final wheels, strict typing/stubtest, affected changed-crate/risk gates, deterministic hash attribution, verified archives, metadata/policy closure and final integration/handoff remain separately required. Native host substitution and print-background preferences do not establish deterministic library parity. Source review does not waive the retained source-contour or native acceptance requirements.

Only this external review artifact was written. No worker source, tracked documentation, Cargo, tests, product imports, target or environment mutation occurred. Return for distinct D1 remediation and independent narrow resolution.
