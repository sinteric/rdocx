# F-X204, full, pass 1

**Reviewed**: Independent `/microscope F-X204 --working` across correctness, contract, panics, OOXML, tests and structure. Worker `/private/tmp/rdocx-f-x204-codex`, HEAD `4bb17d5fe0dc6187553fa5f8c63aba224f5d621c`, original claim `9ef5ec193c7ebb5daa49acf1144dd2fa088d441f`. All 32 changed files, 4198 insertions and 88 deletions, including native providers, both bindings, both CLIs, tests, references and HLD.

**Verdict**: 4 defects, 0 smells, 0 nitpicks. Not ready for completion.

## Source and evidence binding

Read the approved plan and its specific packaging, binding, presentation ownership, fitting and backlog references before auditing the complete diff. The source manifest `.claude/scratch/F-X204-start/source-combined-2.json` has SHA-256 `bbf93e14737ef28059696b1cf4f4c87ef70448a386c3108450e643b8aaa1bee1`. All 3197 tracked entries matched their recorded bytes, mode and size before and after the review. Index SHA-256 remained `2db16757b08c55284a78801006d7446ba9afdb5223c6054d8c160426d091874e`. Full working diff SHA-256 remained `a250ea9af4c6582c968e01e65c1c5791f48cfc35fb549a36f3e31bd996f8d959`.

The finite evidence map has SHA-256 `664a694e4727f8f869a545b373139b35069fa0147bdd0ad504efb8b8c9462722`. It is a navigation aid, not the runtime authority. Inspected the actual paired installed, stub successor and final scoped receipts and authenticated every referenced log hash in those four receipts. Seven exact installed controls passed. Word strict mypy and stubtest passed, while the retained first PPT stubtest failed. The separate PPT stub-only successor passed strict mypy and stubtest with complete distribution guards. No runtime command, CP access, Office automation or source edit was performed for this review.

The initial complete CLI command recorded Word 3 unit and 82 integration passes, and PPT 47 integration passes plus a pinned corpus fixture failure. The exact corpus successor passed. Support had 13 passes. The native suite receipt records Word 555 passes, 7 ignored and 2 retained pinned Poppler failures, so that command is not a complete green native gate. Later native successors and the five affected archive metadata records remain outside this review's present evidence boundary. No archive verification, package-size or final verification completion is claimed. The final measured metadata must receive a bounded review before completion, as the plan requires at `.claude/plans/F-X204-design.md:368` and `.claude/plans/F-X204-design.md:410`.

## Defects

### D1, field time parsing accepts and silently truncates inputs the contract refuses

`crates/rdocx-cli/src/commands.rs:2473`, `crates/rdocx-cli/src/commands.rs:2474`, `crates/rdocx-cli/src/commands.rs:2487`, `crates/rdocx-cli/src/commands.rs:2522`

The approved resolved contract at `.claude/plans/F-X204-design.md:487` requires exactly `YYYY-MM-DDTHH:MM:SS` and refuses fractions because the native context has no fractional field. The helper supplies midnight for `2026-10-10`, supplies seconds for `2026-10-10T08:30`, and discards `.5` from `2026-10-10T08:30:00.5`. The public parser only rejects an offset, so all three inputs are accepted and can publish changed field caches. They must instead refuse before mutation. Help also advertises date-only input at `crates/rdocx-cli/src/main.rs:287`, contrary to the current binding specification at `docs/hld/10-bindings-spec.md:3292`.

The field integration control at `crates/rdocx-cli/tests/integration.rs:5301` tests one full civil timestamp and an invalid calendar date. It does not distinguish these forbidden forms from the required grammar. Add deciding refusal controls when remediating.

### D2, CLI combined fields omit the promised input filename context

`crates/rdocx-cli/src/commands.rs:2552`

The command passes only `now` and a default context. Both `file_name` and `file_path` remain absent. A source containing a supported `FILENAME` field therefore retains an unresolved cache even though the selected input path is available. The approved plan explicitly requires a documented input-path policy at `.claude/plans/F-X204-design.md:487`, and the implementation's HLD states that the input supplies filename context at `docs/hld/10-bindings-spec.md:3295`. Resolve filename and full-path field facts from the selected input under that policy before calling the common operation, with a source-built saved and reopened filename control. Never use the staging output path as context.

### D3, combined field results discard required evaluation and cache-scope facts

`crates/rdocx/src/field.rs:1831`, `crates/rdocx/src/field.rs:1836`, `crates/rdocx-cli/src/commands.rs:2561`, `crates/rdocx-py/src/document.rs:7417`

The resolved contract at `.claude/plans/F-X204-design.md:485` selects a common field report with ordinary count, layout report, ordered retained `FieldEvaluation` facts, `total_updated` and `saved_cache_scope`. The implementation returns a two-value tuple. It consumes the original evaluations and retains only strings from `KeepStored`, losing field identity, order across evaluated and retained outcomes, and the distinction between evaluated display facts and saved caches. Neither the Python result nor the CLI report exposes total or saved-cache scope. This matters for ordinary and layout-backed fields, particularly the intentionally retained dynamic header/footer caches. Passing cache-update tests does not prove that report contract.

Complete the approved common report and both consumers, or obtain an explicit bounded contract amendment before changing acceptance. The current HLD's reduced tuple wording is not an approval to override the resolved plan. Tests should assert the complete common report against actual supported updates and retained caches, rather than only destructuring the tuple and checking selected counts.

### D4, the Python field result tuple is allocated after publishing the mutation

`crates/rdocx-py/src/document.rs:7417`, `crates/rdocx-py/src/document.rs:7457`, `crates/rdocx-py/src/document.rs:7459`

The method allocates the report object before publication, but returns a Rust tuple of a native `usize` and that object. PyO3 still must convert the count to a Python integer and allocate the enclosing Python tuple after the method returns. At that point `publish_api_candidate` has committed scopes, assigned the candidate and bumped the revision. An allocation failure in that final result conversion can therefore report failure after changing the document and retiring held handles. The explicit publication invariant requires every returned Python allocation before commit at `docs/hld/10-bindings-spec.md:68` and `docs/hld/03-architecture.md:14`. Allocate the complete returned result before publication, as the adjacent counted batch methods already do, and cover the prepublication result-allocation boundary in remediation evidence.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: No additional defect in ordered replacement cascades, count mismatch rollback, tag-only selection, checked slide placement, exact notes source handling, physical media identity or read-only fit consumption.
- Panics: No new reachable panic found in the reviewed argument conversion, checked slide indexes, bounded calendar slices or map shape handling. The Python postpublication allocation defect is recorded above under the explicit transactional contract.
- OOXML: No additional namespace, schema-order or opaque-subtree defect found in this diff. Native operations reuse the existing owner-aware providers and staged package publication. The local notes correction stays in notes authoring rather than changing general DrawingML text behavior.
- Tests: Existing and new controls prove real native, installed and CLI behavior for the observed passing cases. The missing deciding controls corresponding to D1 through D4 are identified above. Pending archive metadata and retained native failures are explicitly bounded, not accepted as green gates.
- Structure: No new trait, generic parameter, dynamic dispatch, module, product file, crate, feature flag or reverse family dependency found. Both CLIs consume the common native operations, and fit uses the completed provider rather than a second solver.
- Contract: No further scope expansion or separate bibliography work found. Conditional architecture HLD changes are unnecessary because native ownership and dependency edges remain unchanged.

Review ends here. Remediation and final metadata review belong to separate phases.
