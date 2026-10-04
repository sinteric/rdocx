# F-X161, serialization, pass 1

**Reviewed**: working diff, 16 files, 193 insertions and 55 deletions
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness: the canonical `w` binding is omitted only for its exact URI, while alias and foreign prefix tests retain their declarations. Contract: the changed Word part writers use compact serialization and preserve raw subtree boundaries. OOXML: the source tests and the 11-case producer matrix reopen output with namespace and `mc:Ignorable` coverage. Tests: the 50-paragraph reproduction failed on the claimed base, then passed. The baseline diff has exactly 20 Word XML entries and no PDF or PNG movement. Structure and panics: no new type, trait, module, unchecked index or panic path was introduced.
