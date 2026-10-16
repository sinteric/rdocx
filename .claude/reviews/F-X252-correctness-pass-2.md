# F-X252, correctness, pass 2

**Reviewed**: Six-file working diff against b8d064f304f09e8451198c31b3d91b62b6e23c80, 1342 insertions and 10 deletions. Bounded independent review after separate remediation of pass 1 D1 and D2, including changed traversal/context helpers, new regressions, HLD amendments and the one installed-test scaffold correction. Approved finite plan remains SHA256 fc46d27703d00857b4e0ec8be2fe844445ad68f2536d6d2efef174c526dac86f.

**Verdict**: 1 defect, 0 smells, 0 nitpicks. D1 and D2 closed. New D3 requires separate remediation.

## Defects

### D3, diagnostic namespace copies retain multiplicative URI storage

`crates/rdocx/src/document.rs:7864`
`crates/rdocx/src/document.rs:8076`

Each island still owns the complete namespace URI in its diagnostic name, despite interning that URI for route and meaning records. A paragraph can declare one long foreign namespace URI once and contain many tiny direct children using that inherited prefix. With an applicable Ignorable grant those are permitted new foreign direct children, so each reaches island collection. All copies remain alive until both admission scans and map comparisons complete.

For a namespace URI of length L and N tiny islands, the incoming fragment needs approximately L plus the short element bytes, while the diagnostic names retain N times L bytes. A one-MiB URI and 10000 self-closing children can require about ten GiB solely for those namespace copies. Existing modeled-depth checks do not constrain namespace URI length or direct opaque occurrence count. This preserves a concrete resource amplification path after the depth-squared route/scope repair.

Store the existing interned namespace identity for this diagnostic and resolve its text only when constructing a refusal, or otherwise bound this retained storage before amplification. Add a safe finite long-URI/many-island storage sensitivity control. No huge input or product runtime was executed by the reviewer. D1 and the original D2 remain closed, but this separate source-confirmed allocation defect blocks completion.

The worker supplied this new self-check concern immediately after the initial report was written and before lease return. Root explicitly authorized this bounded report-only amendment. No source or test was modified.

## Smells

None.

## Nitpicks

None.

## Prior findings

D1 closed. The island scan recognizes the actual MC Choice element and its unqualified Requires attribute. Each bare prefix token contributes its resolved namespace meaning at that declaration element. Local declarations and subsequent restoration use the existing exactly-once namespace decoder. The same retained raw island therefore cannot acquire a different Requires namespace through outside rebinding. The new regression first admits an unrelated scalar edit with unchanged binding, then refuses a changed binding while preserving bytes.

D2 closed. Original and incoming scans share a concrete namespace and route interner. Each route stores one parent ID, interned namespace, local name and sibling ordinal. Active frames retain only a route ID, next ordinal and namespace/grant undo deltas. Namespace entry replaces/moves previous values into its own delta instead of cloning all ancestor maps. Ignorable URI deltas remove only grants first introduced at that scope, preserving inherited grants. Island meaning also uses the same mutable scope with balanced undo, and interns namespace references instead of copying long URI strings into every meaning record. The previous depth-squared retained path/scope state is removed.

Safe depth64 controls count actual retained structural entries and namespace/grant entries. One exercises delegated opaque depth through both admission and checked replacement. The other introduces a distinct namespace/grant at each depth and verifies retained scope state remains bounded by a linear entry count. No new recursive unknown-Word policy, public API or broad parser framework was introduced to impose that resource property.

## Authentication and checks

Successor frozen manifest SHA256 209e225d6624c669ea6a01fceb8558be62f1083a7db13daa7d444e05486e0015 and all six source/test/HLD/plan digests authenticate, with closing checks unchanged. Previous pass 1 freeze also authenticates. Remediation receipt SHA256 192d4d25b7b546cda7ed53e16c909d9cbf6481708f11f3836a3511d11d6a33cb authenticates all five retained log hashes. The actual pre-remediation log records eight passes and two failures, naming both D1/D2 regressions. The actual successor log records eleven passes and zero failures, including the two regressions and the namespace-delta control. These are retained worker executions, not reviewer reruns.

The revised scope machinery preserves declaration-time URI accumulation, blank-list behavior, sibling restoration, local shadow/default clearing and exact-once normalization. Double entry of an island root is paired with separate inner and outer undo scopes, preserving the caller's active context. Error paths mutate only local scan state. Shared route IDs retain original ordered physical correspondence across scans. Direct exceptions cannot transfer into wrappers. Existing exact no-op, selected source/owner checks, finite35 direct grammar, unsupported MC attribute raw-value/meaning comparison and nested typed/drawing/math delegation remain intact. Existing checked XML admission guarantees balanced lexical structure before the new helper's frame expectations. No new production panic or incorrect staging boundary was found.

The Python test now re-fetches the fresh held paragraph after an actual remove before checking stale-owner refusal. It retains the intended stale paragraph operation and byte assertions, without demanding survival of an unrelated retired capture. The prior scaffold failure remains an explicit historical failure and is not reported as current installed success. HLD04 and HLD12 describe the compact resource state and retained Requires meanings. HLD10 and the approved plan remain unchanged from pass 1.

## Boundary

Correctness review only. No source, test, HLD, plan, progress, private proof, cache, build, UI or product runtime was modified or executed by the reviewer. Only this report was written. Fresh installed qualification, typing/stub checks and the remaining scoped riders are still pending their own closure. The retained focused GREEN and this review do not establish final feature or sprint acceptance. Return the sole worktree lease to the orchestrator and stop this review pass. D3 requires a separate remediation phase and independent review of the changed state.
