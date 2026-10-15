# F-X211, native resource correction, pass 3

**Reviewed**: Frozen source32 on `work/f-x211-codex`, HEAD `36cfafaa5999709a4899f74ad4ccef773981a876`. Complete feature diff is eleven carriers, 2,591 additions and twelve deletions. Since source30 only native document.rs changed, adding the anchor-context table predicate and one distinguishing test. This pass is bounded to those corrections.

**Verdict**: Zero defects, zero smells, zero nitpicks within this correction. Prior authored-table and source-wrapper findings are resolved. Full F-X211 acceptance remains pending.

## Resolved findings

The prior authored-table correction remains at `crates/rdocx-oxml/src/document.rs:210`: separate existing32 table depth follows all borrowed stored owner paths before clone or output. Facade and table carriers are byte-identical to source30.

The source predicate at `crates/rdocx-oxml/src/document.rs:561` now requires anchor_depth to be absent before charging a Word tbl as a typed table. Unknown wrappers traversed within a captured anchor retain its active context and consume no table charge. A genuine txbxContent clears that context at `crates/rdocx-oxml/src/document.rs:538`, so actual tables in its typed body still consume the inherited effective table depth. Outer typed depth survives both routes, and sibling restoration remains frame-based. Existing32 and weighted256 stay separate, with no additional mixed quota.

`crates/rdocx-oxml/src/document.rs:4409` proves the native derived anchor parser constructs no typed tables from33 unknown Word tbl wrappers, preserves the exact captured anchor bytes, then proves admitted complete document save/reopen equality. The same control requires a real txbxContent with33 typed tables to refuse. It executes in debug and ordinary2MiB release runs.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Aspects checked

- Correctness: zero findings in the corrected source owner predicate, effective-depth propagation, borrowed32 bound, typed-body transition or sibling scope.
- Contract: zero findings. The change restores actual native ownership and opaque preservation without widening the modeled table allowance or selecting a new mixed-recursion policy.
- Panics: zero findings in this correction. The predicate introduces no allocation, indexing or arithmetic. Previously corrected borrowed over-limit refusal remains before recursive writer or facade clone.
- OOXML: zero findings. Unknown captured anchor wrappers retain exact bytes, real typed body tables remain charged, and existing namespace origin and parser ownership remain unchanged.
- Tests: zero findings. Genuine RED31 first proves native zero-table/raw-byte behavior and then fails the source false refusal. Current32 executes its preservation successor, typed33 refusal, effective source32/33 boundary and existing authored32 lifecycle/33 writer/33-and128 guard controls. Unchanged source30 facade evidence remains separately authenticated.
- Structure: zero findings. One production predicate and one existing unit test carrier change, with no new crate, dependency, module, file, trait, generic, feature flag or larger public consumer stack.

## Authenticated evidence

Source32 map SHA-256 is `3279711e80417def263bf7fcaa280ecdbcec3a6d0f9d4a64695c121b2e613d68`. Complete tracked binary diff SHA-256 is `aaa2338d62fe1a95eb5c50981f1d997d2ccd261f15c1d6f64cb8682cb662bf56`. All3,203 source32 file hashes, sizes and modes authenticate. Only native document.rs differs from source30.

RED31 receipt `127c2da6ee6f40e39cbb971c4126fccdc5510e7eb6eeac574a694079c2fd6f80` retains an actual101 child row, safe cleanup and deciding log SHA-256 `c2ba42791e2a4bb0ec6c99dd5f123b2b9865b8c3582ebfab344fe8334b44a236`. The log proves zero typed tables and exact raw anchor bytes before the source admission failure. Its outer receipt retains status running after the driver propagated the recorded failure. That metadata is not relabelled as successful acceptance.

Current32 receipt `ef9e4bb4a2a9112778775738a328299877c2a69a07cfe4795dec13432446c4f7` authenticates eight qualified terminal zero rows and six exact controls. Owning OXML all-target check and release build pass. Four deciding release controls run on ordinary2MiB test stacks. Actual logs include the33 wrapper preservation proof, source32/33 captures, authored32 clone/save/source parse/reopen/drop and33 writer/33-and128 guard refusal. Every actual log hash matches and all rows have no timeout with owned group gone and leader reaped.

Unchanged source30 receipt `7f2ee52640c1a987fabe8ef8e66fd2ece5a856b04f3a3e52f43d1026007384ae` remains authenticated, including facade package/destination atomic refusal and bounded16 plain,16 control and12 control/box observations. It is predecessor evidence for byte-identical facade/model carriers, not a claimed source32 full replay or arbitrary mixed full-range qualification.

## Remaining boundary

Any separately measured mixed-recursion policy, known-property and relationship kind/cardinality validation, remaining native XML owners, public Word/PPT bindings and stable handles, native PowerPoint XML ownership, final frozen100 installed consumers, HLD and ordinary feature verification remain due. This clean bounded correction does not grant a full-feature handoff.

## Review integrity

Read canonical workflow/review rules, approved resource contract and prior native resource passes. Complete source map, HEAD, branch and binary tracked diff remain exact before and after report. Physical index remains `5a949eb5421f440d730a533607236de39b84e42a012d523c6e0412701ed84532`. Logical index remains `d1ac0aa6c587289eb6fc90884f73d985c585b9a037387546047402f222247b59`.

Only this review file was written. No source fix, runtime, Cargo, Git mutation, Office, CP, plan, intake or shared ledger changes occurred. This pass stops and returns control and the writer lease to the orchestrator.
