# F-X200, decimal preservation test, pass 54

**Reviewed**: One existing test-only correction in crates/rdocx/tests/integration_test.rs, against approved F-X200 imported-source preservation and checked authoring contracts. No production changes.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication

Actual worker HEAD7c9b46a37a5c40ff3d7a21953dec06571b1e5964 and semantic index34a791385771b53d4288429b4b573cdecb5274ec9c9d93db41a4ba86c6f95cc4 remain exact. All2714 current byte, size and mode guards match existing scratch source-after.json SHA5bb9c7015c285988d6d36d86619a4c5aac234a36b79cfd49452666df1197dd0f. Its difference from source-before is exactly the integration test carrier. Actual after carrier SHAfbc5946de4be36693d4202f3acfb38613e56e80c95d9a0abd239f241611cc1d7, size972649. Complete diff SHA151694db3ed179638e08e1cc5aa1987f60bfc705dbb36dd44e90296ea45a19ce. Prefix and suffix outside the existing named test are byte-identical to before.rs. Other2713 records are unchanged.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Six-aspect assessment

- Correctness: 0 findings. Fixture source remains exact at crates/rdocx/tests/integration_test.rs:22320, including imported border size4.0, space1.5 and tab position1440.0. Added typed projections at :22390 through :22402 require4,2 and1440, so lexical retention cannot substitute for actual integer rounding. The existing table/grid, section, paragraph, style, numbering and header rounding assertions remain.
- Contract: 0 findings. Full unchanged part equality remains at :22417. After an unrelated paragraph addition, :22435 explicitly requires the original source-owned border and tab tokens while preserving the prior normalization assertions for the other modeled measurements. This matches accepted source ownership rather than requiring an unrelated edit to canonicalize imported leaves. Native source authority is visible in crates/rdocx-oxml/src/borders.rs:135, with the same typed values parsed at :66 and :68.
- Panics: 0 findings. Added unwraps belong to the positive fixture assertions. Failure does not become a skipped or successful test. No executable production panic path is changed.
- OOXML: 0 findings. Authored bottom border and tab controls at crates/rdocx/tests/integration_test.rs:22453 require integer lexical writes. Exact one-count assertions at :22469 through :22483 distinguish authored integer leaves from retained imported decimal leaves. They fail both accidental normalization of imported leaves and duplicate replay of source-owned elements. Actual package contents and saved/reopened authored projections at :22485 are checked, rather than only in-memory setter output.
- Tests: 0 findings. The existing selector decimal_integer_measurements_round_to_the_nearest_integer is retained. The correction replaces three incompatible normalization expectations with typed-rounding checks, exact imported lexical preservation and explicit new-authoring/reopen assertions. It does not remove rounding coverage or change the fixture. No new test entrypoint, ignored flag, threshold or conditional early return appears.
- Structure: 0 findings. Existing test carrier only, no new API, product file, helper framework or dependency.

## Runtime and handback

No compiler, test, import, rendering or environment action ran. Static review does not credit the pending actual integration suite. ROOT can build its integration binary using cargo test --no-run --message-format=json, authenticate the unique test compiler artifact and then run that executable directly. This preserves the original libtest inventory while avoiding an enclosing Cargo process during the existing public consumer's nested Cargo invocation. Ordinary26.09 and exact F267/F26926.01 partitions and pinned LibreOffice remain required, including truthful ignored counts.

Only this canonical review was written. The review pass ends and returns the writer lease to ROOT.
