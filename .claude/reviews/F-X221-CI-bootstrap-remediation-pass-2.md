# F-X221 CI bootstrap remediation, pass 2

**Verdict: 0 defects, 0 structural smells, 0 nits.** The two findings in pass1 are resolved. This bounded successor reviews their remediation and unchanged six-carrier composition against HEAD `342207c9`. It is not sprint review pass2 or a declaration that final verification is complete. The reviewer performed no tests, builds, source edits, environment changes or admission execution.

## D1 resolved: actual compiler and isolated transitive closure

`crates/rdocx-py/tests/test_rendering_threads.py:2314` resolves the actual rustup compiler and verifies version1.97.1. Lines2317 through2335 capture its sysroot libraries and the Cargo-emitted dependency artifact superset together with source and lockfile hashes. Lines2326 through2339 link through a fresh isolated dependency directory. Lines2340 through2342 check every input hash, the link inventory and sysroot inventory after compilation. The receipt at lines2344 through2351 binds actual compiler bytes, physical inputs, isolated inventory, Cargo records, command and output binary. This supplies the compiler and isolated transitive custody required by `.claude/plans/F-X195-design.md:282` without qualifying a PATH shim or an open shared link directory.

The actual successor `/private/tmp/S91-F-X221-CI-bootstrap-B1-1/inputs-2/reference-compile-receipt.json`, SHA-256 `9e05909469c9f71ccc0515fc018c158331e274c8eceb46def3eb54ad475836a9`, records exit0 and successful before/after guards. Read-only authentication found all339 input hashes, all194 isolated artifact targets, the actual compiler hash and output binary hash exact. Qualification `path-reference-qualification.json` in the same directory has SHA-256 `f5a870ae7c453734e764232c96723eadc768fd041e83a24ad2551949042de43e` and binds that actual receipt. The original incomplete qualification remains a predecessor, not the authority for this successor.

## D2 resolved: independent advance totals and actual104

`crates/rdocx-py/tests/test_rendering_threads.py:2289` now derives `total_x_advance` from the independent static HarfBuzz glyph advances. The preceding selected-source/static shape comparison remains in place. Actual400 and700 controls contain8300 and8742 respectively, matching the independently observed shape sums. The existing Word and PowerPoint geometry consumers, positive paint comparisons and thresholds remain intact.

The actual successor `/private/tmp/S91-F-X221-CI-bootstrap-B1-1/actual104-2-receipt.json`, SHA-256 `d3ce736585e5ff802b80415ce312c87383361226ee16fd718ede04370b6f44a6`, records exit0, no timeout, source guards passed and104 actual passes. Its log SHA-256 is `5c89de067ccb3a1555ed31ffdff661dd5892f1c1120d6ca0e9e4aec55801ec10`. Read-only authentication confirmed104 unique passing node IDs equal the receipt membership, current source hashes, all eight artifact receipt hashes, the bootstrap receipt and clean installed-wheel proof hashes. The earlier98-pass/six-failure attempt remains failed. No failed attempt or original wheel build is relabelled.

## Bounded native-eight preparation

The external, unexecuted `prepare-current-CI104-native-eight.py`, SHA-256 `0a64f0d72f3f1d1558d14b6c7706e90f78c82f76250139cbf6205db793996961`, has no findings within the requested binding scope. It authenticates immutable build capture51977448 and original wheel proofa815d0bf, confines current overlays to six existing nonproduction carriers, checks original wheel member hashes and complete installed member sets excluding RECORD, and binds current317 source inputs, exact104 membership and eight actual artifact receipts. Its component-only admission explicitly disclaims a whole-suite pass. The native-eight Rust consumer remains a future ROOT serial execution after admission, not an inferred success from the Python run.

## Remaining gate boundaries

The source-bootstrap changes retain the independent pinned HarfBuzz and licensed-font oracles, portable authenticated Poppler source-build route, explicit pre-pytest preparation, complete-directory CI invocation, Pillow dependency and actual CLI discovery. No further material finding was identified in the unchanged scope of pass1. No new tracked source file, module or asset was introduced.

Full-policy successor32256 is pending at report time. Its predecessor was cancelled after registry DNS failures and is not credited. Focused policy and prose checks reported by the author do not substitute for the pending full policy gate. The fresh104 result is component acceptance only. Native-eight, final full suites, packaging, hosted verification, publication and sprint closure still require their own actual evidence. This report grants no such credit.

The pass ends on writing this report and releases the exclusive canonical review-file-only lease. Normal workflow control returns to ROOT.
