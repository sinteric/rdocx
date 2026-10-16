# F-X253, correctness, pass 2

**Reviewed**: Frozen fourteen-file working diff against 4b1ef1342c600c481d42df9e32de97aed27225b6 after separate remediation. 700 insertions and 20 deletions. File hashes match review-pass-2-inputs.json, SHA2563776939e041402ef662e223f405c75a7cb52da7ca5382878d655a0d5a02f5110. Patch SHA256c55521d7dca31561d340cb4ad6d0d77aa889c7b4ad0ef5121a4e99ced27d4174 matches the actual diff.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None found. Both pass1 defects are resolved.

## Smells

None found.

## Nitpicks

None found.

## Previous findings

D1 is resolved. Selected scatter and radar styles now use their existing finite enum parsers and require their scalar. Unknown lexical values raise errors. Valid unsupported smoothMarker scatter and filled radar remain None. Selected marker symbols use the finite domain before the none distinction. Native controls cover malformed grouping/styles/symbols and paired valid values. Installed controls exercise malformed versus unsupported style behavior without changing source bytes. The retained old-wheel run fails both new style cases, while the remediated focused installed run passes all seven selected cases.

D2 is resolved. A single physical direct-child index replaces global scans inside each series lookup. Marker, symbol, explosion and plot scalar lookups traverse disjoint parent-child lists, with constant repeated visits bounded by the four-visits-per-node counter. The remaining whole-inventory passes occur a constant number of times. The ten-thousand-series control exercises the actual getter and a separate exhausted-budget control proves refusal. The retained real global-rescan mutant charges the same unchanged work budget and fails the many-series test with chart metadata work limit, exit101. Mutant source, failing log and exact restored source hashes independently authenticate. No elapsed-time threshold substitutes for the work bound.

## Not found

- Correctness: no further finding. Duplicate selected children still refuse and foreign/nested lookalikes remain excluded. The exact four required types and seven authored cases retain their original classifications.
- Contract: no finding. Sparse metadata still avoids workbook/cache projection and preserves the physical owner, relationship/content-type checks and read-only authority. No unrequested authoring surface is introduced.
- Panics: no finding. Direct-child indices and source spans derive from the validated scanner inventory. The work counter uses checked subtraction and its initial multiplication saturates. No caller-controlled index bypass is introduced.
- OOXML: no finding. Namespace-qualified direct ownership, source preservation and output write order remain unchanged. Visibility and placeholder routing are unchanged from the independently reviewed pass1 implementation.
- Tests: no further finding. New malformed-enum pairs and actual work-bound mutation address the original gaps. Previously authenticated native/four-route and old-renderer/over-suppression controls remain separate evidence, not replaced by a product-derived expected value.
- Structure: no finding. The child index is local concrete data, and the existing optional-child helper has multiple actual callers. No new module, dependency, trait, generic or public field is added.

## Evidence boundary

The reviewer authenticated the actual four-package publication log hash and Verifying records for oxml-chart, rpptx-layout, rpptx and rpptx-cli, plus all four retained archive hashes. This genuine worker dry run uses all22localpatches and --allow-dirty. Retained --no-verify packaging is explicitly measurement only. Numeric catalog/README reconciliation and its subsequent affected checks remain separate pending work. These are not claims of an integrated clean full gate.

The retained focused installed log reports seven passes. Complete installed-suite and typing/stub qualification were still running when this review was requested and are not declared passed here. Native/clippy/WASM and other worker results remain scoped evidence requiring ordinary orchestration authentication. No new command was run by the reviewer beyond read-only inspection and hashing.

Only this review file was written. All fourteen source/test/HLD files remain frozen. Return the lease to the orchestrator for completion of remaining scoped evidence and any separate numeric amendment. No further correctness confirmation pass is needed without a new relevant change.
