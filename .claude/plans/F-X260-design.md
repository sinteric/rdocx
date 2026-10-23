# F-X260, Restore live handles and bound structural edit cost

**Status**: completed
**Sprint**: S95.3
**Size**: L
**Estimate**: 6 to 10 developer-days, provisional after Issue398. Original scope was 4 to 6.
**Depends on**: F-X211c, F-X237

## Problem

Issue 388 is a live-owner regression on main `7496a81c`. Setting Word cell shading, width, vertical alignment, borders, margins, column width or grid widths drops the cell's paragraph and run owners. A fresh `cell.paragraphs[0].runs` then raises `ValueError: element has no current checked physical owner`. The same failure happens through a handle taken before the edit, including a nested table. On the PowerPoint side, a second `Paragraph.add_run` or `Paragraph.add_field` on one handle retires that handle, and `TextFrame.add_paragraph` retires earlier run handles.

The current Python publication writes the edited document back without moving owner addresses. Creating `w:tcPr` shifts the raw child index of every item in that cell. `PyDocument::adopt_property_edit` does not exist on this branch. PowerPoint append proof currently drops every paragraph and run owner whose path starts at the appended owner. That code is `appended_text_owners` in `crates/rpptx-py/src/presentation.rs`.

Issue 389 is separate. On a release build, seven operations cost more as the unrelated package grows, and the reporter fails when the large package is more than three times the small one. The operations are `add_textbox`, alt text, `left`, `fill.solid`, PowerPoint `replace_xml`, Word `replace_xml` and Word `remove_content`. The same report asks for one call that removes a range of body blocks, the counterpart of `insert_document`. There is no such method today. `Document::remove_content` removes one direct body child.

PR393 head `beb9c448d0d99636a799d95f718acccb9ea55646` is unchanged by the 2026-10-18 refresh. It is a candidate, not accepted code. Its Word half relocates owners when the candidate inventory pairs one to one with the current owners and otherwise keeps the conservative publication. Its PowerPoint half keeps earlier paragraph and run owners and marks only the appended owner's text views changed. The PR body says the Word repair still walks the whole document on each property setter. That walk is not acceptance for Issue 389.

Integrated sprint source includes F-X259 at `4879e58a`. The claim base is `c776041c`. F-X259 does not edit the Python owner files this story owns. It does edit `crates/rdocx/src/document.rs` and `crates/rpptx/src/lib.rs`, and that diff is already an ancestor of this claim.

## Spec reference

- `docs/hld/03-architecture.md`, Owned publication and causal paths.
- `docs/hld/10-bindings-spec.md`, Python handle lifetime and checked XML mutation.
- `docs/hld/12-testing-strategy.md`, Binding tests and Integrated Python compatibility qualification.
- `docs/hld/14-development-backlog.md`, F-X260.

## Approach

Reimplement the accepted owner repair on the integrated sprint source. Do not merge PR393.

For Word property edits, stage the existing table, row and cell setter on a candidate. If every current owner relocates to exactly one candidate address with the same story, item kind, path depth and cell coordinates, rewrite those addresses and keep the handles. Any other inventory change, including a real insert, removal or replacement, keeps today's conservative retirement. Fresh handles taken after the edit must see the same runs. This covers the setters named by Issue 388 and the related setters PR393 lists: shading, width, vertical alignment, borders, margins, column width, grid widths, vertical merge and grid span.

For PowerPoint append, `add_run`, `add_field` and `add_paragraph` keep earlier owners. Only the appended-to owner's cached text views retire. An already-materialised paragraph collection keeps its earlier members and is not silently refreshed. A stale handle that was already invalid stays invalid. Word's documented run retirement is not changed to match this rule.

For cost, profile the seven named operations in a release build before choosing a bounded path. The target is affected-owner work using the current publication and OPC transaction. Do not add a second owner registry. Do not skip validation, namespace custody or stale-handle checks to win the ratio. A deterministic visit or allocation observation must fail if the implementation still scans unrelated slides or body blocks.

Range removal is one half-open direct-body operation on the existing fragment transaction. `Document::remove_content_range(start: usize, end: usize) -> Result<usize>` and Python `Document.remove_content_range(start, end) -> int`. An empty range returns 0. A reversed or out-of-bounds range returns an error and does not publish. The call does not recurse into tables. Relationship cleanup follows the same owned-unreachable rule F-X261 defines for pictures, so the two stories must not invent two pruning policies. This story calls the policy. It does not own style or font preservation.

No new crate, module or file. Python changes stay in `crates/rdocx-py/src/document.rs`, `crates/rdocx-py/src/table.rs` and `crates/rpptx-py/src/presentation.rs`. The native range method lands on `Document` in the existing `document.rs`.

## Consumer map

| Rule | Entry points and outputs | Shared provider and caller policy | Lifecycle stages compared |
|---|---|---|---|
| Non-structural Word cell edits keep owners | Python cell, row and table property setters. Fresh and held paragraph and run handles | `PyDocument` owner inventory. Native table edits do not grow a second policy | In-memory handle, fresh lookup, save and reopen |
| PowerPoint append keeps earlier owners | `Paragraph.add_run`, `Paragraph.add_field`, `TextFrame.add_paragraph` | Existing generation proof in `appended_text_owners` | Held handle, earlier collection, new run identity, stale view |
| Local edits do not scan the unrelated package | The seven Issue 389 operations | Existing publication and OPC transaction | Release timing plus a visit or allocation bound |
| One range removal is one transaction | Native `Document` and Python `Document.remove_content_range` | Existing direct-body fragment boundary. Picture pruning policy shared with F-X261 | Empty, one, many, invalid, save and reopen |

## Behavior contracts and evidence

| Surface or placement | Exact behavior and boundary examples | Primary source identity and location | Local discriminator or pending evidence |
|---|---|---|---|
| Word cell, row and table property setters | A property edit that keeps the same cells rewrites owner addresses and keeps fresh and held paragraph and run handles. A real insert, removal or replacement keeps conservative retirement. Flat and nested cells use the same rule. | Issue 388 on main `7496a81c`. Current publication assigns the candidate without an owner revision. `PyDocument::adopt_property_edit` is not on this branch. | A1 and A2. Old source raises `element has no current checked physical owner`. |
| PowerPoint append | `add_run`, `add_field` and `add_paragraph` keep earlier paragraph and run handles. Only the appended owner's cached text view is retired. A list already taken from `frame.paragraphs` does not grow. A later iteration sees the new paragraph. | Issue 388. `appended_text_owners` in `crates/rpptx-py/src/presentation.rs`. Paragraph collection capture binds the text scope. | A3 and A4. |
| Seven release operations | Ratio below 3. PowerPoint small package is an empty deck. Large package is 40 slides with 40 text boxes. Operations are `add_textbox`, alt text, `left`, `fill.solid` and `replace_xml`. Word small package is 50 paragraphs. Large package is 800 paragraphs. Operations are `replace_xml` and `remove_content`. A visit counter fails when the edit reads an unrelated slide or body block. Debug timings do not pass. | Issue 389 body, the release-build table on main `7496a81c`. | A5. The ratio is final-sprint evidence. The visit counter is the local gate. |
| Direct body range removal | `Document::remove_content_range(start: usize, end: usize) -> Result<usize>` and Python `Document.remove_content_range(start, end) -> int`. Half-open over direct body children. Empty range returns 0 and does not stage. `start > end` or `end > len` returns `Error::Other("content range is reversed or out of range")` and does not publish. Python surfaces that as `RdocxError`. The call does not enter a table. Picture pruning calls the F-X261 reachability rule. | Q8. The error variant matches `Error::Other` already used by checked story edits. `try_remove_content` is `crates/rdocx/src/document.rs` around line 26748. | A6 and A7. |

## Dependency readiness

All formal provider dependencies and shared-file leases were integrated before
this implementation. The applicable local gates and final provider riders now
pass with actual producer/source correspondence. S95.3 final release preparation
and its final sprint review remain separate, unfinished obligations.

## Acceptance criteria

| ID | Input or setup | Expected observation or refusal | Consumers | Local test | Final gate or pending evidence |
|---|---|---|---|---|---|
| A1 | Flat and nested Word cell. Shading, width and column width. Held run and a fresh cell taken after the edit | `runs[0].text` reads the cell text. A later run edit survives save and reopen. The old source raises the missing-owner error | Python cell handles. Native package bytes | `issue388_cell_property_edits_preserve_live_owners` | Installed `rdocx` product suite at final sprint |
| A2 | The related setters listed above, plus a setter that really replaces the cell content | Property edits keep the same owner. A genuine replacement still retires the removed owner | Python table, row and cell setters | `issue388_property_edit_owner_boundaries` | Same installed suite |
| A3 | One PowerPoint paragraph. Two `add_run` calls and one `add_field` on the held handle | The paragraph handle and the earlier run stay usable. The new run's handle addresses that run | Python paragraph and run handles | `issue388_ppt_append_owner_identity` | Installed `rpptx` product suite |
| A4 | `TextFrame.add_paragraph` while an earlier run handle and an already-materialised paragraph collection exist | Earlier owners stay live. The collection keeps its earlier members and gains no silent refresh. Callers iterate again to see the appended paragraph | Python text frame, paragraph collection, run | `issue388_ppt_append_collection_contract` | Same installed suite |
| A5 | The reporter's small and large packages for all seven named operations | Each large/small ratio is below 3 under the pinned release workload. A visit counter shows the unrelated slides or blocks were not scanned | Python Word and PowerPoint editors. Native publication | `issue389_local_edits_do_not_scale_with_unrelated_package` | Release-build samples recorded with the final sprint. Not a worker-only debug timing |
| A6 | Empty, one-item, many-item and invalid direct body ranges | The approved half-open call stages once. Invalid input returns the old package bytes. Empty removes zero items | Native `Document` and Python `Document` | `issue389_range_removal_is_one_owned_transaction` | Installed product suite and hash harness unchanged |
| A7 | A refused XML edit, a nested table, and a package with shared media | The faster path still refuses atomically and does not drop a shared image | Python checked mutation and native package graph | `issue389_edit_cost_preserves_checked_mutation_contract` | Final hash harness unchanged |
| A8 | Issue398 exact small 50-paragraph/two-table and large 400-paragraph/sixteen-table documents, each table four by three | All three read-only handle rows have large/small release ratio below3, including first calls and a late table. Reference cell.text remains correct. Setup and any inventory construction are identified separately, never hidden by warming the timed API | Python body paragraphs, row.cells and cell.paragraphs | Deterministic selected-route work controls plus reporter workload | Actual installed release artifact, raw timings and source/build identity |
| A9 | Walk body paragraphs and cells, including nested tables and related stories | Exact source text and physical routes, bounded total work rather than per-handle whole-document scans. First and last table selection agree | Python paragraph, table and cell collections | Constructed zero/one/many, body/nested/related and table-position controls | Installed compatibility suites and scoped performance evidence |
| A10 | Read before and after text, property, row/grid, structural and no-op edits, then reopen | Current handles see documented current values or original stale cause. No stale cached route, resurrection, changed source on read or lost atomic refusal | Same handles and derived indexes, all publication paths | Read/edit/read lifecycle matrix, exact saved bytes on read and refusal | Installed suites and unchanged hash harness |

Issue 388 rows map to A1 through A4. Issue 389's seven timing rows map to A5. Its range-removal request maps to A6. A7 is the control that forbids buying A5 by skipping checks. PR393 does not satisfy A5 or A6.

## Test matrix

| Case | Modes or state combined | Expected difference or agreement | Wrong implementation rejected |
|---|---|---|---|
| C1 | Zero, one and many runs, flat and nested, held and fresh | Nested `w:tcPr` shifts indexes the same way a top-level cell does | A repair that only rewrites the top-level inventory |
| C2 | Property edit versus a real content replacement | Only the property edit keeps the owner | A blanket "never retire" change |
| C3 | PowerPoint append versus Word run retirement | Earlier PowerPoint runs survive. Word retirement outside this story stays as documented | Copying the PowerPoint rule into every Word setter |
| C4 | Small package versus large package, same edited element | Ratio below 3 and a bounded visit count | A timer-only pass that still walks every slide |
| C5 | Current source, PR393 candidate, integrated successor | Same inputs. Old evidence is not relabelled as a new run | Merging the candidate without re-running it on `67423b6b` plus F-X259 |

CLI and WASM are unaffected. They do not hold these Python owner inventories. Native Word table mutation already has its own handles and is only a byte-level control for A1 and A6.

## Rejected alternatives

- Merging PR393. It leaves the whole-document walk in place and adds no range API.
- A second owner registry. The current inventory already has the addresses.
- Disabling stale checks on large packages. That fails A7.
- Treating Issue 389 as solved because Issue 388's handles work.

## Test plan

**Test gate**: regression. Each A-ID needs an old-source semantic RED and a repaired GREEN. A missing range method is a compile failure and is recorded separately from the timing RED.

| Timing | Category | Test | Asserts |
|---|---|---|---|
| local scoped | regression | A1 through A4 | Old missing-owner and stale-append failures, then live handles |
| local scoped | regression | A5 visit counter | Unrelated slides and body blocks are not visited. The ratio is the release observation |
| local scoped | regression | A6 and A7 | One transaction, atomic refusal, shared media kept |
| final sprint | risk rider | installed `rdocx` and `rpptx` suites, release ratio samples, full hash harness | No Word hash delta. Only the range method and the two approved doc-hidden native read projections are added. Python adds only the range method |

Local commands are the existing Python files `crates/rdocx-py/tests/test_formatting_tables.py` and `crates/rpptx-py/tests/test_documented_examples.py`, plus the visit assertion in the existing library unit tests named under Deviations. No new test binary.

## Deviations

This section records the original implementation checkpoint. Accepted provider completion below supersedes earlier pending evidence and readiness statements. Historical failures and missing-input limits are retained.

The visit counter is `cfg(test)` state inside `rdocx`, `rpptx`, `rdocx-py` and `rpptx-py`. An integration binary and pytest do not compile that state, so the visit assertion lives in each library's existing unit tests under the A5 name. The same name in the two Python files checks that the edited owner changes and an unrelated owner does not.

The affected-owner paths also edit `crates/rdocx/src/document.rs`, `crates/rdocx-py/src/paragraph.rs`, `crates/rpptx/src/lib.rs`, `crates/rpptx-py/src/shape.rs` and `crates/rpptx-py/src/dml.rs`. Those files already own the operations. No new crate, module or file was added.

The final workspace suite rejected a plain removal in a package that retains a shadowed `wp` prefix. The local path now stays closed for that package. The decision is stored when the package is opened or saved, so the edit itself still reads one child.

The same suite rejected a signed PowerPoint run replacement that stayed cryptographically valid. A retained package signature keeps the staged transaction, which publishes the part and leaves the signature parts in place.

## HLD impact

- `docs/hld/03-architecture.md`
- `docs/hld/10-bindings-spec.md`
- `docs/hld/12-testing-strategy.md`
- `docs/hld/14-development-backlog.md`

`docs/hld/04-opc-and-packaging.md` is not edited. Relationship pruning stays the F-X261 contract.

## Risk routing

- Contributed PR or new issue acceptance. Pin the refreshed heads. Map every Issue 388 and Issue 389 row. PR393 is not whole-issue closure.
- Public API of a published crate. Range removal is additive. State the semver impact. `cargo publish --dry-run` and the crate size check run in the final union.
- WASM or PyO3 bindings. Final `cargo check --target wasm32-unknown-unknown` for both graphs. Workspace tests keep `--exclude rdocx-py --exclude rpptx-py`.
- Shared semantic behavior with multiple consumers. The consumer map is the check. Held, fresh and reopened handles see the same text.
- Grammar, scope, compatibility mode or empty collection. C1 and A6 cover zero, one, many and the invalid range.
- Any parser or serialiser, only for the range edit. Child order and atomic reopen. No new XML vocabulary.

Layout, theme colour, unit constructors and the hash baseline are not triggered. A5 must not change paint.

## Hash harness

Unchanged. A Word or PowerPoint byte change that is not required by the owner repair stops the story. F-X262 owns any intentional PowerPoint paint delta.

## Implementation checklist

- [x] Inspect the integrated owner providers after F-X259 lands. Re-read PR393 at the pinned head.
- [x] Record old-source RED for A1 through A5. The release ratio is a final observation, and debug timings do not pass A5.
- [x] Implement through the existing inventories and publication path.
- [x] Update the HLD impact list and obtain a zero-finding microscope.
- [x] Accept the actual source-qualified release ratios and complete installed suites, with original producer attribution.

## Contract closure

R4 in `.claude/reviews/S95.3-design-quality-pass-1.md` does not change A1 through A7. The running worker acknowledged the canonical plan, including the behavior contracts added after claim base `c776041c`, in its progress file before further implementation. The local visit gate, the HLD update and microscope pass 2 are recorded. The release ratio of 3 stays final evidence.

## Open questions

| ID | Affected F-IDs and A-IDs | Decision and recommendation | Status and answer/source |
|---|---|---|---|
| Q8 | F-X260 A6 and A7 | Half-open `remove_content_range` on direct body children. Empty returns 0. Reversed or out-of-bounds does not publish. No table recursion. Picture pruning uses the F-X261 rule. | Answered 2026-10-18 by the user. Rust `Result<usize>`, Python `int`, error `Error::Other("content range is reversed or out of range")`. |
| Q9 | F-X260 A3 and A4 | Keep earlier PowerPoint paragraph and run handles across append. An already-materialised paragraph list is not refreshed. Word run retirement outside these setters stays as documented. | Answered 2026-10-18 by the user. |
| Q10 | F-X260 A5 | All seven Issue 389 release operations keep a large-to-small ratio below 3. A visit counter fails a whole-package scan. Debug timings do not pass. Range removal is not one of the seven rows. | Answered 2026-10-18 by the user. Workload sizes are the Issue 389 release table: empty deck versus 40 by 40, and 50 versus 800 paragraphs. |

No issue or pull request closes from this plan.

## Codex takeover, installed regression reconciliation

The user's takeover request includes final installed checks. A fresh rpptx
binding exposed A7 violations in existing tests: the fast local XML path
projected away foreign geometry before checking submitted bytes, and could
not establish inherited MC authority. Validate submitted frontier bytes before
projection and defer unproven fragments to the existing whole-part transaction.
A graphic-frame table-presence refusal must restore exact pre-edit package
state, not reconstruct a snapshot after mutation. Textbox append must retain
the existing structural duplicate-ID refusal, inspecting only its target slide.
The existing installed fx206/fx211b/fx254 tests are discriminating regressions.
Requalify A5 because append/XML hot paths changed. No new registry or public API.

Takeover microscope D1 rejects a full-presentation rollback clone for every
graphic-frame edit. Preflight incoming table presence with the existing
`rpptx-oxml` concrete graphic-frame parser, before native publication. The Python
binding adds that existing workspace dependency for this named consumer. Reject
payload transitions before mutation and keep unchanged-payload edits local.

## Codex takeover, public Python cost repair

The actual release Python workload on source `6f206c83` failed four Q10 rows.
The native-only observations do not qualify A5. Retain the failed release run
and exact extension identities in `/tmp/s95.3-codex-issue389-release-qualified.log`
and `/tmp/s95.3-codex-release-manifest.json`. No threshold changes or timing
retries without an implementation change.

Keep the existing owner inventories authoritative. Add derived generation and
direct-body indexes where needed for checked owner lookup. Rebuild these indexes
only with ordinary inventory replacement and update affected entries for local
publication. A direct plain paragraph edit resolves its captured generation
without serializing every story, and retires only its content/property and
contiguous plain-run scopes. Unsupported paragraph structures keep the checked
staged path.

For Word tail removal, prepare a shared removal epoch before mutation. Capture
that epoch in every physical Word owner path so conservative invalidation does
not allocate successors for unrelated owners. Use the shortcut only for the
last direct plain paragraph when its inventory descendants are contiguous and
the remaining suffix contains only sections. Remove that local range and update
its derived indexes. Other removals keep the existing conservative path. Held
paragraphs/runs and retained wrappers must still refuse after removal, including
after a later append reuses the removed numeric slot. No history is retained.

PowerPoint append must prepare only the three newborn owners, reserving storage
before publication, and preserve all existing owner identities without cloning
the entire inventory. Checked generation lookup must avoid a full inventory
search on each property/XML read. Existing staged paths remain the fallback
for unproved structural changes. Instrument actual traversals at the acceptance
boundary and retain selected-owner, stale-handle, refusal and reopen controls.
Independent microscopes and a fresh seven-row release measurement are required.

Binding cost microscope pass 1 adds three concrete repair obligations. Group
the two document epochs in the capture signature without a lint exception.
Stage complete replacement indexes and their owned keys with the prepared
inventory, then install them by moving. Reserve local successor-key capacity
before local native edits. Route non-tail Word removal through the existing
candidate transaction rather than mutating live state before preparation.
Prepare textbox scopes, index/inventory capacity, captures and the returned
Python shape before native append. Bind its constructor-assigned id only after
append succeeds, before publishing the three prepared current owners. No
fallible scope preparation or Python result allocation follows that append.

The required D2 designer investigation is recorded in
`.claude/reviews/F-X260-publication-design-investigation.md`. Implement its
pre-native local/staged decision. Exact owned XML equality is a no-op, matching
native admission. Ordinary shape proof distinguishes absent/empty bodies and
regular runs from fields/breaks. A real name/accessibility/geometry scalar
change witnesses publication. Frame scalar proof requires unchanged complete
graphicData. Enumerate affected frame/table descendants with the existing
owner-address builder, not slide-contiguous assumptions or a second registry.
Unproved XML, other Element kinds and topology changes use a candidate, all
post-edit reads and preparation on that candidate, then one publication.

Scalar shape editing uses explicit concrete alt-text and direct-geometry
proofs for bounded paths. Unproved callbacks stage, including uncertain imported
state. Non-blip shape fill prepares before native mutation. No post-live parser,
serializer, new owner proof or fallback remains. Add the existing workspace
`oxml-drawing` dependency for the concrete TextRun/transform proof consumers,
with no new public API. The designer's publisher and discriminator tables are
the work list. Native oracle debt and Q10 stay unchanged.

## Approved intake extension, 2026-10-22

The user's new intake request adds A8 through A10 for Issue398 body SHA-256
`4e940897286f91d30bf5c2b54497462e50077e89313262db6ea38eebccaf6124`.
Candidate PR399 head is `34dfd42782de08bc331a5f6c819774720bb60067`.
It claims faster repeated walks but discloses that the exact reporter's script
still fails from per-state first-call inventory construction. A warmed ratio or
a four-times-size walk below6 does not replace the three exact below3 rows.
Nested and non-body routes remain explicit acceptance, not silent exclusions.

Prefer derived indexes over current checked owner inventory, with complete
preparation and coherent invalidation at every publisher. Inspect the existing
native location and story refresh paths before choosing the minimal mechanism.
Read-only handle construction must not clone or serialize unrelated content.
If extra cached source facts are necessary, derive them at the existing state
preparation boundary and prove mutations that preserve revision serial cannot
leave stale entries. Do not import PR399's TrackedDocument wrapper, LocationSource
trait or hidden public batch methods without a demonstrated need and explicit
contract. Only the two doc-hidden native read-projection methods specified below are approved additions for A8 through A10. No new Python API.
Supported imported routes and native refusal semantics remain authoritative.

Consumer map adds body/nested/related paragraph, table and cell collections,
physical owner capture, fresh lookup, held reads and read/edit/reopen. Test
zero/one/many, first/last table and repeated read without publication. Compare
exact routes/text against the existing checked provider. Measure actual owner
work as well as final release timing. Scope of any new native batch method must
be designed before implementation, not inherited from candidate code.

PR397 head `d9721353314ba969c05bd9f886bdd24d15d51a8d` is an additional
candidate for A5 through A7, not a new accepted implementation. Its first-fill
and add-textbox ratios do not qualify all Q10 rows. It leaves both Word cost
rows unimplemented. Its Range-based Rust API and Python IndexError differ from
the already approved A6 signature and RdocxError contract. Keep the approved
contract. Do not add public equality or serialization-comparison APIs merely
because the candidate contains them. Candidate archive values do not describe
this sprint's artifacts and cannot replace fresh affected measurements.

Both PRs target `b072b9852d5b17c7bcd19889cef058556ce9b67d`. All acceptance
runs must identify this sprint's actual source and binaries. Finish the current
publication repair and focused review before the read-only performance changes.
The latter share Word binding files and require another focused review of their
actual incremental diff. Final suites, hash, release ratios and all existing
risk riders remain due once over the integrated union. No new material product
choice is open. No issue closes at intake. Grok documentation proposals are
untrusted input until checked by Codex against these pinned sources.

## Read-index mechanism closure

Source inspection shows that current physical owner snapshots already carry
qualified table/cell ancestry and ordered paragraph source, but facade body and
cell ordinals are different axes. A direct projection-ordinal shortcut would
misbind paragraphs in nested controls. The existing single native lookups clone
or scan unrelated source and cannot be used once per handle.

Use two additive doc-hidden native batch projections with concrete Python
consumers: `paragraph_story_locations() -> Result<Vec<Option<ContentLocation>>>`
and `cell_paragraph_story_locations() -> Result<HashMap<(usize, usize, usize),
Vec<ContentLocation>>>`. Match the corresponding existing single lookup exactly
on supported entries. Missing/unsupported entries retain that original lookup,
including its refusal. These are source facts, not new mutation authority or a
Python API. PR399 supplies the candidate algorithms. Verify actual batch/single
parity, including nested/control cases, before trusting their output.

Prepare these projection maps with the existing immutable owner inventory at
open or before candidate publication. Do not build them lazily inside the first
timed handle call. Keep prepared captures on the existing checked path. Current
captures use a derived route index with exact same_route confirmation, including
hash collisions and first-match behavior. Current-generation lookup checks the
actual token, not the numeric generation alone.

Cache only read facts derived from that same admitted inventory: accepted
paragraph text and table row/cell paragraph counts. These facts are not serialized
and grant no ownership. Unsupported table-source parsing retains native reads.
They are prepared with full replacement or property relocation. A local plain
paragraph XML edit prepares its next accepted text before native mutation and
installs it only on successful publication. A tail removal drops only removed
facts and updates affected indices without rescanning survivors. No-op/refusal
keeps the old facts. Do not introduce a forwarding document wrapper, lazy
whole-document cache, trait or second owner registry.

Bindings resolve body/table/cell paragraph aliases through prepared projections,
parent-cell indexes and current scopes. Read text/counts through the current
owner only after checking captured authority. Mutations still refresh native
fingerprints through existing checked providers. Nested-table enumeration uses
the same current source inventory and preserves source order. Repeated route
reads must not call stories or serialize source merely to capture authority.

Tests compare body, cell, nested and related read facts to actual native/saved
content, then edit text, properties, rows and XML and repeat. Include unsupported
routes, duplicate content, stale collections, removal/reinsertion, no-op and
refusal. Meter capture/projection/text traversal at its actual boundary after
open and after edits. Report initial projection preparation cost separately.
The two hidden native additions are additive pre-1.0, with no new public types.
Final packaging, WASM and hash riders cover them with the integrated union.

## Issue398 D1 publisher closure

Focused Issue398 pass1 found that Run.add_tab and Run.add_break install an
owner-preserving native candidate without changing the revision. Their paragraph
read facts must change even though captured scopes stay live. Both are the only
callers of the existing PyRun.edit helper in run.rs. Keep that shared edit path.
After candidate mutation, read accepted paragraph text through the existing
native reader. Validate the current run and its paragraph parent before
publishing, then move the prepared text and candidate together without changing
scopes. Preserve legacy unbound fallback, held runs/fonts, native refusal and
paired text-box behavior. No post-live preparation or broad cache invalidation.

A10 now explicitly covers tab, default line, page and column breaks in body,
cell, nested cell, header and qualified paired text-box contexts. Header page
break is the atomic refusal control. Observe held/fresh paragraph text, actual
run text, retained font views, both paired physical XML copies and save/reopen.
The old dev extension is retained with the review's SHA-256, alongside the first
release wheel. The actual initial matrix is15failures and1passing refusal, not
a source counterfactual. A fresh focused pass2 is required after GREEN.

## First-run and physical-cell follow-up

F-X260 source-backed follow-up, Issue389 initial empty paragraph
Actual release failure7.336295, child1, /tmp/s95.3-final-issue389-release.log.
No threshold or workload changes. Current native plain shortcut refuses a
change in accepted run paths. Reporter first introduces a run into a previously
empty paragraph. Local existing test incorrectly exercises only an existing run.

Proposed bounded extension: keep equal-path edit support and additionally
allow a plain paragraph with zero accepted runs to acquire direct plain runs.
Retain all native paragraph checks, section refusal, namespace/MC/identity and
relationship admission before mutation. No other topology change is local.
Prepare existing owner scope replacements and newborn run authority before
native publication. Append newborn runs to the current finite owner vector to
avoid shifting unrelated indices. Exact run route buckets grant authority only
after native admission. No historical owners are retained. Later full admission
rebuilds source order as before. Local follow-up edits find child run owners
through the existing exact route index rather than relying on adjacency.
Tail removal falls back for non-contiguous descendants of the removed owner.
Independent appended runs whose parents precede the removed owner may remain
in the suffix, with their route/generation indices shifted alongside sections.
This visits the actual appended suffix, not every physical body owner. Do not
claim a universal bound independent of all earlier local edits.
No new API, registry, trait, generic, crate, file or feature flag.

Use a private preparation struct in existing document.rs if necessary to make
the staged state explicit and avoid another tuple of opaque positional values.
Pre-reserve affected owner and hash-index storage and prepare collision buckets,
revision scopes, accepted text and new owners before native writes. Native no-op
or refusal must not publish prepared states or births. Commit only moves prepared
values. Preserve first stale cause and existing paragraph owner identity.

Prove actual old-code RED for first empty-to-run public call at50/800 native
source sizes with metered work. Then assert constant bounded preparation and
native work. Exercise0->1 and0->2, follow-up equal-path edit on appended owners,
held paragraph live, previous content/run/font wrappers stale as contracted,
fresh run edit and formatting, no-op unchanged identities, malformed/section/
unsupported refusal exact bytes, save/reopen, later property/full edit and tail
removal conservative behavior. Do not generalize growth/removal with existing
runs. Rebuild release artifacts only after fix and rerun original strict scripts
once on changed source. Preserve original failed wheel and measurement hashes.
Fresh independent microscope must review this new publication path.

D1 designer investigation, root Codex, before further mutation:
ParagraphLocation::Cell.read uses immutable CellRef.paragraph, which includes
controlled paragraph content. Its edit callback uses mutable Cell.paragraph_mut,
which selects direct paragraphs only. A controlled paragraph before a direct
paragraph makes the same ordinal address two owners. Run.edit reads accepted
text through the former but mutates through the latter. Updating additional
cache entries would conceal the incorrect physical edit. Resolve the checked
cell_paragraph_story_location through the original native reader and invoke
edit_story_paragraph on that exact physical location. Keep body and related
providers unchanged. This shared provider covers all callback publishers, not
only tabs/breaks. Test controls first/middle and direct/control targets, run
text setter and paragraph content setters beside tabs/breaks, both held and
fresh values and actual XML, unchanged neighbors, and save/reopen. Refusal must
publish nothing. This is a source-based identity correction, not native parity.

## D2 physical section admission

Generic paragraph XML refuses introduced, removed or changed physical section
properties on both local and staged paths. The shared paragraph admission
scanner inventories direct qualified sectPr children of pPr for Start and Empty
events. Normalize a temporary Empty representation to paired tags only for
checked semantic comparison, with the actual namespace scope closed. Parse that
temporary section through the existing CT_P/CT_SectPr model. Compare the ordered
section values and multiplicity before publication. Keep original submitted
bytes and parser/serializer behavior unchanged. Retain the local modeled-section
guard so no new section-owner topology can slip through a local commit.

Tests cross first-run, same-run and staged run-topology paths with self-closing,
paired-empty and nonempty section forms. Prove actual three-failure RED beside
six passing refusal controls. Preserve unchanged imported empty and nonempty
sections during text edits, including qualified aliases. New/removal/change
refuses with exact package bytes and live handles. Related-story placement uses
the same shared guard. This does not claim authentic Word qualification.

A constructed positive with an alternate prefix declared on the entire
paragraph exposed an existing local namespace-owner replay failure at save.
Preserving a section's qualified alias is a separate positive control, declared
on the section itself. Do not mislabel the outer-paragraph failure as a section
guard regression or erase it. Route packages with retained nested modeled
namespace owners through staged identity proof. If the original provenance
cannot be reconciled, refuse atomically rather than publishing an unsavable
local mutation. No namespace declaration or unmodeled byte may be discarded.

## D2 opaque imported section preservation

The first D2 repair's temporary paired parse fails for a valid imported Empty
sectPr with a foreign root attribute. The existing paired parser captures root
attributes using its Word prefix scope, so a normal foreign binding is lost.
Do not change that public parser in this repair or reject previously readable
opaque bytes. Use two explicit private section cases in shared admission:
a successfully parsed semantic section and an opaque Empty section whose
temporary semantic parse fails. The opaque case uses the existing protected
island representation with exact raw bytes and expanded-name/namespace meaning.
It is never an authored-Ignorable escape. Only an unchanged opaque section may
remain. New, removed or changed instances refuse by ordered comparison.
The regular incoming checked paragraph parse still runs before admission.
Paired input parser failures retain their existing refusal. This fallback does
not create a second mutable document model or normalize serialized bytes.

The enum represents two actual imported states and avoids invalid combinations
of optional semantic/raw fields. Box the concrete parsed model if needed for
its large size. Prove text edit and exact preservation for self-contained
foreign attributes in canonical and qualified-alias sections, plus refusal of
attribute changes and removal. Keep all introduction and namespace controls.


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
