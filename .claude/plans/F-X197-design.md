# F-X197, Unify Python colour inputs and font colour formats

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: none

## Problem

Word Font.color is tuple-only at
`crates/rdocx-py/src/formatting.rs:291`, and style create/update colour
parameters are tuple-only at `crates/rdocx-py/src/document.rs:2763` and
`:2881`. Word table/cell border and shading parameters instead take strings
at `crates/rdocx-py/src/table.rs:446`, `:454`, `:1190` and `:1200`.

PowerPoint font colour accepts tuples and bare hex at
`crates/rpptx-py/src/text.rs:1230`, whereas existing fill/line/shadow
ColorFormat.rgb requires RGBColor and reports ValueError for other types at
`crates/rpptx-py/src/dml.rs:448`. Font getters disagree: Word returns
RGBColor or None at `crates/rdocx-py/src/formatting.rs:276`, PowerPoint
returns string or None at `crates/rpptx-py/src/text.rs:1365`. Neither
supports font.color.rgb. Issue299 requests a consistent input contract and
live font colour-format getter across both families.

## Spec reference

- `docs/hld/10-bindings-spec.md`, "Python API shape", specifically direct
  font, shading, border, style, PowerPoint fill/line/shadow and font contracts.
- `docs/hld/10-bindings-spec.md`, "The invalidation problem, handled loudly".
- `docs/hld/05-drawingml-model.md`, "Colour, the part everyone gets wrong"
  and "Do not touch the Word path".
- `docs/hld/14-development-backlog.md`, "F-X197, Unify Python colour inputs
  and font colour formats".

## Approach

Inventory and migrate every existing colour-setting surface, without new
files or modules. Initial complete inventory from actual sources:

| Family | Public setters |
|---|---|
| Word | Document.add_style(color=...), Document.set_style(color=...) |
| Word | Font.color direct assignment, new Font.color.rgb, Font.shading |
| Word | Table.set_borders(color=...), Table.set_border(color=...) |
| Word | Cell.shading, Cell.set_border(color=...) |
| PowerPoint | Font.color direct assignment and new Font.color.rgb for runs and paragraph defaults, including cell fonts after F-X194 |
| PowerPoint | FillFormat.fore_color.rgb for supported shape/picture/connector and cell fills |
| PowerPoint | LineFormat.color.rgb for supported shape lines and four cell borders |
| PowerPoint | ShadowFormat.color.rgb |

Repeat the source and stub inventory before completion and add any other
existing colour-taking call found. Named highlight colours are a separate
Word highlight vocabulary, not scalar RGB setters.

Use one concrete PyO3 extractor in existing oxml-py-support/src/lib.rs,
consumed directly by both current bindings:

```rust
pub fn rgb_channels(
    value: &Bound<'_, PyAny>,
    parameter: &str,
) -> PyResult<(u8, u8, u8)>;
```

Add the existing workspace PyO3 dependency to this unpublished support crate,
without a new feature flag or extension-module setting. Both real consumers
already use the same pinned PyO3 version. Keep extension-module in the binding
crates. No new type, trait, generic parameter, wrapper or file is needed.
Native RGB/hex representation is adapted at each existing call site.

Accept tuple-compatible RGBColor and existing legacy channel triples, plus
exactly six ASCII hex digits with an optional single leading hash. Normalize
hex uppercase. None omission/clearing and existing Word auto are handled by
the calling property before shared extraction, retaining their original
surface-specific semantics. Do not add generic policy switches to the helper.

Validate before mutable source access. Wrong types raise TypeError naming
the full affected parameter, accepted forms and actual Python type. Invalid
string syntax raises an explicit ValueError. Reject malformed hash forms,
non-ASCII digits and arbitrary string coercion. Failed assignments leave
source and revisions unchanged.

Add Word PyColorFormat inside existing formatting.rs with document and
ContentPath ownership, using existing paragraph-location/run resolution.
Extend FontSnapshot and FontUpdate with available native theme metadata.
Read RunRef::color_theme, color_theme_tint and color_theme_shade, and author
through Run::set_color_theme. Native methods exist at
`crates/rdocx/src/run.rs:768`, `:1399`, `:1405` and `:1409`.

Extend existing PowerPoint PyColorFormat and concrete ColorSource enum in
dml.rs with a font source. Its path distinguishes paragraph default font
from run font. Reuse direct font-property read/update behavior through
explicit pub(crate) functions in existing text.rs. No second font-specific
PowerPoint format class or generic abstraction.

Public signatures:

```python
Font.color -> ColorFormat
Font.color = RGBColor | str | legacy_rgb_triple | None
ColorFormat.rgb -> RGBColor | None
ColorFormat.rgb = RGBColor | str
ColorFormat.type -> str | None
ColorFormat.theme_color -> str | None
ColorFormat.theme_color = str | None
```

Font format rgb may also clear with None, delegating to the existing direct
font clear operation. Do not extend None clearing onto fill/line/shadow
surfaces whose existing contracts reject it.

Use supported native tokens for type and theme_color rather than inventing
new enum modules. Type distinguishes rgb, theme, Word auto, and PowerPoint
preset/system where modeled. Validate theme tokens against each family's
supported model. Scheme colours read no guessed RGB. Word retains cached
literal colour and tint/shade when editing theme references. Preserve
appropriate PowerPoint colour transforms and foreign source metadata.
RGB replacement retains existing family-specific transform semantics.
Existing fill foreground prerequisites, line solidification and shadow
opacity behavior remain intact. Format getters do not create source
properties or advance revisions.

Scalar RGB getters consistently return RGBColor, whose string form remains
uppercase hex. Word border tuples become
`(style, size, RGBColor | "auto" | None)`. Word shading reads
`RGBColor | "auto" | None`. Preserve non-RGB auto instead of inventing an
RGB value. Update existing Python RGBColor.from_string to recognize the
same optional hash syntax with explicit validation.

The unconditional live Font.color getter is a deliberate pre-1.0 source
break from Word RGBColor|None and PowerPoint str|None. It is explicitly
requested by Issue299 and authorized by the sprint scope. Preserve direct
assignment and legacy tuple inputs. Direct scalar reading moves to
font.color.rgb. Update the HLD's old PowerPoint string-getter promise and
every affected example/test/stub, including comparisons and None checks.

## Rejected alternatives

- Theme-to-RGB coercion destroys source semantics and guesses theme mapping.
- A second PowerPoint font colour-format class duplicates existing formats.
- Arbitrary str() conversion hides caller mistakes.
- Removing auto or legacy tuples causes unrelated compatibility breaks.
- A compatibility proxy masquerading as string/tuple and ColorFormat adds
  ambiguous equality and truthiness behavior instead of a clear migration.

## Test plan

**Test gate**: regression, copied from the backlog.

| Category | Test | Asserts |
|---|---|---|
| regression | `test_every_word_colour_setter_accepts_rgb_and_optional_hash_hex` | Parameterized complete Word inventory accepts RGBColor, bare/lowercase hex and hashed hex. |
| regression | `test_style_colour_inputs_are_validated_before_style_mutation` | Create/update styles support both forms and failures preserve the style graph. |
| regression | `test_word_font_colour_format_supports_rgb_theme_and_direct_assignment` | Unset format, nested rgb, direct assignment, theme metadata and cached literal survive reopen. |
| regression | `test_word_scalar_rgb_getters_return_rgbcolor_and_keep_auto` | Font rgb, shading and border snapshots return consistent RGBColor or original sentinel. |
| regression | `test_every_powerpoint_colour_setter_accepts_rgb_and_optional_hash_hex` | Parameterized run/default/cell font, fill, line, cell border and shadow inventory. |
| regression | `test_powerpoint_font_colour_format_preserves_theme_and_transform_sources` | Scheme metadata, alpha/tint/luminance transforms, read-only getter and direct setter preserve source semantics. |
| regression | `test_invalid_colour_inputs_name_the_parameter_and_leave_source_unchanged` | Actual type, accepted forms and parameter appear in TypeError, malformed hex raises ValueError, source/revisions remain unchanged. |
| regression | `test_colour_format_handles_follow_structural_invalidation` | Word font and PowerPoint font/fill/line/shadow format handles obey stale-handle rules. |
| regression | `test_colour_inputs_and_nested_font_formats_reach_actual_rendering` | Saved/reopened documents/decks and actual deterministic PDF/raster colours, including theme controls. |

Use existing Word test_formatting_tables.py and test_core.py, PowerPoint
test_documented_examples.py, test_shared.py, both existing typing_smoke.py
files and handwritten stubs. No new integration binary or test file. Generate
theme/auto/system/preset/foreign metadata XML controls in code. Include
unchanged getter reads, RGBColor string behavior and legacy tuple regression
controls. Reproduce the missing nested font API against the claimed base.

Run installed wheel runtime tests, exact mypy==2.3.0 --strict and recursive
stubtest. If oracle comparisons are used, assert python-docx==1.2.0 and
python-pptx==1.0.2 and document the approved getter adaptation.

## HLD impact

- `docs/hld/03-architecture.md`: shared Python support dependency and extraction ownership.
- `docs/hld/14-development-backlog.md`: full shared Issue299/303/308 colour acceptance.
- `docs/hld/10-bindings-spec.md`: complete input/getter inventory, source
  preservation, type/theme tokens, auto sentinel, live format and deliberate
  getter compatibility change.

## Risk routing

- Dependency graph: HLD03, inspect the support PyO3 dependency and both actual consumers.
  Preserve oxml family direction and keep PyO3 outside both WASM graphs.
  Run the support crate's actual existing unit entrypoint with Python runtime
  initialization, and retain binding exclusions on workspace test binaries.
- Theme colour: read HLD05 colour sections. Run source-preservation controls
  for themes and transforms. Do not modify legacy Word tint/shade math.
- PyO3: read HLD10. Run installed runtime, strict typing and stub checks,
  and `cargo check --target wasm32-unknown-unknown -p rdocx-wasm -p rpptx-wasm`.
  Exclude both binding crates from workspace test binaries.
- Public API: explicitly state the intentional pre-1.0 getter return-type
  break and scalar-read migration. Run applicable package dry-runs and
  `.crate` size assertions for affected published package surfaces.
- Rendering controls use deterministic fonts, with no system-font baseline.
- External oracle comparisons, if used, earn exact version assertions and
  `.claude/skills/differential-testing.md` reading.
- No parser or serializer implementation change, new file/module/crate,
  trait/generic or feature flag is intended. If actual implementation earns
  an additional rider, revise the plan before crossing that boundary.

## Hash harness

Expected unchanged. Binding normalization writes equivalent existing native
values and changes no colour resolver arithmetic. An unexplained delta
blocks completion.

## Implementation checklist

- [x] Reproduce the nested font colour failure on the claimed base.
- [x] Freeze and recheck the full existing setter/getter inventory.
- [x] Implement the single shared PyO3 colour extractor and both consumers.
- [x] Add Word live format and extend existing PowerPoint format.
- [x] Expose supported type/theme source values without flattening themes.
- [x] Migrate styles, borders, shading and every inventoried setter.
- [x] Normalize scalar getters while preserving auto and legacy inputs.
- [x] Update existing examples, runtime tests, stubs and typing smoke.
- [x] Run installed regressions, rendering controls and risk riders.
- [x] Complete scoped verify and a zero-finding microscope.

Exclusive paths include both Python families' existing formatting/text,
dml/table/document binding files, RGBColor Python files, exports, stubs,
README/examples and existing tests. Serialize after F-X194 because its text
resolver and nested-cell font paths overlap. Neither feature changes the
exclusive hash baseline.

## Open questions

None. Issue299 explicitly requests the live getter. Preserve direct assignment
and tuple inputs, document scalar reading through rgb, retain existing auto,
None and colour-source semantics, and use supported native theme tokens.

## Approved expanded colour intake

Issue303 explicitly requires the shared extractor described above, replacing
the earlier package-local proposal before implementation begins. Issue308
adds Python colour-format type, theme colour and brightness expectations.
They belong here, not duplicate implementations in F-X200 or F-X205.
Expose supported MSO_THEME_COLOR values through existing enum/export files,
retaining accepted native token inputs and source semantics. Do not introduce
a new enum module merely to mimic an import path. Use existing native colour
transform models to read/write brightness for the PowerPoint font/fill/line/
shadow formats. Preserve unknown transforms, cached literal/source distinction
and alpha. Verify the exact supported brightness contract against pinned
python-pptx1.0.2, rather than inventing a tint formula. Word retains its native
tint/shade vocabulary and deliberately pinned conversion convention.

Add shared helper malformed/type/legacy controls to the existing support test
module and installed binding suites, then explicit theme-enum and brightness
save/reopen, unknown-transform and rendering controls. Shared extraction must
validate before any mutation. No fresh parser model, file or dependency beyond
the already pinned workspace PyO3 is approved. If actual missing native
transform support earns parser changes, revise the risk contract first.

## PR317 reuse assessment

Assess immutable head3d056f46e97e9cc1caf2969b5096eeb6775a5fa0. Reuse
concrete Word font colour view, existing PowerPoint colour-source extension,
actual setter inventories and installed test/typing migration scaffolding.
Adapt duplicated Word/PPT extractors into the one approved neutral support
helper. Keep parameter-qualified errors and all accepted legacy triples.

Do not adopt Word themeTint/themeShade removal at contributor
formatting.rs:125/127, auto-to-None scalar conversion at Word lib.rs:99/102,
or universal None rejection for nested font RGB at PPT dml.rs:615/617.
Preserve the approved sentinel, modifiers and font-only clearing contract.
Both contributor format implementations lack brightness and the full308
contract, so complete that once here. Qualified native source and existing
transform preservation remain mandatory. No new enum file is approved.
Use existing enum/export files for the required values, retaining public
spelling where the existing package layout can support it without a new file.
Contributor archive/hash records and host gate claims are not local evidence.

## Pinned brightness oracle preparation

A root-owned independent python-pptx1.0.2 probe executed56 cases over run
font, paragraph font, solid fill and line colours. All56 colour trees retain
their exact structure after save/reopen, and all24 refused operations retain
the prior tree. Source module hashes, enum inventory, inputs, errors and
structural results are authenticated under
`/private/tmp/S91-F-X197-brightness-oracle/receipt.json`. This is oracle
preparation, not a local F-X197 pass or rendering proof.

Positive brightness writes lumMod plus lumOff, negative writes lumMod only,
and zero removes those two adjustments while preserving existing alpha and
tint. The measured0.123456 input stores87654/12346 and reads0.12346. Endpoints
-1 and1 store0 modulation, with100000 offset only at1. Refused out-of-range,
infinite and wrong-type values retain source. The oracle also accepts bool
True as1 and NaN as zero, clearing luminance. These are measured compatibility
discriminators, not an automatic local policy choice. Classify any deliberate
divergence in the implemented control and cited HLD before completion.
Unknown-transform, inheritance, local source/token, shadow and rendering
controls remain required under the existing approved acceptance.
