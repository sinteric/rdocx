# F-X162, correctness, pass 1

**Reviewed**: `080438e1..2996f886`, 7 files, 142 insertions and 20 deletions.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness, contract, panic, OOXML, test, and structure checks found no issue. The section scan assigns each paragraph, table, and body control item to the section ending at the next direct paragraph break, or to the final body section. It does not inspect section properties nested in a cell. The regression gate failed on the pre-fix branch with the reported 348.6 pt first-section measure and passed after the source change. The only hash movements are the two declared `feature_showcase` PDF entries.
