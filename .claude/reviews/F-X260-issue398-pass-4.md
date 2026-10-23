# F-X260, issue398, pass 4

**Reviewed**: independent `/microscope F-X260 --working --aspect issue398`, branch `sprint/s95.3`, frozen HEAD `3c3d7d69623ed30f7994baa28c1bda60b9263a71`. Eleven-file scope envelope has 1756 insertions and 160 deletions against HEAD, including excluded shared-file hunks. Incremental focus is D2 section admission and retained namespace-owner shortcut exclusion.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. D1 remains resolved within the prior review scope. D2's introduction bypass is repaired, but its preservation contract still fails on a valid imported section attribute. No acceptance gate is discharged.

## Scope and authentication

Read CLAUDE.md, canonical WORKFLOW and microscope instructions, F-X260 design and latest progress, complete Issue398 pass3, and relevant HLD03/10/12 sections and additions. Both delivery records still mark F-X260 in-progress. Consumed the unchanged binding, batch/index and first-run preparation audit conclusions without repeating their large audits.

All 25 full-file entries in `/tmp/s95.3-issue398-pass4-source-manifest.json` matched before probes and immediately before writing. HEAD remained frozen. The imported current extension was `crates/rdocx-py/python/rdocx/_rdocx.abi3.so`, SHA-256 `3f1d23f57f0a79ed5193d372713627fb4a8909a0b40330d7685baeb9e40836ec`. The successful editable dev install is recorded in `/tmp/s95.3-section-and-alias-dev-build.log`. Source association uses that supplied build record and manifest, not embedded compiler attestation.

Excluded F-X259 process changes, TOC source and five TOC tests, field/regression/HLD08 hunks and unrelated dirty content. No agents, builds, suites, benchmarks, oracle runs, commits or state changes occurred. Public probes used in-memory ZIPs and disabled bytecode.

## Defects

### D2, temporary paired parsing falsely rejects unchanged opaque section attributes

`crates/rdocx/src/document.rs:8178`
`crates/rdocx/src/document.rs:8182`

Import a paragraph with this valid, self-contained property subtree:

```xml
<w:pPr><w:sectPr xmlns:x="urn:x" x:flag="keep"/></w:pPr>
```

The paragraph also contains a regular run with text `old`. Read `paragraph.xml`, then call `paragraph.replace_xml(paragraph.xml.replace('>old<', '>new<'))`. No section byte or namespace meaning changes.

Actual current result is `RdocxError: OXML parsing error: invalid value: root attribute prefix `x` is unbound`. The operation refuses atomically and retains the old text and exact package bytes. Expected is a savable text edit preserving the unchanged imported section and its opaque attribute, as promised by `.claude/plans/F-X260-design.md:425` and `docs/hld/10-bindings-spec.md:778`.

The new admission expands the raw Empty section into a paired temporary element and wraps it in a paragraph whose ancestors declare only `w`. Closing namespaces on the section does not make this existing parser route consume its foreign root binding correctly. The paired section parser captures root attributes through its prefix scope at `crates/rdocx-oxml/src/document.rs:1804`, and root-attribute capture resolves against that supplied scope at `crates/rdocx-oxml/src/text.rs:475`. The original Empty representation remains raw through the paragraph-property fallback at `crates/rdocx-oxml/src/paragraph_properties.rs:913`. Thus normalization introduces a stricter, incorrectly scoped parse for previously preservable bytes. Adding the same declaration to the submitted paragraph root did not cure the current refusal.

This is an actual compatibility regression, not merely a hypothetical parser limitation. The preserved old release imported from `/tmp/s95.3-final-release-probe/rdocx/_rdocx.abi3.so`, SHA-256 `be930059d0ff87f26dd65e6579b0b8490a5a691790871b574a300f2751536acc`, accepted this exact text-only edit, retained the section XML exactly and saved/reopened with text `new`. Current import and read-only save also succeed before mutation. A separately attempted originally paired foreign-attribute section fails at import and is not counted as an additional admission regression.

Retain D2 because this is the same physical-section admission contract on a different discriminator. Repair must preserve supported imported attributes while retaining section-change refusal. No blanket parser expansion is requested by this finding.

## Shared callers and successful controls

The scanner identifies direct qualified section children of the selected pPr using expanded names and parent routes, handling Start and Empty. Ordered `Vec<CT_SectPr>` equality preserves multiplicity at the shared boundary. Both staged `replace_paragraph_xml` and local `try_replace_plain_body_paragraph` invoke that boundary at `crates/rdocx/src/document.rs:23512` and `crates/rdocx/src/document.rs:27307`. The local modeled-section guard remains before it, preventing an unprepared modeled-owner transition.

Executed 16 direct invocations of existing section/alias assertion cases, all passing. Their assertions check exact refusal bytes, retained text, exact unchanged imported paragraph XML and reopen. These cross first-run, same-run and staged topology changes with Empty, paired-empty and populated sections, plus w/q imported aliases. Four additional cell/header section-introduction cases refused atomically through the shared staged provider.

Additional bounded probes confirmed rejection of changed retained w:rsidR, duplicate section introduction and foreign-namespace substitution. An unrelated alignment property edit and an equivalent qualified q section succeeded. Unchanged unknown section children and w:rsidR survived text edits exactly. Imported duplicate sections refused through existing checked property validation. These outcomes do not establish arbitrary malformed-import edit support.

Retained namespace owners now block local eligibility at `crates/rdocx/src/document.rs:3890`. Native replacement and removal consult `plain_direct_paragraph`, and the Python XML and tail-removal shortcuts consult that same eligibility before live mutation. Checked fallbacks stage and reconcile. Additional probes retained the paired outer-q paragraph dependency while editing or removing its owner, removing another paragraph, and removing a range. Refusals preserved exact bytes, while the unrelated text edit produced a savable reopened document. No unsavable successful edit was observed in these controls. Plain generated reporter input still has no retained nested dependency and retains local eligibility.

## Evidence, zero categories and pending gates

Consumed supplied semantic section RED, 3 failed and 6 passed, current union 73 passed including five excluded TOC cases, native controls 27 passed and 3 ignored, and scoped Clippy success. The earlier E0502 compile failure is not a successful build. The genuine old alias save failure remains distinct from exploratory passing and unbound-fragment cases.

Correctness/contract/OOXML: D2 counted once. Panics: 0. Additional caller/publication defects: 0. Tests: 0 separate findings, though the opaque-attribute discriminator is missing. Structure/API: 0. Smells: 0. Nitpicks: 0.

Pre-D2 Wordcore470, hash49 and both WASM results remain historical. Missing soffice, fonts, public inputs and CLI explain recorded environment failures without proving product defects. Native Word qualification, fresh release rebuild, all ten strict ratios and applicable final integrated gates remain pending. No thresholds, baselines or review bounds changed. Only this report was written. Review stops here.
