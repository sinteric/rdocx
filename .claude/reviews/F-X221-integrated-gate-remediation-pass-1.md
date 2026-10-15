# F-X221 integrated gate remediation, pass 1

Scope: Working changes to the 16 source and test carriers on canonical HEAD `c48624f58122d4c22026434acc1496accb061342`. This is an independent static review of the integrated gate repairs against F-X221 and the approved F-X200, F-X211, F-X216 and F-X218 provider contracts. Source was frozen throughout this pass. No source fixes or test executions were performed by this reviewer.

Verdict: Two fix-now findings. No fresh full gate, current wheel, installed suite or native appearance admission is implied by this review.

## D1. Current relationship IDs are accepted without proving their retained XML owner

`crates/oxml-opc/src/relationship.rs:319` now validates explicit map keys against current `self.items`, while `crates/oxml-opc/src/relationship.rs:252` still takes the historical retained XML and `crates/oxml-opc/src/relationship.rs:274` applies the map to each historical raw record ID. These are different identity sets after an existing public item ID has been edited.

For a parsed row `Id="kept"` containing an opaque attribute and child, change its public item ID to `draft`, propose that item as `imported`, and supply `draft -> imported`. The new validation accepts that map. The retained row is still looked up as `kept`, finds no proposed item and is deleted at `crates/oxml-opc/src/relationship.rs:275`. The proposed item is subsequently emitted as a new bare row at `crates/oxml-opc/src/relationship.rs:333`, losing the original opaque attribute and child. Previously this particular current-ID map was refused. Conversely, an explicit `kept -> imported` map that identifies the retained record is now rejected at line324 because `kept` is absent from current items.

The added test at `crates/oxml-opc/src/relationship.rs:1447` covers a newly appended row with no retained record, but does not cover a renamed retained row. Accept current IDs for genuinely new rows without treating that as proof of a retained row's identity. Retain the explicit retained-record mapping route where ownership is proved, and refuse ambiguous ownership before returning a destructive rewrite. Do not infer opaque ownership from matching positions or targets. Add deciding renamed-retained-row controls covering opaque attributes, children and unchanged input alongside the appended-row case. This follows the lossless opaque relationship contract in `.claude/plans/F-X216-design.md:33` and `.claude/plans/F-X216-design.md:39`.

## D2. A committed review fails the mandatory prose gate

`.claude/reviews/F-X221-wasm-manifest-oracle-pass-1.md:8` contains a prose semicolon between the retained failure log citation and the statement about the failed run. Root's actual prose gate reports this location. Replace that separator with a sentence boundary while retaining the failed-run qualification. The repository prose rule applies to tracked Markdown, including reviews.

## Remaining categories

Zero additional correctness or panic findings. Zero additional OOXML namespace, child-order or atomicity findings. Zero additional public API, lifecycle, default-stack storage or inheritance findings. Zero additional test-strength or structure findings. The parser storage changes retain public parser signatures and use private boxes. The revised old oracles retain actual retired-child, package, qualified-attribute and source-template checks.

The original full-suite failures and stack predecessors remain evidence. Root reports the focused ordinary-default-stack notes successor and strengthened local relationship guards as passing, but those scoped results do not replace the pending final integrated gates or fresh binding qualification. This pass ends with this report and returns control for distinct remediation.
