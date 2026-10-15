# F-X199, Model namespace-qualified empty table cell margins

**Status**: completed
**Sprint**: S91
**Size**: S
**Depends on**: none

## Problem

`CT_TcPr::from_xml_with_prefixes_and_owner_bindings` models expanded `tcMar`
at `crates/rdocx-oxml/src/table.rs:1711`, but self-closing children route through
`parse_property_element` at `:1683`, whose branches omit `tcMar` at `:1781`.
The equivalent empty container therefore becomes `extra_xml` and makes
`CellRef::has_unmodeled_properties` report unsupported content at
`crates/rdocx/src/table.rs:2494`. A fail-closed downstream importer can omit an
ordinary table containing this common producer spelling.

## Spec reference

- `docs/hld/14-development-backlog.md`, "F-X199, Model namespace-qualified empty table cell margins".
- `docs/hld/04-opc-and-packaging.md`, "The package", source namespaces and byte-preserved rejected subtrees.
- `docs/hld/06-presentationml-model.md`, "Preservation strategy", shared reader and writer conventions.
- `docs/hld/10-bindings-spec.md`, "Native Word facade stability", stored typed table and cell readers after reopen.
- `docs/hld/12-testing-strategy.md`, "Test taxonomy", regression and round-trip.

## Approach

Assess and reuse PR302 by `pedroassumpcao` at exact head
`30e4a93df7549990c5838da9c95866d19f1247b0`, based on
`733ce6f7b2b816c5760692abf5c99d68b29ae897`. It is a draft, has no linked issue
and has zero issue comments, inline comments and reviews at intake. Its two
changed files are `crates/rdocx-oxml/src/table.rs` and
`crates/rdocx/tests/integration_test.rs`. Recheck the head and discussion before
implementation and integration. Adapt accepted contributor changes rather
than independently recreating the same patch.

The pre-implementation refresh assessed updated head
`7ac2851cb61b5e7b26427004517cc0cd4db0dbd9`. Its native source and regression
patches are unchanged. Additional contributor README archive records correctly
identify generated footprint maintenance required by these source changes.
Include actual local measurements for both affected package inventories in
the existing README and measurement-script records, generated through
`python3 scripts/readme_doctests.py --record-measurements` against this worker's
reviewed source before preparation. The command emits the full existing
inventory, but only the two affected tuples and README rows change here.
Root repeats measurement against final reviewed integrated sources at the
sprint gate. Until actual local evidence and record updates exist, retain the
noncompleted measurement checklist barrier.

Contributor archive byte counts and Apple M3 platform values are not local
evidence. The actual M5 host matches the existing platform pin, so omit the
contributor-only per-package provenance dictionary and its M3 assertion.
Preserve compression tolerances, inventory member counts and historical
performance observations. No new policy mechanism or dependency is required.

Add the qualified `tcMar` branch to the existing private
`CT_TcPr::parse_property_element` path. Admit an empty container only when its
attributes are namespace declarations. Resolve the element through the existing
`word_prefixes_at` and `is_word_element` helpers, including local alias and
default namespace declarations. A real attribute, foreign element or locally
rebound prefix keeps the existing exact raw capture path. Do not change the
expanded parser's scope or widen unrelated property admission.

For an admitted self-closing container assign
`Some(CT_TblCellMar::default())`. Reuse the current writer and absolute raw
schema boundaries. Its ordinary canonical expanded spelling is valid and
reparses to the same typed facts. The typed empty value does not invent zero
margin values or alter inherited layout defaults. Unknown raw siblings retain
their original bytes and slots after other typed properties mutate.

Reuse the contributed namespace and raw-preservation tests in the existing
table.rs unit module, including its updated absolute-slot expectation. Reuse
the existing integration entrypoint's generated 7-by-3 table regression,
verifying all cell texts and public reader facts before and after save/reopen.
Add uncovered attribute or namespace controls only where needed. No public
signature, dependency, feature flag, crate, module, implementation file or
binary fixture is introduced.

## Rejected alternatives

- Treat every local-name `tcMar` as modeled, which hides foreign or rebound producer content.
- Ignore arbitrary attributes on the recognized empty container, which falsely clears unsupported-content flags and discards unknown facts.
- Change the reader flag rather than modeling the known property, which masks genuinely unsupported properties.
- Reimplement the contribution before assessing its exact immutable patch.

## Test plan

**Test gate**: regression.

| Category | Test | Asserts |
|---|---|---|
| regression | `self_closing_cell_margins_are_the_same_typed_value_as_expanded_empty_margins` | Compact and spaced self-closing, expanded empty, alias and default-namespace forms are the same typed value after serialization |
| regression | `foreign_or_attributed_empty_cell_margins_remain_preserved_unmodeled_xml` | Foreign elements, local prefix rebound and real known or foreign attributes retain exact raw bytes |
| regression | `unmodelled_standard_cell_properties_keep_absolute_slots_after_typed_mutation` | Modeled margins and unsupported siblings keep schema order after grid, merge, wrap, direction and alignment mutation |
| regression | `empty_cell_margins_do_not_make_ordinary_tables_unmodeled_after_reopening` | All 21 cells expose margins, no false unsupported flag and exact text before and after save/reopen |

Prove the public reproducer fails at the exact claimed Base before applying
the accepted patch. Run focused table unit tests and the existing integration
regression. Run `cargo check -p rdocx-oxml --all-targets` and
`cargo check -p rdocx --all-targets` during iteration, scoped lint and formatting,
then `/verify --scoped F-X199` and independent zero-finding `/microscope`.
Do not claim the contributor's test counts as local evidence. Existing native
Word or pixel oracles are not needed to prove this parser spelling correction.

## HLD impact

- `docs/hld/10-bindings-spec.md`
- `docs/hld/14-development-backlog.md`

Completion records qualified empty-margin reader equivalence and retained
unknown-content boundaries in the native Word facade section. HLD14 retains
this story's current acceptance rather than a change history.

## Risk routing

- Parser and serializer, `.claude/skills/risk-routing.md`: read HLD04 "The package" and HLD06 "Preservation strategy". Require qualified aliases and rebound controls, exact raw capture round trips and absolute `xsd:sequence` slot checks. Write canonical prefixes only for admitted modeled content.
- Native published behavior, HLD10 "Native Word facade stability": reader semantics change without a new API or signature. Verify affected package dry-runs and archive size assertions. Preserve published source membership and existing MSRV.
- Rendering exposure, HLD08 "Tables": use deterministic font mode for the unchanged hash harness. An unexplained delta blocks completion.
- No new trait, generic parameter, dependency, module, implementation file, crate or feature flag.

## Hash harness

Expected unchanged. Empty default margins contain no explicit edge values and
do not change the sample generator or effective rendering defaults. Check all
49 entries with deterministic fonts. Do not move the exclusive baseline.

## Exclusive file claims

- `crates/rdocx-oxml/src/table.rs`
- `crates/rdocx/tests/integration_test.rs`
- `README.md`, the locally measured rdocx archive row only
- `crates/rdocx-oxml/README.md`, the locally measured rdocx-oxml archive row only
- `scripts/readme_doctests.py`, the two locally measured archive tuples only
- The F-X199 HLD14 section and native Word empty-cell-margin reader prose in HLD10, updated under serialized completion ownership

F-X193 owns html.rs and F-X198 owns document.rs and regression_test.rs, so
their source phases can share wave 1 with this story. F-X194's native evidence
uses integration_test.rs and regression_test.rs and follows wave 1. Shared HLD
completion edits, canonical ledgers and integration remain serialized. Cargo
execution queues against the shared target without concurrent cache writes.

## Implementation checklist

- [x] Refresh PR302 immutable head, changed files and discussion.
- [x] Prove the generated public reproducer fails at claimed Base.
- [x] Reuse the accepted parser branch and contributor regressions.
- [x] Verify namespace, unknown-byte preservation, schema slots and all 21 texts.
- [x] Run focused checks, scoped gate and independent zero-finding microscope.
- [x] Update the declared HLD sections under serialized completion ownership.
- [x] Generate and record both affected worker package measurements with actual local provenance before preparation, preserving the completion barrier until evidence exists. Root repeats final integrated measurements.
- [x] Prepare the validated handoff with contributor provenance.

## Open questions

None. The user's rolling intake instruction explicitly includes new independent
PRs. PR302 is a narrow equivalence correction with in-memory controls and no
new surface. Issues264 and281 remain excluded. Draft contribution status does
not substitute for local acceptance evidence or authorize GitHub closure.

## Rolling PR refresh and archive measurements

PR302 advanced to head7ac2851cb61b5e7b26427004517cc0cd4db0dbd9 before implementation. Its native fix and tests are unchanged. The added archive records are relevant generated maintenance, but the contributor's Apple M3 measurements and per-package provenance override are not local evidence for this Apple M5 host.

The worker runs the existing measurement generator before prepare, updating README.md, crates/rdocx-oxml/README.md and scripts/readme_doctests.py with actual local package bytes and provenance. The integrator repeats the measurement gate after all sources are integrated. Do not copy foreign measurements or loosen archive guards. This required integrated measurement remains a completion barrier. The existing host provenance is sufficient, so no new override dictionary is planned.
