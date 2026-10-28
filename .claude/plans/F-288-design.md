# F-288, Legacy form field creation

**Status**: completed
**Sprint**: S95.4
**Size**: M
**Depends on**: F-237, F-278
**Base**: a57be365 on sprint/s95.4
**Design date**: 2026-10-23

## Execution boundary, 2026-10-27

The API, preservation, refusal and acceptance contracts below remain binding. Problem descriptions, approval-time unrun statements and local provider-prefix checkpoints retain their historical source and scope. They do not describe the final integrated execution state. At this checkpoint, the native union and affected corrective cases, installed Python suites and typing, native Word and PowerPoint qualification with explicit source applicability, A7 measured memory and retained-history review, A9, the original Issue407 gate, hash49, lint, WASM, rustdoc, supply chain and policy observations are available. Actual producer identities remain attached to each observation. No old binary is relabelled as a new build.

The remaining integrated acceptance debt at this checkpoint is Cell.add_picture's required flat-cost gate, the active README example batch, the clean22-package publication dry run and the final full-union record. The original failed attempts remain evidence. Status stays approved until the required final acceptance passes. When this plan becomes completed, its final source-bound delivery entry in docs/sprints/AS_BUILT.md supersedes these historical execution-status statements. Completion changes neither the contract nor any threshold. Build-only hosted wheel qualification belongs to release preparation before final release approval. The subsequent F-X270 intake prepares fresh0.16.4/0.14.4 patches within this sprint because the old0.16.3/0.14.3 candidates were already published. It is not an additional acceptance criterion in these eight sprint designs. Publication remains separately approved at the exact reviewed main SHA.

## Problem

Public callers can read and fill existing legacy form fields, and they can attach a generic `Field`, but they cannot author a complete text, checkbox, or drop-down legacy field with help, status, defaults, formatting, size, list, and calculation properties. `Field::from_legacy_form` does not exist. The current text projection prefers `w:default` over the result cache (`crates/rdocx-oxml/src/text.rs:4523`). The existing value rewriter writes the new value into `w:default`. That conflates the persistent reset default with the current typed or cached value. `ComplexFieldBuilder::separate_run` (`crates/rdocx-oxml/src/text.rs:5227`) is the physical fact for a present outer result region. The current writer emits an empty begin at `crates/rdocx-oxml/src/text.rs:12632` and has no `ffData` integration. Order and validation already live at `crates/rdocx-oxml/src/text.rs:4820`. Attachment already exists at `Run::add_field_value` (`crates/rdocx/src/run.rs:521`) and `Document::insert_field` (`crates/rdocx/src/field.rs:635`). Inventory and fill remain `Document::legacy_form_fields` and `Document::set_legacy_form_field_value`.

Word 16.114 (16.114.26101123) is installed. At the 2026-10-23 approval, no new authentic Word comparison had run. Final-union V4c authoring and the approved four-input library consumer later ran on this exact Word build. Their actual evidence is retained below. The later normal c62 six-file read qualification restores ten fields. Its empty-cache setter defects were repaired, and the six-file A6 setter qualification passed at ef048ffb in `/private/tmp/s954-word-qualification-ef048ffb/native-a6/result.json`. The earlier failures and bounded remediation below remain attributed to their original source. Exact final-artifact applicability remains pending. Successful file production alone never supplies setter qualification. S95.3 evidence does not qualify this authoring. Issue #381 is a separate later AcroForm plan (body SHA `f7d9cbcf79f954046b4e0f7f4bf8da3d8ba8ee85559587dcf6cf6aa966eabcd6`, updated 2026-10-15T19:26:15Z). Intake on 2026-10-23 is read-only: 4 open issues, 0 open PRs, no S95.4 acceptance work.

## Spec reference

- `docs/hld/14-development-backlog.md`, "F-288, Legacy form field creation" (lines 3133-3138). Test gate copied below.
- `docs/hld/03-architecture.md`, "What stays put" and "Facade conventions".
- `docs/hld/04-opc-and-packaging.md`, "The package", including Q25 exact outer/child source and both physical bookmark markers.
- `docs/hld/10-bindings-spec.md`, "Native Word facade stability".
- `docs/hld/12-testing-strategy.md`, "Test taxonomy" and "Typed content-control native Word qualification".
- `.claude/skills/differential-testing.md`, oracle pin, self-generated fixtures, and composition checks.
- Open XML SDK 3.0.1 [`FormFieldData`](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.formfielddata?view=openxml-3.0.1), [`TextInput`](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.textinput?view=openxml-3.0.1), [`CheckBox`](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.checkbox?view=openxml-3.0.1), and [`DropDownListFormField`](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.dropdownlistformfield?view=openxml-3.0.1). These pages are primary API names. They are not a passed Word comparison.
- Local `Word.sdef` form help, status, check, date, and dropdown scripting names. They are not a passed comparison.
- Existing parser span `crates/rdocx-oxml/src/text.rs` lines 4304-4855.

## Approach

Work only in existing files. Re-export new types from the `rdocx` facade. No second attachment route, module, trait, generic, feature, or test binary.

```rust
pub enum LegacyFormInfoText { Text(String), AutoText(String) }
pub enum LegacyTextInputType { Regular, Number, Date, CurrentTime, CurrentDate, Calculated }
pub enum LegacyCheckBoxSize { Auto, HalfPoints(u64) }
pub struct LegacyFormFieldOptions {
    pub name: Option<String>,
    pub enabled: bool,
    pub calculate_on_exit: bool,
    pub help_text: Option<LegacyFormInfoText>,
    pub status_text: Option<LegacyFormInfoText>,
}
pub enum LegacyFormFieldDefinition {
    TextInput { input_type: LegacyTextInputType, default: Option<String>, value: String, max_length: Option<usize>, format: Option<String> },
    CheckBox { size: LegacyCheckBoxSize, default: bool, checked: bool },
    DropDownList { choices: Vec<String>, default_index: usize, selected_index: usize },
}
impl Field {
    pub fn from_legacy_form(options: LegacyFormFieldOptions, definition: LegacyFormFieldDefinition) -> rdocx_oxml::Result<Self>;
}
```

`Default` for `LegacyFormFieldOptions`: `name` is `None`, `enabled` is true, `calculate_on_exit` is false, help and status are `None`.

Do not add `default_value` to `LegacyFormFieldData` or `LegacyFormFieldInfo`. Both public struct shapes stay unchanged. Q15, answered by the user on 2026-10-23, adds queries only.

```rust
impl Field {
    pub fn legacy_form_default_value(&self) -> rdocx_oxml::Result<Option<LegacyFormFieldValue>>;
}
impl Document {
    pub fn legacy_form_fields_with_defaults(
        &self,
    ) -> Result<Vec<(LegacyFormFieldInfo, Option<LegacyFormFieldValue>)>>;
}
```

`Field::legacy_form_default_value` reads explicit reset-default metadata. `None` means that metadata is absent. Existing `legacy_form_value` still distinguishes a non-legacy field. Effective reset fallback stays `Text("")`, `Checked(false)`, or `SelectedIndex(0)`. `set_legacy_form_field_value` changes the cache and current value. It never rewrites the reset default. No new default setter and no second single-field default query are promised. F-289 reset uses the precomputed batch pair and the known fallback.

`Document::legacy_form_fields_with_defaults` is one additive native batch snapshot. It uses the same compatibility ordinal inventory walk as `legacy_form_fields`. The old method keeps its return shape and signature and delegates to that private inventory. The new batch does not walk a second time. Python `legacy_form_fields` converts the fused pairs and freezes the default on the existing `LegacyFormField`. The readonly `.default_value` property stays.

Private `Field` author and source metadata is the only default authority. There is no new public metadata field and no second form tree. The shared parser preserves accepted import states, including an out-of-list explicit default. A valid current import with an invalid reset index does not gain a new read refusal. New factory and reset payloads still validate actual choice bounds. Malformed raw default reading follows the bounded parser and leaves accepted raw states untouched.

Authoritative current read. At approval this display and no-op closure was proposed and not yet implemented. The local worker now has that closure. Keep the raw `Field.cached_result` field as the physical stored cache. Never return early because the typed current value already matches. The controlled setter computes the canonical desired display and its metadata. The final candidate bytes decide the no-op.

```rust
impl Field {
    #[doc(hidden)]
    pub fn legacy_form_value(&self) -> rdocx_oxml::Result<Option<LegacyFormFieldValue>>;
    pub fn effective_display(&self) -> std::borrow::Cow<'_, str>;
    pub fn effective_projected_text(&self) -> Option<std::borrow::Cow<'_, str>>;
    pub fn effective_display_field_segments(
        &self,
    ) -> Vec<(std::borrow::Cow<'_, str>, Option<&CT_RPr>, &Field)>;
}
```

`effective_projected_text` keeps the complex-field visibility predicate already used by `Field::projected_text` at `crates/rdocx-oxml/src/text.rs:1667`. These three methods are additive. The existing borrowed signatures stay explicit physical stored-cache projections:

```rust
impl Field {
    pub fn projected_text(&self) -> Option<&str>;
    pub fn cached_display_field_segments(&self) -> Vec<(&str, Option<&CT_RPr>, &Field)>;
}
impl FieldRef<'_> {
    pub fn cached_display_segments(&self) -> Vec<FieldDisplaySegmentRef<'_>>;
}
```

Those three live at `text.rs:1667`, `text.rs:1707`, and `crates/rdocx/src/run.rs:319`. Do not change their borrowed return types.

Private effective cache policy owns default fallback, actual result presence, observable parent overrides, and nested children. Private explicit default and presence metadata may still return borrowed strings. Nested concatenation uses `std::borrow::Cow::Owned`. An observable parent `cached_result` override wins. Otherwise the segments use effective nested children. `effective_display_field_segments` keeps the actual child owner and that child's `CT_RPr`. No new module, file, generic, or trait.

No-op formula. On an absent result, force a result region when the desired canonical display differs from the raw stored cache, or when the requested typed current differs from the pre-edit effective current. A default fallback with a raw empty cache materializes that fallback when the setter asks for the fallback. A setter that asks for empty materializes empty. An absent default with a raw empty cache and a setter that asks for empty is the existing byte no-op. A typed empty cache that is an explicit publication always materializes. The same checkbox or list typed value can repair a custom or wrong display. An F-289 assignment of the same current value can still repair bound state and display, and this no-op must not skip that repair. The private presence flag participates in raw `is_unchanged` and in the shared selection. Run-aware materialization preserves unknown bytes and shared run content. A factory complex field always has a result region, even when that region is empty.

Checkbox and drop-down typed checked or selected metadata is the authority. Never infer a checkbox from a glyph. A cached custom display is legal. The explicit default does not change because a cache edit happened. The existing public `LegacyFormFieldData.value` projection is documented as limited after arbitrary direct edits. Controlled setters sync it. Do not invent a refusal when a custom display disagrees with the typed state. Refuse only kind or structural contradictions. A raw stored cache that remains empty under the byte no-op is intentional physical state. It is not a stale view of the effective text. Semantic readers use the effective methods.

An admitted own-bookmarked field keeps pure begin and end marker runs when an empty physical result is materialized. If publication reaches the end before writing a cache, a dedicated result run precedes that unchanged end run. The result inherits its namespace scope and formatting, while non-namespace run owner attributes remain on the original. This preserves the existing parser boundary rather than broadening admission or returning early on typed equality. The facade compares the ordered legacy owner descriptors before and after reopen, including part, ordinal, name and kind, before accepting current value. The pre-edit inventory error is deferred until the existing requested mutation checks finish. Ordinary field counts are not an invariant because literal assignment may remove a Q25 result child.

Semantic consumers route through the effective policy. `CT_R::text` uses the field arm at `text.rs:2870`, which today calls `projected_text`. Layout character offsets at `crates/rdocx-layout/src/engine.rs:2518` today call `projected_text`. Layout segments at `engine.rs:10238` today call `cached_display_field_segments`. The note-cache fallback at `engine.rs:3368` today builds a run from `field.cached_result` when `cached_result_runs` is absent. The new legacy current getter, the inventory, and the CLI read the actual current policy. The new source claim is the existing `crates/rdocx-layout/src/engine.rs` file for those three consumers. Do not add a file.

Do not globally rewrite physical `cached_run_display` at `text.rs:2042`. That function reads `field.cached_result` so parser cache lengths and source offsets keep physical data. Use an effective equivalent only where the consumer is semantic.

Literal replacement stays on the existing physical character map. `Document::replace_batch` at `crates/rdocx/src/document.rs:36514` calls `replace_in_paragraph` at `document.rs:36537`, which calls `rdocx_oxml::placeholder::replace_in_paragraph`. `build_char_map` at `crates/rdocx-oxml/src/placeholder.rs:552` records only `RunContent::Text`. Simple-field physical wrappers enter at `placeholder.rs:412` and write back at `placeholder.rs:714`. Do not add a virtual-default replacement. Those fallback characters are not source bytes. Effective extraction shows fallback text to native text, layout, and PDF. Literal replacement of that fallback returns 0 and leaves the bytes unchanged. A typed legacy setter of empty materializes an empty result and an empty current, and it preserves the default, on a fresh document, after reopen, and through the CLI.

Publishers that must be mapped: direct `cached_result`, `set_cached_runs`, annotation cache, `cached_field_mut`, refresh, attachment, and reopen. Add a discriminating case for a parent override plus a changed child, and direct, typed, and nested cases. Native inventory and downstream getters are the accessor. Python keeps frozen and fresh snapshots. There is no new low-level Python getter.

Inventory fill covers every currently attachable story, including comments and text boxes. Per part, old ordinal prefixes stay. Append newly discoverable physical begin spans in part source order. Dedup a root that is also reached as a cell or text box. Comments use a fresh part sequence. The same frozen ordinal to physical map is used for read and write. Rename any pure source-order claim to compatibility order. Tests cover a text box that precedes an old body field, a related text box, a shared header, and comments, with no renumber and no wrong owner.

Factory output is one complex field whose instruction matches `FORMTEXT`, `FORMCHECKBOX`, or `FORMDROPDOWN`, with typed initialized caches. Character formatting stays the existing `CT_RPr` on the run. `ffData` is written only on the complex begin `fldChar`. Simple, separate, and end marks do not carry it.

Child order reuses the owner at `text.rs:4820`: common `name`, `enabled`, `calcOnExit`, `entryMacro`, `exitMacro`, `helpText`, `statusText`, then exactly one kind. Text children are type, default, maxLength, format. Checkbox children are `size` or `sizeAuto`, then default, then checked. List children are result, default, then `listEntry` items. Imported entry and exit macros and unknown bytes stay verbatim. This story does not execute macros and does not author new macros.

Schema fact from the pinned Open XML SDK v3.0.1 schema at `https://raw.githubusercontent.com/dotnet/Open-XML-SDK/v3.0.1/data/schemas/schemas_openxmlformats_org_wordprocessingml_2006_main.json`. `CT_FFData` / `w:ffData` is around line 30857. That model is a Choice, not a global `xsd:sequence`. The existing bounded parser and the chosen canonical author order remain the owner. This plan does not claim that the canonical common `ffData` order is the normative schema's only valid ordering. Native changed writers follow that chosen existing canonical order and preserve untouched producer bytes. Kind child content models still match the exact schema.

Bounds, refused before attachment, with no destination mutation:

- Name at most 20 Unicode scalars.
- Help at most 255. Status at most 140. Default at most 255. Format at most 64.
- `max_length` is `None` (omit) or 1 through 32767.
- Drop-down has 1 through 25 entries. Each label is at most 255 scalars. Duplicate labels are legal. Indices are zero-based and inside the list.
- Strings are XML 1.0. Explicit empty help or status is distinct from `None`. `AutoText` is an explicit selector.
- `HalfPoints(n)` stores the exact integer `n`. `HalfPoints(25)` is 12.5 pt, with no float conversion. `legacy_form_hps_measure_value` at `text.rs:5026` accepts `parse::<u64>()` for every integer from 0 through `u64::MAX`, and it also accepts imported universal measures (`mm`, `cm`, `in`, `pt`, `pc`, `pi`). `HalfPoints(u64)` therefore admits 0 through `u64::MAX`. Imported universal-measure checkbox size stays untouched. The new enum controls only newly authored data. Parser acceptance does not prove native Word behaviour for zero or extreme sizes.
- A manual `Field` whose public metadata contradicts the kind is refused before attachment.

Placements are the two existing routes only: direct main, table, and nested inline controls, plus supported related stories those routes already allow. Unsupported insertion contexts stay unsupported. Ordinals stay physical and part-scoped. Stale, missing, and wrong-kind targets are rejected. No stable mutable handle is added. Publication stays atomic under existing live-handle rules.

Q3 selected answer, user answer on 2026-10-23: store settings and let Word execute them. Author all six text kinds (`Regular`, `Number`, `Date`, `CurrentTime`, `CurrentDate`, `Calculated`), format strings, and `calcOnExit`. The caller supplies typed values and cached displays. Existing `update_fields` preserves `FORM*` state and keeps its current unsupported-evaluation diagnostics. This crate adds no formula, macro, or locale evaluator in Rust or Python. The historical evaluator alternative stays rejected.

## Python API contract

One frozen public `LegacyFormFieldDefinition` contains the checked native `Field`. There is no generic `Field` grammar wrapper and no new Python XML path. The class is registered on public `rdocx` and `rdocx._rdocx`. `_rdocx.pyi`, `__all__`, and `__init__` imports stay aligned. The existing type-factory private constructor stub is `__new__(cls, *, _private: _Never) -> LegacyFormFieldDefinition`. That return is the class, so the static factories stay callable. Fields are readonly. A readonly definition is reusable. Attachment clones the native `Field`.

Shared error and lifetime contract. Strict wrong Python shapes and types raise `TypeError`. Negative or out-of-domain primitives raise `ValueError`. Nonnegative integers outside the `u32`, `u64`, or `usize` extraction range raise `OverflowError`. Bool is forbidden for numeric indices, counts, and sizes. Native semantic, XML, and package errors keep the existing `rdocx_to_pyerr` at `src/lib.rs:117`. Oxml maps to `XmlError`, package and io map to `PackageError`, and Other and Story map to `RdocxError`. Do not introduce another error hierarchy and do not convert every native error to `ValueError`. Wrong-kind and out-of-list-domain native validation uses that existing mapping. Factories validate in native code before return. All new method results, tuples, PyObjects, and revision tokens are prepared before publishing a candidate. Refusal and no-op preserve bytes, revisions, and live handles. Changed publication follows the existing physical owner policy and never revives dead owners. Owned old inventory snapshots remain old. Structural insertion uses the existing conservative publication.

```python
class LegacyFormFieldDefinition:  # frozen
    @staticmethod
    def text_input(
        *,
        value: str = "",
        default: str | None = None,
        input_type: Literal["regular", "number", "date", "currentTime", "currentDate", "calculated"] = "regular",
        max_length: int | None = None,
        format: str | None = None,
        name: str | None = None,
        enabled: bool = True,
        calculate_on_exit: bool = False,
        help_text: tuple[Literal["text", "autoText"], str] | None = None,
        status_text: tuple[Literal["text", "autoText"], str] | None = None,
    ) -> LegacyFormFieldDefinition: ...
    @staticmethod
    def checkbox(
        *,
        checked: bool = False,
        default: bool = False,
        size_half_points: int | None = None,
        name: str | None = None,
        enabled: bool = True,
        calculate_on_exit: bool = False,
        help_text: tuple[Literal["text", "autoText"], str] | None = None,
        status_text: tuple[Literal["text", "autoText"], str] | None = None,
    ) -> LegacyFormFieldDefinition: ...
    @staticmethod
    def drop_down_list(
        choices: Sequence[str],
        *,
        selected_index: int = 0,
        default_index: int = 0,
        name: str | None = None,
        enabled: bool = True,
        calculate_on_exit: bool = False,
        help_text: tuple[Literal["text", "autoText"], str] | None = None,
        status_text: tuple[Literal["text", "autoText"], str] | None = None,
    ) -> LegacyFormFieldDefinition: ...

class Run:
    def add_legacy_form_field(self, definition: LegacyFormFieldDefinition) -> None: ...

class Document:
    def insert_legacy_form_field(self, position: StoryRunPosition, definition: LegacyFormFieldDefinition) -> None: ...

class LegacyFormField:
    @property
    def default_value(self) -> str | bool | int | None: ...
```

`calculate_on_exit` defaults to `False` on every factory. That matches the approved native `LegacyFormFieldOptions` default. The earlier agent recommendation of `True` is not used. Defaults are the actual `inspect.signature` values. Use the existing PyO3 `text_signature` where needed so a no-argument form is `()` and not `Ellipsis`.

Help and status tuples are inline. No additional public alias is required. `("text", "")` is explicit empty text and differs from `None`. A bad tuple or tag is `TypeError` or `ValueError` under the shared mapping. `size_half_points is None` means Auto. An integer half-point size is `0` through `u64::MAX` inclusive, and bool is rejected. Indices are `0` through `usize::MAX`, and native checks enforce the actual choice bounds. `choices` is copied. Duplicates are preserved. `str` and `bytes` are rejected as a sequence of choices.

`Run.add_legacy_form_field` delegates to native `Run::add_field_value`. `Document.insert_legacy_form_field` delegates to native `Document::insert_field`. `StoryRunPosition` is already public. Paragraph-based positions do not admit cell paragraph handles. Table and related insertion use the existing `item: StoryItem` form at `document.rs:7505`. There is no new position carrier.

Disposition. Every new native factory, options, and definition variant is exposed through these static factories and keyword arguments. Generic native `Field` raw construction gets no new Python exposure. Existing `Run.add_field` remains unchanged.

Old snapshots freeze. A new inventory shows current and default matching native and reopen. The Python `default_value` property is readonly.

Python tests in existing `tests/test_core.py`: `test_authored_legacy_form_fields_keep_defaults_and_current_values`, `test_legacy_field_factories_and_attachment_refuse_atomically`, and `test_legacy_field_definition_public_factories_are_typed`. They cover all three public factories, both attachment routes, nested and related `StoryItem`, held frozen, fresh, and reopen, the default and current discriminator, strict bool and indices, help tuples, factory invalid and late attachment byte atomicity, the actual factory class import, typing, and observable defaults. Typed public usage goes through the existing `crates/rdocx-py/tests/typing_smoke.py`. Do not add a temporary string runner. No new tracked file.

Source files for the binding are the existing `src/document.rs`, `src/run.rs`, and `src/lib.rs` under the Python crate, plus `python/rdocx/__init__.py`, `python/rdocx/_rdocx.pyi`, and `tests/test_core.py`. Native and HLD claims retain the files already listed in this plan.

The historical Q4 user answer on 2026-10-23 listed five named local native groups. The active provider checkpoint covers every named local native group in this test plan, including A7 and `cross_consumer_render_fallback_before_empty_assignment`, plus the affected Python factory, attachment, current, and default checks, with scoped verification and an independent clean microscope. The gate is not weakened to the original five. Those local groups and the clean microscope exist. Integration commit `96718dadf6840fe6ff73663dff3cd2b286418844` makes source head `0c5c2a5d7fe9a25bb8445cc984a9665482d4726e` the ancestor for F-289. Do not leave only three native groups unchecked, and do not leave A7 or the Python bindings unchecked.

Native Word names legacy fields with a bookmark start immediately after the pure begin run and a matching end immediately after the pure end run. Recognize only one isolated valid legacy field with its exact own-name, unambiguous pair. Preserve the physical start inside the field source and the end outside. Project direct, accepted and tracked bookmark endpoints around the virtual field run using their actual preceding counts. The actual paragraph namespace frame remains with that projection, including different marker aliases, foreign opaque names and the default namespace. Local declarations still shadow it. Bindings conflicting with current generated inline writer prefixes decline the new projection before any run changes. Other interior ranges, partial or crossing pairs, duplicate IDs, mismatched names and shared boundary-run content retain the existing untyped fallback. Nesting also retains that fallback except for the bounded Q25/Q27 ordinary child exception below. Factories do not acquire automatic bookmarks.

Typed rich mail merge can turn a fragment bookmark into an authored `MailMerge1` identity. If later authored-ID canonicalization reaches a legacy-bearing paragraph, use the existing whole-XML bookmark patch and reparse path so both physical markers change together. Ordinary preserved native IDs remain excluded from authored renumbering. The native original, current/default setters, bookmark and REF text, untouched/current-edited save/reopen, marker counts, accepted/tracked axes and fragment import are separate discriminators. This is the bounded native A6 remediation. It adds no public API, evaluator, retained cache or resource budget.


### Q25 and Q27, bounded native ordinary children

The user answered "Allow bounded nested results (Recommended)" on 2026-10-27. The exact decision is retained at `/private/tmp/s954-q25-approved-contract.json`, timestamp 2026-10-27T00:40:53.668202+00:00. This replaces only the blanket nested-field exclusion for the following shape. It does not revoke the genuine integrated Q4 local provider-prefix completion or any unchanged approved choice. Q25 remediation and native A6 closure are pending.

Admission requires one isolated parsed complex `FORMTEXT` field with a valid legacy `TextInput` owner and the existing exact own-name bookmark pair. There is exactly one total child. Q25 admits one direct cached-result child with no instruction child. Q27 alternatively admits one direct Nested instruction argument, with no additional literal argument, nested switch argument or cached-result child. Existing scalar switches remain. That child is a parsed complex ordinary leaf named `TIME`, `DATE`, or the existing formula class, including native no-space `=2+3`. The child has no legacy owner or malformed legacy sidecar, no further descendants, and no additional typed or private interior owners. Keep all current pure outer begin/end, unique pair, no crossing/interior ranges, namespace-frame, reserved-prefix, raw-source and direct/accepted/tracked-axis checks. Do not correlate field names or the stored text subtype with child kind. Real native Word originals have subtype-name swaps.

A simple `fldSimple` child, two or more children, children in both regions, deeper nesting, an extra literal instruction operand, a nested switch argument, another ordinary kind, nested legacy, a checkbox/list outer, malformed owner, shared outer boundary content or ambiguous/private marker still takes the untyped raw-preserving fallback. Unknown attributes and formatting already preserved by the supported parser/writer stay preserved. This exception grants no new XML placement, namespace rebinding, field grammar or cache proof.

Current getters use existing effective cached-result projection in the result variant. In the instruction variant, the outer saved w:t current stays authoritative, including `5.00` rather than inner instrText `5`. The existing parser preserves the inner instrText raw and does not model it as a child cached display. No new instrText interpretation is added. The pinned native originals restore their outer saved time/date/`5.00` strings. Their explicit defaults remain `None`, `None`, and `=2+3`. Inventory lists the outer legacy owner once and does not count the ordinary child as another legacy field. The real native fixture must list ten, not seven or thirteen. Later part-scoped ordinals change when those three missing owners are restored. Python snapshots remain frozen and report the same existing text kind and current/default types. A dropdown's absent explicit default still reads `None` with the existing effective index-zero reset fallback.

Inspection, untouched save and reopen preserve the child, original source identity and raw nested instructions. Same-current assignment still uses candidate-byte equality and must preserve a true no-op. A changed explicit parent assignment uses the existing literal-cache replacement contract. It may remove a modeled cached-result child while materializing the caller's literal current. An instruction child and its raw configuration stay preserved. Both variants retain outer reset metadata, unknown surrounding XML and both physical bookmarks. A removed child has no identity-preservation promise after that replacement. Do not claim Word's `FormFields.Result` removes the same inner XML, because that native setter comparison has not run and is not an additional mandatory A6 gate. Q3 still excludes evaluated-output parity.

An actual child-cache edit retains the original child object/source ID and uses the existing nested source writer. Replacing a child with a new object of identical instruction text does not grant source identity. The outer physical bookmark start stays inside retained raw field source and the end outside. Semantic start adjustment subtracts only the pure outer begin contribution before the existing span remap, using each axis's actual preceding counts. Numeric source IDs need not survive a fresh parse.

Existing ordinary `update_fields` evaluation and materialization policy applies to the newly projected ordinary child. Instruction-child updates do not replace the outer saved current. Cached-result children retain their existing display refresh policy. Native no-space `=2+3` retains its existing evaluator dispatch or unresolved behavior. No new evaluator, implicit clock, legacy formatting or `CalculateOnExit` execution is added. Newly visible child evaluation records and update counts may increase. Compare explicit-context, unresolved-context, locked and parent-override behavior against an otherwise identical already-supported unbookmarked control. The default and both physical bookmarks remain, even when the ordinary child cache changes. Fields stay in the F-X268 full/private-fact fallback. No writer/read cache admission or budget changes.

The independently Word-created native inputs are `/private/tmp/s954-native-v4c-observation/legacy-word-original.docx` SHA `ff864c00770971f9ac379081f86d096e14ebc1bbf6125a6d6213378d264f3168` and `legacy-word-updated.docx` SHA `e0b1eabec32cc86918dd1fa404c2da897e0c438fcc40f6ae4c5e18b2ae1badd6`. Their native inventory is ten, while the authenticated installed 398a artifact returns seven, omitting CurrentTime, CurrentDate and Calculated. This actual semantic RED is retained in `/private/tmp/s954-f288-a6-public-read-result.json`. `/private/tmp/s954-f288-native-a6-comparison.md` and its JSON bind the stages, source/output hashes and successful four-input native consumer run. Four library-created consumed outputs each retain ten through library save/reopen, and checkbox vectors match native originals. Those subchecks remain valid, but they do not close the two failing native-original reads or claim byte identity/evaluated parity.

The existing ordinary-DATE negative in `partial_or_ambiguous_legacy_bookmarks_keep_the_untyped_source` intentionally encoded the prior blanket rule. Q25 explicitly authorizes moving that exact eligible shape into a named positive. Retain the excluded-shape negative matrix. Construct meaningful positive XML source cases against the old algorithm before repaired GREEN, preserving the independently native 398a RED too. Use existing `text.rs`, the native integration entrypoint and `tests/test_core.py`, not a new module, test binary or binary fixture.

| Q25 publisher or reader | Required closure | Refusal or compatibility discriminator |
|---|---|---|
| Parse, untouched write and reopen | One outer and one complex leaf child, exact raw source, both physical markers and all three axes | Zero remains old non-nested behavior. Multiple/deep/simple/instruction/nested-legacy/private shapes remain untyped |
| Inventory/defaults, Python and CLI | Ten native legacy owners in physical part order. Actual cached current and explicit defaults. Held snapshots remain old | No double count of ordinary child, no synthesized default, no hidden ordinal compatibility change |
| Text/layout/PDF and REF | Existing effective child text/properties and bookmark text select the same owner | No virtual-default literal replacement or new evaluator |
| Explicit parent current setter | Candidate-byte no-op, then literal replacement for changed assignment. Defaults/markers preserved | Late error is atomic. Same-current no-op cannot discard a child |
| Child cache and ordinary update | Original child identity, actual source correspondence, explicit/unresolved/locked/override comparison | Equal instruction text cannot authorize a replacement child |
| Authored bookmark remap and caches | Existing whole-XML patch changes both physical IDs. Fields retain private/full fallback | Rich `MailMerge1` composition, aliases, default/local scopes and unchanged neighbors |

## Consumer map

| Rule | Entry points and outputs | Shared provider and caller policy | Lifecycle stages compared |
|---|---|---|---|
| Author one legacy field | `Field::from_legacy_form`, then `Run::add_field_value` or `Document::insert_field` | Caller owns options and definition. Factory owns XML order through `text.rs:4820` | Create, serialize, reopen |
| Distinct default and current | `Field::legacy_form_value` reads current. `Field::effective_display(&self) -> std::borrow::Cow<'_, str>`, `Field::effective_projected_text(&self) -> Option<std::borrow::Cow<'_, str>>`, and `Field::effective_display_field_segments(&self) -> Vec<(std::borrow::Cow<'_, str>, Option<&CT_RPr>, &Field)>` are the semantic reads. `Field::legacy_form_default_value` reads an explicit default. `legacy_form_fields_with_defaults` is the one batch. `legacy_form_fields` keeps its old signature and delegates. `set_legacy_form_field_value` writes current and cache only | Parent `cached_result` override, else effective nested children. Glyph is not checkbox or list authority. Candidate bytes decide the no-op. Physical `projected_text`, `cached_display_field_segments`, and `FieldRef::cached_display_segments` stay borrowed stored projections | Update, serialize, reopen |
| Effective versus literal | `CT_R::text` at `text.rs:2870`, `engine.rs:2518`, `engine.rs:10238`, `engine.rs:3368`, inventory, and CLI | Effective fallback is visible. `cached_run_display` at `text.rs:2042` stays physical. `replace_batch` at `document.rs:36514` through `placeholder.rs:552`, `412`, and `714` does not invent fallback bytes | Read, layout, render, replace |
| Field update beside forms | Existing `update_fields` | Ordinary fields may change. `FORM*` state stays | Update, serialize |
| Read and fill | Existing inventory and `set_legacy_form_field_value` | Part plus ordinal snapshot. Wrong kind refuses | Update, reopen |
| Python, WASM, CLI | Q1 selected answer: new Rust and Python authoring APIs. The exact surface is `## Python API contract`. WASM and CLI gain no new authoring methods. Existing `Run.add_field` stays unchanged. Existing Python fill, WASM, and CLI consumers retain package preservation and current-value behavior | Frozen `LegacyFormFieldDefinition` factories, both attachment routes, exports, stubs, and the named Python tests. Codex root approved this contract on 2026-10-23. The local worker has that surface. Final gates stay pending | Serialize, fill, reopen |
| PDF text | Existing deterministic PDF text extraction | A7 reads extracted text. The unchanged low-level PDF backend is not legacy qualification by itself. Issue #381 remains a separate later AcroForm plan. A source audit found no DOCX legacy-default to AcroForm consumer, so no AcroForm runtime pass is claimed and that future exporter is outside F-288 acceptance | Render |

## Behavior contracts and evidence

| Surface or placement | Exact behavior and boundary examples | Primary source identity and location | Local discriminator or pending evidence |
|---|---|---|---|
| Text input | Six kinds. `default: None` omits `w:default`. `value: ""` with a separate run stores an empty current result. Format `None` omits format. `max_length: None` omits maxLength. `max_length: 0` and `32768` refuse | SDK 3.0.1 `TextInput`. Parser `text.rs:4304-4855`. Current preference of default over cache is the defect at `text.rs:4523` | `legacy_reset_defaults_survive_current_value_updates` |
| Checkbox | `Auto` writes `sizeAuto`. `HalfPoints(25)` writes integer 25. `default: false`, `checked: true` stay distinct | SDK 3.0.1 `CheckBox`. Local Word.sdef names are not a passed run | Same reopen test, checkbox arm |
| Drop-down | Two identical labels are legal. `default_index: 0`, `selected_index: 2` stay distinct when the list has at least three entries | SDK 3.0.1 `DropDownListFormField` | Same reopen test, list arm |
| Help and status | `None` omits the element. `Some(Text(""))` stores empty text. `Some(AutoText("x"))` stores the AutoText selector | SDK `FormFieldData`. Word.sdef help and status names | Property reopen test |
| Begin-only `ffData` | Present on complex begin. Absent on separate and end | Writer gap `text.rs:12632`. Order owner `text.rs:4820` | Ordered attachment in A1 |
| Result region | Present empty region wins over default. Absent region uses default, then empty fallback. Candidate bytes decide materialization. A raw empty cache can remain while effective text shows the fallback | `separate_run` at `text.rs:5227`. Semantic reads at `text.rs:2870` and `engine.rs:2518`, `10238`, `3368`. Literal map at `placeholder.rs:552` | A2 and A7 |
| Literal replace | Fallback display `foo` is visible to text, layout, and PDF. Replacing `foo` returns 0 and leaves bytes unchanged. Typed empty materializes empty current and keeps default `foo` | `document.rs:36514`, `document.rs:36537`, `placeholder.rs:412`, `placeholder.rs:552`, `placeholder.rs:714` | `cross_consumer_render_fallback_before_empty_assignment` |
| Native Word | Compare stored current and default, field structure, and display after an actual Word open, update, save, and reopen | Word 16.114 / 16.114.26101123. Actual V4c originals and four approved consumer pairs are hash-bound in the Q25 evidence above. Independent native originals still expose seven rather than ten fields through the old installed parser | A6 blocked by Q25 inventory gap. Successful native execution is not final acceptance |

## Dependency readiness

`docs/sprints/BACKLOG.md` records F-237 and F-278 done. At the 2026-10-23 approval, F-288 was pending and unclaimed. Codex root approved this contract on 2026-10-23 at source `a57be365`. That approval stays historical and does not by itself mark the shared ledger done. The local Q4 checkpoint is now recorded. Source head `0c5c2a5d7fe9a25bb8445cc984a9665482d4726e` is integrated at `96718dadf6840fe6ff73663dff3cd2b286418844` with tree `8cbc037b99a3664d5359629ea28069b9bc17f942`. Consumed local evidence is 14 native cases plus one private oxml unit at 5 visits and 5 nodes, previously 62, Python 25 on checked extension `2cd326a9198b1336424be835e5eba087d882167b67e0d6384ee46eac14c23cac`, local mypy, fmt, and five-crate all-features clippy, CLI fill 1 passed and 99 filtered on test binary `09d10b975e1d8650695f0cfc73227bdb6d164a81ce2f5ed2789b53c46743bc89` and CLI `8c22713ad61ec432a2bf88022fc4f698f12ac9f3e87dbd48072c9341c71ebb00`, and A7 native, layout, PDF, and CLI text. Product microscope pass 8 and metadata pass 10 are Root Codex reviews transcribed by Grok. Grok did not author those reviews. Two metadata policy tests passed on the worker. This prefix marks the dependency edge done. The plan stays approved. F-289 implementation has not started. Final Word A6, hash harness, full verification, installed typing and stub test, runtime, WASM, docs, archive, and riders stay pending. F-237 building blocks remain the glossary route. This story does not add a second glossary store.

## Acceptance criteria

| ID | Input or setup | Expected observation or refusal | Consumers | Local test | Final gate or pending evidence |
|---|---|---|---|---|---|
| A1 | Three factories with the property set, attached by both existing routes, including the Python static factories and both Python attachment routes | Ordered `ffData` on begin only. Typed caches match the definition. Formatting stays `CT_RPr`. A new API check is distinct from the semantic default and current RED | Rust facade and the Python contract below | `public_legacy_fields_keep_properties_and_typed_values_on_reopen` and `test_authored_legacy_form_fields_keep_defaults_and_current_values` | Differential gate pending Word |
| A2 | Q25 adds the admitted own-bookmarked ordinary result child, its unchanged default, byte no-op and explicit literal parent assignment. Unequal default and current, then fill and reopen. Held frozen snapshots stay old. Fresh inventory and reopen match. Fresh, reopen, unchanged default, and no-op cover the publication flag | Explicit default from `legacy_form_default_value` is unchanged. `None` is absence. Current follows the separator flag. Python `.default_value` stays readonly `str`, `bool`, `int`, or `None` on the existing object | Rust fill. Python property and fill. CLI fill. Deterministic PDF text is A7. No AcroForm consumer belongs to this story | `legacy_reset_defaults_survive_current_value_updates` and the Python defaults test | Native unequal pairs pending |
| A3 | One ordinary field plus one legacy field, then `update_fields`, and the Q25 ordinary leaf in a legacy cached result | Ordinary result may change under existing context/locked policy. The outer reset metadata and physical bookmarks remain. Newly visible ordinary evaluation/update counts are disclosed. No new native or Python evaluator | Existing field update | `generic_field_updates_preserve_legacy_form_state` | Word-open effects pending under the Q3 store-settings answer |
| A4 | Bound violations, bad Python shapes, and contradictory manual metadata | Native `Err` or the Python error mapping before attachment. Destination bytes unchanged. Factory invalid and late attachment failure are byte atomic | Both native routes and both Python routes | `invalid_legacy_field_creation_cannot_change_destination` and `test_legacy_field_factories_and_attachment_refuse_atomically` | Local atomicity. Native not required for refusal |
| A5 | Alias text, opaque unknown bytes, nested and related owners. Table and related insertion use the existing `item: StoryItem` form at `document.rs:7505`. Paragraph positions do not admit cell paragraph handles. No new position carrier | Macros and unknown bytes preserved. Owner part and ordinal stay physical | Existing story routes and the Python `StoryRunPosition` route | `legacy_authoring_respects_nested_story_owners_and_opaque_xml` | Reopen fidelity |
| A6 | Word-created originals and library-created files for all three kinds, including Q25 nested ordinary TIME/DATE/formula result inputs | Ten actual native fields are listed once, with current/default and physical bookmark fidelity. Stored properties are separated from Word formatting/calculation effects | Native Word 16.114 and public Rust/Python reads | Existing native inputs, actual 398a seven-of-ten RED, named Q25 source regressions and fresh/reopen checks | Blocked by native original inventory gap. V4c and four consumer runs/subchecks exist. Repair GREEN, independent microscope and applicable final acceptance remain pending. This remains the only Word gate |
| A7 | Absent result, raw stored cache empty, explicit default `foo`. Then a typed empty assignment. Deterministic bundled fonts | Before assignment, native text, layout, and PDF text show `foo`. Literal replace of `foo` returns 0 and the package bytes stay unchanged. After the typed empty setter, fresh, reopen, and CLI current are empty and the default remains `foo`. Physical `cached_result` follows the no-op formula and is not rewritten into a virtual display | Rust text, layout, PDF, literal replace, CLI | `cross_consumer_render_fallback_before_empty_assignment` | Local regression only. No new external oracle |

Backlog promise mapped above: complete help, status, default, formatting, size, list, and calculation properties, with typed values through update and reopen. Calculation properties are stored metadata under the Q3 answer. Word executes them. Evaluated output parity is not claimed.

## Test matrix

| Case | Modes or state combined | Expected difference or agreement | Wrong implementation rejected |
|---|---|---|---|
| C1 | Text, checkbox, and list. Explicit default unequal to current. Reopen | Inventory shows both facts | A projection that still prefers `w:default` as current fails |
| C2 | `None` help versus `Text("")` versus `AutoText` | Omission, empty text, and AutoText stay distinct | Collapsing `None` and empty fails |
| C3 | `update_fields` with a changed ordinary field beside a form | Form bytes that encode state stay | An evaluator or a wiped `ffData` fails |
| C4 | Valid `HalfPoints(0)` and `HalfPoints(25)` beside an overlong name, 26 list entries, an index past the end, and a contradictory kind | Valid 0 and 25 attach. Invalid list, length, and kind refuse atomically | A partial write, or a rejection of valid 0 or 25, fails |
| C5 | Nested table run and a supported related story, plus opaque imported bytes | Same physical owner. Unknown span preserved | A main-only insert or a dropped macro fails |
| C6 | Absent result region versus present empty result, then literal replace of the fallback and a typed empty setter | Effective text, layout, and PDF show the fallback. Literal replace count is 0. Typed empty materializes empty current and keeps the default | A virtual-default replacer, or an early return on typed equality, fails |
| C7 | Parent override plus a changed nested child, and a text box before an old body field, plus comments | Override wins. Ordinal prefixes stay. New spans append in part source order | A glyph boolean or a source-order renumber fails |
| C8 | Native checkbox with a separator but no result run, own bookmark, styled pure end, aliased or default scope and both marker spellings. Same and opposite current beside another checkbox and a list | A dedicated cache preserves pure boundaries, formatting, defaults, ordered legacy identity and reopen. A second identical canonical assignment is byte-identical | Text inside the end run loses admission. A following equal Boolean cannot satisfy a shifted ordinal |

Omitted: a generic Python `Field` grammar wrapper and any new Python XML path. Generic native `Field` raw construction stays without a new Python exposure. Existing `Run.add_field` stays unchanged. WASM and CLI gain no new authoring method. Existing consumers retain preservation and current-value behavior. Picture and modern controls belong to F-289. Mail merge belongs to F-290. The historical Q1 recommendation of native-only scope stays in the question table as history.

## Rejected alternatives

- Keeping the mirrored `w:default` write. Source analysis on 2026-10-23 shows that write conflates reset and current. Q2 is a necessary correction, with a separate behaviour commit and consumer disclosure.
- A new formula, macro, or locale evaluator. Q3, user answer on 2026-10-23, stores settings and leaves execution to Word. The evaluator alternative stays rejected.
- A second attachment API or mutable field handle. Existing routes and part-scoped ordinals remain.
- Calling metadata-only storage equal to Word-evaluated display.

## Test plan

Backlog test gate, verbatim: differential. Public-created legacy fields match Word and retain their typed values through update and reopen.

| Timing | Category | Test | Asserts |
|---|---|---|---|
| local scoped | regression | `public_legacy_fields_keep_properties_and_typed_values_on_reopen` | Properties and typed values survive save and reopen |
| local scoped | regression | `legacy_reset_defaults_survive_current_value_updates` | Reset default stays while current changes |
| local scoped | regression | `generic_field_updates_preserve_legacy_form_state` | `FORM*` state remains beside an updated ordinary field |
| local scoped | regression | `invalid_legacy_field_creation_cannot_change_destination` | Refusal leaves the destination unchanged |
| local scoped | regression | `legacy_authoring_respects_nested_story_owners_and_opaque_xml` | Owners and unknown bytes survive |
| local scoped | regression | `cross_consumer_render_fallback_before_empty_assignment` | A7. Deterministic fonts. Fallback `foo` is visible in native text, layout, and PDF text. Literal replace of `foo` returns 0 with unchanged bytes. Typed empty then shows empty current and default `foo` on fresh, reopen, and CLI |
| provider prefix | Q4 local gate | Every named local native group in this test plan, including A7 and `cross_consumer_render_fallback_before_empty_assignment`, plus the affected Python factory, attachment, current, and default checks | The 2026-10-23 user answer originally listed five native groups. The active contract includes the later focused regressions and the Python bindings. Do not weaken the gate back to those five, and do not leave only three native groups unchecked. The required checkpoint is local RED then GREEN, scoped verification, an independent clean microscope, and an integrated source ancestor before F-289. That local evidence is consumed and the ancestor is integration commit `96718dadf6840fe6ff73663dff3cd2b286418844`. The semantic RED is default and current through the old existing providers. Additive factory absence is a compile failure, not that semantic RED. The A1 new API check is distinct and comes first. Differential, full verification, and riders stay pending |
| local scoped Q25 | regression | `native_named_legacy_text_fields_keep_ordinary_result_children` in existing `text.rs` | TIME, DATE and formula source XML, native tail instrText, subtype swaps, raw source, current/default, child identity, three bookmark axes and untouched/current-edited reopen |
| local scoped Q25 | regression | Extend `partial_or_ambiguous_legacy_bookmarks_keep_the_untyped_source` | The old eligible DATE case becomes a named positive under the approved choice. Multiple/deep/both-region/simple/nested legacy/private, extra literal/nested-switch arguments and existing pair/purity/namespace negatives stay |
| local scoped Q25 | integration | `native_named_legacy_result_fields_keep_public_defaults_updates_and_bookmarks` and `native_named_legacy_result_parent_assignments_are_atomic_literal_replacements` in existing integration entrypoint | Inventory once, source IDs, child update, explicit/unresolved ordinary contexts, locked/override, REF, literal no-op/replacement, atomic refusal, rich authored-ID remap and physical markers |
| local scoped Q25 | Python | `test_native_legacy_result_fields_keep_inventory_defaults_and_literal_assignments` in existing `tests/test_core.py` | Held frozen versus fresh, actual current/default, same/different assignments, source/public ordinal selection, no-op/refusal and reopen through an authenticated installed artifact |
| final sprint Q25 | native read regression | Exact six-file public fresh/save/reopen check bound in `/private/tmp/s954-f288-a6-public-read-result.json` | Replace the actual four-pass/two-fail result only with a successful new identified artifact execution. Both native originals and four consumed library outputs list ten. Native provenance stays independent |
| final sprint | oracle | Word 16.114 comparison | Inputs: text reset versus filled, checkbox false versus true, list index 0 versus 2 with duplicate labels, literal and AutoText, 12.5 pt, disabled and calc toggles. Compare current, default, fields, and display after native update and reopen. Bind input and output SHA-256 and the Word version |
| local scoped | regression | `test_authored_legacy_form_fields_keep_defaults_and_current_values` | Python public factories, both attachment routes, nested and related `StoryItem`, held frozen, fresh, and reopen, default and current discriminator |
| local scoped | regression | `test_legacy_field_factories_and_attachment_refuse_atomically` | Strict bool, indices, help tuples, factory invalid, late attachment failure, byte atomic |
| local scoped | typing | `test_legacy_field_definition_public_factories_are_typed` | Factory class import and observable defaults in `crates/rdocx-py/tests/test_core.py`. Typing uses existing `crates/rdocx-py/tests/typing_smoke.py`. No temporary string runner and no new tracked file |
| final sprint | risk rider | Integrated `/verify --full` once, hash harness, strict mypy and stubtest, WASM, and a genuine publish dry-run with archive under 10 MiB, plus riders in Risk routing. Native Word only in that final union | Existing fixture hashes unchanged aside from the declared behaviour commit. Scoped public API tests stay on this feature |
| before final freeze | metadata | `python3 -m unittest scripts.test_sprint_workflow.SprintWorkflowTests.test_modern_docx_capability_matrix_has_closed_classifications_and_evidence scripts.test_sprint_workflow.SprintWorkflowTests.test_every_incomplete_modern_docx_row_has_one_live_owner` | DOCX-055 cells, classification, evidence, and owner match the implemented and tested scope. `Y` means that bounded operation exists today. Native Word parity stays explicit debt. Do not mark the row complete only so the assertion passes |
| after completion metadata | metadata | `python3 -m unittest scripts.test_sprint_workflow.SprintWorkflowTests.test_every_incomplete_modern_docx_row_has_one_live_owner` | Backlog and current-sprint status agree with the DOCX-055 owner. A previous green run does not prove the new status. `capability_owner_problems` still rejects an incomplete owner that is done |

Named local regressions and Root Codex microscope pass 8 are consumed with integration commit `96718dadf6840fe6ff73663dff3cd2b286418844`. This prefix records the Q4 dependency edge. It does not close the differential gate, full verification, or the riders. DOCX-055 stays the implemented additive row from source head `0c5c2a5d7fe9a25bb8445cc984a9665482d4726e`. Final rider debt stays pending.

## Native empty-cache remediation boundary

Actual c62 normal module4f02 restores ten-field read, current, default and save/reopen agreement across both native originals and four consumed library files. The earlier seven-of-ten failures remain historical. Additional same-current probes expose four atomic setter refusals on the two native-original checkboxes. Two opposite AutoFlag requests also return the following ExactFlag after the modified projection disappears. Those actual REDs and saved XML remain in `/private/tmp/s954-native-checkbox-setter-diagnosis`. This is a writer invariant repair within existing setter scope, not Q25/Q27 admission expansion. Final repaired normal setter and integrated qualification remain pending.

Local regressions are `native_legacy_empty_cache_keeps_end_marker_run_pure` in the existing OXML unit entrypoint and `native_empty_checkbox_setters_retain_legacy_identity_and_neighbors` in the existing facade integration entrypoint. They compose with existing ordinary text, Q25/Q27 child, alias, source identity, marker axis and atomic refusal groups in one scoped batch. The ABI metadata amendment in F-289/F-290 changes no constructor or form setter. No normal build, hash, timing, memory or native execution is part of this scoped repair.

## HLD impact

- `docs/hld/14-development-backlog.md`, the implemented additive default-value queries in F-288, with final acceptance kept separate
- `docs/hld/02-scope-and-non-goals.md`, row DOCX-055 for F-288 (stored form properties, Q25 imported native result boundary, no new evaluator or macro execution)
- `docs/hld/03-architecture.md`, "What stays put", "Facade conventions", and "Owned publication and causal paths" for effective versus physical field cache, Q25 source identity and explicit literal parent replacement
- `docs/hld/04-opc-and-packaging.md`, "The package", including Q25 exact outer/child source and both physical bookmark markers
- `docs/hld/10-bindings-spec.md`, "Native Word facade stability", including `effective_display`, `effective_projected_text`, `effective_display_field_segments`, Q25 imported variants and part-scoped ordinal compatibility
- `docs/hld/12-testing-strategy.md`, "Test taxonomy", "Typed content-control native Word qualification", the local deterministic-font PDF regression for fallback display versus literal replace, and Q25 positive/negative/native-source proof

## Risk routing

Matched rows from `.claude/skills/risk-routing.md`. Extra checks run once on the integrated sprint:

- Parser or serialiser. Round-trip proving unmodelled `ffData` siblings and macros stay byte for byte. Fixed prefixes on write.
- Public API of a published crate. Semver note. Genuine `cargo publish --dry-run` with archive under 10 MiB, only in the final union. README measured package inventory may be recomputed by the existing `README.md` and `scripts/readme_doctests.py` only if that archive change requires it. Do not fabricate those values. No Cargo dependency change and no new module.
- External oracle. Pin Word 16.114 / 16.114.26101123. Q25 retains the actual native V4c and approved consumer provenance, with A6 inventory repair/closure still pending. No library-generated baseline substitutes for those native files.
- Compatibility and collection ordering. Field-specific bounds and the `text.rs:4820` order. Q2 behaviour commit states any hash delta. Q25 separately labels the native seven-to-ten inventory and later ordinal delta, newly visible ordinary updates and existing explicit literal assignment. Native formatting/calculation effects remain distinct.
- New getter projection. Inventory explicit default, absent default, present empty result, and absent result.
- Cached read facts. Publishers are factory initialization and `set_legacy_form_field_value`. `LegacyFormFieldInfo` is owned, so a held pre-edit snapshot keeps the old current value. A fresh inventory, the native current value, and the reopened current value agree, and the reset default stays unchanged. Existing Python live handles follow their survival and staleness policies. A refusal or a no-op preserves valid handles. This story does not revive a dead owner. Native agreement is pending.
- XML in multiple parents. Both existing insertion paths and supported related stories.
- Shared semantic behavior. Rust authoring and the Python contract below. Python fill, CLI fill, CLI text, and deterministic PDF text are separate observations. No DOCX AcroForm consumer is in this story, so none is requalified here. The prior native planning basis remains 8 to 11 developer-days across the sprint. Q1 Python scope is an explicit planning estimate of 4 to 6 additional developer-days, for an expanded planning total of 12 to 17. That figure is an estimate, not a measurement, and it is not an undisclosed capacity promise.
- Grammar and empty collection. Empty strings, omitted elements, and one-to-25 list entries. Q25 distinguishes zero/one/many cached children, result versus instruction, ordinary versus legacy, complex versus simple, namespace/purity and direct/accepted/tracked owner contexts. No ordinary-kind allowlist expansion beyond TIME, DATE and formula.
- PyO3 is in scope under the Q1 answer. The exact classes, factories, and errors are `## Python API contract`. Scoped affected public API tests and the small typed use run with this feature. Strict mypy, stubtest, the installed full suite, WASM, publish dry-run, full verify, and the hash harness run only in the final union. WASM gains no new authoring method. Existing WASM and CLI consumers retain preservation and current-value behavior. Workspace tests keep `--exclude rdocx-py --exclude rpptx-py`. Codex root semantic validation on 2026-10-23 approved this contract. The local worker has the approved surface. The final union stays pending.

## Hash harness

Existing fixtures are expected to stay unchanged. Q25/Q27 is its own labelled behaviour commit, declaring native inventory ten rather than seven, resulting ordinal compatibility and newly reachable ordinary update/literal-current behavior. Actual hash harness deltas are measured and reviewed, never inferred away or combined with a fixture move. No new baseline is authorized by this amendment. The Q2 repair is a separate labelled behaviour commit during implementation. That commit states its expected delta if any current fixture observes the mirrored default write. No baseline is re-recorded in this approval. Rendering baselines stay on deterministic fonts. No fixture move is combined with the behaviour change.

## Exclusive resources

Product source and test files already named in this plan stay the product lease. This amendment adds one existing file:

- `scripts/test_sprint_workflow.py`, only the expected live-owner inventory in `test_every_incomplete_modern_docx_row_has_one_live_owner`, and only when DOCX-055 genuinely leaves the incomplete set. Do not change the closed-classification rules in `test_modern_docx_capability_matrix_has_closed_classifications_and_evidence`. Do not add a done-owner exception. Do not edit this file while the row stays incomplete. F-289 and F-290 lease the same file later for their own rows.

The Q4 local provider-prefix completion exception remains. Final Word differential, full verification, and riders stay pending in this plan. This metadata pass reconciles DOCX-055 for the implemented additive surface. The prefix done write and the final source freeze still reconcile it again if later evidence changes the cells. An incomplete row cannot name a done owner. A resolved row cannot keep F-288 as owner. Satisfy that with cells and evidence from the tested scope. `Y` is a complete bounded operation today, not native Word parity. Leave that native debt explicit in the evidence boundary.

## Implementation checklist

The checked local provider work below stays genuine and consumed. Q25 is a new final-native remediation, not a revocation of that completion.

- [x] Q25 narrow parser admission, all excluded-shape controls, literal/current/default and ordinary-update closure in the existing files.
- [x] Q25 independent microscope with zero defects/smells, scoped checks and explicit hash delta, then authenticated installed native-original ten-field GREEN. Qualify native A6 and final acceptance through their actual remaining observations.

- [x] Add the enums, options, definition, and `Field::from_legacy_form` in existing `text.rs` and re-export them. Native and HLD edits stay on files already named in this plan.
- [x] Bind Python only in existing `crates/rdocx-py/src/document.rs`, `crates/rdocx-py/src/run.rs`, `crates/rdocx-py/src/lib.rs`, `python/rdocx/__init__.py`, `python/rdocx/_rdocx.pyi`, and `tests/test_core.py`. No new tracked file, module, or test binary.
- [x] Add `Field::legacy_form_default_value` and `Document::legacy_form_fields_with_defaults` without a new field on `LegacyFormFieldData` or `LegacyFormFieldInfo`.
- [x] Split current result selection from `w:default` with the candidate-byte no-op and the private presence flag, not typed-value equality and not a string diff alone.
- [x] Add `effective_display`, `effective_projected_text`, and `effective_display_field_segments` on `Field` with the `std::borrow::Cow` signatures in Approach. Leave `projected_text`, `cached_display_field_segments`, and `FieldRef::cached_display_segments` borrowed.
- [x] Route `text.rs:2870`, `engine.rs:2518`, `engine.rs:10238`, `engine.rs:3368`, inventory, and CLI through the effective policy. Leave `cached_run_display` at `text.rs:2042` physical.
- [x] Keep literal replacement on `placeholder.rs:552` and the simple-field wrappers at `placeholder.rs:412` and `714`.
- [x] Stop value updates from rewriting the reset default.
- [x] Integrate `ffData` into the empty begin writer, reusing `text.rs:4820`.
- [x] Refuse invalid bounds and kind contradictions before publication.
- [x] Preserve imported macros, unknown bytes, and universal-measure sizes.
- [x] Land native tests in the existing integration entrypoint. Land the three named Python tests in existing `tests/test_core.py`.
- [x] Register the frozen class on public `rdocx` and `rdocx._rdocx`, with `_rdocx.pyi`, `__all__`, and `__init__` imports aligned.
- [x] Python fill is `test_fx241_python_fill_all_five_reported_form_kinds` in `crates/rdocx-py/tests/test_core.py`. It is one of the 25 pytest cases that passed in the remediation 7 batch. It is not the Rust A7 case.
- [x] A7 `cross_consumer_render_fallback_before_empty_assignment` covers Rust native text, layout, deterministic PDF text extraction, and CLI `rdocx text` only. It passed in the remediation 7 batch. It does not execute Python fill or CLI fill.
- [x] CLI fill is `fx241_cli_fill_five_kinds_preserves_order_source_and_old_tag_contract` in `crates/rdocx-cli/tests/integration.rs`. This metadata pass ran that one current-source test. 1 passed, 0 failed, 99 filtered, exit 0. Test binary `target/debug/deps/integration-27068b8b419f3d54` SHA-256 `09d10b975e1d8650695f0cfc73227bdb6d164a81ce2f5ed2789b53c46743bc89`. CLI `target/debug/rdocx` SHA-256 `8c22713ad61ec432a2bf88022fc4f698f12ac9f3e87dbd48072c9341c71ebb00`.
- [x] Applicability audit. No DOCX converter and no legacy-default to AcroForm consumer was found under `crates/rdocx`. Issue #381 stays a separate later plan. No AcroForm runtime pass is claimed. That future exporter is outside F-288 acceptance. The unchanged low-level PDF backend is not legacy qualification.
- [x] Qualify the required Word differential, schema-order and source-applicability observations with their actual native producers.
- [x] Reconcile DOCX-055 for the implemented additive surface in this metadata pass. `scripts/test_sprint_workflow.py` excludes F-288 from the incomplete live-owner inventory because the row leaves that set.
- [x] Reconcile final completion metadata and source freeze after actual Word A6, full verification, hash harness, installed suites and required riders pass.

## Open questions

| ID | Affected F-IDs and A-IDs | Decision and recommendation | Status and answer/source |
|---|---|---|---|
| Q1 | F-288 A1 and A6. Also F-289 and F-290 | Historical recommendation: native Rust additions only, matching existing generic field and mail-merge authoring. Existing Python, WASM, and CLI behaviour preserves authored packages and form-fill compatibility. No new APIs there. Alternative: add Python creation APIs in this sprint, with exact binding, stub, and export signatures plus installed strict typing, stubtest, and real public calls fully specified before approval. Native-only fits the bounded sprint. If Python is selected, revise this plan before approval. | Answered 2026-10-23 by user answer. Rust and Python new APIs. WASM and CLI gain no new authoring methods. Existing consumers retain preservation and current-value behavior. The exact Python surface is now recorded in `## Python API contract`. Codex root later approved this contract on 2026-10-23. This row keeps the original answer. |
| Q2 | F-288 A2 and A3. F-289 A3 and A4 | Separate persistent legacy reset defaults from current typed values. | Answered 2026-10-23 by source analysis. Necessary correction. Explicit `w:default` stays metadata. `None` is absent. Fallback `Text("")`, `Checked(false)`, `SelectedIndex(0)`. A published empty result wins. An absent separator uses default then empty. `set_legacy_form_field_value` does not change the reset default. No new default setter. Disclose Python fill, CLI fill, and PDF and AcroForm. Separate behaviour commit. The later Q15 answer keeps this split and refuses a new struct field. |
| Q15 | F-288 A2. F-289 A3 and A4 | Additive default queries. Do not change existing public struct shapes. | Answered 2026-10-23 by user answer, after the earlier assembly. `Field::legacy_form_default_value(&self) -> rdocx_oxml::Result<Option<LegacyFormFieldValue>>` reads explicit reset-default metadata and returns `None` when it is absent. Existing `legacy_form_value` distinguishes a non-legacy field. `Document::legacy_form_fields_with_defaults(&self) -> Result<Vec<(LegacyFormFieldInfo, Option<LegacyFormFieldValue>)>>` is one shared physical inventory walk. `legacy_form_fields` keeps its old signature and maps owned info from that walk. Python inventory uses the fused batch and freezes `.default_value` on the existing object. F-289 reset uses the pre-batch pair. No new singular query. No `default_value` field on `LegacyFormFieldData` or `LegacyFormFieldInfo`. Private `Field` author and source metadata is the only authority. The shared parser preserves accepted import states, including an out-of-list explicit default. No new read refusal beyond structural or malformed input. Factory bounds stay. |
| Q3 | F-288 A1, A3, and A6 | Historical recommendation: author all six text kinds, format, and `calcOnExit`, with caller-supplied caches. `update_fields` preserves `FORM*` state and existing diagnostics. Word performs formatting and calculation. Alternative: native execution inside rdocx needs a separate design. Do not call stored metadata equal to evaluated output. | Answered 2026-10-23 by user answer. Store settings. Word executes. No new native or Python calculation or formatting evaluator. |
| Q4 | F-288 A1, A2, A4, and A7. F-289 dependency | Historical recommendation: approve the genuine local provider gate named in the test plan so F-289 prefix work can proceed, with Word differential, full verify, and riders still pending. Alternative: wait for the final F-288 differential before F-289 starts. That alternative contradicts one final oracle pass and needs workflow reconciliation. A reviewed local gate is not completed differential evidence. | Answered 2026-10-23 by user answer. The original answer listed five named local native groups. After a verified local provider checkpoint. The active contract is F-288 local RED then GREEN for every named focused regression now in this test plan, including A7 and the render-fallback case, plus the Python factory, attachment, current, and default checks, scoped verification, a clean microscope, and an integrated source ancestor, before F-289. That expansion does not weaken the original gate. Final Word differential, full verification, and riders remain pending. |
| Q25 | F-288 A2, A3, A5 and A6. F-289 imported legacy consumption | Admit one complex ordinary TIME/DATE/formula leaf cached-result child under one valid own-bookmarked FORMTEXT TextInput. Preserve until explicit literal-cache assignment. Keep other nesting/private/purity/namespace/axis fallbacks. | Answered 2026-10-27 by the user, "Allow bounded nested results (Recommended)" at 2026-10-27T00:40:53.668202+00:00. Exact scope and compatibility are the Q25 subsection above. No subtype/name correlation, public API, new evaluator, cache or budget. Existing ordinary evaluation and explicit literal-parent assignment apply. The existing eligible DATE negative is deliberately superseded. Implementation has not started at this amendment. |

| Q27 | F-288 A2, A3, A5 and A6, imported legacy consumption | Add exactly one native-shaped ordinary complex leaf instruction argument as an alternative to Q25 result ownership, preserving outer saved current and instruction configuration. | Answered 2026-10-27 by the user, "Allow bounded instruction variant (Recommended)". Exact scope is the approved Q27 section. User then directed recommended choices without further questions. No new evaluator or clock. |

Q16 is answered on F-290 only. Provenance is the 2026-10-23 user choice "Author Word-compatible canonical slots (Recommended)". It is not a decision of this story. Original Q1 through Q15 on this plan stay unchanged. Q25 only changes the bounded native result-nesting exclusion.

## Contract closure

Result: user answers recorded. Codex root semantic validation on 2026-10-23 approved this contract at source `a57be365`. Q15 additive default queries, `Cow` readers, and candidate-byte no-op materialization stay the approved contract. The local worker has that implementation. Native Word qualification and final gates stay pending. This approval does not mark the story completed.

Remaining decisions: Q1, Q2, Q3, Q4, Q15, Q25 and Q27 are answered. Q25/Q27 remediation is pending and does not reopen the consumed Q4 provider checkpoint. Q2 keeps its source-analysis provenance. Q15 is the 2026-10-23 user answer recorded above. No blocked product choice remains. Local implementation evidence and the integrated Q4 ancestor are recorded above. Final Word acceptance and Q25 remediation stay pending.

Blockers before approval:

- Q1 selects Rust and Python. The signatures in `## Python API contract` are the approved surface. At approval, implementation had not started. The local worker now has that surface.
- Q3 stores settings. Word executes them. Evaluated-output parity is out of scope. An evaluator would need a new design.
- Q4 selects the local checkpoint. The 2026-10-23 user answer originally listed five named local native groups. The active checkpoint is every named focused regression in this test plan, including A7 and the render-fallback case, plus the Python bindings. The gate is not weakened. That checkpoint is not completed differential evidence. Word differential, full verification, and riders remain pending.
- Authored `HalfPoints` values are exact integers from 0 through `u64::MAX`, the same set `legacy_form_hps_measure_value` accepts. Native Word behaviour for zero and extreme sizes is not proved by that parser and is not claimed.
- Absent-default fallback and child order are specified as policy plus existing writer order. They are not claimed as verified by a new Word run.
- Native A6 inputs and the six-file ef048ffb setter result are actual evidence. Preserve source pins and original native files, and require exact final-artifact applicability before final acceptance.

Inventory of operations: `from_legacy_form` creates. `add_field_value` and `insert_field` attach. `Field::legacy_form_value` reads the authoritative current value. `Field::legacy_form_default_value` reads an explicit default. `legacy_form_fields_with_defaults` is the one batch. `legacy_form_fields` delegates and keeps its old shape. `set_legacy_form_field_value` updates current values. `update_fields` is the preservation publisher. No repeat, remove, clone, or unwrap API is added for legacy fields.

Cached projection: source facts are `w:default`, typed checkbox and list metadata, and the physical `cached_result`. Effective readers are `effective_display(&self) -> std::borrow::Cow<'_, str>`, `effective_projected_text(&self) -> Option<std::borrow::Cow<'_, str>>`, and `effective_display_field_segments(&self) -> Vec<(std::borrow::Cow<'_, str>, Option<&CT_RPr>, &Field)>`. Borrowed `projected_text`, `cached_display_field_segments`, and `FieldRef::cached_display_segments` stay physical. A present empty publication wins over the default. An absent outer result falls back for effective text only. The setter no-op is the candidate-byte formula in Approach, never typed-value equality. Publishers are the factory, `set_legacy_form_field_value`, direct `cached_result`, `set_cached_runs`, annotation cache, `cached_field_mut`, and refresh. Attachment and reopen are compared. An observable parent `cached_result` override wins over effective nested children. Typed run format stays. Custom display that disagrees with a checkbox or list is legal and can be repaired by the same typed value. Kind and structural contradictions refuse. `LegacyFormFieldInfo` is owned, so a held pre-edit snapshot keeps the old current value. A fresh inventory, the native current value, and the reopened current value agree, and the reset default stays unchanged. Python uses frozen and fresh snapshots and adds no low-level getter. Existing Python live handles follow their survival and staleness policies. A refusal or a byte no-op preserves valid handles. This story does not revive a dead owner. Literal replacement does not search effective fallback text. Native Word agreement remains the existing A6 gate only. The A7 render check is local and uses deterministic fonts.

Story inventory: every currently attachable story, including comments and text boxes. Old per-part ordinal prefixes stay. Newly discoverable physical begin spans append in part source order. A root reached again as a cell or text box is deduped. Comments use a fresh part sequence. Read and write share one frozen ordinal to physical map. Compatibility order replaces a pure source-order claim. Proving case is C7. Codex root semantic validation on 2026-10-23 approved this contract. At approval, implementation had not started. The local worker now has this inventory.


## Q27 approved native instruction variant

The user answered "Allow bounded instruction variant (Recommended)" on 2026-10-27, then instructed Root to choose recommended options without further questions and finish the sprint. This approves the exact additional variant in the Q25/Q27 subsection above. The result-only implementation remains useful and approved. The instruction extension is not yet implemented at this contract checkpoint.

Actual native evidence disproved the earlier result-region premise. Both hash-pinned originals put their one ordinary child before the outer FORMTEXT separator, in an instruction argument. The child result uses instrText and is preserved raw. Existing parsing does not model that child instrText as a cached display. The outer w:t current remains authoritative, including Calculated 5.00 rather than inner 5. Prior result-only local positives and the actual seven-of-ten native failures remain retained, with no retroactive A6 credit.

Admission requires exactly one total complex ordinary TIME, DATE or formula leaf in either the approved result region or one direct Nested instruction argument, never both. The instruction variant has no extra literal operand, nested switch argument, second child or deeper nesting. Existing scalar switches remain. Keep all valid TextInput ownership, pure boundary, exact own-name bookmark pair, namespace, reserved-prefix, source identity, private-owner and direct/accepted/tracked-axis guards.

The instruction variant preserves the outer current/default and retains its instruction child on changed literal parent assignment. Candidate bytes still decide no-op. Same-child cache edits keep original identity and source correspondence. A replacement child cannot borrow source authority merely from equal instruction text. Canonical publication must preserve both physical markers or refuse atomically through checked publication. Compare existing explicit/unresolved/locked/parent-override ordinary updates against the already supported unbookmarked control. Do not add a clock, evaluator, interpretation of inner instrText, or dispatch for no-space =2+3. Repeat rich MailMerge1 remap/REF with instruction ownership and namespace/axis controls.

Exact cited facts and the full matrix are `/private/tmp/s954-q27-corrected-native-contract.md`, accepted with the user answer. Frozen result-only evidence is `/private/tmp/s954-q25-frozen-hashes.json`. Actual original/updated event proof is `/private/tmp/s954-q25-native-region-mismatch.json`. Implementation and local checks use existing source/test files and the five named HLD files. Both native originals and four library-consumed files must list ten on an authenticated repaired artifact, with fresh/save/reopen equality. Part-scoped ordinals and ordinary update visibility change as already disclosed. The genuine consumed Q4 provider checkpoint remains completed, while final A6 and other riders remain pending. No delivery credit or new test binary is added.

Q27 closes the additional material question. Any earlier statement that the instruction variant is excluded or awaits a decision is superseded by this approved bounded section. Q25's original user answer and the failed premise remain historical provenance. Root's recommended option authority never waives actual acceptance checks.
