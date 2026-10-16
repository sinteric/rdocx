# F-X241, Python and CLI form reading and filling

**Status**: completed
**Sprint**: S94
**Size**: M
**Depends on**: none

## Problem

The current Python binding already exposes frozen content-control snapshots, tag/alias setters and atomic tag filling at crates/rdocx-py/src/document.rs:4672,:6352,:6361,:6369,:6389. CLI fill --tag already exists at crates/rdocx-cli/src/main.rs:339 and commands.rs:2702. Do not duplicate these providers or replace their callable API because Issue380's original surface inventory predates them.

Missing surface is bind_content_controls(dict), legacy form inventory/mutation and a strict CLI --values route resolving SDT tags/aliases and legacy names in one outer transaction. Native SDT binding already selects tag before alias at crates/rdocx/src/content_control.rs:186. Legacy fields already have checked source-part/ordinal identity and staged save/reopen mutation at crates/rdocx/src/field.rs:839,:857. Simple wrapper loops could otherwise partially fill the live document, choose an ambiguous field or retire held owners before a late error.

## Spec reference

- docs/hld/14-development-backlog.md, F-X241, Python and CLI form reading and filling. Size M, no dependencies and regression gate retained verbatim.
- docs/hld/10-bindings-spec.md, Python API shape, Python Word document operations, and the existing atomic fill_content_control_tags/CLI fill contract.
- docs/hld/04-opc-and-packaging.md, Package integrity and Checked Word operation source ownership, including bound custom XML and candidate publication.
- docs/hld/06-presentationml-model.md, Parts and raw XML preservation/schema sequence principles used by risk routing. This story adds no PresentationML form API.
- docs/hld/12-testing-strategy.md, Test taxonomy, legacy form and glossary round-trip gate, Binding tests and Word document-operation parity controls.
- docs/hld/15-build-and-toolchain.md, Packaging and Dependency policy, exact artifact inventories and existing limits.
- docs/hld/03-architecture.md, Owned publication and causal paths.
- Issue380 and accepted maintainer comment6200485985, authenticated in /private/tmp/S94-Issue380. The requested five-kind template is plain text, checkbox, drop-down, date SDT and a legacy text field. No reporter script exists.

## Approach

S94 is open from main3a3b0de8be9f8bf4eb588a9a24c565f0af8ee190. This approved design claims no unexecuted evidence. Establish the exact feature Base at claim and retain actual source/producer identities.

### Keep the existing SDT snapshot and callable contract

Retain content_controls() returning the existing tuple of frozen ContentControl snapshots with tag,alias,id,type,text. Keep both existing set-by-tag/alias methods and fill_content_control_tags unchanged, including all-match native behavior, per-tag count order and strict missing-tag refusal for the latter. Existing snapshot values stay detached and immutable after mutation.

Add this missing wrapper and stub:

```python
Document.bind_content_controls(values: dict[str, str]) -> int
```

Require actual string keys/values and validate the entire dictionary before staging. Reuse native bind_content_controls exactly: for each SDT, its tag map entry wins over its alias map entry, unmatched input keys do not cause an error, empty input returns0 and a no-op preserves bytes/held authority. Do not accidentally impose the new CLI's strict selector rules on this established native contract. Build any returned Python allocation before prepared publication.

Native SDT inventory/binding remains main-body scope, including its nested controls. It does not acquire header/footer coverage merely because legacy fields span related stories. Existing checkbox lexical admission, glyph/font selection, bound custom XML edits and unsupported picture/group text refusal remain at content_control.rs:72,:140,:175,:186. Date and drop-down values use the current native display/bound string contract. Do not invent a date parser, change producer date-format/fullDate metadata or claim selected-item validation absent from that provider. The gate inspects actual stored display and any existing bound XML value separately from retained producer metadata.

### Owned legacy snapshots and deterministic mutation

Expose a frozen LegacyFormField snapshot in the existing binding document.rs alongside the existing ContentControl class, with exact readable properties:

```python
class LegacyFormField:
    source_part: str
    ordinal: int
    name: str | None
    enabled: bool
    calculate_on_exit: bool
    type: Literal["text", "checkbox", "dropdown"]
    value: str | bool | int
    choices: tuple[str, ...]

Document.legacy_form_fields() -> tuple[LegacyFormField, ...]
Document.set_legacy_form_field_value(
    name: str | None,
    value: str | bool | int,
    *,
    source_part: str | None = None,
    ordinal: int | None = None,
) -> LegacyFormField
```

The class is an owned frozen result, not a live proxy or a second owner registry. Its metadata projects native LegacyFormFieldInfo at field.rs:528. It has no public constructor or setters. Inventory order and related-story reach follow existing native legacy_form_fields, including body, table/control nesting and supported headers/footers/notes. Enum string mappings are exact text/checkbox/dropdown. Preserve every native choices value and source part.

For unqualified mutation, name must be str and exactly one current native inventory entry must have that name. Unknown or duplicate names refuse. For qualified mutation, source_part and ordinal must be supplied together. Resolve the exact current native pair, then require name equality when name is not None. Name None is allowed only with the complete pair, supporting unnamed fields without guessing a name. Reject partial qualifiers, negative ordinal, bool used as ordinal and stale pair/name before editing. A snapshot is informational, never authority to skip current inventory checks.

Choose value by actual selected native kind. Text requires str, checkbox requires an actual bool and drop-down requires a nonnegative int excluding bool. No implicit stringification or bool-to-index conversion. Existing native maxLength, selection range, malformed owner and source preservation checks remain authoritative. Use native set_legacy_form_field_value(source_part,ordinal,typed_value), then allocate the complete returned snapshot and prepare owner publication before assigning the candidate. The native setter already reparses and checks the value at field.rs:901. Return the current reopened snapshot, not a fabricated value from the input.

### CLI flat JSON and strict mixed selector resolution

Extend the existing Fill command only:

```text
rdocx fill INPUT --tag NAME=VALUE [--tag NAME=VALUE ...] -o OUTPUT [--json]
rdocx fill INPUT --values VALUES.json -o OUTPUT [--json]
```

The two input modes conflict. Existing --tag behavior and JSON counts stay unchanged. --values accepts one nonempty finite flat JSON object. Keys are exact nonempty strings without trimming. Values are only JSON strings, booleans or nonnegative integral numbers fitting usize. Reject null, negative/fractional numbers, arrays, nested objects and duplicate object keys. Bound the UTF-8 file at1MiB and assignments at4096 before mutation. Invalid UTF-8/JSON fails without output.

Use existing serde_json token decoding in source order for keys and scalar values, retaining a key set before map insertion. A small private flat-object routine in existing commands.rs handles the fixed braces/separators and delegates escaping, strings and numeric syntax to serde_json. Do not parse into Value first and silently lose duplicate keys, add a general JSON parser or a serde/derive dependency solely for this finite schema. Existing replacement-map last-key behavior in oxml-cli-support remains unchanged, because that is a separate established command contract.

Resolve every input key against the same unchanged pre-edit document inventory. Deduplicate tag and alias matches to the same physical SDT. Require exactly one physical target across all SDT tag matches, SDT alias matches and legacy name matches. More than one SDT with a duplicate tag is ambiguous in --values, even though old --tag intentionally edits every matching control. A key shared by an SDT and a legacy field is ambiguous. Two different keys selecting the same target are duplicate assignments and refuse, even if values agree. Missing/unknown keys refuse. Error messages identify the key and candidate kinds/locations without selecting a first match.

For an SDT, permit a string, or a bool only when its current type is checkbox, converted to the existing native true/false lexical input. No integer-to-text coercion. For legacy text/checkbox/drop-down, use the exact str/bool/nonnegative-int rules above. SDT dates/drop-downs remain explicit strings under current native semantics. The CLI name selection does not silently become a new legacy typed conversion rule.

After all JSON, selector and type checks succeed, use one private cloned Document and two internal phases: SDT assignments first, legacy assignments second, preserving original input/result order through preallocated assignment slots. Reuse the native inventories, not a new XML parser or public location API. Freeze the complete pre-edit legacy inventory at field.rs:839/:528, including source_part, ordinal, name, kind, enabled, calculate_on_exit, choices and current typed value. Identity keys include every stable metadata field, not merely name/count.

SDT replacements can remove legacy markers inside an SDT: content_control.rs:510/:523 retains only Text, DeletedText and CommentReference run content, so prior field markers may disappear and shift later ordinals. Consequently, after all SDT candidate edits and before any legacy setter, require the complete current legacy inventory to equal the pre-edit frozen inventory. Any removed/added/reordered field, source-part/ordinal/name/kind/metadata drift or changed legacy typed value causes strict overlap/identity-drift refusal of the whole candidate. Match count1 alone is insufficient. This intentionally refuses mixed overlapping content rather than retargeting a later field with a now-shifted ordinal. An SDT-only operation retains its existing semantics. The guard applies when this new mixed operation also has legacy assignments.

Explicit SDT selectors are re-resolved before each write against their frozen unique route and metadata, then checked again after the SDT phase and final reopen. Vector indices alone do not grant authority. Existing native display replacement retains nested SDT nodes and does not create new controls. An outer control needs its own supported typed display location. Its nested selected control can remain valid in either input order, and this is admitted when both selected targets remain provably unchanged in identity. Do not invent a parent/child overlap refusal when the native provider retains both. Actual disappeared or changed selected targets refuse atomically. Unselected control inventory is not a blanket restriction on the existing SDT-only native contract. The distinct removal of legacy Field run content is real and still triggers the complete mixed legacy-inventory guard below.

Before every legacy write re-resolve its original physical source-part/ordinal and compare the full identity/metadata inventory against the frozen identity sequence, allowing only typed-value changes already produced by successful earlier planned legacy assignments. Apply native set_legacy_form_field_value only after this check. Verify its returned/reopened entry matches the original target identity and intended typed value. Recheck the final inventory with exactly the planned value substitutions. Any drift, unexpected count or late native failure discards the entire private candidate. No implicit re-selection by newly shifted ordinal, first-match name or a generic count1 is allowed. Individual native commits affect only the candidate.

Reopen/validate the complete final candidate before one existing atomic output publication, reusing commands.rs:2042 and existing input/output collision protections. All emitted assignment rows remain in original JSON order even though execution uses two phases. Any validation or result-allocation failure leaves input, existing output and held live owners untouched. Do not expose mixed live-document mutation halfway through a loop.

For --values --json, retain the existing mutation envelope and output assignment rows identifying key, target kind, selector route, selected source-part/ordinal for legacy entries and count1. Existing --tag envelope/schema stays unchanged. Ordinary output reports the total applied targets. Success0 requires all assignments and publication. Runtime validation/I/O/native failures exit1, and Clap usage conflicts retain exit2. Empty --values input refuses just as an empty --tag operation already does.

### Owner, preservation and signing interactions

All Python mutations use the established candidate/publication protocol, including conservative proven owner survival, exact no-op short circuit and full Python result allocation before live commit. Reuse document.rs:5272 rather than assigning inner directly. Test held Section, Paragraph, Table/Cell and selected related-story physical owners through successful supported edits, with byte/owner equality on refused or exact no-op operations. Frozen form snapshots remain historical values, not stale mutable handles.

The existing generic Conservative publisher retires every owner, so these two new form adapters use a private FormValues publication mode in the existing document.rs. Before any runtime repair, qualify the actual untouched-owner retirement RED. Exact no-op serialized bytes short-circuit before retirement. Actual form mutations preserve only certified unchanged owners: native batched old/candidate story snapshots supply raw paragraph/table XML and physical routes, while existing section_properties_xml supplies section source. Independent exact-route maps retain all old/current indices and require unique matches on story kind, part, owner index, item kind and complete index path. Cells additionally bind their exact table/row/cell and physical story. Existing indexed matcher agreement is necessary but not sufficient, because that matcher historically selects a first candidate. Every ancestor pair must also be uniquely certified. Whole changed tables and their cells/descendants retire. Changed/ambiguous paragraphs retire, and Run/Hyperlink tokens and existing global run/snapshot epochs always retire on actual mutation. No raw XML equality, matching text/count or frozen field metadata alone can establish authority. Existing native scalar forms cannot remove/recreate paragraph/table/section containers, and old retired tokens are never reintroduced. Source collection shares each native batch with the address inventory, avoiding repeated per-paragraph scans. Serialized state is flushed through the existing to_bytes route before collecting either side, including prior live related-story edits. Frozen Story metadata remains historical and readable, but its captured mutation authority follows the existing retired snapshot epoch. Other Conservative publishers keep their behavior unchanged.

The preservation matrix is exact: unchanged uniquely proved Paragraph/Table/Cell/live Section and related-story paragraph owners may survive, changed owning source or an uncertified ancestor retires the entire descendant branch, and exact no-op or refusal preserves every existing token. Tests retain changed-paragraph/cache, changed-table, duplicate-equal-text separate routes and already removed/recreated handles as negative controls. Only existing native operations are used, with no fabricated form structural behavior or new general owner API.

Use native custom XML binding resolution and legacy typed source patching. Preserve unrelated raw children/attributes, schema order, producer metadata and resources. Do not introduce generic form ownership or another XML engine. Related-story legacy operations retain native physical part identity and refusal boundaries.

At S94 integration with F-X240, fill then sign must produce complete coverage of the actual filled package. Sign then fill must retain the native invalidated-signature evidence, and the current-state verification route must name changed parts before and after save/reopen. No form operation silently strips or authenticates stale signatures. These are cross-story integration controls, not an implementation dependency added to the backlog.

## Rejected alternatives

- Replacing existing content_controls() with a property or creating a new live SDT proxy, breaking the established snapshot/callable contract.
- Relaxing --tag or reusing bind_content_controls' ignored-key behavior for strict CLI --values, which would conceal misspelled form names.
- First-match legacy selection or silently updating all ambiguous --values targets.
- Parsing JSON into an ordinary object map before checking duplicates, accepting arbitrary nested values or stringifying unsupported Python/JSON objects.
- Publishing SDT changes before a later legacy assignment succeeds, or bypassing owner publication for a convenience wrapper.
- Adding form-control creation, new date-format semantics, all-story SDT mutation, PDF widgets or a general template engine to this existing-provider parity story.

## Test plan

The **test gate** is **regression**, as the backlog requires. Reuse existing Python test_core/test_python_docx_parity, typing smoke, native integration modules and CLI integration entrypoint. Construct package fixtures in source and add no new test binary or product file.

| Category | Test | Asserts |
|---|---|---|
| regression | fx241_python_and_cli_fill_all_five_reported_form_kinds | Source-built plain text, checkbox, drop-down, date SDTs and legacy text fill through both actual installed Python and release CLI, with independent ZIP/XML and saved/reopened values |
| regression | fx241_existing_sdt_callable_bind_and_tag_fill_keep_their_contract | Existing frozen snapshots/tag/alias methods and --tag semantics remain. New Python bind preserves native tag-before-alias, unmatched-key, empty/no-op and bound XML behavior |
| regression | fx241_legacy_name_and_physical_identity_refuse_ambiguity | Unique name, qualified pair and unnamed qualified fields work. Duplicate/unknown/stale names, partial qualifiers, invalid ordinal/value kinds, maxLength and index range errors refuse atomically |
| regression | fx241_values_json_refuses_duplicates_unknown_and_cross_kind_collisions | Exact duplicate keys including escaped-equivalent spellings, tag/alias/legacy collisions, repeated target assignments, null/float/nested values and file/entry bounds fail before publication |
| regression | fx241_mixed_sdt_legacy_ordinal_shift_refuses_original_target_drift | Source-built SDT contains an earlier legacy field and a later outside legacy target. SDT text replacement removes the earlier field markers, shifting ordinals. Unchanged old ordinal would select the wrong later same-kind field. The actual CLI refuses on pre-edit inventory drift before any legacy write and preserves input/output bytes and held owners. Include late failure and independent XML+reopen checks, not just count1 |
| regression | fx241_mixed_fill_late_failure_preserves_bytes_and_held_owners | A valid early SDT edit followed by a failed legacy/bound-XML edit leaves original package, output path and captured owners unchanged. Exact no-op keeps authority |
| round-trip | fx241_related_legacy_and_bound_sdt_values_preserve_source | Main-body/nested SDTs and supported related-story legacy fields retain opaque XML, schema order, producer date/drop-down metadata, custom XML bindings and actual typed values through save/reopen |
| integration | fx241_native_fill_and_sign_share_current_package_state | Worker controls use existing native signing/verification over the current serialized candidate. Fill-before-sign verifies coverage and fill-after-sign names changed parts. F-X240 owns the later installed public cross-story acceptance |

The primary five-kind observer must inspect w14:checked and its display glyph/font, SDT display and bound XML strings, legacy ffData typed value and cached text, plus reopen through public inventories. It must not equate a matching generic text getter with a changed checkbox or legacy field. Retain source-built template and exact expected part relationships. The date gate asserts the specified existing display/bound-value contract and unchanged producer formatting, not an unimplemented calendar engine.

Meaningful RED/negative controls include missing old binding names/--values, actual early/late mixed failure, removed preceding legacy field with ordinal shift after nested SDT replacement, deliberately restored first-match ambiguity, lost checkbox state despite matching text and a legacy cache/typed-value disagreement. Return tests only when the public observer fails for the intended semantic reason, not a compilation artifact or annotation-spelling assertion.

## HLD impact

- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md
- docs/hld/15-build-and-toolchain.md

## Risk routing

- Parser/serializer: HLD04 and HLD06. New finite JSON admission and existing native SDT/legacy source patching require bounds, duplicate/unknown rejection, source-built raw-XML preservation, namespace/schema sequence and save/reopen controls. No new general XML parser is permitted.
- PyO3/public surface: HLD10 and structural rules. Exact new callable/snapshot/setter signatures and stubs, true bool versus int admission, prepared result allocation, held-owner survival, no-op/refusal and both current complete installed family suites after integration. Run strict typing and stubtest, with final Rust workspace tests excluding both extension crates.
- Published source/API consumers: HLD10/HLD15. No Rust signature expansion is planned. Run denied-warning rustdoc/changed-crate Clippy, genuine local-patched publication dry run and affected archive inventory below10MiB, retaining exact member totals/counts and64byte compressed allowance. Any provider repair that contradicts an unlisted HLD section requires a plan revision first.
- WASM/binding graph: check both ordinary WASM consumers and no-default-font path when integrated with the sprint's feature-wiring union. This story adds no crypto feature or dependency itself and must not accidentally unify new features into unrelated consumers.
- External package inspection: pin actual Python XML/ZIP tools and compare selected physical source parts independently of production text getters. Native schema/metadata claims require actual saved artifacts, not generated success prose.
- Packaging/artifact observations: freeze ordinary wheel/CLI size/hash/profile/source before feature edits, compare matched after artifacts and explain actual deltas. The existing crates.io10MiB rule does not become an invented wheel/binary ceiling. Root owns the shared measurement carrier and all22archive observation.

## Hash harness

Expected unchanged, all49entries and the shared pixel manifest. New form fixtures are constructed independently. No shared sample baseline write lease is granted. F-X241 extends tools over existing form providers and does not tune rendering or units. Unexplained deltas stop completion.

## Implementation checklist

- [x] Adopt after S94 opens, confirm pending status/branch/Base, approve the consolidated design and claim an isolated worktree.
- [x] Authenticate the live Issue380 criteria and source-built five-kind acceptance template. Preserve already supported baseline behavior.
- [x] Freeze matched-profile prefeature wheel/CLI/package artifacts and execute missing-surface RED controls.
- [x] Add bind wrapper and frozen legacy snapshots/setter with exact stubs, type admission and prepared publication.
- [x] Extend the existing CLI Fill command with bounded flat JSON, duplicate detection and strict mixed selector preflight.
- [x] Apply SDTs then guarded physical-identity legacy writes on one candidate, retain original output row order, refuse any overlap/ordinal drift and publish only after final reopen/validation.
- [x] Prove five-kind saved/reopened values, raw preservation, held owners, ambiguity/late-failure atomicity and meaningful semantic negatives.
- [x] Qualify fill/sign semantics with the existing native providers, preserve old --tag behavior and hand exact fixtures to F-X240 for the later public cross-story gate.
- [x] Pass scoped checks/riders, independent microscope, integrator-owned HLD/archive observations and structured preparation without worker ledger edits or push.

## Open questions

No unresolved blocking product question remains. The orchestrator approves the concrete approach and risk riders in this plan under the authorized S94 scope. Native and external controls are future gates until actually executed.
Existing SDT callable/snapshot and native bind behavior, physically qualified legacy identity, strict flat mixed JSON selection and staged whole-operation refusal are approved. Validate full legacy identity after SDT replacement before ordinal-based writes.

## Approved sprint waves and exclusive resources

Wave1 runs F-X238 and F-X247 in separate worktrees. F-X238 owns Word parity tests and the existing binding naming/stub lanes, plus its PowerPoint XML-divergence pointer. F-X247 owns native math/conversion, the existing OfficeMath oracle, Word core math tests and the sole published-package numeric catalog lease in this wave. Neither edits the other's test entrypoint or source files.

Wave2 runs F-X239. Wave3 runs F-X241. Wave4 runs F-X240. These stories normally serialize because their actual edits share binding carriers and the source file scripts/readme_doctests.py for mandatory genuine archive measurements. If F-X238 is blocked, its fully checkpointed, frozen worker may explicitly release its source, test, HLD and cache leases so independent F-X239 can start from clean canonical source without inheriting or integrating that blocked implementation. F-X247 may continue on its disjoint math lanes, retaining the exclusive numeric-catalog/archive lease until explicitly released. F-X239 defers that shared stage and remains unprepared until every rider passes. F-X241 also waits for actual Word core/test lease release. F-X240 follows validated integrated form source for public fill/sign integration tests. A later F-X238 resume requires explicit lease return, reconciliation against the current approved consumers, a fresh microscope and affected union checks. Operational source leases add no speculative backlog dependency and do not waive any completion gate.

Each worker updates exactly its listed HLD impact under feature-specific semantic section leases before its prepared handoff. The integrator coordinates shared HLD sections and numeric catalog ownership. Shared package observations use genuine archives, and combined observations run again after integration. Workers never update sprint delivery totals. The integrator alone writes AS_BUILT and SPRINT_TRACKER. Keep complete installed environments and target/build caches isolated or exclusively leased. The hash baseline remains exclusive, with no approved new baseline move.
