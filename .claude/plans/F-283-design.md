# F-283, Complete numbering-aware navigation fields

**Status**: completed
**Sprint**: S95.2
**Size**: L
**Depends on**: F-248, F-279 through F-282, F-X200

## Scheduling and readiness

F-283 remains unfinished and pending in S95.2. F-248, F-279, F-280, F-281,
F-282 and F-X200 are done in `docs/sprints/BACKLOG.md`. F-X192 is not a formal
dependency. Its pending bibliography formatter expansion cannot be presumed.
This approved revision reconciles the earlier approved additive API and navigation contract
against current integrated providers. Historical progress and native records
remain evidence of their original source and tool epochs, not implementation
or final acceptance of F-283.

## Problem

`field_needs_numbering_layout` recognizes numbered REF only at
`crates/rdocx/src/field.rs:13201`. Numbered STYLEREF still returns an explicit
unsupported diagnostic at `crates/rdocx/src/field.rs:14099`, and its last-source
search is whole-story rather than page-local at
`crates/rdocx/src/field.rs:14062`. REF targets already use uniquely paired
bookmark numbering, but relative caller context still joins a flattened main
paragraph ordinal at `crates/rdocx/src/field.rs:13853`.

Legacy `headings` intentionally excludes tables and detects heading styles at
`crates/rdocx/src/document.rs:36745`. Its `document_outline` tree inherits that
scope at `crates/rdocx/src/document.rs:36764`. Renderer heading metadata uses
style-name detection and title text at `crates/rdocx-layout/src/engine.rs:4052`
and `crates/rdocx-layout/src/engine.rs:8617`. The paginator records only direct
paragraph outline entries at `crates/rdocx-layout/src/paginator.rs:1009`.
Table paragraphs already have physical source IDs and use the same numbering
state at `crates/rdocx-layout/src/table.rs:1718`, but this alone does not publish
their heading navigation or placement.

`ResolvedNumbering` already owns marker, level, full context, literal-free
forms and concrete instance identity at
`crates/rdocx-layout/src/style_resolver.rs:37`. `WordLayoutResult` exposes
source paths, paragraph numbering and bookmark placement at
`crates/rdocx-layout/src/lib.rs:280`. F-X250's existing direct-body
`restart_numbering` at `crates/rdocx/src/document.rs:28814` is a mutation
provider, not a table/control heading discovery provider. Compose these
existing contracts without another counter or font resolver.

## Spec reference

- `docs/hld/02-scope-and-non-goals.md`, capability matrix DOCX-050.
- `docs/hld/03-architecture.md`, "What stays put", immutable field evaluation, sequence state, native TOC discovery and result-local Word source maps.
- `docs/hld/08-rendering-spec.md`, "Word bookmark field pagination", "Word revision views", and "Word section geometry and page numbering".
- `docs/hld/12-testing-strategy.md`, numbered REF, TOC and Word source-path regression contracts.
- `docs/hld/14-development-backlog.md`, "F-283, Complete numbering-aware navigation fields" and F-248.

## Approach

Consume the completed F-280 `SequenceOptions`, `CaptionOptions`,
`CaptionTarget` and `CrossReferenceOptions`, F-279 immutable physical bookmark,
page and section records, F-281 generated-result ownership, F-282 current
bibliography result boundaries, and F-X200 authoritative continuation rules.
No new formatter catalogue, numbering mutation API or Python/WASM navigation
surface is included. The existing approved additive Rust outline decision
persists. Do not ask for the same API approval again.

Use `Document::layout_deterministic(&self) ->
Result<Arc<rdocx_layout::WordLayoutResult>>` at
`crates/rdocx/src/document.rs:35428` for the new read-only outline. Existing
field update operations already obtain that same deterministic provider.
Use one immutable WordLayoutResult for numbering and placements within each stage. A structural result rebuild may require a later snapshot because generated paragraphs affect pagination. Never treat an old snapshot as final page truth. Match sources by WordSourcePath or result-local SourceNodeId, not another story's flattened ordinal. Do not increment a separate navigation counter. Bind caller and target through
the existing physical field registration and source paths. A `SourceNodeId` is
local to its producing snapshot. Never equate IDs from independently produced
sequence and layout snapshots just because their integers match. Reuse the
existing source registry and batch any required path join once per operation,
not a full source scan for each field. Keep that join in the existing provider
files, without a new registry abstraction.

The existing-file provider clarification adds
`WordLayoutResult::paragraph_navigation(&self) -> impl Iterator<Item =
(SourceNodeId, &WordSourcePath, Option<u32>, &str, Option<&str>,
Option<&ResolvedNumbering>, Option<usize>)>` as a doc-hidden facade hook. It reads private
facts captured by the production paragraph cascade, including unplaced
eligible paragraphs. The final optional value is the first actual physical page
from positioned source spans, absent for unplaced content. It adds no public
struct fields, counter or font resolver.
The companion doc-hidden
`paragraph_navigation_source(SourceNodeId) -> Option<(&WordSourcePath,
Option<u32>, &str, Option<&str>, Option<&ResolvedNumbering>, Option<usize>)>`
reads the same private facts by key. Evaluators batch style aliases, source joins,
field placements and per-style first/last page records once per operation.
Candidate selection uses ordered source/page indexes, with no per-field full
paragraph scan or duplicate navigation tree.
The existing paginator records tab-alignment metadata for physical field
sources. Only source-qualified STYLEREF value replacements use it through the
established width-change realignment. Unsupported fields retain their values
and positions. Rich multilingual runs preserve the existing absence of
alignment metadata, without adding public output fields.
Placement continues to use the same result's actual positioned source spans
and existing field placements. This is the same private-owned-facts boundary
as the established source/numbering hooks.

Generated-range ownership is supplied by the existing facade
`scan_dynamic_table_spans` and `TocRunPosition` once per current input. The
narrow doc-hidden bridge is
`WordSequenceSnapshot::set_navigation_exclusions(&mut self,
input: &LayoutInput, exclusions: Vec<(WordSourcePath, Vec<usize>)>) -> Result<()>`.
Indices identify accepted runs inside each exact physical paragraph. Private
snapshot fields retain the complete current input document model, its physical
main part identity and these exclusion facts. Registration validates unique
main-story paths and accepted-run bounds. Ordinary CITATION is not an outer
cache owner. The existing TOC/INDEX/TOA/BIBLIOGRAPHY scanner qualifies only its
result ranges and preserves genuine text before and after a same-paragraph
cache. Exclusion applies to navigation discovery and cached SEQ events. It does
not suppress visible list-counter advancement from a valid cache paragraph
numPr. That established numbering provider remains authoritative. No second
XML parser is introduced. The scanner's disjoint physical order drives one
monotonic paragraph window. Per accepted run, binary search of result-end
positions narrows that window to one candidate before reusing the existing
ownership predicate. Many owners in one paragraph must not reintroduce a full
window scan. Test-only comparison counts bound both dispersed and packed owners.
Suppressed numbering supplies the base zero value, then follows the same
STYLEREF position switch and cross-story refusal as other numbering values.

Full layout still regenerates sequence values and source IDs. Only exclusions
whose complete document and physical owner stamp match current input transfer
into that fresh snapshot. Counter events, values and unrelated old metadata
never transfer. Stale or malformed stamped facts return an error. Facts-only
inputs also regenerate a snapshot, and private facts participate in existing
snapshot/cache equality. The facade prepares this bridge from its actual input.
Detached lower layout callers without facade facts keep their established
projection and do not gain a claimed physical generated-range inference.
Focused stale-input, partial-paragraph and cached-SEQ controls must pass locally.

Preserve the previously approved additive compatibility choice. Existing
`headings` and `document_outline` signatures and text semantics remain unchanged.
Add and re-export beside the existing `OutlineNode`:

```rust
pub struct NumberedOutlineNode {
    pub level: u32,
    pub text: String,
    pub number: Option<String>,
    pub display_text: String,
    pub source: rdocx_layout::WordSourcePath,
    pub children: Vec<NumberedOutlineNode>,
}
impl Document {
    pub fn numbered_document_outline(&self) -> Result<Vec<NumberedOutlineNode>>;
}
```

The additive result includes accepted body, table-cell and valid nested-control headings in physical document order using effective outline levels and resolved numbering. It retains source identity and separates title text from displayed marker. A suppressed marker returns `number=None`. `text` is the accepted title without
a synthesized marker. `number` is the resolved visible marker text, and
`display_text` joins a nonempty marker and title with one readable space.
The marker field retains its exact punctuation, while tab/space suffix geometry
stays in layout and exact field-result formatting. New API construction propagates layout errors through Result rather than concealing them in an infallible legacy accessor.

Extend numbering-demand classification and STYLEREF evaluation for supported n, r, w and t combinations. Resolve style identity through the actual cascade. Nearest-source and page-local last-source selection retain distinct semantics. Page-dependent selection uses F-279 placement records, not whole-story reverse search. Unplaced or unsupported sources retain cache and ordered diagnostics. Pure evaluation does not invent layout-derived page scope.

REF reuses ResolvedNumbering::relative_to, level, full-context and literal-free values. Same-paragraph position and cross-story bookmark identity follow F-280. PAGEREF uses the same validated target and final placement as REF and captions. Repeated header/footer placements share physical source identity while retaining placement-specific page/STYLEREF values.

Heading eligibility uses effective outline levels, accepted revision projection and actual source paths. Direct body-text outline suppression overrides inherited heading style. numId=0 suppresses the marker without erasing an eligible heading. Deleted and moved-from paragraphs do not advance accepted counters. Respect accepted paragraph-mark joins. Hidden/suppressed numbering and heading suppression remain separate cases.

Share effective numbering and outline facts while preserving each consumer's
selection rules. The additive outline suppresses effective body-text outline
level 9. The captured TOC with both o and u excludes that source, while an
explicit custom-style selection and o without u include it. Page omission
for a selected level is separate from source exclusion. A numId=0 heading
has no outline marker, but numbered REF modes return the captured zero.
Do not replace these distinct outcomes with one missing-marker fallback.

Shared furniture STYLEREF selects by physical placement. In the captured
first page without a qualifying heading, both first and last modes search
forward to the first qualifying source. Later pages use their own first and
last sources. The initial saved shared-part caches reflect the last explicit update
placement and do not stand for every page's rendered value. Native reopen,
offline PDF export, save and close without F9 retained every baseline header
cache but changed all eight mutated header STYLEREF caches. The mutated
stored last-heading value became 2.4 while the PDF still rendered 2.1 on
page one, 2.4 on page two and 7.2 on page three. No intermediate snapshot
identifies which operation changed the caches. Do not infer a universal
placement policy from that lifecycle or require native Word to retain all
stored cache bytes. Ordinary library save/reopen retains its own published
caches, while Word oracle comparisons assert the captured lifecycle and
per-placement render separately. The equal H2
n, r and w results in this capture do not prove wider switch equivalence.
Retain discriminating mode tests over the shared authoritative facts.

Table-cell and nested-control headings consume the same authoritative numbering that visible layout uses. Set effective heading metadata in the existing shared paragraph construction
path, including table-style and paragraph-style cascade plus direct overrides.
Carry it through existing table pagination, recording a heading once at its
first actual placed fragment. Repeated table header painting must not duplicate
navigation sources. Do not add a second all-paragraph navigation tree or mutable
index. Use existing `ParagraphBlock` metadata and source IDs, adding private
operation-local facts only where the current representation demonstrably lacks
an authoritative fact. Renderer outlines use the resolved visible marker, suffix policy and accepted heading title. Preserve tab versus space suffix semantics in field results, while outline display uses a documented readable separator. No new number parser or conversion is added.

Generated TOC, TOF, TOA and bibliography ranges are excluded from new source discovery. Copied fields inside result caches must not advance caption sequences or manufacture source headings. Each existing explicit cache/navigation operation stages, validates, reparses
and commits once. Preserve its public transaction boundary rather than adding
a new umbrella update API. A caller may compose existing updates, obtaining a
fresh source-qualified snapshot after each structural rebuild. The regression
must assert the final layout after the last generated-result growth, not only a
pre-insertion snapshot. Stale paths, ambiguous targets or invalid dependencies cannot partly publish. Ordinary save remains leave alone.

## Rejected alternatives

- Separate TOC counters disagree with table cells, restarts and accepted revisions.
- HeadingN spelling alone misses direct/inherited outline levels and suppression.
- Cross-story flattened ordinals select unrelated paragraph markers.
- Existing outline API changes would break consumers when additive results suffice.
- One full-context STYLEREF result for every switch loses Word semantics.

Unplaced eligible headings are limited to sources admitted by the existing
shared effective-paragraph provider. Raw storage in vertical-merge continuation
cells remains outside that provider, as in established table layout. No skipped
cell traversal or counter is added. An actually unplaced admitted source has no
invented page, and unsupported STYLEREF source selection retains its cache.

Direct resolved numbering-level `marker_rpr.vanish=true` suppresses marker ink
and the outline's visible marker only. The numbering counter, suffix layout,
accepted heading and counter-derived REF/STYLEREF values remain independent.
No paragraph-mark or ordinary-text hidden property is inferred as marker
formatting. Actual Word hidden-marker field parity remains final native debt.

An ambiguous generated-owner scan becomes a private stamped failure fact on
WordSequenceSnapshot through the narrow doc-hidden
`set_navigation_failure(&mut self, &LayoutInput, String) -> Result<()>` bridge.
Fresh sequence regeneration preserves only the exact-current-document and
physical-part-qualified failure. Ordinary rendering keeps existing stored cache
paint and counter behavior and reports an existing layout Diagnostic. The new
numbered outline refuses through `WordLayoutResult::navigation_failure() ->
Option<&str>`, and newly supported STYLEREF retains cache with an ordered
diagnostic. Explicit ambiguous rebuilds retain their existing atomic refusal.
This is not an empty successful source discovery or a new ordinary REF/SEQ
policy. Snapshot equality includes the private status and invalidates caches.

TOC selectors reuse the existing source-qualified navigation tuple. Explicit
custom-style selection remains independent. Heading-style selection without
u keeps its established rule, while u requires the qualified shared heading
eligibility, including effective inherited or direct outline9 suppression.
An unavailable owner is an explicit refusal, not a fabricated suppression fact.
No additional cascade, flag, counter or per-field source scan is introduced.

## Test plan

**Test gate**: regression. One multilevel numbered document stays consistent
across visible markers, navigation structures, references, and saved caches.

The worker first retains actual old-source RED and then fixed-source GREEN for
the same focused regression bodies. Existing passing controls are not relabelled
as new failures. Extend the existing `regression_test.rs` and existing module
tests, without a new test binary or committed binary fixture.

| Timing | Category | Test | Asserts |
|---|---|---|---|
| local scoped | regression | `multilevel_navigation_remains_consistent_after_mutation` | One public source-built document agrees across actual positioned markers, TOC, STYLEREF, REF, PAGEREF, captions, numbered outline and reopened caches before and after insertion, deletion and restart to 5. Final pagination follows generated-result growth |
| local scoped | regression | `table_and_control_headings_share_navigation_numbering` | Body, table cells, nested tables and valid nested controls preserve accepted physical order, style cascade and unique source paths. Repeated table header placement does not duplicate a source |
| local scoped | regression | `suppressed_paragraphs_do_not_invent_navigation_markers` | numId zero, outline level 9, hidden marker, deleted/moved-from content and accepted paragraph-mark joins remain distinct. Pure evaluation does not invent unsupported numbering facts |
| local scoped | regression | `styleref_number_modes_share_authoritative_counter` | Discriminating n/r/w/t combinations reuse authoritative values. Nearest and actual first/last page selection differ, including first-page forward fallback and repeated furniture |
| local scoped | regression | `relative_ref_uses_physical_caller_context` | Source and target in distinct table/control/related owners cannot alias flattened ordinals. Same-instance, shared-definition cross-instance and explicit-restart controls retain concrete IDs |
| local scoped | regression | `generated_results_are_not_navigation_sources` | Existing TOC, TOF, TOA and bibliography caches manufacture neither headings nor caption sequence events |
| local scoped | round-trip | `numbered_navigation_caches_reopen_without_change` | Library save/reopen preserves published caches, instructions, bookmarks and opaque neighbouring XML. Legacy outline APIs retain signatures and title-only semantics |
| local scoped | regression | `navigation_updates_are_atomic` | Stale/foreign source bindings, ambiguous targets and invalid physical owners leave the live package and held owners unchanged. Unsupported valid fields retain caches and ordered diagnostics |
| final sprint | differential | `navigation_switches_match_fresh_word_record` | Final integrated producers match actual Word updated, saved and normally reopened fields plus per-placement PDF text for the measured switch/page/continuation/restart controls |
| final sprint | differential | `navigation_selectors_keep_distinct_suppression_rules` | TOC o/u, o-only and custom-style selection distinguish outline suppression, numId zero and omitted pages. Numbered REF retains the measured zero rather than a missing-marker fallback |
| final sprint | risk rider | `/verify --full` plus the union below | Full native/binding consumers, deterministic 49-hash and 7-pixel checks, package/API and WASM qualification run once over the final integrated source |

Build primary inputs through current public dependency APIs. If a physical
owner cannot be authored through those APIs, use a readable in-code XML
construction confined to that discriminator and retain the actual package and
source path. Never fake a counter or compare duplicated text alone. Assert
positioned marker glyph/text provenance as well as scalar field values.

Historical evidence was described at the paths in `.claude/scratch/F-283-progress.md`,
including `/private/tmp/S90-F283-evidence-map.md`. Current custody inspection
finds that map, the navigation checkpoint/source records and PR336 assessment
absent. The surviving narrative is historical only, not an authenticated native
packet. Reconstruct and capture the declared discriminators at final acceptance.
The historical account says initial/mutated captures
already distinguish H3 REF n/r/w/wt, numId-zero REF from markerless outline,
TOC selection, restart 4 to 7, table/control/direct-outline sources and three
page furniture placement. Equal H2 n/r/w values are nondiscriminating.
Hidden-marker numbered REF remains unmeasured. PR336 source assessment at
`/private/tmp/S91-PR336-935ac16-intake/assessment.md` is a proposal, not a native
oracle. Do not adopt abstract-definition REF context from its contributor test
or infer it from F-X200 visible continuation.

At final integrated acceptance, pin the actually available Microsoft Word
version/build, locale and source fingerprint before capture. Current native
availability is Word 16.114 build 26101123, which must be authenticated again
when the final capture actually runs. Preserve the old
16.113.2 build 16.113.26092012 records as historical observations, without
claiming that build is still installed. Reuse admitted discriminator recipes
and source identities, then qualify the final integrated producers against
actual explicit update, save, close and ordinary disk reopen. Capture source
numbering/style/target records, field values and actual per-placement output.
Missing required switch evidence stays pending or failed, not synthetic parity.
No external oracle runs per worker.

Native lifecycle normalization is separately classified. Historical captures
remap three bookmark IDs and remove a row property exception. The mutated
capture adds grammar markers and splits six REF cache runs. These are not
package byte-equality failures. Assert semantic ranges, effective formatting
and generated results independently. Library ordinary save remains cache
preserving. Native shared-part cache changes do not replace per-page painting
observations.

The local floor is the named RED/GREEN controls, affected-crate fast checks,
formatting and `/verify --scoped F-283`, plus an independent zero-defect,
zero-smell microscope. Use the integrator's exclusive existing build lease,
`CARGO_INCREMENTAL=0` and deterministic font mode. No worker whole-suite,
publication, WASM, hash/pixel or native rider replay. Final debt remains named
in progress and handoff until the single integrated union actually passes.

The optional `RDOCX_F283_NATIVE_OUTPUT_DIR` hook in the existing regression
carrier writes each shared real fixture's baseline or actual mutation once,
using fresh immutable DOCX and JSON names. Field labels distinguish all six
REF/STYLEREF modes and concrete context cases. Records separate the expected
library observation from the unmeasured Word result. No second generator,
new product file or early native oracle is introduced. Input generation can
join the final workspace run instead of replaying a complete suite.

## HLD impact

- `docs/hld/02-scope-and-non-goals.md`
- `docs/hld/03-architecture.md`
- `docs/hld/08-rendering-spec.md`
- `docs/hld/10-bindings-spec.md`
- `docs/hld/12-testing-strategy.md`
- `docs/hld/14-development-backlog.md`

## Risk routing

These riders are added to the final integrated union, not per-worker work.

- Layout/pagination, `docs/hld/08-rendering-spec.md`: deterministic fonts,
  source-qualified marker and placement comparisons, ordinary/incremental
  layout equivalence where the changed heading metadata participates, and the
  existing 49-hash/7-pixel gate. Declare and independently review any actual
  intentional delta before baseline recording.
- Parser/serializer, `docs/hld/04-opc-and-packaging.md` and
  `06-presentationml-model.md`: extend existing owned field/cache writers only.
  Prove schema order, qualified physical ownership and byte-preserved opaque
  neighbours on successful update, refusal and reopen.
- Published Rust API, `docs/hld/10-bindings-spec.md` and CLAUDE structural rules:
  additive new result and method only, existing consumer signatures unchanged.
  Final genuine `cargo publish --dry-run` for the affected package union and
  existing archive size assertions, including the 10 MiB ceiling. Final Python
  and WASM consumers must still compile without exposing a new binding API.
- External oracle, `.claude/skills/differential-testing.md`: independently
  captured actual pinned Word field and per-page observations above. Record
  native lifecycle normalization, do not silently rewrite expected values.
- Crate boundary, `docs/hld/03-architecture.md`: existing facade-to-layout
  dependency only. No layout-to-facade edge or new cross-family dependency.
- No font asset, unit conversion, release, flag, trait, generic, crate, module
  or product file is proposed. Existing font resolution and counter ownership
  remain unchanged.

## Hash harness

Expected unchanged. The seven samples are fixed at `scripts/hash_harness.py:33` and generated by generate_all_samples at `scripts/hash_harness.py:316`. Their numbered-looking heading prefixes are authored literal text, for example `crates/rdocx/examples/generate_all_samples.rs:177`. Numbered list items at `crates/rdocx/examples/generate_all_samples.rs:340` are separate non-heading paragraphs. The custom style at `crates/rdocx/examples/generate_all_samples.rs:616` inherits Normal and sets shading/run formatting, not direct outline or numbering. The generator does not call set_numbering or set direct outline levels on headings or table-cell paragraphs. Legacy headings/document_outline calls at `crates/rdocx/examples/generate_all_samples.rs:644` and `crates/rdocx/examples/generate_all_samples.rs:652` retain existing behavior. Therefore this additive API and newly numbered/table/direct-outline cases should not move the existing 49-entry baseline. No baseline claim is allocated. If integrated checks expose an unexpected change, stop and diagnose rather than record it.

## Exclusive file claims

- `crates/rdocx/src/field.rs`
- `crates/rdocx/src/document.rs`
- `crates/rdocx/src/lib.rs`
- `crates/rdocx-layout/src/lib.rs`
- `crates/rdocx-layout/src/engine.rs`
- `crates/rdocx-layout/src/style_resolver.rs`, only if a shared representation gap is demonstrated
- `crates/rdocx-layout/src/table.rs`, for heading metadata propagation
- `crates/rdocx-layout/src/paginator.rs`, if actual table-heading placement requires it
- `crates/rdocx-layout/src/notes.rs`, one existing private paragraph-call argument to retain physical navigation identity
- `crates/rdocx/tests/regression_test.rs`
- HLD files listed above

Formal prerequisites are done. F-X192 and F-284 can share these files, so
implementation writes serialize through explicit leases and current integrated
bases. Their drafts do not grant concurrent source ownership. Do not change
F-X192 formatter cases or F-284 template grammar as navigation work.

## Implementation checklist

- [x] Approve this revised design and reconcile current completed providers.
- [x] Retain actual old-source RED for each new focused regression group.
- [x] Extend numbering demand and STYLEREF using authoritative source facts.
- [x] Add the source-qualified outline without changing legacy accessors.
- [x] Carry effective table/control heading metadata to actual pagination.
- [x] Prove distinct suppression, revision, physical ownership and cache rules.
- [x] Exercise public mutation, continuation, restart-to-5 and final layout refresh.
- [x] Pass the named local GREEN, fast scoped checks and independent microscope.
- [x] Handoff with explicit final Word, package, consumer and harness debt.
- [x] Integrator runs the final union once before final completion ledgers.

## Open questions

No new product file, module or API choice beyond the previously approved
additive `NumberedOutlineNode` and `numbered_document_outline` is requested.
Existing accessors keep their behavior. The earlier explicit approval of
that additive API persists without a repeat question. No approval for changing `ResolvedNumbering` public
fields or concrete-instance REF context is inferred from PR336.

The cross-instance REF n/r/w/t and hidden-marker outcomes require actual native
qualification. Reuse the concrete-instance contract until evidence earns a
bounded change. If native observations require a new public field or a different
compatibility policy, stop for the material design revision and approval rather
than expanding this draft silently. Availability of a fresh Word capture remains
final-gate debt, not a prerequisite on F-X192 or an invented local GREEN.
