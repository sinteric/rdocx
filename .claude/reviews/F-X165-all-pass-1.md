# F-X165, all aspects, pass 1

**Reviewed**: uncommitted worker diff, 12 files, 319 changed lines
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, PDF oracle gate can silently skip or use an unreviewed version

`crates/rdocx-py/tests/test_core.py:109`

The Issue 253 PDF text test skips when Poppler is absent and accepts any
installed version when present. The design plan requires a pinned Poppler
comparison. On a machine without the tool, the acceptance gate would pass
without testing the PDF text layer. Make the oracle mandatory and assert the
reviewed version before comparing text.

## Smells

None found.

## Nitpicks

None found.

## Not found

No other correctness, contract, panic, OOXML, test, or structure findings.
