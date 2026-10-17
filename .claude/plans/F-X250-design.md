# F-X250, Complete everyday Word chart equation list and note APIs

**Status**: completed
**Sprint**: S95.11
**Size**: L
**Depends on**: F-X200, F-X202, F-X215, F-X247

## Problem

The five remaining immutable Word reporter bodies in the four chart, equation, list and note groups need existing native providers exposed consistently. `Paragraph.add_equation` always selects inline math at `crates/rdocx-py/src/paragraph.rs:776`. `ListLevel` omits the native marker-property carrier at `crates/rdocx-py/src/document.rs:2678`. The numbered-item helper has no restart argument at `crates/rdocx-py/src/document.rs:8828`, and its native provider selects the first decimal instance at `crates/rdocx/src/document.rs:28579`. The chart binding exposes legend operations and authoring but no document read collection at `crates/rdocx-py/src/document.rs:8640`.

Native note creation already stages reference and related-part publication at `crates/rdocx/src/document.rs:24405` and `24484`, but new marker runs lack the matching reference styles. Checked removal already exists at `25204` and `25209`. A document-level Python removal method must call that provider using the selected id, without needing a retired note handle. All four dependencies are done in `docs/sprints/BACKLOG.md`, rows F-X200, F-X202, F-X215 and F-X247. No implementation or acceptance evidence is claimed by this design.

## Spec reference

- `docs/hld/14-development-backlog.md`, F-X250, the exact size, dependencies and round-trip gate.
- `docs/hld/09-charts-spec.md`, The ChartML model and Python authoring and owned replacement, especially single-plot data projection, precise optional type inspection and physical relationship ownership.
- `docs/hld/08-rendering-spec.md`, Word OfficeMath layout and Authored Word page appearance and note references, especially inline/display boundaries and exactly-once default note superscripting.
- `docs/hld/10-bindings-spec.md`, Python Word formatting and checked list publication, Python Word document operations, and binding owner/publication rules.
- `docs/hld/04-opc-and-packaging.md`, Checked Word operation source ownership, candidate publication and unmodelled source preservation.
- `docs/hld/12-testing-strategy.md`, Binding tests and integrated Python compatibility qualification.

## Approach

### Chart read-back

Add read-only `Document.charts: tuple[Chart, ...]`. A chart is an owned immutable data snapshot, with `chart_type: int | None`, `categories: tuple[str, ...]` and `series: tuple[CategorySeriesData, ...]`. Reuse the existing Python `CategorySeriesData` carrier and its tuple-valued `values`, plus native `oxml_chart::ChartData`. A small concrete snapshot class belongs in the existing Word binding source, with a private constructor and matching existing exports/stub carriers. It carries actual owned data, without a forwarding proxy or a live mutation API. No new module or file is needed.

The proposed finite collection scope is qualified chart occurrences in live direct main-body paragraphs, the scope of current Word chart authoring and legend ownership. Walk those paragraphs in physical body order and each paragraph's owned drawing occurrences in XML order. Repeated occurrences of the same chart part produce separate snapshots. A paragraph with two valid charts contributes two entries, rather than inheriting the legend setter's exactly-one restriction. Headers, footers, notes, tables, text boxes, content-control wrappers and opaque descendants are outside this collection scope. A relationship-only unreferenced part is not an occurrence. State this boundary in the binding and HLD, rather than implying a package-wide chart inventory. No recursive story provider is introduced.

Reuse the qualified direct paragraph/run/drawing/inline-or-anchor/graphic/graphicData/chart ownership rule at `crates/rdocx/src/document.rs:36760`, carrying the complete once-normalized namespace context. A foreign, rebound or opaque lookalike cannot supply a chart or relationship id. Resolve exactly one internal chart relationship from the actual main document part, accepting renamed parts and relative targets. Validate actual chart content type and target existence. Do not guess `word/charts/chartN.xml`, rely on relationship iteration order, or change the selected chart's ownership through reads.

Use the existing single-plot `read_chart_data` at `crates/oxml-chart/src/lib.rs:294` for bar/column, line, pie, doughnut, area, scatter and radar. Categories, series names and numeric values retain the provider's order and finite dense projection contract. Scatter categories retain the existing provider's numeric-X-to-label representation, without inventing a new XY snapshot API. Use the separate existing `read_chart_type` classifier for precise imported subtype integers or `None`. A supported data plot with a valid unsupported subtype can have data and `chart_type=None`. Unsupported combination/unknown data plots, inconsistent category/X vectors, absent or sparse required data, malformed selected metadata and broken relationships return an explicit existing chart/document error naming the occurrence or target. They are never omitted, replaced by empty data or given a guessed type. No new workbook requirement is added solely for this cached snapshot read. Preserve the existing shared provider's actual refusal domain.

A minimal additive native `Document::charts(&self) -> Result<Vec<(Option<i64>, oxml_chart::ChartData)>>` can expose this exact collection without a new public Rust struct. Keep discovery in the existing native document carrier. Reads must not flush/publish a revision, retire handles, edit caches or alter package bytes, including on partial collection failure. Source-preserved chart XML, opaque extensions and shared targets remain untouched.

### Display equations

Extend `Paragraph.add_equation(source, *, format, display=False)` with a real boolean display selector. Keep the required format argument, existing latex/mathml conversion entrypoints, finite F-X247 support and conversion diagnostics. Select the existing `OfficeMath::display` or `OfficeMath::inline` constructor before the same prepared paragraph publication. The return stays `EquationInsertResult`, whose equation and paragraph inspection expose `is_display`. Display authoring emits `m:oMathPara`, inline remains `m:oMath`. Preserve mixed content order, held paragraph ownership where its existing append path allows it, and atomic refusal for stale owners or invalid source. No renderer, conversion parser or returned-result redesign is required.

### Numbering and marker font

Extend `add_numbered_list_item(text, level=0, *, restart=False) -> Paragraph` and add `restart_numbering(paragraph, *, start=1) -> int`. Restart creates a fresh numbering instance pointing at the existing selected abstract definition, with the selected level's `startOverride`. The boolean helper restart starts at1. Following helper-added items continue that restarted instance, including after save/reopen. Do not change earlier paragraphs or repurpose their existing instance ids. Derive continuation from persisted numbering and physical paragraphs, without a new document-side hidden counter.

Preserve historical default-instance selection when no explicit restart override is active. For continuation, use the most recent direct-body paragraph at the requested compatible decimal level whose instance has an explicit start override for that level. Preserve the selected definition and its other overrides. An unrelated plain paragraph or bullet item does not cancel that continuation. A different requested level keeps its existing selection unless that level has an explicit compatible restart. These rules prevent a restart from silently changing other levels or custom list formats.

`restart_numbering` accepts a live same-document direct-body paragraph with an explicit valid numbered, non-bullet level. Preserve its level, definition, unrelated level overrides and source properties. Do not reconstruct unmodelled override data lossily. If the existing public projection cannot preserve required imported override source for the new instance, refuse with an explicit unsupported error before publication. Start accepts non-bool integers in0..4294967295, retaining the existing native u32 domain, including intentional zero. Reject bool, unsupported objects before numeric extraction, negative/overflow values, absent numbering, bullet levels, foreign, retired and unsupported related-story/cell owners. Return the new instance id only after allocation and prepared publication succeed. Allocation, package-graph and late publication failures preserve bytes, identifiers, held owners and the old instance.

Use existing definitions, `NumberingInstance`, `NumberingLevelOverride::new(level).start(start)`, checked instance creation and `Paragraph::set_numbering` on one staged candidate. Add only the minimal native helper needed to make the default helper's restart/continuation selection common and checked. Preserve the historical native signatures through delegation, with additive checked entrypoints if required. No public struct field or signature break is authorized.

Add `font: str | None = None` to the existing Python `ListLevel` constructor and read-only value. Carry it through the unpublished binding value to the native private `ListLevel::marker_properties` builder using `CT_RPr.font_ascii` and `font_hansi`. Existing callers and `ListLevel.checklist` default toNone. Validate a supplied nonempty XML-valid font name before definition allocation. Do not require the font installed, alter unit conversion, round indents, or replace other marker/source properties. The native published `ListLevel` fields remain unchanged. Qualify the explicit Arial bullet through XML, with bundled known-face controls for deterministic appearance.

### Note reference styles and removal

Set `FootnoteReference` or `EndnoteReference` on newly authored reference runs in the common native creation route, including the corresponding note-body marker where authored. Preserve custom source and existing imported notes. The reference has one effective superscript treatment, retaining the renderer's default exactly-once rule. Reuse existing style providers if definitions are required for a new document. Add only the matching character-style definitions actually needed, without overwriting a producer's existing style or setting an extra redundant baseline displacement.

Add `Document.remove_footnote(id: int) -> None` and `remove_endnote(id: int) -> None`. Validate a non-bool signed32 id before extraction or mutation. Use existing native checked removal and its missing/reserved/non-normal-note refusal and error mapping, without changing native missing-id behavior to success or fabricating a KeyError contract. Resolve the current note by id after unrelated structural edits, independently of any held note handle. Stage and publish through existing Python revision authority. Successful removal retires selected note aliases according to existing owner rules, removes its references through the native provider and preserves other notes, separators, relationships and opaque source. It must not revive already retired handles. True no-op operations elsewhere keep their existing behavior. Failed removal preserves package bytes and all live owners, with a subsequent valid edit proving usability.

## Rejected alternatives

- A relationship-order chart list or guessed type drops physical source identity and misclassifies imported plots.
- A new chart model, workbook synthesizer or recursive document-story engine exceeds the finite provider repair.
- New public Rust struct fields break existing callers when current tuple, chart, marker and numbering carriers suffice.
- A private Python numbering counter cannot survive reopen or reflect actual paragraph ownership.
- Calling a retired note handle, rewriting imported note runs or duplicating native removal bypasses existing checked source authority.
- Per-worker complete suites and native/oracle reruns repeat final verification without establishing a new configuration.

## Test plan

**Test gate**: round-trip

Use existing test carriers only. Local cases are unique regressions and run against the claimed F-ID Base and fixed source with recorded identities. Reuse an applicable actual observation. Complete suites, native controls and external comparisons run once on the final integrated sprint union, with no oracle pass claimed at worker readiness.

| Timing | Category | Test | Asserts |
|---|---|---|---|
| local scoped | regression | `test_fx250_chart_snapshots_follow_physical_occurrences` | Old source lacks charts, fixed source round-trips two plots, repeated/shared target and owned ordered series without byte or held-owner changes |
| local scoped | regression | `test_fx250_chart_snapshot_scope_and_refusal` | Multiple charts per paragraph, aliases and renamed targets work, table/header/opaque/foreign lookalikes do not enter the stated scope, malformed or unsupported selected data refuses without a partial result or mutation |
| local scoped | regression | `test_fx250_display_equation_preserves_inline_and_diagnostics` | Actual old-source display argument RED, fixed-source m:oMathPara and is_display, inline default and finite diagnostics unchanged, independent expanded XML and stale-owner atomicity |
| local scoped | regression | `test_fx250_numbered_restart_and_font_roundtrip` | Actual missing-argument RED, new num id, continuation before/after reopen, explicit start5 at retained level, exact override and rFonts ascii/hAnsi, earlier paragraphs and unrelated overrides unchanged |
| local scoped | regression | `test_fx250_restart_numbering_refuses_before_publication` | bool and callback-bearing wrong types do not extract, start boundaries, missing/bullet/foreign/retired/non-body owners, identifier and held-owner atomicity, later valid edit |
| local scoped | regression | `test_fx250_note_reference_styles_and_remove_by_id` | Both note styles, removal after handle retirement, other note retained, separators and note-part identity, missing-id and late failure atomicity, no revived alias |
| final sprint | round-trip | Five unchanged bodies from the four Issue385 Word groups | `test_charts_are_read_back`, `test_a_display_equation`, `test_a_numbered_list_restarts_and_a_bullet_level_takes_a_font`, `test_a_note_reference_is_styled_as_word_writes_it`, `test_a_note_is_removed_after_other_edits` execute against ordinary final installed Word and actual final CLI |
| final sprint | reporter accounting | Original pinned Issue385 test/conftest/gap bytes | Keep strict raw XPASS/exit status separate from unchanged-body runxfail PASS, exact five-body membership and independent saved-package observations, no edited markers or caught-exit success claim |
| final sprint | oracle | Pinned python-docx1.2.0 parsed package controls and independent ZIP/XML inspection | Actual chart type/category/series caches, numbering definition/instance/override links, display math and reference styles survive reopen. python-docx supplies supported structure observations only, with no invented chart/note-authoring API oracle |
| final sprint | native appearance | One bounded Word/LibreOffice control document | Actual application versions pinned, save/open/reopen without repair, chart series order, inline/display distinction, restarted values1/2/1/2/5 and note marker superscripting. Our exports use bundled fonts. Independent XML/visible glyph and position observers remain distinct. Fix any render metric/tolerance before capture, without a fabricated numerical threshold or claim of already performed work |
| final sprint | installed/API | Complete ordinary installed Word suite, strict typing and stubtest, affected native tests | New read-only chart types, optional font, restart arguments and note removals have matching real signatures, existing provider contracts retained. Full suites absorb their included local cases rather than duplicating them |

Keep the original source `/private/tmp/S95-issue385-word-tests.py` immutable. The five-body denominator is separate from S95's historical twelve-body evidence and the separate Issue386 nineteen-attempt record. Root reconciles the complete Issue385 integrated acceptance union before any full issue-closure claim. A known unsupported data projection or explicit collection exclusion is recorded as the stated finite API boundary, not broad chart parity.

## HLD impact

- `docs/hld/04-opc-and-packaging.md`, checked Word chart-read source scope and atomic list/note publication.
- `docs/hld/08-rendering-spec.md`, authored reference styles and the distinction between authored display math and existing rendering.
- `docs/hld/09-charts-spec.md`, Word chart snapshot scope, occurrence ordering, data/type projection and unsupported behavior.
- `docs/hld/10-bindings-spec.md`, exact new signatures, scalar admission, snapshots, list continuation and note-owner behavior.
- `docs/hld/12-testing-strategy.md`, finite local regressions and final integrated reporter, structure and appearance obligations.

Update exactly these current-intent sections before source freeze. Completion ledgers may claim native/oracle acceptance only after actual final results. No HLD03 ownership or dependency change, HLD14 scope expansion or HLD15 toolchain change is planned.

## Risk routing

- Parser or serialiser: read HLD04 and HLD06 preservation/order sections. Final round-trip union proves schema child order, aliases and once-normalized namespace meaning, unchanged opaque subtree bytes, relationship ownership, mixed math and numbering override source preservation.
- Public API of a published crate: read HLD10 and CLAUDE structural rules. Minimal additive native read/restart entrypoints only, no existing public struct or signature break. Final genuine affected-package `cargo publish --dry-run` verification and unchanged archive-size assertions, including actual local dependency patches, run once in root's union. Root owns any measured archive catalogue update.
- PyO3 bindings: read HLD10. Final WASM check uses `cargo check --target wasm32-unknown-unknown -p rdocx-wasm -p rpptx-wasm`, with all workspace tests excluding rdocx-py and rpptx-py extension binaries. A focused installed binding test may run locally, not a whole installed suite.
- External oracle comparison: follow pinned differential-testing rules. Final Word/LibreOffice/structural controls execute once with actual producer/tool identities and declared limitations. No new oracle is a product dependency.
- Layout, pagination and text shaping are not changed. Authored marker/font/display output still earns the existing final deterministic appearance controls. If implementation needs a renderer change or bundled font modification, stop and revise this plan rather than silently adding that scope.

## Hash harness

Expected unchanged for the existing deterministic49 hash cases and7 pixel cases. The new authored outputs do not authorize corpus or baseline rewriting. Record and review any actual intentional behavioral delta separately, with root's exclusive baseline lease. The final integrated harness runs once, without per-story whole-corpus execution.

## Implementation checklist

- [x] Confirm the approved finite chart scope and bind actual provider/source authority.
- [x] Establish unique local old-source RED and fixed-source GREEN observations without changing original reporter bytes.
- [x] Add chart snapshots, display selection, persisted list restart/font and common native note styles/removal through existing carriers.
- [x] Preserve scalar admission, relationship/owner authority, opaque source and late-failure atomicity with independent package observers.
- [x] Align existing exports/stubs and update exactly the HLD impact sections before source freeze.
- [x] Obtain independent zero-defect, zero-smell microscope and scoped local verification.
- [x] Prepare `sprint-final-acceptance` with named actual local checks passed and named final sprint checks pending.
- [x] Complete only after the final integrated round-trip/oracle/risk union actually passes.

## Open questions

No user-blocking question remains. The approved bounded story uses the explicit direct-main-body chart scope, seven existing data families, optional conservative type classification and source-preserving unsupported errors described above. The direct-main-body scope is the bounded proposed contract for approval. Extending charts into tables or related stories requires an explicit scope decision and named controls before implementation, rather than silently changing the collection after it is reviewed.

The restart continuation rule above preserves historical default selection while recognizing persisted explicit overrides. It does not promise an all-format numbering engine or reconstruction of opaque replacement levels. The new snapshot type stays in an existing carrier. Any new module/file, published Rust source break or work beyond the stated providers requires the normal explicit design approval. This draft grants no such additional approval.
