# F-X255, correctness, pass 1

**Reviewed**: Frozen 54-file working diff against actual claim base852a47334398552bf63c0b14965d715c2dce6427,318 insertions and184 deletions. Approved sole plan SHA256 c7d7b0d51794823e94dc58b48bbbef93e28bc14f3f959f78a7385bf0b103f56d, progress, canonical workflow/microscope/release contracts and cited HLD10/15 reviewed. This is source/contract review before pending numerical and acceptance work.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Checked without findings

Carriers and graph: Semantically inspected every manifest, lock, README and current literal diff. Root workspace and ten inherited packages resolve to0.16.2. Sixteen changed explicit shared/PowerPoint carriers resolve to0.14.2. The seventeenth explicit manifest is unchanged rpptx-wasm0.12.1. Its publish=false and separate npm boundary remain. All24 internal workspace pins match their target packages. Exactly26 lock records change only their version field, with package count, dependency lists, registry sources, checksums and external records unchanged. Exact seven/fifteen publishable family contracts and existing binding/WASM exclusions remain enforced. Feature/default/dependency graphs do not change.

Runtime and compatibility: All changed Rust lines are existing current-version assertions inside cfg(test) blocks or existing integration assertions. Production prefixes remain byte-identical. CI's Word WASM expected version advances while the separate PPT WASM literal stays0.12.1. The release introduces no product API, module, framework, feature flag or runtime behavior.

Current versus historical tests: New target-specific S95 tests independently require exact target families, versions, lock identities, binding carriers, family-only contribution URL sets, note structure, compatibility disclosures and the separate WASM carrier. Existing current source assertions advance to the new carriers. S91 historical note and contributor inventories remain intact, with current graph checks separated from immutable note checks using the established historical pattern. Existing mismatch negatives for binding projects, publishable membership and lock records still operate on current values and retain their refusal assertions. The real old-source target regression log fails on0.16.1 versus0.16.2, rather than setup. Retained focused successor logs record three notes/historical tests and two target/historical tests passing. These are focused worker observations only, not a complete policy/scoped pass.

Contribution provenance: Root's retained contribution authority authenticates all120 listed files and the published baseline80117123c1073b826392b06a6e6601311a929b44. The finite inventory contains23 outcomes. New Word and shared/PPT notes assign each applicable outcome and exact linked record to the proper family, including tooling PR376 and only the newly delivered PowerPoint HTML increment from PR359. Andrewkoro105 and Hadrien Mary are credited through authenticated profiles and linked reports. Closed-but-unmerged PR374/375/376 are described as selected donor slices or adapted/hardened work, without claiming unchanged merges. The prior40-issue/30-PR shipped inventory is not recounted as new.

Notes and boundaries: Both exact release sections have the five required subsections. Word compatibility explicitly names RenderOptions.revision_style, LayoutInput.revision_style, ParagraphBlock bool-to-Option<Color> change_bar and NoteLayout color-valued revision ranges. Plain remains default and stored formatting unchanged. ZIP9 OpcError::Zip payload compatibility and named Python refusals are disclosed without pretending the patch line guarantees Rust source compatibility. Prior intentional rendering deltas remain attributed, and F-X255 adds no paint delta. Formula/border/native evidence limitations are explicit. Partial Issue385, Issue386 and the future Issue381/fillable-PDF, bibliography/navigation, watermark, multi-face and Python3.14 work are excluded. Family publication order and separate final approval remain explicit. HLD10/15 and CLAUDE current prose distinguish prepared0.16.2/0.14.2 from the actual published0.16.1/0.14.1 baseline.

Numerical ownership: Static README script comparison changes only README_REQUIRED_TEXT and PACKAGE_VERSIONS assignments. Archive values, dates, compression tolerance, measurement normalizer/validator and historical performance constants are unchanged. README changes are installation/API examples. Root-owned measured rows and archive catalog reconciliation remain pending, as explicitly declared, and are not claimed accurate for final artifacts by this pass.

## Authentication and boundary

Frozen manifest SHA256 a257fd04b4f4813f937afd9f27694e8dd8e0fe7ff3a201d79d545fcfa1c0d324 authenticates all54 files, before and after the audit. Its paired binary diff exactly equals the reviewed working diff. No source outside the declared frozen set was changed by the reviewer.

Only this report was written. No product execution, test rerun, build, cache, UI, source, HLD, test, plan, progress or private proof mutation was performed. Worker external scoped checks continue against frozen source. Actual22 archive measurement and genuine publication verification, full policy, root numerical reconciliation, fresh local42/44 priority suites on3.9/3.12, typing/stub checks, source availability checkpoint, actual hosted27-job acceptance and downloaded artifact qualification remain pending. This review does not claim those checks passed or authorize a worker push, main merge, tag, registry publication or final sprint acceptance. Return the sole worktree lease and stop this pass. No additional confirmation pass is required for this clean source state.
