# F-X211, Python script compatibility and safe XML access

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: F-X211a, F-X211b, F-X211c and F-X211d, retaining F-X200 through F-X210 for final corpus acceptance and refined source barriers below

## Problem

Issue316 introduces a measurable first-draft compatibility contract, validated
writable element XML, stable owners and actionable errors. Current Word
test_python_docx_parity.py:60 holds17 documentation examples, while
PowerPoint test_documented_examples.py:34 explicitly re-fetches handles and
:701 holds7 pinned records. HLD10:160 through :174 documents their adaptation,
not a100-script unmodified correct-file metric. CI runs installed binding
tests with exact oracle versions at .github/workflows/ci.yml:220/:221.

PowerPoint Shape.xml is bytes-only read at shape.rs:568 and Word
StoryItem.xml is a snapshot read at document.rs:540. No replace_xml wrapper
was found on the requested owners. Global generations stale unchanged
ancestors, including table.rs:50 row/column restructure bumping at :66 and
notes_text at slide.rs:229. Word Hyperlink edit resolution at
document.rs:1545 uses a whole link-layout digest and fails after unrelated
add/remove. Stale errors currently name revisions and re-fetch paths but
not the mutation call. Provider fixes cover some examples, not this complete
cross-binding acceptance.

## Spec reference

- docs/hld/14-development-backlog.md, "F-X211, Complete Python script
  compatibility and safe XML access", full316 acceptance and providers.
- docs/hld/10-bindings-spec.md, "Python API shape", the17/7 compatibility
  gate, XML/preservation boundaries, and "The invalidation problem, handled
  loudly".
- docs/hld/12-testing-strategy.md, "Binding tests" and "What CI runs".
- docs/hld/02-scope-and-non-goals.md, public compatibility and current raw
  XML escape-hatch boundary.
- docs/hld/04-opc-and-packaging.md, actual owner relationships and preserved
  parts.
- docs/hld/06-presentationml-model.md, part owners, child sequence and
  retained source.
- docs/hld/03-architecture.md, allowed dependency graph and concrete ownership.
- docs/hld/08-rendering-spec.md, deterministic saved/rendered evidence.

## Approach

### Full acceptance and provider barriers

Preserve all316 requirements: about100 short idiomatic scripts spanning
303..313, saved-file assertions and per-area unmodified correct-file rates
in CI and final PR, existing17/7 examples as subset, declared actionable
divergences, complete element XML read/write/type validation, named supported
handle survival and invalidating operations, silent-failure audit and helpful
attribute/type errors.

Final corpus acceptance depends on integrated F-X200..X210. Corpus design
and native inventory can proceed read-only before all those stories finish.
Implementation barriers are actual ownership contracts:

- F-X194 and F-X206 must settle the common presentation text/layout/topology
  partition before XML adapters capture or invalidate handles.
- F-X202 must settle Word hyperlink and picture/physical-owner operations.
- F-X201 supplies checked section ownership and geometry validation.
- F-X209 supplies checked layout/master owners and editable native records.
- Other feature stories supply the exact APIs exercised by the frozen corpus.

Known fonts296, fragment-link305 and section304 fixes remain F-X195, F-X202
and F-X201. F-X194/F-X206 own already-requested cell/layout/title stability.
Do not reproduce their source fixes here. X211 owns residual Word hyperlink
and table-grid ancestor stability in both bindings unless the root explicitly
amends those provider plans to own that implementation. Notes ancestor
survival is assigned once alongside X206's title-text survival. X211 retains
the cross-story regression and acceptance evidence either way.

No new module, crate, dependency or feature flag is proposed. Existing test
entrypoints can hold the inline corpus rather than100 standalone files.
Root owns canonical plans, ledgers, baseline lease and final integration.

### Frozen first-draft corpus and rate

Extend existing Word test_python_docx_parity.py/test_core.py and PowerPoint
test_documented_examples.py with a frozen typed manifest of about100 short
scripts. Each record has stable ID, family, area, exact statements, expected
saved-file assertion and any explicitly deliberate divergence. Cover Word
formatting/lists/styles/tables/pictures/header/footer/notes/template/export,
PowerPoint slides/placeholders/shapes/tables/notes/fills/master/layout/theme/
fit/export, and the remaining303..313 operation families. Plan the per-area
allocation before measurements, not after observing failures.

Scripts run under isolated source-built installed bindings. They use rdocx or
rpptx imports with upstream idioms and no injected re-fetches, coercions,
aliases or preprocessing. Execution alone is insufficient. Reopen each
saved file and assert modeled structure and package invariants. Compare
supported public records against pinned python-docx1.2.0/python-pptx1.0.2.
Use deterministic rendering for visually meaningful assertions and preserve
actual native oracle provenance where a producer comparison is required.

Keep the existing17/7 gate, including its declared adaptations, as a named
subset. The metric also records whether the original upstream-style body
runs unmodified. An adapted pass is not an unmodified first-draft pass.
The existing held-row and title/subtitle adaptations become explicit baseline
cases. Do not quietly edit their source bodies to make the rate rise.

Report per area: attempted count, executed count, correct-file count,
deliberate divergence count, unexpected failure count and percentage of
attempted scripts that ran unmodified and yielded correct files. Deliberate
divergences remain in the denominator and do not count as successful runs.
Their expected actionable-error tests can pass independently. CI fails on
unexpected failure, missing assertion, duplicate/missing manifest identity or
unreviewed divergence. Retain fixed denominators and stable corpus digest.

Capture a real Base rate and final integrated post316/post303..313 rate with
source SHA, installed build identity, interpreter/oracle versions and corpus
digest. Put the observed final rates in the eventual PR. This proposal
contains no invented percentages or completed test claims. Existing CI's
installed binding jobs are the integration point, with no new workflow file.

### Element XML API and concrete owner matrix

Public signatures:

```python
paragraph.element.xml: str
paragraph.element.replace_xml(xml: str) -> None
```

The same two operations apply to every requested owner. Keep existing
Shape.xml and StoryItem.xml byte snapshot properties compatible. The new
element view is not an lxml proxy, mutable tree or promise of arbitrary
private python-docx attributes.

Issue316 literally lists ten generic names, not eleven. Cover all of them.
Paragraph/run/table/cell exist in both families, yielding fourteen concrete
family cases before shape variants:

| Family | Owner | Expected qualified root |
|---|---|---|
| Word | Paragraph | WordprocessingML p |
| Word | Run | WordprocessingML r |
| Word | Table | WordprocessingML tbl |
| Word | Cell | WordprocessingML tc |
| Word | Section properties | WordprocessingML sectPr |
| PowerPoint | Paragraph | DrawingML p |
| PowerPoint | Run | DrawingML r for regular Run handles |
| PowerPoint | Table | DrawingML tbl |
| PowerPoint | Cell | DrawingML tc |
| PowerPoint | Shape | Actual captured shape kind, such as PresentationML sp/pic/grpSp/cxnSp/graphicFrame |
| PowerPoint | Text body | Actual owner root, PresentationML txBody or DrawingML txBody |
| PowerPoint | Slide | PresentationML sld |
| PowerPoint | Layout | PresentationML sldLayout |
| PowerPoint | Master | PresentationML sldMaster |

Reject a regular run replacement with a field/br root. Any field-node escape
contract is an explicit additional owner, not hidden behind Run. Shape
replacement preserves the actual captured kind unless an independently
approved conversion operation permits changing it. Table/text-body nesting
uses the actual namespace of its owner, not a fixed prefix string.

The concrete element view owns a document/presentation reference, existing
owner/path information and captured generations. It performs qualified type
validation and atomic owner mutation, so it is not a forwarding-only wrapper.
Use existing ParagraphLocation, AcceptedRunPath, ContentLocation,
StoryRunPosition, ShapeMut/Ref and the X209 part-owner discriminator as
applicable. No generic unsafe replace-part method, trait, Box<dyn> or new
revision framework. A finite concrete discriminator for actual owner cases
may reduce reader cases if the final source layout needs it.

Read returns namespace-complete UTF-8 serialization for exactly the selected
owner, without mutation or generation changes. Standalone XML may use the
existing canonical writer prefixes. Preserve intentionally retained raw
subtrees and required inherited namespace closure. Do not return a fragment
whose prefixes only work in a removed parent.

### Native write operations, validation and atomicity

Binding wrappers call checked native owner operations, not a Python package
editor. The native parser for each owner is inventoried before implementation.
Provisional concrete Word facade signatures for design review are:

```rust
Document::replace_paragraph_xml(&mut self, location: &ContentLocation, xml: &str) -> Result<()>;
Document::replace_run_xml(&mut self, position: &StoryRunPosition, xml: &str) -> Result<()>;
Document::replace_table_xml(&mut self, location: &ContentLocation, xml: &str) -> Result<()>;
Document::replace_cell_xml(&mut self, table: &ContentLocation, row: usize, cell: usize, xml: &str) -> Result<()>;
Document::replace_section_properties_xml(&mut self, section: usize, xml: &str) -> Result<()>;
```

Their final location meaning must match actual physical owner semantics,
including accepted nested controls, related stories, merged cells and run
identity. Do not adopt these provisional signatures if ContentLocation
cannot faithfully resolve the requested target. Existing types are reused
where valid, with concrete native gaps recorded rather than fabricated APIs.

PowerPoint uses equivalent concrete checked operations in existing lib.rs:
replace_shape_xml, replace_text_body_xml and explicit cell/table/paragraph/
run paths, plus replace_slide_xml, replace_layout_xml and replace_master_xml.
Finalize signatures against X209's actual part-owner API. Slide-relative
index parameters cannot safely masquerade as master/layout ownership.
Borrowed ShapeMut alone lacks package relationship scope, so whole-node
replacement stays in the owner-aware Presentation operation.

Every replacement must validate:

- Exactly one complete root with expected namespace URI/local name and no
  trailing root or forbidden XML characters.
- Prefix aliases and default namespace equivalence, with wrong foreign or
  rebound same-local-name root refused.
- Modeled element invariants, valid known attributes and schema child order.
  Do not claim a complete XSD validator from an existing permissive parser.
- Existing owner identity, relationship target/type/scope, IDs and applicable
  comments/fields/sections/table grids. A dangling or wrong-owner r:id fails.
- Existing physical-alternate-copy guarantees from X198/X202. Raw writes are
  not a shortcut around unsupported scoped ownership.

Parse, mutate a staged candidate, validate affected owner/package, serialize
and reopen before commit. Failure preserves native state, XML/parts,
relationships and all generations. Success applies one mutation and advances
only the generations the approved stable-handle contract requires.
Identical replacement no-op semantics are explicit and tested. Unsupported
known invariant validation must produce a named refusal, not a false success.

Unknown target XML remains preserved where the model already permits it.
Exact unrelated opaque bytes remain unchanged. Intentionally replacing a
target does not require preserving its old target XML. Cross-part relationship
creation/import is a separate checked operation, not implicit in replace_xml.

### Stable owner and invalidation contract

Keep the approved provider partitions and extend only residual cases.
PowerPoint Table remains the same shape owner after a row/column edit, while
old indexed Row/Column/Cell and paragraph/run/font descendants can become
unsafe. Word tables have the analogous owner/descendant distinction.
Use checked owner identities or concrete topology counters that actually
express this distinction. Do not re-fetch a table by an old index after its
shape owner was removed or replaced.

Word Hyperlink snapshots retain read facts, while live edit resolution must
identify the original physical hyperlink after unrelated add/remove. Current
document.rs:1545 uses index plus story_link_layout digest at :666. Value
equality, URL or relationship ID alone cannot distinguish identical links.
Design a checked physical selector or native identity preserving unrelated
links. Removed/replaced owner must fail clearly rather than resolve another
identical link. Preserve constructed snapshot ambiguity refusal.

SlideRef.id is lib.rs:7674 and ShapeRef.non_visual_id is :9636. Check actual
non-reuse, uniqueness and owner kind before choosing stable-ID resolution.
Master/layout owners come from X209. No unbounded ID registry or universal
stable-handle system is proposed. True removed/reordered unsafe descendants
retain loud checks, and raw writes use the same generations.

Extend the existing concrete revision counters with bounded operation
provenance. The error names the real API call responsible for the relevant
generation transition and the exact recovery path. Current shared
StaleElementError only records kind/captured/current revision/recovery hint.
Do not substitute the latest unrelated call for the first causal invalidator
without explicitly defining that weaker supported message. Final design
must prove repeated mutations, no-op/failure cases and owner-specific causal
reporting. No generic history tracker or speculative event framework.

### Silent failures and actionable names

Inventory accepted Python arguments and CLI options, their native consumer,
unit/value bounds and actual saved output. Every detected ignored value or
modeled invalid OOXML gets a failing Base public test, cited source, owning
story and fixed-after evidence. Keep the known matrix explicit:

| Case | Implementation owner |
|---|---|
| Image output silently ignores --font-dir296 | F-X195 |
| add_hyperlink(text, "#name") becomes external305 | F-X202 |
| Section lacks valid page size304 | F-X201 |
| Wrong unit produces inconsistent tiny page304 | F-X201 |

Those fixes are reused, not duplicated. Do not invent minimum page thresholds
or change canonical length truncation to hide a wrong Python unit contract.
Raw escape writes validate the same modeled invariants as regular authoring.

For deliberate differently spelled upstream names, a bounded known-name
mapping returns AttributeError/TypeError or the existing documented exception
with attempted name, accepted forms and correct rdocx/rpptx call. Correct
already-authorized features such as font.color.rgb through X197 rather than
leave a divergence hint after implementation. No fuzzy matching, automatic
coercion, hidden retry, silent setter ignore or lxml emulation.

## Rejected alternatives

- Duplicate provider fonts/links/section/colour fixes. One source change and
  reusable evidence prevents redo and semantic conflicts.
- Inject refetches into first-draft scripts. It measures adaptation quality,
  not unmodified agent success.
- Remove divergences from denominator or count raised errors as success.
  That inflates the required correct-file rate.
- Use arbitrary package-part XML writes or XML in binding wrappers. It loses
  native owner, relationship, schema and atomicity guarantees.
- Disable stale checks or match stable handles by values. Removed/indexed
  owners can silently bind to another object.
- Add universal traits, dynamic registries or new modules. Concrete current
  owner types suffice, and absent capabilities must be designed explicitly.

## Test plan

**Test gate**: regression. Installed isolated bindings run the full corpus
and assert saved package structure. All requested element kinds round-trip
opaque content and refuse wrong types atomically. Handles survive supported
edits, unsupported invalidations name the triggering operation, and each
silent failure has a failing-before/fixed-after public control.

| Category | Test | Asserts |
|---|---|---|
| regression | first_draft_corpus_runs_unmodified_and_reports_per_area_rates | Stable exactly100-script denominator, saved-file facts and honest divergence counts |
| regression | existing_documentation_examples_remain_identified_subset | All17/7 IDs and declared old adaptations retained |
| regression | actionable_divergences_name_the_supported_call | Known attribute/type names, correct forms and no hidden mutation |
| regression | element_xml_covers_all_requested_family_owners | Fourteen concrete cases plus declared shape-kind variants and namespace closure |
| regression | replace_xml_refuses_wrong_qualified_roots_and_trailing_content | All roots, aliases/defaults, foreign/rebound controls and unchanged bytes/generations |
| regression | replace_xml_preserves_owner_relationships_and_opaque_content | Shared media, hyperlink IDs, comments/fields, foreign nodes and save/reopen twice |
| regression | raw_xml_cannot_bypass_physical_alternate_owner_safety | X198/X202 supported/refused cases, no unequal physical copies |
| regression | table_owner_survives_grid_edits_but_unsafe_descendants_fail | Both families, row/column add/remove/move, merged cells and removed-table refusal |
| regression | hyperlink_edit_handle_survives_unrelated_add_remove | Identical links, physical owner identity and removed-owner failure |
| regression | provider_layout_slide_and_cell_stability_is_integrated | X194/X206 layout, title, notes, cell text and topology acceptance |
| regression | stale_error_names_the_actual_invalidating_call | Both bindings, repeated edits, no-op/failure and exact recovery path |
| regression | accepted_option_audit_reuses_each_known_provider_failure | Four known cases, real consumer inventory and per-case failing/fixed evidence |
| integration | installed_binding_ci_prints_final_corpus_rate | Isolated source-built package, exact oracle/interpreter provenance and schema |

Use existing native modules and integration/regression entrypoints. Python
tests stay in current test_core.py, test_formatting_tables.py,
test_python_docx_parity.py, test_documented_examples.py and current typing
entrypoints. Stubs describe element views and ownership. Extend existing
.github/workflows/ci.yml and existing test scripts only. No new tracked file
is required by this proposal. Run strict mypy/stubtest against installed
bindings and meaningful deterministic render/native comparisons.

No regression was executed by this design. Preserve Base failure, provider
evidence and final integrated corpus result as separate records. Root records
rates in the eventual PR, after genuine measured completion.

## HLD impact

- docs/hld/02-scope-and-non-goals.md, concrete XML escape and compatibility boundary.
- docs/hld/03-architecture.md, native owner/path/provenance responsibility if changed.
- docs/hld/04-opc-and-packaging.md, checked raw replacement and relationship guarantees.
- docs/hld/06-presentationml-model.md, actual raw owner/type mutation boundaries.
- docs/hld/10-bindings-spec.md, exact element API, stable handles, errors and corpus metric.
- docs/hld/12-testing-strategy.md, frozen corpus, rates, divergences and CI methodology.
- docs/hld/14-development-backlog.md, delivered316 acceptance and provider dependencies.

## Risk routing

- Parser/serializer. HLD04/06 mandatory, qualified root checks, correct child
  sequence and exact unrelated opaque-byte round trips. Issue331 adds bounded
  control-depth admission, recoverable error propagation, default-stack
  subprocess checks and at-limit traversal/clone/save/drop controls.
- Published native APIs. HLD10/structural rules, explicit additive pre-1.0
  changes, affected-crate archive dry-run/size and actual measured carriers.
- PyO3. HLD10, installed source-built tests, accurate strict mypy/stubtest,
  workspace binding exclusions and WASM graph compilation.
- Dependency edges. HLD03, no family reverse edge, native-to-Python bridge or
  generic mutator depending on binding-only path types.
- Layout/rendering and unit validation. HLD01/08/CLAUDE, canonical truncation,
  deterministic fonts and actual evidence for saved/rendered claims.
- External oracle. Canonical differential-testing skill, exact pinned
  python-docx/python-pptx/native producer versions and provenance.
- New concrete view/error fields. Structural rules, existing files and real
  current consumers, no unused framework or extra module.

Normal scoped gate and independent zero-defect/zero-smell microscope remain
required, followed by root's final integrated corpus/sprint verification.
Read HLD15 packaging before actual archive maintenance, not stale size reuse.

## Hash harness

Expect no default delta from metric/error infrastructure or additive XML
operations. Requested raw writes intentionally change only explicitly edited
targets, with opaque sibling controls. Any native parser/default rendering or
silent-behaviour correction gets its own labelled reviewed expected delta
under its actual provider. Root retains exclusive baseline ownership. Never
change baselines or corpus denominators to manufacture a passing rate.

## Implementation checklist

- [x] Reconcile provider ownership and final corpus/source capability barriers.
- [x] Freeze complete corpus IDs, statements, assertions and per-area denominators.
- [x] Measure actual isolated installed Base first-draft rate and missing API failures.
- [x] Inventory each native XML owner/parser/namespace/relationship validator.
- [x] Apply the shared native Issue331 control-depth admission and default-stack regression gates.
- [x] Finalize concrete native/Python element signatures and atomic replacement.
- [x] Implement only assigned residual stable-owner/provenance/error behaviour.
- [x] Reuse provider silent-failure fixes with exact integrated public controls.
- [x] Add existing-entrypoint tests, stubs, CI reporting and HLD contract updates.
- [x] Run scoped/risk gates and independent microscope over actual final diff.
- [x] Measure final integrated first-draft rate and record observed PR evidence.

## Open questions

Resolved by the exact fourteen-family XML matrix, bounded causal compatibility
contract, provider ownership and100-script allocation below. F-X206 owns
shared state and existing consumer migration after F-X194/F-X197. Providers
create their new handles on that one foundation, with no provider dependency
on F-X211. This story owns residual audits/XML and the final integrated corpus.
Allocation is frozen now. Actual original statement/assertion bodies and their
separate full corpus digest must be frozen in existing entrypoints before
Base measurement or production changes. Allocation digest is not a body
freeze, executed rate or successful-file evidence.

## Bounded causal-token refinement

The existing shared RevisionCounter in oxml-py-support/src/lib.rs:80 serves
Word document.rs:1508 and PowerPoint presentation.rs:154. ContentPath owns
only a numeric revision today. Word StoryItem at document.rs:487 bypasses
ContentPath and builds a stale error directly at document.rs:1619, so it
must be included. The full read-only consumer and publication inventory is
.claude/scratch/F-X211-causal-invalidation-analysis.md.

The minimal concrete candidate replaces each active counter value with an
Arc to a RevisionState containing its immutable revision and a OnceLock of
InvalidationCause. Captured handles retain the same Arc. A successful relevant
structural commit fills that retiring state's actual public operation and
transition revision once, then swaps a fresh active state. Retired states have
no next-state link and are owned only by live captured handles. This bounds
memory by live captured generations instead of total intervening mutations.
The cause is one-time publication, not a literally immutable future value.

A handle captured before A then B must retain A as its first cause. A handle
captured between them reports B. No current-operation lookup or history map
is allowed. Remove counter Copy and owner cloning. Validate bound token and
owner identity, not equal numeric counters from different documents. Preserve
ContentPath and StoryItem visible value equality by excluding hidden mutable
cause state. Native StoryItems can capture authority, while publicly constructed
snapshots cannot invent it from a supplied revision. Preserve their established
snapshot behavior and give factual unavailable-provenance guidance when needed.

Preflight successor allocation and overflow before native publication. Failed
candidate validation, expected-count mismatch and contractual no-ops retain
native bytes, token identity and cause. Pass real public entrypoint names
through shared helpers and aliases, including property setters. Existing
text-setter semantics are not changed merely because display text is equal.
Use the same relevant generation partitions as X194/X206/X202/X209. This
mechanism does not make a currently unstable owner survive by itself.

Where multiple owner tokens can invalidate one handle, accurate chronological
first-cause selection requires one existing document commit serial shared by
the affected transitions. Provider coordination must prove that serial before
claiming cross-scope first causality. Otherwise report the named owner-generation
cause without falsely promising chronological first. Actual provider partitions,
constructed-snapshot edit authority and stable physical selectors remain
approval barriers. Add existing-entrypoint tests for repeated edits, cloned
handles, Arc weak lifetime reclamation, owner survival, failure/no-op stability,
actual alias labels and cross-document/constructed snapshots. This refinement
settles the bounded mechanism candidate, not full F-X211 design approval.

## Final causal compatibility and common serial contract

The authenticated resolution memo is
/private/tmp/S91-F-X211-causal-finalization/resolution-memo.md. Its15 source
references match current canonical bytes. Preserve visible Story/StoryItem
constructors and field-based equality. Manual numeric construction remains
an explicit unbound current-target selector, never a captured owner token.
An unmatched numeric guard reports unavailable causal provenance, not an
invented first invalidating operation. Preserve existing missing-manual-story
error ordering. A manually constructed Item using a bound Story retains that
Story's originating-owner constraint without fabricating item generation.

Native captures and their clones carry hidden document/owner state tokens.
Reject foreign document identity before lookup or revision comparison. Validate
retired relevant tokens before resolving their current native location.
Visible equality excludes identity, token pointers and mutable cause state.
Removed/recreated physical owners cannot resurrect captured handles.
ContentPath::new stays const/unbound with no allocation. Genuine captures
use token-aware validation. Removing RevisionCounter Copy and adding hidden
Rust fields is an intentional pre-1.0 source change, not claimed compatibility.

Keep the current top-level revision as one monotonic invalidating-transaction
serial alongside partitioned scope generations. Capture's visible serial
is distinct from the local generation token. An unrelated owner's commit
must not make a still-live scope stale by comparing these different numbers.
The earliest relevant retired token is selected by minimum common committed
serial, never unrelated local generations or validator traversal order.
Same-transaction ties have one public label/serial and fixed scope ordering.

Each owner retains only its current Arc<RevisionState>. Live handles may
retain old states with one OnceLock first cause. No event history, next-state
pointer, revision lookup map or document ownership cycle. Preflight scalar
and local checked_add plus successor allocation before native publication.
Publish success and retire/swap exactly once without a fallible step afterward.
No-op/refusal retains bytes, all state identities, causes and serial values.

Existing-entrypoint tests cover bound/manual equality, foreign equal-valued
capture refusal, manual selectors, bound Story propagation, alias labels,
clone/removal/reuse, reversed multi-scope validator order, overflow-before-edit,
late failure and weak-state reclamation after thousands of uncaptured edits.
These settle the design's causal question, with provider implementation and
actual frozen-corpus acceptance still required once, not already passed.

## Approved XML scope and exact corpus allocation

X211 element matrix is exactly the plan's ten generic names/fourteen family cases. Keep paragraph.element.xml:str and element.replace_xml(xml:str)->None, namespace-complete serialization, concrete checked native ContentLocation/StoryRunPosition or true part-owner routes, no unsafe generic package write or lxml promise. Exactly one matching qualified root, modeled invariant/schema order and owned relationship validation, inherited namespace closure, staged serialize/reopen and same-token atomicity. Raw writes retain X198/X202 paired-owner barriers. Regular Run refuses fld/br roots. Declared limits reuse current package/text parsing caps, reject DTD/entity/network resolution and malformed root/trailing content before mutable access. Not a complete XSD validator. Identical fully equivalent owner replacement is no-op. replacing owner topology retires precisely its unsafe descendants. Existing manual snapshots remain compatible.

## Fixed allocation and evidence boundaries

Selected exact100-script allocation: Word49 and PowerPoint51. Existing examples24 (Word17/PPT7), XML14 (Word5/PPT9), lifetime16 (Word7/PPT9), provider acceptance46 (Word20/PPT26). Shape XML case includes sp/pic/grpSp/cxnSp/graphicFrame variants within one script, not extra denominator identities. Each XML script bundles positive qualified/default/alias closure and save/reopen with wrong-root/foreign/late-failure atomic checks. Lifetime scripts include wrong-owner and retired-handle actionable failures alongside required survival. These nested controls do not increase script count.

Canonical allocation JSON is UTF-8 compact json.dumps(records,sort_keys=True,separators=(",",":")), each record has id/area/case, with area equal to the uppercase second component of id (BASE/XML/LIFE/AREA). Independently derived SHA256 f6adc336ed82bcfa2a3c034e91513ef58543fb322e4de259c1a6d6d2010ebcbf. This digest freezes only allocation identities below. No full statements/assertion-body corpus digest or Base/final rate currently exists. Freeze actual original bodies, setup and saved-file assertions in existing test entrypoints before Base measurement. Retain original17/7 bodies and named adaptations. No injected re-fetches/coercions. Successful adapted gate remains separate from unmodified correct-file metric. Every attempt stays denominator, deliberate divergence counts as non-success, unexpected failure/missing identity/assertion/unreviewed divergence fails CI. Finite deliberate refusal cases have independent actionable-error tests, not fake successful-file credit.

Saved-file assertions must reopen through both binding/native relevant paths and verify semantic contents plus qualified owner/relationship/grid identities. visible contracts add deterministic rendering. XML cases verify exact unrelated opaque bytes and qualified schema facts, exports verify format/page/selection/diagnostic counts, lifetimes verify both bytes and generations. Source/build/interpreter/oracle versions and final body digest accompany Base/final rates. Final corpus runs once on integrated200..210, not provider worker copies. These unrun gates do not block design approval.

### Exact allocation IDs

- W-BASE-01: documents.opening-a-document
- W-BASE-02: documents.really-opening-a-document
- W-BASE-03: quickstart.opening-a-document
- W-BASE-04: quickstart.adding-a-paragraph
- W-BASE-05: quickstart.adding-a-table
- W-BASE-06: quickstart.table-cell-text
- W-BASE-07: quickstart.held-row-two-cell-assignment
- W-BASE-08: quickstart.table-iteration
- W-BASE-09: quickstart.table-style
- W-BASE-10: quickstart.adding-runs
- W-BASE-11: text.horizontal-alignment
- W-BASE-12: text.indentation
- W-BASE-13: text.line-spacing
- W-BASE-14: text.paragraph-spacing
- W-BASE-15: text.pagination-properties
- W-BASE-16: text.font-name-and-size
- W-BASE-17: text.font-tristate-and-underline
- P-BASE-01: hello
- P-BASE-02: bullet
- P-BASE-03: textbox
- P-BASE-04: picture
- P-BASE-05: shapes
- P-BASE-06: table
- P-BASE-07: text-extraction
- W-XML-01: paragraph
- W-XML-02: run
- W-XML-03: table
- W-XML-04: cell
- W-XML-05: section-properties
- P-XML-01: paragraph
- P-XML-02: run
- P-XML-03: table
- P-XML-04: cell
- P-XML-05: shape-kinds
- P-XML-06: text-body
- P-XML-07: slide
- P-XML-08: layout
- P-XML-09: master
- W-LIFE-01: table-held-row-after-cell-text
- W-LIFE-02: table-ancestor-after-grid-edit
- W-LIFE-03: hyperlink-after-unrelated-link-edit
- W-LIFE-04: removed-duplicate-link-no-rebind
- W-LIFE-05: manual-vs-captured-story-item
- W-LIFE-06: first-cause-multiple-edits
- W-LIFE-07: noop-failure-overflow-and-weak-retirement
- P-LIFE-01: title-subtitle-ancestor-survival
- P-LIFE-02: notes-ancestor-survival
- P-LIFE-03: layout-after-add-slide
- P-LIFE-04: master-theme-proxy-survival
- P-LIFE-05: table-after-row-column-edit
- P-LIFE-06: removed-shape-no-id-rebind
- P-LIFE-07: gradient-stop-duplicate-insert-delete
- P-LIFE-08: cross-scope-first-cause-and-foreign-owner
- P-LIFE-09: noop-failure-overflow-and-weak-retirement
- W-AREA-01: style-create-delete-kind
- W-AREA-02: style-cascade-and-language
- W-AREA-03: lists-numbering-restart
- W-AREA-04: paragraph-keep-and-tabs
- W-AREA-05: sections-orientation-margins
- W-AREA-06: header-footer-distinct-stories
- W-AREA-07: footnote-endnote-scope
- W-AREA-08: table-indent-look-style
- W-AREA-09: table-grid-merge-split
- W-AREA-10: picture-add-resize-inventory
- W-AREA-11: bookmark-heading-fragment-link
- W-AREA-12: external-link-retained-format
- W-AREA-13: content-control-edit
- W-AREA-14: math-expression-loss-report
- W-AREA-15: template-nested-conditions
- W-AREA-16: checked-validation-diagnostics
- W-AREA-17: core-app-custom-properties
- W-AREA-18: scope-text-and-format-export
- W-AREA-19: caller-font-fit-pdf-render
- W-AREA-20: paired-textbox-physical-replacement
- P-AREA-01: fresh-metadata-all-layout-geometry
- P-AREA-02: font-baseline-spacing-scripts
- P-AREA-03: numbered-bullet-details
- P-AREA-04: gradient-live-stops
- P-AREA-05: pattern-and-path-fill-pixels
- P-AREA-06: picture-fill-owner-pruning
- P-AREA-07: solid-theme-alpha-brightness
- P-AREA-08: cell-text-anchor-banding-style
- P-AREA-09: core-app-custom-sections
- P-AREA-10: movie-poster-lifecycle
- P-AREA-11: jpeg-tiff-multipage-selection
- P-AREA-12: notes-blank-default-handout
- P-AREA-13: pdfa-and-odp-loss-reports
- P-AREA-14: alt-title-decorative-qualification
- P-AREA-15: flip-rotate-native-order
- P-AREA-16: group-ungroup-id-zorder
- P-AREA-17: nested-nonuniform-affine-refusal
- P-AREA-18: glue-move-delete-regroup
- P-AREA-19: issue323-connector-scaled-group
- P-AREA-20: picture-placeholder-identity-crop
- P-AREA-21: run-internal-link-highlight-smallcaps
- P-AREA-22: align-visual-selection-slide
- P-AREA-23: distribute-gap-ties-cardinality
- P-AREA-24: footer-title-date-first-zero
- P-AREA-25: master-layout-theme-apply
- P-AREA-26: all-transitions-retained-sound

## Full statement freeze, independent static checkpoint

The100-script pre-measurement review candidate is retained under
/private/tmp/S91-F-X211-frozen-corpus. Allocation remains exactly the approved
49 Word/51 PowerPoint IDs. Full manifest SHA256 is
1889f6683eb788fc2a9007e84c66028cf5f1701a22a452de8767edbb0c8042be, and
complete setup/body/assertion JSON SHA256 is
10e1c6dc3d7bf0ca02ac185d54fa9fbfd01fc9488e8a82885fae4f99861ff106.
This supersedes the earlier absence of a full statement digest. Independent
root checks authenticate156 artifact hashes/sizes,100 syntax-valid bodies,
seven pinned upstream PPT code blocks and three original read-in-place font
or media hashes. No binding execution, selected source-built Base, native
obligation acceptance or rate is asserted.

The17 Word originals remain exact local oracle bodies. The seven PPT originals
are literal code blocks from python-pptx1.0.2 commit
278b47b1dedd5b46ee84c286e77cdfb0bf4594be, blob
a24b8bc63ca6fefaa7df8f4ba0c6cff1f4dc797d. Original indented bytes and line
citations remain retained. Executable bodies dedent, remove trailing blank
lines and substitute import namespaces only. Seeded extraction input and
controlled PNG are documented fixture exceptions. Original reconstructed or
refetched adaptations remain separate and cannot earn original-body credit.

Causal tests explicitly assert the first invalidating operation and exclude
the later different operation. Stress edits do not prove weak-state memory
reclamation. Twelve cases require separate concrete native acceptance. The
movie case remains in the fixed denominator with qualification incomplete:
its real installed MP4 has container/sample facts, but actual frame decode
failed. Do not admit playable/movie-render credit or a complete corpus metric
until that input has authentic successful qualification. No further font copy,
media distribution or production fixture file is authorized by this record.
Final body freeze/admission and unchanged-hash Base/final execution remain
required in existing integration entrypoints, once over the integrated result.

## Premeasurement coverage revision required

Independent root body review found no explicit fit_text/refresh_autofit or
Georgia/Segoe UI/Aptos/Symbol/Wingdings operations in the first frozen100
statements. Its static100/100 result is not complete303..313 coverage.
Before any Base execution, amend only W-AREA-19, P-AREA-02, P-AREA-03 and
P-AREA-11 within their existing identities. Add explicit X207 fit/refresh,
X210 actual selected-face and same-pass diagnostic assertions, source-
preserving qualified SymbolF0B7/U2022 and WingdingsF0A7/U25AA controls.
Keep original24 example bodies, all100 IDs and the exact allocation digest.
Freeze a separately identified revision with old/new body/support hashes.
Never edit specimens after observing a production execution to raise the rate.

Use X207's concrete owned result schema above and X210's approved additive
render diagnostics. The bounded H457 explicit Arial28 seven-line150-percent
control expects92500/0 from actual shared Q/S measurement. Its exact
geometry/font/style operands must match the retained native input. General
solver parity remains a native provider gate. Meaningful positive XML edits
and preserved unrelated opaque/source siblings replace alias-only success
through the existing14 XML identities. Native relationship/identity/lifetime/
late-failure riders remain required, not implied by a scalar edit.

PDF inspection can use the already installed pinned Poppler text/font output
as local test evidence without adding a production dependency. Actual font
selection must bind font data or emitted embedded faces, not merely search
PDF bytes for a family string or count nonblank pixels. Missing selected-face
inspection is a recorded acceptance gap. Movie frame qualification remains
unresolved. Root reviews the revised exact manifest before unchanged-hash
Base/final source-built execution and scoring.

## Revised premeasurement statement freeze

Revision2 is retained in /private/tmp/S91-F-X211-frozen-corpus-r2. It
supersedes the first candidate for future measurement while retaining that
original evidence. Exact manifest SHA256 is
7501e001301a703988526f76c9961b35b345a1c17f8e1d383e9477d5947b22e1,
and complete setup/body/assertion SHA256 is
2f4348dd55dcd1b8889badd8ca407d9f790cf38327c00ec3ae31dd5b02b3f66e.
Independent root checks authenticate161 artifact hashes,100 syntax-valid
scripts,24 byte-identical original examples and exactly18 amended identities
(the14 XML cases and four planned AREA cases). Allocation remains unchanged.
No binding body ran. There is no Base or final rate.

The authentic H457 frame input preserves the native txBody qualified XML
attributes/children except saved fontScale92500, serialized with namespace
aliases. Root independently compared parsed XML to the authenticated slide,
verifying that sole semantic change, exact geometry and expected92500/0.
This is not original lexical-byte identity. XML positives now perform
meaningful text/name/margin writes while checking retained opaque attributes,
two children, sibling text span counts, owner relationships and unaffected
parts. Master/slide source-name checks do not pretend a public typed name
getter exists. Full source-span/relationship/lifetime/late-failure native
gates remain required beyond these observations.

The four AREA revisions use explicit fit/refresh and owned diagnostics, plus
Georgia/Segoe UI/Aptos and qualified Symbol/Wingdings controls. Pinned local
pdffonts26.09.0 records embedded face inventory. It does not prove which
resource rendered each string, glyph outlines or caller-symbol precedence.
Those exact native provider obligations remain explicit, as does the failed
movie decoded-frame qualification. Neither static checks nor inventory-only
font names may become full-acceptance credit.

## Source-built runner provenance preparation

Root read the full revised runner at
/private/tmp/S91-F-X211-corpus-runner-prep-r2/runner.py, SHA256
47c5c868f47dee23e37f0004d136b62c97a4d5592c87088c1e5d855dab297f6c.
All six preparation artifacts authenticate and AST parsing passes. This is
runner preparation only. No specimen, binding import, build or installation
ran, and neither Base nor final source/environment is selected yet.

Before execution, bind both families to genuinely observed source-built
wheels from one selected exact commit. Authenticate the Git commit/tree,
complete captured crate/Cargo/toolchain/config membership and each source
blob before and after the observed build. Retain exact invocation, source
root, true exits and linked build/install log paths and hashes. Compare
installed package and extension bytes with the measured wheel. A version
number, old normalized receipt or cryptographic correspondence alone cannot
prove compiler input provenance. The earlier candidate314-file inventory is
not a selected Base and must not be relabeled after199 integration.

The bounded QuickTime selection attempt was interrupted before opening the
movie. Refreshed surfaces showed no owned movie window/tab. No decoded
frames were observed, and the UI lease is released. The retained receipt
/private/tmp/S91-F-X211-native-movie-playback/attempt-receipt.json has SHA256
b23993e3965079f5a89503dd0d4058303604879bb0aaa7c5fc8c7d0697e0f05b.
This tool selection failure is not codec refusal or playback qualification.
Preserve the separate failed AVAssetReader result. P-AREA-10 admission and
additional native obligations still prevent a complete acceptance metric.
R2 corpus bodies and allocation remain unchanged.

## Selected preprovider Base and whole-tree runner

The selected Base is the clean post-F-X199 integrated commit
`d44304c7a9223bf34f4e6f8e99ec8d20659aaf7e`, root tree
`86be028ad3f6e2745c35bc08a4d1c92652be355b`. Keep this selection for both
families and final comparison. The immutable source capsule under
/private/tmp/S91-F-X211-Base-d44304c7a922 authenticates2679 tracked blobs
and87452686 bytes, including root licenses and packaging inputs.

Runner R3 is /private/tmp/S91-F-X211-corpus-runner-prep-r3/runner.py, SHA256
194a3b395474aa00b54dd86981a4f657ed3bb6172dee1f14a10549068eeb5561.
Its root-reviewed guard requires exact whole tracked-tree membership,
Git modes/blob identities and actual bytes, rejects extra untracked inputs
and keeps target/log/wheel outputs outside the source capsule. It supersedes
the earlier crates-only source guard. Frozen corpus revision2 is unchanged.

The root-observed serial debug wheel build has started against that exact
capsule with authenticated maturin1.15.0, CPython3.12.13 and the official
retained Pillow12.3.0 wheel. Complete build/install exits, unchanged before/
after inputs, installed-wheel correspondence and runner admission are still
required before specimen execution. No Base rate, final rate or complete
native/movie acceptance is asserted by source capture or build startup.

## Observed frozen Base measurement

The original build attempt ended with actual exit1 after the idle Word
compiler was explicitly interrupted. Its failed execution and unchanged
source inventories remain under
/private/tmp/S91-F-X211-Base-d44304c7a922. Recovery-r1 reused the same
immutable selected source, tools, debug profile, locked offline dependency
inputs and shared target. All12 recovery commands ended with actual exit0,
including both wheel builds and installations. The original failure remains
distinct. The independent artifact authentication receipt is
/private/tmp/S91-F-X211-Base-recovery-helper/artifact-authentication.json.

Root admission completed with actual exit0. Both isolated native imports
and whole-tree, wheel, installed-byte and external-input checks passed.
Its separate approved config is under
/private/tmp/S91-F-X211-Base-admission-r1, SHA256
5a0651dc689241cc2ac83de985e48595edd964aae4ef5dd50acce7b75e5216fe.
The original recovery config remains unapproved and unchanged.

The unchanged revision2 corpus ran all100 scripts against selected Base
with no timeouts. Positive script assertions passed17 of100. The24 BASE
scripts passed15, the14 XML scripts passed0, the16 LIFE scripts passed1
and the46 AREA scripts passed1. The runner ended with actual exit1 for
83 failed scripts after its final source and installed-input checks.
All per-case traces and produced artifacts remain under
/private/tmp/S91-F-X211-Base-corpus-r1. Terminal evidence is
/private/tmp/S91-F-X211-Base-corpus-terminal-receipt.json.

These are Base observations with the fixed denominator100. Do not repair
the frozen scripts, retry failures into pass credit or attribute later
unexecuted operations as observed failures. Native rendering obligations
and decoded-movie qualification remain pending. The full acceptance metric
is unset, and no final source measurement or90-percent claim is made.

## Issue331 shared native parser admission

[Issue331](https://github.com/tensorbee/rdocx/issues/331) is assigned once
to this story. It strengthens the same native admission used by document
open and validated raw XML replacement. Do not add separate CLI or Python
parser fixes. The reported5000-level crash is contributor evidence, not a
local reproduction. Read-only source assessment is retained at
/private/tmp/S91-issue331-readonly-assessment.md.

Current inline admission calls CT_Sdt from text.rs:8115. The recursive
cycle runs through content_control.rs:1010/:1061/:1427. The .ok() at:1011
and RawXml fallback at:1431 erase typed parser errors. Body admission also
falls back at document.rs:2760. A limit inside that optional branch alone
cannot satisfy a recoverable document-open refusal. Guard before recursive
construction at shared Result-bearing native admission boundaries. Retain
ordinary unsupported opaque XML fallback, but never swallow a resource
limit refusal into raw preservation.

Select an inclusive limit of256 simultaneously nested namespace-qualified
Word sdt controls, counted across paragraph, table/cell and control-owner
transitions. A sibling starts after its predecessor closes, rather than
consuming a cumulative document quota. Namespace aliases and defaults have
the same count. Foreign or rebound lookalikes remain opaque and must not
be mistaken for modeled Word controls. Existing table and revision limits
remain in force, with no budget reset that lets alternating modeled owners
bypass admission. Share the bounded native check in existing files and
return the existing OxmlError::InvalidValue with an actionable message
identifying content-control nesting and limit256. No new module, crate,
dependency or integration binary is required.

Cover every existing checked Word document and paragraph/control parse
entry, including body, table/cell, header/footer and note/comment parts,
plus native parsing used by element replacement. No binding-only guard,
larger consumer thread stack or panic catching substitutes for the native
check. Exact256 must be safe for normal-stack parse, accepted-tree traversal,
clone, serialization and drop. If recursive implementation cannot prove
that bound, make the affected traversal iterative inside existing files
rather than lowering the accepted boundary silently.

Add independent existing-entrypoint regressions for256 accepted,257 refused
and5000 refused. Open crafted packages through native Document::open and
ordinary CLI/Python subprocesses, requiring a controlled error or Python
exception instead of interpreter abort. Check named limits, namespace
alias/default equality, foreign/rebound preservation, sibling reset and
alternating control/table ownership. Replacement refusal preserves owner
XML, relationships, package bytes and generations. Run normal consumer
stack controls in addition to the declared CI stack configuration.

These are dedicated Issue331 regressions, not extra frozen corpus records.
The observed100-script Base bodies, digest, denominator and result remain
unchanged. Land the intentional over-limit parser refusal in a separate
labelled commit. Ordinary valid deterministic outputs are expected unchanged
and still require the hash harness. No source implementation or passing
Issue331 regression is claimed by this amendment.

## PR333 immutable contribution reconciliation

[PR333](https://github.com/tensorbee/rdocx/pull/333), immutable head
8636af0949b192665490659041b7541ae5ef4090, maps once to Issue316/F-X211.
All163 complete changed-source blobs and patch paths are authenticated
in /private/tmp/S91-PR333-intake. Static assessment is retained there.
No contributed gate or source is integrated by intake.

Keep the existing frozen100 bodies, digest and observed17/100 Base result.
The contribution's120 records and mutable needs labels are supplemental
examples, not replacement acceptance or a projected success rate. Implement
the approved element.xml string view and complete fourteen-owner matrix,
preserving existing byte snapshots. Qualified URI/sequence/opaque bytes,
physical owner/relationship closure and staged package reopen are required.
Local-name roots or element counts do not replace those invariants.

Reuse useful one-root/DTD/CDATA/vocabulary/actual-part relationship checks
selectively in existing approved files. The125 contributed new files and
two raw_xml modules are not automatically authorized or needed. Consume
Issue331's common native bound and the actual X194/X202/X206/X209 owner
providers. Do not add the contributor's unbounded per-edit BTreeMap replay
next to the bounded causal foundation. Same-value replacement/no-op keeps
bytes and tokens, and real topology changes retire only unsafe descendants.
Known formatting, sections, notes and scalar-validation changes retain
their provider owners. Reconcile legitimate subquantum Length values and
minimum signed bounds rather than copying i64::abs or blanket rejection.
No duplicate implementation, corpus mutation or new material question is
introduced by this source assessment.

## PR336 rolling intake, 2026-10-10

PR [336](https://github.com/tensorbee/rdocx/pull/336), immutable head
`c6ab986008782e8a376a9cdd0bda2afd4847cf47`, contributes to Issue331 bounded native parsing.
The same contribution also contains the other story's slice, owned by
F-X200. Adopt accepted changes once per owner.
Complete source and discussion assessment is pending. Its proposed extra
inline-wrapper and cross-control nesting policy is not automatically approved
by intake. Contributor gates and hash expectations are not local evidence.
The approved contract and completion prerequisites remain authoritative.

### Completed immutable PR336 assessment

Result-returning control admission and staged nested-reader parsing are
reusable ingredients. The contribution counts matching local names across
all namespaces, contrary to approved qualified-only admission and exact
foreign/rebound preservation. Keep the approved inclusive256 qualified-control
limit. Do not adopt the additional64 inline-wrapper refusal from this patch.
Normal-stack boundary lifecycle, all story owners, sibling reset, namespace
controls and atomic XML replacement remain required. Any additional wrapper
resource policy needs separate evidence and a reviewed design amendment.

Complete assessment: `/private/tmp/S91-PR336-c6ab986-intake/assessment.md`.
All fourteen changed paths and twenty-eight base/head Git blobs were captured
and independently authenticated. No contributor gates were executed or
accepted as local completion evidence. No new module or F-ID is required.

### PR336 updated-head intake

The 2026-10-10 07:27:15 UTC refresh observed head
`935ac165afcd29d34454568ba3b2287a4bac52dc`, replacing the retained assessed
c6ab986 head. Complete incremental source assessment is pending. The new
body claims64 controls,64 inline wrappers,512-element revision refusal and
additional relative-REF numbering changes. These are contribution proposals,
not automatically selected resource limits or locally executed gates. Preserve
the approved contract until actual changed-source assessment is reconciled.
The prior immutable assessment remains historical evidence, not approval of
this new head. Issue331 and321 ownership is unchanged.

## Qualified hyperlink attribute silent-failure intake

F-X194's installed grouped-cell regression exposed a pre-existing shared
reader gap. `crates/oxml-drawing/src/text/paragraph.rs:500` extracts literal
r:id, while a valid alternate relationship prefix survives raw storage but
leaves the modeled relationship_id absent. The binding then returns None at
`crates/rpptx-py/src/text.rs:1044`. Canonical p/a/r input resolves the same
relationship successfully. F-X194 retains the canonical imported-owner
regression. This uncovered alias behavior belongs to this story's shared
silent-failure and qualified XML contract, not a second cell-frame API.

Preserve exact namespace URI ownership for inherited and locally declared
relationship aliases, canonical and default element namespaces, rebound and
foreign same-local-name attributes. Do not replace r:id with an unqualified
local-name guess. Retain raw unknown attributes and avoid duplicate expanded
attribute names during save/reopen. Prove hyperlink reads for ordinary and
grouped shape/cell owners, correct slide relationship context, and unchanged
package bytes on reads. Fix the shared reader once in existing files. The
existing `crates/oxml-drawing/src/text/paragraph.rs` is an additional concrete
source reservation under this owner, with shared parser/publication riders.

Exact original and canonical fixture recipes and XML hashes are retained in
`/private/tmp/S91-F-X194-evidence/grouped-fixture-cause/receipt.json`. The
original valid-alias failure remains unresolved evidence, not a passed gate.
The fixture correction does not claim that aliases have been repaired.

### Completed updated-head source assessment

Incremental assessment of immutable935ac165afcd29d34454568ba3b2287a4bac52dc
is complete, superseding the pending checkpoint above. All sixteen changed
full blobs, five reused blobs and seven missing prior versions were
authenticated by Git identity, size and SHA256. Source and discussion
evidence is retained at `/private/tmp/S91-PR336-935ac16-intake/receipt.json`
and its `assessment.md`. No contributor gate is local completion evidence.

Related-story preflight and staged parsing are reusable ingredients, including
comments, footnotes/endnotes, headers/footers and glossary relationships.
Preserve inclusive256 qualified-control admission, exact foreign/rebound
content and the normal-stack lifecycle. Do not inherit proposed64 controls,
64 inline wrappers or512 total XML-element revision depth. The new revision
guard runs after transaction staging and does not establish staging safety.

The updated numbering state still lacks selected per-level override
consumption/rearm and deferred section initialization with retained
lvlRestart0 counts. Preserve native P/Q/R/T/U acceptance. The new
abstract_num_id and relative_to changes affect numbered REF and belong to
F-283's actual native discriminator, not marker-only inference. Preserve
concrete numId identity and account for published struct-literal compatibility
before adopting a new public field. Reuse accepted slices once per owner.

## Typed text-box recursion safety reconciliation

Issue331's native admission and staged replacement contract also reaches
bare typed text boxes. The authenticated existing path is run text.rs:2627,
drawing.rs:703-708 and text_box_body:1374-1389, which reparses CT_Body and
deep-clones paragraphs while retaining the complete body. This establishes
recursive duplication risk, not a locally measured crash or a proven limit.
Exact source and issue evidence is retained at
`/private/tmp/S91-PR336-935ac16-intake/acceptance-reconciliation/receipt.json`.

The worker must remove unsafe recursive admission and unbounded duplicate
projection cost in the existing text/drawing/body parser files. Prefer a
qualified iterative path preserving admitted semantics. If a separate drawing
resource refusal is necessary, first measure a bounded source-built normal
consumer-stack probe, then record and independently review its exact boundary,
accepted lifecycle and atomic refusal in this plan before production edits.
No arbitrary drawing quota is selected now. Preserve inclusive256 qualified
sdt admission and existing table/revision limits across mixed owner paths.

Run linear qualified admission before recursive capture or cloning at every
checked native story and independent paragraph-fragment entry used by staged
replacement. AlternateContent's drawing.rs:370 .ok() must not conceal a
resource refusal as successful opaque fallback. Foreign/rebound lookalikes
remain preserved. No binding-only guard or enlarged consumer stack suffices.

Add bare nested typed textbox open and CLI/Python subprocess controls, mixed
textbox/control paths without budget reset, and accepted finite-depth parse,
text traversal, clone, save/reopen and drop. Staged paragraph/run replacement
failure preserves exact owner XML, relationships, package and generations.
A foreign/rebound textbox preservation control remains required. Use existing
unit modules, native regression carrier and Python test_core.py. These are
dedicated admission controls, not extra frozen compatibility corpus records.
Do not claim universal parser safety from the content-control guard alone.

## Issue337 rolling intake

Issue [337](https://github.com/tensorbee/rdocx/issues/337), captured at
2026-10-10 08:47:21 UTC, belongs to the existing typed text-box safety
contract above. It reports exponential nested projection and normal-stack
abort in bare and alternating control/text-box paths. These are contributor
measurements, not local completion evidence. No duplicate F-ID is created.

Freeze source-built 2000-level bare text-box and mixed control/text-box
subprocess controls before implementation. Both native open and installed
CLI/Python consumers must return an actionable resource error without abort.
Measure accepted boundary open and extraction in a release build, retaining
actual timing, input size and consumer stack. At the reviewed boundary the
reported target is under one second. Show linear retained representation and
traversal cost rather than timing alone. Preserve the independently reviewed
qualified256 content-control boundary. The exact text-box or combined
resource boundary still follows the measured and reviewed selection above,
with admission before recursive capture, staging or clone.

The updated PR336 body has the same immutable source head and links this
report. Reuse the authenticated source once. Full issue/body evidence and
actual read-only command receipts are retained at
`/private/tmp/S91-rolling-refresh-20261010T084721Z`. Assessment is not acceptance.

## PR333 advanced-head reconciliation, 2026-10-10

Immutable head `b228d5dc650c9aff669fa78ebc5932b7d32159e3` replaces
`8636af0949b192665490659041b7541ae5ef4090`. Root authenticated the manifest
and all169 source records by Git blob identity, bytes and SHA-256 and inspected
the exact six-path delta. Assessment and receipt are retained in
`/private/tmp/S91-PR333-b228d5d-intake`. Issue316 remains F-X211, with each
provider assigned once to its existing story.

Reuse finite/type checks and qualified parent-child negative fixtures, with
validation before publication. Positive-only line spacing needs compatibility
classification and storage-quantum controls because tiny positive multiples
can round to zero. Preserve imported values and canonical truncation. Broader
child vocabulary membership does not prove cardinality, sequence or choice.
Check actual owner context, including geometry versus gradient a:path, with
positive saved/reopened edits and duplicate, order, choice and context refusal
controls. Qualified element counts do not prove opaque attribute, text and
order preservation. Keep exact bytes, relationships and causal token gates.

No new contributed raw_xml module is authorized. The ten-name/fourteen-case
matrix, inclusive256 control-depth policy and Issue337 bounded text-box
acceptance remain. Contributor archive tuples must be replaced by measured
local values, and contributor tests or CI provide no local acceptance credit.

## PR338 rolling intake for Issue337

[PR338](https://github.com/tensorbee/rdocx/pull/338), immutable initial head
`c189dbd73b9a61543ed64a03d2305d0017bbf498`, closes existing Issue337 and
remains F-X211. The09:35:24UTC root-authenticated refresh captures its body
and stacked-on-PR336 ancestry. Complete source/discussion capture and independent assessment are
authenticated, with no implementation acceptance. Root checked all53 source
records against Git blob identity, decoded bytes, SHA-256, modes and complete
trees, and inspected the exact nine-path delta from PR336.
Its proposed one-body ownership, namespace lookup ordering and measured
resource bounds are useful evidence to assess against the existing contract.

Do not copy public CT_Shape.text behavior changes,16-text-box/64-mixed limits
or weight4 by intake. Preserve approved qualified256 control admission until
actual independent resource measurements and compatibility review justify
the final text-box and combined bounds. Reconcile the two new commits once
against the existing PR336 provider work. Contributor CI, timing and stack
claims are foreign evidence, not local completion proof. The reported missing
block-control text-box discovery and drawing-local namespace story extraction
need distinguishing controls under the full existing Issue337 owner contract.

PR333 also advanced to `193819ca0a1dcf3415017c0be1186fe8df6bcb86`. Its
previous authenticated assessment remains retained. Root authenticated all202 source records, full ancestry and complete trees.
The incremental change is one HLD10 prose punctuation correction, with no
production, test, carrier or API delta. Reuse the prior b228d5d assessment
without repeating implementation. No new F-ID.

## PR338 independent source assessment, 2026-10-10

Root authenticated both immutable source capsules and inspected the exact
incremental diffs. Receipts and assessments are retained in
`/private/tmp/S91-PR338-c189dbd-intake` and
`/private/tmp/S91-PR333-193819c-intake`. Independent authentication is recorded
in the09:35:24UTC refresh directory. Issue337 remains owned once by F-X211.

Adapt one authoritative text-box body and cheap qualified eligibility before
namespace scans, while preserving the public CT_Shape.text contract. The
contributed boxed iterator adds indirection without a second implementation.
Avoid it. Repeated raw subtree capture followed by recursive reparsing still
needs linear retained-memory and traversal proof. Qualified admission must
precede capture and recursion, not run only after a fragment is allocated.

The proposed local-name quota counts foreign or rebound elements. Retain the
approved inclusive256 qualified control boundary and independently measured
text-box and combined budget. Cover block controls containing text boxes and
drawing-local namespace declarations under the existing full story contract.
An in-process2000-depth test and contributor release timing do not replace
bounded native, installed CLI and Python subprocess refusal controls, ordinary
consumer stacks and accepted-boundary release measurements. Archive tuples
and CI are foreign evidence, not local verification.


## Reviewed native recursion resource boundary

Independent native measurements and resource-boundary pass1 select an inclusive
weighted path budget of256. Each namespace-qualified modeled content control
costs1. Each actual typed Word text-box body costs16. Costs accumulate only
along the current owner path and reset when that owner closes. Checked
arithmetic refuses overflow. Pure256 controls remains accepted. The measured
normal2MiB lifecycle corners are16 typed boxes,240 controls plus1 box,
128 controls plus8 boxes, and12 boxes each containing a control. These are
independent local measurements, not adoption of the contributor's quotas.

The deciding measurements are retained in the worker receipt
`.claude/scratch/F-X211-start/actual-native-resource-measurements-13/receipt.json`,
SHA256 `74ebf84393deba32d4e9187b6671efb737299db17c14310186ed853737506d97`.
Independent review is `.claude/reviews/F-X211-resource-boundary-pass-1.md`.
Its inline-owner defect is remediated separately before policy implementation.
Inline shape text remains opaque, including qualified-looking descendants.
The current anchor parser constructs qualified Word txbxContent even when
its graphicData URI is unknown. Admission follows that actual typed behavior
without expanding or narrowing parser ownership. Foreign and rebound opaque
content retains its exact source bytes and consumes no typed-owner cost.

One linear source preflight must refuse unsupported paths before recursive
construction or repeated subtree capture. An iterative borrowed-model guard
must refuse before recursive serialization and facade staging clones, with
no thread-local admission authority or larger consumer stack. Refusal returns
an actionable ordinary resource error and leaves source, relationships and
owner generations unchanged. Related-story, VML, mixed-table/control,
normal-stack installed CLI/Python and frozen100 acceptance remain required.
The corner measurements do not waive those remaining gates.

## Delivery children and retained parent acceptance

The oversized implementation is partitioned into F-X211a for Word checked XML
and Issues331/337, F-X211b for PowerPoint checked XML and DrawingML aliases,
and F-X211c for final shared errors, lifetime and immutable100 compatibility.
Each child has its own design, scoped gate, independent review and handoff.
The parent closes only after all children close, with the entire acceptance
above unchanged. The frozen100 and historical Base17/100 remain authoritative.

F-X211a inherits the exact frozen parent Word source and evidence through a
root-authenticated transfer. The original parent worktree stays inactive and
retained. F-X211b uses an isolated claimed worktree. F-X211c starts only after
both children and its declared provider completion barriers. Family-local
owner retirement belongs to the corresponding child. Shared token, parser,
HLD, archive and baseline changes have one explicitly assigned writer.


## Reviewed immutable Run specimen conflict

Independent F-X211-corpus-contract-pass-1 authenticates a contradictory
expectation between unchanged W-XML-02 and W-LIFE-06. Both capture a regular
Run and append another paragraph. XML02 then edits the held Run, while LIFE06
requires that append to be its first invalidating call. The later table append
in LIFE06 cannot retroactively manufacture an earlier first cause. There is
no legitimate physical-owner distinction or caller-sensitive policy that
satisfies both expectations.

Root adopts the existing approved deliberate-divergence route. Keep both
original bodies, support,100 identities, denominator and recorded digests
unchanged. Retain conservative Run retirement on Document.add_paragraph and
its actual first cause. W-XML-02 is a reviewed deliberate non-success if that
setup refuses the retired Run, never a successful-file credit or a removed
denominator entry. Childc owns the final classification and per-area metric.
Its expected actionable-error control must verify the actual append cause and
unchanged bytes, tokens and saved source after the refused stale edit.

All fourteen XML owner kinds remain mandatory positive, qualified-refusal,
opaque-source and saved/reopened acceptance. Childa must independently qualify
current regular Run element XML through the installed public surface in an
existing test entrypoint. This independent current-owner control does not
replace, refetch or adapt the frozen W-XML-02 specimen. No specimen is edited
after observing execution, and no handle is revived by value, index, XML
equality or the subsequent call being an XML operation.


## Failed candidate corrective partition, 2026-10-14

The actual first final candidate executed all100 unchanged scripts with72
assertion successes and28 failures. It does not meet the original90-percent
complete-correct-file acceptance, and no final metric is credited. Preserve
this complete failed candidate, bodies, support, denominator and artifacts.
F-X211d owns the exposed Word owner/alias/style repairs and receives c's four
reviewed Word carriers once. F-X211c retains PPT bindings, shared causal
messages, CI/reporting and final composed-source qualification. The parent
now closes only after a, b, c and d are complete. This partition adds no issue,
PR or top-level sprint story and weakens no acceptance.

After genuine repairs, one complete unchanged100 run on the freshly admitted
new source is ordinary regression qualification. Never compute a best-of
retry rate or stitch individual successes from different candidates. Preserve
each complete candidate separately with source identity and honest outcome.
Contract conflicts require independent disposition and remain non-success
without script, support, input, expected-value or denominator changes.


## Reviewed frozen render and owner-path specimen conflicts

Independent F-X211c-frozen-render-assumptions-pass-1, SHA256
`6e8317b7f604a4c466215e7559a3d3669ff7b0ff93259351eb2b073ea00db9c5`,
authenticates two further incompatible specimen assumptions. W-AREA-19
requires Gelasio although the approved normal Word additional-font operation
retains host discovery and the actual PDF contains the Georgia font program.
P-AREA-02 filters a relative ZIP member spelling although the approved native
owner identity is the absolute `/ppt/slides/slide1.xml`. Independent current
H457 computation returned the expected92500 font scale and0 line reduction.
Neither finding authorizes font coercion or a native owner-identity change.

Adopt the existing deliberate-divergence route for these two named cases.
Preserve their original bodies, support, inputs, expected values, digests and
the100 denominator. They remain non-successes with no successful-file credit.
Subsequent unreached assertions remain unqualified. Authenticate each
disposition with its case-specific actual source, failure and independent
proof. Reject arbitrary failure reclassification and altered evidence. Keep
the first failed100 candidate unchanged rather than rescoring it.

P-AREA-11 remains a real owned-font diagnostic defect until remediation and
independent qualification, including stable first-use request deduplication
and explicit actual-face approximation. The corrected source still requires
one complete unchanged100 attempt, the original at-least90-percent complete
correct-file gate and every independent XML, lifetime, native and render gate.


## Reviewed frozen alt-text owner assertion

Independent F-X211c-frozen-alt-owner-pass-1, SHA256
`bb4a8ef797ae64b3f7dd0e3d61473c0dad82803bde555fc9f0eabed88b87524d`,
authenticates a later invalid owner selector in unchanged P-AREA-14. The first
actual100 attempt failed at the missing Shape.alt_text facade. That original
provider defect and failed candidate remain unchanged and receive no credit.

After the real facade repair, the exact current candidate retained the
authored shape2 descr, title and decorative state through saved source and
reopen. The frozen assertion instead selects the first descendant cNvPr,
which is the slide-tree owner1. Do not put shape metadata on that owner to
satisfy the assertion, and do not rewrite the frozen selector. If the new
source reaches this incompatible assertion, classify this exact evidenced
conflict as deliberate non-success. Preserve the100 denominator and no
successful-file credit. Subsequent unreached frozen assertions remain
unqualified. Independent actual shape-specific saved/reopened alt metadata
controls remain required, as do every original hard owner and native gate.
