# F-X216, public chart oracle, pass 2

**Reviewed**: Static test-only successor to public-chart oracle pass 1, against the exact original specimen preimage. Current `crates/rpptx-py/tests/test_documented_examples.py` SHA is `31e1918037d2b1527bf1f1ca4f146be37990f1d732c035414880cfc82c56fc22`. Original retained preimage SHA is `1daa3b89cbc05cf33b7497cc49ed0b13470c52da309c47371e7ac206e277658d`. Pass-1 report SHA `cec3a9336c83d14ece237ec265c5b83bca774bde9c9664c3b97d3a9a5ca419f3` remains unchanged. Only the old chart-refusal block is replaced.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

0 found in this bounded test-authority successor.

## Findings resolved

**D1**: The imported source retains its original Title Only fixture. The test selects members by actual `has_chart`, requires exactly one, and then inspects that actual chart. Reopen repeats the same unique actual-member qualification rather than assuming shape index 0. Category and series value assertions retain the independently authored fixture values. Workbook transfer requires exactly one source workbook and byte-equal imported workbook bytes.

**D2**: The actual `chart_target` now contains a controlled preexisting blank-layout textbox slide before import. The test captures that held slide's XML and actual serialized package before the chart operation. It asserts unchanged held-owner XML, exact original `slide1.xml` and its relationship part after the operation, then reopens and checks the original textbox text. The newly imported chart is independently selected from the appended slide. The separate `prs` sentinel remains for its existing later negative and picture/link/notes/background purposes, without being credited as the chart destination's preservation proof.

Comparison with the retained original file shows no changes outside this corresponding chart-oracle block. Foreign-layout/index refusals, original import structure/media/notes/background and differential reopen assertions remain intact. The change does not weaken production validation, ignore a failing test, remove a placeholder or mutate the frozen 100-case corpus.

## Smells

0 found.

## Nitpicks

0 found.

## Authentication and limits

Current and retained preimage hashes were independently authenticated before review. Current carrier remains exactly `31e1918037d2b1527bf1f1ca4f146be37990f1d732c035414880cfc82c56fc22` after writing. No runtime, installed probe, build, CP, source, UI or Git change was performed. The assertions provide a meaningful defining control but their actual installed execution is still pending exact artifact/admission. This static zero verdict closes the two bounded test-authority findings only. It claims no full public or F-X216 acceptance. The pass ends here and returns the one-carrier writer lease.
