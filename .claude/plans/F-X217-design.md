# F-X217, Import HTML and Markdown through public Word surfaces

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: F-X200, F-X203
**Issues**: 350, 351
**Test gate**: regression
**Implementation**: reviewed native, installed Python and CLI import surfaces with qualified scoped gates and native Word acceptance

## Problem

Native `Document::from_html` and open_html in crates/rdocx/src/html.rs524/528 return the document and structured import diagnostics. `Document::insert_html_fragment` at document.rs18881 stages source reconciliation and reopens before commit. Current Python has none of the requested creation/file/paragraph insertion names. CLI convert at rdocx-cli/commands.rs678 always calls Document::open before dispatch, so HTML and Markdown cannot be input.

Outbound `rdocx-html/src/lib.rs:34` has one actual HtmlInput backed by CT_Document/styles/numbering/media and shared body traversal. Its existing markdown.rs emits supported Word paragraphs, tables, formatting and links. No CommonMark parser is present in Cargo.lock or the inspected local Cargo registry. Extending the outbound emitter by regex or a small ad hoc block parser would not satisfy CommonMark nested syntax or the complete issue acceptance.

## Spec reference

- docs/hld/02-scope-and-non-goals.md, Word interchange and bounded HTML capability rows.
- docs/hld/04-opc-and-packaging.md, HTML fragment transaction, caller-owned resources and import publication.
- docs/hld/10-bindings-spec.md, native Word HTML contract at1541, Python API shape and checked owner publication.
- docs/hld/12-testing-strategy.md, Test taxonomy, Binding tests and private from-scratch DOCX corpus.
- docs/hld/03-architecture.md, The dependency rule and Why these seams.
- docs/hld/14-development-backlog.md, F-X217 at7579, exact L/dependencies/regression gate.
- CommonMark0.31.2 block and inline grammar, with GitHub table and strikethrough extensions as named below.

## Approach

Expose existing native HTML operations without a second importer. Python `Document.from_html(html) -> tuple[Document, tuple[HtmlDiagnostic, ...]]`, `Document.open_html(path) -> tuple[Document, tuple[HtmlDiagnostic, ...]]` and `document.insert_html(html, *, after, images=()) -> tuple[HtmlDiagnostic, ...]` return detached structured diagnostic records with location/property/message from the existing HtmlDiagnostic type. after is a required live paragraph of this document, resolved to its actual supported direct-child ContentLocation. Foreign/stale/nonrepresentable owner, resource or parser errors leave OOXML, revisions and handles unchanged. Stage native candidate and Python returned allocation through the completed owner provider before publication. The method does not guess a mutable ordinal from paragraph text. Retain the native supported body/table-cell/header/footer boundary where actually representable by the supplied owner.

Add native Markdown creation/file operations in existing rdocx/src/html.rs and native reexports in lib.rs: `Document::from_markdown(&str) -> Result<HtmlReadResult>`, `open_markdown(path) -> Result<HtmlReadResult>`, and a concrete with-template/resource variant accepting an optional `&Document` template and the existing bounded resource slice. Existing HtmlReadResult/HtmlDiagnostic carriers are reused rather than introducing a second diagnostics model. Python mirrors from_markdown/open_markdown with keyword-only template=None and images=(). Template is an actual loaded Document or explicitly admitted file path at the Python constructor boundary. Clone it, preserve style definitions, headers, footers, section metadata and package resources, replace only its main body content and retain a valid final section boundary. Do not mutate the supplied template or silently append its old sample body. Existing style names win, missing Heading1..6/body/quote/code styles are created through accepted F200 checked style operations with deterministic defaults. Retain the template's numbering graphs and reconcile imported list IDs without collision.

Use a genuine CommonMark event parser, proposed exact dependency `pulldown-cmark = { version = "=0.13.4", default-features = false, features = ["html"] }` in the existing rdocx Cargo.toml and lockfile only. The user approved this exact external parser dependency on2026-10-11. It adds no repository crate. Enable only ENABLE_TABLES and ENABLE_STRIKETHROUGH in parser options. Do not enable smart punctuation, arbitrary extensions or implicit task-list glyphs. CommonMark ordinary syntax includes ATX/setext headings, escapes/entities, hard/soft breaks, nested block containers, reference and inline links/images, variable-length code fences and code spans. Retain source byte offsets for diagnostic location.

Filter raw HTML block/inline events out with explicit ordered diagnostics before generating the parser's escaped HTML projection. Project that bounded HTML through the same existing native HTML importer/fragment assembler, with small necessary extensions in html.rs for semantic quote/code styles and exact nested list context. Retain block structure and inline formatting, do not convert every event into plain paragraph text. No document model other than native CT_Document/HtmlInput is introduced. Extend existing outbound rdocx-html/markdown.rs where necessary to recover the same supported constructs from their actual Word styles/formatting/images, not from a hidden copy of the input Markdown. The round-trip is a canonical CommonMark/GFM representation, not byte-identical source punctuation. Define and test canonical escaping, list indentation, fence length, link normalization and table separator formatting before measuring product output.

Map headings to Heading N and ordinary paragraphs to the template body style, emphasis/strong/strikethrough to actual native run properties, code to explicit code run style, quotes and fenced/indented code blocks to named paragraph styles. Retain nested unordered/ordered list structure and numeric starts through the completed F200 numbering contract, without treating every list as one flat sequence. Tables preserve header and ordered cell contents, alignment where representable and escaped pipe characters. Links become checked native hyperlinks. Images become inline pictures with preserved alt text and recognized content type.

Reuse existing HTML 64MiB, node/depth/image/resource limits, byte sniffing and checked package graph. Markdown parser/event/projection bounds must stop before unbounded allocations. Input decoding is UTF-8. Resource bytes come from data URIs or an explicit caller resource map. No HTTP fetching. CLI can resolve relative local image paths through a bounded explicitly selected input-directory resource loader, refusing traversal outside that root and unsupported schemes with diagnostics. Missing images are diagnosed according to existing import behavior and never invented placeholders without a declared policy.

CLI keeps the current command shape: `rdocx convert input.html --to docx -o out.docx`, and `.md`/`.markdown` input with the same DOCX target. With -o out.docx and omitted --to, recognize these text extensions and select DOCX only for this new input route. Explicit existing DOCX output-format behavior is unchanged. Add `--template TEMPLATE.docx` only to text-to-DOCX input. Diagnostics print through the existing command channel, output is staged/atomic, neither input nor template is overwritten. Invalid combinations refuse before output creation. No pandoc process or hidden compiler.

Existing carriers: rdocx html.rs/document.rs/lib.rs/Cargo.toml, Cargo.lock, rdocx-html lib.rs/markdown.rs/emitter.rs only where accepted outbound round-trip needs it, rdocx-py document.rs/lib.rs existing __init__.py/stub/typing tests, rdocx-cli main.rs/commands.rs and existing integration tests. No new repository product file/module/crate, generic importer or new feature flag. Issue350 mentions optional PowerPoint follow-up, but this story's registered acceptance is Word. Do not duplicate F218 or widen to a PPT HTML binding without an actual admitted need.

## Dependency evidence

The inspected lock contains html5ever through the existing HTML stack, but no pulldown-cmark/comrak. No candidate parser was found under the current local Cargo registry. Primary version-pinned [manifest](https://raw.githubusercontent.com/pulldown-cmark/pulldown-cmark/v0.13.4/pulldown-cmark/Cargo.toml) declares MIT, Rust1.71.1, edition2021 and optional HTML rendering via pulldown-cmark-escape. [Options documentation](https://docs.rs/pulldown-cmark/0.13.4/pulldown_cmark/struct.Options.html) exposes table/strikethrough flags. [CommonMark0.31.2](https://spec.commonmark.org/0.31.2/) is the grammar authority. These support selecting this parser, not claiming it compiles or passes the current graph. At implementation pin checksum/lock, verify actual dependency licences/deny, Rust1.93 MSRV and both WASM graphs. No dependency was fetched or installed during design.

## Rejected alternatives

- A regex or handwritten partial Markdown parser fails nested CommonMark constructs.
- Shelling out to pandoc adds an unowned external runtime and alternate model.
- Storing original Markdown solely to pass round-trip evades real Word projection.
- Automatically fetching image URLs violates bounded explicit resource ownership.
- Replacing the template package discards requested headers, footers and source styles.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| regression | `fx217_installed_html_create_open_insert_has_structure` | Fresh Python from_html/open_html/insert_html and real CLI headings/lists/table/link/image, actual styles/relations/resources, structured diagnostics and correct after owner |
| unit | `fx217_markdown_all_supported_constructs` | Each heading/body/emphasis/strong/code/strike/nested list/table/link/image/quote/code-block construct, CommonMark escapes/reference syntax, checked native OXML and numeric starts |
| round-trip | `fx217_supported_markdown_docx_markdown_canonical_round_trip` | Canonical normalized supported Markdown is equal after DOCX save/reopen and actual existing outbound emitter, no cached input shortcut |
| round-trip | `fx217_template_styles_headers_footers_and_opaque_parts` | Source-built template style metadata, header/footer images/links/fields and section graph survive, body replaced, template source bytes unchanged |
| regression | `fx217_import_diagnostics_and_limits_are_atomic` | Raw HTML diagnostic, missing/unsafe resources, malformed text and limits, foreign/stale owner and allocation failure preserve source/revisions/output absent |
| differential | `fx217_commonmark_tables_and_inline_semantics` | Selected versioned grammar examples and independent parsed-event expectations, table pipe/escape and code fence negatives |
| regression | `fx217_cli_html_markdown_and_template_output` | Both source types, exact DOCX target/default selection, diagnostics, no overwrite, original existing DOCX routes unchanged |

Genuine Base distinguishes current valid native HTML success from missing binding/CLI behavior and missing native Markdown semantics. Missing future API compilation is not RED credit. Native Word must open a source-built combined supported-construct document and template result without repair, normal save/reopen preserves styles/header/footer/images and a native render shows styled structure. Record actual application build and input/output hashes. Fresh installed Word wheel, exact node collection, mypy/stubtest and native changed crates execute against the admitted dependency graph. Required test inputs are source-built in existing carriers, not new tracked binary fixtures. Parse acceptance is not an Office-open or render claim.

## HLD impact

- docs/hld/02-scope-and-non-goals.md
- docs/hld/03-architecture.md
- docs/hld/04-opc-and-packaging.md
- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md
- docs/hld/14-development-backlog.md
- docs/hld/15-build-and-toolchain.md

## Risk routing

Parser/serializer04/06: bounded syntax, exact unmodelled template preservation and schema child order. Dependency graph03: approved exact external parser dependency only, no low-level reverse dependency, locked graph/deny/MSRV. Public API10: additive constructors/insertion and concrete diagnostics, verified affected archives/10MiB. Bindings10: fresh Word wheel/strict typing/stubtest and both WASM graphs. Numbering/units01/08: preserve completed F200 list semantics and checked units, deterministic native render. Oracle: pinned CommonMark/source examples, python-docx1.2.0 surrounding Word structure and actual Word build. No font asset, version carrier or feature flag changes. Normal scoped lint/tests/hash/repository policy remain mandatory.

## Hash harness

Existing sample defaults expected unchanged. Any outbound Markdown/HTML correction needed for the named round-trip is declared as selected construct behavior with source-built controls, never folded into a move. Any measured existing49 delta requires separate attribution, review and labelled commit before baseline change. The scoped49 hash gate is unchanged. The final integrated sprint hash gate remains pending.

The selected behavior delta retains valid imported URL relationships instead of
diagnosing and dropping their targets, gives nested unordered lists distinct
bullet numbering inside ordered lists, preserves supported tables within list
items in their actual body order, and exports the four accepted pictures
in the existing wrapped-content Markdown fixture as data URI images, including
the table cell. The original three regression failures are retained. Their
successors assert actual owned hyperlinks, list kind and depth, exact image bytes
and wrapped/plain equivalence. These are intentional F-X217 behavior changes,
to be named in its labelled implementation commit. The existing49 deterministic
sample outputs are still expected unchanged. No baseline change is authorized.

## Implementation checklist

- [x] Record approved exact parser dependency and canonical round-trip policy, wait for F200/F203 done.
- [x] Preserve real Base controls and expose native HTML surfaces once.
- [x] Implement bounded CommonMark projection, template/resource/diagnostic semantics and outbound coverage.
- [x] Complete installed/CLI/native per-construct and round-trip controls plus Word native open.
- [x] Run dependency/MSRV/WASM/package/hash/policy and zero-finding review, update listed HLD only.

## Open questions

The user approved the exact pinned external parser dependency and its optional HTML renderer dependency on2026-10-11. No blocking question remains. No new repository file/module is requested. Semantic canonical round-trip, bounded explicit images and template-body replacement are stated defaults for batch approval. F200/F203 and the shared CLI owner provider F204 are completed. The reviewed native patch now runs on completed provider prefix38d1697b, including F222. Public consumer and native Word acceptance are qualified for this feature. Final integrated sprint verification and review remain pending.

## Batch approval, 2026-10-11

The rolling intake instruction authorizes the full linked issue acceptance.
ROOT reviewed the source, dependencies, test gate, risk riders and HLD list.
The stated defaults are adopted. No blocking product question remains.
Implementation waits for completed dependencies and an exclusive resource wave.

## Rolling intake, PR359

The 2026-10-11 21:07:23 UTC refresh found [PR359](https://github.com/tensorbee/rdocx/pull/359) at head 7567b4031cc2c247a74ebb61b45cf5ee367f739c. It joins existing approved F-X217 for Issues 350 and 351, Word HTML and Markdown import. Exact patch c6fe3fd750a6ec23391626b161ee6b2a92eff28a244cae904510e6337733ff4f is captured at /private/tmp/S91-PR359-intake/diff.patch. Contribution assessment is complete at /private/tmp/S91-PR359-intake/assessment.md, SHA6fc5a95a1e5a96ff9b114223778a17f435b05842873ae469d732462373638933. The seven new commits are isolated in f217-only.diff, SHAb025943ebe9bbf1882a1957ec01f04ca457a28549f7227e055c13f9f87ea1ef9. Reuse requires the approved exact dependency/features, bounded decoded resources, complete admitted constructs, preserved template graphs and transactional owner publication. The branch is stacked on PR354, whose export/constructor work stays with F-X203 and must be reused once rather than duplicated. The approved exact pulldown-cmark =0.13.4 dependency and original linked-issue acceptance remain controlling. Contributor omissions and reported checks do not replace local acceptance. No source adoption or duplicate story is recorded.

There are 28 open PRs and 38 included issues. Only discussion Issue264 changed in this refresh and remains excluded alongside Issue281. No included issue or existing PR head changed. Exact refresh evidence is /private/tmp/S91-intake-refresh-20261011T210723Z/receipt.json.

## Incremental PR359 formatted whitespace contract

PR359 at e974ea2240691c0de581efa172ba8c88a77c054b corrects native HTML and
Markdown import formatted whitespace ownership. A collapsed space retains
the producing text run properties rather than acquiring the following
span's font, bold, italic, underline, strikethrough or hyperlink. Highlight
follows the actual source mark owner. Only ASCII HTML whitespace collapses.
Nonbreaking spaces and CommonMark code-span internal spaces remain text.
This repartition is an intentional formatting behavior correction, recorded
in its own labelled commit and reviewed independently. Visible text remains
unchanged. Existing semantic highlight, CSS precedence and no-leak controls
remain required with exact reopened per-run formatting facts. The original
14-pass and one-partition-failure receipt remains failed, and its successor
is separate evidence. The bounded proposal is
.claude/scratch/F-X217-start/formatted-whitespace-contract-proposal.json,
SHA d69a2e4121b274c3c2a1ae7ac8b066625d1cb2f8f8f200d730af6412d21fac2f.
ROOT admitted this amendment under the user's explicit advanced-PR intake
instruction. Existing 49 baselines are expected unchanged. Any measured
delta requires separate attribution and review before a baseline change.
PPT HTML import donor changes remain outside this Word story.

## Completion evidence

The source-safe checkpoint `fce4d7bd5d7c172f55b33a706859483e7a7c7356a34501d7910692a8adc74407` qualifies the scoped feature gates and retains original failures with exact passing successors. Whole-feature and supplemental expectation/archive reviews have zero defects and smells. The unchanged49 hash gate and all named risk riders pass. The three corrected regression expectations retain the declared selected behavior delta, not a baseline change.

Native Word16.113.2 build16.113.26092012 opened both actual source-built combined and template inputs without repair. Each was saved as a distinct DOCX and local printing PDF, closed, normally reopened, saved and exported again, then closed through the known task window. Combined artifact receipt `f011ad5d3699fd1dadb76cdb2326f6191be9c3a6f7844941cda2244bd74e9a0f` and template receipt `ccd3e2470cd2471d648547844b9759ffc839021a37491247917715416553025f` qualify preserved content, styles, numbering, owned links and images, template section/header/footer/PAGE facts, and identical first/reopened PDF text, geometry and rasters. The two DOCX inputs are the full native acceptance set. Attempt suffixes are not additional cases.

The combined receipt pins the observation record before the final owned close. The current supplemental close observation is authenticated separately in the preparation checkpoint. Original timed-out AppleEvent attempts and their failed receipts remain unchanged. The historical installed Word pair remains credited only for its recorded gate. Final integrated source, package, hash and paired interpreter gates belong to the sprint integrator.
