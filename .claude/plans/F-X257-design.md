# F-X257, Complete remaining Word names and legacy dropdown diagnostics

**Status**: completed
**Sprint**: S95
**Size**: L
**Depends on**: F-X238, F-X241, F-X252

## Problem

Issue386 names missing Word compatibility contracts. Frozen Section snapshots
at crates/rdocx-py/src/document.rs:2504 expose margin_left rather than live
left_margin. TabStop at crates/rdocx-py/src/formatting.rs:973 is immutable,
but assignment lacks an actionable replacement route. Custom properties have
checked set/delete providers at document.rs:3272 without mapping operations.
The missing paragraph/run private names lack the existing checked XML guidance.
Legacy dropdown refusal at document.rs:6973 misidentifies strings as bool.

ColorFormat.type at formatting.rs:778 returns strings. Matching the reported
python-docx enum comparisons requires an explicit source compatibility choice,
including the matching stubs and existing consumers.

## Spec reference

- docs/hld/14-development-backlog.md, F-X257. Test gate: regression.
- docs/hld/10-bindings-spec.md, Python API shape, Python Word formatting and checked list publication, and Python Word document operations.
- docs/hld/12-testing-strategy.md, Binding tests, First-draft Python name qualification, and Integrated Python compatibility qualification.
- docs/hld/05-drawingml-model.md, Colour, the part everyone gets wrong, and Do not touch the Word path.
- .claude/skills/differential-testing.md, pinned Python object-model oracle and independent saved-package observations.

## Approach

Keep existing owners and native publication. Add explicit read and assignment
refusals for frozen Section.left_margin naming
document.section(snapshot.ordinal).left_margin. Preserve margin_left and the
other existing snapshot fields. Refuse TabStop.alignment assignment with the
delete-and-readd collection route, preserving position and leader. Do not
suggest adding at the still-occupied position or create child mutation owners.

Implement CustomProperties.__setitem__, __delitem__ and __contains__ through
the existing checked set/delete/get behavior. Keys are str, with TypeError for other key types and no coercion. Names
remain case-sensitive. Empty-string assignment retains ValueError, empty
membership is false, and missing or empty deletion raises KeyError.
Membership does not create a properties part. Existing get
returns an immutable CustomProperty and existing delete retains bool behavior.
Validate all arguments before mutation and reuse current candidate publication.

Add Paragraph._p, Run._r and Run._element diagnostic properties. Validate
current ownership before raising NotImplementedError with the actual xml and
replace_xml(xml) alternative. No lxml object or second owner graph is added.
Legacy dropdown non-integer refusal names the actual Python type, nonnegative
integer index and legacy_form_fields snapshot choices. Preserve bool/negative/
out-of-range refusal, typed identities, native bounds and exact no-op behavior.
Do not apply SDT display/value selection to legacy forms.

The user explicitly approved the source compatibility change. Add the small rdocx.enum.dml module
with pinned python-docx1.2.0 MSO_COLOR_TYPE members RGB=1, THEME=2 and AUTO=101.
Return those enum values or None from ColorFormat.type, retaining exact source
color selection and all setters. Word theme_color retains str or None. Update stubs, existing string-comparison
consumers and current documentation together. Document the affected getter
return migration for release notes despite patch numbering. Do not return a
custom value that compares equal to both strings and integer enums.

All product edits remain in existing source, test and stub files except the
explicitly requested enum module, which the user explicitly authorized.
F-X256 edits the same Word carriers and integrates first. Re-read its resulting
source and preserve its numeric contract before starting this implementation.

## Rejected alternatives

- Mutable snapshot proxies add an unrequested occurrence owner model.
- A generic mapping wrapper adds indirection around existing concrete methods.
- String/enum dual equality violates ordinary equality or hashing expectations.
- Blanket AttributeError or a successful import alone does not qualify the reported contracts.
- Default-template lists and one-dimension pictures already have qualified alternatives and earn no new completion credit here.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| regression | frozen_section_margin_names_the_live_owner_route | Read/assignment refuse with exact live alternative, source and held owners survive, supported typed live edit saves and reopens |
| regression | immutable_tab_alignment_names_delete_and_readd | Refusal preserves bytes and handles, positive delete/readd preserves position/leader and writes selected alignment |
| regression | custom_property_mapping_reuses_checked_publication | Set/delete/membership, missing deletion, wrong keys/values, two handles, no-op and absent-part membership preserve existing semantics |
| regression | private_word_names_point_to_checked_xml | Each exact name raises actionable NotImplementedError, stale owners retain established errors, checked replacement still works |
| regression | legacy_dropdown_reports_received_type_and_index_route | String, bool, negative and out-of-range controls preserve source, selected integer index and snapshot choices survive reopen |
| regression | word_color_kind_matches_the_pinned_enum | Actual enum identity/value for RGB/THEME/AUTO/None, imports, saved XML, unchanged setters and explicit migrated consumers |
| integration | immutable_issue386_word_name_rows | Original script bytes stay pinned, selected row outcomes and independent saved XML are reported separately from Issue385's twelve bodies |
| differential | pinned_word_color_and_supported_provider_controls | python-docx1.2.0 color values and saved source agree, snapshot refusal and custom-property extension remain declared divergences |
| installed | fresh ordinary Word suite, strict typing and stubtest | Actual new wheel/native/RECORD identity, complete suite without silent skips, direct imports and typed mapping operations agree |

The backlog test gate is regression. Extend existing test modules and typing
consumers. Assert package bytes, held owners and unrelated parts around every
invalid operation. Original self-catching exit zero is not acceptance. Existing
qualified list and picture refusals remain explicit controls without new credit.

The worker uses an explicitly labeled selected-row adapter against its fresh
family wheel. Root executes the untouched whole Issue386 script once against
the paired final artifacts. Missing peer imports are not behavioral RED.

## HLD impact

- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md

## Risk routing

Public binding API: read HLD10 and structural rules, document the approved
color return change. Binding-only source has no affected publishable Rust
crate. If native published source becomes necessary, stop for scope revision
and genuine affected publication/archive checks. Root retains final integrated
packaging and any numerical catalog amendment.
PyO3/WASM: fresh installed Word suite, strict typing/stub checks, changed binding
clippy and both WASM checks, excluding binding Rust test binaries. External
oracle: pin python-docx1.2.0 and record actual versions and declared divergences.
Theme color: read HLD05, preserve source/tint/shade behavior with saved XML
controls, without changing color math. The user explicitly authorized the new enum file. No native parser, dependency, font,
default-feature or rendering change is planned. Scoped policy/hash checks and
the final integrated union remain mandatory.

## Hash harness

Expected unchanged49 hashes and seven deterministic pixel buffers. This changes
Python projections and diagnostics, without changing accepted native rendering.
No baseline lease.

## Implementation checklist

- [x] Resolve color return compatibility and enum module authorization.
- [x] Reconcile the completed F-X256 prefix before touching overlapping files.
- [x] Establish actual old-source failures and preserve the exact Issue386 input.
- [x] Implement bounded diagnostics, mapping operations and approved enum projection.
- [x] Pass focused saved-file/atomicity/oracle controls, fresh installed typing and scoped riders.
- [x] Obtain independent zero-finding microscope and prepare validated handoff.
- [x] Update exactly the named HLD sections with concrete current contracts.

## Open questions

None. The user explicitly approved matching enum-valued color getters with
source compatibility migration and the two requested modules, rdocx.enum.dml
and rpptx.enum.lang. The existing patch version targets remain unchanged.
The bounded source, owner, language and refusal choices above are approved.
