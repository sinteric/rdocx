# F-X206, localized additional Base controls, pass50

**Reviewed**: External test-only reuse on actual R13 source, two existing carriers,116 added lines. Worker branch `work/f-x206-codex`, HEAD `7b1e5ef29464e93ae30e68df1523ce09143578ff`.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Independence and authentication

This reviewer authored the inherited R1 preparation. This pass independently verifies localized reuse and current-source/API compatibility, without claiming a new independent authorship audit of those unchanged bodies. Independent pass45 and incremental pass46 retain their original scopes and findings. The R2 saved-picture geometry correction remains required for later GREEN.

Receipt SHA256 `335c826d51133a6c85dd8615c33f505c9ae8afefcfeba19753b76d93d405d4e3`. Patch SHA256 `cc46c35856e0c3f777493ef5089b21c0741e73542bb3c89e8c8073c758793d57`. Reconstructed the patch exactly from actual current and candidate bytes. There are additions only, with no changed production bytes. All2714 actual SHA/size/mode guards match R13 map `f1fe07fc52e484c163b1d1a5b9b2d8f37dbe983056b9a1a18b4eaa26d7457895`.

Candidate paragraph SHA256 `7780b79b8cf20efa7f4814a3797f46611eb0b2f24ea684c0a6f8e34858a40d7e`, candidate installed carrier SHA256 `3f0a489bba86e8c93cd659559ef046340159b2857e8127adebc870af049d3ab0`. Prospective map `f60e4c3b25305cc84cf58a12f42b4f80777335d7a2e5110b1d3e033dd50b995e` changes exactly those two records. Every added line matches the original sealed R1 test patches. Original-five input SHA256 remains `3538960e56c3ea4d0fea551e9476e555bfe7e67c15af5222d3d5f540a48c378e`.

## Defects

None in localized reuse.

## Smells

None.

## Nitpicks

None.

## Aspect results

Candidate citations refer to `/private/tmp/S91-F-X206-additional-picture-highlight-Base-prepared/candidate`.

- Correctness: zero findings. Candidate `crates/oxml-drawing/src/text/paragraph.rs:3098` distinguishes ancestor-owned highlight from a local foreign shadow, validates red source colour, edits to cyan, requires exact foreign/extension payload and repeat/reopen stability, and rejects duplicate owned highlights. Existing CT_RegularTextRun parser/writer, ColorChoice and set_highlight APIs support the control without prospective struct fields.
- Contract: zero findings. Candidate `crates/rpptx-py/tests/test_documented_examples.py:7340` has two meaningful crop parameterizations. It first creates the existing template picture owner and checks actual inherited geometry, then reaches the explicit absent-capability assertion at :7356. Existing current effective_geometry API at `crates/rpptx-py/src/shape.rs:650` supports setup. Missing imports, fixture errors or absent owner/geometry do not qualify as semantic RED.
- Panics: zero findings. Native unwraps express mandatory test preconditions. Both candidate bodies leave production error and allocation policies unchanged. Picture malformed-image and filled-owner attempts retain byte-exact atomic refusal checks.
- OOXML: zero findings. Highlight tests preserve exact foreign payload and source entities. Picture checks retain ID/name/order, selected placeholder metadata, relationship identity and save/reopen. These Base-only reused bodies do not replace pass46's additional saved rectangle/schema-order controls, which remain necessary when production is applied.
- Tests: zero localized findings. Both original bodies are exact. The crop oracle uses original image dimensions and inherited rectangle, independently calculates centered cover mapping and proves it differs from stretch. It does not use product crop values for expected pixels. No actual two-case collection or execution is inferred. Current-source wheel/native/module provenance and explicit interpreter/rootdir remain unbound prerequisites, so an old wheel failure or pass cannot earn current Base credit. Rustfmt exit1 is retained as formatting evidence, not compilation.
- Structure: zero findings. Two existing test carriers only, no future production field, new repository file, module, dependency or abstraction.

## Limits and handback

Proposal remains external and unadopted. The expected highlight assertion and absent insert_picture capability are proposed discriminators only. They do not prove completed crop, geometry, replacement, owner atomicity or native PowerPoint acceptance. Original five Base controls, complete issue309/323 and frozen PR acceptance remain required. Only this report was written, with no source, Cargo, environment, cache, baseline, UI or product-import action. Review ends and returns the lease to ROOT.
