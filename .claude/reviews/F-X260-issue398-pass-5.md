# F-X260, issue398, pass 5

**Reviewed**: independent `/microscope F-X260 --working --aspect issue398`, branch `sprint/s95.3`, frozen HEAD `3c3d7d69623ed30f7994baa28c1bda60b9263a71`. Eleven-file feature scope envelope has 1818 insertions and 160 deletions against HEAD, including excluded shared-file hunks. Incremental focus is opaque Empty section preservation.
**Verdict**: 0 defects, 0 smells, 0 nitpicks. D2 is resolved within this focused admission contract. This is neither a sprint review nor an acceptance waiver.

## Scope and authentication

Read repository guidance, canonical WORKFLOW, microscope and relevant verify evidence rules, F-X260 plan and latest progress, complete Issue398 pass4, and HLD03/10/12 contract and qualification sections. BACKLOG and CURRENT_SPRINT still mark F-X260 in-progress. Consumed prior D1, first-run publication and retained namespace-owner exclusion conclusions. Checked their interaction with this increment without repeating the large batch/cache audit.

All 27 entries in `/tmp/s95.3-issue398-pass5-source-manifest.json` matched before probes and immediately before writing. Imported the current dev extension directly from `crates/rdocx-py/python/rdocx/_rdocx.abi3.so`, SHA-256 `8b5d9b75811d474fb6a1e75af955e6a4ab51cda546081c7a03e06ee210b07b3e`. Asserted its resolved path and hash. Build association uses the supplied successful build record and manifest, not embedded compiler attestation.

Excluded dirty F-X259 process/TOC implementation, field/regression/HLD08 changes, five TOC cases and unrelated content. No builds, broad suites, benchmarks, agents, external oracles, artifact extraction/install, commits or state changes occurred. Probes used in-memory ZIPs with Python bytecode disabled. Only this report was written.

## Defects

Zero. D2's pass4 preservation failure no longer reproduces. No concrete defect justifies reopening D1 or the earlier namespace-owner finding.

## Smells

Zero.

## Nitpicks

Zero.

## Admission and authority analysis

`crates/rdocx/src/document.rs:8121` represents two actual states: a boxed concrete `CT_SectPr` or an existing protected island. This removes invalid optional-field combinations without a trait, generic, forwarding wrapper or public API. Concrete boxing is appropriate for the large parsed model.

The scanner inventories only expanded Word `sectPr` children of the selected direct pPr at `crates/rdocx/src/document.rs:8171`. It temporarily expands Empty events, closes namespace scope, and attempts the existing checked model. Only a failed temporary parse of an Empty event selects preservation at `crates/rdocx/src/document.rs:8200`. Paired failures propagate. Namespace closure errors are not swallowed. The original incoming checked paragraph parse precedes shared admission in staged replacement at `crates/rdocx/src/document.rs:23538` and local replacement at `crates/rdocx/src/document.rs:27312`.

The fallback therefore preserves an already imported raw representation rather than admitting arbitrary new parser failures. Exact raw bytes, expanded names, attribute namespace meaning and relevant QName-value bindings participate in island equality through `crates/rdocx/src/document.rs:8037`. The temporary scan clones context only on this opaque branch. Re-entering declarations affects that clone, leaving the main scanner's scope intact.

Preserved sections have route0 and `authored_ignorable=false`. Route0 deliberately avoids freezing unrelated property positions. It cannot borrow authority from another paragraph: each admission compares only the selected owner's original and incoming section vectors. Ordered equality at `crates/rdocx/src/document.rs:8346` retains multiplicity and rejects additions, removals and changed preserved instances. Matching bytes in a sibling paragraph supplied no authority in an actual probe. Ignorable declarations did not bypass section refusal.

The local modeled-section guard remains before shared admission at `crates/rdocx/src/document.rs:27330`. The Python local and candidate routes still call the existing native providers at `crates/rdocx-py/src/paragraph.rs:762` and `crates/rdocx-py/src/paragraph.rs:783`. No new live-state publisher or post-publication parse was introduced. The namespace-owner exclusion continues routing retained modeled dependencies through staged reconciliation.

## Assertions and bounded challenges

Inspected the actual foreign-attribute tests at `crates/rdocx-py/tests/test_core.py:10289`. Canonical and qualified-alias cases construct imported sections, change text, compare exact paragraph XML, check current text and reopen. Changed attribute values and whole-section removal must raise and retain exact package bytes. These assertions distinguish preservation from merely accepting an operation or silently dropping opaque attributes.

Additional completed probes confirmed canonical/qualified foreign-attribute text edits, unrelated alignment edits, and added-run staged edits. Unchanged opaque children survived exactly. Attribute value changes, attribute removal, section removal, duplication, conversion to an unsupported paired representation, namespace substitution, duplicate attributes and unbound attributes refused atomically. Changing an actually used QName binding refused, while adding an unused binding succeeded. Exact no-op retained bytes and the held run.

Cell and header probes each covered text edits, staged run growth, changed opaque attributes and section removal. Successful operations saved/reopened with expected text and exact section bytes. Refusals retained package bytes and held paragraph/run text. These exercise the shared staged boundary alongside body local eligibility and staged topology fallback.

Exploratory scripts contained incorrect fixture assumptions and are not counted as passing batches. One namespace replacement changed no bytes because serialization had omitted that declaration. Duplicate sections can import, but a subsequent changed checked paragraph refuses them. One sibling fixture had no matching serialized pPr. Header serialization also does not promise whole-fragment lexical equality, so its corrected check asserts exact section bytes and reopened text. Corrected probes established the intended discriminators without changing source or weakening the section contract.

## Evidence limits and explicit zero categories

Consumed current successful final-suffixed check/install and Clippy logs, 75 focused passes including five excluded TOC cases, 11 binding units, and 27 native controls with 3 ignored. Inspected the actual two-failure/six-control foreign-attribute RED. The earlier E0502 attempt and tests against its old module are not current evidence.

Correctness, contract and OOXML: zero remaining findings. Panics: zero. Shared callers/publication: zero new findings. Tests/evidence classification: zero findings. Structure/API: zero. HLD03/10/12: zero discrepancies for this increment.

Fresh current release build, all ten strict ratios, final hash/WASM/package observations and a complete permitted workspace run remain pending. Restored pinned LibreOffice/chart and Word-input observations do not supply native Word qualification. Missing font qualification remains unavailable. The partial Word result of 1003 passed, 14 missing-variable-font failures and 42 deselected cases is not a passed full gate. No baseline or threshold changed. Review stops here.
