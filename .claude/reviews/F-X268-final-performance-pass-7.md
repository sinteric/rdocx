# F-X268 final performance remediation, pass 7

**Reviewer:** Independent Codex reviewer `/root/review_source_pieces`.
**Scope:** Frozen working diff against `65e71f6e5df563519ba0028f4350e35eaf4f35b3`, four files, 128 insertions and 18 deletions. Reviewed the complete address-carry batch, D1 remediation and new failure controls. Pass 6 supplies the unchanged carry analysis.
**Method:** Read-only diff, consumer, invariant, assertion and recorded evidence inspection. No source edits, tests, builds or delegation.
**Verdict:** Zero defects, zero smells and zero nitpicks in this bounded batch. Pass 6 D1 is closed. Release timing and final feature acceptance remain pending.

## D1 closure

The original `stories()` enumeration and body selection now occur before the new direct-slot branch (`crates/rdocx/src/document.rs:33183`). Missing, malformed and duplicate related-story errors therefore retain the original validation and failure order for both direct and controlled paragraphs. This is the complete enumeration, not merely source collection. The controlled two-segment path remains unchanged. The direct branch still filters the actual checked result to paragraph kind (`crates/rdocx/src/document.rs:33190`).

The new regression has an ordinary paragraph and a typed controlled paragraph, with a valid header positive. It verifies the controlled paragraph really has a two-segment path. Missing and malformed header variants compare each paragraph lookup's exact error with `stories()`. A duplicate footnote relationship variant verifies the explicit duplicate-reference error and repeats both lookups (`crates/rdocx/src/document.rs:51349`). These controls discriminate the previously bypassed validation and preserve a valid direct and controlled case.

`/private/tmp/s954-direct-paragraph-related-errors-red.log` records the genuine previous-source failure returning an address instead of the missing-header error. `/private/tmp/s954-direct-paragraph-related-errors-final.log` records the final extended discriminator passed, one case in 0.06 seconds. It is completed execution, not a pending compile or an inferred pass.

## Entire batch assessment

The carry repair remains unchanged from pass 6. It qualifies the current serialized partition, requires exact head and tail, invalidates a changed partition's body fingerprint and retains neighboring read facts (`crates/rdocx/src/document.rs:23702`). `main_story_layout` still consumes only exact aligned child bytes with sufficient item coverage and composes current physical offsets and owner ordinals (`crates/rdocx/src/document.rs:25523`). Unaligned children, unsupported shapes and changed frames take their existing scan or fallback paths. Read facts do not become writer permission. The final installation retains the newly qualified serialization memo separately from those address facts.

No new flaw was found in reordered or duplicate child alignment, namespace context, field/tab/bookmark bytes, lock lifetime, error propagation or current memo preservation. The existing changed-before-reopen test still compares the full reader and requires exactly one fresh child scan among 50 and 200 children through both reopen routes (`crates/rdocx/src/document.rs:51578`). The original compile failure remains distinct from the actual old-source count RED.

The plan and HLD 03/12 accurately describe exact-frame carry and consume-time child qualification. They do not claim constant whole-call time or release acceptance from local scan counts.

## Evidence and remaining gates

- `/private/tmp/s954-address-carry-native.log`: seven main-body layout cases passed in 20.92 seconds on the unchanged carry implementation.
- `/private/tmp/s954-address-carry-scoped-native.log`: seven affected scoped cases passed in 147.12 seconds before the D1-only validation relocation.
- `/private/tmp/s954-direct-paragraph-related-errors-final.log`: the final direct and controlled error-order control passed in 0.06 seconds.

Current source SHA-256 for `crates/rdocx/src/document.rs`: `d3642ec6fc89c5420ca91aeabbf6ce43aec703f1e6980fa230296fb9670eda5e`.

Current scoped clippy and hash checks are not claimed by this report. The previous release measurements remain failures until affected measurements on the corrected release pass. Installed Python, A9 counted work, the 79 flat gates, five linear gates, complete catalogue, memory qualification, native Word and final sprint riders remain open where not separately discharged. This report grants no whole-feature, release or sprint acceptance.

## Not found

Correctness: zero findings. Contract and failure order: zero remaining findings. Panics and resource checks: zero findings. OOXML namespace, physical ownership and opaque preservation: zero findings. Structure: zero findings. Tests and documentation: zero findings in this bounded scope. Smells: zero. Nitpicks: zero.
