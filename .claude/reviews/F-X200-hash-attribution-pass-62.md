# F-X200, hash delta attribution, pass 62

**Reviewed**: Actual49-entry harness failure84f369d05856bf7fabe0f0f5f53c25efda746b80e426d5225a358fef5f3bd304, complete two changed XML parts and source-aware slot closure implementation.

**Verdict**: 0 defects, 0 smells, 0 nitpicks in attribution. Hash acceptance remains pending a declared measured policy and exclusive reviewed baseline update.

## Exact attribution

Root independently inspected canonical generated samples matching both exact existing baseline digests, and current worker samples matching the actual failure digests. feature_showcase document XML changes ada34bba6c97288dce7f60a96b32f2d20c3ea2254f6f82e08c660a39ecfc3b54 to9d3418ef8ef11cbcd64acb40b6f2d54648f985050138181bfb32531c8eecf48f. Exactly three pBdr and four tabs owners gain the canonical Word xmlns:w declaration. report document XML changes bc7d4995931e61ff1fc1c0f004349e64b483428b8c0a5b652a47c46ccb50915e to8df95515887d2b7c7f5604db9c99f5cf2dcbce6a2ea2e202799adf6885c68ec4. Exactly one pBdr owner gains that declaration. Removing exactly those eight added root declarations reconstructs both original part byte streams exactly. Expanded element names, attributes, text, tails and recursive child order compare exactly.

Source attribution is crates/rdocx-oxml/src/paragraph_properties.rs:636, which captures and closes imported slots with actual expanded namespace bindings, and:1859, which re-emits those retained raw bytes. The sample save/reopen path converts generated owners into imported raw slots. The writer has no qualified parent scope. Unconditionally stripping the closure by canonical spelling could break imported or rebound owner contexts. Preserve the source-aware closure policy instead of weakening namespace safety to force hashes. There is no measured numbering delta here. All remaining47 hash entries, including deterministic image and PDF facts, match according to the actual harness comparison.

## Not found

Correctness, contract, panics, OOXML, tests and structure: zero additional findings in this attribution scope. The explicit new declarations are equivalent in these canonical documents and necessary context for independently retained raw owners. No baseline mutation or numerical rendering change occurred during review.

## Handback

Only this report was written during review. Return for explicit measured two-entry declaration in the approved plan, labelled source behavior attribution and exclusive baseline handling. Preserve failed receipts. Actual package, remaining policy, full feature and integration gates remain due.
