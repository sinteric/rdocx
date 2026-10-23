# F-X261, Resolve Word package preservation and inspection contributions

**Status**: completed
**Sprint**: S95.3
**Size**: L
**Estimate**: 6 to 9 developer-days, provisional
**Depends on**: F-X238, F-X220, F-X241

## Problem

Issues 387, 390 and 391 are open against main `7496a81c`. No candidate covers Issue 387. `CustomProperties` in `crates/rdocx-py/python/rdocx/_rdocx.pyi` supports `get`, `set`, `delete`, `__setitem__` and `__contains__`. It has no `__iter__`, `__len__`, `items` or name lookup. PowerPoint already iterates `PyCustomProperties` from `crates/rpptx-py/src/presentation.rs`.

Issue 390 has six rows. `set_style` and `add_style` drop `w:latentStyles` and other unmodelled style-root children. Removing a picture paragraph leaves its image relationship and media part. A plain TrueType part with content type `application/x-font-ttf` and a null `w:fontKey` opens and saves, then `fonts`, `render_pages` and `to_pdf` fail. A header relationship to a non-XML target is ignored by `validate`. `Comment.resolved` returns `bool` from `crates/rdocx/src/comments.rs` and the Python stub, and a missing `commentsExtended` entry becomes `false`.

The issue also names a control-owner failure that the reporter could not reduce, and PR396 discloses an existing remove, reinsert and `set_header_image` regression. Those two observations are not in the six reduced rows.

Issue 391 has five rows. `replace_xml` rejects a declared `w14` root attribute unless the prefix is written before the declaration is considered. `StoryItem.xml` is not standalone, while `Paragraph.xml` is. There is no read-only part XML accessor. Nothing reads or writes `w:pPrDefault`. A text-box story item can be listed and its text replaced, but no `Paragraph` handle reaches it.

PR395 head `5d481eb53f00926896f9de632115f4cb245fc941` and PR396 head `c71708541397f96128dea2dc8b9c1a21c7da498a` are the 2026-10-18 identities. Each is one archive-measurement commit ahead of functional parents `c36bc80c1c23a7fa32c02ba7f1d26ef53b666939` and `7d573cc67c6ca7e7884608224a89bba9bbb4a587`. The measurement commits touch README files and `scripts/readme_doctests.py` only. Both pulls are candidates. They overlap on `crates/rdocx/src/document.rs`, `crates/rdocx-py/src/document.rs` and `docs/hld/10-bindings-spec.md`, so they are one implementation, not two waves.

Current source is `67423b6b` plus the uncommitted F-X259 diff, which already edits `crates/rdocx/src/document.rs`. This story starts after that diff is integrated.

## Spec reference

- `docs/hld/04-opc-and-packaging.md`, checked Word source ownership, relationship reachability and embedded fonts.
- `docs/hld/03-architecture.md`, shared Word operations and owned publication.
- `docs/hld/10-bindings-spec.md`, Word properties, paragraph handles, comments and checked XML.
- `docs/hld/12-testing-strategy.md`, Binding tests, Word checked XML qualification and embedded font acceptance.
- `docs/hld/14-development-backlog.md`, F-X261.

## Approach

Inspect the functional parents and reimplement the accepted rows on the integrated tree. Do not merge either pull.

Style mutation keeps unmodelled root children, including `w:latentStyles`, in schema order ahead of `w:style`. Empty style records and foreign attributes on those children round-trip byte for byte through `set_style`, `add_style` and `insert_document`.

Picture removal drops a media part and its relationship only when the candidate package proves the part unreachable. Shared targets, pop then reinsert, and caller-embedded unused media stay. The disclosed header-image reinsert sequence gets its own constructed package and must pass. The unreduced control-owner report stays unqualified. The story adds a diagnostic that names the owner route it could prove, and it does not claim that private document repaired.

Plain supported font parts are listed, removed and rendered through the existing embedded-font provider. An unsupported MIME still refuses, and the error names the part and the content type. Native fallback is measured at the final gate. It is not assumed from a valid header.

`validate` reports a header or footer relationship whose target is not a header or footer part, including the part name and content type. Supported variants stay usable and still save.

`CommentRef::resolved` and the Python and CLI projections grow an unknown state. Rust returns `Option<bool>`. Python returns `bool | None`. A freshly added comment that writes an extension entry stays explicitly unresolved, which is `False`. A package with that part removed reports unknown, which is `None`. CLI treats only `Some(true)` as resolved. This is a public break on the pre-1.0 family.

`replace_xml` binds a root attribute when that prefix is declared on the same root, in either declaration order. An unbound prefix still refuses, and the target bytes do not change. Table and cell `replace_xml` use the same rule. `StoryItem.xml` gains the namespace declarations `Paragraph.xml` already writes, for every supported physical owner. The existing native exact-slice API stays.

Part inspection is read-only. Python `part_names() -> tuple[str, ...]` and `part_xml(part_name: str) -> str`. Rust `part_names(&self) -> Result<Vec<String>>` and `part_xml(&self, part_name: &str) -> Result<String>`. Names are package paths with a leading slash, returned in lexicographic order. That order includes parts with no relationship and stays put when a part is rewritten. Missing and binary parts return an error that names the part and content type. The read sees unsaved staged parts and returns no mutable handle. There is no WASM part API.

Default paragraph format uses Python `DefaultParagraphFormat` with `space_before`, `space_after` and `line_spacing`. Rust exposes `default_paragraph_properties(&self) -> Option<&CT_PPr>` and `set_default_paragraph_properties(&mut self, Option<CT_PPr>) -> Result<()>`. The Rust value is the stored `CT_PPr`, so every parsed `line_rule` string and `line_spacing` twip stays visible there. An omitted Python keyword preserves the existing value. A keyword set to `None` clears only that property. Clearing the entire native paragraph-default object preserves `w:rPrDefault` and unrelated opaque source. Malformed lengths do not publish.

The Python setter for `line_spacing` stays on `Length`, as Q12 decided. A float, an int and a bool refuse and do not publish. Assigning a `Length` writes `w:line` from that length in twips and `w:lineRule="exact"`. The twip conversion is the existing paragraph setter's integer division of EMU by 635. `Pt(12)` is 240 twips. Assigning `None` clears both `w:line` and `w:lineRule`.

The Python getter follows `Paragraph::line_spacing` and `Paragraph::line_spacing_multiple`. Storage for `exact` and `atLeast` is twips. Storage for `auto`, and for a missing rule, is 240ths of a line. The public multiple is that stored integer divided by 240.0, with no extra rounding. `w:line="360"` and `w:lineRule="auto"` read as `1.5`. The imported fixture `w:line="259"` and `w:lineRule="auto"` reads as `259 / 240`. `w:line="240"` with `exact` or `atLeast` reads as a `Length` of 240 twips. A missing rule with `w:line="480"` reads as `2.0`, and omitting the keyword leaves the rule attribute absent. A missing `w:line` reads as `None`, whether or not a rule is present. Any other rule string reads as `None`. Omitting the keyword keeps that unknown rule. Assigning the `Length` read from `atLeast` writes `exact`, so preserving `atLeast` or an unknown rule means omitting the keyword.

A story-item paragraph handle writes the physical text box and its qualified alternate in one transaction. If either copy cannot be updated, neither changes. A stale handle still fails. Custom properties gain `__iter__`, `__len__`, `items` and `__getitem__(name)` on the existing class, in native part order. An absent part has length 0. There is no second collection type.

No new crate, module or file. Issue 387 extends the existing `CustomProperties` class. It does not add a second collection.

## Consumer map

| Rule | Entry points and outputs | Shared provider and caller policy | Lifecycle stages compared |
|---|---|---|---|
| Custom properties can be listed | Python `Document.custom_properties` iteration, length, items and name lookup | Existing native `custom_properties()` order. PowerPoint is the behaviour reference, not a shared type | Absent part, imported part, edit then iterate, save and reopen |
| Unmodelled style root bytes survive | `set_style`, `add_style`, `insert_document`, package writer | Existing styles raw slots and schema order | Import, mutate, save, reopen |
| Picture media follows reachability | `remove_content`, pop and reinsert, header image | One pruning rule, shared with F-X260 range removal | Shared target, orphan, caller-embedded unused part |
| Comment resolved distinguishes unknown | Rust `CommentRef`, Python `Comment`, CLI comment listing | One optional value. CLI collapses only for display | Authored comment, stripped extension, explicit true and false |
| Checked XML and part reads agree | `replace_xml`, `StoryItem.xml`, `part_xml`, default paragraph format, text-box paragraph | Existing checked physical source. No second XML owner | Unsaved edit, standalone parse, save and reopen, paired text boxes |

## Behavior contracts and evidence

| Surface or placement | Exact behavior and boundary examples | Primary source identity and location | Local discriminator or pending evidence |
|---|---|---|---|
| Custom properties | `__iter__` yields each `CustomProperty` in package order. `__len__` is the count. `items()` yields `(name, property)` pairs in that order. `__getitem__(name)` returns the property and raises `KeyError` when the name is absent. An absent part has length 0. `keys()` is not added. | Issue 387 acceptance text. PowerPoint `__iter__` yields property objects at `crates/rpptx-py/src/presentation.rs` lines 560 to 570. | A1. |
| Comment resolved | Rust `Option<bool>`. Python `bool` or `None`. Missing `commentsExtended` entry is `None`. A library-written comment that still has an extension is `False`. CLI treats only `Some(true)` as resolved. | Issue 390. Today `CommentRef::resolved` returns `bool` and maps missing to false at `crates/rdocx/src/comments.rs` line 408. | A6. |
| Part inspection | Python `part_names() -> tuple[str, ...]` and `part_xml(name) -> str`. Rust `part_names(&self) -> Result<Vec<String>>` and `part_xml(&self, part_name: &str) -> Result<String>`. Names are package paths with a leading slash, in lexicographic order. The order includes parts with no relationship. Missing, binary and non-UTF-8 parts return `Error::Other` naming the part and content type. Reads unsaved staged parts. No mutable handle and no WASM part API. | Issue 391 asks for a read-only accessor by part name or relationship. Q11 chose part name and `Result<String>`. Q20 chose lexicographic order. `package.parts` is a `HashMap` in `crates/oxml-opc/src/package.rs`. | A9. |
| Default paragraph format | Python setter accepts `Length` only. `None` clears `w:line` and `w:lineRule`. A float, int or bool refuses and does not publish. Assigning `Length` writes twips and `w:lineRule="exact"`. `Pt(12)` is 240 twips. The getter returns `Length` for `exact` and `atLeast`, the stored value divided by 240 as `float` for `auto` and for a missing rule, and `None` when `w:line` is absent or the rule is any other string. Omitting the keyword preserves `atLeast`, a missing rule and an unknown rule. Rust returns the stored `CT_PPr`. | Q12 and Q22. `CT_PPr::line_rule` at `crates/rdocx-oxml/src/paragraph_properties.rs` line 198 accepts `auto`, `exact`, `atLeast` and any other stored string. `Paragraph::line_spacing` and `line_spacing_multiple` are `crates/rdocx/src/paragraph.rs` lines 2968 to 2984. The Python paragraph getter is `crates/rdocx-py/src/formatting.rs` lines 1426 to 1439. | A10 and A13. |
| Text-box paragraph handle | One transaction updates the physical text box and its qualified alternate, or changes neither. A stale handle still fails. | Q13 and Issue 391. | A11. |
| Picture reachability | Only proved-unreachable owned media is dropped. Shared targets, pop then reinsert, and caller-embedded unused media stay. | Q14 and Issue 390. | A3 and A12. |

## Dependency readiness

All formal provider dependencies and shared-file leases were integrated before
this implementation. The applicable local gates and final provider riders now
pass with actual producer/source correspondence. S95.3 final release preparation
and its final sprint review remain separate, unfinished obligations.

## Acceptance criteria

| ID | Input or setup | Expected observation or refusal | Consumers | Local test | Final gate or pending evidence |
|---|---|---|---|---|---|
| A1 | Imported properties in package order, an empty document, then one `set` | Iteration, `len`, `items` and `[name]` agree. Absence iterates empty. Order is the part order | Python `CustomProperties`. Native slice is the source | `issue387_custom_properties_are_enumerable` | Installed `rdocx` suite |
| A2 | Styles root with `latentStyles`, an opaque child and an empty style | `set_style`, `add_style` and `insert_document` keep the raw bytes and the schema order | Native styles writer and Python style API | `issue390_style_edits_preserve_unmodelled_root_source` | Hash harness unchanged |
| A3 | Shared image, unique image, orphan media, pop then reinsert | Only a proved-unreachable owned part is dropped | `remove_content` and package relationships | `issue390_removed_picture_media_has_proved_reachability` | Final package corpus |
| A4 | Plain TTF and OTF, a key variant, an unsupported MIME | Listing, removal and the layout reader agree. The bad MIME names the part | `Document.fonts`, render and PDF entry | `issue390_plain_fonts_share_decoding_and_diagnostics` | Deterministic font render at final sprint |
| A5 | Header relationship to `application/octet-stream`, plus a valid header | `validate` names the bad target. The valid header still renders | `validate` and header stories | `issue390_invalid_story_targets_are_diagnosed` | Final validate suite |
| A6 | Authored comment, extension removed, explicit open, explicit resolved | Unknown, false and true stay distinct in Rust and Python. CLI lists only explicit true as resolved | Rust, Python, CLI | `issue390_comment_resolution_preserves_unknown_state` | Installed suite and CLI integration |
| A7 | `w14:paraId` declared before or after the attribute, on paragraph, table and cell. A second document with an unbound prefix | The declared prefix binds. The unbound edit refuses and the bytes match the input | Python `replace_xml` | `issue391_root_attributes_use_declared_namespace_scope` | Hash harness unchanged |
| A8 | Story items for paragraph, table, cell, header and text box | `StoryItem.xml` parses alone. The native exact slice stays available | Python story item and native snapshot | `issue391_story_item_snapshots_are_namespace_complete` | Installed suite |
| A9 | Unsaved style edit, a missing name, a binary part, a header part, and a part with no relationship | Names are lexicographic package paths with a leading slash, including the unreferenced part. Binary and missing names refuse. XML matches the staged part. Restaging a part does not change the order | Python `part_names` and `part_xml` | `issue391_part_inspection_reflects_current_package` | No WASM part API |
| A10 | An omitted keyword, one keyword set to `None`, a `Length`, a clear of the whole native paragraph-default object, and a malformed length | The omitted keyword keeps its value. `None` clears only that property. The whole-object clear preserves `rPrDefault` and unrelated opaque source. The `Length` writes that property. Malformed input does not publish | Python default paragraph format and Rust `CT_PPr` | `issue391_default_paragraph_format_is_owned_and_atomic` | Hash harness unchanged |
| A13 | Imported `w:pPrDefault` line states: auto 360, auto 259, exact 240, atLeast 240, missing rule with line 480, no line, and rule `precise` with a line. Then assign `Pt(12)`, assign `None`, and assign `1.5` | Reads are `1.5`, `259/240`, a 240-twip `Length`, the same `Length`, `2.0`, `None` and `None`. Omitting the keyword keeps atLeast, the missing rule and `precise`. `Pt(12)` writes line 240 and `exact`. `None` clears line and rule. `1.5` refuses and publishes nothing | Python `DefaultParagraphFormat.line_spacing` and Rust `CT_PPr` | `issue391_default_paragraph_line_rules_keep_imported_states` | Hash harness unchanged |
| A11 | VML text box, DrawingML text box, and a qualified pair | The approved handle updates both copies or refuses both. A stale handle still fails | Python `Paragraph` on a story item | `issue391_textbox_paragraph_handles_preserve_physical_custody` | Installed suite |
| A12 | Constructed remove, reinsert and `set_header_image`. The unreduced control report has no fixture | The constructed sequence keeps a reachable image. The control row records the diagnostic and stays unqualified | Package graph and the progress record | `issue390_asset_reinsertion_and_unreduced_control_diagnostic` | No closure of the private report |

Issue 387 is A1. Issue 390's six reduced rows are A2 through A6, and A12 holds the two disclosures. Issue 391's five rows are A7 through A11.

## Test matrix

| Case | Modes or state combined | Expected difference or agreement | Wrong implementation rejected |
|---|---|---|---|
| C1 | Empty, one and many properties or style children | Length and order agree before and after one edit | A snapshot that ignores a later `set` |
| C2 | Shared versus unique media, valid versus binary parts | Shared bytes stay. Binary part XML refuses | Deleting every image part, or decoding a font as XML |
| C3 | Explicit false versus missing extension | Python `False` and `None` differ | Mapping both to `False` |
| C4 | Prefix declared first versus attribute first | Both bind. An unbound prefix still refuses | Accepting every `w14` attribute |
| C5 | One text box versus a qualified pair | Both copies change, or neither does | A silent edit of one copy |
| C6 | Current tree, candidate parent, measurement head | Same fixtures. The measurement commit is not a behaviour change | Treating the new head as an unreviewed feature commit |
| C7 | `HashMap` iteration, relationship order, and ZIP order | `part_names` is lexicographic and includes an unreferenced part | Returning map order, relationship order, or central-directory order |
| C8 | Imported auto, exact, atLeast, missing rule, absent line and an unknown rule | The getter matches the paragraph projection above. Omitting the keyword keeps atLeast and the unknown rule. Assigning `Length` writes exact | A getter that drops atLeast, or a write of the read `Length` that keeps `atLeast` |

## Rejected alternatives

- Merging PR395 and PR396 onto the dirty F-X259 tree.
- A new ZIP walker for part reads. The package already has part bytes.
- Closing Issue 390 because six reduced tests pass.
- A binding-only comment flag that leaves Rust returning `bool`.

## Test plan

**Test gate**: regression. Each reduced A-ID has an old-source semantic RED and a repaired GREEN. A12's unreduced input has no RED fixture and stays unqualified.

| Timing | Category | Test | Asserts |
|---|---|---|---|
| local scoped | regression | A1 through A11 in the existing Python and native entrypoints | The row observation, including atomic refusal |
| local scoped | regression | A12 constructed package | Reachable header image survives. Diagnostic text is stored for the missing fixture |
| final sprint | rider | installed products, deterministic font render, full hash harness, both WASM checks | Word 49 hash and seven pixel baselines unchanged |

Add tests to `crates/rdocx-py/tests/test_core.py`, `crates/rdocx-py/tests/test_formatting_tables.py`, `crates/rdocx/tests/regression_test.rs` and `crates/rdocx/tests/integration_test.rs`. Do not add a test binary. Font bytes used by A4 must already be licensed in the repo or be a checked-in minimal fixture with a licence note. Do not download a Google Docs sample.

## HLD impact

- `docs/hld/03-architecture.md`
- `docs/hld/04-opc-and-packaging.md`
- `docs/hld/08-rendering-spec.md`, only if the plain-font reader changes a documented fallback
- `docs/hld/10-bindings-spec.md`
- `docs/hld/12-testing-strategy.md`
- `docs/hld/14-development-backlog.md`

## Risk routing

- Contributed PR or new issue acceptance. Pin the refreshed heads and both functional parents. Keep A12 unreduced.
- Any parser or serialiser. Style child order, prefix-tolerant `replace_xml`, and byte-identical latent styles.
- Public API of a published crate. `resolved` becoming optional is the break. Part inspection and default paragraph format are additive. Final `cargo publish --dry-run` and crate size check.
- WASM or PyO3 bindings. Final wasm check for `rdocx-wasm`. Python tests stay excluded from the workspace test binary.
- Shared semantic behavior with multiple consumers. Rust, Python and CLI comment state, and both text-box copies.
- Grammar, scope, compatibility mode or empty collection. Empty properties, missing extension, absent part, binary part.
- Bundled fonts, only if A4 adds a font file. The licence file must match the bytes. Prefer an existing licensed face.
- Layout and deterministic font mode, only for the A4 render observation at the final gate.

Unit constructors stay truncating. This story does not round twips.

## Hash harness

Unchanged Word 49 hash and seven pixel baseline. A paint change is a stop, not a re-record. F-X262 owns PowerPoint paint.

## Implementation checklist

- [x] Re-read both functional parents after F-X259 is integrated.
- [x] Record old-source RED for every reduced row.
- [x] Implement in the existing files and keep one pruning policy with F-X260.
- [x] Update the HLD list and obtain a zero-finding microscope.
- [x] Accept actual installed, independently produced font, WASM and unchanged hash evidence. Keep the private unreduced input unqualified.

## Contract closure

Reviewed against `.claude/reviews/S95.3-design-quality-pass-1.md` R3 on 2026-10-19.

| Finding | Corrected section | Acceptance and test |
|---|---|---|
| R3 `atLeast`, a missing rule and a missing line had no read | Approach and the default-paragraph behavior row. The projection is the existing paragraph getter | A13 and C8 |
| R3 no stored-to-float example | Approach. `360 / 240 = 1.5` and the fixture `259 / 240` | A13 |

No new product choice. Q12 still limits the setter to `Length`. Q22 still writes `exact` on assignment. The other stored states stay readable or preserved by omission.

## Open questions

| ID | Affected F-IDs and A-IDs | Decision and recommendation | Status and answer/source |
|---|---|---|---|
| Q1 | F-X261 A6 | Rust `CommentRef::resolved(&self) -> Option<bool>` and Python `bool \| None`. Missing extension is `None`. An authored comment that still writes an extension stays `False`. CLI prints resolved only for `Some(true)`. | Answered 2026-10-18 by the user. |
| Q11 | F-X261 A9 | Read-only part names and part XML. Names have a leading slash. Missing and binary parts error with the part name and content type. Reads unsaved staged parts. No mutable handle and no WASM API. | Answered 2026-10-18 by the user. Rust returns `Result<String>`, not `Option`. |
| Q12 | F-X261 A10 | `DefaultParagraphFormat` exposes `space_before`, `space_after` and `line_spacing` through `Length`. Omitted keywords preserve. `None` clears one property. Clearing the native paragraph-default object preserves `w:rPrDefault` and unrelated opaque source. Malformed lengths do not publish. | Answered 2026-10-18 by the user. This does not adopt `set_style`'s float multiple. |
| Q13 | F-X261 A11 | A story-item paragraph handle updates both text-box copies or changes neither. A stale handle still fails. | Answered 2026-10-18 by the user. |
| Q14 | F-X261 A2 to A6 and A12. F-X260 calls the same pruning rule. | The six reduced rows and the constructed reinsert sequence can pass. The private unreduced control-owner document stays unqualified. A diagnostic is not a repair. | Answered 2026-10-18 by the user. |
| Q15 | F-X261 A1 | `__iter__`, `__len__`, `items` and `__getitem__(name)` on the existing class, in native part order. No second type. Absent part has length 0. | Answered 2026-10-18 by the user. Issue 387 says iteration yields `CustomProperty` objects. `keys()` stays optional and is not added. |
| Q20 | F-X261 A9 | `part_names` returns lexicographic package paths. That order includes parts with no relationship and does not move when a part is rewritten. | Answered 2026-10-18 by the user. `package.parts` is a `HashMap` in `crates/oxml-opc/src/package.rs`. |
| Q22 | F-X261 A10 and A13 | Imported `w:pPrDefault` with `w:lineRule="auto"`. Q12 keeps the setter on `Length`. The getter returns `Length` for an exact rule and the stored multiple as `float` for an auto rule. Omitting the keyword preserves the auto element. Assigning a `Length` writes `w:lineRule="exact"` and replaces the auto rule. The 2026-10-19 closure also projects `atLeast`, a missing rule, a missing line and an unknown rule through the existing paragraph getters. That does not change this answer. | Answered 2026-10-18 by the user. `Paragraph::line_spacing` and `line_spacing_multiple` supply the other stored states. |

No issue or pull request closes from this plan.


## Accepted provider completion, 2026-10-22

The applicable unique local checks, independent clean microscopes and actual
provider final riders are accepted with their original producer attribution.
Native Word evidence and Q24 resolve the earlier missing-Word condition for
F-286. Source-qualified full suites pass6732native tests with73ignored,
1061installed Word and1028installed PowerPoint tests. Two added external Word
controls pass separately. Both WASM configurations, minimal layout, lint/docs,
README inventories and22verified publication dryruns pass. All49hashes and
seven deterministic pixels stay unchanged. All seven edit and three read release
ratios are below3. Qualified independent font inputs are real producer outputs.
The50-deck/421-slide corpus has zero dropped shapes. Its below-target SSIM is
an explicitly advisory trend, not a passed target or an automatic failure.

The authoritative retained logs and source/artifact correspondence are in
`.claude/scratch/S95.3-takeover-evidence/`. Only test_core.py adds the exact150
independently reviewed external-test lines to its earlier qualified bytes.
Other324manifest inputs match. No product/native source or baseline changes
in this completion. Historical failed attempts stay failed. Private unreduced
Issue390 remains unqualified under its approved bounded disposition. This
completion does not close a whole external issue or claim a new full-suite run.

At this historical provider completion checkpoint, the eight-story global
gate was not passed. F-X263 new-version packages, local/hosted artifacts and
final independent sprint review were still due. Q25
permits at most two further final review passes,4and5. Publication still follows
sprint acceptance, closure and separate exact-main-SHA final approval.

## Typed-context qualification checkpoint before sprint review pass 5

The existing B2 customXml and typed-context amendments now have clean focused
microscopes, clean35238e5b verified rdocx archive and actual updated hosted
39820083897qualifications. The21other verified archives retain unchanged
inputs and their082aafa9producer. Current native6733composed, installedWord1065
and applicable unchangedPPT1028retain actual artifact/source attribution.
The exact observations, counts, limits and failures are reconciled in
CURRENT_SPRINT's current integrated amendment acceptance and the retained
repeating-context-amendment full-union receipt. Earlier checkpoint paragraphs
remain historical. No duplicate delivery credit. Final sprint pass5 and close
remain due, with publication separately approved at exact clean main.

## Namespace amendment qualification, 2026-10-22

The existing B2 namespace amendment has a clean independent microscope and
actual current artifact qualification at source 01d800f9. Both affected crates
have genuine clean archive verification, with 20 unchanged verified archives
retaining their producer. Actual build-only run 39830343186 passes 27 deciding
jobs with four publication skips. Fresh downloaded wheels pass 172 priorities,
both typing/stub suites and six additional matrix calls. Current local Word
has 1,066 passes. Native acceptance composes 6,734 passes with 73 ignored,
including 4,801 freshly executed affected cases. Exact observations, source
correspondence, failures and limits are reconciled in CURRENT_SPRINT's namespace
amendment qualification and namespace-identity-amendment full-union receipt.
Earlier checkpoints retain their historical meaning. All eight original
deliveries remain unchanged. Final sprint pass 6 and closure are pending at
this checkpoint. Their actual outcomes follow in the numbered review and sprint
tag. Publication is separately approved at exact clean main.
