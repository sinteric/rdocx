# F-X258, Complete remaining PowerPoint names color and language contracts

**Status**: completed
**Sprint**: S95
**Size**: M
**Depends on**: F-X239, F-X211b

## Problem

Issue386 names six PowerPoint rows without the requested compatibility or
actionable alternative. Slide lacks the registered slide_id projection,
ColorFormat.type/theme_color at crates/rpptx-py/src/dml.rs:1298 return strings,
Font at text.rs:2386 exposes only the language tag route, and the shape/body
private XML names lack guidance to the existing checked Element API.

## Spec reference

- docs/hld/14-development-backlog.md, F-X258. Test gate: regression.
- docs/hld/10-bindings-spec.md, Python API shape, Native PowerPoint text layout, Live PowerPoint masters and layouts, and PowerPoint Element XML surface.
- docs/hld/12-testing-strategy.md, Binding tests, Notes owners and unit refusal qualification, and Integrated Python compatibility qualification.
- docs/hld/05-drawingml-model.md, Colour, the part everyone gets wrong, and Do not touch the Word path.
- .claude/skills/differential-testing.md, pinned object-model comparison and structural saved-package observations.

## Approach

Expose read-only Slide.slide_id:int after current physical-owner validation
and path resolution. Read existing SlideRef::id at crates/rpptx/src/lib.rs:21152,
also used by Python sections. Do not allocate new IDs, cache ordinals or change
native source. Preserve imported noncontiguous registrations through moves and
save/reopen, and retain existing retirement behavior.

Add diagnostic-only Shape._element and TextFrame._txBody. Validate the owner
first, then raise NotImplementedError naming shape.element or
text_frame.element and its actual xml/replace_xml methods. Stubs use the
existing never-return annotation. Retired owners retain their causal stale
errors. No lxml graph, alias returning the wrong object type or second owner.

The user explicitly approved the source compatibility change. Project actual
ColorChoice through the existing dml_enum helper and MSO_COLOR_TYPE. Map Srgb
to RGB, Scheme to SCHEME, Preset to PRESET and System to SYSTEM. Unset type
remains None. ColorFormat is shared by fonts, fills, lines, borders, shadows,
patterns and gradients, so qualify each affected context rather than only
the example font. Do not change source color selection, theme resolution,
brightness, tint/shade, transforms or retained opaque XML.

ColorFormat.theme_color returns the matching MSO_THEME_COLOR_INDEX for the
sixteen supported scheme tokens. Match actual python-pptx1.0.2 executable
branches: nonscheme color returns NOT_THEME_COLOR, while no direct color
raises AttributeError. Unknown imported scheme tokens and phClr cannot be
guessed into an enum. Refuse their enum projection actionably, naming preserved
source inspection through the owning element.xml. Preserve their XML and the
existing supported string setter route. None clearing and current string/enum
setter inputs retain their contracts. Update current string-return regressions,
stubs and documentation under the explicit approved migration, without dual
equality wrappers or globally changing enum equality.

The user explicitly authorized the new module. Add the concrete rpptx.enum.lang module
with the216 literal numeric/tag declarations authenticated from
python-pptx1.0.2. Use one finite table, exact pinned member names and values,
and explicit conversion behavior. Do not add a generic enum base, dependency,
runtime oracle import or public locale mapping API. Font.language_id reuses
the existing physical Font owner and checked property update. Preserve the
existing Font.language string API and its current authored tag admission.

Absent language_id returns MSO_LANGUAGE_ID.NONE. None and NONE clear. MIXED,
bool, wrong types and unmapped numeric values refuse before mutation. Preserve
the pinned declaration-order/alias behavior when converting supported tags,
without blindly inverting the empty NONE/MIXED strings. An unknown imported
tag raises an actionable language_id refusal naming Font.language, preserving
that string and all source. No LCID inference, case normalization or locale
engine. Qualify every declared enum mapping plus meaningful positive and
negative saved-file controls.

Product edits use existing src/slide.rs, dml.rs, text.rs, shape.rs and the
existing stub and the precise existing enum/dml.py docstring that currently
calls color enums independent of ColorFormat.type, plus the specifically
requested enum/lang.py.
Extend existing test_documented_examples.py and typing_smoke.py. HLD sections
shared with Word workers require explicit separate write leases or serial
completion reconciliation. Root owns numerical catalog rows and baselines.

## Rejected alternatives

- A copied slide ordinal is not the producer registration ID.
- Returning checked Element under a private lxml name advertises unsupported behavior.
- A custom dual string/enum value breaks normal equality or hashing contracts.
- A two-member language imitation would falsely imply the requested enum inventory.
- Generic locale lookup or normalizing unknown imported tags expands scope and loses source meaning.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| regression | slide_id_uses_registered_source_identity | Imported noncontiguous IDs, reorder/save/reopen, live resolution and retired-owner errors |
| regression | powerpoint_color_getters_match_pinned_enums | Each modeled color kind, sixteen scheme slots, nonscheme sentinel, absent AttributeError and unknown/phClr actionable refusal |
| regression | color_projection_preserves_all_existing_owners | Font/fill/line/border/shadow/pattern/gradient contexts preserve transforms, opaque XML and saved source, existing setters still work |
| regression | language_id_uses_finite_pinned_mapping | All216 declarations and conversion aliases, en-US/fr-FR save/reopen, absent NONE and None/NONE clearing |
| regression | language_id_refuses_without_publishing | MIXED/bool/type/unmapped code and unknown imported tag, complete bytes and held-owner preservation, later valid string/enum edits |
| regression | private_powerpoint_names_point_to_checked_element | Exact diagnostic names and alternatives, live/stale/removed body controls, real Element replacement remains supported |
| differential | powerpoint_names_match_pinned_python_pptx | Actual python-pptx1.0.2 enum identity/value and structural outputs, with explicit private-graph and unknown-source refusals |
| installed | fresh ordinary PowerPoint suite, strict typing and stubtest | Actual wheel/RECORD/native identity, direct language import, enum returns and complete installed suite agree |
| integration | immutable_issue386_powerpoint_rows | Exact original six labels and actual outcomes, independently observed source, no self-catching exit-zero credit |

The backlog test gate is regression. Preserve original Issue386 bytes and keep
its denominator separate from Issue385's twelve-body union. For every refused
assignment, compare complete before/after bytes and held handles, then exercise
the valid supported route. Imported chart, table-cell, layout/master and
paragraph-default owners remain required positive controls. Report actual
oracle versions and outputs without inferring Python3.14 qualification.

The worker uses an explicitly labeled selected-row adapter against its fresh
family wheel. Root executes the untouched whole Issue386 script once against
the paired final artifacts. Missing peer imports are not behavioral RED.

## HLD impact

- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md

## Risk routing

Published Python API: read HLD10 and structural rules, disclose approved getter
return migration and preserve setter compatibility. Binding-only source has no
affected publishable Rust crate. If native published source becomes necessary,
stop for scope revision and genuine affected publication/archive checks.
PyO3/WASM: binding clippy, fresh installed complete PowerPoint suite, strict
typing/stub checks and both WASM graphs, no binding Rust test binary. Theme
mapping: read HLD05 and retain all source transforms/color math with independent
saved XML controls. External oracle: pin python-pptx1.0.2 and actual source,
version and output identity. The user explicitly authorized the enum module.
No parser, dependency, font or layout change is planned. Root retains scoped
policy/hash floors and the final integrated packaging/full union.

Adding the diagnostic in shape.rs changes a strict F-X253 historical mutant
source hash. Root must review an exact additive-only applicability exception
or run fresh actual mutants against final source. Do not weaken whole-file
checks globally or relabel historical executions as current-source runs.

## Hash harness

Expected unchanged49 hashes and seven deterministic pixel buffers. Native
paint and stored color/language semantics remain unchanged. No baseline lease.
Preserve F-X253 actual master/chart controls through final-source qualification.

## Implementation checklist

- [x] Resolve enum getter compatibility and new-module authorization.
- [x] Establish actual old-source failures and preserve the original Issue386 input.
- [x] Implement registered ID and checked XML diagnostic projections.
- [x] Implement only approved color returns and finite language mapping.
- [x] Align stubs, existing consumers and independent saved-file/atomicity controls.
- [x] Pass focused installed/oracle/scoped checks and independent zero-finding microscope.
- [x] Update exactly the named HLD sections and prepare validated handoff.

## Open questions

None. The user explicitly approved matching enum-valued color getters with
source compatibility migration and the two requested modules, rdocx.enum.dml
and rpptx.enum.lang. The existing patch version targets remain unchanged.
The bounded source, owner, language and refusal choices above are approved.
