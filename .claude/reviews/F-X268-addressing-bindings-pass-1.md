# F-X268, addressing bindings, pass 1

**Reviewed**: Codex root reviewed the mechanism 6 Python consumers, private counter export and new regression on worker HEAD `6579e2e0dcc9625eaead146d7c324117d43c3bca`. The four-file diff has 114 added and 22 removed lines. Native addressing has its separate clean pass 3. This is not the whole F-ID microscope.
**Verdict**: 0 defects, 1 smell, 0 nitpicks

## Defects

None.

## Smells

### S1, the duplicate regression does not require the clone to remain present

`crates/rdocx-py/tests/test_core.py:14243`

The fixture starts with two paragraphs whose text is same. Pop and reinsert restore those two, and clone_content must add a third. After moving the table and reopening, the assertion accepts any count at least two. A lost cloned occurrence therefore passes. Require exactly three and check their distinct direct physical indices after the table moves to index zero. This makes the duplicate-position observation discriminate missing or aliased occurrences.

## Nitpicks

None.

## Evidence and limits

The loaded debug extension SHA-256 is `b3c7ee223d9465698570a5c5e58526727d98cc6a6933d57862c79696cfa9f165`, independently checked against the file. The private counter is exported through the native Document counter. It is not a stable Python export.

The old installed mechanism 5 baseline passed two semantic cases in 2.46s. On the new extension, each of the first two batches passed five existing consumer cases and failed the two parameterized new cases on an invalid cold-count assumption. Those failures were not product RED evidence. The corrected isolated paired node passed in 7.83s. Both 20 and 80 record first_measured 0 and repeat 0.

The corrected node exercises body_locations through an end-index refusal and exercises valid indexed edits, table movement, stale handles, controls, import and reopen. Functional checks run at both sizes even when the debug helper is absent. Counter assertions follow the functionals and use local records. Zero first-measured work is not claimed as a cold construction cost or a total constant-time result.

## Not found

No additional correctness, panic, binding signature, export or structural finding was established in the inspected consumers. The new helper preserves the selected body and table filters. HLD alignment, complete forms and settings composition, the current native seven-case release execution, mechanisms 7 and 8, measurements, Word qualification, hash harness and full sprint verification remain pending.
