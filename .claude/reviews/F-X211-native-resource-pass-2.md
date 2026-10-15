# F-X211, native resource correction, pass 2

**Reviewed**: Frozen source30 on `work/f-x211-codex`, HEAD `36cfafaa5999709a4899f74ad4ccef773981a876`. Complete feature diff is eleven carriers, 2,526 additions and twelve deletions. Since source26 only native document.rs, table.rs and facade document.rs changed. This pass audits the table correction, not full public-feature acceptance.

**Verdict**: One defect, zero smells, zero nitpicks. Pass 1 authored-table D1 is resolved. A new source-side false charge remains.

## Resolved prior finding

`crates/rdocx-oxml/src/document.rs:210` checks the shared existing recognized-table limit32 and increments a separate borrowed table depth. Every owner edge carries it, including control originals, stored field caches and instructions, revisions, ruby runs, and authoritative and legacy anchor text. Siblings inherit their parent's depth independently. Table depth remains separate from weighted256 control/text-box cost, with no arbitrary mixed quota.

`crates/rdocx-oxml/src/document.rs:559` carries effective source depth through admitted controls and captured typed text bodies rather than resetting it at local recursive parser entries. The existing table constant at `crates/rdocx-oxml/src/table.rs:29` is shared crate-internally without changing its value. Authored33 and128 refuse before serialization, authored32 completes actual clone/save/source parse/reopen/drop, the33 writer buffer stays unchanged, and facade refusal preserves package bytes and the existing destination.

## Defects

### D1, table-named anchor wrappers are charged as typed tables

`crates/rdocx-oxml/src/document.rs:560`

The new table charge requires admitted, Word namespace and local name tbl, but does not exclude the active anchor traversal context. In that context, admission deliberately traverses unknown wrappers at `crates/rdocx-oxml/src/document.rs:515`, retaining anchor_depth until a real txbxContent starts at `crates/rdocx-oxml/src/document.rs:538`. The native anchor parser similarly continues through unknown starts at `crates/rdocx-oxml/src/drawing.rs:1177`. It constructs a recursive Word body only at its txbxContent branch at `crates/rdocx-oxml/src/drawing.rs:1152`.

Consequently,33 nested qualified w:tbl wrappers inside an actual wp:anchor, outside its txbxContent body, consume33 table charges and refuse at the existing32 limit even though the parser constructs no CT_Tbl for those wrappers. A real small txbxContent nested inside those wrappers still has zero typed tables attributable to the wrappers. Their captured anchor source should remain preserved. This is a false refusal of unmodelled wrappers, not a reason to relax the existing table limit.

Count a source table only when it represents the actual typed owner route. Preserve effective depth from outer modeled tables through the anchor and into its real typed body. Add a distinguishing native source control proving the wrappers do not construct tables and survive unchanged, with an actual txbxContent containing33 modeled tables still refusing. Control accounting already distinguishes anchor traversal from typed owners at `crates/rdocx-oxml/src/document.rs:541`. No successor correction or runtime proof for this new finding is credited in this pass.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Aspects checked

- Correctness: new D1 identifies the source false charge. Zero additional findings in the borrowed32 limit, separate weighted256 arithmetic, path propagation through all stored owners, sibling reset or checked existing boundary behavior.
- Contract: D1 violates actual modeled-owner accounting and opaque preservation. Zero additional findings in the prior authored D1 correction. Existing32 is reused, pure control/text-box policy is unchanged, and no additional mixed quota is selected.
- Panics: zero further findings. Borrowed over-limit tables refuse before recursive clone or writer output. The increment follows the32 comparison, so it cannot overflow. Source frame inheritance and closure do not use an unchecked decrement of a separate table counter.
- OOXML: D1 counts nonmodeled Word-looking anchor wrappers. Zero additional findings in namespace origin, direct anchor semantics, empty modeled tbl counting, inherited bindings, current opaque/rebound routes or schema ownership. This pass does not expand native parsing to make source accounting fit.
- Tests: D1 lacks its distinguishing opaque-wrapper control. Zero additional findings in genuine guard-only33/128 RED, current authored32 lifecycle, actual source32/33 admission through control and typed-body captures, writer/facade atomic refusals and previously qualified bounded mixed-owner successor observations.
- Structure: zero findings. The correction stays in three existing carriers and exposes only the existing constant crate-internally. No new crate, dependency, module, file, trait, generic, feature flag, ambient authority or public stack enlargement is introduced.

## Authenticated evidence

Source30 map SHA-256 is `c4e046f4167d2359467b66b53a81a044a7fae2164cba594d8d29e0f35754f360`. Complete tracked binary diff SHA-256 is `8983339b1d4ef21325c6462d988ca8062025c79d1d90e5630b322df33355c711`. All source26 entries outside the three correction carriers remain exact.

RED28 receipt `acfb978ed14f8e54f907d500215d76d829b22be8614c96bbb9b2a2ab6dec616e` retains a completed actual101 child row and safe cleanup. Its actual log proves authored33 and128 returned Ok before the correction, then the distinguishing assertion failed. The control dismantles ownership iteratively and does not run unsafe over-limit serialization or recursive clone. The outer receipt retains status running because the driver propagates that failure after recording the child. It is not relabelled as a successful completed receipt.

Correction29 receipt `f00d25e7be889e955270746d67c2a81b4ebc6a93946501f98c86acf97f8d179d` authenticates four terminal zero rows: all-target native/facade check and three focused controls. Its exact source29 map and every actual log hash authenticate. Earlier debug results remain predecessor evidence rather than ordinary-stack completion.

Current normal-stack30 receipt `7f2ee52640c1a987fabe8ef8e66fd2ece5a856b04f3a3e52f43d1026007384ae` was supplied only after its owned command reached terminal zero. It authenticates eight qualified zero rows and seven exact controls on ordinary2MiB release test stacks. Actual authored32 and reopened typed-table counts, clone/save/source parse/reopen/drop,33/128 guards, source boundaries,33 writer-buffer refusal and facade atomic refusal pass. Existing safe observations retain exact16 plain tables,16 tables/controls and12 tables/controls/boxes. Zero-control output from the other crate in dual-crate selectors is kept separate from the one actually executed control. Every actual log hash matches, every row has no timeout and a gone owned group/reaped leader, and source30 remains exact. These results do not prove a new combined mixed limit or resolve the new D1.

Whitespace diff check is clean. This review authenticated retained execution and did not run Cargo or the product.

## Remaining boundary

New D1 remediation and independent re-review, any separately measured mixed-recursion policy, known-property and relationship kind/cardinality validation, remaining native XML owners, Word/PPT bindings and stable handles, native PowerPoint XML ownership, final frozen100 installed consumers, HLD and feature verification remain due. This pass grants no full-feature handoff.

## Review integrity

Read canonical workflow/review rules, CLAUDE.md, the approved resource contract, prior native pass1 and its current progress context. All3,203 source30 hashes, sizes and modes match before and after review. HEAD, branch and complete tracked diff remain exact. Physical index remains `5a949eb5421f440d730a533607236de39b84e42a012d523c6e0412701ed84532`. Logical index remains `d1ac0aa6c587289eb6fc90884f73d985c585b9a037387546047402f222247b59`.

Only this review file was written. No source fix, runtime, Cargo, Git mutation, Office, CP, plan, intake or shared ledger changes occurred. Review stops here and returns control and the writer lease for separate remediation.
