# F-X211a, native remediation, pass 2

**Reviewed**: Frozen source102 to source103 remediation in the existing `crates/rdocx-oxml/src/text.rs` carrier,232 additions and53 deletions. Capsule `review-remediation-capsule-103.json` SHA-256 `6bef3887818ab69a515374c70a8cd32201888fea6d943b347b2d2e8b7a169ae2`, patch `d3df4c6e054351b21b93b5b54d2d27d9aa7598a290e4a498bd236265d9041ba3`, actual102 preimage `e55fe8481e96717d82186fc7a5e7515654766b82d7ecab223c9bddabe27774d9`, source map `f42245839ca3f6172ef660d6f59b51f919d52794b30596ed94ce53e068cca228`. All3228 source entries authenticate.

**Verdict**: One defect, zero smells, zero nitpicks. D1 remains incomplete because raw namespace metadata can rebind generated drawing owners. D2 and D3 are statically addressed. Source103 runtime qualification is pending, and no passing gate is inferred.

## Defects

### D1, lifted non-Word metadata can rebind generated drawing owners

`crates/rdocx-oxml/src/text.rs:3663`

The new root preparation localizes foreign w but places every other retained raw-child binding on the generated run. A source run containing `<wp:opaque/>` under `xmlns:wp="urn:producer"`, followed by a legitimate authored Drawing appended to its typed content, now places that foreign wp declaration on the generated w:r. `CT_Drawing::to_xml` at `crates/rdocx-oxml/src/drawing.rs:2248` emits w:drawing without correcting wp. Authored `CT_Inline::to_xml` at `crates/rdocx-oxml/src/drawing.rs:1563` emits wp:inline, wp:extent and wp:docPr without a local namespace declaration. Those generated owners consequently belong to urn:producer instead of WordprocessingDrawing, even when the enclosing part supplies the correct canonical wp binding.

The same root-placement problem affects a foreign r binding used by raw payload. Authored picture output at `crates/rdocx-oxml/src/drawing.rs:2012` emits r:embed without a local canonical relationship declaration. It becomes a foreign attribute and loses its modeled media relationship authority. The foreign-a test at `crates/rdocx-oxml/src/text.rs:14362` does not cover either defect because a:graphic explicitly declares canonical a at `crates/rdocx-oxml/src/drawing.rs:1970`.

Retained raw namespace facts must not change the namespace of modeled descendants. Localize actual incompatible fixed writer bindings on affected raw children, or close the typed descendants under their actual canonical namespace. Add meaningful wp and r collision controls asserting both retained opaque authority and generated drawing element/relationship expanded names. Do not weaken raw preservation or merely check serialization success. This conclusion follows from the concrete emitter paths. No runtime was executed by this review.

## Smells

Zero additional findings.

## Nitpicks

Zero findings.

## Not found

- D1 original redundant canonical-w closure: zero additional findings. The filter at `crates/rdocx-oxml/src/text.rs:3317` excludes canonical generated w from synthetic metadata. Required non-w facts are lifted to the owner rather than inserted into each raw child, retaining the raw bytes. New controls at line14289 check opaque payload and default/QName namespace authority, output/model fixed points and the canonical separator slot. The remaining incompatible generated-prefix problem is D1 above.
- D2 unrelated opaque-value decoding: zero findings in this remediation. `crates/rdocx-oxml/src/text.rs:13660` restricts value decoding to the actual MC prefix-list/QName attributes and xsi:type. Other prefixed attribute names still contribute namespace ownership but their opaque values are not decoded. The original malformed legacy-form control remains unchanged and still needs actual source103 execution.
- D3 repeated metadata scanning: zero findings in this remediation. `crates/rdocx-oxml/src/text.rs:3375` prepares metadata once per run emission and supplies its concrete binding slice to all raw slots. Cached-field admission at line2033 likewise prepares once before the raw-child loop. The512/1024 control at line14331 counts actual preparation visits and independently checks raw occurrence counts. No per-child scan of the full raw-slot vector remains in the new helper. This does not claim that every preexisting mixed-boundary serializer loop is linear.
- Cached-field admission: zero additional findings. `crates/rdocx-oxml/src/text.rs:2063` consumes the same prepared root and localized raw bindings as emission. The foreign-w field lookalike keeps producer authority while the generated w:r remains Word. Existing nonempty root attribute, QName, local-shadow and unused-binding controls remain present.
- Contract, panics and structure: zero additional findings. The source-only remediation preserves public APIs and exposed raw bytes, existing checked owner publication, resource limits, consumed-reader cursor proof, fallback classification and marker identity. It adds no production type, module, file, dependency, trait, generic parameter, cache or quota. The concrete helper replaces the prior repeated extraction rather than creating a second owner representation. No new production unwrap or expect appears.

## Evidence boundary

Reused the approved plan, canonical workflow and pass1 contract audit. Authenticated the actual source102 preimage, exact source103 patch and every frozen current source entry. The previous source102 result remains704 OXML passes and nine failures, with three focused authority controls passing. Source103 compilation, focused controls, affected OXML suite and previously unexecuted Word library/regression qualification are still pending. The new test bodies are reviewed statically, not counted as executed passes.

No source fixes, runtime probes, Cargo, CP, Office, Git, tracked delivery records or full-sprint qualification were performed. Return control to the orchestrator for a distinct remediation phase.
