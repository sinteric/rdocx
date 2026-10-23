# F-285, Content control creation and lifecycle

**Status**: completed
**Sprint**: S95.3
**Size**: L
**Depends on**: F-253, F-254

## Problem

The facade can list content controls and replace the value of a control that already exists. `ContentControlRef` in `crates/rdocx/src/content_control.rs` exposes `tag`, `alias`, `id`, `control_type` and `text`. `Document::set_content_control_value_by_tag`, `set_content_control_value_by_alias`, `fill_content_control_tags` and `bind_content_controls` stage display text and a bound custom XML value together. Python `ContentControl` is a frozen snapshot of those fields.

There is no public way to create a control without building raw `w:sdt` XML. `ContentFragment::content_control` accepts a `CT_Sdt`, and `insert_content`, `clone_content`, `move_content` and `remove_content_at` can move a block control that is already a direct story child. They do not allocate an id, write lock, appearance or placeholder metadata, or address a row, cell, paragraph or inline placement. `CT_SdtPr` stores `alias`, `tag`, `id`, `control_type` and `data_binding`. Lock, appearance and `w:showingPlcHdr` stay in raw slots. The parser already keeps foreign-namespace lookalikes of `showingPlcHdr`, covered near `crates/rdocx-oxml/src/content_control.rs` around the placeholder fixture.

DOCX-052 is still partial. F-X249's fill repair does not create controls. F-286 and F-287 need this shell before they add type payloads and stores.

The story starts after F-X259, F-X260, F-X261 and F-X262, because those stories also edit `crates/rdocx/src/document.rs` and `docs/hld/10-bindings-spec.md`.

## Spec reference

- `docs/hld/14-development-backlog.md`, F-285.
- `docs/hld/02-scope-and-non-goals.md`, DOCX-052.
- `docs/hld/03-architecture.md`, the content-control model and fragment ownership.
- `docs/hld/10-bindings-spec.md`, the present content-control read and replacement surfaces.
- `docs/hld/12-testing-strategy.md`, content-control round-trip.

## Approach

Add creation and metadata edits to the existing facade and the existing `content_control.rs` model. Do not add a crate, module or file.

Python `Document.insert_content_control(destination, placement, kind, tag=None, alias=None, lock=None, appearance=None, placeholder=None)` inserts one shell. Rust `Document::insert_content_control(&mut self, destination: &ContentLocation, spec: ContentControlSpec) -> Result<()>` takes those same fields in `ContentControlSpec`. Python does not expose that spec type. `ContentControlPlacement` is `Block`, `Row`, `Cell`, `Paragraph` or `Inline`, and it must match the destination parent. Block and cell parents accept a block control. A table accepts a row control. A row accepts a cell control. A paragraph accepts an inline control. A mismatched placement returns `Error::Other` and does not publish.

`kind` is `SdtType::RichText` or `SdtType::PlainText`. Any other kind returns an error that names F-286 and does not publish. `tag` and `alias` are optional strings and reject non-XML characters. The facade allocates the smallest positive `i32` id that no surviving control uses. Lock, appearance and placeholder are location-addressed edits on the existing property model. A value that the model cannot store returns `Error::Other` and does not publish.

The metadata methods are `set_content_control_tag`, `set_content_control_alias`, `set_content_control_lock`, `set_content_control_appearance` and `set_content_control_placeholder`. Each takes a `ContentLocation` and an `Option`. `None` clears that field. `remove_content_control(location)` deletes the shell and its content. `unwrap_content_control(location)` leaves the content only when the parent can hold it. Python uses the same names. Rust returns `Result<()>`. A missing control, a stale location or an invalid value returns `Error::Other` and leaves the package unchanged.

Move, clone and remove use the existing fragment transaction once the control bytes are selected. Block controls that are direct children keep `move_content`, `clone_content` and `remove_content_at`. Row, cell, paragraph and inline controls get location-aware wrappers on that same transaction. Clone drops `w14:paraId` and `w14:textId` through the existing freshen path and allocates a new sdt id. Move keeps the id. Remove deletes the shell and its content. A separate unwrap mode drops the shell and leaves the content in place, and refuses when that content is illegal in the parent.

Python adds the same document methods and keeps `ContentControl` as a frozen snapshot returned after the edit. It does not become a live owner in this story. That avoids a second handle inventory beside F-X260. WASM and CLI gain no method.

Child order on write stays `w:sdtPr`, `w:sdtEndPr` when present, then `w:sdtContent`. Unmodelled property children stay in their raw slots. Foreign-namespace `showingPlcHdr` lookalikes stay. Proposed content is validated against the OOXML schema of the actual placement before publication. An unsupported placement and type combination refuses and publishes nothing.

The existing `SdtOwner` grammar is that check. A block parent or a cell parent accepts a block control whose content is a paragraph or a table. A table accepts a row control whose content is a row. A row accepts a cell control whose content is a cell. A block parent accepts a paragraph control whose content is one paragraph. A paragraph accepts an inline control whose content is runs. An inline control contains valid inline content and does not contain a nested paragraph. The default inline shell is one empty run. The default block or paragraph shell is one empty paragraph, and a plain-text paragraph in that shell contains one empty run. The default row shell is one schema-valid row. The default cell shell is one schema-valid cell whose block content is one empty paragraph.

## Consumer map

| Rule | Entry points and outputs | Shared provider and caller policy | Lifecycle stages compared |
|---|---|---|---|
| One shell grammar at five placements | `Document::insert_content_control` and the Python method | `CT_Sdt` and `SdtOwner`. Callers do not write `w:sdt` | Create, save, reopen, list |
| Metadata edits keep content | Location-addressed tag, alias, lock, appearance, placeholder | Existing raw slots for unmodelled children | Before and after bytes of `sdtContent` |
| Move, clone and remove are fragment edits | Existing direct-child methods and the nested wrappers | `insert_content` transaction and id freshening | Same story, cross-story refusal, clone identity |
| Value fill still addresses the new shell | Existing tag and alias setters | F-285 does not change fill semantics | Create, fill, reopen |

Layout reads controls it can already see. This story does not change paint for a document that never calls the new API.

## Behavior contracts and evidence

| Surface or placement | Exact behavior and boundary examples | Primary source identity and location | Local discriminator or pending evidence |
|---|---|---|---|
| Block or cell parent | A block control whose content is a paragraph or a table. The default shell is one empty paragraph. A plain-text paragraph in that shell contains one empty run. | Q16. Existing `SdtOwner` grammar. Child order on write is `w:sdtPr`, then `w:sdtEndPr` when present, then `w:sdtContent`. | A1. An inline payload in this parent returns the original package. |
| Table parent | A row control whose content is one schema-valid row. | Q16. | A1. |
| Row parent | A cell control whose content is one schema-valid cell, and that cell's block content is one empty paragraph. | Q16. | A1. |
| Paragraph parent | An inline control whose content is runs. The default shell is one empty run. A nested paragraph refuses and publishes nothing. | Q16. The user correction requires inline content to be runs. | A1. |
| Kind and identity | Kind is `SdtType::RichText` or `SdtType::PlainText`. Any other kind returns an error that names F-286 and publishes nothing. The id is the smallest unused positive `i32`. Tag and alias reject non-XML characters. | Q16. `SdtType` already names the later kinds in `crates/rdocx-oxml/src/content_control.rs`. | A6. |
| Metadata, remove and unwrap | `set_content_control_tag`, `set_content_control_alias`, `set_content_control_lock`, `set_content_control_appearance` and `set_content_control_placeholder` each take a location and an `Option`. `None` clears that field and does not replace content. `remove_content_control` deletes the shell and its content. `unwrap_content_control` leaves the content only when the parent can hold it. A refusal returns `Error::Other` and the original package. Foreign-namespace `showingPlcHdr` lookalikes stay. | Q16 and Q21. Placeholder preservation is covered near `crates/rdocx-oxml/src/content_control.rs` around the placeholder fixture. | A2 and A4. |
| Python, WASM and CLI | Python receives the same document methods and keeps the frozen `ContentControl` snapshot. Destination is the existing `ContentLocation`. WASM and CLI gain no method. | Q16. `PyContentLocation` is `crates/rdocx-py/src/document.rs` line 4989. `PyContentControl` is frozen at line 5126. | A1. |

## Dependency readiness

All formal provider dependencies and shared-file leases were integrated before
this implementation. The applicable local gates and final provider riders now
pass with actual producer/source correspondence. S95.3 final release preparation
and its final sprint review remain separate, unfinished obligations.

## Acceptance criteria

| ID | Input or setup | Expected observation or refusal | Consumers | Local test | Final gate or pending evidence |
|---|---|---|---|---|---|
| A1 | Insert one control at each of the five placements in a body, a table, a header and a text box. Also insert an inline control whose proposed content is a paragraph, and one placement the parent cannot hold | Reopen lists the same placement, tag, alias, id and rich or plain kind. Inline content is runs. The package contains no caller-supplied raw `w:sdt`. Each unsupported combination returns the original package | Native `Document` and Python `Document` | `content_control_create_round_trips_each_placement` | Hash harness unchanged on fixtures that do not call the API |
| A2 | Set and clear tag, alias, lock, appearance and placeholder on an imported control with an unmodelled property child | The edited fields change. The child bytes and `sdtContent` stay. A foreign `showingPlcHdr` stays when the qualified one is removed | Native property edit | `content_control_metadata_edit_preserves_raw_slots` | Round-trip suite |
| A3 | Move and clone a block control and an inline control. Clone across a story that accepts the placement | Move keeps the id and the content. Clone has a new id and no `w14:paraId`. A cross-story move refuses and publishes nothing | Fragment transaction | `content_control_move_and_clone_keep_or_refresh_identity` | Round-trip suite |
| A4 | Remove by deleting the shell and its content. Unwrap when the parent can hold the content, and unwrap when it cannot. Also try to remove the last paragraph of a cell when that would leave the cell empty | Delete removes the shell and the content. A legal unwrap leaves the content. An illegal unwrap or removal returns the original package | Native remove | `content_control_remove_is_atomic` | Round-trip suite |
| A5 | Create a control, then `fill_content_control_tags` and a tag miss | The new tag accepts the existing fill. A missing tag still rolls back | Existing value facade and the new shell | `content_control_create_then_fill_uses_existing_setter` | Existing fill tests stay green |
| A6 | Ask for a checkbox, date or repeating section | The call returns an error naming the missing type payload and does not publish | Native insert | `content_control_typed_payload_is_refused_until_f286` | None beyond the local refusal |

## Test matrix

| Case | Modes or state combined | Expected difference or agreement | Wrong implementation rejected |
|---|---|---|---|
| C1 | Zero controls, one control, and a control nested inside another | Listing order is document order. The inner control is not dropped | A writer that flattens nested shells |
| C2 | Body, cell, header and text box | The same spec produces the placement that parent allows | A body-only inserter |
| C3 | Rich text versus plain text, at block, paragraph, row, cell and inline placements | The type element is `w:richText` or `w:text`. Inline content is runs. A nested paragraph in an inline control refuses | A plain control that still writes `w:richText`, or an inline shell that contains `w:p` |
| C4 | Create, edit, save, reopen, then fill | The id survives reopen and fill does not replace the shell | A fill path that rebuilds `sdtPr` from scratch |
| C5 | Valid remove versus a cell-emptying remove | Only the valid call changes bytes | A remove that publishes a schema-invalid cell |

WASM and CLI stay on their current read and fill behaviour. They gain no new method. Rendering is compared only to prove an untouched fixture stays byte-identical.

## Rejected alternatives

- A public raw-XML helper. The test gate forbids that fallback.
- A new `content_controls` module. The model and the facade file already exist.
- Live Python owners. F-X260 owns handle lifetime, and a frozen snapshot does not add another inventory.
- Creating every `SdtType` here. F-286 owns the payloads that make those types valid.

## Test plan

**Test gate**: round-trip. Each placement and each metadata field survives public creation and mutation with no raw-XML fallback.

| Timing | Category | Test | Asserts |
|---|---|---|---|
| local scoped | round-trip | A1 through A6 in the existing `rdocx` integration entrypoint | Placement, metadata, identity, atomic refusal |
| local scoped | regression | Existing fill and content-control parser tests | Old value behaviour and raw-slot preservation |
| final sprint | rider | full hash harness and installed `rdocx` suite | No hash delta from documents that do not create controls |

Python tests go in `crates/rdocx-py/tests/test_core.py`. No new test binary.

## HLD impact

- `docs/hld/02-scope-and-non-goals.md`, DOCX-052 status after the approved surface exists
- `docs/hld/03-architecture.md`
- `docs/hld/10-bindings-spec.md`
- `docs/hld/12-testing-strategy.md`
- `docs/hld/14-development-backlog.md`

## Risk routing

- Any parser or serialiser. Schema child order, fixed write prefix, and a round-trip that keeps an unmodelled `sdtPr` child byte for byte.
- XML creation in multiple parent contexts. The five placement rows above are the check. A block payload in an inline parent refuses.
- Public API of a published crate. The new methods are additive. Final publish dry run and crate size check.
- WASM or PyO3 bindings. Python is included. Final `rdocx-wasm` check. Python tests stay excluded from the workspace binary.
- Shared semantic behavior with multiple consumers. Create, list, fill and reopen see the same tag and text.
- Grammar, scope, compatibility mode or empty collection. C1 covers zero, one and nested controls.
- A new file or module. This plan does not add one. An implementation that wants a file stops and asks.

No external oracle. The test gate is round-trip. F-286 owns the Word differential.

## Hash harness

Unchanged. Creation must not alter the bytes of a document that does not call the new API.

## Implementation checklist

- [x] Confirm F-X259 through F-X262 have left `document.rs` and the bindings spec.
- [x] Add the failing round-trip stubs to the existing integration entrypoint.
- [x] Implement insert, edit, move, clone and remove on the existing types.
- [x] Update the HLD list and obtain a zero-finding microscope.
- [x] Complete and integrate the actual provider prefix before F-286/F-287, then accept the unchanged final source-qualified hash observation.

## Deviations

This section records the original implementation checkpoint. Accepted provider completion below supersedes earlier pending evidence and readiness statements. Historical failures and missing-input limits are retained.

The file lease named in Dependency readiness is met. F-X259 is integrated at `4879e58a`, F-X260 at `883080bb`, F-X261 at `6adb7258`, and F-X262 at `8c14bd3c`. That last commit is the parent of claim `2a6078ce`. The readiness sentence stays as written. No product contract changed.

There is no RED commit. The new methods did not exist, so compile-time absence is not a semantic failure. The six integration tests and the Python shell test are fixed-source checks.

Block and paragraph placement insert at a direct-child boundary of a block story or a table cell. A paragraph item means insert before that item. Row placement takes a table item whose path length is 1. Cell placement takes a table path of length 2 and splices into that direct row. Inline placement splices into a paragraph item. Q21 has no content argument, so an inline shell whose content is a paragraph is covered by the parent mismatch and by unwrapping an imported inline control that already holds a paragraph.

Story-item classification inherits ancestor prefix bindings, including a document-level `w14` binding, so a reopened control with `w14:paraId` stays typed. The scan does not call `story_namespace_scope_at`.

The accepted B2 remediation also exposes Python move_content_control(source, destination) and clone_content_control(source, destination), each returning None through checked native publication. Rust returns Result<()>. Move retains ids, clone refreshes ids, and incompatible placement refuses atomically. `Document::content_controls` remains the body inventory. Header and text-box controls are read through story items.

DOCX-052 is complete for rich and plain shells. Typed payloads stay on DOCX-053. The owner exclusion in `scripts/test_sprint_workflow.py` records that split.

## Open questions

| ID | Affected F-IDs and A-IDs | Decision and recommendation | Status and answer/source |
|---|---|---|---|
| Q16 | F-285 A1 to A6. F-286 and F-287 consume the shell. | Create rich and plain shells at the five placements. Validate each placement's content model. Inline content is runs. Unsupported combinations publish nothing. Python keeps a frozen snapshot. WASM and CLI stay unchanged. Remove deletes the shell and content. Unwrap refuses a result the parent cannot hold. Ids are the smallest unused positive `i32`. | Answered 2026-10-18 by the user. |
| Q19 | F-285, F-286 and F-287 | F-285 completes its dependency-prefix checkpoint before F-286. F-287 starts after F-286 leaves the shared files. | Answered 2026-10-18 by the user. |
| Q21 | F-285 A2 and A4. F-286 and F-287 consume the names. | Python `insert_content_control(destination, placement, kind, tag=None, alias=None, lock=None, appearance=None, placeholder=None)`. Rust takes the same fields in `ContentControlSpec`. The setters are `set_content_control_tag`, `set_content_control_alias`, `set_content_control_lock`, `set_content_control_appearance` and `set_content_control_placeholder`. Each takes a location and `Option`, and `None` clears that field. `remove_content_control(location)` and `unwrap_content_control(location)` use the same names in Python. Rust returns `Result<()>`. A refusal returns `Error::Other` and does not publish. | Answered 2026-10-18 by the user. |

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


## Pass4 B2 amendment

The existing destination-content contract also applies to admitted context-dependent
customXml wrappers. A valid table-row wrapper must not become inline content
by relocation. Row, cell, block and inline payloads, metadata-bearing wrappers
and alternating customXml/sdt nesting discriminate name-only admission. Move,
clone and unwrap retain compatible content and identities. Incompatible move
and clone refuse with identical package bytes. F-286 repeating conversion
applies the same inherited block grammar. This is the existing invariant,
not general imported-document validation or a new authoring API.

Actual old hosted native5c2997d2 supplies the new public regression RED.
Local amendment checks now pass: native710library,407ordinary integration and
four separately executed pinned-viewer integration cases, plus two doctests.
Earlier whole commands remain failed, not relabeled. The final rebuilt native
3958f3a4 passes the full1064Word suite, priorities and typing/stubs. Both Python
interpreters pass the final matrix, including exact customXmlPr retention
after save/reopen. Direct-body insertion and inline insertion are distinct
fixture contexts. BothWASM and scoped clippy pass. The focused independent
microscope, updated genuine verified archive and actual hosted artifacts
remain pending. Original completed delivery entries remain historical. Final
sprint acceptance remains blocked until the amendment and qualifications.


Independent custom-XML placement microscope pass1 reports zero defects, smells
and nitpicks. It also passes18bounded decoded-boundary probes on the final
native. Fresh49hashes, seven deterministic pixels and README/package inventory
checks pass. Policy qualification joins186sandbox passes with four blocked
methods passing externally, plus the two existing skips. The original failed
commands remain failed. Commit this reviewed amendment for a genuine clean
verified rdocx archive and build-only hosted source qualification. Those final
artifact gates and independent sprint pass5 remain pending.

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
