# F-X215 whole-feature implementation review pass 2

**Reviewed:** Separate D1 remediation frozen on `work/f-x215-codex`, HEAD `38d1697b1d56bf97f7eabc4c02eb6996acf0ae3d`. Complete feature diff is twenty-one carriers, 4,519 additions and26 deletions. Only oxml-sml/lib.rs differs from the reviewed phase. All twenty other reviewed carriers remain exact. This pass re-reviews D1 and authenticated successors, without granting full feature acceptance.

**Verdict:** Zero defects, zero smells, zero nitpicks. Pass1 D1 is resolved.

## Defects

Zero findings.

## Resolved finding

`crates/oxml-sml/src/lib.rs:932` obtains the actual qualified destination row prefix, instead of reusing sheetData's prefix. New cells use that prefix at `crates/oxml-sml/src/lib.rs:958`. The row was already admitted as an actual SpreadsheetML row by namespace resolution at `crates/oxml-sml/src/lib.rs:869`. Therefore its QName prefix, including the empty default-prefix case, is bound to SpreadsheetML in that row's own scope. Inserted cells and their values inherit that scope. Locally rebound ancestor prefixes and defaults do not redirect the new cells.

Empty row expansion preserves the row opening attributes and declarations before appending cells. Cells added to a new row still use the actual sheetData insertion context, rather than an existing differently bound row. The fix changes no style, formula, range ownership, untouched source splice or package publication logic. It introduces no permissive namespace fallback.

The deciding control at `crates/oxml-sml/src/lib.rs:2257` covers five contexts: rebound ancestor prefix, default row with rebound ancestor prefix, prefixed row with rebound default, ordinary alias and ordinary default. It inserts title/numeric cells into both empty and populated destination rows. A foreign cell with the same address cannot satisfy the qualified-cell assertion. Each requested formula target resolves to exactly one qualified cell and an actual qualified typed payload after package save/reopen. Unselected styled formulas, foreign source and unrelated parts/relationships remain exact.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Aspects checked

- Correctness: zero findings in destination-prefix selection, empty-row expansion, existing/new row context or formula-target payload checks.
- Contract: zero findings. The correction restores imported namespace-alias support and consistent editable workbook cells without changing accepted chart kinds or data ownership.
- Panics: zero findings introduced by the correction. The suffix removal follows the admitted qualified row local name. There is no new unchecked input arithmetic or indexing.
- OOXML: zero findings in this correction. Emitted cells inherit their actual row's qualified namespace, and unrelated declarations and source bytes remain preserved.
- Tests: zero findings. The genuine failed-before assertion counts zero qualified cells against one required cell. Native and installed successors assert namespace-resolved values rather than a successful root parse or substring alone.
- Structure: zero findings. One existing concrete carrier changes, with a local destination-prefix value and existing inline control. No new file, module, crate, dependency, trait or generic.

## Actual evidence authenticated

Checkpoint SHA-256 is `75835f88f0a38819c50af9483aebb42ba3c85b07ff47de960a023f53ab1295fc`. Current oxml-sml source SHA-256 is `ad638c522bc9872d49d61f37bd02437a4a0965ea7ebe8633e3c7df8fcdd356cd`. Every checkpoint evidence hash and retained command log matches. Compared retained genuine RED source with the current source: production delta is the destination row-prefix selection. No runtime was executed by this reviewer.

Original RED receipt `dc3707cff9c6f0954596ec8e465a64be0fad88378a45c5b54c9064926334e80b` retains actual101, an unqualified child and the qualified-cell assertion failure. It is not relabelled. Successor green2 receipt `f5848392894200ca6f294b1ee34bb7d253cd15b3c880bc22e8f16a21b27f5b95` records the deciding namespace test passing across all five contexts.

The actual installed imported-workbook successor, receipt `2abecc42373386af7652cd0d1834034710bac1691d67c1a86655da94f13749ec`, invokes the genuine live Chart.replace_data API, adds a series under a row that rebinds the ancestor prefix, saves and reopens. An independent Python XML reader checks all ten qualified formula/cache cells against actual worksheet values. Saved package SHA-256 is `09f7e1c27df34538cbf7d9a6d281fa7a96f29c2370d176e311b1a8ff9b595875`. Untouched styled formulas, original style definitions and unrelated workbook parts remain present. Original installed namespace setup failures remain separately retained and receive no passing credit.

Current seven exact installed nodes pass, as do both strict mypy consumers and both complete stubtests, two real returned-type allocation refusals and five pinned structural oracle rows. Current native PPT controls pass10 and Word relative insertion passes1 on the ordinary stack. oxml-sml library passes12 with one existing native-viewer gate ignored, eight focused chart controls pass, strict changed Clippy, WASM and all49 hash entries pass. Authenticated actual raw logs and source-guard comparisons for these claims. No managed CP or native Office credit is inferred.

Independently verified current wheels, full RECORD digests and lengths, all current Python/stub files and archived native equality with retained built targets. Word wheel SHA-256 is `36b36eb1682060806864096e535662089340fc9dce6acde81437d46c4b1ff9bf`, twelve members. PPT wheel SHA-256 is `6629890fbaf015d6ecf88898f2c992b89db154055c694dc50b86f35960f1c209`, fifteen members. Retained paired installed attribution matches these successors. Existing constructor-stack and earlier fixture/tooling failures remain preserved as recorded.

## Remaining gates

Native Word/PowerPoint opens, Edit Data, normal save/reopen and deterministic value-specific render/stale-value negatives remain due. Root managed paired CP acceptance, four verified archives/current measurements, remaining applicable policy/scoped checks and structured completion handoff remain pending. A zero-finding remediation review does not authorize full feature completion, sprint closure or release publication.

## Review integrity

Read canonical workflow/review rules, approved contract, progress and original pass1. Captured and compared all3,205 tracked file hashes, sizes and modes, HEAD, branch, full binary diff and all index representations before and after review. Sorted complete-source aggregate remains `2c693882184964fc41cfdb5f90b508f3fc62b71306631d0b5ce641fee44ca303`. Binary diff remains `f2dbd68d8b55e46100e2b694b8c051fdb5c901e41ef9e95610ea6208762366b8`. Textual logical index remains `af79b53d8878c28ad5bdf271aded0e77721de17636314579e39b1e59601e8a3a`, NUL-delimited logical index `3e55920e60cde135bb08358e9c9b71be1e035f8e11dfdeb9ad888c635062da75`, physical index `4c543b6dbeaac6addb738ef83a3986b58c2f764b1a504a0006fe395ca896f545`.

Only this report was written. No source, plan, progress, HLD, tests, ledger, runtime, Cargo, Git mutation, CP, UI or Office changes occurred. This review ends and returns control and the writer lease to the orchestrator.
