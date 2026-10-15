# F-X200 scoped execution, pass 52

**Reviewed:** The six-artifact external packet `/private/tmp/S91-F-X200-scoped-completion-current-prepared`, including the complete 142-line executable, retained native and installed inventories, oracle qualification and archive mechanism. Receipt SHA-256 `021535115f678853ce867c895aeacbb40bab637d2a7aa2f90f21ec0996500721`. Executable SHA-256 `21f32943fcd2b6fc2b1feef55aef83d4128395a980fb16ed3e0baa92b70d13fe`.

**Verdict:** 1 defect, 0 smells, 0 nitpicks. Static execution review only. No command in the packet was executed.

## Defects

### D1, required scoped workflow-policy gate is omitted

`/private/tmp/S91-F-X200-scoped-completion-current-prepared/execute.py:113`

The riders phase executes format, WASM, hash, prose, adapter synchronization and diff checks, then can report completion without executing `python3 -m unittest scripts.test_sprint_workflow` or requiring an authenticated source-equivalent result for that gate. `/private/tmp/S91-F-X200-scoped-completion-current-prepared/VERIFY.md:21` defers full policy to the integrated gate. Canonical `.claude/commands/verify.md:55` and `.claude/commands/verify.md:66` require this unittest module under `--scoped` as well as `--full`. F200's design retains normal scoped verification at `.claude/plans/F-X200-design.md:190`.

Trigger: acceptance succeeds and every listed rider succeeds. The resulting completed riders receipt still lacks the mandatory scoped policy evidence. Include the existing required command, or explicitly require a genuine current source-equivalent policy receipt before declaring this scoped phase complete. Final integrated workspace verification remains a separate obligation.

## Smells

0 findings.

## Nitpicks

0 findings.

## Checks without additional findings

- **Correctness:** All six artifact hashes match the sealed receipt. All 2714 current source records match bytes, size and filesystem mode. HEAD is `7c9b46a37a5c40ff3d7a21953dec06571b1e5964`. The staged-entry digest is `34a791385771b53d4288429b4b573cdecb5274ec9c9d93db41a4ba86c6f95cc4`. No additional correctness defect found in the three full native crate suites, fresh-wheel installation, CLI selection or four-package archive capture.
- **Contract:** D1 above. The packet expressly retains the joint F210 U2610/U2611 physical-font and deterministic PDF/raster paint obligation as unsatisfied at `/private/tmp/S91-F-X200-scoped-completion-current-prepared/VERIFY.md:25`. Adopted F206 factory source does not establish F206 acceptance. Other provider/foundation gates require actual source-equivalent evidence or affected reruns. This review grants none of those results.
- **Panics and lifecycle:** No additional finding. Owned subprocess execution uses the hash-bound R68 helper's timeout and finally cleanup. Spawned groups must disappear and leaders must be reaped. Unexpected descendants and unproved cleanup block release. The executable records attempted source before launch and recovers compile/test-binary observations from retained logs on failure. Its source and index guards remain active around execution.
- **OOXML:** 0 findings in this runtime packet. The native named inventory retains namespace, opaque-source, styles/defaults, list ownership and twelve EPUB controls. No serializer or production XML change is proposed here. Actual production authority remains the separate working pass 51.
- **Tests:** D1 above. The native inventory contains 106 required named controls. Ignored tests cannot satisfy those controls. Installed collection must contain exactly 188 unique approved nodes and execution must match that membership with only PASSED outcomes. Four strict typing/stub checks remain. Fresh wheel package/native members and installed provenance are checked before and after acceptance. Cargo JSON selects the actual paint CLI and its bytes, inode and mode remain guarded. The 67 retained oracle function bodies and four constant source bodies match the current carriers without a cross-version AST assumption.
- **Structure:** 0 findings. The packet reuses the bounded runner, wheel/node helpers and existing archive capture functions. It adds no repository module, dependency or product API. ROOT must still authenticate the real latest cache producer and own the runtime/install lease. Future producer and admission values remain null, and this executable supplies no implicit timestamp transition.

## Evidence limits and handback

This is an independent audit of the execution proposal, not a fresh independent audit of historically self-authored F195 provider regions. Existing separately reviewed provider and working-pass authorities remain distinct. All named native tests, full 188 installed cases, riders, measured four archives and scoped completion remain unexecuted by this pass. Archive carrier mismatches must stop for attribution and reviewed measurement maintenance. No full workspace gate or feature completion is claimed. Return the review-only lease to ROOT.
