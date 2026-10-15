# F-X197, tests, pass 3

**Reviewed**: One added fixture line in the existing Word colour test on
work/f-x197-codex, HEADb96c7888b7d99bf1ab6c720ea5ba318c7103d611.
The root authored this incremental correction, independently of this reviewer.
The full implementation retains independent all-aspect review2.

**Verdict**: 0 defects, 0 smells, 0 nitpicks in this supplemental test review.
No Cargo, binding import, test execution, source fix or formatting occurred.

## Contract and authentication

Read CLAUDE, WORKFLOW, microscope, the approved197 plan, current progress,
review2 and the HLD strict structural-handle contract. At
 docs/hld/10-bindings-spec.md:171 the first cell text replacement expressly
advances global revision and stales the previously held handle.
The feature remains in progress.

Remediation receipt SHA-256 is
2fac488cfe2c78f16dfb94a9e5f17078678c2ed6a57d5fd62f7d55e81fdb61b2.
Current test source SHA-256 is
c0b817b7cea29ac7c4ce3d5c06f0cf401afdc9c77b57be4696a054f4fb19f2a2.
Independently removed the exact new line in memory and recovered the frozen
R2 test hash651da918ea1bf3b179db067169842f835ca9bdd3b520b6b3a69a489a91e91897.
All other21 R2 owned inputs match their original hashes, including production,
Cargo.lock, support manifest, stubs and Python package source. No production
contract or installed package input changed.

Authenticated actual failed scoped receipt SHA-256
56fb5c988b9acdd143b82c5234cf87c9b4db474460c0f170b2e708aa107dcdd2,
and its exact Word log SHA-256
19e22d368f74072b9ad05f2949ed958591680fd127f642fe86d75f4b401a5fb3.
The log reports155 collected,154 passed and one failed. The actual first
failure is stale cell access immediately after text replacement, before the
colour getter under test is reached. No later scoped rows ran. Original
failure and its two Pillow deprecation warnings remain retained.

## Assessment

At crates/rdocx-py/tests/test_formatting_tables.py:1355 the new line obtains
a fresh document-owned table and cell after the existing cell.text setup.
This follows the real setter at crates/rdocx-py/src/table.rs:1119 through
:1125, which validates the captured cell, replaces text and bumps the global
revision. The correction uses the supported public lookup and does not catch,
ignore or disable a stale-handle error.

The live accepted-run colour view is then obtained at test :1356. All previous
RGB mutation, exact RGBColor equality and ordinary save/reopen assertions
remain. The later document.add_paragraph still invalidates that captured
colour view, and both stale getter and setter assertions retain whole-package
atomicity at :1360 through :1366. Reacquiring the setup cell does not refresh
the colour view after the deliberate invalidating action.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Not found

- Correctness: fresh setup ownership matches the actual revision contract.
- Contract: accepted cell-run colour and later structural staleness remain asserted.
- Panics: no production or unchecked input path changes.
- OOXML: all source preservation and reopened colour assertions remain intact.
- Tests: the original failure is corrected without dropping any colour, stale or atomic assertion.
- Structure: one line in an existing test, with no helper, module, dependency or API.

## Remaining qualification

This clean review is source-only. The corrected test and remaining scoped
runtime, typing, stub and full suites still require actual terminals. Reusing
the fresh wheels requires explicit compiled-input equivalence authentication
and fresh source-guard preparation for this one test change. This pass does
not grant a runtime pass or require an unnecessary production wheel rebuild.
The full feature gates and completion remain pending. Only this review was
written, and the review pass ends here.
