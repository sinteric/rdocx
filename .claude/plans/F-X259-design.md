# F-X259, Repair reviewed consistency defects and strengthen acceptance gates

**Status**: completed
**Sprint**: S95.3
**Size**: M
**Depends on**: F-283, F-284, F-X254

## Problem

The independent evaluation at `.claude/reviews/S95.11-S95.2-code-quality-evaluation.md` reproduces STYLEREF evaluator/render disagreement, grammar1 empty implicit-loop failure and contradictory completed capability owners. It also identifies an overly dense XML traversal predicate. The current source is 67423b6b on sprint/s95.3. All prerequisites are done. The user explicitly requests the fixes, current contribution intake and stronger shared model instructions, while reserving remaining sprint implementation for Grok and final sprint review for Codex.

## Spec reference

- `docs/hld/08-rendering-spec.md`, Numbering-aware heading placement, defines unnumbered STYLEREF r and the distinct REF context.
- `docs/hld/03-architecture.md`, structured template evaluation, defines lexical lookup and one staged publication.
- `docs/hld/02-scope-and-non-goals.md`, Modern DOCX capability matrix, defines capability status and owner semantics.
- `docs/hld/12-testing-strategy.md`, Numbering-aware navigation regression and scalar/structural template controls, defines local regression observations.
- `.claude/WORKFLOW.md`, The design plan is a machine-consumed contract and Git workflow, defines canonical instruction sources and completion evidence.

## Approach

Put the missing STYLEREF default caller policy in existing ResolvedNumbering::numbered_reference_text. Both facade and renderer already call it. Remove the facade-only fallback. REF with absent caller retains full context.

Make grammar1 lexical resolution return the existing ResolvedValue::Deferred when an empty-loop implicit item cannot be inspected, retaining the legacy distinction between unknown item properties and known invalid outer/root containers. Use the same resolver for structural paths and scalar leaves. Keep the existing expression parser and filters. Do not introduce another scope engine or public type.

After product regressions pass, reconcile the completed bounded DOCX-050 and DOCX-051 capabilities with evidence and no active owner. Extend metadata adoption to validate the final capability-owner invariant rather than trusting a pre-completion policy result. Add a real temporary-history regression for the adoption failure.

Name local traversal cases in the existing rpptx frontier scanner without changing its predicates. Strengthen existing commands and reference skills. Add prepare-only planning and requested independent reviewer handoff semantics to run-sprint. Regenerate adapters. Keep process and behavior changes independently reviewable, with separate commits if later committed. No commit, push or full sprint completion is requested by this preparation.

The user's follow-up extends this instruction work to detailed design validation. Require pinned factual compatibility evidence, exact applicable API/numeric/schema contracts, complete decision aggregation and authentic oracle provenance. Separate provider-contract approval from actual dependency readiness and correct draft mode to always write its plan. Preserve Grok's current draft plans and decisions for explicit revision and approval.

## Consumer map

| Rule | Consumers | Shared authority | Required agreement |
|---|---|---|---|
| STYLEREF number context | facade evaluation/cache update and deterministic layout | ResolvedNumbering::numbered_reference_text | r/t/n/w with unnumbered and numbered callers |
| Deferred template lookup | scalar leaves, nested loops and if paths in both grammars | existing lexical scopes and ResolvedValue | empty item lookup defers, known invalid root paths still refuse |
| Completion ownership | capability matrix, backlog and metadata adoption | current tracked files | incomplete rows have live owners, complete rows have none |
| Workflow instructions | all implementing models and generated adapters | canonical commands/skills | same requirements and honest reviewer identity |

## Acceptance criteria

| ID | Observable criterion | Evidence |
|---|---|---|
| A1 | Multilevel r and r+t evaluate and render Clause1 and1, before/after update and reopen | new unit regression using actual field_source positioned text, n/w controls |
| A2 | Empty, single and multi-record implicit/explicit loops work under both grammars | new unit regression with exact counts/output and nested structural paths |
| A3 | Missing known root members, wrong intermediate kinds and malformed syntax still refuse atomically | companion negative controls with before/after bytes |
| A4 | Final capability records agree with completed owners | existing matrix tests and new metadata adoption regression |
| A5 | XML acceptance remains identical after naming traversal cases | existing seven fx254 frontier tests |
| A6 | Cached projections name fact-changing publishers even with unchanged owner scopes or revisions, prove read/write physical-owner equivalence and native/held/fresh/reopened agreement, and stop dependent tests after failed builds. Canonical instructions require measurable design, applicable API/numeric/schema contracts, pinned compatibility facts, complete decisions, authentic oracle evidence and discriminating cross-consumer/final-state checks | reviewed command/skill diff, prose and generated-adapter checks, dependency/draft-mode consistency inspection |
| A7 | Current sprint records every new issue/PR criterion and Grok/Codex handoff | CURRENT_SPRINT, HLD14 and draft contribution plans |
| A8 | With TOC heading range 1 through N, outline-derived headings and directly outlined paragraphs outside that range are excluded. Native, Python and CLI rebuild agree. Outline-only instructions retain all admitted levels, body-text suppression and explicit custom-style selectors keep their existing semantics | Issue400 and PR401 pinned below. Actual local RED/GREEN plus binding/CLI observations, update/reopen controls and final unchanged hash harness |

## Behavior contracts and evidence

| Rule | Exact contract | Source and evidence |
|---|---|---|
| Existing consistency repairs | A1 through A7 retain their approved outputs, atomicity, consumer map and local regressions | Original evaluation and existing scoped records, with final integrated acceptance pending |
| TOC outline range | Filter outline-derived levels by both endpoints only when heading_levels is Some. None retains levels1through9. Custom styles are selected independently. No new API, unit conversion or namespace policy | Current field.rs toc_paragraph_level, pinned Issue400/PR401. Contributor Word claims remain attributed |

## Test matrix

| Case | Input and expected observation | Wrong implementation rejected |
|---|---|---|
| Existing rules | A1 multilevel numbering, A2/A3 empty versus known invalid scope, A4 completed-owner metadata, A5 finite XML admission | Passing a convenient same-consumer or nonempty case only |
| TOC range composition | Inherited/direct levels1and2 with ranges1through1,1through2,2through2 and outline-only, before/after reopen | Upper-bound-only filter, heading-style-only filter and unconditional range default |
| Independent selectors | Explicit custom-style mapping and body outline9 suppression retain existing outputs | Blanket range filtering before custom-style selection or lost body-text suppression |
| Actual consumers | Native, installed Python and CLI rebuild yield the same expected entry text/count at the integrated source | Correct helper with an untested binding or stale executable |

## Rejected alternatives

- Copying the renderer fallback repeats the policy that already diverged.
- Skipping empty-loop validation hides malformed templates and known root errors.
- Marking every capability complete solely to satisfy a test invents acceptance. Reconcile the actual bounded contracts after their defects are fixed.
- Repeating workspace/native gates per small edit adds cost without proving missing combinations.
- A new general agent framework, scope registry or generic policy engine is unnecessary.

## Test plan

**Test gate**: regression.

| Timing | Category | Test | Asserts |
|---|---|---|---|
| local scoped | regression | reviewed_styleref_relative_fields_render_the_evaluated_number | actual old RED and repaired GREEN across evaluation/layout/update/reopen |
| local scoped | regression | reviewed_template_empty_implicit_scopes_match_populated_scopes | empty/single/multi and both grammars, nested paths, source atomicity |
| local scoped | regression | metadata adoption temporary-history test | completed incomplete-owner status refuses actual metadata adoption |
| local scoped | regression | existing matrix and fx254 tests | status consistency and unchanged finite XML admission |
| local scoped | unit | existing template and numbering-resolver cases | parser/filter/ref context and refusal controls |
| final sprint | integration | verify full and approved final rider union | actual integrated workspace, installed binding, WASM, docs, package and deterministic49/pixels7 checks |

Local checks use scoped cargo check/clippy, fmt, named tests, prose and adapter checks. Preserve old-source failures in the existing progress record. External/native claims remain pending for the final union.

## HLD impact

- `docs/hld/02-scope-and-non-goals.md`
- `docs/hld/03-architecture.md`
- `docs/hld/08-rendering-spec.md`
- `docs/hld/12-testing-strategy.md`
- `docs/hld/14-development-backlog.md`

## Risk routing

Layout/output: HLD08, deterministic scoped positioned-text comparisons and final49hash/sevenpixel gate. Parser/serialization: HLD04 preservation and HLD06 sequence conventions, existing grammar/refusal tests. Process/state: metadata adoption temporary-history controls. Published provider behavior changes without a new public API, final affected package/archive and binding/WASM applicability remains in the sprint union. No new dependency, module, trait or feature flag. User explicitly authorizes normal plan/review artifacts and shared instruction updates.

## Hash harness

Expected unchanged for all49documents and sevenpixel cases. These inputs do not use the repaired opt-in multilevel STYLEREF or template combination. Unexpected deltas block final acceptance and cannot be absorbed into the refactor.

## Implementation checklist

- [x] Preserve old-source failing regressions.
- [x] Repair the shared numbering and deferred scope policies.
- [x] Reconcile capability metadata and guard adoption.
- [x] Simplify XML traversal without behavior changes.
- [x] Update shared instructions, current intake and draft contribution plans.
- [x] Execute the HLD impact list and scoped checks.
- [x] Obtain clean independent feature reviews and accept provider-specific integrated evidence. The eight-story final sprint gate remains due.

## Open questions

None for the bounded reviewed fixes and instruction updates. Contribution API choices are recorded in the separate approved F-X260 through F-X262 plans. The user already requests Grok implementation and independent Codex sprint review. No remaining sprint authoring or contribution implementation starts in this preparation.

## Sprint-final acceptance

**Checkpoint**: sprint-final-acceptance
**Verify**: local scoped checks consumed from `.claude/scratch/F-X259-progress.md`, pass. Those checks are fmt, clippy for `rdocx`, `rdocx-layout` and `rpptx`, the named `reviewed_` regressions, template and numbering units, seven fx254 frontier tests, prose and adapter drift. They were not rerun after `527a3ca5` because these sources did not change. `/verify --full` is pending.
**Test gate**: `reviewed_styleref_relative_fields_render_the_evaluated_number`, `reviewed_template_empty_implicit_scopes_match_populated_scopes`, `reviewed_template_empty_scopes_still_refuse_known_invalid_roots`, and the metadata-adoption owner refusal, pass
**Final checks**: 49-hash, seven-pixel, installed products, native Word comparison, WASM, docs, archive and the sprint risk union, pending
**Hash harness**: deferred to the final sprint gate, expected unchanged

The final checklist item stays open. This marker is not feature completion.

## Approved intake extension, 2026-10-22

The user requests new GitHub work in this sprint and Codex continuation. A8
owns Issue400 body SHA-256
`656762fcfaf40919000730cb39099c6d3c0da427cacba45360a9cfcd4fa0836f` and
candidate PR401 head `62124737c428a77e0c4fec52b56665c67f221e7c`.
Its base is `b072b9852d5b17c7bcd19889cef058556ce9b67d`. The candidate
changes the existing `toc_paragraph_level` outline branch and the existing
regression entrypoint. Its contributor-reported Word observation is attributed
to the reporter, not a native oracle execution by this workspace.

Consumer map: native `Document::rebuild_toc`, Python `Document.rebuild_toc`
and CLI `toc rebuild` share the existing field selector. Filter only the
outline fallback by an explicit parsed heading range. No new API or selector
engine. An absent range leaves outline-only behavior intact. Custom style
selection stays independent. Do not silently constrain its assigned TOC levels.

Discriminating matrix: inherited Heading1/Heading2 and direct outline0/outline1
under heading ranges 1 through1 and 1 through2, with and without outline switch,
with h/z switches, and outline-only. Compare exact selected entry texts and
counts before/after save and reopen. Add a lower-bound range to reject a
maximum-only predicate. Retain the existing body-outline9 suppression and
custom-style selector regression. Exercise the actual Python and CLI surfaces
at final integrated qualification, not merely native tests. Expected source
change is confined to `field.rs`, existing tests and listed HLD sections.

HLD08 describes current range/outline composition. HLD12 records these local
discriminators. HLD14 and CURRENT_SPRINT retain intake and pending evidence.
Published behavior changes only for incorrectly admitted TOC entries. No new
public signature, dependency, module or file. Native Word provenance remains
pending where already required. A8 is not qualified by the contributor's tests.
No material product choice is open for this bounded correction. The added
shared files are leased serially after F-X260 source freeze. Final full/hash
and affected metadata evidence must bind the actual integrated changes.


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
