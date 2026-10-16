# F-X234, measured metadata, pass 1

**Reviewed**: newly added metadata-only diff in README.md, scripts/readme_doctests.py and the approved design clarification. Product source remains covered by the independent all-aspects pass1.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Not found

Correctness and contract: only the measured rdocx archive tuple changes, with
37members and51,954,517normalized member bytes. Independently read the generated
archive and ran the actual repository measurement validator. The observed
3,733,495compressed bytes differ by5bytes from the recorded3,733,500, inside
the unchanged64byte tolerance. The10MiB limit, source policy and unrelated
measurements are unchanged. The plan explicitly records the existing carriers.
Structure, OOXML, panics and test behavior receive no new implementation here.
This pass ends after reviewing the newly added metadata, without repeating the
already clean product-source review.
