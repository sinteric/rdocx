# F-X264, Repair Word defaults package inspection and comment projection

**Status**: completed
**Sprint**: S95.4
**Size**: L
**Depends on**: F-X260, F-X261

## Execution boundary, 2026-10-27

The API, preservation, refusal and acceptance contracts below remain binding. Problem descriptions, approval-time unrun statements and local provider-prefix checkpoints retain their historical source and scope. They do not describe the final integrated execution state. At this checkpoint, the native union and affected corrective cases, installed Python suites and typing, native Word and PowerPoint qualification with explicit source applicability, A7 measured memory and retained-history review, A9, the original Issue407 gate, hash49, lint, WASM, rustdoc, supply chain and policy observations are available. Actual producer identities remain attached to each observation. No old binary is relabelled as a new build.

The remaining integrated acceptance debt at this checkpoint is Cell.add_picture's required flat-cost gate, the active README example batch, the clean22-package publication dry run and the final full-union record. The original failed attempts remain evidence. Status stays approved until the required final acceptance passes. When this plan becomes completed, its final source-bound delivery entry in docs/sprints/AS_BUILT.md supersedes these historical execution-status statements. Completion changes neither the contract nor any threshold. Build-only hosted wheel qualification belongs to release preparation before final release approval. The subsequent F-X270 intake prepares fresh0.16.4/0.14.4 patches within this sprint because the old0.16.3/0.14.3 candidates were already published. It is not an additional acceptance criterion in these eight sprint designs. Publication remains separately approved at the exact reviewed main SHA.

## Problem

Python line spacing is inconsistent across default, paragraph and style writes. The current style writer already handles `Length` in its else branch. The defect is that a plain `int` is treated as EMU, a float is a multiple, and the default surface accepts `Length` only, plus on-grid loss of an absent `lineRule`. The old claim that the `Length` branch misclassified `Length` is withdrawn. Native `Paragraph::set_line_spacing_multiple` at `crates/rdocx/src/paragraph.rs:1613` casts `multiple * 240.0` with `as i32` and does not refuse non-finite values. `auto_line_spacing` stays that infallible cast domain and also repairs the on-grid case. Length-first dispatch is required for the new numeric rule. Default space fields stay `Length` only.

Comment listing uses the strict doc-hidden map at `crates/rdocx/src/comments.rs:2650`. One unrepresentable anchor fails the whole inventory. Package inspection at `crates/rdocx/src/document.rs:29895` and `29911` lists model part keys and decoded XML. It does not return the exact `[Content_Types].xml`, root rels, main document and header rels that `OpcPackage::write_to` at `crates/oxml-opc/src/package.rs:282` would emit.

The design baseline is `a57be36578b1b419b87986b1accbcfca481277ce`. The diagnosis in this section is the design-time reading of that source. Donor diffs are candidates, not product authority. "Nothing here is implemented" refers to that design-time baseline. The current actual claim base `8084d259` is separate, and local progress records implementation against it. Final acceptance remains pending.

## Spec reference

- `docs/hld/10-bindings-spec.md`, Python Word formatting and checked list publication, and Word checked XML owner authority.
- `docs/hld/04-opc-and-packaging.md`, The package, and Part naming.
- `docs/hld/03-architecture.md`, Owned publication and causal paths.
- `docs/hld/12-testing-strategy.md`, Checked Word comment anchor projections, and Word checked XML qualification.
- `docs/hld/14-development-backlog.md`, F-X264. Test gate is regression.

## Approach

Work only in existing files. No new module, trait, feature flag, or test binary. Planning estimate is 3 to 5 developer-days. That figure is provisional and is not a measurement.

### Numeric spacing (Q10, user answer 2026-10-23)

Shared doc-hidden helper, re-exported from `rdocx` so the Python crate and the native paragraph setter share one conversion. No new module.

```rust
#[doc(hidden)]
pub fn auto_line_spacing(multiple: f64) -> Twips;
```

Python default, paragraph and style writes use one rule. Test `Length` before `int`, because `Length` subclasses `int`. A `Length` is exact spacing. A non-bool `int` or `float` is a line multiple. `None` clears the line value and the rule. An omitted argument preserves both. Bool and other types raise `TypeError`. A Python multiple must be finite. Non-finite values raise `ValueError`. A huge Python `int` whose numeric conversion overflows `f64` raises `ValueError`, not `OverflowError`. A keyword batch is atomic: one bad field writes nothing from that call.

The Python domain and the native helper domain are separate. Native `set_line_spacing_multiple` keeps an infallible `f64` setter. It retains the Rust cast, including NaN becoming 0 and saturation through `as i32`. Do not claim the Python refusals on that native setter. `paragraph.rs:1613` may share `auto_line_spacing` on that saturating native path. Python calls the helper only after the domain check below.

A finite Python multiple is accepted in exactly two ways. If the input is the exact represented stored grid `(n as f64) / 240.0` for an `n` inside signed `i32`, normalize back to `Twips(n)` even when the raw binary product `multiple * 240.0` is one ulp away from `n`. Otherwise require the raw scaled product `multiple * 240.0` to lie inside the closed range from `i32::MIN` through `i32::MAX` before toward-zero truncation, and only then truncate toward zero. The next representable float above `i32::MAX as f64 / 240.0`, and the next representable float below `i32::MIN as f64 / 240.0`, are refused in Python even though toward-zero truncation of those products would fit in signed `i32`.

A pure Python probe of `n / 240 * 240` for `n` in `-2147483648`, `-4800`, `-123`, `123`, `259`, `4800`, and `2147483647` is design evidence only. It is not a product RED or GREEN run. In that probe, `-123` and `123` produce `122.99999999999999` and `-122.99999999999999`. The exact-grid rule still recovers `123` and `-123`. `-4800`, `259`, `4800`, `-2147483648`, and `2147483647` multiply back to themselves and stay on the stored grid. The next representable neighbours are refused by the raw-bound rule even though truncation would fit. Off-grid truncation stays toward zero only after that raw product is inside the closed range. `1.0021 * 240` is `240.504`, which is inside the closed range and is not an exact `n / 240.0`, so it becomes `240`. A negative off-grid control truncates toward zero as well. No second public helper is added. Strict `bool` and other non-numeric types stay `TypeError`.

Unknown and `atLeast` imports stay untouched. An explicit `Length` writes exact spacing and does not invent an `atLeast` authoring API. A no-op that writes the same represented `n/240` preserves an absent `lineRule`. The old rule that a number always writes `auto` does not apply to that no-op. A new numeric value writes `auto`. A `Length` writes exact. Float and int of the same value are a no-op under the absent-rule rule. Native and Python boundary tests cover defaults, style, and paragraph saved XML, including the absent `lineRule` byte round-trip, the `123` and `-123` exact identities, `1.0021`, both signed `i32` endpoints, and the next representable value above and below those endpoints. Python refuses those neighbours even though toward-zero truncation would fit.

`Document.set_default_paragraph_format` keeps the existing omitted-sentinel shape. Only the `line_spacing` keyword and property change. Default `space_before` and `space_after` stay `Length` only. The live stub is `set_default_paragraph_format(self, *, space_before: Length | None = ..., space_after: Length | None = ..., line_spacing: Length | None = ...) -> None` in `crates/rdocx-py/python/rdocx/_rdocx.pyi`. Do not clone the rest of the formatting API. The changed `line_spacing` annotation becomes `Length | float | None` on the default, paragraph and style surfaces that already take a number. Paragraph and style signatures stay the existing ones aside from that keyword and property.

### Comment projection (Q11)

Keep the old doc-hidden signature exactly:

```rust
#[doc(hidden)]
pub fn comment_anchor_snapshots(
    &self,
) -> Result<BTreeMap<i32, (Option<StoryRunRange>, Option<String>)>>;
```

Add a sibling. Do not rename the old method.

```rust
#[doc(hidden)]
pub fn comment_anchor_snapshot_results(
    &self,
) -> Result<BTreeMap<i32, Result<(Option<StoryRunRange>, Option<String>)>>>;
```

One authoritative projection body serves both. The old method is a strict collect wrapper over the new helper. Document-wide inventory errors stay the outer `Err`. A per-record projection or Python endpoint that cannot be represented is an inner `Err` with an identified diagnostic. Other comments stay listed.

Accepted-view text is one enclosing non-cell physical flow, including table cells in that flow, joined by newlines. An independent nested text box is a separate flow and is excluded. Each comment exposes readonly `anchor_issue` when its projection or Python endpoint is unrepresentable. The snapshot copies the existing seven metadata fields. Derived anchors and `anchor_issue` are outside that equality. Creation and movement use strict destination ownership.

The existing exported frozen class is `Comment`, not `FrozenComment`. The live stub spells the constructor `__new__`. The reconciled keyword contract is one constructor:

```python
class Comment:  # frozen, existing export
    def __new__(
        cls,
        *,
        id: int,
        author: str | None,
        initials: str | None,
        date: str | None,
        text: str,
        parent_id: int | None,
        resolved: bool | None,
        anchor_text: str | None = None,
        anchor: StoryRunRange | None = None,
        anchor_issue: str | None = None,
    ) -> Comment: ...
```

That block is the live stub constructor, which is `__new__` and returns `Comment`, plus the additive `anchor_issue` keyword. Do not also declare `__init__`. A runtime call stays `Comment(...)`. Do not add a second class. CLI JSON adds nullable `anchor_issue`. Plain metadata text is unchanged. Python and the CLI call the new helper.

`Document.add_comment(range: RunRange | StoryRunRange, *, author: str, text: str, initials: str | None = None, date: str | None = None) -> int` stays. `Document.move_comment(id: int, range: StoryRunRange) -> None` stays. Do not pass `RunRange` to `move_comment`. Creating a comment retires conservative old handles. Movement reacquires the current destination after that retirement. There is no new Python comment-move signature.

### Package view (Q12)

Existing signatures stay and become the save-equivalent view. Native `Document::part_names(&self) -> Result<Vec<String>>` and `Document::part_xml(&self, part_name: &str) -> Result<String>` live at `document.rs`. Python `part_names(self) -> tuple[str, ...]` and `part_xml(self, name: str) -> str` live in `_rdocx.pyi`. Published `part_xml` is a read-only output view. It is not a public raw XML input API and does not earn a raw XML input rider. That includes root rels, main document rels, header rels, and `[Content_Types].xml`, using the exact entry XML `to_bytes` would write. The same full serialization validation failures apply even when an unrelated part is invalid. This is an explicit compatibility change.

Additive OPC method. `write_to` uses the same entry owner.

```rust
pub fn zip_entries(&self) -> Result<Vec<(String, Cow<'_, [u8]>)>>;
```

Names are the exact entry-name strings the existing writer emits, with no additional path normalization. For ordinary parts, the existing mapping removes at most one leading slash from the first authored stored key. A canonical `/word/a.xml` becomes `word/a.xml`, while caller-authored `//word/a.xml` retains `/word/a.xml`. Metadata and relationship names use the existing serializer mappings. Order is content types, root rels, owner-sorted rels, then name-sorted parts. The method owns metadata bytes and borrows normal part bytes. Normal name sorting, slash, case, optional slash, UTF-8 BOM, NUL, binary, and missing-part semantics stay. Path save extension conversion is not `to_bytes`.

### Behaviour commits that are in scope and not silently claimed

These are separate labelled behaviour commits when they change output. Evidence is pending.

- A neighboring self-closing `<w:p/>` in a header, footer, footnote or endnote inventory is the field case. It is not a self-closing `w:fldChar`. Endnote `PAGE` output is its own commit. Header and footer cache Word policy stays unchanged.
- Body range text across a table includes `before`, newline, `A`, newline, `B`, newline, `after`. That visible change is a separate declared acceptance.
- `PAGEREF` to a related bookmark in another owner, and a generated table page range across cells, stay a named limitation outside the gate. They get no acceptance credit and no passing test that asserts the bug. Issue 401 and an original 252 case are not manufactured into this row.

## Consumer map

| Rule | Entry points and outputs | Shared provider and caller policy | Lifecycle stages compared |
|---|---|---|---|
| Line multiple | `rdocx::auto_line_spacing`, Python default, paragraph and style setters, native `set_line_spacing_multiple` | Python refuses non-finite and out-of-range values. Native f64 stays infallible | Assign, serialize, reopen |
| Exact length | Python `Length` before `int` on the same three surfaces | Explicit `Length` writes `exact`. Import of `atLeast` stays | Assign, serialize |
| Comment list | `comment_anchor_snapshot_results` and strict `comment_anchor_snapshots` | One body. Old method collects strictly. Python `Comment` and CLI JSON | Read, list, save |
| Anchor move | Existing comment move and `RunRange` flush | Strict destination ownership. No new Python move signature | Update, serialize |
| Package view | `Document::part_names`, `Document::part_xml`, `OpcPackage::zip_entries`, `write_to` | Same entry owner. Validation failures are not narrowed | Serialize |
| Story fields | Existing evaluate, update and layout | Self-closing paragraph field in header, footer, footnote and endnote. Namespace before `/>` | Evaluate, update, layout, serialize |

## Behavior contracts and evidence

| Surface or placement | Exact behavior and boundary examples | Primary source identity and location | Local discriminator or pending evidence |
|---|---|---|---|
| Python multiple | `1` and `1.0` write 240 twips and rule `auto`. `1.0021` writes 240. Bool is `TypeError`. NaN and infinity are `ValueError` | Base style branch `document.rs:2998`. Policy is Q10, not the donor | `line_spacing_length_is_exact_before_int` |
| Python length | `Length` exact. `None` clears line and rule. Omitted preserves. Batch with one bad keyword writes nothing | Same three Python surfaces | `line_spacing_batch_is_atomic` |
| Native multiple | `auto_line_spacing(1.0)` is `Twips(240)`. NaN still follows the existing cast | `paragraph.rs:1613` | `auto_line_spacing_matches_native_cast` |
| Comment text | Cells `A` and `B` join as accepted view with newlines in physical paragraph order | `comments.rs:2650`. Donor PR412 is a candidate | `cross_cell_comment_lists_accepted_text` |
| Bad anchor | One unrepresentable endpoint sets `anchor_issue` and stays in the tolerant map. Strict wrapper returns `Err` | Q11 | `mixed_comment_anchors_keep_good_rows` |
| Parts | `[Content_Types].xml` and root rels match `to_bytes` entry bytes. An invalid unrelated part still fails the call | `document.rs:29895`, `package.rs:282` | `part_xml_matches_zip_entry` |
| Field | A neighboring self-closing `w:p` in a header, footer, footnote, or endnote story evaluates. This row is not a self-closing `w:fldChar` fix. Header cache policy stays | Existing field writer. Endnote `PAGE` output change is its own commit | `self_closing_story_paragraph_fields_evaluate` |
| Same-part range omission | Exactly one begin and one end in valid physical order may be omitted. Reversed and crossing orders stay errors. The inherited `f_x264_cross_owner_fixture` order is the crossing negative. The positive moves `intoEnd` 3 before `acrossStart` 1 and `permissionStart` 0. Order uses matched unreplayed source spans plus the closed XML namespace insertion delta | Q11 and A4, user answer 2026-10-23, fixture clarification 2026-10-24. PR412 composition is the negative input, not authority for a positive | `cross_cell_range_stays_in_one_owner` and `story_range_order_rejects_reversed_and_crossing_omissions` |

Patch pins, not executed: PR414 `3d3d42db0f9c` sha256 `f5dd5f17cb620204e50f84532ddf435668b696d9edc058e4b35a3bcbc858ee8a`. PR412 `38fefb1730b4` sha256 `009c09e118c683d1e03a4ba0223f402af8ac07d3033af4f00014a0cb1f5b2a24`. PR409 `7df1c7e0` sha256 `e03759be4b96a544afa097778c427472faf22c2d6975f14e201c6ebcd7e8d487`. Receipt `/private/tmp/s95.4-contributions/patch-receipt.json`.

## Dependency readiness

`docs/sprints/BACKLOG.md` records F-X260 and F-X261 done. F-X264 is pending and unclaimed. This draft does not rely on an unfinished provider contract. Approval would not start the story. `/start-feature` still requires those two providers done on the worktree base.

## Acceptance criteria

| ID | Input or setup | Expected observation or refusal | Consumers | Local test | Final gate or pending evidence |
|---|---|---|---|---|---|
| A1 | Issue 402. Python default, paragraph and style. Boundaries 0, 1, 1.0021, 123, -123, 259, negative off-grid, signed `i32` endpoints, the next representable float above `i32::MAX / 240` and below `i32::MIN / 240`, `Length`, plain int, float, bool, None, omitted, NaN, and a huge `int` | `Length` exact. Non-bool int and float are multiples and must be finite. None clears. Omitted preserves. Bool and other non-numeric types are `TypeError`. Non-finite input, a raw scaled product outside the closed signed `i32` range before truncation, the next representable neighbours even though truncation would fit, or a numeric `int` whose conversion overflows `f64`, is `ValueError`. An exact stored-grid `n/240` normalizes back to `n` even when the raw product is one ulp from `n`, including `123` and both signed endpoints. `1.0021` truncates to 240 only after the raw product is inside the closed range. Batch atomic. Same represented value preserves an absent `lineRule` in saved XML. Native saturation stays a separate helper domain | Python three surfaces and `auto_line_spacing` | `line_spacing_length_is_exact_before_int`, `line_spacing_batch_is_atomic`, `auto_line_spacing_matches_native_cast`, `line_spacing_grid_boundaries_round_trip_without_line_rule` | Installed typing pending. Native cast is not a Python refusal. The Python math probe is not this gate |
| A2 | Issue 405. Plain comment and Google Docs SDT across cells. Mixed bad anchors. Removal. Existing `Comment`. Retained-run content edits `Run.text`, `Run.add_tab`, and `Run.add_break` inside the anchor | Listed text is newline-joined physical order inside one enclosing non-cell flow. An independent nested text box is excluded. Bad rows keep `anchor_issue`. Others remain. Removal of a listed comment works. Existing seven-metadata equality ignores derived anchors and `anchor_issue`. A retained-run tab or break keeps the owner and may keep revision metadata, and the fresh anchor text follows the edited accepted text. A held frozen comment keeps the old text | Python list, CLI JSON, strict and tolerant snapshots | `test_cross_cell_comment_lists_accepted_text`, `test_mixed_comment_anchors_keep_good_rows`, `test_comment_anchor_fields_preserve_constructor_compatibility`, `test_comment_anchor_text_tracks_retained_run_edits` in `crates/rdocx-py/tests/test_core.py`. The constructor test is the existing evidence for the logical seven-metadata case that ignores `anchor_issue`. That name is an evidence mapping, not a new API or a new test | Local regression. No new Word comment oracle |
| A3 | Issue 406. Root, main, header rels and content types. Invalid unrelated part. Slash, case, BOM, NUL, binary, missing | Entry XML equals `to_bytes`. Same validation failure. Path extension conversion is not this API | Native `part_names` and `part_xml`, Python `part_names` and `part_xml`, `zip_entries` | Native `part_xml_matches_zip_entry`. Python `test_part_xml_matches_zip_entry` and `test_part_xml_keeps_slash_case_and_binary_errors` in `crates/rdocx-py/tests/test_core.py` | Hash delta, if any, is this compatibility commit |
| A4 | `StoryRunRange` read across a cell inside one physical flow. Placement is one owner. Same-part cross-owner omission requires exactly one begin and one end in valid physical order. Cross-part, duplicate, unmatched, reversed and crossing pairs stay errors. The inherited auxiliary `f_x264_cross_owner_fixture` order is that crossing and is a negative, not a positive. A distinct well-nested positive moves `intoEnd` 3 to cell 0 before `acrossStart` 1 and `permissionStart` 0. Document ownership is not weakened | That one valid pair is omitted from `story_ranges`. Names and ids stay occupied. Reversed and crossing pairs still error. The original crossing composition is refused atomically and is not retired | Existing range readers | Native `cross_cell_range_stays_in_one_owner` and `story_range_order_rejects_reversed_and_crossing_omissions` | Donor extras are this A-ID, not a silent extra feature. The old RED of the crossing fixture is not positive contract proof |
| A5 | Cross-owner bookmark remove and comment move | Atomic refusal. Destination unchanged | Existing remove and move | `cross_owner_bookmark_and_comment_move_refuse` | No new Python move signature |
| A6 | Ordinary `REF` whose related target crosses story owners. Separate `REF \f`. A valid primary body bookmark beside an unrelated invalid checked related pair | The ordinary case keeps the old cache and the diagnostic `runs across story owners`. It is not a resolved cache hit. `REF \f` still refuses on its own. An invalid checked related inventory prevents only new related admission. It does not clear a valid primary body bookmark map. Existing name uniqueness still applies even when marker scan or pairing fails, if paragraph projection succeeded. `BodyKeep` is `ALPHA` with a body `REF` whose cache is the old value. Unrelated header `BadRelated` is a reverse pair whose ref cache is old. `BodyKeep` still resolves and updates `ALPHA`. `BadRelated` retains the old not-found diagnostic. The duplicate-name body and header control is the same not-found and old-cache observation. That preservation is existing behavior, not a new public choice | Existing field update | Native `ref_cross_owner_keeps_old_cache` proves the ordinary related `REF`, separate `REF \f`, and update and reopen. `story_range_order_rejects_reversed_and_crossing_omissions` proves `BodyKeep` equal to `ALPHA` beside invalid `BadRelated`, and the same duplicate-name body and header not-found and old-cache control | Not a new evaluator |
| A7 | `RunRange`-created comment, then move | Model flush through the existing route | Native move | `run_range_comment_move_flushes_model` | Python move signature unchanged |
| A8 | Neighboring self-closing `<w:p/>` in header, footer, footnote and endnote inventory. Not a self-closing `w:fldChar`. The original seed footnote stays the unreferenced negative control. A separate fresh clone of that seed appends the same note id through `paragraph_mut(0).add_footnote_ref(footnote_id)`. Three shapes times the original four stories are 12 retained cases, and the three referenced-footnote positives make 15 observations. No second note is added | Evaluation is one `PAGE` row with `FieldOutcome::DeferredPagination` and cached `9`. A visible ordinary `update_fields` returns `1`, marks `dirty` true, and keeps cached `9`. That update may change source metadata or bytes while the result stays deferred. There is no ordinary byte-no-op promise. Pure evaluation and layout reads still preserve bytes. The unreferenced footnote has no layout placement, layout-backed `page_fields` `0`, cached `9`, and the expected unplaced-source diagnostic. Each referenced footnote, header, footer, and endnote positive has one placement on physical page 1 and displayed page 1, the correct owner, and direct child paths `[2]`, `[3]`, and `[2]` for the three shapes. Layout-backed header and footer keep `page_fields` `0`, cached `9`, and the retention diagnostic. Placed footnote and endnote have `page_fields` `1` and cached `1`. Save and reopen verify the cache, the dirty state where it applies, and either the placement or the deliberate non-placement. Existing header and footer cache policy is unchanged. Endnote `PAGE` output remains its own labelled commit | Field consumers | Native `self_closing_story_paragraph_fields_evaluate` | Local cache and layout observations are required before that labelled endnote `PAGE` commit. Hash-corpus, native Word, and the full gate remain pending. The count-1 failure in `/tmp/fx264-a8-exact.txt` is a design and harness failure, not a product semantic RED. This revision does not claim that any test newly ran |
| A9 | Body range across table text | Visible text is before, newline, A, newline, B, newline, after | Range text | `body_range_across_table_includes_cell_text` | Declared acceptance. Not a silent digest copy |
| A10 | `PAGEREF` whose bookmark is in another owner, and a generated table page range across cells | Named limitation outside the gate. No delivery credit. No test asserts the bug or an intermediate state | Field and TOC readers | None. The removed row `cross_owner_pageref_and_table_page_range_stay_open` is not a gate | Known non-fix |

Issue rows 402, 405 and 406 map to A1, A2 and A3. Donor extras in the PR412 patch map to A4 through A10. Unreduced inputs stay these cases. No reduced substitute retires an original row.

## Test matrix

| Case | Modes or state combined | Expected difference or agreement | Wrong implementation rejected |
|---|---|---|---|
| C1 | `Length(1)` versus int `1` versus float `1.0` on style, paragraph and default | Length is exact. Numbers are 240 twips auto | Float-before-Length fails |
| C2 | None, omitted, bool, NaN, and a second valid keyword in one call | None clears. Omitted preserves. Bool and NaN refuse with no write | A partial batch fails |
| C3 | One good comment and one unrepresentable endpoint, then removal of the good id | Tolerant list keeps both. Strict map errors. Removal drops only the targeted id | A donor rename of the old snapshot fails |
| C4 | Content types plus an unrelated invalid part | Both `part_xml` and `zip_entries` fail the same way | A best-effort XML read fails |
| C5 | Bookmark pair that crosses owners, and a cell-local pair | Cross-owner pair is absent from `story_ranges`. Cell pair remains | Treating omission as deletion of the name fails |
| C6 | Self-closing endnote `PAGE` versus an ordinary header `PAGE` | Endnote output may change in its own commit. Header cache policy does not | One shared cache rewrite fails |
| C7 | Reversed pairs in every identified family. Two omitted markers that cross, plus an omitted-local pair. Positive cell-to-body, body-to-cell and nested-table pairs. Nested text box interleaving with proper complete pairs. A namespace-closure offset delta. The field alternate caller | Same validity policy on every caller. Only one begin and one end in valid physical order is omitted. Reversed and crossing omissions error. The inherited `f_x264_cross_owner_fixture` order stays the negative. The moved `intoEnd` 3 order is the distinct positive | Owner-grouped paragraph order, post-namespace replay offsets, or treating the crossing fixture as a positive pass fails |

Omitted: a new `atLeast` authoring API, a new Python move, and a claim that `PAGEREF` or generated table page ranges are fixed.

## Rejected alternatives

- Changing the old `comment_anchor_snapshots` return type. Q11 keeps it and adds the tolerant method.
- Making the native f64 setter fallible. Q10 keeps the cast and puts refusals on Python.
- A new public module for `auto_line_spacing`.
- Treating path-save extension conversion as `part_xml`.

## Test plan

Backlog test gate, verbatim: regression.

| Timing | Category | Test | Asserts |
|---|---|---|---|
| local scoped | regression | Native `auto_line_spacing_matches_native_cast` and `line_spacing_grid_boundaries_round_trip_without_line_rule`. Python `test_line_spacing_length_is_exact_before_int`, `test_line_spacing_batch_is_atomic`, and `test_line_spacing_grid_boundaries_round_trip_without_line_rule` in `crates/rdocx-py/tests/test_core.py` | A1. Exact stored-grid `n/240` normalizes back to `n`, including `123`, `-123`, and both signed `i32` endpoints, even when the raw product is one ulp from `n`. Other finite values require the raw scaled product inside the closed `i32` range before toward-zero truncation. The next representable neighbours refuse in Python even though truncation would fit. `1.0021` stays 240. Same-value no-op preserves an absent `lineRule` in default, style, and paragraph saved XML. Huge `int` numeric conversion overflow is `ValueError`. Bool and other non-numeric types are `TypeError`. Native saturation is a separate domain |
| local scoped | regression | `test_cross_cell_comment_lists_accepted_text`, `test_mixed_comment_anchors_keep_good_rows`, `test_comment_anchor_fields_preserve_constructor_compatibility`, `test_comment_anchor_text_tracks_retained_run_edits` | A2. Class name is `Comment`. The constructor test is the existing evidence mapping for seven-metadata equality that ignores `anchor_issue`. It adds no API and no test. The retained-run test uses a real `add_tab` or `add_break` inside the retained range. It compares a fresh native projection, a held frozen comment that still has the old text, a fresh native and Python list, and save and reopen. Physical flow and per-record diagnostics stay. No new oracle |
| local scoped | regression | Native `part_xml_matches_zip_entry`. Python `test_part_xml_matches_zip_entry` and `test_part_xml_keeps_slash_case_and_binary_errors` | A3 |
| local scoped | regression | Native `cross_cell_range_stays_in_one_owner`, `story_range_order_rejects_reversed_and_crossing_omissions`, `cross_owner_bookmark_and_comment_move_refuse`, `ref_cross_owner_keeps_old_cache`, `run_range_comment_move_flushes_model`, `self_closing_story_paragraph_fields_evaluate`, `body_range_across_table_includes_cell_text` | A4 through A9. A8 keeps the original unreferenced footnote as the negative control and adds a separate clone that references that same note id. The matrix is 12 retained cases plus 3 referenced-footnote positives. Evaluation is one `PAGE` row with `FieldOutcome::DeferredPagination` and cached `9`. Visible ordinary `update_fields` returns `1` with `dirty` true and cached `9`, and it is not a byte no-op. Pure evaluation and layout reads preserve bytes. The unreferenced footnote has no placement. The referenced footnote, header, footer, and endnote positives place page 1 on paths `[2]`, `[3]`, and `[2]`. A10 has no test. `story_range_order_rejects_reversed_and_crossing_omissions` sits beside `cross_cell_range_stays_in_one_owner`. It covers reversed pairs in every identified family, two omitted markers that cross plus an omitted-local pair, positive cell-to-body, body-to-cell and nested-table pairs, physical nested text box interleaving with proper complete pairs, a namespace-closure offset delta, and the same validity policy for the field alternate caller. The inherited crossing composition is the atomic negative. The distinct well-nested move of `intoEnd` 3 is the positive. Completion still needs an actual old F-ID Base RED of that corrected positive. `ref_cross_owner_keeps_old_cache` proves an ordinary related `REF`, a separate `REF \f`, and update and reopen. `story_range_order_rejects_reversed_and_crossing_omissions` proves `BodyKeep` equal to `ALPHA` beside invalid `BadRelated`, and the same duplicate-name body and header not-found and old-cache control. Existing name uniqueness still applies when marker scan or pairing fails if paragraph projection succeeded. That preservation is existing behavior, not a new public choice |
| final sprint | risk rider | `/verify --full`, hash harness, installed typing, stub and package dry-run, WASM check, native Word forms | Union once. The contract is approved. This gate is unrun and stays pending |

Tests join the existing integration entrypoint and `crates/rdocx-py/tests/test_core.py`. Typing uses existing `crates/rdocx-py/tests/typing_smoke.py`. No new test binary.

## HLD impact

- `docs/hld/03-architecture.md`, Owned publication and causal paths
- `docs/hld/04-opc-and-packaging.md`, The package, and Part naming
- `docs/hld/10-bindings-spec.md`, Python Word formatting and checked list publication
- `docs/hld/12-testing-strategy.md`, Checked Word comment anchor projections
- `docs/hld/14-development-backlog.md`, F-X264 definition content, and the shared staged-package and default-format wording in the completed F-X261 definition. Completion reconciles that definition's obsolete "A float line spacing refuses" statement to the Q10 current composed policy. F-X261 historical status and delivery evidence stay unchanged. Status is not the only edit at completion

## Risk routing

Matched rows. Extra checks run once in the sprint union.

- Unit conversion. Read the glossary units and the pinned truncation rule. Declare any harness delta for the endnote `PAGE` commit and the table-range text commit separately.
- Parser or serialiser. `zip_entries` and `write_to` share one owner. Unmodelled part bytes stay borrowed.
- Public API. Semver note for `zip_entries` and the Python spacing compatibility change. `part_names` and `part_xml` signatures stay. Publish dry-run only in the final union.
- PyO3 and WASM. Workspace tests keep the binding excludes. WASM check is final-only. No new WASM authoring method.
- Compatibility and numeric conversion. Field-specific examples in A1. Native and Python policies stay distinct.
- New projection. Comment absence, orphan, reference-only point, and unrepresentable endpoint.
- Cached facts. Comment snapshots flush before projection. Publishers include comment create, remove, and move, and anchored content edits `Run.text`, `Run.add_tab`, and `Run.add_break`. Those content edits can preserve the owner and sometimes the revision while changing accepted anchor text. Header and footer field cache policy is unchanged.
- XML in multiple parents. Comment anchors in body cells. Fields in header, footer, footnote and endnote.
- Published `part_xml` is read-only producer output. It does not earn a raw XML input rider.
- Shared consumers. Python, CLI JSON, and native snapshots for one comment body.
- Contributed criteria. Pins above. Donor README timings are not evidence.
- Performance row is not earned here.

Sprint union also includes the other six stories' rows: full verify, hash, installed typing, stub, package dry-run within the archive cap, native Word forms, and the PowerPoint digest review owned by F-X267.

## Hash harness

Expected hash-corpus deltas remain final and unrun. The self-closing endnote `PAGE` commit and the body-range-across-table commit are separate labelled behaviour commits. They are not folded into a file move. Existing fixtures that do not observe those outputs stay unchanged. Rendering baselines stay on deterministic fonts. No baseline is recorded in this approval.

## Implementation checklist

- [x] Add `auto_line_spacing` in existing `paragraph.rs` and re-export it.
- [x] Point the Python default, paragraph and style writers at the Q10 rule.
- [x] Add `comment_anchor_snapshot_results` beside the old strict method.
- [x] Add readonly `anchor_issue` on the frozen comment and CLI JSON.
- [x] Add `zip_entries` and point `part_names` and `part_xml` at the same entry owner.
- [x] Land the named tests in existing entrypoints.
- [x] Qualify the required native Word and final-union evidence while retaining A10 as the explicit unsupported limitation outside this gate.

## Open questions

| ID | Affected F-IDs and A-IDs | Decision and recommendation | Status and answer/source |
|---|---|---|---|
| Q10 | F-X264 A1 | Consistent numeric spacing across Python default, paragraph and style. | Answered 2026-10-23 by user answer. Test `Length` before `int`. `Length` is exact. Non-bool int and float are multiples and must be finite. `None` clears line and rule. Omitted preserves. Bool or wrong type is `TypeError`. Non-finite input, a raw scaled product outside the closed signed `i32` range before toward-zero truncation, and a numeric `int` whose conversion overflows `f64` are `ValueError` before any saturating cast. An exact stored-grid `n/240` for signed `i32` `n` normalizes back to `n` even when the raw product is one ulp away from `n`. The next representable float above `2147483647/240` and below `-2147483648/240` is refused in Python even though truncation would fit. Off-grid values truncate toward zero only after the raw product is inside that closed range, so `1.0021` becomes 240, with a negative control. Unknown and `atLeast` imports stay untouched until an explicit new value. A same-value no-op preserves an absent `lineRule`. A new numeric value writes `auto`. Explicit `Length` is exact and there is no new `atLeast` authoring API. The style writer already handled `Length`. The defect is plain int as EMU versus float multiple, default `Length` only, and on-grid loss. The native saturating helper domain stays separate. Do not claim Python refusals on native. Shared doc-hidden `rdocx::auto_line_spacing(f64) -> Twips` serves the Python crate and the native paragraph setter. No new module and no second public helper. The pure Python probe is not a product gate. |
| Q11 | F-X264 A2, A4, A5, A7 | Cross-cell comment text and a tolerant snapshot beside the old strict map. | Answered 2026-10-23 by user answer. Accepted-view text is one enclosing non-cell flow, newline-joined, including table cells in that flow. An independent nested text box is excluded. Per-comment readonly `anchor_issue` when the projection or Python endpoint is unrepresentable. Other comments stay listed. Creation and movement use strict destination ownership. Keep `comment_anchor_snapshots` as `Result<BTreeMap<i32, (Option<StoryRunRange>, Option<String>)>>`. Add `comment_anchor_snapshot_results` with the same outer `Result` and a per-record inner `Result`. One projection body. The old method is the strict collect wrapper. Python and CLI use the new helper. The existing frozen class is `Comment`, not `FrozenComment`. The live stub constructor is `__new__(cls, *, ... ) -> Comment`. It gains `anchor_issue: str | None = None` beside the existing nine keywords. Do not also declare `__init__`. A runtime call stays `Comment(...)`. Existing seven-metadata equality excludes derived anchors and the issue. CLI JSON adds nullable `anchor_issue`. Document-wide inventory errors stay the outer `Err`. `move_comment` still takes `StoryRunRange` only. |
| Q12 | F-X264 A3 | Save-equivalent `part_names` and `part_xml`. | Answered 2026-10-23 by user answer. The view includes root, main and header rels plus content types, as exact `to_bytes` entry XML. The same full serialization failures apply even for an unrelated invalid part. Existing Rust and Python signatures stay. Additive `zip_entries(&self) -> Result<Vec<(String, Cow<'_, [u8]>)>>` returns the exact entry-name strings the existing writer emits, with no additional path normalization. For ordinary parts, the existing mapping removes at most one leading slash from the first authored stored key. A canonical `/word/a.xml` becomes `word/a.xml`, while caller-authored `//word/a.xml` retains `/word/a.xml`. Metadata and relationship names use the existing serializer mappings. Order is content types, root rels, owner-sorted rels, then name-sorted parts. It owns metadata and borrows normal bytes. `write_to` uses that entry owner. Keep slash, case, optional slash, UTF-8 BOM, NUL, binary and missing semantics. Path-save extension conversion is not `to_bytes`. |

Q16 is answered on F-290 only. Provenance is the 2026-10-23 user choice "Author Word-compatible canonical slots (Recommended)". It is not a decision of this story. Q9 through Q15 stay answered where they already apply.

No unanswered user choice remains on this story.

## Physical range validity clarification (2026-10-24)

This is a contract and fixture clarification. It preserves the already approved strict A4 policy. It is not a source fix and it does not claim that any test passes. Status stays approved. Q10, Q11 and Q12 stay as answered. Scope, exclusions and the HLD impact file list stay unchanged. The canonical sprint head for this note is `380da336765f98eeafde12ce9b577a6322ecc40e` on `sprint/s95.4`. The stopped Grok F-X264 worker is preserved dirty and is not edited here. `/implement-feature` must acknowledge this revision before that worker continues.

The current auxiliary `f_x264_cross_owner_fixture`, inherited from PR412, composes body `intoStart` (id 3), cell 0 `acrossStart` (id 1), `permissionStart` 0, text `A`, `intoEnd` 3, and later cell 1 `permissionEnd` 0 with `acrossEnd` 1. Under the already approved strict A4 policy that order is a crossing. It cannot be a positive qualification case.

Keep that original composition as the actual negative case. Inventory and authoring refuse it atomically. Do not drop or retire the fixture. Add a distinct valid positive variant by moving `intoEnd` 3 to cell 0 before `acrossStart` 1 and `permissionStart` 0. After that move, `into` remains a body-to-cell cross-owner pair, `across` still covers `A\nB`, the permission stays properly nested, and the names and ids stay occupied. This is not a reduced-equivalence claim. The original composition stays the negative. The distinct well-nested positive proves only the valid subset.

Physical order uses matched unreplayed source spans plus the closed XML namespace insertion delta. It does not use owner-grouped paragraph order or post-namespace replay offsets. Private helper interfaces may adapt. Public doc-hidden `story_range_paragraphs` and the old strict and new tolerant snapshot signatures stay unchanged. One physical stack validates every admitted marker, including omitted markers, before materialization. Output owner ordinals and order stay. The hidden marker policy and the proofing boundary stay. No new public API, module or parser is added.

The prior original A4 baseline positive fixture contained this crossing, so its old RED alone is not valid positive contract proof. Completion requires an actual old F-ID Base RED for the corrected valid positive. Use the existing test entrypoint in a controlled temporary verification-only checkout when that checkout is needed. Retain the old log honestly, with the same patch and input identities. A compile failure from a missing new helper is not that RED. The current guard negative RED must be actual new-code behavior, not a fixture defect.

The existing successful legacy projection of a body `REF` stays. An invalid checked related inventory prevents only new related admission. It does not clear a valid primary body bookmark map. The existing name uniqueness filter still applies. The concrete preservation is `BodyKeep` equal to `ALPHA` plus a body `REF` whose cache is the old value, beside an unrelated header `BadRelated` reverse pair whose ref cache is old. `BodyKeep` still resolves and updates `ALPHA`. `BadRelated` retains the old not-found diagnostic. That is existing behavior preserved, not a new policy.

## Composed HLD and local evidence clarification (2026-10-24)

This records one bounded clarification of the approved contract. Status stays approved. That clarification left the implementation checklist unticked. Later local preparation ticks only implementation items present on this worker. Final debt stays unticked. Q10, Q11 and Q12 stay as answered. No source fix is claimed by this clarification.

HLD completion still edits `docs/hld/14-development-backlog.md`. That edit includes F-X264 definition content and the shared staged-package and default-format wording inside the completed F-X261 definition. The obsolete statement "A float line spacing refuses" is reconciled to the Q10 current composed policy. F-X261 historical status and delivery evidence stay unchanged.

`ref_cross_owner_keeps_old_cache` proves the ordinary related `REF`, separate `REF \f`, and update and reopen. `story_range_order_rejects_reversed_and_crossing_omissions` proves `BodyKeep` equal to `ALPHA` beside invalid `BadRelated`, and the same duplicate-name body and header not-found and old-cache control. Existing uniqueness still applies when marker scan or pairing fails if paragraph projection succeeded. This is existing behavior preservation, not a new public choice.

The actual native range-ready receipt `/private/tmp/s95.4-fx264-range-repair-ready.txt` is local slice evidence. It is not scoped verification, an independent microscope, or final acceptance. The canonical sprint head for this note is `41311cb957f7f8d48cd807451c2dc4b6d1a28803` on `sprint/s95.4`.

## Review pass 1 disposition (2026-10-24)

S1. `zip_entries` returns the exact entry-name strings the existing writer emits. There is no additional path normalization and no normalize-all policy. Ordinary parts drop at most one leading slash from the first authored stored key. Metadata and relationship names keep the existing serializer mappings.

S2. The existing native test `self_closing_story_paragraph_fields_evaluate` carries the A8 observations in the acceptance row. No new file or API is added.

This is a factual contract clarification. It is not a public API choice. Q10 and Q12 stay answered. Status stays approved. The implementation checklist stays unticked. Final native checks, the full gate, and the hash harness stay pending. This note does not claim that any test newly ran.

## Deferred field fixture correction (2026-10-24)

Two earlier A8 expectations are withdrawn. They were fixture assumptions, not a reason to change runtime behavior.

`update_fields_with_policy` at `crates/rdocx/src/field.rs:2578` maps `FieldOutcome::DeferredPagination` to a `CachedFieldUpdate` with the same `cached_result` and `dirty: true`. `apply_cached_field_updates` counts every `Some` update, so a visible `PAGE` ordinary update returns `1` and may change source metadata or bytes while the cache stays `9`. The earlier return-`0` and ordinary byte-no-op claims are withdrawn. Pure evaluation and layout reads still preserve bytes.

`add_footnote` at `crates/rdocx/src/document.rs:25276` returns an unreferenced note. The caller pairs it with `Paragraph::add_footnote_ref`. The original fixture `FNOTE` is unreferenced and correctly has no layout placement. That negative control stays. A separate fresh clone of the original seed may append that same note id through `paragraph_mut(0).add_footnote_ref(footnote_id)`. It does not add a second note and it does not change the original fixture. Three shapes times the original four stories are 12 retained cases. The three referenced-footnote positives make 15 observations.

`/tmp/fx264-a8-exact.txt` compiled and failed on count `1` for header, footer, and endnote. That failure is a design and harness failure, not a product semantic RED. S1 exact writer strings stay. Q10 and Q12 stay answered. Status stays approved and the checklist stays unticked. Final hash, native Word, and the full gate stay pending. This note does not claim that any test newly passed.

## Contract closure

Codex root semantic validation on 2026-10-23 approved this contract at source `a57be365`. The numeric rule is exact stored-grid rescue, and otherwise a raw closed `i32` product bound before truncation. The `Comment` constructor is `__new__` and returns `Comment`. Anchored run content edits publish accepted text. All evidence above is unexecuted at that approval. That sentence is approval-time history. Later local progress is in `.claude/scratch/F-X264-progress.md`, including the native range receipt and the scoped preparation batch. That record is not scoped completion. The local Python, CLI, and typing_smoke evidence is in `.claude/scratch/F-X264-progress.md`. The historical `test_fx256_line_spacing_distinguishes_multiples_and_lengths` row expected truncation of two multiples whose raw products sit outside the closed signed 32-bit range. Later local preparation aligned that expectation to the Q10 refusal. That alignment is recorded in progress and is not approval-time evidence. Final Word evidence and the full acceptance gate stay pending. This approval does not mark the story completed.

The 2026-10-24 physical range validity section clarifies fixtures and evidence only. It does not relax Q11 or A4, and it does not record a source fix or a passing run.

Operations: `auto_line_spacing` proves exact stored-grid rescue and, for every other finite multiple, the raw closed-range check before toward-zero truncation (A1). Python setters on default, paragraph and style prove assign, clear, refuse, saved XML, and an absent `lineRule` byte round-trip (A1). The pure Python probe is design arithmetic only. `comment_anchor_snapshot_results` and `comment_anchor_snapshots` prove listing (A2). `test_comment_anchor_text_tracks_retained_run_edits` proves anchored content edits. Existing remove and move prove A5 and A7. `part_names`, `part_xml` and `zip_entries` prove A3. Existing field evaluate, update and layout prove A6 and A8 through the neighboring self-closing paragraph case, not a `w:fldChar` fix. Existing range text proves A4 and A9. A10 is a named limitation outside the gate and has no passing test.

Kind coverage is limited to the named Python spacing types, comment anchor states, OPC entry classes, and the named field and range cases. There is no "all supported" promise.

Repeated signatures agree with Q10, Q11 and Q12. The donor PR412 rename of `comment_anchor_snapshots` is rejected.

Imported states: `atLeast` and unknown line rules are preserved until an explicit `Length` or multiple is assigned. Comment orphans and reference-only points stay on the existing strict contract. `anchor_issue` is additive.

Cached facts: comment projection reads a flushed package. Publishers are comment create, remove, and move, and anchored content edits through `Run.text`, `Run.add_tab`, and `Run.add_break`. Those content edits can preserve the owner and sometimes the revision while changing the accepted anchor text. A held frozen comment keeps its old text. A fresh native projection, a fresh native and Python list, and save and reopen agree after a real tab or break inside the retained range. Direct cache edits of unrelated field results do not change comment anchor text. Header and footer field cache policy is unchanged. The endnote `PAGE` commit is the only new field-output claim. Its local cache and layout observations are required before that commit, and its hash-corpus observation remains final and unrun. The field case is a neighboring self-closing `w:p`, and the discriminator is `self_closing_story_paragraph_fields_evaluate`.

`PAGEREF` across owners and generated table page ranges stay a named limitation under A10. They are not a gate row.
