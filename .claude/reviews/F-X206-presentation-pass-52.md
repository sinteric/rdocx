# F-X206 PowerPoint aspect review, pass52

**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Scope and authority

Read-only microscope of the current PowerPoint authoring, DrawingML model, resolver, renderer, native facade, Python binding and applicable HLD projection. This is an aspect review, not the complete shared Word/foundation or final integrated feature review. ROOT separately reviews that foundation. The original37-carrier input remains useful PowerPoint material although the corrected45-carrier ownership partition supersedes its incomplete feature inventory.

The frozen worker is `/private/tmp/rdocx-f-x206-codex`. All2714 physical source records match `metadata-source-after.json`, SHA256 `0d0739c6dd1e641900dfa775d67b4e88c877d70dbef54240a71bbf8b50cbbb5d`, including bytes, sizes and modes. HEAD is `7b1e5ef29464e93ae30e68df1523ce09143578ff`. The staged-entry inventory, SHA256 of `git ls-files --stage -z`, is `c37a36fb85cd0d0055de6dad78d342e20dd2084e45113fc2d1424f7c3338175c`. This is not the hash of the raw index storage file. There is no staged source change.

Review baseline is accepted F195 prefix `2485015090c4817b394abec40540c9acc55f3f6c`. The PowerPoint diff is `owned-native-and-bindings.diff`, SHA256 `414f4b69a0ef985cb499aee9b0c9e977ec9d121a0b1eda788f6bbd038e779015`, 1076128 bytes,37 carriers,17988 additions and1449 deletions. The eight-HLD diff is `owned-hld.diff`, SHA256 `a15a863982eb7526ddf8479aec43812a65dba25d74c2910e2e3539a54121b42c`,46696 bytes,242 additions and224 deletions. Both are under the worker's `.claude/scratch/F-X206-placeholder-fixture-remediation-r18/installed104-decorator-relocation-r59/final-owned-review/`.

Read canonical CLAUDE, WORKFLOW and microscope instructions, approved F-X206 design and risk obligations, current status and progress, original working-pass1 D1-D5 and subsequent ownership/preservation dispositions. Current sprint and backlog still classify F-X206 as in progress. I historically prepared geometry, namespace and compiler/runtime proposals in this feature. Those unchanged portions retain their prior independent reviews. This report does not claim that my earlier authored proposals became independently reviewed through self-review. The present picture-admission and HLD-partition findings arise from reading the composed current source and projection.

## Defects

### D1. Picture conversion treats opaque text-body root facts as an empty body

`crates/rpptx-oxml/src/shape_tree.rs:1225` admits conversion when the imported text body compares equal to either parsed empty_body or CT_TextBody::new(). That comparison uses `crates/oxml-drawing/src/text/mod.rs:57`, whose equality compares modeled body properties, list style, paragraphs and raw children, but omits source_root. Root attribute tokens, including unknown attributes, are retained by `crates/oxml-drawing/src/text/paragraph.rs:344` and stored when parsing at `crates/oxml-drawing/src/text/mod.rs:165`.

A source-built picture placeholder can therefore contain this otherwise empty body:

```xml
<p:txBody xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:x="urn:producer" x:keep='&#65;'><a:bodyPr/><a:lstStyle/><a:p/></p:txBody>
```

Its x:keep token has no modeled effect, so the admission comparison accepts it. The conversion then constructs the picture from nonvisual properties, blip fill and shape properties at `crates/rpptx-oxml/src/shape_tree.rs:1332`. It does not retain a text body. The original opaque root attribute is lost when the candidate is published. This violates the approved exact opaque-source preservation and atomic unsupported-conversion refusal boundary. The existing conversion test at `crates/rpptx-oxml/src/shape_tree.rs:3426` checks nonvisual/geometry opaque payload and refusals, but does not discriminate an empty modeled text body with opaque root facts.

The admission test needs a source-preservation qualification rather than treating general semantic model equality as proof that this discarded owner contains no opaque facts. A valid ordinary empty body must remain supported. An unrepresentable opaque text-body root must be preserved through a legal representation or cause the existing named refusal before publication. No global equality change or test relaxation is implied by this finding. This witness was derived from source, not executed during review.

### D2. Owned HLD projection deletes accepted provider and unrelated feature contracts

The proposed eight-file HLD projection is not restricted to F206 intent. In `final-owned-review/owned-hld.diff:615`, it introduces an obsolete Word caller-only font paragraph ending with the claim that SVG and raster take no caller fonts. The current worker repeats this at `docs/hld/10-bindings-spec.md:2434`. It contradicts the accepted F195/F210 caller-font and output-owned layout prerequisites. The diff also deletes the selected-instance SVG declarations at `final-owned-review/owned-hld.diff:264` and the accepted provider routing paragraph at `final-owned-review/owned-hld.diff:320`.

The same artifact removes unrelated F202 Issue339/PR342 authority at `final-owned-review/owned-hld.diff:681` and the F213 TOC contract at `final-owned-review/owned-hld.diff:717`. The partition lists these whole eight HLD files as owned without implementing hunk exclusions. Applying this projection as the F206 documentation delta would regress accepted contracts and remove other owners' intent.

Reconcile only F206-specific HLD clauses onto current canonical documents, preserving completed F195/F210/F213 and retained F202 authority. The explicitly required source-level font overlay reconciliation does not make these documentation deletions safe. This is a projection defect, not evidence that the already accepted canonical font or TOC implementation has regressed.

## Aspect dispositions

- Correctness: D1. No additional concrete defect established in physical-owner selection, staged group/ungroup, affine arrangement, live connector rerouting, naming, cell-anchor precedence, internal run relationships, caps/highlight shaping, recovery or facade publication.
- Contract: D1 and D2. The old working-pass1 naming, cell-anchor, picture API, internal target and ancestor highlight branches now exist with deciding preservation controls. Their presence does not resolve D1 or grant whole-feature completion.
- Panics and error paths: 0 additional findings. Inspected input checks and candidate/publication boundaries retain checked geometry, foreign/stale-owner ordering, relationship validation and allocation-before-publication. Private invariant expects are tied to validated paths or exclusive borrows.
- OOXML: D1. No additional concrete namespace, qualified attribute, schema-order or opaque-child defect established in the inspected source. Imported aliases/default scopes, foreign r closure/refusal, accessibility extensions and highlight replacement use explicit owner qualification.
- Tests: 0 separate defects. The missing D1 discriminator is described with its production finding. Existing native and installed assertions retain source, owner, atomicity, geometry, cluster, corpus and reopen requirements. No threshold or expected-output change is justified by this review.
- Structure: 0 findings. The new concrete scope records, caps occurrence shaping and local geometry helpers have actual consumers. No new trait, module, dependency or forwarding-only abstraction was established in this slice.

## Evidence and limits

Authenticated `evidence-index.json` SHA256 `10193edbb0b7edf56ff8abcdb1f71b2881f67573f3a6b8b4f6667091d40b981b` and `review-evidence.md` SHA256 `18e7b7225f68244d153a73f31a389e2b3edeb7054d6b7e2bd8815a9e60604c7f`. All16 indexed receipt hashes and64 indexed log hashes match the physical artifacts. Earlier failed wrappers remain failed. Later named retries and source-equivalence qualifications do not claim a fresh whole-suite invocation. Retained104, eight-owner sixteen-Q/C SVG, corpus, native, installed, typing and archive observations remain scoped to their actual source and provenance. The two borrowed F200 Word XML hash changes remain assigned to their owner, with no F206 baseline mutation.

The worker predates completed canonical F210/F213. Its font/caps and facade changes require localized composition onto those current providers, preserving assets, selected-instance behavior, operation-owned diagnostics and TOC contracts. The reviewed PowerPoint slice does not qualify the corrected shared Word/foundation projection, future isolated compilation, integration source equivalence or feature completion. No Cargo, runtime, product import, native UI, source, environment, baseline or index mutation occurred in this pass. Only this review report was written. Remediation is a separate phase.
