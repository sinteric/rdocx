# F-X206, thirteen failure batch, pass 78

**Reviewed**: R2 remediation of pass77 D1 only. External incremental patch158 lines changes one existing carrier, shape_tree.rs. The complete537-line four-carrier proposal retains the other three R1 candidates byte exactly. Receipt73a50d2308b5d013fdd246e909f52dad3631d6a70d770d4bbcc656c6a1f75664 binds incremental e7485072bfa1b8033c3c1c8180b463ca4f9be58a01b4a151e633acc6d71638e8 and complete49e80afd2186e0b760d5a9471d801d720331590f95d87eb6d3e6785cac9782a3.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

The reviewer authored historical namespace remediation slices, but did not author this external correction. This is an independent incremental review of the new R2 changes. Pass77 remains the authority for the unchanged R1 slices, not a new independent whole-feature claim.

All2714 actual source byte, size and mode records match source6280656cec69273a09cef270439a645b77070c062268930ddbb01d5fe9ed31de. HEAD7b1e5ef29464e93ae30e68df1523ce09143578ff and NUL-delimited stage digestc37a36fb85cd0d0055de6dad78d342e20dd2084e45113fc2d1424f7c3338175c match. All four candidate hashes match the receipt and prospective map0f4f64d835f31018e2141d06cf04064c3b08be1811c6197d91614fe5cf59d831. The incremental patch reconstructs the complete shape_tree candidate in memory. No source, runtime or packet was changed.

## Defects

None. Pass77 D1 is resolved in the proposed source.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: candidate `/private/tmp/S91-F-X206-thirteen-failure-batch-r2-prepared/candidate/crates/rpptx-oxml/src/shape_tree.rs:1390` suppresses only the synthetic canonical root relationship declaration when retained attributes already own xmlns:r. Raw attributes are still emitted once at1400. The group writer uses the same condition at2384 before emitting its retained attributes at2389. Unbound ordinary shapes and groups still receive their canonical relationship declaration. Inherited foreign ownership remains closed by unchanged self_contained_attributes in candidate namespace.rs558, including children serialized separately.
- Contract: shape_tree.rs1215 refuses only picture conversion with an explicitly retained foreign root relationship binding. The method takes an immutable shape and returns before allocating or composing the new picture. It preserves representable canonical conversion and the previous nonempty-text and owned-lock refusal rules. The proposal explicitly records this as an unsupported imported-conversion boundary, not full picture-placeholder acceptance. Approved identity, metadata, order, inherited rectangle and atomic owner allocation obligations remain due for supported cases.
- Panics: the production additions are bounded attribute iteration and existing error returns. They add no untrusted indexing, slicing, arithmetic or unwrap. The new test indexing and unwraps are deciding setup/assertions.
- OOXML: shape_tree.rs3354 tests a genuine PresentationML shape with a foreign root relationship binding and an opaque payload containing an original entity spelling. The namespace-resolving helper at3325 checks duplicate attributes and actual expanded payload identity. The group at3363 supplies the foreign binding through its ancestor and exercises embedded and standalone child serialization at3374. Raw subtree emission and child ordering are unchanged. Strict conflicting p/a/mc namespaces remain negative operands at3383. The correction does not broadly admit foreign shape, picture or connector roots.
- Tests: shape_tree.rs3393 requires the precise foreign-root conversion refusal, unchanged serialized source and stable reparse. Its canonical relationship positive at3407 still converts and reparses successfully. The first new control would reject the R1 duplicate declaration through attribute validation, and the second would reject the R1 conversion path rather than treating invalid XML as success. All sixteen original failure selectors and four original additional controls remain byte-equal in the R2 queue. Exactly two controls are added, making six new controls total. No expected XML, native fixture or package SHA is changed by R2.
- Structure: no new tracked file, module, dependency, trait, generic, public model field or forwarding wrapper. The correction stays in the existing owner serializer/conversion carrier. The local test helper has actual namespace and duplicate-attribute assertion consumers.

## Retained barriers and limits

The other three candidates, paragraph.rs, namespace.rs and integration.rs, are byte-identical to R1 and retain pass77's nonfinding conclusions. R2 does not resolve the two original SmartArt/F116 package SHA barriers. Their assertions and retained diagnostic artifacts remain unchanged. The queue keeps their qualificationPending flags and hashGateReady false. No failed runtime is relabeled, and runtimeAdmission remains null.

The new source controls are uncompiled and unrun. Standalone syntax and apply checks are not GREEN evidence. Actual focused selectors, full changed suites, native/installed paint, supported imported picture conversion, archives and final whole-feature acceptance remain outside this incremental review. Return for distinct source adoption and authentic runtime admission.
