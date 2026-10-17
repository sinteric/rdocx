# F-X254, Refuse invalid PowerPoint values XML and empty edits

**Status**: completed
**Sprint**: S95.11
**Size**: L
**Depends on**: F-X204, F-X206, F-X211b, F-X220

## Problem

Issue385's remaining PowerPoint refusals span three existing ownership routes.
Python save and dimension setters delegate without the requested format/range
admission (`crates/rpptx-py/src/presentation.rs:942`, `:864`, `:1782`). Frame
flips stage geometry before applying flags (`crates/rpptx-py/src/shape.rs:225`).
Table style mutation checks GUID spelling but not actual availability
(`crates/rpptx/src/lib.rs:22094`). These calls can succeed while storing values
PowerPoint cannot honor.

The checked XML scanner enforces order for recognized children but turns
unrecognized children into delegated opaque content
(`crates/rpptx/src/lib.rs:1519`, `:1600`). This admits four concrete malformed
owner mutations from the original reporter. Existing validation checks model
and package invariants without reporting those schema frontiers (`:3770`).
The CLI only treats zero implicit matches as an error for a single pair
(`crates/rpptx-cli/src/commands.rs:611`, `:632`), and font removal publishes even
when no face was removed (`:1973`, `:1990`).

All prerequisites are done in `docs/sprints/BACKLOG.md`: F-X204 at767-773,
F-X206 at773, F-X211b at780 and F-X220 at791. Confirm exact rows again at claim.
The story's **test gate** is **regression**, verbatim from
`docs/hld/14-development-backlog.md:8325`.

## Spec reference

- `docs/hld/06-presentationml-model.md`, Checked shape and relationship
  authoring, Validation and Checked XML owner replacement.
- `docs/hld/04-opc-and-packaging.md`, Checked PowerPoint XML transactions.
- `docs/hld/10-bindings-spec.md`, Licensed embedded fonts in Python and CLI.
- `docs/hld/12-testing-strategy.md`, Binding tests and rendering regression
  controls, for local/public qualification and the final deterministic union.
- `docs/hld/01-glossary.md`, units, and `CLAUDE.md`, Things that are
  deliberately wrong, for unchanged EMU conversion and truncation.

Finite schema authority is the original ECMA-376 Part4 5th edition December2016
ZIP retained under `/private/tmp/S95-F-X252-schema-admission-research-1`, nested
`OfficeOpenXML-XMLSchema-Transitional.zip`, `pml.xsd` and `dml-main.xsd`. The
relevant declarations are `CT_Shape`, `CT_Slide`, `EG_ChildSlide`,
`CT_PresetGeometry2D`, `CT_TextParagraph` and `EG_TextRun`. Authenticate the
original receipt/ZIP hashes before implementation and retain the exact extracted
particles in private evidence. Part3 5th edition MCE and W3C Namespaces XML1.0
pins from the same retained research establish namespace and Ignorable meaning.
This is a finite schema-backed admission contract, not full schema conformance
or native execution evidence. Do not change default import to Strict OOXML.

## Approach

### Refuse selected authored values before publication

1. Python `Presentation.save` refuses a case-insensitive `.pdf` path with
   `ValueError` naming `to_pdf`, before native serialization or filesystem
   writes. Other extensions and `to_bytes` keep existing behavior. Do not add
   streams or unrelated format policy in this story.
2. Python width/height setters require the selected integer EMU dimension in
   inclusive `[914400, 51206400]`, corresponding exactly to1through56 inches.
   Do not change generic OXML `validate_dimension`, used by other geometry.
   Put the range admission in the existing binding `set_slide_size` route,
   before native mutation. Preserve current integer/Length conversion policy.
   Imported out-of-range dimensions remain readable and source-preserved on
   ordinary round-trip. Setting an unchanged imported value is a true no-op.
   Validate only an authored changed axis, so a valid correction does not
   silently rewrite or reject an unrelated retained imported axis.
3. `Shape.flip_h` and `flip_v` refuse a changed flag on a table or chart
   graphic frame before geometry materialization. Read the existing physical
   frame payload, not shape-name strings. Exact unchanged flags remain no-op,
   including clearing an already false flag. Ordinary shape/picture/group
   flip semantics remain unchanged. Use `ValueError` naming unsupported frame
   flip behavior. This binding refusal avoids silently broadening native raw
   transform APIs used for import/source inspection.
4. `Table.style_id` preserves `None` clear and unchanged-source no-op. A newly
   selected value must pass existing braced-GUID syntax and resolve either to
   an existing built-in style or an actual uniquely identified deck style.
   Reuse `oxml_drawing::table::CT_TableStyle::builtin`, whose74-style inventory
   is already implemented (`crates/oxml-drawing/src/table.rs:2200`), not a new
   GUID catalogue or a hardcoded single default. Use GUID identity with ASCII
   case-insensitive comparison, preserving the selected spelling on write.
   Reuse existing native table-style relationship resolution
   (`crates/rpptx/src/lib.rs:17192`) and qualified style parsing (`:16667`).
   Resolve from the actual presentation part, verify one internal relationship,
   expected content type and one qualified matching style. Missing, external,
   wrong-type or duplicate/ambiguous definitions refuse atomically. Foreign
   `tblStyle` lookalikes and a default-style fallback do not prove availability.
   Imported unknown unchanged IDs remain exact. Implement this at the existing
   native `TableMut::set_style_id` ownership boundary if its owner can supply
   the presentation graph without public-field changes. Otherwise use the
   existing Python staged table-properties route with one bounded private
   native availability helper. Do not create a second generic style API.

### Finite checked XML admission and validation

Reuse the existing `check_ppt_xml_input`, `ppt_xml_child_order` and
`replace_checked_owner_xml` transaction (`crates/rpptx/src/lib.rs:1321`, `:1519`,
`:5255`). Add only the following four schema frontiers, encountered in actual
supported owner traversal. Do not infer a model-wide XML grammar from renderer
projection or traverse delegated chart, timing, extension or foreign payloads
as though they were ordinary shapes.

| Physical qualified parent | Admitted direct qualified children and cardinality |
|---|---|
| `p:sp` / CT_Shape | `p:nvSpPr` exactly1, `p:spPr` exactly1, then optional single `p:style`, `p:txBody`, `p:extLst` |
| `a:prstGeom` / CT_PresetGeometry2D | optional single `a:avLst`, no additional recognized-namespace child |
| `a:p` / CT_TextParagraph | optional single `a:pPr`, then any interleaving of `a:r`, `a:br`, `a:fld`, then optional single `a:endParaRPr` |
| `p:sld` / CT_Slide | `p:cSld` exactly1, optional single `p:clrMapOvr`, `p:transition`, `p:timing`, `p:extLst`, in that order |

Namespaces, not prefix spelling, decide every name. Wrong recognized namespace,
unknown `p:`/`a:` direct children, order/cardinality errors and missing required
children at these four frontiers are actual violations. Existing checker
constraints at other frontiers remain intact. Valid field runs, shape geometry,
DrawingML subgrammars, diagrams/charts and schema extension hooks remain
admitted through their existing providers. This does not recursively validate
all OOXML attributes, drawing effects, animation or every shape grammar.

Perform exact selected snapshot no-op before new admission. Keep existing
namespace-complete selected snapshot requirements and relationship checks.
A changed replacement cannot introduce an invalid child at a finite frontier.
An original direct opaque exception may survive only at its unique same
physical owner, same structural/ancestry occurrence, same raw bytes and same
relevant namespace/MC meaning. Copies, moves, rebinding or another owner's
identical byte string do not supply provenance. Stable inherited opaque
content must not freeze unrelated typed text/style edits. Do not build a
model-wide inventory of opaque raw carriers or freeze whole SDT/field/drawing
objects because they store original XML.

Foreign extension admission uses actual physical-source ancestor MC authority:
accumulate Ignorable namespace URIs resolved where declared, preserve parent
grants through blank/local declarations and restore scope after siblings.
Prefix rebinding cannot retarget earlier grants. Normalize decoded namespace
URIs exactly once, including entity spellings. Qualified MC attributes and
unqualified Choice Requires tokens retain their applicable prefix meaning.
Recognized `p:`/`a:` namespaces remain understood and cannot authorize bogus
children through Ignorable. Preserve existing valid extension hooks and
unchanged complex MC source. Newly authored unsupported complex MC refuses
explicitly rather than guessed validation. Use bounded iterative traversal,
shared/interned namespace facts and one live path or compact route IDs, without
quadratic retained ancestor-path or repeated URI copies.

Ordinary import still preserves malformed and opaque inputs. `validate()`
reports violations at the same four schema frontiers using the same predicate,
including actual source-only children that typed projection ignores. Check the
physical source snapshot for each selected slide/layout/master part, overlaying
live dirty source as the existing owner transaction does. Do not count foreign
lookalikes under opaque extension payloads as modeled slide violations.

Add one bounded `ValidationIssue::InvalidXmlChild { part: String, path: String,
message: String }` variant to the existing diagnostic carrier, mapped to
Python kind `invalid_xml_child` in the existing `PyValidationIssue` conversion
(`crates/rpptx-py/src/presentation.rs:130`). It also covers finite particle
order/cardinality diagnostics, with the concrete parent/child path in message.
This is an explicit additive published Rust enum surface. Exhaustive downstream
matches need a new arm and release notes must disclose that source compatibility
impact. No public Rust struct fields or new modules are introduced. If a more
appropriate existing diagnostic variant is found, do not repurpose an unrelated
package issue to hide XML violations.

Refusals occur before live publication. Keep the existing cloned candidate,
serialize/reopen checks, held physical owner registry, metadata/signature
invalidation and relationship pruning (`lib.rs:5255-5305`) as one transaction.
No parser/import-wide rejection and no new public XML entry point.

### CLI publication refusals

- `replace --map`: after the existing all-pair expected-count preflight, any
  pair with omitted expect and zero matches fails before publication. Evaluate
  every pair, including later pairs after an earlier match. Explicit expect0
  remains valid. Preserve valid multi-pair ordering/count semantics and existing
  staged no-overwrite output identity/alias checks. Single `-p` behavior stays.
- `font remove`: missing family or selected kind fails before serialization
  and staging, with nonzero status and no output. Native `Option` removal
  semantics remain unchanged, so other callers can still inspect absence.
- Font commands that lack force compose existing identity checks with truthful
  availability checks (`oxml-cli-support/src/lib.rs:183`) before publication.
  Do not remove --force hints from genuinely force-capable commands. Preserve
  hardlink/symlink aliases, input identity protections and existing sentinels.

### Scope and existing files

Use existing `rpptx/src/lib.rs`, `rpptx-py/src/{presentation,shape,table}.rs`,
`rpptx-cli/src/commands.rs`, and existing Python stub/test and native/CLI
integration carriers. Add test modules inside existing integration entrypoints,
not new test binaries. Shared OXML table style providers remain read-only unless
a proven missing private capability earns a bounded plan revision. No new
product file/module/crate/dependency/feature flag, no renderer algorithm change,
no version bump and no public API beyond the explicit diagnostic enum arm.

## Rejected alternatives

- A whole recursive schema validator: exceeds the four owner frontiers and
  would wrongly turn retained unmodeled content into import failures.
- Renderer-modeled whitelist or local-name matching: rejects valid schema and
  grants foreign/rebound lookalikes false authority.
- A global raw-XML multiset: lets copies/moves and namespace reinterpretation
  borrow another owner's provenance and can freeze modeled source carriers.
- Change generic OXML positive dimensions: alters unrelated geometry/import
  contracts while the story asks for authored Python slide-size admission.
- Any braced GUID as an available style: reproduces the reported silent result.
- Change native font-removal return type or global --force wording: breaks
  callers that legitimately inspect absence or offer actual replacement.

## Test plan

**Test gate: regression.** Local readiness is actual named old-source RED and
fixed-source GREEN, scoped fast checks and independent microscope. Complete
ordinary suites, external comparisons and native Office checks run once in the
final S95.11 union. Keep their explicit pending debt in the handoff and plan.

| Timing | Category | Test | Asserts |
|---|---|---|---|
| local scoped | regression | `fx254_python_value_refusals_are_atomic` | .pdf/.PDF refusal naming to_pdf, dimensions at endpoints and +/-1EMU, unchanged imported dimension no-op, no output/package/owner changes |
| local scoped | regression | `fx254_frame_flip_axes_and_payloads` | Both axes and table/chart payloads refuse before materialization, unchanged false and valid ordinary shape flips preserved |
| local scoped | regression | `fx254_style_availability_uses_real_deck` | Existing built-in74 inventory representative family variants, actual custom style, case identity, None/noop, unavailable/foreign/duplicate/wrong-target refusal |
| local scoped | native regression | `fx254_four_xml_frontiers_refuse_before_publish` | All four exact reporter mutations plus order/multiplicity/required-particle controls, no package or owner mutation |
| local scoped | native regression | `fx254_finite_xml_delegates_valid_subgrammars` | Correct fields/line breaks, geometry/diagram/chart/extensions and non-frontier typed edits still admitted, no recursive whitelist |
| local scoped | native regression | `fx254_opaque_frontier_provenance_and_mc_scope` | Original unique unchanged islands survive scalar edits, copy/move/rebind/foreign lookalikes refuse, alias/default/entity URI/sibling/additive/blank MC controls, exact noop first |
| local scoped | native regression | `fx254_import_validation_diagnoses_four_frontiers` | Imported invalid source still opens/round-trips, validate exposes actual part/path issue, valid/foreign delegated controls clean |
| local scoped | resource regression | `fx254_xml_scope_storage_is_linear` | Safe bounded retained path/URI work counters, no deliberately huge allocations or elapsed-time-only assertion |
| local scoped | CLI regression | `fx254_map_implicit_zero_is_pairwise_atomic` | Early match/later zero map refuses, explicit expect0 and valid map pass, sentinels/input aliases retained |
| local scoped | CLI regression | `fx254_missing_font_and_output_messages` | Family/kind absent nonzero no output, existing output no unavailable --force advice, valid licensed removal, real force command unchanged |
| final sprint | original reporter | Original pinned `test_a_value_powerpoint_cannot_honour_is_refused`5cases, `test_raw_xml_with_an_unknown_or_misplaced_element_is_refused`4cases and `test_a_map_pair_without_expect_that_finds_nothing_is_refused`1body | Original marker/body bytes preserved at reporter commit91a57108f13877cfb6179a70077d425d6ebf098b, strict XPASS raw exit separate from marker-free body success |
| final sprint | installed binding | Complete ordinary installed PowerPoint suite, strict mypy consumer and stubtest at integrated source | New diagnostics and existing APIs qualified once from fresh ordinary wheel, no PyO3 Rust test binary |
| final sprint | primary/native oracle | Pinned ECMA particles and MC authority, pinned python-pptx supported positive size/style/shape controls, actual native PowerPoint open/save/reopen valid schema/frame controls | Independent saved-package expanded XML, native repair-free valid positives, deliberate rejection differences documented without claiming python-pptx rejects permissive values |
| final sprint | rendering | Existing deterministic hash/pixel corpus | No unexplained delta, no new baseline merely to pass |
| final sprint | full/riders | Full workspace all-features plus specifically justified distinct configurations absent from broader coverage, binding-inclusive clippy, both WASM targets, docs/readmes, policy/adapters, actual affected publication and archive ceilings | Run once per integrated source/config tuple, shared final union owns whole22archive measurement |

Original immutable reporter inputs are `/private/tmp/S95-issue385-ppt-tests.py`
and its GitHub response JSON. Verify their body hash against commit91a57108
before use. Do not edit gap markers or claim the schema is a native oracle.
Local implementation can transcribe the exact behavior into existing regression
carriers without running external oracle suites per worker. Capture current
prefix RED before the production fix and retain failed setup attempts honestly.

## HLD impact

- `docs/hld/04-opc-and-packaging.md`
- `docs/hld/06-presentationml-model.md`
- `docs/hld/10-bindings-spec.md`
- `docs/hld/12-testing-strategy.md`

Update source intent during implementation before microscope/final verification.
Record actual native/oracle outcomes in delivery evidence only when executed.

## Risk routing

- Parser/serializer: read HLD04 Checked PowerPoint XML transactions and HLD06
  Checked XML owner replacement. Final union includes schema order, alias/MC,
  byte-exact opaque and relationship round-trip controls. Local unique
  regressions cover new finite predicate and actual RED/GREEN atomicity.
- Unit boundary: read HLD01 units and deliberate truncation guidance. No unit
  conversion algorithm changes. Final deterministic harness unchanged.
- Published Rust API: explicit additive ValidationIssue enum variant, no
  struct-literal break. Final union performs genuine affected rpptx and
  rpptx-cli publication dry runs with all local family patches and exact
  retained-source/member/size checks, no per-worker dry run.
- PyO3: HLD10 relevant formatting/owner contracts. Final union has both WASM
  checks, complete fresh installed runtime, strict mypy and stubtest. Rust
  workspace test binaries continue excluding both extension crates, clippy
  includes both.
- External oracle: canonical differential-testing pinning, with version and
  actual tool/input hashes retained. All external/native comparisons run only
  in final S95.11 union, not after each story or nested subcommand.
- No new graph edge, dependency, bundled asset, layout change, flag, module,
  file or trait is proposed. If implementation earns one, revise first.

## Hash harness

Expected unchanged, including existing49hash and7pixel controls. Refusals and
source validation add no renderer paint. An observed delta blocks final gate
unless separately declared, isolated and independently reviewed. Do not change
baselines or performance/archive tolerances from the worker. Root owns final
numeric catalog/README reconciliation and shared baseline authority.

## Implementation checklist

- [x] Reauthenticate current branch/Base, completed prerequisites and exact reporter/schema pins.
- [x] Establish actual current-prefix RED for named local behavior tests.
- [x] Add prepublication .pdf, selected dimension and graphic-frame flip refusals.
- [x] Resolve selected styles through existing74built-ins and actual qualified deck definitions.
- [x] Add the four finite schema frontiers with source/MC provenance and linear storage controls.
- [x] Expose finite imported XML diagnostics through the explicit existing enum carrier and Python mapping.
- [x] Reject implicit zero-match map pairs and absent CLI font removals atomically with truthful output wording.
- [x] Run only unique local GREEN regressions and scoped fast checks, reusing applicable observations.
- [x] Update exactly the HLD impact files before freeze, obtain independent zero-defect/zero-smell microscope.
- [x] Prepare the explicit sprint-final-acceptance handoff with named pending final checks, no delivery completion.
- [x] At integrated final gate, execute original reporter groups, complete installed/typing/stub/native/oracle/rider union once.
- [x] After actual final acceptance, write truthful delivery records and compatibility note, complete plan once.

## Open questions

The user explicitly approved the diagnostic variant on16October2026, including
the exhaustive Rust match compatibility consequence. These are explicit design
assumptions for approval: the new authored Python dimension policy does not
change generic native/OXML geometry admission, unchanged imported unsupported
values are no-op, clear style is allowed, and XML scope is exactly the four
schema frontiers plus existing checker constraints. The additive diagnostic
enum arm is an intentional published Rust source-compatibility change for
exhaustive matches and is explicitly approved for this design. If a complete provenance implementation or unexpected
native API need exceeds the L estimate, root splits the story without dropping
any stated acceptance criterion.
