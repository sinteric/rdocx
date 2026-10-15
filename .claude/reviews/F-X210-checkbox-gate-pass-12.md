# F-X210, joint checkbox gate, pass 12

**Reviewed**: External R2 packet at `/private/tmp/S91-F-X200-F210-checkbox-gate-r2-prepared`. Four executable candidates and proposal total197 lines. Input and source guard inventories were read in full. This is a source and oracle-design review, not execution or whole-feature acceptance.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authority and independence

Receipt SHA256 is `054c34bf2d11d40270b682ba69b079c430e32b94146de9e6341b9698980affc4`. All seven sealed artifact hashes and sizes match, as do all four input guards and nine source guards. Native candidate SHA256 is `a61275690a8d4b668d01e96176e623bead68fdb048fc12380904fa33933b29dd`. Source-authority candidate SHA256 is `1179822cadfd57b455aef581076ce5239a629c51d5a869ce46a0d263a417ef50`.

The approved F-X200 checklist and F-X210 deterministic coverage contracts, canonical workflow and microscope command govern this review. Current sprint and backlog mark both stories in progress. Historical F-X200 implementation and runtime preparation authored by this reviewer are disclosed. This pass independently reviews the new externally authored joint gate, not those historical implementation changes.

The actual F200 XML control remains unchanged at `crates/rdocx-py/tests/test_formatting_tables.py:1705`. It saves real public checklist definitions and concrete numbering references, checks both original Unicode characters and reopens the document. R2 generates additional isolated paint documents rather than modifying that existing control or manually rewriting its numbering XML.

## Checked behavior

At external candidate `generate.py:14`, both definitions use the real public `ListLevel.checklist`. Lines27 through29 require the exact U2610 or U2611, bullet format and absent marker rPr. Lines30 through35 authenticate saved style48pt, direct whitespace Carlito11pt and concrete numbering ownership. Reopen and repeated source serialization remain mandatory at lines18 through21. These explicit paint operands isolate marker ink while retaining the unamended preset.

At external candidate `native.rs:13`, Arial is only the caller selection alias. Line32 requires selected physical bytes to equal the approved3968-byte Noto subset. Face index, coordinates, synthesis, nonzero glyph and actual one-marker owner checks are explicit at lines25 through32. The selected deterministic facade avoids host discovery. PDF and PNG derive from one owned layout, repeat exactly, and source bytes remain unchanged at lines14 through20 and35.

The source placement derivation is independent of product observations. External `source_authority.py:13` checks full-source and subset metric equality. Lines16 through21 apply the relevant original table ascender and require48pt marker ascent to exceed both generic and Word body regimes. Lines27 through43 validate saved paragraph/style/numbering/section operands and derive margin-plus-first-line-indent and margin-plus-ascent coordinates. This agrees with actual marker shaping at `crates/rdocx-layout/src/engine.rs:8808`, generic font metrics at `crates/oxml-layout/src/font.rs:1999`, maximum item ascent at `crates/oxml-layout/src/line.rs:630`, first-line indent at line1888 and paginator placement at `crates/rdocx-layout/src/paginator.rs:4253`. Actual default list indentation is in `crates/rdocx/src/document.rs:31027`. No observed glyph position or raster score feeds the reference derivation.

External `checker.py:9` pins original source font bytes. Lines13 through24 construct original48pt contours and a fixed source-derived ROI. Lines33 through40 retain.99 positive acceptance, wrong-character, empty and6pt displacement negatives, with no registration, fitted origin or threshold relaxation. Original source authority requires separate ROOT admission at line47. Reachable PDF text, distinct CID/GID handling and unused-resource negatives remain the already-authenticated verifier requirements in `PROPOSAL.md:7`, not a matching resource-name shortcut.

## Explicit acceptance boundary

The no-caller pair is not qualified by the prepared caller reference. External `source_authority.py:11` selects original full Noto for the caller authority and line44 labels that scope explicitly. `native.rs:12` also produces two no-caller outputs, but `PROPOSAL.md:9` requires separately authenticating their actual selected physical face against the unchanged registered inventory, deriving its reference placement from original source metrics, proving reachable U2610 or U2611 and admitting its contour positive and negatives. Those two visibility cases remain a mandatory unresolved execution qualification. They cannot inherit the caller reference, a future score, or a selected font inferred from exported PDF bytes.

Fresh actual F210 artifacts, accepted F200 wheels and full188, actual bounded compiler invocation, qualified resvg and Poppler paths, PDF text-paint authentication, both product raster comparisons and all four case outcomes remain unexecuted. Receipt fields remain null or false. This packet does not replace other F210 routes, genuine-font precedence, scoped riders or whole-feature acceptance.

## Defects

0 findings.

## Smells

0 findings.

## Nitpicks

0 findings.

## Not found

- **Correctness**: 0 findings in the bounded source-derived caller reference and actual-owner harness.
- **Contract**: 0 findings. Original public presets and mandatory separate no-caller coverage remain intact.
- **Panics**: 0 findings. Strict fixture and artifact preconditions fail closed.
- **OOXML**: 0 findings. The generator uses public saved definitions, preserves original markers and checks actual numbering ownership.
- **Tests**: 0 findings. Positive and negative requirements are causal, with no product-result fitting or unperformed acceptance credit.
- **Structure**: 0 findings. No product file, registry, trait, module or dependency is introduced by this external packet.

Review ended without source, environment, target, compiler, product import, rendering or oracle execution.
