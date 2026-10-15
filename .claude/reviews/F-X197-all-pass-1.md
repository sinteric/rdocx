# F-X197, all aspects, pass 1

**Reviewed**: Entire working diff on `work/f-x197-codex` against claimed Base and HEAD `b96c7888b7d99bf1ab6c720ea5ba318c7103d611`. The diff changes 22 existing files, with 1,400 inserted and 199 deleted lines. Binary diff is 98,065 bytes, SHA-256 `ff99dce480535f902a8fd1bf2cda8df0af47c07fe7fb08568286d70937819bad`.

**Verdict**: 3 defects, 0 smells, 0 nitpicks.

## Contract and evidence

Read CLAUDE.md, WORKFLOW.md, the microscope command, the complete approved F-X197 design and progress, HLD03 architecture, HLD05 colour handling, HLD10 binding ownership and colour surfaces, and the differential-testing reference. Reviewed every changed Rust, Python, stub, typing, test, manifest and lockfile hunk, with surrounding native colour and owner-resolution code.

The R1 freeze receipt is `/private/tmp/S91-F-X197-evidence/implementation-freeze-r1.json`, SHA-256 `5b505c48db18cb465dd4d911f5c3b10438bb8805297be47c3f141f264ffb86f7`. Independently reauthenticated all 22 current file hashes and sizes, current HEAD, and the complete binary diff immediately before this report. They match the frozen input. Native parsers and serializers are unchanged.

Authenticated the retained Base RED receipt and log. Its two missing nested colour failures are observed installed-wheel failures. Independently compared all 228 selected Git inputs in `Base-production-equivalence.json` against both claimed Base and source194 `8886305ec6bc5b0586a2388f9ee2c6cc2fc769a8`, including blob OIDs, sizes and SHA-256. All match. This supports the recorded reuse of existing F-X194 wheels for scoped Base equivalence. It does not establish a newly built complete F-X197 Base environment.

Authenticated the pinned python-pptx 1.0.2 brightness probe, observations and three actual oracle module hashes. All 56 recorded after/reopen colour structures agree. The oracle accepts True as 1.0 and NaN as zero. The approved local finite-number policy deliberately refuses both and includes refusal controls. This is a qualified intentional divergence, not a local runtime pass. The oracle receipt explicitly disclaims local F-X197 gate credit.

This is a source review. New tests are uncompiled. Installed runtime, wheels, strict mypy, recursive stubtest, shared-support unit tests, WASM, publication, hash and rendering gates remain pending. HLD and package carriers are also pending under the approved completion phase. No Cargo, import, oracle probe, formatting or source mutation was performed during this review.

## Defects

### D1, malformed surrogate strings bypass the parameter-qualified validation message

`crates/oxml-py-support/src/lib.rs:15`

A Python string containing a lone surrogate, for example `"\ud800"`, reaches `extract::<String>()?` before the explicit ASCII-hex validation. PyO3 0.29 converts through PyString.to_cow and documents UnicodeEncodeError for an unpaired surrogate. That exception propagates unchanged, so setters do not report their full affected parameter or the accepted ASCII-hex syntax. The approved reuse contract explicitly retains parameter-qualified errors at `.claude/plans/F-X197-design.md:261`.

UnicodeEncodeError already subclasses ValueError. The defect is the missing property-qualified validation message, not an incorrect broad exception category or evidence of source mutation. The Python RGBColor.from_string path rejects the same malformed string through its own explicit validation. Normalize this conversion failure into the shared malformed-string error and add the surrogate case to the existing support and installed owner/error-atomic controls. The current pre-mutation ordering must remain intact.

### D2, the opaque-child preservation fixture contains only modeled transforms

`crates/rpptx-py/tests/test_documented_examples.py:5173`

The test named `test_powerpoint_colour_replacement_preserves_family_specific_transforms_and_opaque_children` injects alpha, tint and gamma. Gamma is a modeled ColorTransform, declared at `crates/oxml-drawing/src/color.rs:134` and parsed explicitly at `crates/oxml-drawing/src/color.rs:169`. The preservation assertions at `crates/rpptx-py/tests/test_documented_examples.py:5205` therefore prove modeled-transform retention, without exercising the raw-child carrier that the new font, theme and brightness paths must preserve.

The approved plan requires generated foreign metadata controls at `.claude/plans/F-X197-design.md:164` and explicit unknown-transform controls at `.claude/plans/F-X197-design.md:250`. Add a genuinely unmodeled qualified child and assert its appropriate source retention through read, mutation and save/reopen, including brightness edits. Keep the approved same-kind retention and kind-change clearing policies distinct. This is a test coverage defect, not a claim that the current cloning implementation drops opaque XML.

### D3, rendering controls omit themes and the Word control uses ordinary font discovery

`crates/rdocx-py/tests/test_formatting_tables.py:1226`

The Word rendering gate assigns only literal RGB values and calls the ordinary Python render_page_to_png route. That binding calls the ordinary native render_page_to_png_with_options at `crates/rdocx-py/src/document.rs:2313`. The native route uses layout_with_options at `crates/rdocx/src/document.rs:28261`, whereas the separate deterministic route explicitly uses layout_for_options(options, true). The test does not select deterministic font mode.

The PowerPoint pixel control at `crates/rpptx-py/tests/test_documented_examples.py:5252` also assigns only literal RGB. Its binding does use deterministic slide rendering, but no new actual-render assertion establishes theme selection or brightness reaching pixels. Structural save/reopen assertions cannot establish that distinction.

The named rendering gate requires deterministic PDF/raster colours including theme controls at `.claude/plans/F-X197-design.md:160`, and deterministic fonts are mandatory at `.claude/plans/F-X197-design.md:195`. Qualify the Word control through an existing deterministic route and add actual theme render discriminators, with brightness where relevant to the expanded colour contract. Keep the literal-RGB and same-input-form controls. This finding concerns the missing planned acceptance tests and mode selection, not a demonstrated rendering defect or a system-font baseline change.

## Smells

None.

## Nitpicks

None.

## Aspect coverage and not found

- Correctness: no additional defect found in legacy triple extraction, validated ASCII slicing, theme mapping, Word cached literal/tint/shade retention, PowerPoint same-kind replacement, font-only clearing, source snapshots or brightness arithmetic. D1 concerns error contract handling.
- Contract: D1 and D3 above. The approved live Word/PPT getter compatibility break, scalar RGBColor migration, supported enum vocabulary, application-specific None/auto policy and deliberate bool/NaN divergence are represented. No additional contract finding.
- Panics: zero findings. The hex indices and expect are protected by exact six-byte ASCII validation. No new unchecked untrusted owner index or numeric conversion was found.
- OOXML: zero implementation findings. Reviewed transform/raw-child clones, shadow alpha treatment, ordered source updates and native serializer reuse. D2 is a missing opaque-source proof, not a demonstrated schema or preservation failure.
- Tests: D2 and D3 above, plus the missing surrogate discriminator in D1. Reviewed complete setter inventories, nine named gates, source/error atomicity, structural invalidation, cell text-generation invalidation, read-only/no-op controls, supported theme mappings, finite brightness and typing/stub migrations. Runtime results remain pending.
- Structure: zero findings. The neutral shared extractor has two actual binding consumers. Its PyO3 dependency and exact lock edge are approved. No new module, file, trait, generic, feature flag, native parser model or dynamic dispatch was introduced.

The review ends with this record. Remediation and subsequent runtime checks belong to a separate implementation phase.
