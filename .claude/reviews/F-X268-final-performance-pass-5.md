# F-X268 final performance remediation, pass 5

**Reviewer:** Independent Codex agent `/root/review_source_pieces`.
**Scope:** Canonical working diff against `d92f3d12cca2c6dd84f2b32fc792e8229b510e9e`, including the final mechanical needless-borrow correction. Four files contain 105 added and 21 removed lines. Scope is conservative publication matching, picture inventory batching, comment source-inventory consumption, their regression controls and HLD 03/12. Prior clean reviews supply the unchanged F-X268 implementation assessment.
**Method:** Read-only diff, consumer, contract, assertion and existing execution-evidence inspection. No source implementation, builds, tests or delegation.
**Verdict:** Zero defects, zero smells and zero nitpicks in this bounded batch. Current-source release and linear-gate measurements remain required. The separate native Word bookmark/form discrepancy is outside this diff and is not closed here.

## Source identity

| File | SHA-256 |
|---|---|
| `crates/rdocx/src/document.rs` | `d202ab123ba6a8275ea9ebc2a5532b39432f1a85832bf047ed0194d335ed5896` |
| `crates/rdocx-py/src/document.rs` | `066812f83265b0ad792cfb5a8f34cb7cbd1051e3d45d29fb80fc77e6ee9f8b95` |
| `docs/hld/03-architecture.md` | `8719b34cb171cae81dfcd7b5c795242496b56f5373aa24c5ca4667fdae97dcdd` |
| `docs/hld/12-testing-strategy.md` | `5d059e368a6279cc1a1492c04addbabdf19a41dc6d60eaf86ea01833c1fa4996` |

## Conservative publication

The shortcut applies only to `WordPublication::Conservative`, replacing unused old-route matches with an all-`None` vector (`crates/rdocx-py/src/document.rs:10975`). Candidate inventory assembly and its authoritative fallback still run beforehand. Owner/index validation, result allocation, prepared revision construction and commit remain unchanged.

For this publication kind, selected-owner and form-certificate preservation are absent and snapshot preservation is false. The existing preserve predicate explicitly excludes conservative retention. Therefore the old matcher could not preserve any scope. Supplying no old match retains the same ordered births, complete old-scope retirement and fresh snapshot, while avoiding route-map construction and ancestor walks. Other publication kinds still execute the original matcher.

The amended actual binding test resets and asserts the real route-work counter while retaining its candidate/full-inventory comparison and causal staleness controls (`crates/rdocx-py/src/document.rs:18095`). It verifies preparation cannot publish, changed existing context invokes the full inventory, stable context uses pieces, held owners become stale after conservative commit and fresh owners expose the saved/reopened value. No owner authority check was removed to obtain zero matching work.

## Picture inventory scope and error order

The source's owners are still scanned first. Optional preparation builds their paragraph inventories together and batches inherited scopes (`crates/rdocx/src/document.rs:28252`). Successful results are consumed in the original source, owner and paragraph order. Existing picture deduplication, projection, physical source matching, relationship/media validation and dimensions remain unchanged.

The batched scope is not incorrectly treated as the target's effective scope. At each paragraph, the implementation reads its actual opening event and applies that element's own declarations through `changed_story_namespace_scope` before closing the fragment and determining picture extents (`crates/rdocx/src/document.rs:28284`). This matches `story_namespace_scope_at` for both Start and Empty elements, including local shadows and default declarations. A non-element case retains the authoritative scope lookup.

A batch preparation error is discarded only as an optimization result. It causes the original per-owner inventory and per-paragraph scope traversal to run from the beginning (`crates/rdocx/src/document.rs:28266`). Consequently a later owner's preparation failure cannot replace an earlier paragraph's projection or relationship error. Successful preparation does not reorder paragraph parsing or media checks. No incomplete batch is accepted as an empty inventory.

The new 250/1000-paragraph test counts actual prefix bytes, verifies one picture's media and exact dimensions, and requires at most one source's worth of prefix work (`crates/rdocx/src/document.rs:48744`). It rejects the old repeated-prefix implementation rather than mirroring a new helper. It does not by itself prove public insertion or size-operation timing. The final `&story` to `story` correction only adapts an already borrowed story at the unchanged validation call (`crates/rdocx/src/document.rs:28433`).

## Comment span axis and uniqueness

Comment movement now consumes the raw span, location and namespace-closed paragraph from `story_range_paragraph_sources` (`crates/rdocx/src/document.rs:26068`). The previous `story_range_paragraphs` call already used that same underlying inventory and discarded these coordinates. The marker source and located spans use the same canonical story source, so the change does not mix retained-package and serialized offsets.

The loop still filters the exact story, tests the reference offset against the physical paragraph range before parsing, and checks all matching rows. The original accepted-run reference lookup, exactly-one-reference requirement and duplicate-axis refusal remain (`crates/rdocx/src/document.rs:26075`, `crates/rdocx/src/document.rs:26092`). Both destination endpoints are validated before source editing. No early break, ordinal substitution or weakened reference uniqueness was introduced.

## Intent and evidence

HLD 03 describes conservative retirement and optional source batching without a new cache (`docs/hld/03-architecture.md:67`). HLD 12 states the actual counters and distinguishes them from the required public linear timings (`docs/hld/12-testing-strategy.md:4439`). No public API, storage policy, acceptance bound or user choice changes in this batch.

- `/private/tmp/s954-batched-lookup-picture-red.log`: genuine old-source failure observed 49,196,502 prefix bytes for a 392,562-byte source at 250 paragraphs.
- `/private/tmp/s954-batched-lookup-conservative-red.log`: genuine old-source route-work assertion failure, with vector `[5, 5, 5, 1, 7, 5, 9, 1]` instead of no conservative matching work.
- `/private/tmp/s954-batched-lookup-native-green.log`: ten focused final-gate native tests passed in 15.71 seconds.
- `/private/tmp/s954-batched-lookup-conservative-green.log`: actual native binding full-inventory/staleness control passed in 0.11 seconds.
- `/private/tmp/s954-batched-lookup-affected-native.log`: 40 affected native picture/comment/scoped cases passed in 81.16 seconds. One pre-existing pinned Word comment-thread oracle remains ignored, not passed.
- `/private/tmp/s954-batched-lookup-check.log`: scoped check succeeded in 31.57 seconds.
- Initial clippy found the mechanical needless borrow. After that exact one-line correction, `/private/tmp/s954-batched-lookup-clippy-final.log` records applicable rdocx and rdocx-py clippy success in 37.54 seconds. The initial warning is not represented as an original clean result.

No additional correctness, namespace, panic, resource, structural-smell or nitpick finding was established. Existing six flat-call failures and remaining public linear-gate failures retain their actual failed outcomes until current-source measurement. No current release build or timing pass is inferred here. Memory, incomplete native Word evidence, fixed PowerPoint external resources and all remaining final riders stay pending. No final feature, sprint or release acceptance is claimed.
