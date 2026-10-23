# F-X260, issue398, pass 3

**Reviewed**: independent `/microscope F-X260 --working --aspect issue398`, branch `sprint/s95.3`, HEAD `3c3d7d69623ed30f7994baa28c1bda60b9263a71`. Eleven scoped files have 1615 insertions and 155 deletions against HEAD, including excluded shared-file hunks. Incremental focus is physical cell editing and first-run local publication.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. Previous D1 is resolved within this review scope. New D2 blocks the promised section-refusal contract. Sprint remains blocked.

## Scope and authentication

Read repository guidance, canonical WORKFLOW and microscope, the F-X260 plan, latest progress, complete Issue398 pass2 and cited publication, lifetime and binding-test HLD sections. F-X260 remains in-progress in both delivery records. Consumed earlier batch/index audit conclusions rather than repeating those audits. Examined their interaction with appended owners, physical routes and publication.

All 25 entries of `/tmp/s95.3-issue398-pass3-source-manifest.json` matched before probes and immediately before writing. HEAD remained unchanged. Actual imported extension was `crates/rdocx-py/python/rdocx/_rdocx.abi3.so`, SHA-256 `ae865d44bc9cefc73aff5ebccbd3e565a445e3da8731bb27c909f95936201d55`. Successful editable-install evidence is `/tmp/s95.3-controlled-cell-dev-build.log`. Source association relies on that record and the supplied freeze, not embedded compiler attestation.

Excluded F-X259 Issue400 field/regression/HLD08/TOC hunks, workflow/adapters and unrelated dirty source from the verdict. This is not sprint review4. No agents, builds, suites, benchmarks or external oracles ran.

## Defects

### D2, self-closing section properties bypass the promised first-run refusal

`crates/rdocx/src/document.rs:27260`

The newly extended local admission compares only modeled `properties.sect_pr`. A self-closing section element is preserved as raw paragraph-property XML instead of populating that field. The Start-event parser models `sectPr` at `crates/rdocx-oxml/src/paragraph_properties.rs:663`, while the Empty-event fallback records its raw XML at `crates/rdocx-oxml/src/paragraph_properties.rs:913`. Consequently both modeled section options remain absent and the check admits a newly introduced physical section boundary.

Actual bounded public reproduction on the authenticated extension:

```python
doc = rdocx.Document()
p = doc.add_paragraph("")
p.replace_xml(
    '<w:p xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">'
    '<w:pPr><w:sectPr/></w:pPr><w:r><w:t>x</w:t></w:r></w:p>'
)
```

Actual: success, `p.text == "x"`, with `<w:sectPr/>` retained in paragraph XML and saved/reopened content. The section collection still reports one section. Expected under the approved first-run contract is atomic refusal of introduced paragraph-owned section properties (`.claude/plans/F-X260-design.md:376`). Replacing the empty element with `<w:sectPr></w:sectPr>` or a section containing `pgSz` refuses and preserves bytes. Thus equivalent XML event representations receive different admission.

This is a shared pre-existing parser/admission gap exposed on the newly admitted first-run path, not evidence that the new guard alone introduced it. Existing-run equal-path replacement and a nonzero topology change taking staged replacement also accept the self-closing form. Staged/native equivalence therefore does not discharge the explicit refusal contract. Repair must cover the shared section authority boundary, with both event representations and unchanged-section controls. It must not merely disable the first-run shortcut.

## D1 and affected consumers

D1's physical-coordinate invariant now holds in the examined compositions. Cell callback editing resolves the original native `cell_paragraph_story_location`, then calls `edit_story_paragraph` (`crates/rdocx-py/src/paragraph.rs:269`). Controlled and direct ordinals therefore select the same physical source as immutable reads. Body and related providers remain unchanged. No neighbor-cache refresh conceals a wrong physical write.

Traced every shared callback caller: run tab/break, text, field and removal, paragraph append/hyperlink, equation, text and alignment. Candidate publishers retain their existing revision policy. Alignment edits live source through the native atomic provider. Run-content publication still validates current run-parent authority before moving candidate text and source, retaining run/font scopes. Related and paired paths retain native fingerprint refresh and refusal.

Executed 42 bounded existing assertion invocations in memory: 20 first/middle controlled-cell combinations, two first-run lifetime cases and 20 body/cell/nested/header/paired run cases. All passed. Eight additional cell probes covered alignment, hyperlink, field and removal on both physical targets, unchanged neighbors and reopen. All passed after correcting an exploratory assumption that held paragraphs survive conservative `Run.add_field` publication. The initial stopped probe is not counted as a passing batch.

## First-run publication and evidence

The five-field preparation struct stages exact existing run successors, newborn authority and collision buckets before native admission. Commit appends births without shifting earlier parents. Hash buckets retain exact-route confirmation. Run generation replacements remove prior keys. No historical owner inventory is retained.

Inspected tail suffix handling and all inventory consumers for ordering assumptions. Only independent appended runs with earlier parents survive the shortcut. Non-contiguous removed descendants force reconciliation. A separate successful probe combined three locally born run families, suffix index shifts, later conservative removal, fresh edits, reopen and unchanged first stale cause after append. The work bound remains conditional on actual appended suffix size.

Unknown Word and non-Ignorable foreign children refused atomically in independent probes. Existing cases establish no-op identity, stale old content/run/font views, follow-up exact-route edits and full publication. The exploratory three-refusal batch stopped at D2 and is not reported as wholly passing.

Consumed actual supplied evidence: 11 binding units green, corrected 10-case controlled-cell semantic RED, 46-case repair union and expanded 56-case focused union, plus scoped Clippy. The 56 includes excluded TOC cases. Inspected assertions rather than treating totals as universal coverage. Initial missing-import NameErrors are invalid harness observations. First-run old-source RED is 257 visits at 50 paragraphs, not a completed 800-paragraph RED. Current work tests cover both sizes, but do not meter every parser, allocation or hash operation.

## Explicit zero categories and pending gates

Correctness/contract/OOXML: D2, counted once. Cache freshness and owner lifetime: 0 additional findings. Panics: 0. Tests: 0 separate findings. Structure/API: 0. Smells: 0. Nitpicks: 0.

All seven edit and three read strict release ratios require changed-source qualification. Earlier read passes and the edit ratio7.336295 failure remain historical evidence. No final release pass is claimed. Earlier49hash predates the new native guard. Native Word, qualified fonts, final hash, integrated suites, WASM, packaging and matched riders remain pending. No thresholds or baselines changed.

Only this report was written. Review stops without remediation or state changes.
