# F-286, native-qualification, pass 2

**Reviewed**: `--working` on `sprint/s95.3`, HEAD `7a75195851136db44efae62b68c785e7d0cebc36`. Eight scoped files, 57 additions and 29 deletions: F-286 plan, HLD02/03/10/12/14, CURRENT_SPRINT and the existing capability-owner policy test. The complete working diff is ten files, 101 additions and 40 deletions. F-X262 evidence and F-X263 release-plan changes are excluded. Product and native-test source are unchanged from pass 1.
**Verdict**: 0 defects, 0 smells, 0 nitpicks. The accepted bounded native evidence is sufficient for F-286's approved contract after Q24. No additional native qualification blocker was found within this review scope. This is completion-readiness review, not feature completion or a passed full sprint gate.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Evidence inspected

Read repository instructions, workflow, microscope and complete-feature commands, canonical differential-testing guidance, F-286 and directly coupled F-287 plans, pass 1, latest F-286 progress, sprint/backlog status and the current scoped diff. Both provider delivery rows remain in-progress (`docs/sprints/CURRENT_SPRINT.md:36`, `docs/sprints/BACKLOG.md:519`).

Read-only Python probes used `-B` and disabled bytecode. Reauthenticated all 58 native provenance entries against original and retained bytes, including sizes, and all 46 prior hashes. Reauthenticated twelve consumer input/output hash pairs and the installed Word extension SHA-256 `9b9234fa1969f29c38e22a725dc3e78df253b6e30a8792273a7fb8b569b4b8b5`. Removing exactly the reviewed 150 inserted test lines reconstructs the original `test_core.py` SHA-256. Consumed the actual installed native log, 2 passed and 488 deselected. No builds, tests, native UI, agents or other writes were performed.

## Accepted claims and checks with no findings

- **Q24 contract reconciliation**: clone-first append and atomic last-item refusal remain the approved API. Word's addressed-item default and whole-section last-item deletion remain explicit differences. Native append followed by first-item FormattedText copy is the compared equivalent sequence. This resolves the former pending decision without claiming identical defaults (`.claude/plans/F-286-design.md:229`, `docs/hld/12-testing-strategy.md:4761`).
- **Bounded native discrimination**: authenticated Word for Mac16.114(26101123) expected outputs cover rich/plain replacement, distinct list display/value, checkbox and both actual date formats. Picture and native group cases require atomic library refusal. Distinct First/Second/Third content with only First bold distinguishes template and append order. Existing ids survive, the clone gets a fresh id, middle deletion retains the right survivors, and last-item refusal preserves bytes (`crates/rdocx-py/tests/test_core.py:6371`, `crates/rdocx-py/tests/test_core.py:6425`, `crates/rdocx-py/tests/test_core.py:6482`, `crates/rdocx-py/tests/test_core.py:6498`). These native observations complement the existing local A1 through A8 tests rather than replace malformed-input and metadata-preservation coverage (`crates/rdocx/tests/integration_test.rs:26946`, `crates/rdocx/tests/integration_test.rs:27325`, `crates/rdocx/tests/integration_test.rs:27422`).
- **Authorship and normalization boundaries**: twelve library-authored inputs qualify Word opening/saving, separately from authentic Word-authored replacement and repetition inputs. RichText marker omission, date lid=en-GB and group contentLocked are individually recorded native save normalizations. There is no global XML difference allowance or relaxation of the setter's metadata contract (`.claude/plans/F-286-design.md:215`, `crates/rdocx/src/content_control.rs:1539`).
- **DOCX-052/053 classification**: Native means the implemented Rust facade. Python move/clone are actual checked public methods, correcting the stale DOCX-052 boundary. DOCX-053 completes the ten authored kinds and documented equivalent operations. Citation, equation, bibliography and document-part kinds remain refused. Layout/render Y describes the existing stored-content walk, with no new native UI behavior or whole-page visual-parity promise (`docs/hld/02-scope-and-non-goals.md:319`, `docs/hld/02-scope-and-non-goals.md:320`, `docs/hld/03-architecture.md:1883`, `crates/rdocx-py/src/document.rs:8574`, `crates/rdocx-py/tests/test_core.py:6641`).
- **DOCX-054 classification**: the approved F-287 contract promises additive store creation, binding and existing two-way value updates through the absolute-child-path subset. It promises no store-removal method. Removal NA and package layout/render NA therefore do not hide an unfinished promised operation. Read Y is bounded by the declared binding subset, not general XPath or a new store inspection API. Rust/Python methods and reopen, path-selection, unknown-byte and rollback assertions substantiate that boundary (`.claude/plans/F-287-design.md:29`, `.claude/plans/F-287-design.md:33`, `docs/hld/02-scope-and-non-goals.md:321`, `crates/rdocx/src/content_control.rs:1427`, `crates/rdocx/src/content_control.rs:1454`, `crates/rdocx-py/tests/test_core.py:10082`, `crates/rdocx/tests/integration_test.rs:27926`, `crates/rdocx/tests/integration_test.rs:27974`, `crates/rdocx/tests/integration_test.rs:28048`).
- **Owner reconciliation**: explicit F-286/F-287 exclusions match the now-complete bounded capability rows. The policy still requires complete operation classifications and exactly one live owner for every incomplete row. No assertion was deleted or weakened (`scripts/test_sprint_workflow.py:8739`, `scripts/test_sprint_workflow.py:8880`, `scripts/test_sprint_workflow.py:8890`).

## Limits and remaining completion work

This pass accepts the stated native observables and coupled capability boundaries. It supplies no exhaustive XML equality, full rendering parity, identical Word defaults or blanket external issue closure. Optional bundle absence remains a skip, never native success (`docs/hld/12-testing-strategy.md:4750`). Historical pending statements in producer receipts and pass 1 remain historical evidence, superseded by explicit Q24 approval rather than rewritten.

Actual native acquisition addresses the prior missing-Word condition. It does not relabel a failed attempt or establish the current global eight-story full gate. Ordinary completion must still reconcile accepted evidence, remaining checklist/status text and affected metadata checks under `.claude/commands/complete-feature.md:14`. Existing applicable product evidence remains reusable. Final new-version/full-sprint evidence and independent final review remain due.

Q25 permits at most two additional independent sprint reviews numbered 4 and 5 after required gates. This microscope is neither, resets no pass/finding and grants no acceptance waiver (`docs/sprints/CURRENT_SPRINT.md:58`). F-X263 packaging/hosted obligations remain pending until providers actually complete. Newly collected F-X262 old-source evidence, F-X263 release-plan details and unrelated original product changes were not audited. Private unreduced Issue390 remains unqualified under its approved bounded disposition, not closed by this review.
