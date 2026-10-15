# F-X201 border, text fill and text watermark focused review, pass 6

Reviewer: fonts_resume, independent of this implementation.

**Verdict: 2 defects, 0 smells, 0 nitpicks.** This is a bounded source review of page-edge mutation, qualified solid text-fill projection and semantic style merging, and the authored DrawingML text watermark carried in the same immutable capsule. Other F201 operations, the later image writer, native Office paint and full feature acceptance are excluded.

## Input authentication

Capsule `/private/tmp/S91-F-X201-border-fill-text-watermark-review/receipt.json`, SHA-256 `4140771f63fa15cbe4ba130f1dd572983db10b9c11de6e67a7203b358285966d`. All 18 copied carriers, source map `248826c89a18e9037e9718e19099a3803947a3b6299836e3aeb6b5e2d78389c2`, complete working diff `8674f5b26eb41d4f873c9f5505e2a70e0caf6a0931ca72ae7932319ed1da0d2b`, actual receipt and every raw log and native input hash were authenticated. Copied carriers outside the bounded scope are context only.

Citations refer to immutable files under the capsule's source directory. Native document.rs SHA-256 `c9b57e8de7e01112fc314d5096a28fb9ad63f1388077deea358bee39ebd95236`, run_properties.rs `16746ad8cc885dac881332c608db7aa6806b4ff6f25b3ff18830751023bedfac`, engine.rs `e49d487da959ad4799d6250fcccae2e6536c3b0aabe53be7f3a7ea1e1b5799ce`, regression_test.rs `61f1f215b386d6b8f9b036ad363ed65963704f4da94365108ebc8cc5888ef847`.

Unchanged provider citations were separately bound to the source map. DrawingML drawing.rs `d50d4cb31123a5fe271a307942cb886d03f1f90dfe94f3a363e7a93c271b5285` was read from the exact worker HEAD blob because the live writer had subsequently advanced it. Borders.rs `1dc850570224f5d2b8d6e13ff4df8a73039ab101e5801e421250fcb0f0dca739` and paginator.rs `3f70b77568a488716b05362a470fca27cb5ae9bf2772b61c318620cf9cb1890d` matched the frozen source records. No advancing implementation is substituted for these inputs.

## Defects

### D1. DrawingML text watermark rotation is authored but not painted

`crates/rdocx/src/document.rs:22195`, `crates/rdocx/src/document.rs:930`, `crates/rdocx-layout/src/engine.rs:11000`, `crates/rdocx-layout/src/paginator.rs:2756`.

The new text watermark accepts rotation_degrees, defaults to 315 degrees through TextWatermarkOptions at document.rs:549, and writes that angle into the physical WPS a:xfrm and VML shape. The repository rendering path never carries it. The source-bound CT_Shape projection at rdocx-oxml drawing.rs:326 has preset, solid fill and text but no rotation. collect_anchored_drawings builds AnchoredContent::Shape without a transform, and the paginator emits its text directly into the positioned rectangle. The ordinary Word rendering branch therefore paints the authored diagonal watermark horizontally.

The approved DrawingML policy at `.claude/plans/F-X201-design.md:250` requires checked rotation and actual faithful rendering. Carry the qualified source-owned transform into the actual local text-box paint path, followed by page placement, or refuse an unrepresentable requested transform atomically. Do not infer rotation from the API identity or introduce a watermark-only guessed transform. The current deciding test at regression_test.rs:399684 checks visible text and fill-opacity but never checks the text transform. Add a nonzero-angle versus zero-angle rendered geometry control using the same real text and verify source/save/reopen preservation. A correct native Word XML result alone does not close the repository renderer obligation.

### D2. An explicit page-border color leaves the old theme color in control

`crates/rdocx/src/document.rs:4131`, `crates/rdocx/src/document.rs:4135`, `crates/rdocx-oxml/src/borders.rs:32`, `crates/rdocx-oxml/src/borders.rs:109`.

The new one-edge setter writes the requested literal color but leaves the selected edge's retained themeColor, themeTint and themeShade attributes intact. An imported top edge with a theme color consequently saves both the new literal AABBCC and the old theme selector. Under the common OOXML border color rules the theme color supersedes the literal fallback, so the requested explicit color does not become the actual Word border paint. Retaining foreign attributes is correct, but treating qualified owned color selectors as untouchable opaque data prevents the operation from fulfilling its color operand.

The existing border control at regression_test.rs:399566 has producer attributes and payload but no theme selectors, so its typed color assertion cannot decide this case. On the selected physical edge, remove or replace only the qualified conflicting owned theme-color selectors when authoring a literal color, or return a named atomic refusal if that representation cannot be changed faithfully. Preserve alias-qualified Word ownership, foreign lookalike attributes, all unrelated edges and source children. Add a themed-edge control with exact saved color semantics, foreign namespace negatives and failed-operation package equality.

The common border color precedence is specified in ECMA's WordprocessingML reference, section 2.3.4, color attribute, in the [ECMA reference document](https://citeseerx.ist.psu.edu/document?doi=9fef4fc47d3bbdbb848da75904b50cdb6ac57e15&repid=rep1&type=pdf). No Office paint execution was performed in this review.

## Six aspects

- Correctness: D1 and D2. Zero additional findings in the bounded source paths. Qualified solid fill now survives the actual style merge before resolve_run_color, closing the raw-extension transport risk identified during source research.
- Contract: D1 and D2. Zero additional findings. Real text remains in both WPS and VML representations, with page-relative behind-text placement, no wrap and source-owned header allocation. The new path does not silently replace the legacy VML API.
- Panics and errors: 0 additional findings. Page-border operands validate before any selected-edge mutation. Font size, dimensions, opacity and rotation are checked before staged publication. Header replacement uses a candidate and one prepared reopen.
- OOXML: 0 additional findings beyond the owned color-selector issue in D2. The fill parser verifies the Word2010 namespace on elements and val attributes, supports aliases and rejects foreign elements or invalid alpha. Opaque source remains retained. Semantic merging copies only a recognized fill fact, not arbitrary extension trees. The qualified writer uses textFill/solidFill/srgbClr/alpha and namespaced Word2010 val attributes, consistent with [Microsoft CT_FillTextEffect](https://learn.microsoft.com/en-us/openspecs/office_standards/ms-docx/38a5a1ea-7f9e-48ef-a69f-8df03e8428fc) and [CT_SRgbColor](https://learn.microsoft.com/en-us/openspecs/office_standards/ms-docx/10a451b3-6054-4e0a-b1b2-b68fe9316533). Source-qualified API watermark replacement refuses externally changed payload and preserves unrelated header content.
- Tests: 0 separate findings. The merge control covers inherited and direct admitted fills, alias prefix, foreign namespace, invalid alpha and unchanged original raw bytes. The actual watermark control covers both real run representations, save/reopen, replacement, invalid opacity atomicity and opaque-owner refusal. It does not establish D1 rotation or D2 themed-edge paint, nor native Office parity.
- Structure: 0 findings. The changes use existing model, native, layout and test carriers. The existing Color.a output route is reused by SVG and PDF. No new module, dependency, trait or speculative abstraction is introduced.

## Actual evidence and limits

Authenticated actual receipt `c592b725b95a75326e62d43274a07521fb9cd31f0ad92edc382efd13ba61e7aa` records native/binding all-target check exit0 and three exact one-pass, zero-failure, zero-ignore controls: page-border edges, style-merge alpha, and DrawingML text watermark. All four rows have reaped leaders, groups gone with ESRCH, no signals, and safe lease release. The source map has 3076 records and sourceAfterMatches is true. Earlier failed compile attempts remain failed and are not credited as semantic controls.

The retained page-watermark DOCX, deterministic PDF and SVG hashes were authenticated. They prove the recorded generated inputs and source-built output exist, not native Word/LibreOffice acceptance or source-independent paint parity. ROOT is conducting native acceptance separately. No fresh installed wheel, typing, image-watermark, hash or full-feature completion gate is granted here.

Only this external review file was written. No source changes, formatting, tests, product imports or runtime execution occurred. Return control for distinct remediation.
