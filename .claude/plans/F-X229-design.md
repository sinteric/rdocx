# F-X229, Repair comparison reconstruction correctness

**Status**: completed
**Sprint**: S92
**Size**: L
**Depends on**: F-X203

## Problem

Whole-paragraph serialization wraps hyperlink shells inside revision wrappers
(`crates/rdocx/src/comparison.rs:5490`), so supported hyperlink insertion and
deletion remain refused outside carried complex fields. The terminal paragraph
test only considers a following paragraph (`crates/rdocx/src/comparison.rs:3251`),
misclassifying changes before retained tables. Resolution also rejects a retained
table after a removed paragraph mark (`crates/rdocx/src/revision.rs:747`).

Granular property replacement does not consistently open a self-closing paragraph
(`crates/rdocx/src/comparison.rs:4532`). Trailing deletion lacks the existing
trailing insertion treatment (`crates/rdocx/src/comparison.rs:3236`). Positional gap
pairing matches unlike owners (`crates/rdocx/src/comparison.rs:6918`), turning a
neighboring cell edit into whole-table replacement. These are Issues368 through371.

## Spec reference

- `docs/hld/14-development-backlog.md`, "F-X229, Repair comparison reconstruction correctness".
- `docs/hld/03-architecture.md`, "What stays put", revision resolution and deterministic comparison paragraphs.
- `docs/hld/10-bindings-spec.md`, "Native Word facade stability", comparison staging, granularity and boundary ownership paragraphs.
- `docs/hld/12-testing-strategy.md`, "Test taxonomy", document-comparison regression, source-span round-trip and terminal paragraph coverage.
- `docs/hld/04-opc-and-packaging.md`, "Checked Word operation source ownership".

## Approach

Adapt the correctness subset of [PR374](https://github.com/tensorbee/rdocx/pull/374),
by `hadim`, assessed at head `3d7a3c9b703042b5ad53e586b94fe01eb843daf0`.
The source commits are `ca8372f8cd9adbb538f3c0af7a92fdf7823799bd`
(hyperlinks), `64b686118efd696144bc93146ae7d9ad2cac0e1b` (empty shells),
`4c509c27467e9af5364cee8a5757a63d8ea52328` (trailing deletions),
`313ae8f530a6ec1f05e334696372685c64895921` (table-adjacent marks), and
`f8ba1a3b58507b2b5fe77077dbfd9e87644533cc` (kind pairing). Exclude grouping
commit `7ebb5120aceee40d765a408faf19e1fa4bcc1724`, archive measurements and
unrelated README edits. Attribute reuse separately from F-X230.

Use only existing `comparison.rs`, `revision.rs` and `regression_test.rs` files.
No production file, module, trait, public API or dependency is added.

1. Place whole-paragraph insertion, deletion and move wrappers around hyperlink
   content inside its shell. Retain namespaces, relationships and marker order.
2. Remove a hyperlink during resolution only when selected revisions remove all
   its owned content. Keep previously empty hyperlinks and partly retained owners.
   Sequential scoped resolutions must not remove content prematurely.
3. Keep dedicated atomic refusals for empty whole-paragraph hyperlinks and simple
   fields outside a complex field carried whole. Preserve supported carried fields.
4. Build revised paragraphs from `paragraph_shell`, opening self-closing XML
   before inserting properties and runs. Share this logic between granular paths.
   A private helper may use this concrete signature:

   ```rust
   fn paragraph_with_properties(
       paragraph: &CT_P,
       original_source: Option<&str>,
       properties: &str,
   ) -> Result<String>;
   ```

5. Find trailing deletion chains before emitting body or block-control owners.
   Mark the retained predecessor once, delete intermediate paragraphs normally,
   and keep the terminal mark with correct accepted and rejected properties.
6. Identify the next owner retained on the relevant resolution side, crossing
   tables removed by that resolution. Use the changed paragraph's own mark before
   a retained paragraph or table. Keep terminal handling at actual terminators.
7. Remove an emptied paragraph before a retained table when resolving its removed
   mark. Permit proofing markers. Refuse retained content and opaque children
   rather than dropping them. Keep the entire operation atomic on failure.
8. Pair unmatched body and block-control owners by kind and in deterministic
   order, then compare retained tables in place. Remove obsolete expansion code.
   Use concrete existing alignment and unit types for new private helpers.

Preserve package-wide postconditions, unsupported formatting policy and all
existing story categories. An ignored formatting case remains left-biased.

## Rejected alternatives

- Import all of PR374, combining correctness, grouping and stale archive numbers.
- Add blank paragraphs before tables, changing input documents to avoid a bug.
- Relax reconstruction postconditions, hiding lost properties and relationships.
- Replace edited tables wholesale, losing existing focused cell revisions.
- Track hyperlink shells inside revisions, violating the intended content model.

## Test plan

The **test gate** is **regression**, as declared in the backlog.
Add tests to the existing `crates/rdocx/tests/regression_test.rs` binary.

| Category | Test | Asserts |
|---|---|---|
| regression | `comparison_tracks_a_whole_paragraph_holding_a_hyperlink` | Issue368 cases1a through1d, inserted, deleted, moved, appended and final deleted hyperlinks, shell nesting, targets and both outcomes. |
| regression | `resolving_away_a_hyperlinks_content_drops_the_hyperlink` | Acceptance, rejection and sequential selected resolution remove newly emptied owners and preserve original empty or partly retained ones. |
| regression | `comparison_refuses_whole_paragraph_structures_word_cannot_resolve` | Simple field, simple field inside hyperlink and empty whole-paragraph hyperlink still fail atomically with explicit diagnostics. |
| regression | `paragraph_inserted_or_deleted_before_a_table_compares` | Issue369 cases2a through2g, multiple paragraphs, terminal table and jointly inserted or removed table. |
| regression | `paragraph_inserted_or_deleted_before_a_table_in_a_control_compares` | Equivalent block-control cases retain pretty-printed shell bytes and whitespace. |
| regression | `paragraph_mark_removed_before_a_table_removes_an_emptied_paragraph` | Independent tracked markup permits proofing markers and atomically refuses retained content. |
| regression | `paragraph_deleted_before_a_table_carries_its_own_mark` | Exact changed paragraph receives the mark and retained predecessor does not. |
| regression | `table_edited_beside_a_deleted_paragraph_compares_in_place` | Issue369 case2c and its insertion reverse produce cell changes without table replacement or row revisions. |
| regression | `comparison_revises_an_empty_paragraph` | Issue370 body spacing-after zero and alignment plus typing into an empty paragraph put properties inside the shell and restore prior empty properties. |
| regression | `comparison_revises_an_empty_header_paragraph` | Issue370 header spacing-after zero reconstructs exact properties through reopen. |
| regression | `comparison_deletes_several_final_paragraphs` | Issue371 cases4a through4d, ordinary chains and formatted retained paragraphs in body and controls mark the predecessor once and restore both property states. |
| regression | `a_failed_full_story_comparison_leaves_package_typed_state_and_caches_unchanged` | Replace its newly supported paragraph-to-table failure fixture with an unsupported structural transition and retain every atomicity assertion. |
| regression | mixed-owner gap control | Multiple bounded mixed-kind gaps remain deterministic and retain focused table edits. |
| round-trip | existing unmodelled comparison and granular preservation tests | Raw XML, comments, PI, foreign declarations, whitespace and relationships remain exact and appear once in tracked and resolved reopened documents. |

Exercise run, word and character granularity where applicable, default options,
independent field and formatting ignores and their reported combination. Preserve
Issue368 in-place editing, Issue369 unchanged-table and blank-paragraph controls,
Issue370 nonempty and ignored-formatting controls, and Issue371 one-deletion
controls. For ignored formatting assert original formatting rather than the
destination's ignored properties.

Save and reopen each successful redline, resolve independent copies, then save
and reopen both resolutions. Inspect ordered text, paragraph properties, table
structure, hyperlink identity and relationship targets independently. Reuse
`compare_and_resolve`, supplementing text-only assertions with structural facts.
Keep fields, drawing payloads, unrelated package parts and related-story cases.

Run `cargo check -p rdocx --all-targets` while iterating. Run focused tests with
`RUST_MIN_STACK=8388608 cargo test -p rdocx --test regression_test <filter>`,
using filters `comparison_`, `paragraph_`, `table_edited_beside_` and
`resolving_away_`. Run the existing native regression binary and required
preservation, refusal, field, marker and atomicity controls before completion.
Pass `/microscope F-X229 --working` and `/verify --scoped F-X229`.

Resource review must inspect the donor gap-kind LCS allocation and lifetime
against the existing quadratic `align` matrix. Introduce no worse asymptotic
algorithm or unbounded retry. The deterministic mixed-owner fixture exercises
multiple gaps without timing assertions or arbitrary huge allocation.

## HLD impact

- `docs/hld/03-architecture.md`
- `docs/hld/10-bindings-spec.md`
- `docs/hld/12-testing-strategy.md`

Replace obsolete whole-hyperlink refusal descriptions with supported nesting and
the exact remaining refusals. Document table-adjacent and trailing mark behavior.

## Risk routing

- **Any parser or serialiser**: read `docs/hld/04-opc-and-packaging.md` and
  `docs/hld/06-presentationml-model.md`. Check schema child order for paragraph
  properties and mark formatting, hyperlink revision nesting, prefix-tolerant
  reading and fixed-prefix writing. Require exact opaque-subtree round trips.
- No new public signature, crate edge, binding surface, production file, layout
  behavior or feature flag is planned. Their extra riders do not apply.
- Contributor repros are acceptance inputs, not a newly executed external oracle.
  Existing mandatory oracle gates remain active. Record actual tool versions when
  an existing oracle is executed and do not claim donor results as local evidence.

## Hash harness

Expected unchanged, all49 entries. Samples do not invoke comparison. Record the
intentional correctness XML changes explicitly in the feature commit and review.
Keep presentation grouping deltas exclusively in F-X230.

## Implementation checklist

- [x] Establish reported failures on current source.
- [x] Adapt only the five correctness commits and relevant tests.
- [x] Add independent properties, relationship and opaque XML assertions.
- [x] Preserve unsupported-input refusals, atomic staging and resource bounds.
- [x] Pass focused tests and existing comparison controls.
- [x] Pass independent microscope and scoped verification.
- [x] Prepare reviewed scoped evidence for the orchestrator dependency-prefix checkpoint.

## Open questions

No blocking product question. Assumptions for consolidated approval: empty
whole-paragraph hyperlinks and unsupported simple fields retain explicit atomic
refusal. Gap pairing remains deterministic within the existing resource contract.
No new production file is needed. F-X203 is done in `BACKLOG.md`.

## Package gate maintenance

The mandatory repository policy rederives package size claims from the actual
source archive, including packaged tests. After implementation, use the existing
README measurement mechanism to update only changed package rows and their
scripts/readme_doctests.py tuples and dates from actual local patched archives.
Keep compression tolerance and10MiB bounds unchanged. This is required evidence
maintenance, not a behavioral or performance claim. README.md and
scripts/readme_doctests.py are exclusive integration resources. Reconcile them
against exact combined source and independently review any measured changes.


## Concrete helper instantiations

The private next_kept_owner generic receives two existing closure types, the
body owner selector and the block-control owner selector. The private
wrapped_children generic is instantiated by the current inserted, deleted and
moved paragraph wrapper closures. No hypothetical implementer justifies either.

## Preservation hardening

Actual donor-only and follow-up regressions reproduced foreign same-local-name
hyperlink mutation and loss of foreign text in a deleted paragraph. Namespace-
qualified child selection retains opaque lookalikes. Whole-owner wrapping keeps
raw comments and processing instructions in their original positions. Paragraph
mark removal before a retained table refuses meaningful raw gaps and opaque
children atomically. Hyperlink cleanup requires the resolved inner bytes to be
empty apart from whitespace, so opaque raw gaps cannot disappear with its shell.


## Actual worker archive measurement

The post-D1 source archive was generated with the existing cargo package
--locked --allow-dirty --no-verify mechanism and22 local patches. Actual rdocx
compressed bytes3711139, normalized member bytes51833489 and37 members match
both the README row and policy tuple, dated2026-10-15. The archive generated
AFTER the packaged root README carrier update has SHA256
fb3d1fb87066d2999b032cb329cc7e6005aa244052aa21184962bf48580aa040.
Receipt: /private/tmp/F-X229-archive-D1-final.json. The separate before-carrier
artifact is recorded at /private/tmp/F-X229-archive-D1-before-carriers.json.
Existing64-byte tolerance and10MiB bound remain unchanged. Combined integration
must remeasure its own source. Earlier pre-D1 receipts are not final evidence.

## Microscope D1 remediation

Pass1 identified captured nested Word hyperlinks bypassing modeled nesting
admission and entering unbounded recursive wrapping. Whole-paragraph comparison
now refuses a hyperlink nested inside another Word hyperlink explicitly and
atomically. The established parser leaves these nested owners raw, so this
repair does not add nested hyperlink support. Calls are statically bounded to
one paragraph and one direct hyperlink owner. Namespace and span scans remain
iterative and their simultaneous copied subtrees remain linear in input size.
Direct and aliased hyperlinks remain supported, and foreign lookalikes stay
opaque. A source-built captured-owner test checks depth1 admission and depth2
and257 atomic refusal on both insertion and deletion at all granularities.
The actual pre-fix RED reached depth2 and failed because only the later package
postcondition refused it, without the required bounded ownership diagnostic.


## Final scoped acceptance evidence

The actual final post-D1 source passed cargo fmt --all --check, rdocx
all-target/all-feature clippy with warnings denied and cargo test -p rdocx.
The complete native suite passed625 unit,379 integration,2409 regression and2
compile doctests, with existing ignored counts12,8 and7. The regression suite
ran24.60s. Every assertion remained enabled under development opt-level1 with
CARGO_PROFILE_DEV_DEBUG_ASSERTIONS=true and
CARGO_PROFILE_DEV_OVERFLOW_CHECKS=true. Pinned LibreOffice26.2.5.2 and
Poppler26.01.0 equipped the successful native gate outside sandbox converter
restrictions. The captured raw-depth boundary gate passes in this final suite.

Final source hash harness passed49 unchanged entries. Prose passed0 violations
and generated adapters26 in sync. The complete repository policy suite passed
180 tests with2 skipped in85.545s using real registry access and the expected
archive target path. Independent microscope pass2 reports0 defects and0 smells
and independently verifies the final archive tuple, hash and packaged README.
Pre-remediation broad results remain separate and are superseded by these gates.

The scoped gate does not require workspace-wide tests, no-default-features,
WASM, workspace documentation, publication dry run or supply-chain checks because
this plan triggers none of those riders. Their full sprint gate remains due.
No public signature or semver boundary changes. Intentional comparison XML
changes are the hyperlink wrapper placement, revised paragraph marks beside
tables, contained empty-paragraph properties and trailing deletion mark chains.
Word and character presentation grouping remains assigned to F-X230.

Worker preparation supplies the evidence for the canonical dependency-prefix
checkpoint. It does not complete that checkpoint or mark sprint ledgers done.
F-X230 remains barred until the orchestrator completes canonical F-X229
integration and the required dependency-prefix checkpoint.

## Canonical integration reconciliation

Integration onto6e9fe1f4 preserves F-X178 setters and F-X231 HTML behavior.
Product and HLD hunks merge without semantic conflict. The shared README and
policy tuple conflict is reconciled from actual combined-source metadata,
using the unchanged22-patch helper. No isolated worker number is authoritative.
The rdocx row declares3714005 compressed bytes,51847828 normalized member
bytes and37 members. Its actual after-carrier archive is3714006 bytes, with
exact normalized member bytes and count. The one-byte difference is within
the unchanged64-byte allowance and the archive is below10MiB. Other current
package rows remain intact. Metadata generation does not replace verified
publication dry runs.

The before and after archives remain under
`/private/tmp/S92-integration-preflight`. The combined receipt SHA-256 is
`9817499112aa3eeeb3ae292338c83eaea6e0078260bd20c2a50fd22dabc72495`.
Independent canonical reconciliation review reports zero defects, smells and
nitpicks. Actual focused canonical tests pass54 comparison cases with one
existing ignored case,73 paragraph cases, one table case and one hyperlink
resolution case. All49 hash entries match. The focused receipt and logs remain
in the same evidence directory. Final full verification and sprint review
remain due after F-X230 integration.


## Dependency-prefix completion

The canonical prerequisite is completed after the validated handoff, clean
independent integration review and focused actual combined-source checks. Its
scoped worker verification, exact source preservation and regression gate remain
valid on this prefix. Full workspace verification, final current-source riders
and sprint review are still due after F-X230, and are not supplied by this
checkpoint. The shared ledger records scoped acceptance only.
