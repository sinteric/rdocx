# F-X218, Export presentations as semantic HTML and Markdown

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: F-X205, F-X206
**Issues**: 352, 353
**Test gate**: regression
**Implementation**: Native semantic projection, Python and CLI controls, independent validators and both native PowerPoint source/save/reopen cycles are qualified. Scoped successors resolve the original required corpus and policy setup failures. The behavior-preserving shared image-validator relocation, completion intent and corrected current artifact composition have independent zero-finding reviews. Fresh current PPT wheel/install provenance, typing and stubs, the measured current archive, both current WASM graphs, changed-crate lint and MSRV supplement complete scoped acceptance. Worker preparation keeps integration and final sprint verification separate.

## Problem

Current `rpptx-cli/src/commands.rs:703` emits an outline by iterating slides in source order, extracting title first, recursively walking shapes at752, table origin cells row-major and paragraph levels at776. It loses inline formatting/link identity/table boundaries and pictures. Its text normalization at788 trims and collapses line breaks. Semantic export needs that existing reading order while retaining actual source structure and content, not a PNG per slide.

Native SlideRef hidden/title/notes accessors at rpptx/lib.rs7828/7864/7903, TableRef at8323 and ShapeRef table/children at9853/9874 supply much of the projection. There are no current to_html/to_markdown export methods in the inspected native facade. Issue353 explicitly requests semantic HTML, not pixel layout parity. Issue352 requires alt text, notes options and hidden-slide filtering.

## Spec reference

- docs/hld/03-architecture.md, dependency rule and native media/export dependency edges.

- docs/hld/02-scope-and-non-goals.md, presentation text/outline and HTML capability boundaries.
- docs/hld/06-presentationml-model.md, The shape tree, z-order and actual nonvisual picture/source metadata.
- docs/hld/04-opc-and-packaging.md, owning relationship/resource lookup and HTML resource safety.
- docs/hld/10-bindings-spec.md, Python API shape and native HTML presentation contract at1603.
- docs/hld/12-testing-strategy.md, The deck corpus and Binding tests.
- docs/hld/14-development-backlog.md, F-X218 at7587, exact L/dependencies/regression gate.

## Approach

Implement the exporter in existing rpptx/src/html.rs and lib.rs reexports. Reuse existing shape/title/notes/table/source relationship owners and preserve outline traversal order. No low-level family dependency, new module/file, registry or polymorphic document framework. Do not convert the presentation into an invented Word CT_Document just to borrow its emitter. Small concrete existing escaping/resource routines may be reused where they already have real callers.

Add concrete `PresentationTextExportOptions { include_notes: bool, skip_hidden: bool, inline_images: bool }` and `PresentationTextExport { text: String, assets: Vec<(String, Vec<u8>)>, diagnostics: Vec<String> }` in an existing carrier. Direct constructors/defaults suffice, no builder. Native `to_html_with_options(&options) -> Result<PresentationTextExport>` and `to_markdown_with_options(&options) -> Result<PresentationTextExport>` read without mutation. Convenience to_html/to_markdown use include_notes=false, skip_hidden=false, inline_images=true and return the same concrete result so detached assets/diagnostics cannot be silently lost. Native save methods stage all selected resource files and document atomically. The export result is not a live mutable slide handle.

One semantic section per included slide, source slide number/ID retained in HTML metadata. A title becomes the section heading, absent/empty title uses deterministic Slide N and an explicit fallback classification. Emit the title only once. Body shapes follow the existing outline source order and recurse through groups and selected compatibility branches. Do not sort by coordinates or depth, or move tables after text. Table origin cells remain row-major, merged spanned cells do not duplicate content. HTML represents actual rowspan/colspan. Markdown uses pipe tables with escaped pipes/newlines, with an explicit diagnostic for span layout that a pipe table cannot represent while preserving each origin's text once. No text or alt payload is silently dropped.

Paragraphs retain their source levels as nested lists with safe hierarchy closure. Ordinary paragraphs remain paragraphs instead of being turned into bullets solely because outline historically prints every item with a hyphen. Use existing bullet/number choice and preserve displayed ordered labels where supported by actual source semantics. Preserve bold/italic and hyperlinks from actual typed run properties and owned relationship scope, including explicit false/inherited formatting. Retain soft/hard source breaks with a declared Markdown canonical form. Quote/backtick/bracket/pipe/metacharacter escaping prevents source text becoming unintended markup. Hyperlinks use safe external URLs or supported internal slide anchors, with qualified foreign lookalike controls. Do not resolve links by display text. Unsupported run content, fields, charts/SmartArt/OLE picture projections and decorations get precise diagnostics or existing valid fallback content without inventing pixel or data projection.

Pictures retain actual nonvisual title/description alt text and use the owning image relation's bytes/content type. Resolve grouped/nested picture owners, aliases, local rebound relationship prefixes and inherited metadata correctly. Preserve duplicate picture occurrences in source order but deduplicate identical resource bytes when writing adjacent assets. HTML embeds data URIs by default or writes safe relative adjacent assets when requested. Markdown supports an image destination with alt text, using embedded or detached resources under the same options. Names are deterministic, collision-safe and never user-controlled path traversal. Unsupported/invalid media is diagnosed before output publication, no network fetching. Export does not change the source package, media relationships or native owner generations.

With include_notes, add a labelled notes subsection after that slide's body content. With skip_hidden, omit hidden slides and their assets/notes without renumbering the remaining source identity. Links to omitted slides are classified and diagnosed, not silently retargeted. Preserve original ordering of every remaining block. Empty deck, empty slide, no title, whitespace-only notes and tables with empty cells are deliberate supported cases.

Python `Presentation.to_html(*, include_notes=False, skip_hidden=False, inline_images=True) -> str` and `.to_markdown(...) -> str` default to embedded resources. When inline_images=false, use explicit `save_html(path, *, include_notes=False, skip_hidden=False, inline_images=False) -> tuple[str, ...]` and save_markdown counterpart so adjacent assets are actually written, not discarded. Pure string methods reject detached-image mode with an actionable message directing to save or an explicit export-result companion. Stubs declare all names, keyword defaults and diagnostics. These detached read/export operations do not advance revisions or stale current owners. Foreign/bad option types refuse before output. No repeated rendering to collect text.

CLI extends existing `rpptx convert --to html` and `--to md` with --notes, --skip-hidden and --embed-images options scoped to these semantic formats. HTML defaults to embedded pictures. Explicit adjacent mode publishes safe assets beside the output, preserving input/output no-overwrite rules and atomic staged publication. Default text order and included slides match the existing outline. Rendering flags such as font_dir/DPI cannot accidentally select a pixel export or perform hidden font discovery for semantic output. Either reject irrelevant options as current commands do or explicitly document their no-effect behavior before implementation. Never silently hide a conflicting option.

Existing carriers: rpptx html.rs/lib.rs and narrowly missing actual picture/run metadata projection if necessary, rpptx-py presentation.rs/lib.rs current __init__.py/_rpptx.pyi/typing/Python entrypoints, rpptx-cli main.rs/commands.rs and current native/integration tests. Existing F205 owns rich text/bullet/fill/export aliases and F206 owns causal authoring/native metadata. This story consumes their completed actual sources rather than duplicating their mutations. No new product file/module/crate/dependency is proposed.

## Rejected alternatives

- Slide images are already a separate format and discard semantic text.
- Spatial sorting differs from the explicitly requested outline order.
- One plain-text paragraph per shape loses inline formatting, levels, table structure and alt text.
- Recovering output expectations from the product's generated HTML is circular.
- Treating a forgiving HTML parser's successful DOM as proof of valid HTML hides recovered errors.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| regression | `fx218_corpus_markdown_preserves_every_semantic_construct` | Pinned corpus source titles/nested bullets/bold/italic/links/table/picture alt/notes against independently authored expected Markdown, exact reading order |
| regression | `fx218_corpus_html_is_valid_and_semantic` | Independent expected HTML structure, one section/heading per included slide, real tables/spans, links/pictures/notes and no parser recovery errors |
| unit | `fx218_nested_order_title_and_table_origins` | Mixed group/compatibility shape order, title once, merged table origins, no spatial reorder or repeated text |
| unit | `fx218_inline_escaping_and_qualified_image_links` | Literal markup escapes, run formatting and inherited false, aliases/rebindings/foreign IDs, safe image MIME and alt text, no script/url/path injection |
| round-trip | `fx218_export_source_and_resources_are_unchanged` | Source package and owner equality before/after, deterministic repeated outputs/assets, asset reuse without omitted occurrences |
| regression | `fx218_notes_hidden_assets_and_internal_links` | All options independently and jointly, original slide identity/order retained, skipped assets and omitted-target diagnostic, empty/untitled deck |
| regression | `fx218_installed_and_cli_semantic_exports` | Actual fresh Python Rust-equivalent text/options and stubs, CLI html/md with embedded/adjacent resources, atomic output and input bytes unchanged |

Construct small complete source-built decks inline in existing test carriers. Pin corpus deck hashes before comparing expected source-owned semantic records, not output-derived fixtures. Retain the existing expected-file testing pattern as an inline expected text when new product files are forbidden. Validate HTML with a pinned HTML conformance check or the existing html5ever stack's actual parse-error reporting plus explicit structure constraints, never merely parse-and-recover. Record validator/version and malformed nesting negative controls. Parse exported Markdown with a declared independent CommonMark/GFM oracle as test infrastructure, not a new product parser dependency. Core expected textual/cell order and asset semantics remain exact.

Native PowerPoint qualification reads the actual source deck title/body/nesting/table/alt/notes/hidden flags and saves/reopens without mutation or repair, with app/build and source/output identities recorded. This binds the semantic authority. HTML is checked in its own HTML validator, not presented as an Office document that PowerPoint should open. No pixel-exact promise, arbitrary screenshot threshold or font baseline is introduced. Fresh PPT wheel/install provenance, exact selected installed nodes, mypy/stubtest and actual CLI tests must run against the same reviewed sources. Missing required corpus/authority/validator is uncredited, never a zero-test pass.

## HLD impact

- docs/hld/03-architecture.md

- docs/hld/02-scope-and-non-goals.md
- docs/hld/04-opc-and-packaging.md
- docs/hld/06-presentationml-model.md
- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md
- docs/hld/14-development-backlog.md

## Risk routing

Parser/serializer04/06: correct source/namespace relationship lookup and HTML/Markdown escaping, table order and opaque source preservation. Public API10: additive semantic export surface, existing text/outline unchanged, verified affected archives/10MiB. Bindings10: fresh PPT wheel/type/stub controls and both WASM graphs. External oracle: pinned HTML/CommonMark validation and actual PowerPoint semantic source evidence. Dependency03: bounded semantic media validation reuses existing pinned workspace tiny-skia, gif, zune-jpeg and tiff as unconditional native facade edges. PNG/JPEG full decode and all GIF frames/TIFF pages are bounded before allocation, and BMP physical extents are checked. Root explicitly admits these existing dependency edges and current manifest/lock carriers. No new third-party crate/version or cross-family helper is added. Nodefault library, MSRV, both WASM graphs and verified affected archive checks cover the broader native dependency graph. No layout/shaping algorithm, unit rounding, font asset, feature flag or new product module is proposed. Normal scoped changed-crate tests/lint/format/hash/policy remain required.

## Hash harness

Existing deterministic49 outputs expected unchanged. New semantic export inputs create only new test outputs. A changed existing CLI outline/reading order is not authorized and would require a separately reviewed explicit contract amendment. No current render baseline or measured HTML acceptance is claimed.

## Implementation checklist

- [x] Approve exact read/export/options contract and wait for F205/F206 done.
- [x] Preserve genuine Base valid source setup before missing semantic behavior controls.
- [x] Implement actual native semantic projection and safe resource publication.
- [x] Expose Python/CLI with complete options, stubs and read-only source guarantees.
- [x] Complete corpus expected outputs, validation, native semantic authority and installed/CLI controls.
- [x] Run scoped/WASM/package/hash/policy gates, update only listed HLD and obtain clean working review.

## Open questions

No blocking product ambiguity. Semantic source-order export, included notes opt-in, skip_hidden opt-in, embedded images default and adjacent resources through save methods are stated batch-approval defaults. Supported table text remains exact even when Markdown cannot encode merge geometry, with an explicit span diagnostic. F205/F206 completed before this feature was claimed. No blocking product question remains. Native source qualification is complete. Current source composition and final independent review are qualified. Worker preparation and integrated sprint verification remain separate workflow steps.

## Batch approval, 2026-10-11

The rolling intake instruction authorizes the full linked issue acceptance.
ROOT reviewed the source, dependencies, test gate, risk riders and HLD list.
The stated defaults are adopted. No blocking product question remains.
Completed dependencies and exclusive source leases admit implementation. Native Office qualification remains subject to its exclusive resource wave.

## Contributor PR358 intake, 2026-10-11

[PR358](https://github.com/tensorbee/rdocx/pull/358) at head 649fa8ed6be75624ea5c1c82433c5a02c935fa89 joins existing approved F-X218 for Issues 352 and 353, semantic Markdown and HTML export. Its exact captured patch is dcf904ae0f6001e4ebfdbe83f28421394604e5fb4b62f728715efc0314dbbb45 at /private/tmp/S91-PR358-intake/diff.patch. Read-only contribution assessment is complete at /private/tmp/S91-PR358-intake/assessment.md, SHA 6fd0fb900c70ad64a58d099166d5937741ccb26f2fd0e55f1c5815c36334a346. Reuse source-order traversal, escaping and inline semantic-output tests. Complete inherited bullet/run semantics, usable Markdown images, safe embedded/adjacent assets, internal links and diagnostics. Use the approved existing exporter carriers and retain full source-authority and validator acceptance. Reuse qualifying contributor code and tests within the original approved acceptance and existing carriers. Contributor test claims, archive measurements, omitted acceptance and new product files do not replace local verification or grant additional structural approval. Implementation retains its dependency and exclusive source boundaries.
