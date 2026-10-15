# F-X217 expectations, pass 1

**Reviewed**: Limited supplemental capsule on `work/f-x217-codex`, HEAD `38d1697b1d56bf97f7eabc4c02eb6996acf0ae3d`. Six carriers changed since the clean whole-feature review. The capsule six-carrier patch contains 88 insertions and 25 deletions against the worker base. All production source and dependency carriers are unchanged since that review. Read the approved plan, workflow, cited import and testing HLD, prior clean review, progress, retained receipts and exact successor evidence.

**Verdict**: Zero defects, zero smells, zero nitpicks in this supplemental scope. Native Word acceptance remains pending. This is not a completed feature or full-gate claim.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Intended behavior and regression expectations

The selected behavior contract at `.claude/plans/F-X217-design.md:89` explicitly declares owned valid imported links, distinct unordered numbering within ordered lists, supported list-item tables and accepted picture export. These implement the approved import and canonical round-trip scope. The plan requires a labelled behavioral implementation commit and retains the unchanged 49-output expectation without authorizing a baseline change.

The link successor at `crates/rdocx/tests/regression_test.rs:9824` replaces the obsolete link-target-dropping expectation with a real imported hyperlink relationship and its normalized URL. It retains surrounding text, malformed HTML repair, unsupported CSS and unsafe-content diagnostics. The native normalization route at `crates/rdocx/src/html.rs:2453` supports the observed normalized target.

The mixed-list successor at `crates/rdocx/tests/regression_test.rs:9869` asserts actual nested depth, a distinct concrete numbering instance and bullet kind. Existing ordered start, restarted list, span, cell-content and nine-level limit checks remain. The table successor at `crates/rdocx/tests/regression_test.rs:9944` asserts actual cell text and direct paragraph/table body order rather than waiving a missing table. It also rejects the old dropped-table diagnostic.

The wrapped exporter successor at `crates/rdocx/tests/regression_test.rs:16276` checks four exact PNG data URI payloads, including the table cell, and still requires wrapped/plain equality and the complete ordered Markdown text. The fixture actually contains those four body pictures. Header content remains excluded. Existing HTML and MHTML controls remain intact. The actual image resource path at `crates/rdocx-html/src/markdown.rs:759` resolves owned image bytes, rather than returning a hidden input shortcut.

These changes strengthen the newly approved behavior. They do not merely delete failing assertions or relax unrelated semantics. Test-only `unwrap` and `expect` calls operate on source-built fixtures and introduce no new production panic path.

## Footprint measurements

Independently opened the current retained `target/package` archives without rebuilding, publishing or executing product code. Every ordinary archive member equals its current declared source, including the shared root README packaged by rdocx. Cargo-generated metadata carriers remain identified separately.

| Archive | Actual compressed bytes | Normalized member bytes | Members |
| --- | --- | --- | --- |
| rdocx | 3,668,473 | 51,553,849 | 37 |
| rdocx-html | 27,435 | 109,584 | 11 |
| rdocx-cli | 93,110 | 427,023 | 8 |

The script tuples at `scripts/readme_doctests.py:405` and README rows at `README.md:42`, `crates/rdocx-cli/README.md:28` and `crates/rdocx-html/README.md:20` agree in exact member sizes and counts. The CLI recorded compressed observation is 93,112 bytes, a two-byte difference within the unchanged 64-byte allowance. All three are below 10 MiB. Validation logic at `scripts/readme_doctests.py:1335` and the tolerance at line394 are unchanged. No source-growth tolerance was introduced.

The earlier tmp-registry Word artifact retains normalized size51,553,808 and its earlier test/README bytes. It is not the current acceptance archive. The later actual Word archive matches current source and the corrected size51,553,849 after the 41-byte test-format change. The original full policy failure remains attributable to that exact stale member size. Its actual exact footprint successor passed, and retained logs also show README compilation and documentation completion. No stale artifact was substituted for current-source evidence.

## Evidence qualification

Authenticated supplemental capsule SHA-256 `2c6ddb52347fdd5b9b3c300a9448482b798d49fd0edf587f6ef4b1003083de26`, source map SHA-256 `ca2796c62276d0f2aed92dbc71d861c1d15c54b2de8bb055b917b71b6b5b3471` and patch SHA-256 `c51b0647fff74c35815cc9119e89597bb7996fb29aefc31101436be4ec60a3de`. Compared the old and supplemental 3,205-entry source maps. Exactly the six declared carriers differ, with no production or dependency change. Immediately before writing, every current mapped file matched the supplemental bytes, size and mode.

Authenticated retained raw-log hashes from the original regression, exact three successors, dependency successor, original full policy and final archive/policy successor receipts. The original full regression remains 2,393 passes, three failures and seven ignores. Each of the three exact corrected tests passed separately. The original 141-test policy module remains failed with one footprint failure and two platform skips. Its focused current-source footprint successor passed. These retained failures were not relabelled green. MSRV, WASM, 49 deterministic hashes and supply-chain success remain separate recorded evidence, not executions by this reviewer.

The prepared combined and loaded-template Word inputs match their recorded hashes. Independently compared source and imported template header/footer XML, their relationship parts and both picture payloads. They remain byte-identical. The persisted-template control therefore is not disproven by an earlier helper-object identity normalization. Native Word open, normal save, reopen and render are still pending under the plan and `docs/hld/12-testing-strategy.md:3073`. Static package preservation is not native application acceptance.

## Not found

- Correctness: zero findings in the changed expectations and measured footprint records.
- Contract: zero findings. Selected behavior is explicit, existing output baselines remain protected and pending native acceptance is retained.
- Panics: zero findings. No new untrusted-input production path changed in this scope.
- OOXML: zero findings. Assertions check actual owned resources and body order, with prepared loaded-template preservation independently confirmed.
- Tests: zero findings. Exact failures, intended successors and unrelated controls remain distinguishable.
- Structure: zero findings. No product file, trait, generic, wrapper, module or feature flag was added by the supplemental delta.

## Review boundary

Only this review file was written. No source, tests, plan, README, measurements, HLD, progress, ledger, runtime, Cargo, CP or Office action was performed. Read-only artifact hashing and archive/XML inspection were used for authentication. This limited review does not replace the prior whole-feature review, required native Word acceptance, labelled behavior commit or completion handoff. The review pass ends here and returns the writer lease to ROOT.
