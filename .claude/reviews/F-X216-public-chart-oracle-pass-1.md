# F-X216, public chart oracle, pass 1

**Reviewed**: One test-only extension to the existing Python chart import specimen, current carrier SHA `24953c96754c133e72d61f29435f83b9a041bcf121355e616c74b8d0dfc02ff3` against exact retained preimage SHA `1daa3b89cbc05cf33b7497cc49ed0b13470c52da309c47371e7ac206e277658d`. Only the former chart-refusal block changes. This audit extends the scoped authority of regression-oracle review `dbe7961881b74251b4d0ed07550170d9eb9c169769d677294a8358023f01fa0f` to the corresponding Python specimen.

**Verdict**: 2 test-authority defects, 0 smells, 0 nitpicks. No production defect or executed installed outcome is claimed.

## Defects

### D1, the positive chart oracle selects the title placeholder

`/private/tmp/rdocx-f-x216-codex/crates/rpptx-py/tests/test_documented_examples.py:3693`

The source fixture creates the chart slide with default python-pptx layout index 5 at line 3677, then appends the chart at line 3681. The helper creates an ordinary default python-pptx presentation at line 1959. This is the Title Only layout, already identified by the same file's source-backed specimen at line 2029. Its title placeholder precedes the appended chart. The new `chart_slide.shapes[0].chart` selects that title placeholder rather than the chart. The reopen assertion at line 3707 repeats the incorrect selection. The existing public getter at `crates/rpptx-py/src/shape.rs:1241` refuses a non-chart with `shape has no chart`.

Select the actual chart member, assert exactly one chart and retain the source chart identity/order assertions. Apply the same actual-member selection on reopen. Do not delete the placeholder or substitute a blank-layout fixture to make the assumed slot correct, since the existing fixture also proves carried slide structure. This is a static setup diagnosis, not an observed runtime failure.

### D2, unrelated-target assertion observes a presentation that was never imported into

`/private/tmp/rdocx-f-x216-codex/crates/rpptx-py/tests/test_documented_examples.py:3691`

The positive operation targets a new empty `chart_target`. The unchanged-byte assertion at line 3711 instead checks `prs`, a separate populated destination that never received this chart import. It retains a useful sentinel for the later foreign-layout/index tests, but it cannot demonstrate preservation of unrelated members in the actual destination receiving the chart. The prior approved oracle correction explicitly requires destination unrelated-part preservation.

Seed the actual chart destination with an existing controlled member and capture its original slide/media/relationship bytes or equivalent exact package-part observations. After successful chart import and save/reopen, assert that those preexisting actual destination members remain intact while the new chart and workbook are owned by the imported slide. Keep `prs` and its later negative purposes separate if that avoids changing their setup. An unchanged uninvolved object must not be credited as the receiving destination's preservation proof.

## Preserved purposes and useful new assertions

The obsolete chart refusal is correctly removed under the approved F216 carry-graph contract. The new category, series-name/value, exactly-one embedded workbook byte equality, source byte equality and save/reopen assertions are meaningful once the actual chart member is selected. The exact preimage comparison shows that foreign-layout and invalid-index checks, original picture/link/background/notes import assertions, held-handle observations and differential reopen assertions remain unchanged. The two defects above do not authorize weakening any of those assertions.

## Smells

0 found in this bounded test-only audit.

## Nitpicks

0 found.

## Guards and acceptance limits

Current and retained preimage hashes were authenticated before review and the current carrier remains frozen at the same hash after the report. The reviewer made no source, runtime, CP, build, UI or Git changes. No installed test has been executed or credited by this audit. Fresh artifact/installed qualification remains pending exact admission. This report ends the review pass and returns the one-carrier writer lease.
