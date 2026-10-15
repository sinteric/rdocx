# F-X210, Extend metric-compatible fonts and bullet fallbacks

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: F-X195

## Problem

Issue313 repeats the caller-font and variable-instance work already owned by F-X195 for Issues296 and297. This story owns only the new coverage delta. Current mappings at crates/oxml-layout/src/font.rs:2117 substitute Caladea followed by Liberation Serif for Georgia and Carlito for Segoe UI. Aptos has no explicit mapping or substitution message. The PowerPoint bullet mapper at crates/rpptx-render/src/text.rs:317 handles only F0B7 to U+2022. Word Symbol runs at crates/rdocx-layout/src/engine.rs:9647 still shape the original private-use character. Broader private-use mappings need font identity, not a universal codepoint substitution.

## Spec reference

- docs/hld/14-development-backlog.md, F-X210 and the existing F-X195 ownership.
- docs/hld/08-rendering-spec.md, Text in a shape, The PDF backend, The rasteriser and The SVG backend.
- docs/hld/10-bindings-spec.md, Python API shape, Native PowerPoint text layout and Native Word facade stability.
- docs/hld/15-build-and-toolchain.md, Deterministic rendering, Packaging and Dependency policy.
- docs/hld/12-testing-strategy.md, Test taxonomy, The mixed-script geometry golden gate and Binding tests.
- docs/hld/04-opc-and-packaging.md, The package, and docs/hld/06-presentationml-model.md, Preservation strategy.

## Approach

Wait for the reviewed F-X195 font-selection contract and reuse its owned instance identity, caller precedence, directory validation and renderer propagation. Do not implement a second resolver or repeat Issues296/297 acceptance under a second owner. Existing directory and explicit byte inputs remain supported.

Pin two genuine Gelasio variable faces, upright and italic, and suitable genuine Selawik regular and bold faces from their official upstream distributions. The inspected eight-file proposal and complete immutable hashes are recorded in /private/tmp/S91-F-X210-font-intake/eight-file-proposal.json. The total proposed tracked size is440844 bytes. Gelasio is pinned at commit9228e69b160d79f33950e026293f6e13ba9780d0. Selawik official release1.01 is pinned at tag commitb65ff9ce7c4673e75ec5c622924a6b6f61f2f876 and release archive SHA2563f62c51e05e3b5a1e6241cf92a371f0be2ea1183aa87b30718bbd40832a8d423. Record source format, face index, axes and available styles before copying any asset. Proposed core inventory is Gelasio-Regular.ttf and Gelasio-Italic.ttf, Selawik-Regular.ttf and Selawik-Bold.ttf, LICENSE-Gelasio, NOTICE-Gelasio, LICENSE-Selawik and NOTICE-Selawik under crates/oxml-layout/fonts. The user explicitly approved these exact eight files on2026-10-09. No further file approval is required for this inventory. Creation still waits for the F-X195 completion barrier and approved implementation design. Preserve original embedded family/style names and original license text. Do not imply that Selawik has a genuine italic face if the selected upstream distribution does not supply one. F-X195's verified synthesis policy handles unsupported styles.

Google Fonts metadata currently identifies Gelasio[wght].ttf and Gelasio-Italic[wght].ttf, range400 to700, sourced from SorkinType/Gelasio commit9228e69b160d79f33950e026293f6e13ba9780d0. Recheck the immutable binary source and license during implementation rather than rely on a mutable main URL. Upstream describes regular, bold, italic and bold-italic Georgia compatibility, while documenting repertoire and kerning limitations. Primary references are [Google Fonts metadata](https://github.com/google/fonts/blob/main/ofl/gelasio/METADATA.pb) and [Gelasio upstream](https://github.com/sorkintype/Gelasio). This is upstream intent, not our measured acceptance evidence.

Microsoft describes Selawik as a Segoe UI replacement and explicitly lists missing matching kerning and hinting in the [Selawik README](https://github.com/microsoft/Selawik). [Upstream issue 6](https://github.com/microsoft/Selawik/issues/6) also discusses unresolved letterform and metric differences. Therefore state measured compatibility for a pinned reference version and tested repertoire, not universal Segoe UI equivalence. Character advances, line metrics and shaped strings are separate measurements. A broad tolerance cannot hide pair-kerning or layout discrepancies. Record real divergences and a reviewed bounded acceptance rule before declaring completion. If available evidence cannot meet the approved criterion, retain a noncompleted barrier instead of calling the face exact.

Update the existing bundled inventory and map Georgia to Gelasio and Segoe UI to Selawik. Caller-supplied authentic faces still win. Keep every existing unrelated family, default CJK coordinate and coverage fallback unchanged. Aptos remains a documented approximate fallback, initially the existing Carlito generic choice. Emit a diagnostic that names Aptos, the selected actual face and the approximation whenever no caller or embedded Aptos face was selected. A supplied Aptos face must suppress that warning. Do not claim that Carlito matches Aptos metrics.

Use existing output Diagnostic.message at crates/oxml-layout/src/output.rs:402 and existing owned layout diagnostics. Record substitution facts within the manager's current layout lifecycle, deduplicated in stable first-use order. The selected output bundle owns the result. Do not add a global diagnostic registry. Word native owned layout and PowerPoint render_deterministic at crates/rpptx/src/lib.rs:1108 already expose LayoutResult. Preserve existing byte-only PDF and PNG return types. Coordinate any missing installed Python diagnostic projection with the existing binding/export owners rather than silently invent a new return shape. F-X207 consumes the same substitution facts in fit/refresh reports.

For common Symbol and Wingdings bullet characters, prefer a supplied genuine font that covers the original codepoint. Otherwise apply an explicit font-family-qualified PUA mapping to supported Unicode and shape through an existing bundled face with verified coverage. Put the common mapping in an existing neutral font source file and call it from existing Word and PowerPoint lowering. Preserve source XML and stored char codes. Unknown PUA remains unchanged with an honest missing-glyph diagnostic. Do not remap private-use content from unrelated fonts, arbitrary regular text, or Wingdings2/3 without their own verified table. Include bullet, small square, arrow and check-mark controls only after the exact original glyph is independently verified. The issue's ambiguous section-square description needs an exact family/codepoint before inclusion. The existing F-099 F0B7 control stays green.

Optional Google font pack decision: the refreshed Issue313 explicitly sets a font pack aside and directs callers to the existing caller-font path for ordinary licensed local directories now. Recommend a separate explicitly designed, hash-pinned archive distribution if a curated optional pack is approved. Do not add a new crate, network downloader, automatic download, runtime fetch, default font pack or dependency here. The current issue asks for a decision, not speculative package machinery. A requested distributed pack needs an exact named consumer, selected family/style inventory, license manifest, archive limits, transport/offline behavior and release ownership before implementation. Documentation must state which decision was made and which behavior actually exists.

Issue313 explicitly requests font_dirs= on the listed render and measurement methods. Amend F-X195's API/input-order contract once and add ordered plural support under that owner. Preserve font_dir compatibility and define directory-to-directory precedence and explicit-byte override before writing bindings. Do not create a duplicate loader under F-X210.

## Rejected alternatives

- Claim Selawik metric equality from upstream intent alone.
- Add a speculative font-pack crate or automatic network loader.
- Rewrite arbitrary private-use text without a verified family-specific mapping.

## Test plan

**Test gate**: regression.

- Add existing-entrypoint red tests proving Georgia and Segoe UI select current inaccurate substitutes, Aptos emits no requested approximation diagnostic, and additional verified PUA bullets remain missing before the patch.
- Compare Gelasio four requested styles with hash-pinned licensed Georgia references. Four Georgia files are available in this host's System/Library/Fonts/Supplemental directory, but references remain ignored local oracle inputs. Pin font versions, bytes, UPEM, HarfBuzz14.4.0 and exact shaping options. Do not redistribute Microsoft font files.
- Obtain a valid pinned licensed Segoe UI reference or authoritative numeric metric evidence before approving Selawik claims. A genuine private Segoe UI5.31 Regular WOFF was located in the installed PowerPoint OfflineFiles resources. It remains in place and is not redistributed. Its SHA256 is d79ad533adf61e76cd74ab32d3d2f53ae11f50360f2f7c95613e4d23787502a0. Selawik1.01 differs in hhea metrics while OS/2 typo and Windows metrics match this reference. Actual shaping and metric-selection controls remain required. Test raw advances, kerning-sensitive AV/To pairs, ligatures, punctuation, numerals, ascender/descender/line pitch and long wrapping strings. Record actual tolerances separately for each measurement.
- Verify PDF selected font identity, actual advances and PNG/SVG visible output in both document families, including caller override, genuine italic, variable bold and permitted synthesis.
- Verify Aptos warning presence, actual selected family, stable deduplication, warm/cold equality, caller suppression and fallback propagation into text-layout/fit reports. Diagnostic-only changes must not change rendered bytes by themselves.
- Test verified Symbol/Wingdings mappings in Word bullets, explicit Word symbol runs and PowerPoint bullets. Assert a nonzero Unicode glyph, visible rendering, actual caller-font preference, unknown mappings and unrelated PUA preservation, and unchanged original OOXML after layout and save/reopen.
- Reuse F-X195 acceptance for caller inputs and variable geometry. The existing six licensed ignored weight controls and receipt are at /private/tmp/S91-F-X195-variable-oracles. Do not copy them as tracked binary fixtures.
- Run scoped affected-crate checks, full changed-crate tests, exact installed binding runtime and typing coverage if projected diagnostics change, no-default-features, WASM, verified patched packaging, package source/license inventory, actual locally generated archive rows and deterministic hash comparison. Integrated full verification remains the sprint gate.

## HLD impact

- docs/hld/08-rendering-spec.md
- docs/hld/10-bindings-spec.md
- docs/hld/14-development-backlog.md
- docs/hld/15-build-and-toolchain.md

Replace current mapping, diagnostic and packaged-font inventory prose with actual reviewed behavior. Do not present upstream intent as a verified universal metric claim.

## Risk routing

- Layout/shaping and bundled fonts require deterministic controls, exact license/provenance records, archive source membership and unchanged existing font hashes. Read HLD08 and HLD15.
- Parser/serializer exposure through symbol handling requires unchanged original source and byte-preserved unmodelled XML, alias/rebound controls and schema ordering. Read HLD04 and HLD06.
- Published shared API changes require explicit semver impact, verified affected-package dry runs with the full local patch graph and archive bounds under10MiB. Ordinary diagnostic output is a behavior change. An added public metadata field needs its pre-1.0 source impact stated.
- Bindings/WASM require both wasm target checks and installed runtime, strict typing and stubtest when their public diagnostics are changed. Do not link PyO3 extension test binaries.
- External font/native oracles are pinned and remain test infrastructure. A metric divergence is triaged, documented and asserted. Never use production output as the expected answer.
- New asset files need explicit approval. No new trait, generic, module, crate, feature flag, dependency or binary test fixture is proposed.

## Hash harness

Existing cases that request Georgia, Segoe UI, Aptos or verified symbol fallbacks can intentionally change. Enumerate actual deterministic entries, measured glyph/advance and diagnostic changes, review them independently and move only the designated exclusive baseline. Unrelated Latin and CJK entries must remain unchanged. Do not guess all49 entries stay unchanged before measurement.

## Implementation checklist

- [x] Settle plural font_dirs scope and optional-pack decision with the canonical owner.
- [x] Approve the exact asset file list, immutable sources, licenses and availability of metric references.
- [x] Prove focused public failures on claimed Base.
- [x] Reuse F-X195 selection and implement approved bundled mappings and lifecycle diagnostics.
- [x] Verify font-specific PUA controls without source mutation.
- [x] Record measured Gelasio/Selawik compatibility and actual limitations.
- [x] Run scoped, oracle, packaging, license, binding and WASM riders.
- [x] Review actual harness changes and update declared HLD sections.

## Open questions

Ordered font_dirs, eight core assets, caller-only optional-pack route,
section-square identity, bounded Selawik metrics and same-pass diagnostic
consumers are resolved below. The user approved the exact3968-byte revised
subset on2026-10-10, answering "Approve the 3,968-byte revision". It adds
onlyU2610 to the same authorized ninth asset path. This separate approval
supersedes the3916-byte selection for implementation. All27 existing bundled
faces lack a nonzero Unicode U2610 glyph in their eligible format4/12 cmaps,
as authenticated in existing-bundled-checkbox-coverage.json. No unresolved
material question remains. No other new file, family/axis modification or
pack machinery is proposed.

## Performed bounded shaping experiment

Pinned HarfBuzz14.4.0 shaped five exact Latin controls in each of four
Gelasio styles at wght400 or700, compared with the installed licensed Georgia
faces read in place. Controls cover kerning pairs, ligatures, numerals,
punctuation and longer strings, with explicit LTR/Latn/en, font-size=upem
and kern0/kern1. All40 pairs have genuine glyph coverage. Maximum absolute
string-advance deltas are0.28857421875em regular,0.48486328125em bold,
0.32470703125em italic and0.38427734375em bold-italic across these controls.
These are observed differences, not acceptable universal tolerances or a
claim that every line wraps identically. Compare the existing substitute,
line metrics and actual layout selection before final compatibility claims.

The exact commands, font versions, hashes, glyph results and normalized
advances are retained in S91-F-X210-font-intake/independent-shaping-controls.json,
SHA256ecb815b27f26ecb27b6a1690ce0d595efdd4bf5dff0895e97f82d4d154a11a6e.
HarfBuzz failed to load the licensed Segoe WOFF reference directly in all10
controls. Those are explicit failed controls, with no numerical shaping
claim and no font conversion or redistribution. Selawik shaping parity
remains unproved by this experiment. No production or baseline changed.

The same40 controls compared the secondary Liberation Serif fallback with
the same licensed Georgia references. Gelasio is closer in absolute string
advance on39 controls. The regular kern0 ligature string is slightly farther,
with delta-0.28857421875em versus-0.26953125em. Existing substitute maximum
absolute deltas are3.22607421875em regular,6.80029296875em bold,
3.9931640625em italic and9.2265625em bold-italic. These bounded results
support testing the proposed replacement, without establishing universal
metric equality, line pitch or identical wrapping. Exact commands and source
hashes are retained in current-substitute-comparison.json,
SHA25684f15c46fb5bac01dc86f9d7479050573b32c1b412a1469e4a267df929e4c71c.

## PR319 reconciliation

PR319 at immutable head9a36bc966d1e12e4e9e2f38b777889bd687c5dd9 closes
Issue313 and belongs to this existing story. It proposes four modified
static Gelasio faces from a different upstream commit and a new Noto Sans
Symbols2 subset, which differ from the exact eight assets already approved.
Do not adopt those assets under the existing approval. Assess complete
immutable sources and recipes against the independent shaping controls,
metric-selection policy, genuine font coverage and bounded acceptance here.
Its narrower printable-ASCII advance comparison does not establish shaped
string or universal geometry equivalence. Reconcile its italic fallback
walk with F-X195's verified synthesis and caller precedence. Assess all
claimed twelve rendering deltas before any labelled behavioral commit or
exclusive local baseline update. Caller and variable font work remains
F-X195, with no duplicated loader or resolver under this contribution.

## Correct primary fallback comparison

A direct resolver read at font.rs:2130 confirms Georgia selects Caladea
before Liberation Serif. The earlier Liberation Serif comparison remains
valid secondary-face evidence and does not represent the primary fallback.
The same40 genuine covered HarfBuzz controls now compare actual bundled
Caladea faces with the same Georgia references and approved Gelasio inputs.
Gelasio has a smaller absolute shaped-string advance difference on all40.
Caladea maximum differences are1.233953125em regular,4.59014453125em bold,
3.087125em italic and6.170203125em bold-italic. These are bounded observed
controls, with no universal wrapping or vertical-metric guarantee.
Complete commands, glyphs, face hashes and results are retained in
/private/tmp/S91-F-X210-font-intake/primary-caladea-comparison.json,
SHA2566211cf634c66c5ab31e2a45799a20a6da6e94cf3973f5a4285e1d79d3cef2f62.
No production, asset or baseline changed.

## Immutable contribution assessment decisions

Completed immutable PR319 assessment is retained in .claude/scratch/S91-PR319-9a36bc9-assessment.md. Selawik source blobs match the exact approved release assets. Its unmatched_requests map keyed only by FontId at font.rs:2221 loses request identity during cached trace replay at font.rs:899, causing a prior Aptos request to generate spurious diagnostics on later Calibri/Carlito use of the same face. AptosDisplay can also lose identity. Use the existing bounded lifecycle trace with actual requested and drawn families per event, not a FontId-global warning. Require actual non-Aptos cached replay and alternate Aptos-name regressions. Reconcile its italic walk with F-X195 controlled synthesis before reuse. No different asset or contributed baseline was adopted.

## Independent Selawik regular metric control

All95 printable-ASCII cmap to hmtx normalized advances match exactly between
approved Selawik Regular1.01 and the licensed installed Segoe UI Regular5.31
WOFF read in place. Maximum advance difference is0em. Both faces have
fsSelection64 without USE_TYPO_METRICS. Current Rust WordLineUnits selects
identical Windows ascent2210/descent514, with zero leading and line height
1.330078125em. Their differing hhea values do not change this current Word
line-height selector, while generic metrics remain a distinct comparison.
Exact source/version/hash, table selector and numeric results are retained in
/private/tmp/S91-F-X210-font-intake/selawik-segoe-ascii-control.json,
SHA256c42e62142caf81adf7eeccc72e3467c59e958af2e6ab0cb7b61c04de576be71b.
WOFF tables were read in memory without font copying, conversion or table
export. No kerning, wrapping or genuine Bold equivalence is established.
The earlier HarfBuzz WOFF load failures remain failed shaping controls.

## Additional bullet coverage proposal

The exact existing eight-file approval remains valid. A separate proposed
3916-byte NotoSansSymbols2-bullets-subset.ttf supplies16 Unicode bullet
characters absent from current bundled coverage. Its official source was
independently retrieved by complete Git blob at google/fonts commit
51303ca9e8ac9dcea7b12d307ba568fd0e6fcfca, source SHA256
7d5fb73b7ca67a6798101741f5d280a3d016a56a197afcd4199dbb57b4b82a21.
The original OFL1.1 and copyright name the Noto Symbols project.
Root reproduced the contributed subset byte for byte using FontTools4.66.1,
output SHA2567f5a31d932df09902c1097af020569b63d6296c067bc804ae6a597ef9b3ad21b.
All16 glyphs are covered in the independent HarfBuzz check. Source, license,
reproduction output and one-file proposal are retained under
/private/tmp/S91-F-X210-symbol-proposal. The completed temporary tool
environment was removed after reproduction to save disk space.

The user approved this one exact new tracked binary on2026-10-09, answering
"Approve this one additional asset". Approval is limited to3916 bytes and
SHA2567f5a31d932df09902c1097af020569b63d6296c067bc804ae6a597ef9b3ad21b.
It is not yet copied into the repository. Its copyright, pinned recipe and
hashes will use the existing NOTICE-Noto, with no new documentation file.
This proposal does not authorize the different four static Gelasio faces,
new font installation or unverified original Symbol/Wingdings mappings.

## Authenticated native symbol mapping

## Eight measured code rows

Each row displays low8-bit w:sym, F000+code w:sym, actual F000+code numbering bullet and a separate Unicode reference. Low/PUA/list original glyphs match visually within every row.

| Original font/code | Native visible glyph | Contributor Unicode target | Measured conclusion |
| --- | --- | --- | --- |
| Wingdings A7/F0A7 | Small filled square | U25AA | Semantic shape matches, reference has different size |
| Wingdings D8/F0D8 | Right-pointing arrowhead with asymmetric filled lower half | U27A2 | Matching semantic arrowhead, reference geometry differs |
| Wingdings FC/F0FC | Curved check with thick heel | U2713 | Matching semantic check, reference stroke differs |
| Wingdings76/F076 | Four filled diamonds | U2756 | Matching semantic diamond cluster |
| Wingdings71/F071 | Shadowed outlined square | U2751 | Matching semantic shadowed square |
| Wingdings6E/F06E | Larger filled square | U25A0 | Matching semantic filled square |
| WingdingsFE/F0FE | Square box with check | U2611 | Target is semantic box-check, but generic native Arial reference falls back to colored emoji |
| SymbolB7/F0B7 | Filled circular bullet | U2022 | Matching semantic circular bullet, reference size differs |

Complete immutable contributor mapping is at S91-PR319 source crates/oxml-layout/src/font.rs:2253 through2291. This is bounded independent appearance confirmation for these eight rows, not a complete Wingdings map oracle. Wingdings2/3 are distinct families and were not loaded, remapped or inferred. Unknown codes remain outside this evidence.

An ordinary Wingdings w:t run contains unchanged source characters §Øüv (A7/D8/FC/76). Native glyphs are the same square/arrow/check/four-diamond symbols. Its source text is not rewritten to Unicode. The fallback contract must distinguish preserving original OOXML/text facts from interpreting an explicitly requested symbol font for display. Do not conclude ordinary Wingdings renders Latin §Øüv merely because native AX or PDF text extraction reports those characters. AX also reports bogus 迨 or bullets for visible checks/diamonds. Native printing PDF pixels are authoritative.

## Font provenance and fallback boundary

Original fonts were read in place only. Correct system Symbol path is /System/Library/Fonts/Symbol.ttf. System Wingdings5.00x at Supplemental/Wingdings.ttf SHA709d5848799239f0c51c0544c471c8d02bcbf9f2bcdc49702ebca1691a4c760e/fsType8. System Symbol13.0d2e2 SHA fbd16f271abb9bb55708a15c138e3a45924dec215541e95595f7a6918304d867/fsType4. Word also ships Wingdings5.03 SHA472858c86b3af58a8d46d96ac423f861717c8164a8104432eb92278bbf921a73 and Symbol5.00i SHA08253a10a93a80f978d88087000e6a6f4952aeffc2e972e81f6f84bbfa679fe4, both fsType8. Candidate metadata is exact, but this study has not traced which same-family installed original face version Word selected. Native displayed glyph morphology remains genuine original-family appearance.

Unicode reference uses native Arial with normal fallback. It is not a NotoSymbols visual acceptance result. In particular, U2611 emoji demonstrates why a monochrome dedicated symbols face is needed. The proposed NotoSansSymbols2-bullets-subset.ttf is3916bytes, SHA7f5a31d932df09902c1097af020569b63d6296c067bc804ae6a597ef9b3ad21b, with supplied receipt stating16 contributed targets have no missing glyphs. The user approved this exact tracked asset on2026-10-09. It is not copied or installed by this study. Coverage proves availability, not original Wingdings outline/metric equivalence. After approval and X195 provider integration, compare actual deterministic PDF/raster glyphs to these native shape references and require non-emoji monochrome output. Retain bounded tolerances for geometric substitutions instead of claiming font-exact parity. Supplied genuine symbol face coverage wins over the Unicode substitute as already designed.

## Artifact hashes

| Artifact | Bytes | SHA-256 |
| --- | --- | --- |
| symbol-input.docx | 37353 | `cf89afd62caae73ed1b1914a99b06a2970f8e70f9d240627b8a42d2f248c7e66` |
| symbol-native.docx | 28027 | `4c6b1c446b5557b8736b6467449549fc3cc0d1ada84c36027a5c6b38636d8042` |
| symbol-first-export.pdf | 48525 | `50d84b6dcc1d408e8d85d45eb61530d217eae94c3c83d1202d2dcc4fe265c2c2` |
| symbol-reopened-export.pdf | 48525 | `5cfe26f7a27bb90235968972b951a09cf59502af7141d5f0132cacd908311afd` |
| symbol-first-export.png | 56178 | `18345936af5ab8feb860795fa645a3adefcdd127a8d664de885193f8e7a6d326` |
| symbol-reopened-export.png | 56178 | `18345936af5ab8feb860795fa645a3adefcdd127a8d664de885193f8e7a6d326` |

No local rendering implementation pass, all-symbol mapping proof, gate pass or asset approval is claimed here. Native source remains untouched, original fonts remain in place, and only ignored research artifacts were created.

## Cross-PR checklist glyph coverage

PR320's checklist preset uses U2610 for unchecked and U2611 for checked.
HarfBuzz14.4.0 coverage of all27 current bundled faces finds both glyphs
missing. The exact approved3916-byte PR319 symbol subset has U2611 but
not U2610. Its pinned full upstream Noto Sans Symbols2 face contains both,
with equal821-unit advances at1000 units per em. The complete read-only
receipt is .claude/scratch/S91-checklist-symbol-coverage.json.

The approved asset retains its exact bytes and hash. Do not treat
its16-target coverage as acceptance for the checklist preset, silently
change its approved-file scope, substitute an unrelated square or emit a
missing glyph. Resolve U2610 through the existing caller-font provider or
an exact separately reviewed asset change when needed. Native PDF/raster
appearance and deterministic fallback acceptance remain explicit joint
F-X200/F-X210 obligations, tested once against their actual provider.

## PR319 incremental golden-pixel refresh

PR319 advanced to f0e6be942d4e958f4a07014466c57e7e03f6b8b7. Its sixth commit
changes only scripts/golden_pixel_manifest.json beyond9a36bc9: attribution
and three contract/proposal/report pixel hashes. Four other sample hashes,
dimensions, DPI and pinned rasterizer remain unchanged. Full immutable trees,
31 authenticated complete source blobs totaling3935436 bytes, binary-capable
base patch, incremental patch and complete metadata/discussion receipts are
retained in /private/tmp/S91-PR319-f0e6be9-intake. Unchanged prior source
assessment still applies. This remains F-X210/Issue313, with no new F-ID.

Font bytes, mappings and diagnostic implementation are unchanged. The four
contributed modified static Gelasio faces still differ from the approved two
original variable Gelasio assets. The approved3916-byte Noto subset remains
exact and still lacks U2610. The requested-family cache provenance finding is
not fixed by re-recording pixels. Recompute actual deterministic baselines
from the integrated approved font graph under the exclusive baseline lease,
with declared and reviewed attribution. Do not copy contributor expected
hashes or reinterpret them as native metric-compatibility acceptance.

The read-only assessment predates the separate user approval of the exact
3916-byte subset. That approval is recorded above and does not expand to the
extra static faces, altered notices or a changed subset. No asset is copied,
font installed, source changed or baseline adopted by this refresh.

## Resolved bounded metrics and same-pass diagnostics contract

The authenticated final decision memo is
/private/tmp/S91-F-X210-final-contract/decisions.md. Its20 referenced source
and evidence hashes match. Valid in-memory WOFF reconstruction authenticated
all15 table checksums and whole SFNT checksum before actual HarfBuzz14.4
shaping, superseding only the earlier failed direct-WOFF measurement route.
No Microsoft font bytes or reconstructed SFNT were redistributed or saved.

For pinned Selawik Regular1.01 versus Segoe UI Regular5.31, isolated95 ASCII
advances and all20 measured kern-disabled string advances match exactly.
With kerning enabled18/20 strings differ: AVATAR differs0.26123046875em,
To0.10302734375em and the largest measured mixed-pair control0.39111328125em.
These are per-control observations, never a universal tolerance or identical
wrapping guarantee. Publish those differences and the absent Selawik kerning.
Selected Word Windows/typo metrics agree, while generic hhea metrics differ.
No genuine Segoe UI Bold700 reference was found, so Bold provenance and
style coverage do not become a measured Bold parity claim. Keep original
approved faces as approximate substitutes with these explicit guarantees.

Resolve section-square as WingdingsA7/F0A7 mapped semantically to U25AA,
based on the authenticated eight native symbol controls. It is distinct from
unchecked-checkboxU2610. Genuine supplied symbol coverage has precedence,
ordinary Wingdings text and unrelated PUA are not globally rewritten, and
unknown codes remain source-preserved with honest missing-glyph diagnostics.
Monochrome semantic fallback is not original-font outline or advance parity.

Select additive native/Python diagnostic companions in existing facade files:
one successful layout/encode returns the existing payload paired with owned
diagnostics. PDF returns native Result<(Vec<u8>,Vec<Diagnostic>)> and Python
(bytes,tuple[str,...]). Analogous PNG/raster companions retain old payloads.
Existing byte APIs keep their current return types. Reuse SVG's existing
diagnostic result, expose PPT TextFrameLayout's same-pass diagnostic vector
through its existing Python record, and feed X207 fit reports from those facts.
CLI emits the exact successful render's ordered facts to stderr once.
Do not rerender to infer diagnostics for earlier bytes or expose a last-render
mutable registry. No forwarding-only generic wrapper or new source file.

X195 owns bounded actual requested/selected contributing events and cache
replay. The FontId-global Aptos attribution bug exists in PR319, not current
canonical code. Preserve trace bounds and carrier budget accounting, retain
Aptos versus Aptos Display identity, and suppress fallback claims for genuine
actual caller coverage. Test cold/warm non-Aptos reuse, missing glyph fallback,
empty probes and exact same-provider output across measurement/export paths.
These are selected future gates, not already implemented consumers.

The exact revised3968-byte same-path subset adds onlyU2610, SHA
a95e7d7421e30e8ad304a0eb48aa18aefabcfec568a0f652c1039d72da999373.
The user explicitly approved this exact revision on2026-10-10, answering
"Approve the 3,968-byte revision". Root reauthenticated its3968 bytes and
SHA256 before recording approval. Use these revised bytes at the existing
approved ninth asset path and record the selected recipe in NOTICE-Noto.
All eight core approvals remain valid. Asset integration still follows the
F-X195 completion barrier and the normal implementation lifecycle.

Use diagnostic companion names by adding _with_diagnostics to the existing
Python byte-render methods, with their exact original parameters plus the
F-X195 caller-font keywords. Return (existing_payload,tuple[str,...]) from
one owned output. Native companion signatures retain corresponding native
render options/selection and one explicit font-files slice, with the same
owned diagnostic vector. Do not add a variant for every font-input spelling.
Existing SVG results retain their named diagnostics. Existing native layout
results already expose diagnostics and require no forwarding companion.

## Latest PR319 head equivalence

PR319 now points to0073a154d1f72bf4ab391a5257b343a4bca61d80. Its entire immutable tree
77b869661f0522efc6885725626ec740214604d3 is identical to the previous assessed head,
with the same parent commit. Diverged final commit identity introduces zero
source changes. Full current discussions and reused source blob hashes are
authenticated in /private/tmp/S91-PR319-0073a15-intake/receipt.json.
The prior source assessment and existing feature ownership remain applicable.
No duplicate work, asset approval, baseline or contributor pass claim changes.

## Joint checklist font acceptance, 2026-10-11

The current F-X200/F-X210 provider passed the four actual unchecked/checked caller and no-caller controls. R10 completed all35 bounded command rows with exit0, unchanged complete source guards and naturally reaped process groups. Receipt1b5d6206be78327e5a8664590494b0ce638d21659be607649fe2a4b25d1f40ec is /private/tmp/S91-F-X200-F210-checkbox-font-text-experiments-r10-prepared/actual-comparator-1/receipt.json. Original source-derived origins, exact approved3968-byte physical font, reachable painted CID and Unicode marker mapping are authenticated. Independent approved source-font PDF and original contour PNG positives are1.0 for every owner, with all wrong-character, blank and six-point negatives below the unchanged.99 threshold. Full original font experiment A remains distinguishing evidence only. Previous path-raster comparison failures remain failed, without a threshold or geometry change.

This completes the joint checkbox paint obligation. F-X210 still requires its remaining scoped package/license/risk gates, HLD, final review and integration. F-X200 still requires attributed baseline and policy/feature partition closure. No feature is marked complete by this one obligation.

## Existing font inventory carriers

Update the existing CLAUDE.md inventory and .github/workflows/ci.yml package-member inventory to32 fonts and10 legal files. scripts/test_sprint_workflow.py retains the actual inventory assertions with those counts. Independent review21 covers this three-carrier correction. No additional product file or new acceptance waiver is introduced. Actual verified archive measurements remain required before changing their existing README and policy carriers.

## Scoped completion evidence

Full current implementation and risk gates are qualified in whole-working review29. The consolidated scoped evidence is /private/tmp/S91-F-X210-crate-README-measurements-r1-prepared/consolidated-scoped-completion-evidence.json, SHA256b61ed332ad25d1912e9f67f1b18cef7ff9454ce6b46e8b1630ea0ef60a193084. Current product, assets and wheel-source bytes retain native967978 equivalence. Metadata and two independently reviewed installed fixture corrections are separate. Original genuine provider Base2 and consumer Base4 reached semantic failures before production adoption. Current native, installed, typing, genuine16 and joint checkbox gates pass. No failed wrapper or historical run is relabelled.

Seven verified dry-run archive source/member/mode proofs remain retained. Final metadata remeasurement and two policy recoveries, prose and diff passed in actual receiptbbf9b470473ea68f0b384d4ae27ac3fd388f0782c66f69256655eff92eb3a4fe. Policy completion is composite139 pass and2 existing skips. Actual hash49 passes at receipte4715741e26b8442e5d15c542a6e41b6ebd4341dcd36261d2e33f6d3813288a1 after only the12 reviewed Georgia-to-Gelasio values changed, with37 entries exact. Integrated sprint verification remains due.
