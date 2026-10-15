# F-X206, thirteen failure batch, pass 77

**Reviewed**: External four-carrier381-line patch806550eb824542831b259206bfc958dc38300be78d0d685e458d887795c8003d against current2714 source6280656cec69273a09cef270439a645b77070c062268930ddbb01d5fe9ed31de. Receipt fcd8717419f529e994c5b36069eb4e735a85ba5bbb1dcfe79e8e71d9604d6b1c binds prospective source a03dc8651a59f688ce1ca152b1a3337b19681bc61f86a15f081c4627c32f32f2.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

The reviewer authored earlier namespace remediation slices. This pass independently audits the new external batch, not the historical implementation as a whole. Approved F206 source preservation, qualified namespace ownership, inherited geometry and atomic connector refusal remain the contract. All2714 current byte, size and mode records match. Exactly four prospective carriers change with modes preserved. HEAD7b1e5ef29464e93ae30e68df1523ce09143578ff and stage digestc37a36fb85cd0d0055de6dad78d342e20dd2084e45113fc2d1424f7c3338175c match the receipt. No runtime was executed.

## Defects

### D1, retained foreign relationship binding duplicates self-contained root declarations

`/private/tmp/S91-F-X206-thirteen-failure-batch-r1-prepared/candidate/crates/rpptx-oxml/src/namespace.rs:548`

`/private/tmp/S91-F-X206-thirteen-failure-batch-r1-prepared/candidate/crates/rpptx-oxml/src/shape_tree.rs:1377`

The candidate now accepts a genuine PresentationML shape with a foreign physical `xmlns:r`, through the relaxed conflict check at shape_tree.rs1047, and retains that declaration in raw root attributes. `CT_Shape::to_xml` still unconditionally emits canonical `xmlns:r` before appending those attributes at1380. A valid standalone shape with `xmlns:p` bound to PresentationML, `xmlns:r="urn:foreign"`, required non-visual and shape-property children and an opaque `r:payload` therefore serializes with two `xmlns:r` attributes. The resulting XML is invalid and cannot preserve the original opaque prefix binding on reparse.

The same trigger reaches the standalone group writer at shape_tree.rs2364 and2367. Picture-placeholder conversion adds canonical `xmlns:r` at1324 and then appends the retained shape-root attributes at1329. The conversion must either preserve representable opaque ownership with a separate genuine relationship closure or refuse atomically, rather than emit duplicate declarations. Picture and connector parsers retain their original strict conflict policy, so this finding does not claim that those unrelated parsers newly admit foreign roots.

The new test at namespace.rs801 only inspects attribute vectors and conflict checks. It never serializes a newly admitted shape or group. Add actual standalone, inherited/group and conversion save/reparse controls retaining foreign relationship payload. The existing first-click package controls must also remain deciding gates. Choosing a genuine relationship prefix for a new click does not repair duplicate declarations on these self-contained outer writers.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: no additional defect in sorted paragraph raw-attribute multiset equality. Modeled fields and exact ordered raw children remain in comparison. The parser and source XML, physical namespace provenance, source attributes and writer are unchanged. The new control at candidate paragraph.rs5699 checks both original source encodings write exactly and rejects changed values and opaque bodies.
- Contract: the connector success fixtures explicitly author both attachment targets' transforms at candidate integration.rs28312. They retain original IDs, preset geometry and endpoint site references. The original missing-bounds fixture remains unchanged and its new refusal control at28324 requires InvalidShapeMutation plus whole-package and reopen equality. This preserves the plan's unsupported late-reroute refusal instead of inferring imported bounds. The placeholder correction at10077 verifies actual inherited extents, layout/master byte preservation and reopened effective geometry.
- Panics: no new untrusted production indexing or arithmetic finding. New unwraps are test setup and deciding assertions. The namespace production change adds only filtering and conflict-policy operations.
- OOXML: apart from D1, no finding in the expanded-name assertion helper at integration.rs28341. Element and attribute identities use actual namespace resolution. Nested scope boundaries preserve the matching property owner. Foreign or rebound lookalikes and wrong relationship IDs remain negative controls. Exact style child order and opaque marker/field bytes are retained.
- Tests: no additional weakening found in the eleven proposed corrections. Original three click setup repairs are already current-source inputs and remain retained. Four new controls are source only. The focused queue retains all16 original failure identities plus four new selectors and the original four unrun native rows. No compilation or GREEN claim follows from syntax checks.
- Structure: no new tracked module, file, trait, generic, dependency or production forwarding wrapper. The test-only XML fact structure has actual lexical assertion consumers.

## Retained barriers and scope limits

Candidate integration.rs10922 retains the SmartArt bytes before the unchanged original SHA assertion. At13214 the F116 candidate path is retained until the unchanged original assertion succeeds. Neither expected hash changes, and neither failure is reclassified as a pass. Actual generated ZIP capture, historical source-bound attribution and independent review remain required before any proposed expectation change. Other native, installed, paint, archive and whole-feature acceptance are outside this incremental source review. Stop this pass and return for distinct remediation of D1.
