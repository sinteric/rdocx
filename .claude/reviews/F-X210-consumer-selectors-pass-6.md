# F-X210, consumer selectors, pass 6

**Reviewed**: ROOT execution-input correction for the four actual adopted R2 consumer controls. Existing source map SHA256 `7a02e0f5903b4e86380e4dec97e71c47b7b67f21b26db9d921800d444b006e80`. No assertion or product change.

**Verdict**: 0 defects, 0 smells, 0 nitpicks in the corrected execution inputs. The historical prepared selectors are erroneous and remain retained.

## Historical defect and exact correction

`/private/tmp/S91-F-X210-consumer-Base-tests-r2-prepared/commands-prepared.json:15` and44 incorrectly name `document::tests`. The actual enclosing module is `crates/rdocx/src/document.rs:43579`, `odttf_tests`. Lines73 and102 incorrectly name PPT `tests`. The actual enclosing module is `crates/rpptx/src/lib.rs:12361`, `write_tests`. Reviews4 and5 missed this command namespace defect. Their prior clean selector statement is superseded by this correction.

Use these fully qualified selectors with the original package, all-features, lib, exact and nocapture arguments:

| Package | Exact selector | Definition |
| --- | --- | --- |
| rdocx | `document::odttf_tests::fx210_word_public_aptos_diagnostics_follow_actual_owned_contributions` | `crates/rdocx/src/document.rs:43866` |
| rdocx | `document::odttf_tests::fx210_word_public_symbols_and_list_markers_use_only_qualified_semantics` | `crates/rdocx/src/document.rs:43899` |
| rpptx | `write_tests::fx210_ppt_public_aptos_diagnostics_follow_actual_owned_contributions` | `crates/rpptx/src/lib.rs:13580` |
| rpptx | `write_tests::fx210_ppt_public_bullets_use_only_qualified_symbol_semantics` | `crates/rpptx/src/lib.rs:13619` |

Actual `--list` collection must identify each corrected exact name before semantic execution. Each deciding command must run exactly one test, with zero ignored. A zero-test success, compiler failure or unrelated precondition panic earns no Base credit. Keep the original deciding messages and positive physical-font or marker setup unchanged. Actual semantic outcomes remain pending.

## Authentication and retained acceptance

All2909 physical byte, size and mode guards match the adopted map. Both adopted carriers are byte-identical to the independently reviewed R2 complete candidates. No field, fixture, symbol mapping, source preservation, repeat or caller assertion changed. The existing `cfg(test)` modules and PPT render feature guards match the proposed all-features commands. No additional future API reference was introduced by correcting selectors.

The measurement-owned diagnostic projection remains later mandatory GREEN acceptance. This correction does not claim that frame count proves diagnostic propagation, or that positioned text proves independent painting. All review5 coverage limitations remain applicable.

## Aspect results

- Correctness: Zero findings in the corrected selectors. The retained historical namespace defect is identified above.
- Contract: Zero findings. Listing, exact count and positive setup remain required before semantic credit.
- Panics: Zero findings. Source assertion semantics are unchanged.
- OOXML: Zero findings. Source and save/reopen checks remain unchanged.
- Tests: Zero findings in the corrected names. Actual collection and execution are pending.
- Structure: Zero findings. No repository construct is added.

## Defects, smells and nitpicks

Zero remaining in this incremental correction. No source edit, Cargo, environment change or runtime execution occurred. Review lease returned.
