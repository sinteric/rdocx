# F-X256, Refuse ambiguous Word numeric units and preserve line spacing

**Status**: completed
**Sprint**: S95
**Size**: M
**Depends on**: F-X239

## Problem

The four numeric rows of Issue386 still admit ambiguous values. The setters
at crates/rdocx-py/src/formatting.rs:532 and :1330 silently truncate tiny bare
EMU to zero. ParagraphFormat.line_spacing at formatting.rs:1403 treats integer
2 as EMU rather than the two-line multiple used by python-docx1.2.0. Public
Document.update_section at document.rs:7878 accepts bare1440 as EMU, storing
two twips rather than the caller's apparent1440twip margin.

## Spec reference

- docs/hld/14-development-backlog.md, F-X256. Test gate: regression.
- docs/hld/01-glossary.md, Units, and CLAUDE.md, Things that are deliberately wrong.
- docs/hld/10-bindings-spec.md, Python API shape, Python Word formatting and checked list publication, section operations, and Python Word document operations.
- docs/hld/12-testing-strategy.md, Binding tests, Notes owners and unit refusal qualification, and Integrated Python compatibility qualification.
- .claude/skills/differential-testing.md, pinned object-model oracle and independent package observations.

## Approach

Change only the reported numeric routes. For ParagraphFormat.space_after and
Font.character_spacing, distinguish genuine rdocx.shared.Length instances from
bare integers before extraction. Exclude bool. Keep meaningful existing bare
integer EMU inputs, explicit zero and None. Refuse nonzero bare integers that
truncate to zero twips with Pt/Twips/Emu guidance. Genuine Length instances
retain explicit signed truncation. Character spacing remains signed. Paragraph
space_after retains its valid nonnegative domain. Check complete twip bounds
before narrowing, without changing shared constructors or native conversion.

For ParagraphFormat.line_spacing, test genuine Length first for absolute
spacing. Then treat int or float, excluding bool, as line multiples. Thus
2 and2.0 write line480/auto, while Pt(18) writes line360/exact. None clears.
For absolute Length, divide the full integer EMU by635 with truncation
toward zero, then check the signed32 quotient before narrowing. For multiples,
require a finite product with240 and a truncating signed32 quotient before
narrowing. Test exact limits and adjacent quanta. Retain zero and supported finite signed behavior. Preserve native
truncation for multiples off the240 grid and explicitly record its divergence
from the pinned oracle's rounding. Reject unsupported types and saturation.

Public Document.update_section requires genuine Length for every nonzero
geometry argument: page_width, page_height, margin_top, margin_right,
margin_bottom, margin_left, gutter, column_spacing, header_distance and
footer_distance. Bare zero remains intentional zero subject to native domain
rules. None or omitted remains unchanged. Refuse nonzero bare integers with
Twips(1440) or Emu(value) guidance. This is an explicit new Python refusal
contract, rather than a heuristic for one reported literal. Keep existing
native domain, partner and orientation validation and stage all arguments
before publication.

Do not change checked_section_length globally. Picture, watermark, chart,
wrap/offset and live section routes share it. Live Section setters retain
their raw-EMU contract. Their existing internal delegation at document.rs:1268
must wrap already validated EMU with the existing length_object helper before
calling public update_section, without exposing a public bypass or reclassifying
user bare input at the public boundary.

Update existing section truncation positives at test_core.py:4814 to explicit
Emu(-635), Emu(-1) and Emu(634), preserving their exact expected results and
adding independent bare-input refusals. Match runtime and stubs, including
Length or Literal[0] for public update_section geometry and existing integer
acceptance on the meaningful bare-EMU routes. No new product API, file, module,
dependency or native provider. Style line_spacing, space_before, indents and
other integer setters remain outside this story.

### Complete installed-suite fixture reconciliation

Actual complete installed Word execution found seven stale compatibility cases
outside the new focused controls. Two numeric fixtures must use explicit Emu
for nonzero update_section values, retaining their exact saved geometry and
native invalid-gutter assertion. Five other cases encode contracts superseded
by completed F-X249 and F-X251: one missing individual SDT selector, two
invalid checkbox scalar variants and two duplicate tab snapshot variants.
Reconcile those existing test_core.py and test_formatting_tables.py controls
against the completed plans and current HLD10. Missing individual selection
must assert KeyError, invalid individual checkbox glyph must assert ValueError,
and repeated stored tab positions must assert atomic ValueError. Keep bulk
and native error/count behavior distinct. Preserve snapshot immutability,
held owners, saved values and every original no-op/refusal assertion. Valid
insert/delete/clear checks may use distinct positions with adjusted exact
position expectations. Do not change production SDT or tab behavior, widen
exception catches, skip tests or rewrite historical completion evidence.

These are test-only reconciliations needed by the already declared complete
installed suite, not additional product behavior. Actual failed first-attempt
logs remain retained. Existing F195 reference inputs must be equipped and
source/tool authenticated with their original producer attribution, rather
than treating missing prerequisite environment as a product failure.

## Rejected alternatives

- A section zero-quantization guard misses1440EMU because it stores two twips.
- Guessing points or twips from integer magnitude introduces ambiguous units.
- Tightening the shared section helper silently changes unrelated geometry APIs.
- Globally changing truncation would change pinned native layout behavior.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| regression | issue386_numeric_rows_are_observed_independently | Actual old-source RED, exact original four labels, actionable refusals or correct multiple, independent saved XML |
| regression | spacing_lengths_refuse_quantized_bare_values_atomically | Tiny signed bare values, bool, wrong types, huge values, zero/None, signed character spacing and explicit Length boundaries |
| regression | line_spacing_distinguishes_multiples_and_lengths | Integer/float multiple, absolute Length, rule/value reopen, zero/None, signed finite controls and nonfinite/scaled overflow refusal |
| regression | line_multiple_preserves_named_native_truncation | Off-grid native truncation stays explicit rather than falsely claiming exact oracle rounding parity |
| regression | update_section_requires_explicit_nonzero_lengths | All ten keywords,1440and other bare values, explicit Twips versus Emu, existing domain/partner validation and multi-key atomicity |
| regression | live_section_keeps_its_existing_emu_contract | Existing raw-EMU live setters and typed public update agree after save/reopen, held owners survive refusal |
| regression | existing_integrated_sdt_and_tab_contracts_remain_deciding | Exact F-X249 individual errors, F-X251 duplicate refusal, retained snapshot/owner/no-op controls and positive providers |
| differential | numeric_units_match_pinned_python_docx_and_named_refusals | python-docx1.2.0 structural controls, policy refusals and rounding divergences are distinguished |
| installed | fresh ordinary Word suite, strict typing and stubtest | Real wheel/RECORD/native identity, exact updated annotations, current examples and complete installed tests |

The backlog test gate is regression. Extend existing modules and typing
consumers. Preserve immutable Issue386 body/script identities, separately from
Issue385's twelve-body union. The worker runs an explicitly labeled four-row adapter against its fresh
Word wheel. Root runs the untouched whole script once against paired final
Word/PowerPoint artifacts and reports actual rows, not exit zero. A missing
peer-package import is not numeric RED. Before/after package equality,
signed bytes, held paragraph/run/section owners and later valid edits qualify
every refusal. Independent ZIP/XML observers check stored values after reopen.

## HLD impact

- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md

## Risk routing

Unit conversion: read HLD01 Units and pinned truncation guidance, retain
constructors/native rounding and expect unchanged deterministic output.
PyO3/WASM: changed binding clippy, fresh ordinary installed Word suite and
strict typing/stub checks, both WASM checks, no binding Rust test binary.
External oracle: pin python-docx1.2.0 and authenticate actual source, versions
and saved outputs with declared refusal/rounding divergences. No native public
Rust API, parser, font, dependency or layout changes are planned. Binding-only
changes have no affected publishable Rust crate. If implementation requires
one, return to root to revise scope and add genuine affected publication and
archive checks. Root final integrated packaging and all scoped policy/hash
obligations remain mandatory.

## Hash harness

Expected unchanged49 hashes and seven deterministic pixel buffers. The
intentional Python integer-line correction changes newly authored output only
for the reported contract. Keep native rendering and existing baselines exact.
No baseline lease.

## Implementation checklist

- [x] Establish actual old-source numeric RED and preserve original input identities.
- [x] Implement only the bounded spacing and public section admission changes.
- [x] Preserve live section forwarding, explicit truncation and atomicity.
- [x] Align stubs and extend existing saved-file, oracle and typing controls.
- [x] Reconcile only the named stale installed fixtures against completed F-X249/F-X251 contracts, preserving all deciding assertions.
- [x] Pass scoped checks and obtain independent zero-finding microscope.
- [x] Update exactly the named HLD sections and prepare validated handoff.

## Open questions

None. The user approved the consolidated decisions, including the color
return migration and both requested enum modules. Numeric policies are the
finite choices above. F-X256 and F-X258 may run in parallel under separate
source, test, HLD and build leases. F-X257 follows the integrated F-X256 prefix.
