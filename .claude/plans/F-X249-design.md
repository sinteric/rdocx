# F-X249, Set typed content control values as Word does

**Status**: completed
**Sprint**: S95
**Size**: M
**Depends on**: F-X241

## Problem

Issue385's pinned content-control test at test_docx_everyday.py:186-201 requires placeholder removal, typed dates, declared dropdown selection and missing-selector errors. Native content_control.rs:74 currently passes non-checkbox strings through without those semantics. Individual Python setters at document.rs:5750 return zero when missing. Existing F-X241 assertions deliberately pin lowercase dropdown display and a stale fullDate, so their purpose must be retained while these deferred expectations change.

## Spec reference

- docs/hld/14-development-backlog.md, F-X249. Test gate: differential.
- docs/hld/04-opc-and-packaging.md, Checked Word operation source ownership.
- docs/hld/10-bindings-spec.md, Python Word document operations.
- docs/hld/12-testing-strategy.md, Binding tests and Integrated Python compatibility qualification.
- Official ECMA-376 fifth edition Part1 clauses 17.5.2.3, .7, .8, .15, .20, .22 and .40, plus 17.18.76, retained with authenticated XSD/PDF in the private date/list research. These define metadata, not an executed Word result.

## Approach

Keep string inputs and existing checkbox lexical/glyph/font behavior. Resolve every selected value and binding before publishing any match. On successful selected fills remove only the Word-namespace showingPlcHdr property, including a metadata-only change when display already matches. Preserve foreign lookalikes, unknown slots and child order.

Dropdown input matches exact declared value or display text, without trimming or case folding. A unique mapping writes declared display and bound value. Absent displayText falls back to a present value, while present empty attributes remain distinct. Refuse unknown or conflicting mappings before mutation and list legal displays. A missing value attribute has no schema default. Preserve it on import, but refuse a selected mapping that cannot be proved. Keep comboBox free text separate. Do not choose the first ambiguous item or infer an XML default from the Word Add API. Actual native selection confirmed declared Two maps to bound two. Word selected a missing-displayText item as a five-space placeholder with an empty bound scalar, contrary to the schema fallback. Deliberately retain the Part1 clause17.5.2.22 value fallback and assert this documented divergence rather than copy native data loss.

Admit Gregorian-valid YYYY-MM-DD input, years 0001 through 9999. The finite display masks are explicit yyyy-MM-dd and dd/MM/yyyy, with Gregorian calendar absent, val absent, or explicit gregorian. Actual Word16.114, build16.114.26101123, selected-control edits and disk reopen confirmed both masks for31October2026. Canonical selected date-only fullDate is YYYY-MM-DDT00:00:00Z, a deterministic API policy supported by that observation, not a promise to reproduce every Word timezone normalization. Preserve lid and unrelated metadata. A missing mask depends on language, so refuse it rather than silently assume ISO. Other calendars, masks and invalid metadata are valid unsupported input where applicable, not globally invalid XML.

For bound dates, absent mapping element, absent val or text maps to formatted display. Date maps to canonical XMLSchema date and dateTime to canonical fullDate. Unknown enums refuse. No binding means no invented external scalar. Actual Word-produced controls and saved/reopened XML qualify all three explicit modes and the different-display contrast. Preserve the normative no-translation text semantics for an absent mapping element and text default for an absent val under Part1 clause17.5.2.40. Word rewrote the absent element to dateTime on unchanged save and later produced a timezone-less bound timestamp. Record that observed divergence, preserving metadata absence in the product instead of importing the native rewrite. The absent-val branch is schema-qualified, without a claim of separately observed native equivalence. No global timezone conversion or unselected import normalization is introduced.

Extend existing CT_SdtPr with the minimum concrete methods needed across the OXML/facade boundary. Planned signatures are prepare_text_value(&self, value: &str) -> Result<(String, String, Option<String>)> and set_value_metadata(&mut self, full_date: Option<&str>) -> Result<()>. They return display, bound scalar and an optional date timestamp, then apply qualified placeholder/date metadata to a staged control. Reuse existing private type payload and scope carriers. Keep parser context distinct from emitted namespace additions so unselected controls do not gain incidental rewrites. No public struct fields, new crate, module, file or error variant.

Native prepare_control_value retains its existing checkbox and unsupported-kind guards and uses the concrete OXML normalization for text/date/list. Carry fullDate in the existing private prepared value, then apply metadata and display to the cloned document. Existing bound XML resolution and late-failure publication remain authoritative. Retain matched counts even for exact no-ops. Exact no-op means display, typed metadata and bound scalar all match.

Individual Python tag and alias selectors preflight inventory and raise KeyError when missing. Preserve native missing count zero. New typed normalization refusals use existing OxmlError::InvalidValue. Map that typed variant to ValueError only in the individual setter, keeping the global converter unchanged. This intentionally also classifies selected InvalidValue payload failures in that method, which needs negative controls. Other errors keep their established classes. Keep individual conservative publication and existing bulk FormValues/publication policies. Exact no-ops and refusals retain bytes and authority, while actual mutation follows the existing retirement rules.

Bulk bind retains ignored unknown keys, tag-first alias fallback and matched no-op counts. Strict fill retains missing/duplicate rejection and ordered counts. Their generic Python error mapping stays distinct, and CLI failures remain nonzero with no output. Native shared normalization gives consistent typed results without silently changing bulk selector contracts. Label these native typed and individual Python error changes in the behavior commit.

Native evidence is sealed in /private/tmp/S95-F-X249-native-final-authentication-1/receipt.json, SHA256 c28078a4b6a4ab976c17090374f8da25c6743721068dee731a053660bfde884a. The source, unchanged native save, edited native save, relationship-resolved bindings, producer identities and opaque-property losses are retained. Root performed actual repair-free open, Save As, selected-control edits, close and disk reopen. Word discarded the seed foreign property on unchanged save. This does not authorize product source loss. Native date-picker selection was not claimed: date text was selected and pasted inside each live control, and saved fullDate/bindings were independently observed.

## Rejected alternatives

- Formatting through the existing permissive field parser adds lossy date/time admission.
- A locale/IANA/date framework exceeds the finite issue.
- Matching error text or converting every failure to ValueError loses source/error authority.
- Replacing counts with changed-only counts or changing bulk missing policy breaks existing contracts.
- Global placeholder or imported metadata normalization violates source preservation.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| reporter | Original test_a_content_control_value_is_set_as_word_does | Unchanged body, exact placeholder/date/list/KeyError outcomes, original strict marker report kept separate |
| OXML | fx249_qualified_value_metadata_preserves_opaque_slots | Placeholder aliases and foreign lookalikes, fullDate write, order, missing/empty attributes and raw payload preservation |
| native | fx249_typed_date_and_dropdown_bind_atomically | Explicit masks, leap dates, mapping modes, unique value/display, late selected/bound XML failure and matched counts |
| installed Python | test_fx249_individual_errors_noops_and_bulk_contracts | Tag/alias KeyError, individual ValueError versus bulk XmlError, unchanged other failures, TypeError for non-string and held-owner refusal/no-op controls |
| CLI | fx249_fill_typed_values_match_python_after_reopen | Actual tag/mixed-value outputs, qualified display/fullDate/bound XML, unknown-item no-output and late-failure atomicity |
| differential | Independent Word-produced typed controls | Actual saved/reopened explicit-mode date metadata, non-ISO display, declared choices and placeholder removal, with explicit missing-mapping and value-only native/schema divergences |
| nonregression | Existing F-X241 helper, inverse observers, F-X240 signing/stream controls | Revise only stale typed expectations, retain legacy, checkbox, ordering, opaque, source, ownership and signature assertions |

Use existing OXML tests, native regression_test.rs, Python test_core.py and CLI integration.rs. Current lowercase two becomes Two and stale September fullDate becomes the assigned October value. Keep historical packages and proofs immutable. The shared helper and public artifact consumer require fresh successor outputs and independently sensitive typed XML observers. No new test binary. Pin reporter commit 91a57108 and actual installed/native producers.

## HLD impact

- docs/hld/04-opc-and-packaging.md
- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md

## Risk routing

Parser/serializer: read HLD04/HLD06, namespace-qualified source operations, sequence, bounded parsing and exact opaque round-trip. Public API: read HLD10/structural rules, explicitly label new native typed behavior, additive narrow OXML methods and individual Python error compatibility, genuine affected publication dry run and archive ceiling. Binding/WASM: actual installed suites, strict typing/stubtest and both WASM targets. External oracle: apply differential-testing, pin Word build, source/saved packages, python-docx and XML tools. Native UI evidence and mapping semantics are required, not substituted by a product-generated expected value.

## Hash harness

Expect all 49 shared hashes and seven deterministic pixels unchanged. Native typed form output intentionally changes only the declared selected-value controls and is attributed in its own behavior commit. No shared baseline lease.

## Implementation checklist

- [x] Preserve reporter and establish actual S94 baseline outcomes.
- [x] Acquire independent Word-produced date/list controls and resolve the finite date convention.
- [x] Normalize selected typed values before any publication.
- [x] Apply qualified placeholder/date metadata and bound scalar atomically.
- [x] Add individual Python KeyError/ValueError while preserving bulk/native contracts.
- [x] Revise named stale expectations and generate new successor public artifacts.
- [x] Qualify signing, no-op/refusal owners, differential and scoped riders, microscope and handoff.

## Open questions

Resolved through actual native capture and primary authority. Finite masks and deterministic selected date-only UTC-midnight policy are selected. Missing mapping and value-only display use normative semantics, with the observed Mac Word divergences retained and asserted. No material user decision remains.
