# Hanji, VML shorthand watermark colours, bounded review

**Reviewed**: working diff against 082edbf6, five tracked files with 116 additions and 5 deletions, plus the approved custom-fix plan
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Effects checked

Correctness and contract: the existing private VML colour decoder validates
ASCII hexadecimal input before branching on exactly three or six digits. Each
short digit is duplicated, with a six-character bounded allocation. Named
colours, whitespace, case handling and the six-digit route retain their prior
normalization. Unsupported forms still return None to the existing stable
watermark diagnostic and suppression path. Word tint and shade are untouched.

Panics and structure: no new indexing, unchecked slicing, public field,
dependency, module or source file enters production. The byte predicate rejects
Unicode before the three-character expansion. Source XML parsing and retention,
VML authoring, story selection, geometry, font selection and backends are
unchanged. The implementation uses the current colour and group models.

Tests: the owned renderer regression failed at the prior code because its
watermark group was absent. The corrected test compares all page metadata and
complete positioned elements against expanded RGB and warm output. Four colour
pairs cover mixed digits, mixed case, black and white. It independently requires
one SAMPLE run, the exact resolved colour, non-missing glyphs, unchanged body
text and empty diagnostics. Invalid lengths, non-hex input and unsupported
colour forms remain rejected. Existing watermark selection and diagnostics
assertions remain intact.

The changed layout crate passes all 319 unit tests and one documentation test.
Strict changed-crate Clippy, formatting, prose and generated adapters pass.
All 49 output hashes remain unchanged. A true offline package verification
build and the layout README example pass, with the refreshed archive measured
at 282,348 compressed bytes, 1,511,843 normalized member bytes and 15 members.
The full repository-policy rerun and exact-head remote CI are separate release
readiness gates. No claim of native Word certification or general VML support
is introduced. Hanji consumption must use an explicit immutable revision and
its own SVG, PNG, HTML and real-document regression evidence.
