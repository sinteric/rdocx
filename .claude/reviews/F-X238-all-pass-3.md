# F-X238, all, pass 3

**Reviewed**: Full staged and unstaged working delta in /private/tmp/rdocx-s94-x238 against resumed HEAD55144ff81073f538544c3093404bba0f87f1ef55. 20 files,1,143 insertions and102 deletions, including preserved pass1/pass2 records and the expressly approved section.py file. Original claim Base a644dd9443346223e0003066bbfd33dc7937e3c7 remains provenance, not this reconciliation baseline. Read the approved current plan, progress, prior findings, cited HLD contracts and complete source/stub/test/documentation diff. F-X238 remains in-progress.

**Verdict**: 1 defect, 0 smells, 0 nitpicks. D1 and D2 resolved. D3 is a new test observer defect.

## Defects

### D3, The new form and signing interaction test catches the wrong exception family

`crates/rdocx-py/tests/test_python_docx_parity.py:1637`

The test expects RuntimeError when reading a run intentionally retired by an actual form mutation. The established exception is rdocx.StaleElementError, which inherits RdocxError rather than RuntimeError at `crates/rdocx-py/python/rdocx/__init__.py:25`. The actual fresh installed full Word run fails at line1638 with that StaleElementError and explicitly names Document.bind_content_controls as the first invalidator. See `/private/tmp/F-X238-installed-prefix-run-2/logs/rdocx-acceptance.log:16` and line19. The run completed876 passed and1 failed, not a full-suite pass.

The product has performed the approved Run retirement. The new observer rejects the correct exception and prevents the remaining legacy-fill, Signing and exact-no-op assertions from executing. Catch the established specific stale exception and retain meaningful retirement/cause assertions, then execute the remaining interaction. No production owner-policy or exception-family change is indicated. This test defect blocks completion.

## Resolved findings

### D1, The established picture exception family remains restored

`crates/rdocx-py/src/document.rs:9147`

The one-dimension refusal still uses the existing native Error::Other to rdocx_to_pyerr mapping, preserving RdocxError and the width-and-height alternative. The existing core compatibility test is unchanged. The explicit parity regression requires RdocxError, unchanged bytes and held authority at `crates/rdocx-py/tests/test_python_docx_parity.py:1508`. Earlier actual D1 RED/GREEN evidence remains classified in pass2. No reintroduction was found in this reconciled source.

### D2, The real section module provides the exact typed import path

`crates/rdocx-py/python/rdocx/enum/section.py:1`

The user expressly approved this new file. It defines the seven pinned enum values and identity aliases, replacing the temporary enum.text declarations and runtime-only sys.modules alias. The existing stub imports the real namespace at `crates/rdocx-py/python/rdocx/_rdocx.pyi:10`. The direct import is included in strict smoke at `crates/rdocx-py/tests/typing_smoke.py:601`, and the runtime regression compares the exact pinned oracle values and separate module identity at `crates/rdocx-py/tests/test_python_docx_parity.py:1441`.

Current freeze2 installed strict positive import exits0. The negative consumer exits1 with exactly four intended assignment/missing-member errors, without missing-import suppression. The reviewer authenticated the receipt, consumer files and both logs in /private/tmp/F-X238-D2-installed-freeze2-1. Receipt SHA256 is49bfc0c610f2b4cfd3bf028547db9c79a39c31914d731bd14f713ff81332f851. This resolves D2, independently of D3 and the remaining feature gates.

## Smells

0 findings.

## Nitpicks

0 findings.

## Not found

- **Correctness**: No additional finding in prepared paragraph/heading/page-break/table creation, physical-body insertion, run/font aliases, checked highlight mapping, binary streams or explicit unsupported alternatives. Current FormValues and Signing certificates, current-state verification, notes/unit behavior, math and ZIP9 remain intact in the reconciliation. The two new integration controls are at test_python_docx_parity.py:1620 and:1651. D3 prevents completion of the first control's remaining assertions and is not interpreted as a passing interaction.
- **Contract**: No additional finding. The real enum.section module is within explicit approval. Tuple subtype behavior, frozen Section alternatives, unchanged native picture sizing, unsupported table columns/section cloning and all three first-line divergence pointers remain bounded by the plan. No unrelated F-ID, delivery ledger, native rendering, dependency or baseline change is present.
- **Panics**: 0 findings. Added heading levels and color indices are checked. Existing inserted-paragraph invariant follows successful native insertion. Fallible stream and owner preparation precede publication.
- **OOXML**: 0 findings. Existing native providers still own style/run/table/picture authoring and source order. No second binding XML parser, namespace-local admission or unmodelled XML stripping route was introduced. Signed stream saving serializes through the existing native staging path.
- **Tests**: No additional finding beyond D3. Exact32 requirement dispositions and the three original divergence pointers remain distinct from unchanged raw corpus reports. Saved-file failures cannot receive explicit-refusal credit. D1 retains its precise exception assertion. D2 now has actual installed direct-import positive and negative qualification. Remaining full/corpus/scoped results are not invented.
- **Structure**: 0 findings. The private styles tuple subclass owns its refusal behavior and the getter returns it through a checked PyTuple subtype cast at crates/rdocx-py/src/document.rs:8190. The table-style helper has two actual callers. The one new product file is explicitly authorized. No new trait, generic parameter, dependency or feature was added.

## Source and evidence boundaries

The reviewer independently authenticated all3,527 frozen inputs against /private/tmp/F-X238-reconciled-freeze-2/source.json, SHA256 e541433ae8f84b2850c2a113b3c26e241241d3a3cea3dc4f82cca896835ab7bb. The actual git diff HEAD --binary and frozen diff.patch both hash to2222eba7a28c5d224a1b6138c44d83227b24ab3aa4075a1ca7a1c41a28184e83. These checks passed immediately before writing this report. No staged intent was omitted.

The fresh ordinary Word wheel SHA256 is075a71860a88681910b7d1570d7ecd29fc74035cb8a5eee8dd208de5629829f6. The reviewer authenticated all13 archived members and RECORD hashes/sizes, installed product members and matching current Python/stub sources. Installed native SHA256 isdb63ce62586de1d3d98501c192e563213301e747ee4f2313aac771de308ef0b2. Installed dist-info RECORD was not equated with its archived predecessor.

The actual Python3.9.25 predecessor import failed on evaluated union annotations. The existing __init__.py now enables postponed annotations at line3. Fresh installed minimum-version GREEN exercises direct section imports, tuple behavior and native alias creation. The reviewer authenticated interpreter and RED/GREEN log hashes at /private/tmp/F-X238-resume-prefix-1/python39-import-red.json and /private/tmp/F-X238-installed-prefix-run-2/python39-import-green.json. This is a minimum-version control, not a full Python3.9 suite claim.

The current full Word outcome is876 passed and1 failed. Other running corpus/scoped checks remain pending. Completed Presentation focused/typing/stub evidence may carry only through its specific authenticated unchanged-input closure, never as a fresh all-family integrated pass. Original immutable corpus failures and stale merge-marker diagnostics remain preserved. No full feature, integrated sprint, hosted or completion acceptance is claimed.

The reviewer made no source edits, builds, cache mutations, GUI operations, staging or commits. Only this untracked review record was written. Review pass ended and write ownership returns to the orchestrator for separate remediation.
