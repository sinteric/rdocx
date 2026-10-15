# F-X197, PowerPoint typing tests, pass 5

**Reviewed**: Root-authored assert_type import and movement of three existing
getter assertions in crates/rpptx-py/tests/typing_smoke.py, on
work/f-x197-codex, HEADb96c7888b7d99bf1ab6c720ea5ba318c7103d611.

**Verdict**: 0 defects, 0 smells, 0 nitpicks in this incremental source review.
No Cargo, binding import, mypy invocation, source fix or formatting occurred.
The full implementation retains independent review2. Word corrections retain
reviews3 and4, independently authenticated at their current reviewed hashes.

## Contract and authentication

Reused the approved197 plan and canonical workflow/microscope reading from
review4. Supported RGB/theme/brightness getters, accepted assignments and
installed strict typing remain mandatory. No product or signature amendment
is introduced by this correction.

Current PPT typing source SHA-256 is
8f65b0a4f036ad00cfeb508c7c0e244721047d4938799916ebcdb7291b9946e2.
Independently reversed the precise import addition and three-line movement
in memory, recovering the exact R2 carrier SHA-256
309ec73016f13589be9b72bdaa9c0e9071c818a49c6a1435933ce845bd1aac94.
All other19 R2 owned files remain unchanged. The two Word test carriers match
the separately reviewed cell and typing hashes. Production, stubs, package
Python, manifests and dependency bytes remain unchanged.

The external ppt-typing-receipt records AST success and root's actual strict
mypy exit0, checked10 files. Root retains the preceding four undefined
assert_type errors and the subsequent None-narrowing error. This review does
not execute or independently relabel those commands. R11's actual155 Word
pass and failed Word typing receipt remain distinct historical evidence.

## Assessment

At crates/rpptx-py/tests/typing_smoke.py:3 the test imports standard typing
assert_type beside the existing typing imports. It previously invoked that
name without defining or importing it. No new dependency or stub is needed.

At typing_smoke.py:556 through :559 the unchanged ColorFormat, RGBColor|None,
str|None and float|None declarations are asserted before setter-based narrowing.
The exact RGB, theme and brightness declarations remain in
crates/rpptx-py/python/rpptx/_rpptx.pyi:512, :518 and :522.

All accepted setter statements remain at typing_smoke.py:560 through :564:
hashed hex, legacy triple, RGB None, supported theme enum and finite brightness.
No assertion is deleted, widened, ignored or replaced. Moving the three
getter assertions tests declared property shape rather than the later narrowed
None value. Existing Word fixtures and their deliberate staleness controls
are unaffected.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Not found

- Correctness: imported assertion names and declared property checks match the stubs.
- Contract: all getter and accepted setter statements are retained.
- Panics: no production path changes.
- OOXML: no serialization or package input changes.
- Tests: no ignore or weakened assertion introduced.
- Structure: existing standard-library import and moved lines in one existing test.

## Remaining qualification

Renew scoped/full preparation with all three reviewed test-only deltas and
original successful R10 build/install equivalence. Do not fabricate a fresh
build or require a wheel rebuild for these test-only changes. Retain original
failures and qualified intermediate results. Remaining actual scoped, stub,
PPT runtime and full suites still decide acceptance. Only this report was
written, and this review ends here.
