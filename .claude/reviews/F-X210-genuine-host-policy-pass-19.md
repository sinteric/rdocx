# F-X210, genuine host policy, pass 19

**Reviewed**: The wrong-label control in the external acceptance-current-handles-r4.py at SHA25634b835a5004518b2afe37f57b12c24eac4d4fcee18227df1322caa2a7050e162, its actual failed run and independent physical-font diagnosis. Production source remains12b951ed369c2aed247237d43d2a26611b7fc7c5ff9e4c2b5fec1329338b40ea.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, caller font input does not disable the normal host fallback policy

`/private/tmp/S91-F-X210-genuine-local-acceptance-r3-prepared/acceptance-current-handles-r4.py:294`

The comment and exact Noto-outline assertion at301 assume that supplying a Unicode fallback excludes discovered host fonts. The Word entrypoint explicitly uses the normal system and bundled policy at crates/rdocx/src/document.rs:28629. System discovery remains enabled at crates/oxml-layout/src/font.rs:836. The independent diagnostic report at SHA256f0892e0a301bf8c395e4671c9e66ba00bc82b82aecc43fa9ffc88aac2d359d23 records an actually painted Word CID1 with ToUnicodeU2713 and a nonempty HiraginoSans outline. The original exact Noto assertion fails. The same PPT operand passes the original Noto outline and unused-resource negative control. This is a faulty prepared acceptance assumption, not evidence of incorrect semantic Wingdings conversion or a production font-priority defect.

Keep the failed run immutable. Revise the normal-host control to prove the actual mapped character, physical source-derived visible checkmark and rejection of the wrong Carlito character. Independently run the exact Noto physical-outline control through the existing deterministic caller-and-bundled native entrypoint at crates/rdocx/src/document.rs:28643. Authenticate current build artifacts and source fonts before deriving expectations. ToUnicode alone is insufficient physical-shape evidence. Do not change production fallback priority or infer16 passing controls from the interrupted run.

## Smells

None.

## Nitpicks

None.

## Not found

No additional correctness, contract, panic, OOXML or structural defect was found in this narrow acceptance-policy review. Earlier production and installed-runtime reviews retain their stated scope. Only this review file was written during this pass. Return to separate external-fixture remediation and actual acceptance execution.
