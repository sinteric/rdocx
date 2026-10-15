# F-X201, page color focused review, pass 5

**Reviewed**: Six existing implementation/test/stub carriers within the authenticated 16-carrier page-color capsule. Source map SHA-256 35ed646d843d977ebfd881b0286dfdac1e08ba5c06cd6bcec750e4b6fded4246 and complete working diff SHA-256 70508601a1acbab8167c27f16f100a1d9bf34d82352bc5c8b22078803db6f418. This bounded pass covers the new page-color writer and direct binding/stub, not all feature additions in that diff.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness, contract, panic/error handling, OOXML preservation, tests and structure have zero findings in this bounded path. The facade at `crates/rdocx/src/document.rs:19502` stages a relationship-resolved settings candidate, updates the background and display toggle, reopens and publishes once. The direct binding at `crates/rdocx-py/src/document.rs:4305` preserves that staged publication. The existing stub declares the same string-to-None signature at `crates/rdocx-py/python/rdocx/_rdocx.pyi:695`.

The background writer at `crates/rdocx-oxml/src/document.rs:2888` validates six hexadecimal characters and retains the source outside the selected color attribute value. The shared qualified attribute splice at `crates/rdocx-oxml/src/settings.rs:2694` rejects duplicate expanded attributes. The display toggle at `crates/rdocx-oxml/src/settings.rs:555` rejects duplicate qualified owner children and uses the existing ordered writer. Namespace closure and fixed-prefix admission retain alias correctness. No new product carrier, trait or generic is introduced for this path.

## Evidence and limits

The immutable capsule at `/private/tmp/S91-F-X201-page-color-review` was authenticated against all carrier hashes, source map and diff. Actual receipt at `/private/tmp/rdocx-f-x201-codex/.claude/scratch/F-X201-start/actual-page-color-check-2/receipt.json` records a successful native/binding all-target check and one exact source-preservation test, with zero failed or ignored tests. Raw logs were independently authenticated, source-after guards remain exact and owned groups are naturally gone and reaped. The test at `crates/rdocx/tests/regression_test.rs:399513` proves retained qualified aliases, unknown attributes and children, comment/CDATA content, invalid-input package equality and repeated saved bytes. No tests, imports or source changes ran during this audit. Current installed page-color typing, native paint, DrawingML watermarks, page borders and whole-feature acceptance remain pending and outside this pass.
