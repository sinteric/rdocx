# F-X166, all aspects, pass 1

**Reviewed**: uncommitted F-X166 worker diff against claim base, 9 tracked files, 2,087 added lines and 214 deleted lines
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, combined Word differential gate is pending

`.claude/plans/F-X166-design.md:47`

The plan requires the three Issue 254 picture cases and five Issue 255 final-block pairs in Word for Mac on the combined result. Prior PR comments record Word evidence on their separate heads. Word 16.113.2 is installed here, but its AppleScript query did not return and computer UI inspection returned `cgWindowNotFound`. The combined branch has no Word accept and reject result yet, so F-X166 cannot complete or be handed off as verified.

## Smells

None found.

## Nitpicks

None found.

## Not found

Correctness, panics, OOXML, tests and structure: no source finding from this pass. The 732 active rdocx regression tests, full rdocx crate tests, 84 Python core tests, scoped Clippy, 49-entry hash harness and offline README doctests passed. The Word differential gate remains the only finding in this pass.
