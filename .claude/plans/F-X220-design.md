# F-X220, Embed fonts through both native and public package families

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: F-X195, F-X200, F-X205, F-X210
**Issues**: 360
**PRs**: 361
**Test gate**: regression
**Implementation**: not claimed, no code or execution

## Problem

Issue360 requests the existing native Word embedding through Python and CLI, and equivalent native, Python and CLI PowerPoint embedding. PR361 at a5795f2219d1cde5c613890369df3d88a84fc242 is admitted by the user's bounded intake decision. Native Document::embed_font at crates/rdocx/src/document.rs:29091 checks explicit authorization and identity, sfnt magic and GUID, but requires an existing font record. The Python and CLI surfaces are missing. CT_Presentation preserves embeddedFontLst as raw source at crates/rpptx-oxml/src/presentation.rs:2216, with no corresponding native font-face operation.

The completed F195 contract says caller faces override embedded and fallback faces. Existing Word font input appends caller data after the embedded input at document.rs:31636. New embedding must qualify the actual selection and preserve the completed caller contract across every render path. Donor native application claims, archive numbers and passing reports are not local evidence.

## Spec reference

- docs/hld/03-architecture.md, Native facade conventions and concrete font-table/embedded-face staged ownership.
- docs/hld/04-opc-and-packaging.md, Relationship types, Media and Package integrity.
- docs/hld/06-presentationml-model.md, presentation.xml and Preservation strategy.
- docs/hld/08-rendering-spec.md, deterministic fonts and caller font selection.
- docs/hld/10-bindings-spec.md, Python API shape, Native Word facade stability and checked publication.
- docs/hld/15-build-and-toolchain.md, Deterministic rendering and Packaging.
- docs/hld/14-development-backlog.md, F-X220.
- [Microsoft embeddedFontLst reference](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.presentation.embeddedfontlist?view=openxml-3.0.1), font entries reference package font parts.
- [Microsoft EOT submission](https://www.w3.org/submissions/EOT/), checked header, size, string and font-data representation. Its specification is an input, not proof of native PowerPoint interoperability.

## Admission and sequencing

Formal dependencies are completed. Implementation waits for F204, F215, F216, F217, F218 and F211 to release overlapping facade, binding, CLI, font-manager and HLD carriers. A low-level phase can begin earlier only after an explicit file lease, with its complete contract retained. Release preparation F221 waits for completed F220.

Retained authenticated PR metadata is /private/tmp/S91-intake-admission-360-361/PR361-metadata.json, SHAfdfeecf5ec48321eef7370c224c05bbec76ad2b2cd424ff4ecda18317278e532. Exact41-carrier patch is PR361-diff.patch, SHA051e4909bf077e9fac125381b942899d8922e2c0bca9fbbbae9a64e3829c50b0. Extract the contributor feature once. Do not adopt the whole stack, unrelated version rows, its reported gates, legacy bump-after-mutation Python factories or local-name-only ownership parsing.

## Approach

Expose Word FontEmbeddingLicense(*, authorized: bool, identity: str), FontDefinition and EmbeddedFont read-only snapshots in existing binding carriers. Document.fonts lists the native font-table records and their faces. Expose set_font, remove_font, remove_embedded_font(name, kind) and embed_font(name, font, kind="regular", *, license, font_key=None, subsetted=False). Supported font inputs use existing bounded bytes/path/binary-file input preparation. All four face kinds are accepted explicitly. Omitted key derives a deterministic valid uppercase OOXML GUID from the source bytes. Preserve the existing explicit native GUID API and add a concrete derive_font_key helper, rather than accepting malformed explicit keys.

Add tri-state Word embed_true_type_fonts, embed_system_fonts and save_subset_fonts readers/setters using the actual settings owner. None removes only the owned setting. Preserve alias bindings, unknown attributes/children and xsd:sequence. Native embedding stages a missing bare font-table record and the embed flag with the font part and relationship as one transaction. This intentional native behavior change has its own labelled commit with unchanged ordinary49 sample outputs expected. Removing a last face retains the explicit embedding flag until its setter changes it.

Model PowerPoint embeddedFontLst and the four face relationship ids in existing presentation.rs. Preserve each source entry, its typeface/panose/pitchFamily/charset, list shell, unknown attributes/events/children and unsupported entries. Resolve actual namespace URIs, including alternate/default prefixes and local rebinding, not local names. Expose native concrete EmbeddedFontKind, FontEmbeddingLicense, EmbeddedFont, EmbeddedFontFace/EmbeddedTypeface snapshots, embed_font, embedded_fonts, remove_embedded_font and remove_font in existing rpptx/lib.rs. Native embed_true_type_fonts and save_subset_fonts are tri-state root attributes. License identity is checked input, not fabricated OOXML metadata. No new facade dependency, trait, generic, shared revision engine or public module is needed.

Before designing final writer bytes, inspect a genuine PowerPoint-produced embedded-font package and record its part format, relationship, content type and flags. Include native PowerPoint embedding/save controls when its interface supports them. Qualify uncompressed EOT version0x00020001/flags0 plus embedTrueTypeFonts against real native rendering of an uninstalled caller-owned renamed face. Raw sfnt and missing-flag controls must discriminate fallback. The donor's inability to produce a PowerPoint-saved embedded fixture does not waive Issue360's measured-format requirement. Unsupported compressed MTX and unknown producer formats remain exact through no-op saves and unrelated edits. They are listed with explicit format/absent decoded bytes rather than treated as successfully decoded fonts.

Validate sfnt directory and actual table bounds before indexing or allocating. Preserve font fsType rather than inventing license permission. Explicit authorization and nonempty exact identity are mandatory at every entrypoint, with no permissive defaults. Single-face authoring may reject a collection with a named boundary, but any tightening of existing Word accepted inputs is an independently labelled and reviewed behavior change. Check EOT size/font length/header/string ranges, numTables, duplicate/truncated tables and malformed/XOR input with bounded reads. For PowerPoint, decode each distinct part once and share decoded bytes for repeated references. Preserve the existing Word owned Vec snapshot representation rather than changing a public field type. Add Document::try_fonts() -> Result<Vec<FontDefinition>> for the two actual new binding/CLI consumers, preflighting aggregate snapshot allocation from actual resolved part lengths before calling the common snapshot projection. Errors are explicit, not truncated successful lists. The legacy fonts() signature remains source-compatible, and its existing per-record copy cost is not falsely claimed eliminated. Retain unsupported original bytes. New checked listing and PowerPoint repeated-reference decoding, including hostile declared lengths, must not trigger huge allocation, abort or panic.

Replacement of a face and removal stage package, presentation model, relationships and content types together. Shared targets use copy-on-write or explicit ownership-safe retention. Prune only actually unreachable owned parts and do not drop references held by preserved source. Foreign relationship ids, missing parts, invalid data, allocation failures and unsafe opaque edits refuse before publication. Genuine no-op does not retire existing handles or move revisions.

Bind PowerPoint through existing presentation/lib/shape carriers, exports and stub. Both Python families expose the same named explicit-license policy and four kinds, while retaining their real native result types. Stage native candidate and preallocate snapshots/return values through the completed owner provider before publishing. Do not reuse donor revisions.bump factories. Reads are snapshots with stable byte ownership. Cross-document/retired objects and failed license/data validation leave bytes, revisions and held owners unchanged.

Both CLIs add font list, embed, remove and settings under their existing grammar, with exact help and JSON schema conventions. Require --license-authorized and --license-identity for embedding. Stage output, refuse input/template/font aliases and existing output collisions, preserve exact source input and perform no network font discovery. CLI list returns deterministic font/face order and actual part/format facts. All mutations route through the same native provider.

Presentation PDF, PNG, SVG where supported, slides, notes, handouts, layout and CLI paths load supported embedded faces under their owned typeface and preserve regular/bold/italic selection. Explicit caller faces win matching embedded faces under F195, then existing bundled/system fallbacks apply according to the chosen entrypoint. Keep deterministic font mode and existing no-caller behavior. Reuse the actual font manager and assembly paths, not a second renderer. Any caller-precedence repair is separately labelled with its exact expected delta and genuine distinguishing tests.

Update existing crate READMEs and root skill-consumer guidance with licensed embedding recipes. The README links the separate hadim/rdocx-skills repository, which is not a tracked carrier here. Prepare a concrete downstream skill documentation patch against its existing DOCX/PPTX sources after the API stabilizes, and record the separate review/submission boundary. Do not claim those external skills updated from an rdocx README change, or send an upstream message/PR without authorization. No new tracked font asset or source file is required.

On 2026-10-13 the user approved the documented native Word one-font-per-name limit. [MS-OE376 section2.1.263](https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oe376/22da0d03-7202-4731-9518-b96bfbe4de61) describes one embedded font used for all variations of a font name. Fresh-family and fresh-key pre-save PDFs show genuine regular glyphs with synthetic bold. A separate bold-body control proves the embedded bold font loads. Record native saved empty bold parts truthfully. All four face kinds remain in the API and genuine face selection remains required in our renderer and PowerPoint. Native Word multi-face interoperability is F-X223 in S92, outside S91 close acceptance. This changes only native Word paired-face acceptance.

## Rejected alternatives

- Raw TTF bytes with a .fntdata suffix are not native interoperability evidence.
- Installing the test font on the host hides fallback and cannot prove embedding.
- A Python ZIP editor or a new font/revision engine duplicates native ownership.
- Dropping opaque/MTX entries to simplify authoring loses existing document content.
- Reusing contributor native screenshots or archive numbers evades current-source qualification.

## Test plan

**Test gate**: regression, real supported font inputs and current installed APIs. Missing-method compilation is not RED credit.

| Category | Test | Asserts |
|---|---|---|
| regression | fx220_word_native_python_cli_embed_list_remove | Regular/bold and all four kind mapping, deterministic/explicit key, valid graph and native/Python/CLI read/save/reopen/remove |
| regression | fx220_ppt_native_python_cli_embed_list_remove | EOT bytes, root flags, font-list schema order, actual parts/relationships/content types, list/replace/remove and reopen |
| regression | fx220_authorization_input_and_settings_are_atomic | Explicit license guards, bounded path/bytes/streams, all Word settings/PPT flags, invalid inputs, allocation failure, exact no-op and held owners |
| round-trip | fx220_corpus_embedded_fonts_survive_unrelated_edits | Required actual Word and PowerPoint font-bearing corpus, exact list XML/font parts/relationships/content types, MTX/unknown retained |
| regression | fx220_aliases_shared_parts_and_hostile_lengths | URI aliases/rebindings/foreign lookalikes, shared target COW/removal reachability, repeated2000 references, truncated/max-size EOT and sfnt, no panic/abort/unbounded copies |
| differential | fx220_native_uninstalled_regular_bold_faces_are_used | Real app/build and absent host family. PowerPoint must use genuine regular and bold faces. Word qualifies actual embedded face use within its documented one-font-per-name limit, with synthetic bold and native save changes recorded. Retain raw-format/missing-flag/changed-font negatives, normal save/reopen and original/output provenance |
| golden | fx220_embedded_and_caller_fonts_share_all_render_paths | Own rendering and installed/CLI slide/notes/handout routes use real faces, caller override remains F195, bold/italic distinctions, deterministic fallback |
| docs | fx220_existing_skill_consumer_recipes_match_help | Existing READMEs, typed examples and separately recorded downstream skill patch match current APIs and actual CLI help |

Run changed native/binding/CLI all-target checks including default features, meaningful tests, Clippy, both WASM graphs, formatting and all49 hashes. Build fresh paired wheels, authenticate every member/RECORD/native binary, install from exact current source under the CP lease, collect the exact accepted node set and run strict mypy/stubtest. Qualified source-built and required corpus controls remain separate. Record real native input/output hashes, application version and proof the test face was not installed. Native producer resave that removes embeddings is reported exactly, not relabelled preservation.

Actual affected publishable archives must verify against local patches, exact inventories and10MiB bounds. Update measured README tuples only from those archives, preserving previous families and all unrelated policy logic. Existing product test entrypoints only. No new module/file/crate/dependency/feature flag/binary asset is authorized by this plan.

## HLD impact

- docs/hld/03-architecture.md
- docs/hld/04-opc-and-packaging.md
- docs/hld/06-presentationml-model.md
- docs/hld/08-rendering-spec.md
- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md
- docs/hld/14-development-backlog.md
- docs/hld/15-build-and-toolchain.md

## Risk routing

Parser/serializer: HLD04/06, actual namespace ownership, schema order, byte-exact unsupported/opaque font graphs and hostile lengths. Layout/fonts: HLD08/15, deterministic baselines, actual embedding/face selection and licensed existing inputs. Published API: HLD10/structural rules, minor pre1.0 compatibility notes, verified affected archives and10MiB ceiling. PyO3/WASM: fresh paired provenance, exact installed tests, strict typing/stubs and both WASM graphs. External oracle: canonical differential-testing, pinned real app/build, source/output/font identities and truthful metrics. Graph: HLD03, no low-level facade dependency and no new runtime dependency. All normal scoped checks remain mandatory.

## Hash harness

Existing49 outputs are expected unchanged for ordinary documents. Authored font test inputs change only their own qualified outputs. Any delta in an existing sample needs an attributed labelled behavior commit and ROOT-exclusive reviewed baseline disposition. Do not re-record against system fonts.

## Implementation checklist

- [x] Complete finite donor extraction and actual producer-format qualification.
- [x] Capture valid native/installed Base and font/relationship inventories.
- [x] Implement namespace-safe model, native transactions/settings and bounded font decoding.
- [x] Adapt existing bindings/CLI and all render paths with checked publication.
- [x] Qualify all named native, installed, corpus, Office and hostile controls.
- [x] Measure affected archives, exact metadata and required policy gates.
- [x] Obtain clean microscope, update listed HLD files and prepare handoff.

## Open questions

No blocking product question remains. The user explicitly folded Issue360/PR361 into S91. Existing license policy stays explicit caller authorization, unsupported formats are retained, and no new assets or dependencies are assumed. The downstream skill documentation is a separate external submission boundary and cannot be falsely claimed landed.
