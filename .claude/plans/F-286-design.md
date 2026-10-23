# F-286, Rich, repeating, and typed content controls

**Status**: completed
**Sprint**: S95.3
**Size**: L
**Depends on**: F-285

## Problem

`SdtType` already names rich text, plain text, picture, checkbox, combo box, drop-down, date, group, repeating section and repeating item. The value facade can fill a control that already has the right payload. `CT_SdtPr::prepare_text_value` accepts a Gregorian date in `yyyy-MM-dd` or `dd/MM/yyyy`, and a drop-down value that matches one list item. `checkbox_state` maps the existing true and false vocabulary to a glyph. Picture and group refuse a text value. None of that creates the type element, the list items, the date metadata or a repeating item.

F-285 creates only a rich or plain shell and refuses every other kind. This story adds the missing payloads. It cannot start until F-285 has passed its dependency-prefix checkpoint. F-287 starts only after this story leaves the shared files.

DOCX-053 is complete within the explicit ten-kind and Q24 boundaries. The differential gate is a final observation against pinned Word structure and behaviour under replacement and repetition. It is not a worker oracle. A package this workspace generates is not that observation. Missing authentic Word provenance, version and input identity stays pending.

## Spec reference

- `docs/hld/14-development-backlog.md`, F-286.
- `docs/hld/02-scope-and-non-goals.md`, DOCX-053.
- `docs/hld/03-architecture.md`, content-control type and shell identity.
- `docs/hld/10-bindings-spec.md`, value replacement.
- `docs/hld/12-testing-strategy.md`, differential controls for content controls.

## Approach

Add one type setter on a shell from F-285. F-285 `insert_content_control` does not grow a payload argument.

Rust `Document::set_content_control_type(&mut self, location: &ContentLocation, payload: ContentControlTypePayload) -> Result<()>`. Python `Document.set_content_control_type(location, payload) -> None`. A refusal returns `Error::Other` and does not publish. Python surfaces that as `RdocxError`. WASM and CLI gain no method. The frozen `ContentControl` snapshot stays a read of tag, alias, id, type and text.

`ContentControlTypePayload` is the whole supported list from Q17:

- `RichText` and `PlainText`. The type element is empty `w:richText` or `w:text`.
- `Date { format }`. `format` is `yyyy-MM-dd` or `dd/MM/yyyy`. The writer emits `w:dateFormat`, then `w:storeMappedDataAs` with `text`, then `w:calendar` with `gregorian`, and omits `w:lid`. That is the `CT_SdtDate` child order with `lid` skipped. Any other format refuses.
- `CheckBox`. The type element is `w14:checkbox` with `w14:checked` `0`, then `w14:checkedState` `2612` in `MS Gothic`, then `w14:uncheckedState` `2610` in `MS Gothic`. Those defaults are the ones `checkbox_state` already uses when the slots are absent.
- `ComboBox { items }` and `DropDownList { items }`. Each item is one display string and one value string, written as `w:listItem` in caller order. An empty list, a duplicate value, or a non-XML character refuses.
- `Picture { relationship_id }`. The type element is empty `w:picture`. The shell content must already contain a drawing whose embed id is that relationship, and the part must already own it. This call does not allocate media. Missing content or a missing relationship refuses.
- `Group`. The type element is empty `w:group`.
- `RepeatingSection`. The type element is empty `w15:repeatingSection`. The shell content must already be one repeating item, or one block sequence that this call wraps in one nested `w15:repeatingSectionItem`. Empty content is zero items and refuses.
- `RepeatingSectionItem`. The type element is empty `w15:repeatingSectionItem`. Q17 authors this kind on the template shell inside a repeating section. The addressed control's parent must already be a repeating section. Any other parent refuses and publishes nothing.

Citation, equation, bibliography, document-part list and document-part object are not variants. A call that asks for one refuses.

The setter replaces the type element and keeps tag, alias, id, lock, appearance, placeholder and unmodelled raw slots. The F-285 placement of the shell stays. The type element does not move a block shell into an inline parent.

Value replacement uses the existing `set_content_control_value_by_tag`, `set_content_control_value_by_alias`, `fill_content_control_tags` and `bind_content_controls`. This story does not add a second value engine. Picture and group still refuse a text value in `prepare_control_value`.

`Document::repeat_content_control(&mut self, location: &ContentLocation) -> Result<()>` and Python `Document.repeat_content_control(location) -> None` address a repeating section. The call clones its first `w15:repeatingSectionItem`, assigns a new sdt id, drops `w14:paraId` and `w14:textId` through the F-285 freshen path, and appends the clone. Zero items refuses and does not publish. One item becomes two. Many items append one more.

Removing an item uses F-285 `remove_content_control(location)` and the same Python name. When that location is the only repeating item in its parent section, the call returns `Error::Other` and does not publish. Removing one item from several uses the ordinary F-285 delete.

No new file. Python payload constructors mirror the Rust variants: `rich_text`, `plain_text`, `date(format)`, `checkbox`, `combo_box(items)`, `drop_down_list(items)`, `picture(relationship_id)`, `group`, `repeating_section` and `repeating_section_item`. `items` is a sequence of `(display, value)` pairs.

Local tests construct the replacement and repetition observations in source. They do not add a binary fixture. Those observations are not the Word differential. Differential evidence needs authentic Word provenance, the Word version and the input identity. A package this workspace writes, including a checked-in copy of its own output, is not a Word oracle. Until that evidence exists, the differential gate stays pending.

## Consumer map

| Rule | Entry points and outputs | Shared provider and caller policy | Lifecycle stages compared |
|---|---|---|---|
| Type payload and value fill agree | `set_content_control_type` and the existing tag, alias and bind setters | `CT_SdtPr::prepare_text_value` and `checkbox_state` | Set type, fill, save, reopen |
| Lists reject an unknown choice | Combo and drop-down fill | One item table. Display and value are both accepted as the caller key | Known value, unknown value, ambiguous value |
| Repeating section grows by cloning the template | `repeat_content_control` on the section. `remove_content_control` on an item | F-285 freshen path and F-285 remove. Fresh id, dropped `w14:paraId` | Zero, one and many items |
| Picture uses an owned relationship | Picture payload and the existing image embed | No new media allocator | Present relationship, missing relationship, text value |

## Behavior contracts and evidence

| Surface or placement | Exact behavior and boundary examples | Primary source identity and location | Local discriminator or pending evidence |
|---|---|---|---|
| Type setter | Rust `set_content_control_type(&mut self, location: &ContentLocation, payload: ContentControlTypePayload) -> Result<()>` and Python `set_content_control_type(location, payload) -> None`. The payload variants are rich text, plain text, date, checkbox, combo, drop-down, picture, group, repeating section and repeating item. The call replaces the type element and keeps tag, alias, id, lock, appearance, placeholder and unmodelled raw slots. An invalid payload returns `Error::Other` and does not publish. Citation, equation, bibliography and document-part kinds stay refused. | Q17. `SdtType` names in `crates/rdocx-oxml/src/content_control.rs` lines 24 to 82. Checkbox child order is the fixture in `crates/rdocx-cli/tests/integration.rs` around line 6786. Date child order is `dateFormat`, `storeMappedDataAs`, `calendar`. | A1 and A8. Local tests do not claim a Word match. |
| Checkbox vocabulary | `true`, `1` and `yes` check. `false`, `0` and `no` uncheck. `maybe` refuses and does not publish. | `prepare_control_value` at `crates/rdocx/src/content_control.rs` lines 81 to 88. | A3. |
| Date, lists, picture and group | Date uses the existing Gregorian `yyyy-MM-dd` and `dd/MM/yyyy` forms. Lists reject duplicate, unknown and ambiguous values. Picture requires an already-owned relationship and refuses text. Group refuses text. | Q17. Existing `prepare_text_value` and the picture and group refusal at lines 102 to 106. | A2, A4, A5 and A6. |
| Repeating section and item | `repeat_content_control(location)` clones the first item inside a repeating section. Zero items refuses. One item becomes two. `remove_content_control(location)` deletes one item of several. The same method refuses when the item is the last one in its section. `RepeatingSectionItem` is its own payload and is legal only on a shell whose parent is already a repeating section. | Q17. Element names are `w15:repeatingSection` and `w15:repeatingSectionItem` at `content_control.rs` lines 76 and 77. Removal is the F-285 method. | A7 and A8. |
| Word differential | Every supported control must pass the pinned native Word comparison under replacement and the documented equivalent repetition operations, retaining the explicit Q24 differences. A package this workspace generates is not that evidence. | Backlog test gate. `.claude/skills/differential-testing.md`, native provenance rules. | Native Word for Mac16.114(26101123) originals and input hashes are retained outside tracked fixtures. The installed external controls pass. Q24 is approved. Final feature acceptance still requires the complete evidence reconciliation. |

## Dependency readiness

All formal provider dependencies and shared-file leases were integrated before
this implementation. The applicable local gates and final provider riders now
pass with actual producer/source correspondence. S95.3 final release preparation
and its final sprint review remain separate, unfinished obligations.

## Acceptance criteria

| ID | Input or setup | Expected observation or refusal | Consumers | Local test | Final gate or pending evidence |
|---|---|---|---|---|---|
| A1 | Set each payload except `RepeatingSectionItem` on an F-285 shell, then reopen | The type element and its children follow the order named above. Tag and raw slots remain. The authenticated Word consumer and native structure observations qualify the documented bounded surface | Native `set_content_control_type` and Python `Document.set_content_control_type` | `typed_content_control_payload_round_trips` | Authenticated Word producer, structure/consumer observations and pinned input identities retained |
| A2 | Date `2026-10-18` with both formats, plus `31/02/2026` and a non-Gregorian calendar marker | The valid date stores the existing display and full date. The two invalid inputs do not publish | Existing date preparer | `typed_date_control_rejects_bad_calendar_values` | Native ISO/dmy replacement comparisons pass |
| A3 | Checkbox `true`, `false`, `yes`, `no`, `1`, `0`, and `maybe` | The first six follow today's glyph mapping. `maybe` refuses | Existing checkbox path | `typed_checkbox_control_uses_existing_vocabulary` | Native checkbox replacement comparison passes, local vocabulary/refusal controls pass |
| A4 | Drop-down and combo with two items, a repeated value, and a fill that names the display | Unique items round-trip. The repeated value refuses. Fill selects the matching item and writes its value | List payload and `prepare_text_value` | `typed_list_control_rejects_unknown_and_ambiguous_values` | Native combo/drop-down replacement comparisons pass, local list refusals pass |
| A5 | Picture with an embedded image relationship, a missing id, and a text fill | The owned id round-trips. The other two refuse | Picture payload and value facade | `typed_picture_control_requires_an_owned_image` | No render change claimed |
| A6 | Group, then a text fill | The group shell round-trips and the text fill refuses | Value facade | `typed_group_control_refuses_text` | Native group consumption and text refusal qualify the documented boundary |
| A7 | Repeating section with zero, one and three items. Call `repeat_content_control` once. Call `remove_content_control` down to one item and then on the last | Zero refuses and publishes nothing. One becomes two. Three becomes four. The last item refuses and publishes nothing. A non-last item is deleted by the F-285 method | `repeat_content_control` and F-285 `remove_content_control`, Rust and Python | `repeating_section_clones_the_template_item` | Native equivalent repeat and middle deletion pass, Q24 intentional differences retained |
| A8 | `RepeatingSectionItem` on a shell inside a repeating section, and the same payload on a body shell | The nested shell round-trips as `w15:repeatingSectionItem`. The body shell returns `Error::Other` and the package bytes stay | Native and Python `set_content_control_type` | `repeating_item_requires_a_repeating_section_parent` | Native equivalent repeat and middle deletion pass, Q24 intentional differences retained |

Citation, equation, bibliography, document-part list and document-part object stay readable and are not authored here. A setter call for those kinds refuses. That keeps the backlog list exact.

## Test matrix

| Case | Modes or state combined | Expected difference or agreement | Wrong implementation rejected |
|---|---|---|---|
| C1 | Rich and plain shells converted to each typed kind | Unmodelled `sdtPr` children survive the conversion | A setter that rebuilds properties and drops raw slots |
| C2 | Empty item list, one item, many items | Only one and many can be repeated. `repeat_content_control` and `remove_content_control` are the calls | A repeating section with no template, or a private helper with no public entry point |
| C3 | Display key versus value key on a list | Both select the same item when they are unique | A filler that compares display only |
| C4 | Save and reopen between set and fill | The payload is still there, and fill does not reset it | A fill that depends on in-memory state the package lost |
| C5 | Local replacement and repetition versus the Word differential | Local tests observe child order, refusal and repetition counts. They do not certify Word. Missing authentic provenance stays pending | A test that treats generated XML as the Word oracle |
| C6 | `RepeatingSectionItem` inside a section versus on a body shell | Only the nested parent accepts the payload | A variant list that omits the kind, or a body shell that accepts it |

HTML export and layout already walk `sdtContent`. They are not given a second type policy. A picture control's image bytes are the existing drawing, so this story does not change the renderer.

## Rejected alternatives

- A second value engine beside `prepare_text_value`.
- Authoring citation, equation and bibliography in this story. They are outside the backlog sentence.
- Running Word on every worker iteration. The differential is final evidence, and only when provenance, version and input identity are recorded.
- Treating a source-built package, or a checked-in copy of this workspace's own output, as the Word oracle.
- Adding a binary fixture. The repository keeps fixtures in source, and the external corpus stays outside the published crates.
- Creating type payloads in F-285. That story refuses them.

## Test plan

**Test gate**: differential. Every supported control passes the pinned native Word comparison under replacement and documented equivalent repetition, with the explicit Q24 differences.

Local tests construct those replacement and repetition observations in source and do not add a binary fixture. They are not the Word differential. Differential evidence must name authentic Word provenance, the Word version and the input identity. Generated output is not a Word oracle. Missing evidence stays pending.

| Timing | Category | Test | Asserts |
|---|---|---|---|
| local scoped | regression | A1 through A8 | Child order, refusal, repetition counts and the repeating-item parent. These rows do not claim a Word match |
| local scoped | regression | Existing `prepare_text_value` and checkbox tests | Date and list rules did not move |
| final sprint | oracle | Authentic Word structure and behaviour under replacement and equivalent repetition | Actual native producer/version/input identities and Q24 differences accepted. New-version integration and the final eight-story sprint gate remain separate |

Tests join the F-285 module in the existing `rdocx` integration entrypoint.

## HLD impact

- `docs/hld/02-scope-and-non-goals.md`, DOCX-053
- `docs/hld/03-architecture.md`
- `docs/hld/10-bindings-spec.md`
- `docs/hld/12-testing-strategy.md`
- `docs/hld/14-development-backlog.md`

## Risk routing

- Any parser or serialiser. Type-element child order and raw-slot retention.
- XML creation in multiple parent contexts. A type payload uses the F-285 placement row. An unsupported pairing still publishes nothing.
- Public API of a published crate. Additive payload setter. Final publish dry run.
- WASM or PyO3 bindings. Python is included. Same rider as F-285.
- An external oracle comparison. Authentic Word provenance, version and input identity are required. Missing evidence stays pending. Generated output is not the oracle. Do not add a binary fixture.
- Shared semantic behavior with multiple consumers. Set, fill, save and reopen.
- Grammar, scope, compatibility mode or empty collection. C2 covers zero, one and many repeating items.
- F-285 dependency. Reviewed status is not enough. The prefix checkpoint must have completed.

No hash delta is expected. A fixture that only adds a new document does not move the existing baseline.

## Hash harness

Unchanged.

## Implementation checklist

- [x] Wait for the F-285 dependency-prefix checkpoint.
- [x] Obtain authentic Word provenance, version and input identities, compare native observations and reconcile Q24.
- [x] Add RED coverage for one missing payload and one bad list value.
- [x] Implement `set_content_control_type` and `repeat_content_control` on the existing property model. Last-item removal stays on F-285 `remove_content_control`.
- [x] Update the HLD list and obtain a zero-finding microscope.
- [x] Complete the final native comparison and independent accepted-contract review with Q24 differences explicit.

## Deviations

The F-285 prefix checkpoint is met. Its integration commit is `e7f3e98f`, the ledger commit is `0c020da0`, and this claim is `e615dd73`. The original dependency-readiness guard described F-285 as pending. The actual prefix met it before implementation, and accepted provider completion now supersedes that state.

Combo and drop-down fill share `prepare_text_value`. The previous combo assertion accepted any free string. A4 rejects an unknown combo value, so that assertion now expects a refusal. There is no second value engine.

Citation, equation, bibliography, and document-part kinds have no payload variant. Creation still refuses them through the F-285 insert path.

A missing API is not a semantic RED. The bad-list RED is the combo assertion above, which failed on the previous source. Additive payload tests arrived with the implementation. No separate RED commit was recorded. The Word differential was pending at that implementation checkpoint and is now accepted within Q24.

## Contract closure

Reviewed against `.claude/reviews/S95.3-design-quality-pass-1.md` R1 on 2026-10-19.

| Finding | Corrected section | Acceptance and test |
|---|---|---|
| R1 missing `RepeatingSectionItem` variant | Approach payload list and the type-setter behavior row | A8 and C6 |
| R1 repeat and last-item removal had no callable entry | Approach `repeat_content_control` and F-285 `remove_content_control` | A7 and C2 |
| R1 Python payload disagreed with the native setter | Approach Python `set_content_control_type`. F-285 insert stays payload-free | A1 |

At that design-quality checkpoint no new product choice arose. Q17, Q19 and Q21 stayed answered and the Word differential remained pending. Accepted provider completion records the later authentic qualification and Q24 disposition.

## Native qualification checkpoint, 2026-10-22

This records the pre-approval checkpoint. Q24 and accepted provider completion below supersede its pending disposition without relabeling an earlier observation.

Authentic Word for Mac16.114(26101123) produced the original inputs through
its native object model. About Word supplied the version and build. Native
VBA sources, source/output hashes and the producer receipt are retained in
`.claude/scratch/S95.3-takeover-evidence/native-word-originals/`. Original
packages remain in Word's separate test directory outside git. No library
writer authored the native expected documents. Earlier unavailable-connection
observations are superseded by this actual connection, not relabeled as passes.

`test_f286_native_word_replacement_preserves_typed_payloads` authenticates each
external input hash before comparing rich/plain text, combo/drop-down,
checkbox and genuinely distinct ISO/dmy date replacements. Picture and three
native group fixtures refuse text atomically. The native initial date-iso
filename alone was misleading because Word reset its format on assignment.
The refined native cases set the format after the date and establish both.

`test_f286_approved_repeat_matches_native_equivalent_operations` uses distinct
First/Second/Third items with bold formatting on First. Its native equivalent
appends after the last item and copies the first item's FormattedText. It
checks template selection, formatting, append position, retained old ids,
fresh clone identity and non-last deletion. This is not a claim that Word's
default selected-item repeat command clones the first item. Native default
repeat copies the addressed item. Native last-item deletion removes the whole
section. The approved API instead clones the first and refuses last-item
removal. The user approved Q24 on 2026-10-22: retain the approved API and
document both differences. The differential gate uses the documented native
equivalent operation, preserves both default observations and does not claim
identical native defaults. No product API changes follow from this decision.

Separately, Word opened and saved twelve library-authored consumer inputs with
OpenAndRepair disabled and no repair prompt. These files qualify native
consumption, not native original authorship. All control types, payloads,
ids, tags, text, drawings and image bytes survive. Native rich text omits the
explicit empty richText marker, native dates add lid=en-GB, and native group
adds contentLocked. These normalizations are recorded individually, not hidden
in a general XML difference allowance. The type setter's approved metadata
preservation contract remains unchanged.

The two external installed tests passed against the previously source-qualified
release extension9b9234fa. Their optional bundle absence is a skip, never a
passed native gate. F-286 remains in-progress. This checkpoint does not weaken
the differential contract, supply a clean final sprint review or close B1.

## Q24 approved acceptance boundary

The user approved the recommendation on 2026-10-22. Repetition remains a
clone-first append API and removing the last item remains an atomic refusal.
Native Word's addressed-item default and whole-section last-item deletion are
retained observations of an intentional difference. The authentic comparison
uses native append plus first FormattedText copy for the equivalent operation.
This resolves the contradictory blanket native-default parity claim without
changing other acceptance rows, provenance or malformed-input requirements.

## Open questions

| ID | Affected F-IDs and A-IDs | Decision and recommendation | Status and answer/source |
|---|---|---|---|
| Q17 | F-286 A1 to A8 | `set_content_control_type(location, payload)` on an F-285 shell. The supported kinds are rich text, plain text, date, checkbox, picture, combo, drop-down, group, repeating section and repeating item. Citation, equation, bibliography and document-part kinds stay refused. Replacement and repetition observations stay required. Generated output is not a Word oracle. | Answered 2026-10-18 by the user. Checkbox mapping is the code at `content_control.rs` lines 81 to 88. |
| Q19 | F-285, F-286 and F-287 | This story starts after the F-285 prefix checkpoint and finishes before F-287. | Answered 2026-10-18 by the user. |
| Q21 | F-285, and this story's approval | The F-285 method names. This plan has no separate product choice. | Answered 2026-10-18 on F-285. This plan adopts those names. |
| Q24 | F-286 A1, A7 and final gate | Authentic Word default repeat copies the addressed item and last-item deletion removes the section. Retain the approved clone-first and last-item refusal contract with explicit differences, or revise the API to Word defaults. Recommendation: retain the approved API and document the native differences and equivalent operations. | Answered 2026-10-22 by the user: approved. Retain clone-first append and atomic last-item refusal. Document the native differences and compare the native equivalent sequence. Other native acceptance obligations remain. |

No external issue closes from this plan.


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


## B2 typed-context amendment

The completed delivery record remains historical while the sprint is blocked.
Independent designer investigation pass1 confirms physical repeating-type
placement is separate from payload grammar. MS-DOCX
[2.5.1.11](https://learn.microsoft.com/en-us/openspecs/office_standards/ms-docx/6b50d4d0-3f5d-4d8d-b507-9879bd342f3e)
requires section containment and block, row or cell item placement.
[2.5.1.10](https://learn.microsoft.com/en-us/openspecs/office_standards/ms-docx/b1768ff2-c9fe-4660-86e5-73cd8c94b0a0)
requires section children to be items. Combining these rules excludes inline
section authoring even when empty wrappers fit several payload grammars.
Containment does not require the immediate parent control to be a section.

The existing A7/A8 contract includes the proposed marker's physical slot,
qualified transparent ancestors, retained types at relocation boundaries,
item containment after relocation or section retirement, and item-only
children after a direct item's type or shell is retired. Refusals publish
nothing. Remove of a section deletes its owned content and remains allowed.
Compatible nested section retirement can retain an outer section owner.
An item's unwrap can replace it with an item-only sequence. Moving the last
direct item refuses. Ordinary block wrapping, legal existing row/cell item
branches and malformed imported-content preservation keep their existing scope.
No general schema validator, new API, immediate-parent rule or native parity
claim is introduced. These are concrete existing-invariant clarifications.

The new matrix combines inline/block payload ambiguity, item conversion,
section/item retirement, compatible nested ownership, unwrap replacement,
row/cell existing operations and compatible/incompatible relocation. Actual
old-native3958f3a4 RED and designer probes are retained. Current native integration acceptance composes411passed cases and the corrected
new native atomicity case, with eight existing ignores. The first new Rust
fixture omitted its type marker and failed, then only that corrected case was
rerun successfully. The earlier failed aggregate remains failed. Final local
nativee07f191d passes both two-case Python matrices, fullWord1065, priorities
and typing/stubs. Scoped clippy and bothWASM pass. The policy command reports
192tests,190passes/two existing skips, child0. Archive inventory measurements
are actual no-verify observations, not compilation gates. Independent focused
microscope, clean verified archive and current hosted qualifications remain
pending. Historical f9e hosted39806338556 passed its actual qualifications but
does not qualify this changed Word source.

Independent F-286 repeating-context microscope pass1 accepts this amendment
with zero defects, smells and nitpicks, plus five additional installed probes.
Actual49hashes, seven deterministic pixels and README inventories pass. The
clean verified archive, updated hosted qualification and final sprint pass5
remain due. No publication approval or sprint closure is implied.

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

## Q26 namespace-serialization amendment, 2026-10-22

Q26 is approved: extend the independent final bound from5to7, retaining all
previous passes. Run pass6 after the targeted B2 repair and actual affected
qualification. Pass7 is available only if needed. Publication remains separate.
Original eight completed delivery records remain historical. This is the same
B2 invariant, not new feature, effort or velocity credit.

Pass5 proves qualified type admission can be defeated by earlier modeled
serialization. At the reviewed pass5 source, TypeElement canonicalized every type name
except checkbox. Its writer does not establish the newly chosen canonical
prefix. Preserve imported type QName and its existing namespace meaning,
including local canonical-prefix shadowing. New or changed kind markers keep
the existing canonical authoring fallback. Retain raw children, attributes,
schema order and namespace scope. Do not recognize an unbound marker by local
name or introduce a general imported-document namespace validator.

The affected consumer chain is valid package, lazy import, actual modeled
staging, typed marker write, qualified placement admission, mutation and
complete saved XML. A raw-byte passthrough load/save is a control, not evidence
that modeled serialization ran. Existing date value-metadata prefix repair
and checkbox source names must remain correct. Audit all imported modeled
type names that share the two Start/Empty constructors.

- B2-N1: Native existing OXML unit entrypoint forces serialization of imported
  Word, checkbox and repeating types in empty and nonempty form. Alias and
  canonical bindings, local shadowing and opaque children retain expanded
  identities. Reopen retains the type. Actual old-source RED precedes the fix.
- B2-N2: Valid nested section/item packages with aliases declared at the root,
  outer control, properties or markers refuse move/clone into a paragraph,
  preserving complete package bytes. Canonical controls refuse as well. The
  alias case with canonicalw15boundforeign must not silently lose both types.
- B2-N3: Supported compatible direct-body relocation and a metadata edit force
  modeled staging, retain qualified types, ids or fresh clone ids, and opaque
  metadata after reopen. Do not invent a placement keyword or use the
  unsupported nested-source end-story route as the positive discriminator.
- B2-N4: Native facade and actual installed Python calls cover the same
  namespace/placement contract. Current whole affected native and Word suites,
  both Python priorities/typing configurations, scoped lint, bothWASM,
 49hashes/sevenpixels, README and genuine affected archive verification then
  current hosted Word artifacts precede finalpass6. Reuse unaffected PowerPoint
  and other observations only with exact source/input correspondence.

Risk routing adds the existing parser/serializer and multiple-context rows.
HLD03 and10 state namespace identity through actual modeled staging. Their
existing listed impact applies. Other listed sections retain current intent.
No public signature, payload kind, dependency, module or baseline changes.
Initial root-only diagnostics and the absent placement-keyword TypeError stay
separate from semantic RED and valid positive controls. Full current artifact
observations accepted in pass5 remain real, while this source change invalidates
affected successors.

### Namespace amendment local checkpoint and source availability

The actual independent namespace-identity microscope pass1 is clean. Both
imported marker constructors retain source QNames. Actual current local native
867b26f5passes all1066Word tests, both42priorities, typing/stubs and the three
selected matrices on3.9/3.12. Native OXML739plus1doctest, rdocx710library,
412integration,2457regression and twodoctests pass. The480dependent native
cases, scoped lint, bothWASM,49hashes and seven deterministic pixels pass.
The first OXML aggregate's stale lexical assertion remains a failed attempt,
followed by the corrected739pass suite. The README sandbox network attempt
is not acceptance. Use the existing inventory normalizer for VCS metadata.
Current22README inventories and changed-archive measurement must pass before
the clean source checkpoint. Genuine clean verification of the two affected
archives and current build-only hosted wheels remain pending at that checkpoint.

The orchestrator may push only the clean reviewed sprint/s95.3 source, without
force, for the named wheels.yml workflow_dispatch build gate. No worker push,
main merge, tag or publication. Bind the actual new source SHA, run id, job
outcomes and downloaded wheel identities before final full acceptance. Existing
35238e5bhosted evidence does not qualify this changed serializer. Finalpass6
follows actual affected qualification. All8delivery records remain historical.

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
