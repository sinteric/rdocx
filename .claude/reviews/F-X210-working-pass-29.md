# F-X210, whole working implementation, pass 29

**Reviewed**: Complete working diff against worker HEAD7bbc960e54d2b5acf6763c8d6d13b3796c1a5a7a. Exactly34 existing tracked carriers,2402 insertions and73 deletions, plus9 approved assets totaling444812 bytes.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authority, scope and independence

Read CLAUDE, WORKFLOW, microscope, risk routing, the approved canonical F-X210 design, cited preservation/rendering/binding/build sections, current progress and prior reviews. F-X210 remains in progress. Current2909 source records match bytes, sizes and modes against `/private/tmp/S91-F-X210-crate-README-measurements-r1-prepared/twelve-key-baseline-candidate/source-after-baseline.json`, SHA256 `ac9309581326e14c53d8a8e79a3d9b6b74465a62d268f548dee739a1ed37e7bf`. HEAD and index remain unchanged, index SHA256 `7aef5efe3c9b04d5d50070222990762ac9fd664bbbec4f59fa2d13631c1437be`.

I contributed earlier owned-diagnostic/companion proposals, HLD and metadata preparation. This report does not claim independent self-review of those historical authored portions. Their independent reviewed closures remain authoritative, including provider3, owned-consumers8, companion9/10, HLD11, remaining-routes14, metadata27 and measurements28. This pass audits the full current composition and acceptance reconciliation without modifying the source. The earlier feature defects and failed runtime receipts remain retained.

The nine asset hashes match the approved original eight-file proposal and separately approved3968-byte subset. No contributed replacement font, new Rust/Python module, dependency or optional pack was introduced. Existing F-X195 provider behavior is reused, not reimplemented. Its unchanged source is in worker HEAD, while the F-X210 additions are identified by the complete working diff.

## Defects

0 findings.

## Smells

0 findings.

## Nitpicks

0 findings.

## Six aspects

- **Correctness**: 0 findings. `crates/oxml-layout/src/font.rs:1532` authenticates the selected physical collection face from original bytes and actual index rather than caller labels. At line1557 the two Aptos spellings produce first-use approximation facts only for accepted nonempty contributions. The current-layout reset at line1010, scope rollback at lines1132 and1175, and validated contribution replay at line1393 retain earlier committed facts without retaining provisional probes. Georgia and Segoe UI arrays preserve authentic requested-family precedence and existing secondary fallbacks. The finite symbol table at line1597 preserves genuine original-family coverage and separates the existing PPT F0B7 compatibility route from arbitrary PUA conversion.
- **Contract**: 0 findings. Word companions at `crates/rdocx/src/document.rs:28940` and PPT companions at `crates/rpptx/src/lib.rs:1133` encode or paint one owned layout and return its diagnostics with the original payload shape. Word archival, native PPT archival/notes/handout, installed PPT notes and existing SVG coverage match HLD10. Python archival and handout companions are not claimed. `crates/rpptx/src/lib.rs:1507` resets the font scope for each measured shape before constructing the immutable frame record at line1532. Notes and handout assembly deduplicates only actual equal font facts at line10292, while preserving ordinary warnings and distinct selected-face decisions. Timeline scope starts at line4885 and returns its actual facts at line5034. No persistent last-render registry or diagnostic rerender exists.
- **Panics**: 0 findings. New physical lookup uses checked index and fallible face parsing. Full cached contributions are validated before replay publication. Companion failures return before payload publication and do not mutate the document. The export diagnostic suffix subtraction at `crates/rpptx/src/lib.rs:11052` is bounded by the inspected same-call renderer append at `crates/rpptx-render/src/lib.rs:337`. Existing facade page/slide selection and raster admission remain unchanged. New unwraps and literal indexing are confined to authenticated positive test fixtures.
- **OOXML**: 0 findings. Word symbol lowering at `crates/rdocx-layout/src/engine.rs:9718` and marker lowering at line8825 alter displayed segments only. PPT marker shaping at `crates/rpptx-render/src/text.rs:451` uses the same finite resolver. Stored codepoints, run XML, numbering, relationships and schema order remain unchanged. Existing source/reopen assertions and genuine-input controls retain exact original package XML. Unknown mappings, unrelated PUA and Wingdings2/3 remain outside the substitution table. No namespace or serializer relaxation appears in this feature diff.
- **Tests**: 0 findings. Original provider Base2 and four public consumer Base controls reach genuine semantic101 after successful compile/collection and contribution setup. They discriminate reverting the mapping, diagnostic or symbol implementation rather than missing future APIs. Current selected-instance tests cover four requested styles, original bytes, TTC index, synthesis and warm identity. Lifecycle, nested rollback, overflow and cached replay controls retain committed warnings. Public native and installed controls require unchanged payloads, one layout, actual ordered warnings, immutable frame facts, empty-selection refusal and source/reopen equality. Notes-only and handout-only controls remove the old masking main-slide Aptos request. Genuine16 controls authenticate original physical caller sources and reachable painted CIDs. Joint checkbox B and PNG positives are1.0 for all4 owners and strict negatives remain below.99, with A retained as nonacceptance evidence. No threshold, bitmap registration or source-oracle replacement was introduced.
- **Structure**: 0 findings. Changes remain in existing approved carriers plus the exact nine authorized assets. The helper methods perform actual physical authentication, bounded mapping or concrete ordered fact assembly for named current consumers. No new trait, generic, crate, feature flag, product file, module, downloader or network runtime was added. The public native TextFrameLayout field impact is explicitly documented at `docs/hld/10-bindings-spec.md:2463`. Existing byte APIs and Python defaults remain unchanged. Native7-package verification and full22-local-patch archive proofs cover the published source impact.

## Actual gates and source equivalence

Authenticated the consolidated scoped evidence and107 referenced raw logs. Its immutable receipt is `/private/tmp/S91-F-X210-crate-README-measurements-r1-prepared/consolidated-scoped-completion-evidence.json`, SHA256 `b61ed332ad25d1912e9f67f1b18cef7ff9454ce6b46e8b1630ea0ef60a193084`. Current Rust, font, binding and stub sources match the actual native/wheel producer967978. The16 intervening records are13 metadata carriers, two independently reviewed installed fixture corrections and the reviewed baseline. They do not justify replaying completed native or paint suites.

Retained qualifications cover provider7, owned native17 controls, affected9-crate checks/Clippy, full7 changed nonbinding native scopes, direct serial Word integration with the unchanged recovered public-conformance selector and two exact26.01 controls, fresh paired wheels, installed companions, four typing/stub gates, genuine16 original-face controls, native metric evidence, joint checkbox paint, no-default141 plus3 docs, both WASM targets, license/raw membership and seven genuine verified dry-run archives. Failed wrappers, timeouts, corpus/tool setup failures and original failed policy outcomes remain failed. Successful named recovery rows are composed explicitly, never relabelled as fresh full runs.

Actual final metadata/policy tool84741 closed0. Receipt SHA256 `bbf9b470473ea68f0b384d4ae27ac3fd388f0782c66f69256655eff92eb3a4fe` records six final archive measurements, the two previously failed policy selectors, prose and diff checks. All owned groups were reaped and gone. Final member totals/counts are exact and compressed measurements remain inside the existing64-byte allowance. Original policy141 comprised137 pass,2 fail and2 existing skips. Both failures now pass, giving composite139 pass and2 skips, not a fresh141-pass run.

ROOT adopted only the independently reviewed12 Georgia-to-Gelasio baseline values, preserving37 entries. Actual tool92151 then closed0 with `hash_harness: 49 entries match`. Its receipt is `/private/tmp/S91-F-X210-crate-README-measurements-r1-prepared/twelve-key-baseline-candidate/actual-hash49-1/receipt.json`, SHA256 `e4715741e26b8442e5d15c542a6e41b6ebd4341dcd36261d2e33f6d3813288a1`. Full current source guard matches, leader is reaped and PG98848 is freshly ESRCH. No native replay or baseline re-record ran during that qualification.

HLD08 retains measured ligature, kerning, vertical and wrapping differences and absent genuine Segoe Bold parity. HLD10 matches actual native/Python availability. HLD15 and CI assert32 bundled fonts and10 legal files. Seven package measurements and final README cells follow authentic archive source membership, not donor tuples. No universal Georgia/Segoe metric or native-outline equivalence is claimed.

## Handback

This report supplies zero-defect, zero-smell review of the current composition with the disclosed inherited review authority. It does not mark the story complete, create a commit, publish a crate or generate a handoff. ROOT retains the reviewed labelled behavior commit, exact owned partition and normal `/complete-feature --prepare` validation. Integrated sprint verification remains due. Review ends when this file is written and the review lease returns to ROOT.
