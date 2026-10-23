# F-284, Stable container-wide template grammar

**Status**: completed
**Sprint**: S95.2
**Size**: L
**Depends on**: F-253, F-261, F-276

## Problem

The current evaluator stages body blocks and then replaces scalar tags in other
stories at `crates/rdocx/src/template.rs:137`. Its explicit refusal at
`crates/rdocx/src/template.rs:714` limits structural controls to the main body
and table rows. Path parsing at `crates/rdocx/src/template.rs:1225` has no
filter or literal-escape grammar. This leaves the same template payload with
different structural behavior depending on its physical owner.

`Document::render_template` at `crates/rdocx/src/document.rs:34750` is already
public. Python also exposes the method at
`crates/rdocx-py/src/document.rs:6975`. The older native-only prose at
`docs/hld/10-bindings-spec.md:2484` is stale. Preserve the working Python JSON
admission and publication contract rather than designing another converter.

## Spec reference

- `docs/hld/14-development-backlog.md`, "F-284, Stable container-wide template grammar".
- `docs/hld/03-architecture.md`, structured template evaluation under "What stays put" and "Shared Word document operations".
- `docs/hld/04-opc-and-packaging.md`, "Checked related-story publication" and "Checked Word operation source ownership".
- `docs/hld/10-bindings-spec.md`, native Word template grammar, rich mail-merge formatter and Python template conversion contracts.
- `docs/hld/12-testing-strategy.md`, scalar, structural, repeating-table and continuous-numbering template contracts.

## Approach

Complete the existing evaluator in `template.rs` and physical-owner transaction
helpers in `document.rs`. All three prerequisites are done in the current
backlog. No new product file, module, dependency, trait or feature flag is
proposed. The user approved both recommended choices on17October2026: preserve
legacy default behavior with grammar1 opt-in, and add the native Rust formatter
while Python shares built-in filters. No Python callback is included.

### Version and compatibility

Publish `pub const TEMPLATE_GRAMMAR_VERSION: u32 = 1` through the existing
facade exports. The approved design preserves the existing
`render_template(&Value) -> Result<usize>` behavior, including punctuation in
legacy bare property keys. Add
`Document::render_template_versioned(&Value, u32) -> Result<usize>` admitting
only version1, and Python `render_template(data, *, grammar_version=None)`.
`None` retains legacy behavior and `1` selects grammar1. Other versions refuse
atomically. Both surfaces use the same native parser and evaluator, without a
public options wrapper. The new formatter method explicitly selects grammar1.
Existing scalar conversion, truthiness, lexical scopes, return counts and
first-run formatting persist within each declared grammar.

Grammar1 reserves pipe, bracket and quote punctuation in bare property keys.
Callers use its quoted-key form for such keys. Do not select grammar by testing
which keys happen to exist in a payload. The legacy entrypoint retains its original property-key grammar. Grammar1
selection is explicit and independent of the keys present in a payload.

Use dotted object paths and add bracket array indices and quoted object keys.
Quote parsing must admit delimiter characters in a key without treating them
as an early tag close. Missing paths remain errors unless an explicit `default`
filter supplies a fallback. Wrong intermediate container kinds always refuse,
including when a later `default` filter is present. Arrays and
objects remain invalid scalar output unless a filter converts them.

### Prepared occurrence bridge

The existing accepted-run placeholder mapper owns split-run and wrapper
boundaries. Add only the narrow doc-hidden existing-type helper in
`crates/rdocx-oxml/src/placeholder.rs`:

```rust
pub fn replace_prepared_occurrences(
    paragraph: &mut CT_P,
    occurrences: &[(usize, std::ops::Range<usize>, String, Option<CT_RPr>)],
) -> rdocx_oxml::Result<()>;
```

The tuple identifies an original replaceable stretch, its UTF8 byte range,
literal replacement text and optional replacement-only run-property override.
Validate all bounds, character boundaries, nonoverlap and XML-safe results
before publishing a cloned paragraph. Reuse the substantive existing accepted
mapper and run-splitting providers, preserving unmatched prefix and suffix
formatting even inside one original run. Inserted text is never rescanned or
subject to global sentinel restoration. No new public carrier, generic matcher,
module or trait is introduced. This is an approved internal provider bridge,
not another template grammar. Scoped checks include the changed oxml crate and
focused mapper controls. Final publication/API/archive riders include that
crate's additive doc-hidden existing-type function.

### Original raw textbox scalar preservation

Retain the established unknown-wrapper raw textbox scalar boundary through the
existing placeholder rewriter, excluding actual checked physical textbox
ranges and all their qualified aliases. Add only this doc-hidden existing-type
bridge in the same placeholder.rs:

```rust
pub fn replace_original_literals_in_xml_part(
    xml: &[u8],
    excluded: &[std::ops::Range<usize>],
    replacements: &[(&str, &str)],
) -> rdocx_oxml::Result<(Vec<u8>, usize, Vec<String>)>;
```

Empty operands discover original logical source texts. Resolve source-local
operands before edits, then match simultaneously against original stretches,
with leftmost longest-match precedence including escapes. Reuse the prepared
mapper, never sequential substitution or inserted-data restoration. Existing
AlternateContent counting and copy policy remain. Propagate original offsets
through that existing rewriter rather than infer aliases from displayed text.
Formatter mode preflights unqualified active raw textbox scalars before any
callback and names the missing authentic original paragraph context. It does
not invent ContentLocation or admit unknown structural owners.

Normalize the staged clone once before execution facts. Pair original callback
locations to normalized fresh physical programs through checked part, owner,
item count and kind correspondence. Only original locations are callback
provenance. Fresh physical ranges remain edit authority. Reuse exact original
numbering identities only in this private same-document range import mode,
after proving original definitions and instances still agree. Existing public
fragment/equivalence policies and normal generated-ID canonicalization remain.

### Chart formatter context boundary

Existing chart text and cached-value carriers remain supported for legacy and
versioned built-in substitution. They do not establish an original Word
paragraph ContentLocation. Formatter mode therefore preflights active chart
scalar occurrences and refuses atomically before invoking any callback, naming
the chart part and recommending render_template_versioned. Literal escapes do
not invoke callbacks or cause this refusal. No fake paragraph or chart story
kind is introduced, and no new ownership resolver is added solely for the hook.

### Scalar filters and formatting

Add ordered `| trim`, `| upper`, `| lower`, `| default("literal")` and
`| join("separator")` filters. These are deterministic Unicode/string
operations, with no locale, date, host formatter or executable expression
dependency. `default` supplies its literal only for a missing path or null,
not for false, zero or an empty string. `join` requires an array of scalar
values and converts each with the existing scalar rules. Filter arguments are
JSON string literals. Unknown filters, wrong arguments and invalid values
refuse the whole transaction. Inserted data is literal XML-safe text and is
never scanned as another template program.

Add the approved native formatting hook:

```rust
pub struct TemplateFormatContext<'a> {
    pub location: &'a ContentLocation,
    pub path: &'a str,
    pub occurrence: usize,
}

impl Document {
    pub fn render_template_with_formatter(
        &mut self,
        data: &serde_json::Value,
        formatter: &mut dyn FnMut(
            &TemplateFormatContext<'_>,
            &mut MailMergeFormattedText,
        ) -> Result<()>,
    ) -> Result<usize>;
}
```

Reuse the existing `MailMergeFormattedText` text/run-property carrier rather
than creating an identical wrapper. The callback runs after built-in filters
once per actual scalar occurrence, in declared physical-owner/source order,
with a zero-based global occurrence count. Its location identifies the
original paragraph before expansion. Repeated copies invoke separate callbacks
in expanded output order. Replace literal-wide callback grouping with
occurrence-local staging, so identical tags can produce distinct formatted
results without modifying an unrelated occurrence. Preserve first-run formatting unless the
callback explicitly replaces run properties. Validate callback text and
properties before commit. Callback external side effects cannot be rolled
back and are outside document atomicity. Rust and Python use the same native built-in grammar evaluator under the
selected default/opt-in policy. A Python callback, a new WASM
method and CLI commands are separate public choices, not implied additions.

### Whitespace and escapes

Preserve all text outside consumed tags. Whitespace immediately inside tag
delimiters is insignificant. Dedicated structural marker paragraphs and rows
are consumed, including their whitespace, as today. Do not introduce implicit
line stripping or Jinja whitespace-minus syntax. Define explicit literal
opening escapes `{{{{` for `{{` and `{%{%` for `{%`, decoded once through the
existing sentinel staging. Matching closing escapes are `}}}}` for `}}` and `%}%}` for `%}`.
Scan escapes before tag openers, left to right, with delimiters inside quoted
strings protected. Restore sentinels once. Escapes must cross ordinary run boundaries
with the same first-run formatting rule as scalar tags.

### Qualified field boundary admission

The existing private scan_story_items and scan_story_items_with_scope functions
accept an additional template_field_ranges: Option<&mut Vec<Range<usize>>>
argument. Only template preparation supplies Some. All ordinary callers supply
None and retain identical typed inventory semantics. During that same qualified
marker event traversal, template-only open/depth facts survive paragraph ends.
They reuse complex_field_marker, complex_field_scan_context and the existing
opaque/nested-physical-owner guards. Exact intervals and unmatched boundary
intervals mark only structural admission, with a bounded sorted interval join.
The existing paragraph-local typed Field projection still clears its parser
state at each paragraph end. No field opcode parser, new scanner, ownership
registry, public field or source type is added. Marker programs inside or
crossing those intervals refuse before callbacks. Ordinary scalar field-cache
replacement remains on its established accepted-run boundary.

### Container-wide structural expansion

Use the existing story inventory and checked `ContentLocation`/physical source
owner logic. Parse and pair nested `for`/`if` controls within one sibling
container, including a cell, header, footer, individual note or text box.
Never pair markers across owners, notes, cells, field boundaries, or alternate
text-box representations. Enumerate a shared header/footer once per physical
part, not once per section reference. Exclude note separators and continuation
separator records. Preserve chart-label scalar behavior without inventing a
paragraph/row structural grammar for charts.

Existing qualified equivalent text-box representations form one logical
template owner. Evaluate its selected source program once, count and invoke
formatting callbacks once per emitted logical occurrence, then mirror the
admitted structural edits through the existing paired-representation provider.
Do not pair owners by displayed text or marker spelling. Asymmetric or
unqualified representations retain the existing preservation or refusal
boundary.

Reuse S89's supported fragment/dependency allocation and namespace closure
inside one staged candidate. Public clone/import methods reopen and commit per
call, so reuse their substantive private staging/allocation operations instead
of calling those public transactions in an item loop. Repeated source ranges must not duplicate
bookmarks, control identities, drawing identities, note/comment companions or
invalid relationship operands. Keep continuing numbering identities as today.
Freshly inserted owners must not cause a second evaluation of data or source
tags. Validate expanded source before replacement and refresh physical paths
after each structural edit. Do not publish an intermediate document per item.
Use existing refusal boundaries for unsupported dependency closure, and test
them honestly rather than bypassing the fragment transaction with raw copies.

Preserve container-required final paragraphs and table structure. A loop that
would remove required cell content leaves the schema-required empty paragraph.
Section properties retain their existing body-only placement rules. Source
attributes, inactive alternate content and unmodelled XML outside explicitly
selected template text/ranges remain byte-preserved. Serialize, reopen and
publish once after all owners and callback results succeed. Errors retain
original bytes, relationships and held Python owners.

## Rejected alternatives

- An embedded general expression engine introduces unsupported execution and dependency surface for a bounded text grammar.
- A second fragment importer duplicates relationship and identity rules already owned by completed providers.
- Separate per-binding parsers make identical template payloads diverge.
- Per-item publication loses atomicity and invalidates later source locations.
- Replacing the existing method signature breaks current Rust and Python callers.

## Test plan

**Test gate**: regression. The versioned grammar renders the same payload across
body, table, header, footer, note, and text-box locations without run-split loss.

| Timing | Category | Test | Asserts |
|---|---|---|---|
| local scoped | regression | `versioned_template_payload_is_equal_in_all_supported_owners` | Actual old-source RED and fixed-source GREEN for split-run scalar/filter, nested loop and conditional controls in each required owner |
| local scoped | regression | `template_paths_filters_and_literal_escapes_have_stable_semantics` | Dotted/quoted/indexed paths, Unicode filters, missing/null distinction, literal delimiter data, invalid syntax and argument refusal |
| local scoped | regression | `template_formatter_observes_occurrences_without_losing_run_formatting` | Stable physical source locations/order, distinct callbacks for identical literals, explicit run-property override and late-error atomicity |
| local scoped | regression | `template_expansion_preserves_owned_dependencies_and_source_identity` | Repeated links/images/bookmarks/notes/controls use the existing supported allocation rules and opaque source remains exact |
| local scoped | regression | `template_owner_boundaries_and_required_cell_content_remain_valid` | Shared furniture evaluated once, separate notes/text boxes cannot pair markers, empty cells remain valid, sections stay body-owned |
| local scoped | round-trip | `container_wide_templates_reopen_with_valid_relationships` | Exact rendered order/text and effective formatting, numbering, namespace scope and supported dependency graph after save/reopen |
| local scoped | integration | `python_template_grammar_keeps_atomic_owner_publication` | Existing Python method consumes the same grammar, rejects malformed JSON values and preserves captured owners on refusal |
| final sprint | differential | Source-built readable mixed-container document, actual Word and pinned LibreOffice open/save/reopen | Declared text/order/effective formatting and valid supported relationships observed independently, tool/build/input identities retained |
| final sprint | integration | Integrated full gates, installed Rust/Python surface checks and affected package publication riders | Real final-source suites, typing/stubs, WASM, deterministic hashes/pixels, docs and genuine archive verification once |

Extend existing test entrypoints. No new binary fixture or test binary. Focus
local checks on the changed code and named RED/GREEN cases. External native
observations and full suites remain pending until final integrated acceptance.
Rendering comparisons use bundled fonts and predeclared supported structural
and appearance observations, not a new exact-font-lineage acceptance engine.

## HLD impact

- `docs/hld/03-architecture.md`
- `docs/hld/04-opc-and-packaging.md`
- `docs/hld/10-bindings-spec.md`
- `docs/hld/12-testing-strategy.md`

## Risk routing

- Parser/serializer: read packaging and sequence conventions. Final union checks namespace-qualified parsing, schema child order, exact opaque XML preservation and reopened relationship ownership.
- Published API: additive pre-1.0 constant, context and formatter method require explicit design approval. Final union includes genuine affected `cargo publish --dry-run` and the unchanged package size budget.
- Python binding: existing method shares native grammar. Final union includes installed runtime, strict typing/stubs and both WASM graphs. No new callback or WASM surface is assumed.
- External oracle: apply differential-testing. Pin actual Word build and LibreOffice26.2.5.2, record actual executions and classify normalization separately.
- Layout/output: generated content may affect pagination. Use deterministic fonts and explain any expected harness delta before recording. No font or unit conversion change is proposed.
- New file/module/trait/generic/flag: none. Callback invocation follows the existing mail-merge function parameter convention without a new generic parameter.

## Hash harness

Expected unchanged for the 49 existing documents and seven pixel cases. Template
rendering remains opt-in. Any unexpected output delta blocks completion.

### Prepared related-note occurrence bridge

Structural replay leaves original scalar operands intact. Existing staged
fragment import records ordered private tuples
`(StoryKind, source_part, old_id, destination_part, new_id)` through a mutable
`Vec` argument. Ordinary import callers discard those facts. Template rendering
joins each actual allocation to its original checked note program and already
evaluated execution. No tag or data evaluation occurs again. The private
`template_note_identities(&self) -> Result<Vec<(StoryKind, String, i32, usize)>>`
validates normal-note ID uniqueness and physical owner count/order against the
existing provider. Fresh checked inventory after structural expansion determines
callback order, including original and appended notes. Original paragraph
locations remain callback provenance. Prepared scalar edits are applied only
after this physical-order callback phase. All work remains on the staged clone.

## Implementation checklist

- [x] Settle the concrete public grammar, compatibility and formatter decisions.
- [x] Implement grammar parsing/evaluation and occurrence-local formatting in existing carriers.
- [x] Extend structural evaluation through checked existing physical story owners and fragment transactions.
- [x] Add actual named local RED/GREEN and atomic preservation controls.
- [x] Update exactly the named HLD intent, including the existing Python method.
- [x] Consume focused scoped checks and a fresh independent zero-finding microscope.
- [x] Qualify the final integrated native/oracle/full/rider union and complete records once.

## Open questions

None. The user approved both recommendations on17October2026. Existing calls
retain legacy grammar. Explicit version1 enables the bounded grammar through
Rust and Python. The native formatting callback uses MailMergeFormattedText.
Python callbacks, new WASM methods and CLI surfaces remain outside this story.
