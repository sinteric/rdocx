# F-X268, whole feature, pass 2

**Reviewed**: Consolidated D1 and D2 remediation after whole-feature pass 1, including the actual binding-runtime controls, current plan and the five approved HLD files. The complete feature range remains claim Base `9d2878b0c8d7cb0e1b8d9d3d697b5117893ac1bc` through HEAD `5759559029d2af5c61ad5e69e3ebc3af8a8557fe` and the frozen working tree on `work/f-x268-codex`. The tracked range contains 37 files, 46928 added and 30956 removed lines. Pass 1 supplies the unchanged whole-feature source and acceptance audit rather than another helper-only approval.
**Reviewer**: Independent Codex agent `/root/review_source_pieces`. The reviewer did not implement the remediation, modify source, run builds or tests, or spawn another reviewer.
**Verdict**: 0 defects, 0 smells, 0 nitpicks. D1 and D2 from whole-feature pass 1 are resolved. The local implementation microscope exit condition is met. Final-union acceptance remains pending.

## Source identity

| File | SHA-256 |
|---|---|
| `crates/rdocx-py/src/document.rs` | `eab5e64eac204b6ac26faf137d788c0694f4b8a9d632142c482cbebcb11fca5e` |
| `crates/rdocx-py/tests/test_core.py` | `aa3c7a319596659438205ce25dceb0b615eb34eb84c661f5ab1889ad34bc9588` |
| `.claude/plans/F-X268-design.md` | `fc62af457a08cfa3a983e74efdbc03b961cd602ce626eedc9e7194f8dc1ce867` |

The native facade and OXML source hashes remain those reviewed in pass 1: `bb913fe7589260be55940e519ba01b00969043d8d6abc5b5811ebf775002ca10` and `b7eba9fac61355a2a6f3d687fa86d6564526cad3b29281b6db250b4ddccaf5e8` respectively. Their applicable native and release evidence is unchanged.

## Defects

None. D1 and D2 are closed.

## Smells

None.

## Nitpicks

None.

## Resolved findings

### D1, changed existing context now has actual runtime coverage

`fx268_changed_existing_context_uses_full_inventory_before_publication` primes pieces through an existing live `PyDocument`, then prepares an actual native source-adoption candidate whose `body_item_context` differs. It asserts that the fixture really changes context. The absent-context positive control remains eligible, while the changed existing context returns no piece inventory and actually increments authoritative whole-inventory work before publication. Prepared owners are compared with full-reader locations, parents, physical slots, projections and read facts. Preparation leaves the held Python paragraph at its old value. The test commits the prepared revision, installs the candidate, checks the saved result, then exercises a stable-context candidate with zero whole-inventory work and checks causal staleness, fresh state and reopen. This covers the direct source/candidate publisher named in the consumer map rather than relabelling an M7 frame test as WordPieces coverage. Inspected path is `crates/rdocx-py/src/document.rs:18014`.

The native binding test actually ran with the extension-module feature disabled and CPython 3.12 selected. The first attempt had an invalid fixture assumption and is not semantic RED. The corrected D1 test already passed before the D2 product repair, which is appropriate for a missing-coverage finding. Its runtime result is distinct from all-target type checking.

### D2, duplicate admission now uses resolved physical part identity

`WordPieces::assemble` folds the main and related source names to ASCII lowercase for duplicate admission. Related names are already normalized through `OpcPackage::resolve_rel_target`, so this matches the package's normalized case-insensitive lookup without changing stored target spelling. Duplicate physical sources retain the authoritative full inventory. A valid shared header is not newly refused. Inspected path is `crates/rdocx-py/src/document.rs:5366`.

`fx268_case_equivalent_story_parts_use_the_full_inventory` distinguishes an exact shared target, which source discovery already deduplicates, from `Header1.xml`, which resolves to the same physical header as `header1.xml`. It checks actual source counts, piece admission, whole-inventory work, full owner facts, saved bytes and reopened sources. The positive owner comparison uses the existing body-fingerprint normalization and still compares the other route, parent, slot, projection and read facts. Inspected path is `crates/rdocx-py/src/document.rs:18127`.

The installed Python companion `test_word_piece_shared_header_identity_uses_opc_part_names` checks both cases at actual open and prepared font publication. It verifies fallback work, unchanged main XML, shared-header text, established conservative staleness, fresh owners and exact save/reopen bytes. This is an executed public-path control in addition to the native binding test.

## Not found

Correctness, contract, panics, OOXML preservation, cached fact provenance, physical identity, prepared publication, test validity and structure produced no additional finding in the combined repair. All eight mechanism conclusions from pass 1 remain applicable outside these repaired admissions. The two tests reject the distinct wrong implementations that pass 1 identified. They do not merely mirror a comparison result or replace actual runtime execution with compilation.

The plan now distinguishes current claimed implementation from dated historical approval snapshots and maps the actual A3 context and physical-part controls. It remains approved, not complete. The local checklist records implemented work while preserving final rider debt. Exactly the five approved HLD files describe current intent, including physical identity and changed-context behavior. They do not claim completed release timing, memory, Word or final integrated qualification.

## Evidence consumed

`/private/tmp/fx268-codex-owner-remediation-red.log` records two invalid initial test assumptions. It is not semantic RED. `/private/tmp/fx268-codex-owner-remediation-red2.log` records the corrected D1 passing and D2 failing on actual alias admission, 1 passed and 1 failed in 0.14 seconds. `/private/tmp/fx268-codex-owner-remediation-green.log` records actual native binding execution, 2 selected and 2 passed in 0.16 seconds. The subsequent unused-`mut` cleanup is test-only and does not change that behavior.

The rebuilt installed artifact is `9d76d93817fa0f9d76cc5d5e7ed291cae751fea1c64de5427cc864ddf54690d4`. Its successful 17.71-second build is recorded in `/private/tmp/fx268-codex-owner-remediation-python-build.log`, with the actual imported and archived extension identity recorded in progress. `/private/tmp/fx268-codex-owner-remediation-python-tests.log` records 10 passed in 123.83 seconds: six run, cell and header piece controls at 20 and 80 paragraphs, two dense forms/settings compositions and two installed shared-header identity cases.

`/private/tmp/fx268-codex-owner-remediation-clippy.log` records affected Python-crate all-target all-feature clippy with warnings denied, successful in 13.57 seconds. The unaffected native/OXML combined scoped lint, native source and composition checks, native six-operation counted-work observations, original 14-operation Python work checks and eight-case actual release fallback execution from pass 1 remain applicable to their unchanged paths. The new ten-case installed batch covers the changed WordPieces admission. Rustfmt, prose and diff-whitespace checks pass. No unaffected suite was rerun merely for this review.

## Acceptance boundary

A3's two local blockers from pass 1 are closed. The native and installed composition, A10 counted work and scoped A11 release correctness observations remain local evidence. Pass 1's remaining limits are retained: A6 requires all 79 release timing gates, the complete 252-call report and original Issue 407 gate. A7 requires per-layer and combined memory, cold retention and history measurements. A8 requires full integrated hash, typing, packaging, WASM and native Word qualification. A9 still requires the complete five-call final counted and timed release qualification. Applicable signed and embedded risk riders and the prior stories' final gates remain pending.

This clean whole-feature microscope permits return to the normal scoped completion and integration workflow. It is not final story delivery, a clean integrated sprint review, ready-to-close status or release approval. No additional confirmation round is required by this review.
