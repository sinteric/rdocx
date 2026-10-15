# F-X216, notes placeholder content, pass 1

**Reviewed**: Three-carrier bounded delta sealed by `notes-placeholder-content-frozen-1/manifest.json`, SHA-256 `56c7bdad54af780461209b7845a00a2d11d306a62c5b8abb2987d6d0a6797836`. Actual predecessor and current copies cover `rpptx/src/lib.rs`, `rpptx-oxml/src/shape_tree.rs` and existing integration tests. Review includes separate authority assessment of two unchanged legacy renderer controls. No whole-feature verdict is issued.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, retained decoration references can point to skipped master placeholders

`crates/rpptx-oxml/src/shape_tree.rs:3184`, `crates/rpptx-oxml/src/shape_tree.rs:3216`, `crates/rpptx-oxml/src/shape_tree.rs:3417`

The initial map allocates a fresh ID for every source-master shape definition, including an unmatched master placeholder. Qualified connector endpoints and PresentationML shape references are rewritten through that map before materialization. The new occupancy branch subsequently omits that placeholder, while ordinary decorations containing those rewritten references are appended unchanged. A source-master connector targeting an absent header placeholder therefore retains an endpoint ID with no destination definition. Allocation deliberately excludes occupied local IDs, so it cannot identify an unrelated surviving local shape. The candidate is assigned at3515 without verifying that retained references still resolve.

Retain the new absence semantics. Before publication, distinguish actual retained or matched shape definitions from skipped master hints and refuse an unrepresentable surviving reference by name, or apply an explicitly qualified owning reference policy that preserves appearance. Do not instantiate a missing placeholder merely to satisfy a dangling connector. Add a deciding retained connector to skipped-placeholder control, a corresponding matched-placeholder positive and exact local/master byte identity on refusal. Qualified references are collected at`crates/rpptx-oxml/src/shape_tree.rs:241`, so this is an actual current consumer, not a hypothetical reference kind.

## Smells

None.

## Nitpicks

None.

## Other aspects checked

- Correctness and contract: absent source-master hints are omitted at`crates/rpptx-oxml/src/shape_tree.rs:3417`. Matched local absent and empty text bodies are no longer replaced by master sample text at3396. Decorations remain copied. The skipped-boundary projection searches actual next and previous retained positions at3435 and refuses reversed raw order. Apart from D1, no further finding.
- Panics: zero findings. Render field resolution iterates the current paragraph count and borrows the corresponding existing paragraph at`crates/rpptx/src/lib.rs:24480`. No externally supplied unchecked index is added.
- OOXML: apart from D1, zero findings. Existing physical namespace metadata admission and staged candidate publication are unchanged. Field text is preserved in the saved native owner. Number resolution changes only the render clone and only actual local `slidenum` fields, including empty caches, at`crates/rpptx/src/lib.rs:24487`. Ordinary local text is retained. The Some-notes overlay at24376 removes uninstantiated master shape placeholders. The None-notes path does not invoke that overlay. Handout filtering still uses `keep_body=false` and its existing whole-number replacement branch at24502.
- Structure: zero findings. The changes stay in existing concrete owners and consumers without a new file, dependency, generic framework or forwarding wrapper.
- Tests: the new tree control at`crates/rpptx-oxml/src/shape_tree.rs:4974` distinguishes absent body, empty body, actual empty-cache field, absent master header and retained decoration/raw boundary. The strengthened appearance control at`crates/rpptx/tests/integration.rs:34286` compares actual source/imported occupancy and absence of an inherited number body. These prove their named boundaries, but do not cover D1.

## Legacy renderer test authority

The unchanged fixture `crates/rpptx/tests/integration.rs:9248` instantiates only the local body and an ordinary picture. Its master has `sldImg` at9162 but the actual note has no local slide-image placeholder. Both retained renderer controls fail with zero visible slide-image placeholders at`crates/rpptx/src/lib.rs:24305`, before their intended geometry and unmatched-overlay assertions execute. These are genuine failed test executions, not passing acceptance.

A separate fixture reconciliation should instantiate the actual local slide-image placeholder using sparse properties so master geometry still supplies its exact edge. Preserve `notes_pages_follow_master_geometry_text_metadata_and_slide_order` at10112: two-page order, speaker text, explicit local header/footer/date content, an actual local slide-number field, background and thumbnail edge pixels. Master sample text cannot substitute for missing actual local content under this admitted occupancy contract.

Preserve `notes_pdf_skips_only_unmatched_slide_placeholder_overlays` at10227: unmatched overlay diagnostic/omission, image dimensions, original package byte identity and meaningful paired PDF/PNG comparison. Its matched ordinary slide-number text at10237 now remains ordinary local text, whereas the unmatched owner is skipped. Identical-output expectations must use a defining matched field case and explicit expected painted output, or a separate correctly scoped unmatched fixture. Do not waive a real matched ordinary-text output difference or delete the unmatched-overlay purpose. No test changes were made in this review.

## Authentication and limits

All three current/live and predecessor hashes, source map `f79b0ceabe1fe81f7a7b49a3de6cc1cc159472dda730c8214595c4dbe23828b4`, receipts, raw logs, retained binary `91708f0f9402b32fa5bfa13be2da08620fc826877aa36a996f4fe85ac2b9ca72` and six artifact hashes authenticate. Current source bodies, sizes and modes match before and after audit. The implementing writer confirmed the freeze.

Genuine RED2 receipt `6e6358bd613af6ba35a61e31ff58c0535fdc132946a3bc7709af09f326ef6684` retains the named appearance failure. Successor4 `44834643f29a5e6192398975d93404a6489ad69eb08c16341516a825a9c2307b` has actual appearance1PASS, nine tree controls PASS and strict Clippy exit0 with safe cleanup. Source/import deterministic PNGs are byte-identical `0e9383d93eea9c90d20270845c202aca1bd784f2cdce51734a568b272f84272e`. This is that fixture's own-render equivalence, not independent native Office acceptance. Legacy renderer receipt and both raw failed logs authenticate separately. Root native appearance capture is underway and is uncredited here. Fresh installed, broad suites, package, full sprint and release gates remain pending. This review changed only this report and executed no tests.
