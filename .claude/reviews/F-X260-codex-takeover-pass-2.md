# F-X260, Codex takeover, pass 2

**Reviewed**: fresh independent `/microscope F-X260 --working`, limited to the Codex takeover fixes on `sprint/s95.3`, HEAD `15d55d4c4b46df56362abc6944103f496fd78cd6`. Six scoped files against HEAD, 64 insertions and 4 deletions, 68 changed lines.
**Verdict**: 0 defects, 0 smells, 0 nitpicks. Takeover pass-1 D1 is resolved. Local review passes, final qualification remains pending.
**Reviewer**: Codex in this audit session, without implementation or delegated agents.

Read the supplied AGENTS guidance, `CLAUDE.md`, canonical `.claude/WORKFLOW.md` and `.claude/commands/microscope.md`, approved F-X260 and F-X262 plans including the takeover amendment, HLD 03 owned publication and causal paths, and HLD 10 lifetime, local publication and Element contracts. Checked branch, dirty status, both in-progress delivery rows, F-X260 progress, takeover pass 1 and latest feature reviews. Prior reviews are context, not independent proof of this verdict.

Scope is the full working diff in `crates/rpptx/src/lib.rs`, `crates/rpptx-py/src/presentation.rs`, `crates/rpptx-py/src/shape.rs`, `crates/rpptx-py/Cargo.toml`, `Cargo.lock`, and the modified existing hyperlink test in `crates/rpptx-py/tests/test_documented_examples.py`. Relevant unchanged providers and companion tests were read to trace behavior. Word implementation, other dirty product/process/HLD changes and final sprint acceptance are excluded. This is not a fourth sprint review.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Prior D1 and graphic-frame assessment

Pass-1 D1 concerned a supported graphic-frame scalar replacement unconditionally cloning the whole presentation. `crates/rpptx-py/src/presentation.rs:2387` now parses only the submitted frame with `CT_GraphicFrame::from_xml`, before calling native replacement. There is no eager full-presentation backup. The existing workspace dependency added at `crates/rpptx-py/Cargo.toml:34` and the matching lockfile edge name this concrete consumer and introduce no crate or public surface.

The classification matches the actual model: `shape_structure` at `crates/rpptx-py/src/presentation.rs:3711` uses `ShapeRef::table`, which matches `GraphicDataPayload::Table` at `crates/rpptx/src/lib.rs:24899`. Native graphic-frame assignment at `crates/rpptx/src/lib.rs:6279` uses the same concrete parser as preflight. Table presence follows the decoded graphic-data URI and parsed payload, not a substring or descendant-name search. Namespace resolution, required children and child order remain owned by that parser. Native `replace_checked_owner_xml` at `crates/rpptx/src/lib.rs:6175` still validates the complete input before local assignment, so preflight is not the final raw-XML validator.

Bounded authenticated in-memory probes confirmed:

- Table to opaque and opaque to table both raise the named `NotImplementedError`. Complete package bytes and held Element XML remain equal. The table-to-opaque control also retains readable table and cell handles.
- Changing only the table URI to an opaque URI refuses a table-presence transition even while the submitted payload still contains a real `a:tbl`. Conversely, an imported opaque-URI frame containing that table-looking subtree supports a scalar name edit. An entity-encoded table URI still selects Table and refuses the reverse transition.
- A table URI with an invalid payload refuses during parsing. Truncated input, a changed frame root, unbound attributes and a correctly bound second root refuse without changing package bytes or held Element XML.
- Exact no-op and same-table-payload scalar name replacement succeed with unrelated slides present. Cell content and unrelated slide text remain readable. A valid alternate PresentationML QName prefix succeeds. Same-opaque-payload scalar replacement succeeds too.

The obsolete post-native table-presence rollback branch at `crates/rpptx-py/src/presentation.rs:2435` remains a defensive check. For current graphic frames, the shared parser and the identical payload predicate reject transitions before native mutation. No bypass was found through root changes, URI encoding, payload lookalikes or malformed input. This closes the specific source-derived unrelated-slide clone finding. It is not a measured allocation bound or a Python timing qualification. Exceptional native whole-part fallbacks still use their existing staged transaction.

Some exploratory probe invocations failed because the probe itself globally replaced `p:` inside a namespace URI, assumed canonical prefixes after a prefix-changing edit, or hard-coded an incorrect generated frame name. Corrected QName-only and generic name substitutions passed. Those failed invocations are not recorded as product RED or silently counted as passing compound probes.

## Full scoped assessment and not found

**Correctness and contract: zero additional findings.** The takeover plan explicitly requires pre-mutation concrete parsing and unchanged-payload local behavior. The preflight is after checked live-owner resolution and before native publication. The table-presence refusal therefore does not alter package relationships, dirty state or owner scopes. Existing stable publication and survivor preparation remain in place. F-X260 A7 is the affected acceptance row. A1 through A4 and A6 are not requalified by this narrow audit.

**Raw XML and OOXML: zero findings.** At `crates/rpptx/src/lib.rs:6099`, the local path checks submitted raw frontier bytes before equivalence or typed projection. Failure falls back to the physical whole-part transaction, where inherited MC authority is available. Traced shape, table, cell, body, paragraph and run replacement callers, plus part replacement. Inspected raw Start and Empty handling, qualified attributes, frontier provenance, complex-policy matching, signed and relationship-change fallback, and child restoration after local assignment failure. The fx254 assertions cover the four frontiers, projection-away foreign geometry, root/group ancestor grants, blank/additive grants, namespace rebinding, understood namespaces, once-decoded values and physical identity. The patch neither promotes a detached fragment failure into a final refusal nor admits foreign geometry merely because typed parsing discards it. No new schema-order or preservation defect found.

**Duplicate identities: zero findings.** `crates/rpptx-py/src/shape.rs:1758` scans the target slide and descendants before the append shortcut mutates it. Compared its missing/duplicate policy with structural publication at `crates/rpptx-py/src/presentation.rs:3305`. `ShapeRef::children` at `crates/rpptx/src/lib.rs:24920` visits group and selected fallback children. AlternateContent containers are not treated as ID-bearing shapes. Independent probes confirmed nested/root duplicate IDs and missing IDs refuse before mutation, retain exact package bytes and held child text, while an append on a separate unaffected slide succeeds. This target-slide work is distinct from unrelated-slide scans and does not prove the public performance ratio.

**Hyperlink test and tests aspect: zero findings.** The changed test at `crates/rpptx-py/tests/test_documented_examples.py:4892` follows approved F-X262 A7. It retains exact-byte absent-clear controls for `None` and empty string, adds an external address, checks invalid control-character input atomically, and reads the address after reopen. The companion test at line 11671 covers replacement, clearing, unsupported cell slide targets, actual slide relationship targets and saved-file reopen. Traced `crates/rpptx-py/src/text.rs:1804` through native cell/shape dispatch and shared staged `set_run_click` at `crates/rpptx/src/lib.rs:12405`. The obsolete blanket refusal assertion was not an approved contract to preserve. Existing regression inputs discriminate the targeted faults, rather than relying only on aggregate counts.

**Panics and structure: zero findings.** No new unchecked indexing, unwrap, trait, generic, wrapper, feature flag, module, registry or product file. The concrete dependency is already a workspace crate, with a matching lockfile edge and a named current consumer. No unsupported compatibility claim was newly introduced or externally qualified.

## Actual evidence and limits

Consumed logs as observations, not assumptions about successful qualification:

| Log | Actual observation and limit |
|---|---|
| `/tmp/s95.3-codex-rpptx-installed.log` | Historical failed run, 59 failed, 958 passed, 1 skipped. Scoped failures include duplicate IDs, graphic-frame atomicity, foreign geometry and ancestor MC. It remains failed full-run evidence. |
| `/tmp/s95.3-codex-rpptx-regressions.log` | Earlier selected union, 57 passed, 961 deselected. |
| `/tmp/s95.3-codex-rpptx-native.log` | Seven named fx254 native tests passed, 186 filtered out. Local native evidence only. |
| `/tmp/s95.3-codex-rpptx-repair-build.log` | Earlier optimized dev build and editable install completed. Superseded executable identity is not reused for current probes. |
| `/tmp/s95.3-codex-rpptx-clippy.log` | Earlier scoped clippy log ends with successful dev-profile completion. |
| `/tmp/s95.3-codex-rpptx-preflight-check.log` | Absent at inspection. No separate preflight check success claimed. |
| `/tmp/s95.3-codex-rpptx-preflight-build.log` | Fresh optimized dev build, abi3 wheel creation and editable installation completed. Pip cache warning is present. This is not a release performance build. |
| `/tmp/s95.3-codex-rpptx-preflight-regressions.log` | Fresh selected run, 57 passed, 961 deselected in 7.95 seconds. No assertion failures reported. This is not full-suite evidence. |
| `/tmp/s95.3-codex-rpptx-preflight-clippy.log` | Fresh log checks rpptx and rpptx-py and ends with successful dev-profile completion in 6.80 seconds. It does not replace all final verification. |

The terse regression logs do not embed their command lines or test node lists. Membership was corroborated with the supplied scoped selection and inspection of its existing tests, not inferred from 57 alone. Logs do not constitute a complete source-hash provenance packet. No broad suite, external oracle, benchmark, baseline recording or sprint review was attempted here.

Probes used `/tmp/s95.3-rpptx-venv/bin/python` with `PYTHONDONTWRITEBYTECODE=1`, in-memory packages only. Imported `rpptx._rpptx` resolved to this repository's `crates/rpptx-py/python/rpptx/_rpptx.abi3.so`. Its SHA-256 was `ab6af282ec0907a4e7b0bca0ecc9a7d6c8f543a30fa3d2bd358bdd2187a6791c`, equal to `/tmp/s95.3-rpptx-py-target/debug/lib_rpptx.dylib`. Distribution metadata identifies an editable install from this repository's `crates/rpptx-py`. The extension hash was rechecked after probing.

**Pending qualification, not a new product finding:** final A5 still requires the actual public Python release workload and all seven ratios below 3, plus a meaningful visit or allocation observation that fails a scan of unrelated owners. Native timing and constant visit markers do not qualify it. These append/XML changes require requalification. This audit's source inspection and functional probes neither pass A5 nor demonstrate a new measured ratio failure.

Authentic native Word evidence remains unavailable and needs to be performed, as confirmed by the user. Full integrated suites and applicable native, font/viewer/raster, rendering, hash, WASM and publication riders remain pending. Nothing here marks F-X260 complete, closes an issue or accepts S95.3.

## Source identity before and after

Initial complete-file hash capture and final post-probe verification matched for all six scoped files. The working diff counts and existing dirty status remained unchanged before report creation.

| File | Before and after SHA-256 |
|---|---|
| `crates/rpptx/src/lib.rs` | `c6ba546a8463721703da052b27f60af8aa4549a5c95ea8db7c6a5400cf559562` |
| `crates/rpptx-py/src/presentation.rs` | `aafbc94b0de627bbaba4c3039d8c077d7bde386e7907b7e451bc6e99297453af` |
| `crates/rpptx-py/src/shape.rs` | `28f0aaf81053665ce9bfdbdacdaeb25a72ac757ca27afb0e036bce847a80e5b8` |
| `crates/rpptx-py/Cargo.toml` | `b41c4a657baed797cfdbda8d09fa319a7cf7aa831d6ce02305580a0d80035cef` |
| `Cargo.lock` | `19dcb63d3ecd086eb21abafb2fef9f9cfd6a55a64b8c03d591553770014b6aab` |
| `crates/rpptx-py/tests/test_documented_examples.py` | `594b3af3ea4a6b694596935f50d8659ed41e7ec30a04aad9faeafa31c9654c9a` |

Only `.claude/reviews/F-X260-codex-takeover-pass-2.md` was written. No product, state, progress, plan, prior review, ledger or baseline edit, commit, push or agent invocation. Review ends here.
