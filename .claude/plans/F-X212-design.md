# F-X212, Align native accessibility with visible paragraph content

**Status**: completed
**Sprint**: S91
**Size**: M
**Depends on**: F-X196

## Problem

Native Document::audit_accessibility at crates/rdocx/src/document.rs:29098
counts plain text only, so picture paragraphs contribute to consecutive
empty warnings. PR317 adds native model visibility and fixes this audit,
beyond the already implemented read-only F-X196 CLI contract. Its immutable
head is3d056f46e97e9cc1caf2969b5096eeb6775a5fa0.

Contributor rdocx-oxml/text.rs:2316 treats every unknown raw run child as
visible except a local-name list. That is insufficient for foreign lookalikes,
properties and opaque producer extensions. Its local-name filter at11924
cannot establish qualified Word identity. Reuse accepted typed traversal and
regression scaffolding after proving those boundaries, not the blanket raw
predicate or recorded baseline.

## Spec reference

- docs/hld/14-development-backlog.md, F-X212 and PR317 reconciliation.
- docs/hld/10-bindings-spec.md, CLI validation advisory findings and native
  Word facade stability.
- docs/hld/04-opc-and-packaging.md, preserved XML and package ownership.
- docs/hld/12-testing-strategy.md, Test taxonomy and deterministic hash gate.
- docs/hld/08-rendering-spec.md, deterministic font and output contracts.
- docs/hld/15-build-and-toolchain.md, Packaging and existing archive carriers.

## Approach

Complete F-X196 before claiming this source wave. Preserve its reviewed
namespace-qualified scan, typed control admission, first expanded content
slot and recursive facade inventory. Use a private current-source-aware audit helper in existing
rdocx/src/document.rs, retaining typed accepted semantics and exact raw
namespace provenance. No new
module, file, crate, generic parameter, trait or dependency is needed.

Typed accepted-view runs and equations are primary. Reuse concrete content
and existing accepted traversal. Text needs a non-whitespace display value.
Deleted text and comment references do not count. A soft hyphen alone does
not prove visible content. Ordinary drawings, fields with display semantics,
symbols, break/tab/special characters and note references do count.

For raw pict/object/AlternateContent and preserved Word wrappers, prove the
actual namespace and accepted physical path. Preserve inherited bindings,
alias/default/rebound cases and native content-control slot admission.
Unknown XML is retained, not assumed visible. Do not turn local-name matching
into a second namespace parser. Use the resolved minimal existing-helper contract below, with model prefix
provenance verified by its mandatory source-built controls.

The existing CT_R/CT_P raw storage loses inherited namespace scope. Two
otherwise identical raw fragments under Word and foreign aliases cannot be
qualified by a no-argument model predicate. Do not add public model fields,
a struct-literal break or an unused ParagraphRef projection.

Use the existing one unpublished clone_for_staging at document.rs:13031,
then flush_document_to_package at15177, then get_part(doc_part_name). The
flush uses exact original main source when it parses equal to the current
model. After edits it serializes current model once and replays retained
nested bindings, or explicitly refuses unsafe/ambiguous shadows. Never use
stale original package bytes or independently serialize each paragraph.
Do not call save, to_bytes or full prepare merely to audit. Existing nested
namespace replay has candidate matching costs, so only the added scan is
one pass, not a claim that complete preparation is globally linear.

Extract the existing qualified CLI scanner into existing native document.rs
under an actual native facade operation consumed by CLI and audit. Keep
facade recursive paragraph inventory and direct-body audit inventory as
two explicit concrete call modes of one scanner. The direct mode emits only
direct BodyContent::Paragraph facts, with each nonparagraph resetting the
audit streak. Scanner mode is private and has two actual consumers, with
no new generic parser, module or future-use switch. Verify the emitted fact
count matches each actual native inventory before consumption.

Native visibility source construction returns Result. The infallible audit
retains metadata/image/heading checks on failure, adds one Warning that
paragraph visibility could not be determined with the actual error detail,
and skips consecutive-empty findings. Never convert failure to all-false,
plain-text-only or a stale-source fallback. Add after-edit and ambiguous
namespace failure controls proving unchanged document/package state.

Separate extraction from behavior. First make a labelled code move retaining
F-X196 CLI behavior and unchanged49 hashes. Then make the intentional audit
and any field-display refinement in its own labelled commit, declaring the
expected three-key delta. Independent review covers both final paths.

Native and CLI public controls share the core semantics. Where PR317 differs
from approved F-X196, such as loose field markers, establish the display
behavior with a source-built discriminator and review an explicit small CLI
refinement under this owner. Do not replace the complete scanner with a
predicate that loses opaque/raw context. F-X203 consumes the integrated
validation contract without a duplicate audit implementation.

## Rejected alternatives

- Copy the contributor baseline without deriving accepted-source deltas.
- Reopen F-X196 and repeat its completed scanner implementation.
- Treat unknown non-whitespace XML as visible content.
- Add an unused facade forwarding method or a second generic parser.

## Test plan

**Test gate**: regression.

- Add the contributor actual-picture audit regression to the existing native
  entrypoint and run against unchanged claimed Base before production edits.
- Qualified aliases/default namespace count actual pict/object content.
  Foreign/rebound same-local-name raw content remains excluded and exact.
- Duplicate SDT slots, invalid opaque controls, deleted/moveFrom content,
  loose field/instruction markers, producer hints and soft hyphens do not
  hide true empty paragraphs. Accepted wrappers and equations do count.
- Actual pictures followed by true empty paragraphs produce correct distinct
  consecutive-empty findings, preserving every other audit finding.
- Source serializes identically before/after native audit and CLI validation.
- Run shared native/CLI discriminators and applicable changed-crate checks,
  tests, prose/drift/repository policy and verified packaging riders.
- Derive all49 hash keys locally. Revert only the audit behavior in a bounded
  proof to recover old hashes, then restore accepted source. Never record
  against system fonts or permit unrelated deltas.

## HLD impact

- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md
- docs/hld/14-development-backlog.md

State the actual native audit visibility and any reviewed CLI refinement.
Describe deterministic delta attribution, not historical change prose.

## Risk routing

- Parser/serializer: HLD04 and HLD06 conventions, qualified names, retained
  opaque source, alias/default/rebound and schema child-order controls.
- Visible behavior/hash: HLD08/HLD12, exclusive baseline owner, labelled
  intentional audit commit and independent expected-key review.
- Additive published native API: HLD10/HLD15, actual affected-package
  patched dry runs, membership/size checks and actual archive measurements.
- No binding API, new asset, dependency, version or structural file change.

## Hash harness

Expected intentional delta is feature_showcase:word/document.xml,
feature_showcase:pdf/pages and feature_showcase:pdf/bytes because its generated
report loses the false consecutive-empty finding. All46 other entries must
match. Confirm this exact set from accepted source before any baseline write.
Contributor hashes are evidence to investigate, not expected values to copy.
Review the generated finding count and reverse-control proof independently.

## Implementation checklist

- [x] Complete F-X196 and approve the qualified native predicate design.
- [x] Claim this story and observe real failing public audit regression.
- [x] Reuse/adapt native predicate and audit wiring in existing files.
- [x] Prove raw/typed admission and unchanged source controls.
- [x] Review any explicit CLI semantic refinement before implementation.
- [x] Derive and independently review exact three-key audit delta.
- [x] Commit intentional behavior separately with reviewed expected delta.
- [x] Run scoped/risk gates and clean independent microscope.
- [x] Update listed HLD sections and validated worker handoff.

## Open questions

No unresolved design question remains. F-X196 is completed and integrated
at852f8387 with scoped dependency-prefix evidence. The exact shared native
operation, two private inventory modes and X203 error projection below are
approved. Mandatory field-display controls remain implementation gates and
must establish real Base failures before semantic edits. Claiming still waits
for global sprint design approval and conflict-free source/baseline leases.
Approval does not claim that any regression, hash delta or gate has passed.

## Resolved shared operation and inventory contract


Select this additive native operation in existing crates/rdocx/src/document.rs:

```rust
impl Document {
    pub fn paragraph_visibility(&self) -> Result<Vec<bool>>;
}

enum ParagraphVisibilityScope {
    FacadeRecursive,
    DirectBody,
}

fn paragraph_visibility_from_main_source(
    document: &Document,
    xml: &[u8],
    scope: ParagraphVisibilityScope,
) -> Result<Vec<bool>>;
```

These are approved signatures. The public method performs the actual unpublished clone, current-model source flush, resolved main-part lookup and checked classification. It is not an unused ParagraphRef predicate or a forwarding-only public wrapper. It returns final visibility, including accepted text and nontext, not just the contributor's raw-child heuristic. The private concrete mode has two actual callers: the CLI-facing native operation uses FacadeRecursive, and audit_accessibility uses DirectBody. No new public mode, record type, CT_P/CT_R field, module, trait, generic or dependency is required.

Public preparation is exactly one `clone_for_staging`, one `flush_document_to_package`, then `candidate.package.get_part(&candidate.doc_part_name)` or explicit missing-main-part Error. Call the scanner with the candidate model, not with an independently reparsed body or stale package. Native audit performs the same bounded preparation inline in its existing method, then calls the private scanner in DirectBody mode. Three preparation statements in these two consumer paths are preferable to another forwarding source wrapper. Both source and authoritative typed inventory belong to the unpublished candidate. No save, to_bytes, reopen or repeated per-paragraph serialization is involved.

FacadeRecursive output is exactly one final visibility bool for each `Document::paragraphs()` item, in the same order. The actual current implementation at facade document.rs:18666 delegates to CT_Body::paragraphs. That iterator at rdocx-oxml/document.rs:2518 includes direct body paragraphs and modeled block-control paragraphs, including tables owned by those admitted controls. It skips ordinary direct body tables. Do not broaden the operation to every paragraph in the OPC package. The CLI's current independent scan is commands.rs:2365 and its model-length check is :2330. Preserve expanded typed SDT admission and the first expanded content slot at :2392 and :2427, including opaque controls and duplicate slots.

DirectBody output has one bool for each direct `BodyContent::Paragraph` only, in document.body.content order. It contains no table/control descendants and no sentinel rows. Audit already walks that actual body vector at document.rs:29094. Consume the next bool only for a Paragraph, and reset the streak for every other body item exactly as today. Validate complete count and typed/source ownership correspondence inside the helper before returning. After helper success, there cannot be a partial iterator or invented default false for a missing paragraph. Avoid introducing public Vec<Option<bool>> merely to encode resets the existing body inventory already supplies.

The scanner retains the NsReader source stack and admitted physical routes. Scope selection changes only which outer paragraph owners are admitted and emitted. Once an admitted paragraph begins, both modes use the same qualified child rules, typed control admission, accepted revisions and visibility computation. DirectBody mode rejects entering an outer block control or body table, while an inline control within its paragraph still receives qualified typed admission. Same count alone is insufficient where imported opaque owners might shift correspondence. Match emitted owner order/path and admitted typed slots against the selected model inventory, and return Error for ambiguity rather than guessing based on similar text.

The extracted scanner's Vec<bool> remains a private implementation result until inventory and typed correspondence are proved. Field/raw/typed classification is performed in that same core operation. Do not export raw xml offsets or introduce a second facade projection type. No field-display cache survives a call, so unsaved edits cannot leave a stale visibility value.

## Errors and X203 consumption

Source flush, missing source, XML parse, unexpected EOF, typed/raw admission mismatch or paragraph inventory mismatch returns native Error. Existing unexpected-EOF scanner behavior is commands.rs:2530. Modified namespace shadow refusal is native document.rs:15203. Never return `Ok(vec![false; count])` for one of these errors.

CLI consumer replaces its current `paragraph.text().trim().is_empty() && !nontext` pairing at commands.rs:2335 with false visibility counting. Preserve its recursive inventory and report ordering. If visibility errors and there are no independently collected structural errors, retain a Result error rather than claiming a complete validation report. If structural errors are already collected, retain those findings, add a Warning describing indeterminate paragraph visibility and omit the empty-count advisory. Explicitly remove the current all-false fallback at commands.rs:2326 under this story's reviewed refinement, since it fabricates empty warnings when scanning fails.

Native infallible audit keeps Vec<AccessibilityIssue>. On any helper failure, emit exactly one IssueSeverity::Warning with `Paragraph visibility could not be determined: {error}`, retain title/author/image-alt/heading checks and skip the entire consecutive-empty check. Existing audit content resets and warning severity remain distinct from CLI aggregate empty-count advisories. The helper does not evaluate fields, alter settings, populate live layout caches or mutate source/revision/identifiers.

X203's shared ValidationSeverity/ValidationFinding/ValidationReport is report assembly, not an alternative visibility representation. X203 consumes the public Result<Vec<bool>> in native validation assembly and projects its existing errors/warnings policy. It does not call audit_accessibility to reconstruct CLI semantics, enter direct-body mode, or depend on the CLI crate. Its existing OPC-open Result error boundary and conditional Document-open diagnostic remain separate from visibility indeterminacy. The current CLI returns a Document-open error finding only when other errors already exist at commands.rs:2290, otherwise it returns Err. Preserve that exact condition when reconciling X203, rather than an unconditional Document-open finding claim.

## Extraction and behavior boundaries

First extract the existing F-X196 qualified scanner and wire the shared native operation while retaining the current CLI text-plus-nontext decision exactly. This mechanically equivalent stage may temporarily preserve the existing fldChar/fldSimple affirmative list and accepted text projection. Derive all49 unchanged hashes locally and label it as the code move. Do not silently fix field display, standalone deleted-text projection or scanner-error advisory semantics inside that move.

Then add final typed accepted visibility, direct-body audit use and the explicitly reviewed CLI refinements in the separate intentional behavior stage. The public final signature stays Result<Vec<bool>> throughout. A code-move implementation may compute the old p.text().trim plus extracted nontext flags before the final behavior changes, but its final contract is the shared accepted visibility specified here. Native audit is its first new consumer after the labelled move, while CLI is the existing consumer. Independently review every final semantic difference and actual hash attribution before baseline movement.

## Required field-display discriminators

Current CT_R::content_text at rdocx-oxml/text.rs:2318 uses Field::projected_text, which returns a cache only for complex fields at :1108. Parsed simple fields can contribute ordinary result runs instead. Therefore `projected_text().unwrap_or("")` alone is not a general field visibility operation. `Field.cached_result` is the actual public effective cache at :408, and cached_result_runs at :651 supplies typed result runs only when their display string agrees with that effective cache. The latter deliberately rejects stale typed cache and preserves ordered result content. Do not assume every instruction names an already displayed or evaluated value.

Required source-built controls before changing the F-X196 list at commands.rs:2457:

- A loose qualified fldChar begin/separate/end with no complete field result, and loose instrText or delInstrText, remains empty. Foreign same-local-name markers remain opaque. Repeat in aliases/default bindings and accepted/rejected wrappers.
- Complete simple and complex fields with non-whitespace displayed cached text count visible. An instruction-only PAGE or NUMPAGES field with no effective result does not become visible merely because evaluation could produce text later. This read-only operation does not call update_fields.
- Whitespace-only cache remains empty unless a typed accepted result run contains an independently visible nontext item. Empty simple fldSimple is not affirmative by its element name alone.
- A complex field with a drawing, symbol, tab or break in its accepted result must count even when its text cache is empty. Follow cached_result_runs recursively for that actual typed display, excluding instruction and deleted content. Establish the equivalent parsed simple-field result behavior rather than counting its root unconditionally.
- A stale typed result after direct public cached_result mutation must follow the effective current cache and the consistency-filtered cached_result_runs contract, not the stale parse-time result. Non-whitespace changed cache is visible, changed whitespace/empty cache is empty unless the current result representation proves nontext display.
- A nested field result, a field spanning ordinary runs, an incomplete complex sequence and unmatched delimiters require actual accepted parser projections. Do not reconstruct field ownership from a local fldChar list or scan instruction XML as display text. Nested results terminate structurally by owned source, not arbitrary repeated marker recursion.
- A standalone DeletedText typed item can contribute to CT_R::text at :2314 today. Its exclusion from final accepted visibility is a separately reviewed CLI refinement, not an unnoticed consequence of the extraction. Insertion/moveTo accepted content and deletion/moveFrom rejection must agree across raw wrappers and typed projections.

Prove each discriminator through the existing native audit and public CLI validation entrypoints, before and after source edits, with exact no-mutation bytes and unchanged other findings. No discriminator, test or Cargo command has run during this design task. Only the later approved intentional audit behavior may earn the already proposed three feature_showcase hash deltas, and all46 other keys must remain unchanged unless a new measured and reviewed contract explicitly supersedes that expectation.

## Parallel source reservation, 2026-10-10

Use the existing inline native audit test module in
`crates/rdocx/src/document.rs` for the required public actual-picture regression
and native discriminators. These tests invoke Document::audit_accessibility
and Document::paragraph_visibility directly. The existing CLI carrier is
`crates/rdocx-cli/tests/integration.rs`. Do not edit the native integration or
regression entrypoints reserved by F-X194. This changes test placement only,
not the required public regression, Base failure or acceptance controls.

F-X196 is completed. F-X212 may begin disjoint source work in wave2 alongside
F-X194. Cargo remains leased to F-X194 until explicit handover. Shared package
measurement carriers and HLD10 edits are queued after F-X194. Root owns the
hash baseline. F-X212 baseline measurement and publication wait until the
F-X194 unchanged-hash gate is complete. F-X197 remains behind F-X194 because
its actual source paths overlap. No new source file or test binary is added.

## Parser-owned nontext field-result exposure

Base test preparation exposed an existing producer gap: parsed complex
result runs are populated only for note/comment candidates at OXML
text.rs:4440-4455 and:4981-5002. Simple result projection at:3225-3276
likewise gates typed caches to nested fields or note/comment references.
A valid Field::from_raw typed tab cache is a separate consistency-positive
control. The two failed raw-tab cache fixture preconditions are not credited
as semantic regression evidence and do not relax cached_result_runs filtering.

Reserve existing `crates/rdocx-oxml/src/text.rs` for the smallest necessary
parser-owned nontext result exposure. Reuse existing parser-owned result spans/child-field projection
and existing parsed/simple cached-run fields at admitted complete owner
boundaries. Private transient builder storage may retain accepted result runs
once without adding public model fields. Use parsed_complex_cache_runs only
where measured nested-cost and ownership controls prove the bounded path. Preserve ordinary text-only projection and avoid unnecessary
reparsing. Candidate raw hints may trigger qualified typed parsing, but never
prove visible content by local names. Retain effective-cache consistency
filtering, current public cache mutation semantics, accepted result order
and source ownership. No new API, model field or local field-delimiter
stack is selected. Do not independently reparse each document paragraph.

The existing complex cache helper's fabricated conventional w paragraph
at:10707-10721 must use an actual retained producer Word prefix as the
existing simple helper does at:3232-3254. Preserve foreign/rebound bindings,
default namespaces and locally declared child scopes. Unsupported owned
projection remains absent, while source ambiguity/error produces the declared
indeterminate result, not fabricated visibility.

Add focused parser-owned result controls in the existing text unit module
before these semantic edits. Prove meaningful Base failure separately from
the authoritative typed from_raw control. Cover simple/complex nontext, nested
and spanning fields, incomplete delimiters, namespaces, no-op save/reopen
and changed public cache consistency. Review PartialEq and parse-cost effects
explicitly. This adds the OXML changed-crate/parser and publication gates.
Land only in the labelled intentional behavior stage after unchanged scanner
extraction, preserving the exact forty-nine-key attribution contract. Any
additional output delta stops for diagnosis and reviewed evidence.

## Scoped workflow policy prerequisite

The scoped workflow-policy gate uses the coordinator's existing roadmap
validator for both declared table forms, including S91's delivery-status
column. Mirror canonical7a7851e3's bounded existing-function repair in
scripts/test_sprint_workflow.py. Preserve unique placement, title, size,
backlog and dependency assertions. This is a coordinator prerequisite for
the gate, not additional paragraph-visibility behavior. Its script and this
plan are absent from all22 authenticated publish archives, so retained
package verification is unchanged. The ignored worker target path points to
the existing shared Cargo target, with no second build directory.

## Scoped completion evidence

The labelled scanner extraction is ea803be1. Final scoped native, OXML and
CLI tests and lint pass through the retained exact retries. The deterministic
harness matches all49 updated entries, with only the three declared showcase
keys changed and46 unchanged. Independent source, hash, HLD, carrier and
policy-prerequisite reviews are clean. Verified22 packaging has original
Cargo0 and corrected capture0, with the failed capture wrapper retained.
The final policy command ran141 tests successfully with two existing
publication-dependent skips, followed by diff0. No sprint-wide or integrated
result is claimed by this worker completion.
