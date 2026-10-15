# F-X216, notes placeholder content, pass 2

**Reviewed**: Bounded D1 and legacy renderer fixture remediation, sealed by `notes-placeholder-content-frozen-2/manifest.json`, SHA-256 `9fe9a6bac827fb24b7aeee18f62d4ac2e2de5f0c19156e6e374221d17718fdef`. Three carriers are frozen. Facade library bytes are unchanged from pass1. Production delta is confined to the existing shape-tree materializer. Prior pass1 report `4539aa7710e56ed3544beb96b7d4ec1154eca6e28978a9e56836d12a95860ebd` remains intact.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None. D1 is resolved within the reviewed contract.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: zero findings. `crates/rpptx-oxml/src/shape_tree.rs:3355` records actually skipped placeholder IDs after their owning source remap. At3422 only absent master placeholders enter that set. At3520 the fully staged serialized candidate is checked for qualified retained references to those IDs. The error occurs before assignment to self at3534. Matched placeholders map to the actual local ID and do not enter the skipped set. No phantom placeholder is created to satisfy a connector.
- Contract: zero findings. Named refusal preserves absent-placeholder occupancy, retained decoration semantics and atomic local publication. It changes neither ordinary rendering nor source-master state. The five other approved graph/provider behaviors are not redefined by this check.
- Panics: zero findings. The new set and fallible candidate serialization/reference scan introduce no unchecked indexes, arithmetic or unwrap. Existing physical namespace errors propagate before publication.
- OOXML: zero findings. The scan uses existing `shape_id_occurrences` and namespace-qualified connector/PresentationML reference recognition at`crates/rpptx-oxml/src/shape_tree.rs:241`. Foreign attribute and element lookalikes do not become owned shape references. Raw child placement, ID allocation, namespace closure, original source metadata and schema order are unchanged from the reviewed predecessor.
- Tests: zero findings. `crates/rpptx-oxml/src/shape_tree.rs:5063` supplies a real master header placeholder and connector endpoint. The absent case checks the named refusal and exact local byte identity. The matched case proves both the actual local definition and retained endpoint use42. The master bytes remain identical. The genuine predecessor test fails by returning success instead of the required refusal, and the successor tree selection executes10 passing controls including this defining case.
- Structure: zero findings. One operation-local concrete ID set and existing qualified scan are used without new types, modules, dependencies or generic validation machinery.

## Renderer fixture reconciliation

`crates/rpptx/tests/integration.rs:9254` now supplies an actual local slide-image placeholder with sparse shape properties. Master geometry therefore remains the authority for the existing thumbnail-edge assertion. Actual local header, date, footer and slide-number field are supplied at9256. The body, ordinary note picture, page order, page-count assertions and metadata/geometry checks remain. The field is genuine `slidenum`, rather than ordinary text reclassified solely to satisfy a painted number.

`crates/rpptx/tests/integration.rs:10241` uses that matched current fixture as the control and adds only an unmatched overlay for the negative. The original equality, dimensions, speaker/number checks and source byte-preservation assertions remain. No ordinary matched local text is discarded or silently treated as equal to an unmatched shape. Header/footer labels avoid adding extra F-226 replacement targets, preserving the separate existing text-mutation count purpose. The original renderer failures and intermediate fixture-successor1 text-mutation failure remain retained. The final receipt executes both original renderer names and the text-mutation control successfully, along with notes owner, handout, resource, invalid hierarchy/DPI and package byte-preservation controls.

## Authentication and qualification limits

Manifest, all current/live and predecessor carrier hashes, source map `a7618123442be11e036d6b89b45d4992a820558184937dd18241ff248a18d6bb`, retained integration binary `9921564e4e2a23967506258520a2ef8862cddbdefd58ba6feb0684624f44b783`, all cited receipt files and raw log hashes authenticate. Current carrier bodies, sizes and modes match before and after this audit. The sole implementing writer explicitly confirmed the freeze.

Genuine reference RED receipt `c0d12d94251633859a6976da9664d32c37fdbbc1207a5e0e328920327541ffec` preserves named1FAIL/exit101. Successor2 receipt `4d498b09c4fe1a1740429168e0cc540aff95bdb41a41ece25d94b461439ff45c` proves appearance1, nine exact legacy notes/handout controls, tree10 and strict Clippy, all exit0 with safe cleanup. Separate native-contract receipt `57cd00c713c0c7cadad96921bbba7720e32810fc56ebe7f58e5d026452766dec` authenticates its actual33 selected import controls, tree10, unit1, scoped check and strict Clippy. These are library/test qualification, not native Office UI evidence.

Six current artifact hashes authenticate and retain source/imported package `71b9df47d7d6aec67f70fbb988c956ca802b5fc04f70f10af427132bc1321987` / `2a0206babd09fbad1bd4f09ad60d730e24f84bc051add4ced683eab342bac06d`. Own-render PNG equality remains exact. Native Office acceptance is pending independent evidence. Fresh installed qualification, whole-feature gates, sprint verification and release remain uncredited by this bounded report. This review changed only this file and ran no tests.
