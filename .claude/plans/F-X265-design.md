# F-X265, Preserve fragment relationship targets and duplicate styles

**Status**: completed
**Sprint**: S95.4
**Size**: M
**Depends on**: F-X261

## Execution boundary, 2026-10-27

The API, preservation, refusal and acceptance contracts below remain binding. Problem descriptions, approval-time unrun statements and local provider-prefix checkpoints retain their historical source and scope. They do not describe the final integrated execution state. At this checkpoint, the native union and affected corrective cases, installed Python suites and typing, native Word and PowerPoint qualification with explicit source applicability, A7 measured memory and retained-history review, A9, the original Issue407 gate, hash49, lint, WASM, rustdoc, supply chain and policy observations are available. Actual producer identities remain attached to each observation. No old binary is relabelled as a new build.

The remaining integrated acceptance debt at this checkpoint is Cell.add_picture's required flat-cost gate, the active README example batch, the clean22-package publication dry run and the final full-union record. The original failed attempts remain evidence. Status stays approved until the required final acceptance passes. When this plan becomes completed, its final source-bound delivery entry in docs/sprints/AS_BUILT.md supersedes these historical execution-status statements. Completion changes neither the contract nor any threshold. Build-only hosted wheel qualification belongs to release preparation before final release approval. The subsequent F-X270 intake prepares fresh0.16.4/0.14.4 patches within this sprint because the old0.16.3/0.14.3 candidates were already published. It is not an additional acceptance criterion in these eight sprint designs. Publication remains separately approved at the exact reviewed main SHA.

## Problem

Fragment import can drop a relationship that a later staged edit still needs, and it can copy a shadowed style definition that the target already defined. Picture order and reuse are part of the same relationship rule. Base `a57be365` has no `register_pending_relationship`. The donor adds a crate-local helper. That patch is a candidate, not the product choice by itself.

Issue 404 and PR411 `3c9c296c276f69d2b6cc8a1eab5674627709887f` sha256 `3833f4a3b82bd68c4577ce5fc96a13b333c33e6b91caf02438557b766b67a0eb`. Issue 408 and PR410 `4070a7886d10bced0e6da60d28e351fbc9b8a831` sha256 `2414647ac08df90bd39e1bfabec2af9f1153c25baa85b03057c0877d437975b1`. Receipt `/private/tmp/s95.4-contributions/patch-receipt.json`. Diffs were not executed.

## Spec reference

- `docs/hld/04-opc-and-packaging.md`, Relationship types, and Picture extraction and one-shot publication.
- `docs/hld/03-architecture.md`, Shared Word document operations.
- `docs/hld/10-bindings-spec.md`, Word tables, physical pictures and internal links.
- `docs/hld/12-testing-strategy.md`, Word document-operation parity controls, and Word table, picture and paired-owner controls.
- `docs/hld/14-development-backlog.md`, F-X265. Test gate is regression.

## Approach

No new public API, module, trait, or test binary. Planning estimate is 2 to 3 developer-days, provisional and not a measurement.

Crate-local helper on the existing document type, matching the donor name and visibility:

```rust
pub(crate) fn register_pending_relationship(&mut self, owner: &str, id: &str);
```

Provenance is the main part that staged the relationship. The pending id is removed when a reference appears or when that id disappears from the relationship registry. Owner-part disappearance alone does not end protection. Reconciliation intersects the registry before save. The helper does not promise that an arbitrary unreferenced id survives a fresh reload.

Exact existing signatures, cited from base `a57be365`:

```rust
pub fn import_fragment(
    &mut self,
    destination: &ContentLocation,
    fragment: &DocumentFragment,
    policy: FragmentConflictPolicy,
) -> Result<()>;
pub fn insert_document(&mut self, index: usize, other: &Document);
pub fn embed_image(&mut self, image_data: &[u8], filename: &str) -> String;
pub fn add_hyperlink_relationship(&mut self, url: &str) -> String;
pub fn add_hyperlink_relationship_to_story(&mut self, story: &StoryId, url: &str) -> Result<String>;
```

`Document::import_fragment` parameter order is destination, fragment, policy. The name is `destination`, not `at`. Python `Document.insert_document(self, other: Document, *, at: int, policy: FragmentConflictPolicy | None = None) -> None` is the stub at `_rdocx.pyi:1255`. `embed_image`, `add_hyperlink_relationship` and `add_hyperlink_relationship_to_story` are native only. They have no Python stub. Native typed `insert_document` is a different body route and stays an unaffected control. The shared provider keeps every fragment kind it already admits. No new Word oracle is added for native open, save or reopen.

Native typed `insert_document` is the existing body, style and numbering route. It is not a fragment media importer. `Document::insert_document` stages through `insert_document_content_staged_with_numbering_policy` at `crates/rdocx/src/document.rs:35177`. That function reserves merge bundles, merges styles, clones body content and remaps numbering. It does not import source media relationships. The unaffected control uses a source and a destination that already share the intended media graph, then compares the correct source picture bytes after reopen. Distinct-media native insertion is not newly qualified and is not repaired by this story.

Picture bytes stay identical for both insertion orders, the default path, and reuse turned off. Mixed picture, chart and hyperlink inserts return ids that still name the intended targets after intervening staged edits. Save and reopen keep referenced target bytes. Chart, template, building-block and rich-merge publishers use the same rule. Native typed insert is an unaffected control and is still tested.

Style import dedups the first definition before dependency closure.

- Equal first definitions add zero copies.
- A source first definition that differs from the target first definition adds exactly one renamed copy.
- A shadowed later definition is ignored, including its dependencies.
- Target duplicate style XML stays verbatim.
- The same style id with differing default content, including a renamed Normal, adds one renamed non-default copy of the source.
- A source default whose id differs from the existing target default still uses the existing refusal.

## Consumer map

| Rule | Entry points and outputs | Shared provider and caller policy | Lifecycle stages compared |
|---|---|---|---|
| Pending rel | `register_pending_relationship` | Drop when referenced or when the id leaves the relationship registry. Owner-part loss alone is not enough | Stage, serialize, reopen |
| Picture order | Existing picture insert, default and reuse off | Same bytes either order | Insert, save, reopen |
| Mixed targets | Picture, chart and hyperlink ids | Intervening staging does not retarget a returned id | Insert, serialize, reopen |
| Styles | Existing fragment style import | First definition before closure. Target duplicates stay | Import, serialize |
| Unaffected fragment publishers | Template, building block, rich merge | Same provider. Shared relationship and style rules. No new API | Import, reopen |
| Unaffected native control | Native typed insert | Existing body, style and numbering route. Source and destination already share the intended media graph. Correct source picture bytes after reopen. No fragment media import | Insert, reopen |

## Behavior contracts and evidence

| Surface or placement | Exact behavior and boundary examples | Primary source identity and location | Local discriminator or pending evidence |
|---|---|---|---|
| Picture order | Insert A then B, and B then A. Default path and reuse off | Issue 404. Donor is not authority for a reduced image pair | `picture_order_and_reuse_keep_bytes` |
| Returned ids | Picture, chart, then hyperlink, then another staged edit | Existing relationship maps | `staged_ids_keep_targets` |
| Inserted pictures beside unused rels | Own inserted picture bytes stay through policies, save and reopen, including the best available copied-target and body-replaced scenario. Unused relationship bytes are not a preservation promise | Issue 404. The original unreduced input is unavailable and unverified. Donor image 9 and image 10 remain a reconstructed additional case and are not claimed equivalent | `inserted_picture_bytes_survive_unused_rels` local RED then GREEN on the available cases |
| Equal styles | Two identical first definitions | PR410 candidate | `equal_styles_add_no_copy` |
| Different first | Source first differs from target first | One renamed copy. Later shadow ignored | `different_first_style_copies_once` |
| Default flag | Same id, differing default Normal. Separate case: source default id differs from the target default id | Same id yields one renamed non-default copy. A different source default id keeps the existing refusal | `same_id_default_copies_once_nondefault` |

## Dependency readiness

F-X261 is done in `BACKLOG.md`. F-X265 is pending and unclaimed. No in-sprint provider contract is required. This draft is not approval.

## Acceptance criteria

| ID | Input or setup | Expected observation or refusal | Consumers | Local test | Final gate or pending evidence |
|---|---|---|---|---|---|
| A1 | Issue 404. Both picture orders, default path, reuse off. Mixed picture, chart and hyperlink. Returned ids after intervening staging | Byte identity and stable targets through native library save and reopen | Existing insert and import | `picture_order_and_reuse_keep_bytes`, `staged_ids_keep_targets` | Native library save and reopen. No invented Word oracle |
| A2 | Issue 404 pictures inserted while unused relationships exist, using the actual available reproducer, plus the best available copied-target and body-replaced scenario | Exact inserted picture byte identity through policies, save and reopen. Unused relationship bytes are not required to survive. Do not claim the original unreduced input was tested or is equivalent | Import | `inserted_picture_bytes_survive_unused_rels` | Local RED then GREEN on the available cases. The exact original stays an unverified reporter follow-up and is not a final acceptance blocker |
| A3 | Issue 408. Equal first definitions | Zero copies | Style import | `equal_styles_add_no_copy` | Local regression |
| A4 | Source first differs from target first. A later shadowed definition and its dependencies | Exactly one renamed copy. Shadow ignored | Style import | `different_first_style_copies_once` | Target duplicate XML verbatim |
| A5 | Same style id whose first Normal definitions differ on the default flag. Separate case: the source default id differs from the target default id | Same id yields one renamed non-default copy. A different source default id keeps the existing refusal | Style import | `same_id_default_copies_once_nondefault` | Local regression |
| A6 | Template, building block and rich merge. Native typed insert whose source and destination already share the intended media graph | Fragment publishers use the shared relationship and style rules. Native typed insert is a valid unchanged same-media-graph control and keeps the correct source picture bytes after reopen. No fragment media-import parity claim. No new public method | Fragment publishers, plus the unchanged native control | `template_merge_and_typed_insert_follow_the_same_rule` | No API expansion |

## Test matrix

| Case | Modes or state combined | Expected difference or agreement | Wrong implementation rejected |
|---|---|---|---|
| C1 | Order A-B and B-A, reuse on and off | Same picture bytes | Order-dependent recompression fails |
| C2 | Chart staged between a picture id and a hyperlink id | Each returned id still resolves | Renumber-on-stage fails |
| C3 | Equal styles versus different first plus a shadowed dependency | Zero copies versus one copy and no dependency import | Closure-before-dedup fails |
| C4 | Target already contains duplicate style XML | Those duplicates remain verbatim | A cleanup pass fails |
| C5 | Native typed insert of one picture. Source and destination already share the intended media graph | Correct picture bytes stay unchanged. There is no assertion that an inserted distinct red picture should show blue | A fragment-only fix that breaks the same-graph typed insert fails. A distinct-media red-to-blue expectation is outside this control |

Omitted: a public pending-id API, and any promise that an unreferenced handle survives reload.

## Rejected alternatives

- A public relationship registry.
- Treating donor image 9 and image 10 as the original unused-rel qualification.
- Importing shadowed dependencies after the first-definition dedup.
- Dedup after dependency closure.

## Test plan

Backlog test gate, verbatim: regression.

| Timing | Category | Test | Asserts |
|---|---|---|---|
| local scoped | regression | `picture_order_and_reuse_keep_bytes` | A1 orders and reuse |
| local scoped | regression | `staged_ids_keep_targets` | A1 mixed ids |
| local scoped | regression | `inserted_picture_bytes_survive_unused_rels` | A2 asserts exact inserted picture byte identity on the available unused-relationship reproducer and the best available copied-target and body-replaced scenario. It does not assert unused-rel byte identity and it does not claim the original input |
| local scoped | regression | `equal_styles_add_no_copy` | A3 |
| local scoped | regression | `different_first_style_copies_once` | A4 |
| local scoped | regression | `same_id_default_copies_once_nondefault` | A5 |
| local scoped | regression | `template_merge_and_typed_insert_follow_the_same_rule` | A6 |
| final sprint | risk rider | `/verify --full`, hash harness, installed typing, stub and package dry-run, WASM, native Word forms | Union once. The contract is approved. This gate is unrun and stays pending |

Existing integration entrypoint and `crates/rdocx-py/tests/test_core.py` only.

## HLD impact

- `docs/hld/03-architecture.md`, Shared Word document operations
- `docs/hld/04-opc-and-packaging.md`, Relationship types
- `docs/hld/10-bindings-spec.md`, Word tables, physical pictures and internal links
- `docs/hld/12-testing-strategy.md`, Word document-operation parity controls
- `docs/hld/14-development-backlog.md`, F-X265 status only at completion

## Risk routing

- Parser or serialiser. Relationship and style XML child order stays on the existing writers. Unmodelled style XML on the target stays verbatim.
- Public API. No new public signature. Dry-run only if a later publish gate runs for the sprint.
- PyO3 and WASM. No new binding. Final union still checks WASM because the sprint union includes it.
- Compatibility and collection ordering. Picture byte order and style first-definition order are specified above.
- Shared consumers. Fragment import, template, building block and rich merge share the provider rule. Native typed insert remains the unchanged same-media-graph control.
- Contributed criteria. A2 is qualified on the available cases. The exact original stays unverified.
- Cached facts are not the defect. No new projection API.

## Hash harness

Expected unchanged for existing fixtures. New tests use new inputs. If a style-default commit changes a fixture, that delta is a labelled behaviour commit and is unmeasured now. No file move carries a behaviour change.

## Implementation checklist

- [x] Add `register_pending_relationship` in the existing document module.
- [x] Dedup style first definitions before dependency closure.
- [x] Cover picture order, mixed ids, and the style rows in existing tests.
- [x] Cover A2 with local RED then GREEN on the available unused-relationship reproducer and the best available copied-target and body-replaced scenario. Do not claim original-input equivalence. Donor image 9 and image 10 stay an additional reconstructed case.
- [x] Add no public API.

## Open questions

| ID | Affected F-IDs and A-IDs | Decision and recommendation | Status and answer/source |
|---|---|---|---|
| none | F-X265 | No product choice remained at assembly. Q16 is answered on F-290 only. | Answered by the 2026-10-23 assembly request. At that time the missing unreduced Issue 404 input was recorded as final acceptance debt. Q17 replaces that debt. It was not a design block and it is not a substitute donor case. Q16 provenance is the 2026-10-23 user choice on F-290, "Author Word-compatible canonical slots (Recommended)". |
| Q17 | F-X265 A2 | Qualify Issue 404 on the best available reproductions. The original unreduced input is unavailable. Do not claim it was tested or is equivalent. Donor image 9 and image 10 stay a reconstructed additional case. Local RED then GREEN is mandatory for the available unused-relationship reproducer and the best available copied-target and body-replaced scenario. Exact inserted picture bytes must match through policies, save and reopen. The exact original stays an unverified reporter follow-up and is not a sprint-closure blocker. | Answered by the user on 2026-10-23: "Original input is unavailable we will fix it now as best as we can and leave a note to the reporter to test against their original image and the can reopen if this is not fixed with the example image". The user authorizes a reporter comment only after an actual tested fix is available on a reviewed sprint ref, or in the proper close phase. The note invites a test of the original image and a reopen if it still fails. This amendment does not send that comment, close the issue, or record a fix. |

## Contract closure

Codex root semantic validation on 2026-10-23 approved this contract at source `a57be365`. Evidence is unexecuted. At approval, the missing unreduced Issue 404 A2 input was recorded as final acceptance debt and was not a design block. No donor case is a substitute. Q17, below, replaces that debt. Implementation and final gates stay pending. This approval does not mark the story completed. The plan stays approved.

`register_pending_relationship(owner, id)` is crate-local. It has no Python exposure. Removal is internal when a reference appears or the id leaves the relationship registry. A1 through A6 name the proofs. No Word oracle is added.

The shared provider preserves every fragment kind it already admits. The local discriminators are picture insert, chart, hyperlink, style import, template, building block, rich merge, and native typed insert.

The equal-style, different-first, shadowed-dependency, target-duplicate, and default-flag rows are the whole style contract. There is no broader "all styles" claim.

Before Q17, A2 was the only open evidence item. It did not ask the user to choose a policy. Q17 is that answer.

## Q17 amendment, 2026-10-23

The contract stays approved. This section amends only the active A2 obligation. The approval paragraph above records the pre-Q17 reading. That reading is no longer the acceptance rule.

A2 requires an actual available unused-relationship reproducer. Exact inserted picture byte identity holds through policies, save and reopen, including the best available copied-target and body-replaced scenario. Local RED then GREEN on those available cases is mandatory. Donor image 9 and image 10 remain a reconstructed additional case. They are not claimed equivalent to the original. The exact original input stays unverified. Reporter follow-up comes after a concrete tested fix. It is not a final acceptance blocker. No original-input requirement blocks sprint closure.

The user authorizes a reporter comment only after that tested fix exists on a reviewed sprint ref, or during the proper close phase. The comment invites the reporter to test the original image and to reopen the issue if the example still fails. This amendment does not send the comment, close Issue 404, fetch, push, or record a result. The exact note is drafted later with the real head and the real test details.

## S1 clarification, 2026-10-24

Codex root microscope pass 1, recorded in the sibling review `.claude/reviews/F-X265-all-pass-1.md`, found 0 runtime defects and smell S1. S1 says the native control must not pin a distinct red source drawing as destination blue bytes.

On canonical `sprint/s95.4` at `d5b41edf4191a1f1b37c9f699b2f3dd2ab06b319`, `Document::insert_document` calls `insert_document_content_staged_with_numbering_policy` at `crates/rdocx/src/document.rs:35177`. That function reserves merge bundles, merges styles, clones body content and remaps numbering. It does not import source media relationships. The review names this route. Its citation `document.rs:35220` falls inside `reserve_merge_bundles` on this HEAD. The function entry is line 35177.

The original aggregate Base failed in the template section before the typed assertion. There is no observed isolated Base result for the native control. This clarification does not invent a RED run and does not add a native media-import repair.

Q17, A1 through A5, the public API, estimates, the HLD impact list and the approval history stay as written. Status stays approved. The canonical revision left its own implementation checklist unticked. This worker copy keeps the implementation ticks already recorded for the completed runtime work. No gate is claimed passed. The worker acknowledges this revision and corrects the fixture in a separate task.
