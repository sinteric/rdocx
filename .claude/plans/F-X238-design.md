# F-X238, Restore python-docx Word names from the first-draft corpus

**Status**: completed
**Sprint**: S94
**Size**: L
**Depends on**: none

Approved after S93 closed and S94 opened from main3a3b0de8be9f8bf4eb588a9a24c565f0af8ee190. Source citations were prepared against the unchanged runtime inputs of S93. No S94 runtime evidence is claimed by this design. Establish the exact feature Base when claiming its worktree.

## Problem

Issue379's 32 named Word specimens exercise missing aliases and input shapes. Current add_paragraph requires text and has no style keyword, and add_table has no style keyword at crates/rdocx-py/src/document.rs:9384 and9413. Constructor/open/save require PathBuf at6086,6096 and6209. Existing style, run, section, picture and table providers already own the required native operations. The contract permits either matching python-docx1.2.0 or an actionable refusal naming the existing alternative. Do not add document semantics or expand the default template merely to make a name appear available.

The three original missing divergence pointers include a PowerPoint source edit. Frozen Word sections and styles also expose an existing compatibility boundary: sections is a tuple of frozen Section snapshots at document.rs:7186,2449, while section(index) returns a checked live SectionHandle at7507. styles is currently a tuple at7571, so doc.styles['Normal'] fails before any Style.font accessor could explain the alternative.

## Spec reference

- docs/hld/14-development-backlog.md, "F-X238, Restore python-docx Word names from the first-draft corpus", size L, no dependencies, test gate regression, including add_section and the three divergence messages.
- docs/hld/10-bindings-spec.md, "Python API shape", "Python Word formatting and checked list publication", "Python colour inputs and live font formats", "Live Word sections and rich related owners", "Word tables, physical pictures and internal links" and "Word checked XML owner authority".
- docs/hld/04-opc-and-packaging.md, "Checked related-story publication", "Checked Word operation source ownership" and "Picture extraction and one-shot publication".
- docs/hld/12-testing-strategy.md, "Test taxonomy", "Word live-story and page-authoring controls", "Word table, picture and paired-owner controls", "Word checked XML qualification" and "Integrated Python compatibility qualification". The existing F-X211 metric remains separate.
- docs/hld/15-build-and-toolchain.md, "Toolchain pinning", "Deterministic rendering", "Packaging" and "CI job matrix".

## Approach

### Immutable inputs and acceptance accounting

Use /private/tmp/S94-parity-inputs/source-receipt.json and the exact PR333 commit193819ca0a1dcf3415017c0be1186fe8df6bcb86. Preparation reauthenticated all122 files against both SHA256 and Git blob SHA1, plus archive SHA25635f454637169c3859f40a01e4ab9aae9e1dc714441f33e58ff9aa9b6abf217b9. Membership is76 Word scripts,44 PowerPoint scripts and2 original runners. No source adaptation is authorized. Authenticate bodies, checks, modes, membership and tools again at execution and preserve original reports and first-attempt outcomes.

The original Word runner run_script at test_first_drafts.py:96 executes the body and then its saved-file check. judge at143 accepts many needs failures and rejects a passing divergence. Therefore raw runner exit alone is insufficient. Keep its exact report and exit, then independently classify every issue-listed name as correct saved-file success, explicit alternative refusal with the required first-line token, or unresolved failure. Do not substitute that classification for the raw result, reduce the denominator or combine different candidate runs into one success score.

The immutable tbl_merge_cells.py:2 has a stale divergence token set_cell_grid_span even though current Cell.merge exists at crates/rdocx-py/src/table.rs:2131 and the issue explicitly acknowledges its success. Preserve its actual saved-file success and raw runner diagnostic separately. Never regress Cell.merge, edit the marker or call the original full runner green when it is not. Original expected-pass specimens that receive an accepted named alternative remain raw runner failures and need an explicit independent contract disposition.

### Source-safe binding changes and concrete signatures

Keep every existing native Rust signature and default unchanged. Implement in existing binding/Python files, with the one explicitly user-approved new file crates/rdocx-py/python/rdocx/enum/section.py. The user answered "both approved" on2026-10-16, authorizing this prepared module exception. No additional file, crate, trait, generic parameter, feature or dependency is authorized. Reuse existing checked publication helpers rather than publishing an object and then applying its style in a second transaction.

Proposed Python-facing signatures, with ordinary existing calls preserved:

```python
Document(path: str | os.PathLike[str] | BinaryIO | None = None)
Document.open(path: str | os.PathLike[str] | BinaryIO) -> Document
Document.save(path: str | os.PathLike[str] | BinaryIO) -> None
Document.add_paragraph(text: str = "", style: str | None = None) -> Paragraph
Document.add_heading(text: str = "", level: int = 1) -> Paragraph
Document.add_page_break() -> Paragraph
Document.add_table(rows: int, cols: int, style: str | None = None) -> Table
Paragraph.insert_paragraph_before(text: str = "", style: str | None = None) -> Paragraph
Run.bold: bool | None
Run.italic: bool | None
Run.underline: bool | WD_UNDERLINE | None
Font.highlight_color: WD_COLOR_INDEX | None
```

Document.add_section(start_type=None), Table.add_column(width), Table.columns and frozen Section compatibility properties are deliberately present with named unsupported-operation errors where the existing provider cannot satisfy the oracle without changing semantics. Their actual callable/getter return is Never for the refused cases, not a fabricated successful result. Document these refusals in handwritten stubs where practical and preserve each existing supported alternative.

| Issue-listed names and exact script family | Selected disposition and actual existing source |
|---|---|
| add_heading: exp_pdf, fmt_headings, link_internal_anchor, list_heading_and_bullets, tpl_append_to_template | Stage paragraph plus actual Title/Heading1 through Heading9 style resolution. Existing native add_paragraph at crates/rdocx/src/document.rs:24698 and Paragraph.set_style at crates/rdocx/src/paragraph.rs:1335 suffice. Validate level0 through9, reject bool/out-of-range. Resolve a real appropriate paragraph style using style_id_of_type at crates/rdocx-py/src/paragraph.rs:356. Missing template styles raise a pointer to add_style/set_style rather than synthesizing font appearance or new style semantics. |
| add_paragraph() and style keyword: fmt_font_color_rgb, fmt_font_size_and_name, fmt_run_formatting_loop, fmt_paragraph_style_kwarg, list_bullet_style, list_nested_bullets, list_number_style | Default text to empty. Resolve style before any append publication at document.rs:9384. Preserve existing positional text behavior and None inheritance. Unknown or wrong-type style fails before identifiers, bytes or owner revisions change. |
| default List Number/List Bullet: list_style_assignment | Preserve default template bytes. Existing missing list style error must name supported add_numbering_definition/add_numbering_instance and paragraph numbering/style operations. Do not install name-only fake list styles. Existing style resolution at paragraph.rs:356 and explicit style APIs at document.rs:7575,7769 are the providers. This is an accepted alternative refusal, not a claim that raw expected-pass corpus classification becomes green. |
| Run.bold/italic/underline: fmt_bold_italic_runs, tbl_cell_bold_centered | Delegate through the same owner-aware Font update path used by Run.font at run.rs:372 and Font properties at formatting.rs:563,573,583. Preserve true/false/None and enum underline semantics, source and lifetime behavior. Lease formatting.rs separately from F-X239's font.size check. |
| insert_paragraph_before: doc_insert_paragraph_before | Support live same-document direct-body paragraphs through document.insert_paragraph at document.rs:9479 after physical content-index resolution. Apply optional style on the candidate. Cell/related/unknown owners refuse before mutation, naming supported story authoring or Document.insert_paragraph with the actual direct-body limitation. Do not reinterpret recursive paragraph indices as body indices. |
| add_page_break: sec_page_break | Stage one paragraph and page-break run together using Run.add_break at run.rs:313 and native BreakKind at crates/rdocx/src/run.rs:16,488. Return a bound Paragraph only after prepared publication. |
| frozen Section.header/footer/different_first_page_header_footer and first_page_header: hf_header_text, hf_footer_text, hf_footer_page_number, hf_different_first_page | Preserve Document.sections snapshot semantics. Add explicit refused compatibility getters/setter naming document.section(snapshot.ordinal), whose existing header, footer, first_page_header and flag are at document.rs:1752,1772,1760,1469. The exact first-page script assigns the flag before touching first_page_header, so independently test both names rather than stopping at its first failure. |
| enum.section WD_SECTION/WD_ORIENT and add_section: sec_add_section, plus outside-corpus name | Use the explicitly approved concrete enum/section.py module for WD_SECTION_START/WD_SECTION and WD_ORIENTATION/WD_ORIENT, with the exact pinned numeric members and identity aliases. Remove their temporary declarations from enum/text.py and remove its runtime sys.modules alias from enum/__init__.py. Inline annotations must qualify the exact direct submodule import under strict mypy. No synthetic module or duplicate enum classes. Document.add_section explicitly refuses with a pointer to document.insert_section(len(document.sections)) then document.update_section(..., break_type=...). Native insert_section at document.rs:30237,30260 inherits resolved geometry only, which is not permission to promise python-docx's section-cloning policy. Preserve native final-section ownership and opaque source. |
| WD_BREAK and Run.add_break(enum): sec_run_add_break | Add the bounded enum in existing enum/text.py and accept its line/page/column values through run.rs:313 without changing supported strings. Other enum break forms refuse with line/page/column alternatives. |
| WD_COLOR_INDEX and Font.highlight_color: fmt_highlight_color | Include both the enum import and the property, not an import-only fix. Map supported pinned enum values to existing Font.highlight names at formatting.rs:618,623. None clears only the selected property. Unsupported values name font.highlight with its supported colors. Assert expanded Word highlight names and values after reopen. |
| WD_ALIGN_VERTICAL: tbl_row_height_and_valign | An identity-preserving alias of existing WD_CELL_VERTICAL_ALIGNMENT in enum/table.py:15. Do not add another conversion policy. |
| Document/open input stream and save output stream: doc_open_and_save_stream | Support the exact BufferedReader constructor and BytesIO save chain, not only from_bytes. Reuse from_bytes at document.rs:6103 and to_bytes at6215. Keep filesystem paths/path-like behavior. Read only a binary seekable stream or explicitly refuse with from_bytes(stream.read()) guidance. Serialize fully before write. Detect invalid binary results and short writes, propagate I/O errors, leave document/owners unchanged on refusal, do not close caller streams or promise rollback of externally written bytes. |
| add_picture(stream): pic_from_stream | Extend existing picture_input at document.rs:258 to a checked binary readable input, delegating resulting bytes to the current native provider. Reject text streams and malformed read returns before publication. |
| add_picture(width only): pic_add_picture_width, pic_centered_picture | Keep existing native admission at crates/rdocx/src/document.rs:20799. Refuse with an explicit supported add_picture(..., width=..., height=...) alternative rather than introducing new sizing semantics in a binding-only story. Native metadata/probe behavior for both omitted dimensions is unchanged. Record this residual against the same original issue contract. |
| Table.add_column, columns/Column.width: tbl_add_column, tbl_column_widths | Add actionable refusal surfaces naming existing Table.insert_column(index, before=...), set_column_width(column,width), grid_widths and cell.width. Existing providers are table.rs:1160,1048,1030,2491. An empty zero-column grid needs the explicit Document.add_table(rows, cols) construction alternative rather than an invalid insertion index. Do not fabricate Column objects or change merged-grid authority. |
| add_table(style=...): tbl_style_and_alignment | Resolve/ensure the same existing built-in table style and type rules as table.rs:679 on the candidate before document.rs:9413 publishes the new table. Return only the prepared table owner. |

The exact link_internal_anchor script also passes anchor=heading to Paragraph.add_hyperlink after heading creation. Admit that keyword only to refuse before mutation with document.add_bookmark and paragraph.add_internal_hyperlink guidance. Automatic Paragraph-to-bookmark inference is outside the native-reuse contract.

Existing names not listed retain their behavior. Validate text, enum/style and unit operands before source access that could publish. Read-only properties and no-op aliases must not allocate package nodes or revisions. Expanded names, not prefix spelling or local-name search, decide structured assertions. Unrelated namespace declarations, relationships, source children and source order remain preserved.

### Three exact divergence messages

- Paragraph._element refuses before any XML facade is exposed. The first message line must contain remove_content, matching doc_delete_paragraph.py:2. Name Document.remove_content(Document.find_content_index(paragraph)) and retain .xml/.replace_xml guidance. Existing providers: document.rs:9438,9456 and paragraph.rs:689,694.
- doc.styles['Normal'] must refuse with first-line set_style, matching fmt_normal_style_font.py:2. Preferred design is an immutable tuple subtype in existing python/rdocx/__init__.py whose integer/slice/iteration/equality behavior retains snapshots, while string indexing raises the named pointer. No live Style proxy or mutable collection is introduced. This observable subtype boundary is the selected root-authorized routine compatibility choice below.
- SlideCollection._sldIdLst refuses with first-line slides.remove, matching slide_delete_xml_hack.py:2. Mention slides.move as the supported reorder path. Use existing crates/rpptx-py/src/slide.rs:1462,1524. This cross-family edit belongs to F-X238 by the accepted split.

### Concrete private styles getter bridge

Define private `_StyleSnapshots(tuple)` only in existing `crates/rdocx-py/python/rdocx/__init__.py`. Override `__getitem__` to reject str with a first-line `set_style` pointer and delegate all other operands to `tuple.__getitem__`. Keep ordinary tuple construction, integer/negative indexing, slices (returning ordinary tuples), iteration, equality, length, immutability and frozen Style snapshots. Do not add mutation or live lookup. The helper is not an extension pyclass, is absent from extension registration and all public `__all__` lists, and is not advertised as an importable public API.

Change the actual getter in existing document.rs:7571, retaining `fn styles<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>>`: first allocate its existing snapshot tuple. Obtain `py.import("rdocx")?.getattr("_StyleSnapshots")?` at getter call time after package initialization. Call the helper with that tuple. Checked-cast the returned object to `PyTuple` with the pinned PyO3 API. A tuple subtype satisfies that checked cast. Do not merely declare the helper and continue returning `PyTuple::new`. Read-only allocation/import/cast failures return before any document or owner mutation. The existing `length_object` already uses package import at call time at crates/rdocx-py/src/lib.rs:42, so this follows an actual initialization pattern rather than adding a second loader.

Keep public stub `Document.styles -> tuple[Style, ...]` at `_rdocx.pyi:1067`. No stub-only public helper class or extension export is introduced. Installed tests assert `isinstance(styles, tuple)`, actual private subtype construction, unchanged snapshot contents after subsequent document edits, all tuple protocols and the exact string-index pointer. Strict typing and installed stubtest qualify the public tuple annotation and exports without new ignores. The exact builtin type changes, which HLD10 records explicitly. No claim of exact `type(styles) is tuple` compatibility is made.

### Integrated-prefix reconciliation

The original claim Base remains a644dd9443346223e0003066bbfd33dc7937e3c7. ResumePrefix is55144ff81073f538544c3093404bba0f87f1ef55. Original dirty intent and untracked section/reviews are retained in stash652e0e3d2bf9d9cd4a5f1d1fa006c39c6706a2c2 and a private hashed snapshot. Reconcile only this feature against the prefix, preserving current F239 size/notes, F241 FormValues, F240 Signing/report/verify and ZIP9, and F247 math. Historical source qualifications do not qualify this union. Independent microscope compares the resumed HEAD while retaining the original intent and claim provenance.

Existing parity regressions additionally qualify F238-created paragraph/table/cell/section owner survival through current form operations and Signing. Run authority retires on actual form mutation and survives certified Signing and exact no-op. A signed five-kind form package must save/reopen through BytesIO with valid complete reports and filled values. Short writes leave package bytes, signature validity and held paragraph authority unchanged, without promising rollback of the caller stream.

### Feature source leases

F-X238 owns the Word binding document/paragraph/run/formatting/table files, existing Word enum/package/stub carriers and Word test_python_docx_parity.py/typing_smoke.py. Its third divergence uses existing PowerPoint slide.rs and documented-example/typing carriers. F-X247 owns separate native math/conversion and Word test_core.py math controls in wave1, with no binding API, stub or native PPT edit lease.

F-X239 receives Word formatting.rs and parity tests after F-X238. It owns PPT notes/slide/text/lib/enum/stub carriers and existing documented-example/typing controls, plus the bounded native projection in rpptx/src/lib.rs and existing crates/rpptx/tests/integration.rs. F-X241 subsequently receives Word document/core/stub and CLI form carriers. F-X240 receives both families and both CLIs last. The exact four-wave schedule is recorded below. No simultaneous source, test, package catalog or HLD semantic-section edits are allowed.

## Rejected alternatives

- Replacing frozen sections/styles with silently mutable live proxies, which changes established ownership and snapshots.
- Adding default list/template semantics or image sizing behavior through Python wrappers, outside the native-reuse contract.
- A synthetic section module or runtime-only alias. The user-approved section.py provides the actual direct runtime and typed import path.
- Editing the immutable corpus, treating needs as automatic acceptance, regressing working Cell.merge or claiming raw full-runner green after its stale marker fails.
- Publishing a paragraph/table before style or returned-owner allocation succeeds, which would expose partial mutation on refusal.

## Test plan

The **test gate** is **regression**, verbatim from the backlog. Pinned differential and installed-artifact checks below are additional riders, not a replacement gate. Keep F-X238 Word additions in existing test_python_docx_parity.py and typing_smoke.py. Lease existing PPT test_documented_examples.py and typing_smoke.py for the cross-family divergence. F-X247 owns the separate Word test_core.py lane during wave1. No new tests/ binary. The only new product file is the explicitly approved enum/section.py.

| Category | Proposed existing-file test case | Asserts |
|---|---|---|
| regression | fx238_word_names_create_on_one_prepared_candidate | Empty paragraph, heading levels, style keyword, table style and page break succeed or have the approved explicit refusal, with no partial append or leaked revision on invalid operands. |
| differential | fx238_word_names_match_pinned_python_docx | python-docx==1.2.0 tree comparisons for actual heading style/outline, font tri-state/underline, page-break and supported style/stream behavior. Compare expanded names and normalized values, not cross-writer byte equality. |
| regression | fx238_stream_input_and_output_are_both_qualified | Run exact unchanged stream script, path/path-like controls, malformed/text/read-error/short-write controls, binary saved-file reopen, full document and held-owner atomicity. |
| regression | fx238_highlight_and_first_page_names_are_not_import_only | Actual WD_COLOR_INDEX.YELLOW assignment creates correct expanded-name highlight. Snapshot first-page flag and first_page_header have actionable live-handle pointers. Positive live SectionHandle first/default headers save and reopen without altering unrelated linkage. |
| regression | fx238_no_new_native_semantics_refusals_name_alternatives | add_section, missing list styles, width-only pictures, table columns and unsupported owner insertion each name the actual supported provider, preserve exact bytes/revisions, and retain old valid calls. |
| regression | fx238_three_frozen_divergence_tokens_are_exact | Original _element, styles['Normal'] and PPT _sldIdLst triggers fail with remove_content, set_style and slides.remove respectively, before source or owner mutation. |
| regression | fx238_immutable_first_draft_contract_classification | Authenticate122 files and two unchanged runners. Run76/44 raw corpus cases in isolated installed environments. Retain raw exits, area reports, body/check outcomes, needs gaps and stale merge diagnostic. Independently account for32 listed Word cases, all names outside corpus and three divergence requirements without stitching candidates. |
| round-trip | fx238_aliases_preserve_source_and_owner_boundaries | Fresh/warm body/table/related owners, stale/foreign inputs, exact no-op bytes, namespace/opaque siblings, return identity and complete save/reopen semantics. |
| integration | fx238_fresh_installed_word_and_ppt_pointer | Build locked current Word wheel in a fresh venv and run complete Word suite plus focused PPT divergence. Authenticate archive RECORD, installed native/package/stub members and source. No PYTHONPATH/dev shadowing. |
| integration | fx238_runtime_exports_match_strict_stubs | mypy==2.3.0 --strict and stubtest for changed surfaces, including tuple-subtype and unsupported refusal annotations, on installed artifacts. Full both-family suites run once over the final integrated sprint union with actual membership/failure/skip accounting. |

Required RED evidence is an actual old installed-artifact outcome for missing names/pointers and a semantic negative control proving the independent requirement classifier rejects a wrong saved value or a generic unhelpful exception. Keep original unmodified report distinct from the classifier. No test or runtime outcome is claimed yet.

## HLD impact

- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md
- docs/hld/15-build-and-toolchain.md

## Risk routing

Apply the canonical .claude/skills/risk-routing.md and differential-testing.md.

- WASM or PyO3 bindings: read HLD10. Run cargo check --target wasm32-unknown-unknown -p rdocx-wasm -p rpptx-wasm, installed binding gates above and the required workspace test exclusions. Do not cargo-test an extension-module binary.
- Public API of published consumers: state additive Python aliases and intentional refusal/subtype compatibility below, run affected verified publication dry runs with local reviewed patches and assert every .crate under10MiB. Refresh only actual changed README/archive inventory observations. Build/install actual wheels and authenticate packaged exports, stubs and native library.
- External oracle comparison: pin python-docx==1.2.0 and python-pptx==1.0.2, record actual tool/interpreter/native producer identities and unchanged source bodies, keep F-X211 corpus/metric separate.
- Parser/serializer-adjacent binding authoring: read HLD04 checked publication and HLD06 public facade/source order. Existing native providers write all OOXML. Qualify namespace aliases/default namespaces, complete expanded-name structural assertions, opaque byte preservation, order and failure atomicity. No local-name-only XML owner admission.
- Image/media behavior: reuse current native bounds/admission, no new dimensions or default template render behavior. Deterministic mode for any existing rendering check and all baseline checks.
- New file/module row: the user explicitly approved the prepared enum/section.py exception on2026-10-16. Qualify its packaged membership, runtime import, exact enum values/alias identities and strict direct-import positive and negative consumers. Remove the temporary enum.text declarations and runtime-only alias. No other new product file, trait or generic is authorized.

Semver: existing native Rust signatures remain unchanged. Python aliases are additive. Exact-type identity of styles would change from builtin tuple to a tuple subtype if approved, while immutable ordering/index/snapshot behavior remains. Previously bare missing-name/import errors intentionally become named refusals. These compatibility boundaries require explicit documentation and tests, not an unqualified drop-in claim.

## Hash harness

Expected all49 existing entries and shared decoded pixel baselines unchanged. No default style/template, native renderer, font, unit constructor or baseline change is authorized. Any observed delta blocks the story until attributed and separately designed/reviewed. F-X238 has no baseline write lease.

## Implementation checklist

- [x] Approve all five S94 designs after S93 close and S94 opening. The user explicitly approved the prepared section.py file exception on2026-10-16.
- [x] Acquire actual existing-file leases and preserve current worktree/cache ownership.
- [x] Freeze authenticated122-file inputs and explicit issue-name inventory without copying a second canonical corpus/tracker.
- [x] Add actual old-artifact RED and independent saved-file/refusal controls to existing test carriers.
- [x] Implement selected aliases on prepared native candidates and named refusal paths, including stream save, highlight property and first-page header.
- [x] Preserve exact three divergence tokens and immutable raw/stale-merge reports.
- [x] Update existing exports/stubs and run pinned differential, fresh installed, typing and applicable risk checks.
- [x] Complete independent microscope with zero defects and zero smells, scoped verify and structured handoff. Final integrated full gate and sprint review remain later orchestration steps.

## Open questions

No unresolved blocking product question remains. The orchestrator approves the concrete approach and risk riders in this plan under the authorized S94 scope. Native and external controls are future gates until actually executed.
The immutable tuple-subtype getter bridge and separate immutable-runner/independent requirement classification are approved. The original stale merge-marker failure stays visible. The user answered "both approved" on2026-10-16, expressly authorizing the prepared enum/section.py file and the separately coordinated ZIP migration. F-X238 adopts only the section-file refinement here. Its concrete module replaces the temporary enum.text declarations and runtime alias while retaining the seven pinned enum values and identity aliases. The new section.py inline annotations provide typing, so no new stub file is needed. Review D2 remains unresolved until actual strict direct-import checks and independent review qualify it. Fresh whole-feature qualification follows reconciliation with integrated forms, math, notes, signing and ZIP migration source.

## Approved sprint waves and exclusive resources

Wave1 runs F-X238 and F-X247 in separate worktrees. F-X238 owns Word parity tests and the existing binding naming/stub lanes, plus its PowerPoint XML-divergence pointer. F-X247 owns native math/conversion, the existing OfficeMath oracle, Word core math tests and the sole published-package numeric catalog lease in this wave. Neither edits the other's test entrypoint or source files.

Wave2 runs F-X239. Wave3 runs F-X241. Wave4 runs F-X240. These stories serialize because their actual edits share binding carriers and the source file scripts/readme_doctests.py for mandatory genuine archive measurements. F-X240 follows the completed form surface for public fill/sign integration tests. Operational source leases add no speculative backlog dependency.

Each worker updates exactly its listed HLD impact under feature-specific semantic section leases before its prepared handoff. The integrator coordinates shared HLD sections and numeric catalog ownership. Shared package observations use genuine archives, and combined observations run again after integration. Workers never update sprint delivery totals. The integrator alone writes AS_BUILT and SPRINT_TRACKER. Keep complete installed environments and target/build caches isolated or exclusively leased. The hash baseline remains exclusive, with no approved new baseline move.
