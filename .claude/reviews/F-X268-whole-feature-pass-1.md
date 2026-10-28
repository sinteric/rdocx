# F-X268, whole feature, pass 1

**Reviewed**: The complete F-X268 product delta from recorded claim Base `9d2878b0c8d7cb0e1b8d9d3d697b5117893ac1bc` through worker HEAD `5759559029d2af5c61ad5e69e3ebc3af8a8557fe` and the frozen working tree on `work/f-x268-codex`. The tracked delta contains 37 files, 46730 added and 30998 removed lines, including prior feature-local plans and reviews. The product prefix is the integrated `79047bf193bcd9dfb229a0b37a93d979dc0997a7` tree. This pass covers all eight mechanisms and their native, binding, stored-source and reopen consumers.
**Reviewer**: Independent Codex agent `/root/review_source_pieces`. The reviewer did not implement the inspected changes, modify source or rerun builds or tests.
**Verdict**: 2 defects, 0 smells, 0 nitpicks. One defect is missing required local runtime coverage. One is a physical-part fallback contract mismatch. No output corruption is inferred from the passing cases.

## Source identity

| File | SHA-256 |
|---|---|
| `crates/rdocx/src/document.rs` | `bb913fe7589260be55940e519ba01b00969043d8d6abc5b5811ebf775002ca10` |
| `crates/rdocx-oxml/src/document.rs` | `b7eba9fac61355a2a6f3d687fa86d6564526cad3b29281b6db250b4ddccaf5e8` |
| `crates/rdocx-py/src/document.rs` | `ff2aac81347f906cafa53de82684d9b2c305b48ff0a1c93e7d4fe9d38c7e4edf` |
| `crates/rdocx-py/tests/test_core.py` | `66bd3a0f201fa201d6c99927e7e07bf5f6190f20ad34cfc49a55d1d0448bd974` |

## Defects

### D1, the changed-existing-context fallback still has no runtime regression

`crates/rdocx-py/src/document.rs:5371`
`crates/rdocx-py/src/document.rs:5462`
`crates/rdocx-py/tests/test_core.py:14698`
`.claude/reviews/F-X268-owner-context-pass-2.md:23`

The approved A3 contract requires an existing changed body context to select the authoritative full owner inventory before publication. The source contains that branch, but its earlier bounded repair explicitly left actual runtime coverage pending. The current added A3 composition repeatedly changes paragraph text, preserves forms and settings, then performs a structural insertion. It does not prime `WordPieces`, change `body_item_context` on the same live document and prove that this specific branch invokes the full inventory. The M7 frame-invalidating tests exercise a different cache and do not discharge this row.

Add and execute a discriminating live-document transition. It must distinguish a changed existing context from an initially absent context, observe the authoritative fallback before publication, verify physical owners, XML and established handle policy, and exercise a subsequent stable-context edit. A source inspection or all-target compilation is not the required runtime observation. If the control is implemented as a native binding test, run that actual test without treating its type-check as execution. No new product fault is asserted for the currently inspected branch.

### D2, case-equivalent duplicate story parts evade the promised full fallback

`crates/rdocx-py/src/document.rs:5364`
`crates/rdocx/src/document.rs:24950`
`crates/rdocx/src/document.rs:42687`
`crates/oxml-opc/src/package.rs:537`
`crates/oxml-opc/src/package.rs:894`

`WordPieces::assemble` identifies duplicate physical story parts using a case-sensitive `HashSet<String>`. Related source discovery also keeps distinct target spellings. For example, two referenced header relationships targeting `header1.xml` and `Header1.xml` produce distinct names, while OPC lookup resolves both names to the same physical part through `deterministic_part_key`. The piece path therefore admits two physical-source aliases instead of selecting the duplicate-part full-inventory fallback required by mechanism 2 and A3.

Use the package's normalized physical identity for this admission decision. Preserve the valid shared-header document and the authoritative full-reader behavior rather than introducing a new input refusal. Add a case-equivalent shared-part control that proves the fallback actually occurs, alongside an ordinary unique-source or already-deduplicated shared-source positive control. This finding is established by source and caller inspection. No runtime RED is claimed in this review and no output corruption is inferred solely from the admission mismatch.

## Smells

None established beyond the defects above.

## Nitpicks

None.

## Not found

The inspected native checked insertion methods preserve the four legacy methods and put index and XML text checks before namespace refusal. The Python seven-path guard reaches the same native policy while preserving its existing index and style conversion behavior. The regression captures actual pre-refusal model, XML and live owners rather than equating two failed saves with atomicity.

The owner-piece path retains exact block and related-part provenance, excludes field, tab and bookmark private facts from value-only reuse, validates live hyperlink relationships and uses the full inventory on unavailable piece facts. Prepared owner indexes, result allocation and revision scopes are created before commit. `install_inner` installs prepared pieces or clears unprepared pieces. Paragraph and table return routes use the prepared inventory with the existing native fallback. D1 and D2 identify the remaining admission evidence and identity gaps, not a general failure of those mechanisms.

Opening and reread proofs bind current bytes, namespace context, actual parsed-model equality, successful serialization and relationship dependencies. New, Flat OPC, limited and hook paths keep their ZIP fallback. Related styles, numbering, headers, comments and identifier categories remain checked in their established order. Retained source facts are distinct from a dirty live model and from current writer facts. Clearing the opening is correctly documented as removing trusted reread provenance rather than universally disabling child proofs.

Direct addressing retains one covering body frame and exact aligned child bytes. Unsupported cross-body or opaque spans retain the checked full reader. Prefix and suffix alignment preserve occurrence and order. Source reconstruction uses actual parsed children, exact partitions, checked arithmetic and global resource admission before publication. Normalizing source bytes do not grant writer reuse. The narrow producer whitespace proof does not accept alternate roots or arbitrary frame changes.

The mechanism 8 scoped proof compares normalized frames, validates decodability, checks physical child count and position, uses a collision-free sentinel and performs actual double parsing and writer normalization where required. Identical sibling text cannot select a different child. Field-bearing and non-round-tripping blocks are normalized and read again. Head, tail and unchanged-child decoding failures return to the old whole algorithm. The edit helper validates ordered disjoint bounds before splicing and refreshes changed context facts. Relationship and drawing canonicalization preserve exact retained source publication and keep honestly counted whole fallbacks. The current serialization memo holds no published part or accumulating history.

No additional correctness, panic, namespace preservation, schema-order, private-field, candidate-publication, identifier-order or structural defect was established in those inspected paths. New abstractions remain concrete and local. Debug counters remain hidden and retain their six distinct units. These conclusions do not substitute for the final performance, memory or native Word observations.

## Acceptance coverage and limits

| Criterion | Inspected evidence and current limit |
|---|---|
| A1 | All four native checked methods, seven Python publishers, old-method exemption, bounds, character order, zero dimensions and atomic namespace refusal are implemented and have focused native and Python controls. |
| A2 | Qualified facade splice and replay paths, unchanged raw bytes and source-fact invalidation are covered by the native source and namespace controls and the existing qualified-splice Python cases. |
| A3 | Physical inventory, direct routes, private forms/settings composition, held and fresh reads, save/reopen and source-byte controls are present. D1 and D2 prevent closing the complete local criterion. |
| A4 | Existing head, middle and tail splice, duplicate occurrence, later edit, no-op, refusal and causal staleness controls remain. Prepared publication preserves the existing matching and retirement policy. |
| A5 | New, opened, cloned, source-edited and related-only state, native error order and actual in-memory versus ZIP reopen controls are present. New forms, custom XML, defaults, current values, settings and mail-merge composition controls pass. Applicable signed, embedded and final integrated risk checks remain named final-union obligations. |
| A6 | All 79 flat-cost gates, complete 252-call report, residuals and original Issue 407 timing gate remain pending final-union release measurements. No debug count result is credited as timing. |
| A7 | Per-layer and combined RSS, cold retention, clone/reopen/drop and history measurements remain pending. No donor number, hard cap or summed RSS claim is accepted. |
| A8 | Installed typing, stubtest, packaging, rustdoc, WASM, complete hash and prior-story Word/full qualification remain pending final union. |
| A9 | The shared body-coverage sweep has a 250/1000 counted-work control. The complete five-call counted and timed release qualification remains a final-union obligation. This review does not infer five-call acceptance from that one shared sweep. |
| A10 | The original three native operations, three new native operations and all 14 Python operations are represented by real changed warmups and distinct samples. All six units, eligible reopen work and genuine force-off growth are checked. Counterless release execution retains functional assertions. |
| A11 | The actual eight-case release fallback batch supplies scoped release equality, error-order, context and atomicity observations. It does not discharge the complete final suite or native Word fidelity. |

## Actual evidence consumed

The current Python artifact is `80815612f9f6bca2b328615bb72369e00d81fda35d6be8cc5194b9c23ed6b049`. Its successful build and actual imported archive identity were recorded by the implementing host. `/private/tmp/fx268-codex-m8-python-batch.log` records 16 passed tests in 254.14 seconds, covering the 14 A10 operations, alternate-root control and refusal control. `/private/tmp/fx268-codex-composition-python2.log` records 2 passed in 7.64 seconds. The earlier composition failure was an incorrect test expectation about longstanding conservative `set_font` staleness. The corrected case explicitly verifies that established retirement label and fresh authority.

`/private/tmp/fx268-codex-composition-native2.log` records 2 passed in 3.90 seconds. Its earlier unknown test cache field was a compile failure, not semantic RED. The original native A10 three-operation group passed in 69.57 seconds. The new three-operation native group passed in the M8 batch. The later context batch passed 4 tests in 2.57 seconds, including actual growing whole-reference work. The independently allocated field source and owner IDs are excluded only from the private diagnostic equality comparison. Other private facts, normalized XML and stored parts remain compared.

`/private/tmp/fx268-codex-combined-scoped-clippy.log` records affected native OXML, facade and Python all-target all-feature clippy with warnings denied, successful in 25.51 seconds. Rustfmt, prose and diff-whitespace checks passed. These compilation checks are not runtime execution of the native tests inside the binding crate.

`/private/tmp/fx268-codex-release-fallback-batch.log` records an actual release binary with debug assertions disabled: 8 selected, 8 passed in 0.25 seconds after the normal optimized build. The selected tests cover disjoint edit bounds and invalidation, current frame fact reset, native composition, retained story picture whole agreement, duplicate/marker/source contexts, duplicate splices and frame fallback, source-read versus writer eligibility, and picture decode refusal without mutation. Their functional and whole-reference assertions remain active in release. A debug whole oracle is not substituted for this result.

Earlier bounded source-piece and mechanism reviews were consumed as supporting evidence for unchanged components. They were not treated as complete F-ID acceptance. No test, build, benchmark or external oracle was executed by this review.

## Required next boundary

Address D1 and D2 together in a separate implementation and regression batch, then return for a consolidated remediation review. Final release timing, memory, hash, whole typing, packaging, WASM, Word and integrated suite obligations remain pending, as the approved workflow requires. This report supplies no feature completion, integration approval, ready-to-close state or release approval.
