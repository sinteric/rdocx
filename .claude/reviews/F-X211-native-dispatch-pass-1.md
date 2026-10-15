# F-X211 native causal dispatch review pass 1

**Reviewed:** Frozen source34 and terminal successor checkpoint36 on `work/f-x211-codex`, HEAD `36cfafaa5999709a4899f74ad4ccef773981a876`. Complete feature diff contains eleven carriers, 2,720 additions and 81 deletions. Relative to measured source33 only `crates/rdocx-oxml/src/content_control.rs` changes. This review is bounded to causal parser-frame reduction, not complete F-X211 acceptance.

**Verdict:** Zero defects, zero smells, zero nitpicks in this correction.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Correction audited

`crates/rdocx-oxml/src/content_control.rs:1599` establishes an explicit non-inlining boundary for concrete child construction. The helper at `crates/rdocx-oxml/src/content_control.rs:1600` constructs actual paragraph, table, row, cell and run models, appends them to the owning content vector and records original inline-run source. It is not a forwarding wrapper. Its purpose is to keep these by-value model temporaries out of every recursively nested content-control dispatch frame. Model parsers may themselves recurse through their existing owners, without placing this helper on each direct nested-SDT edge.

Start-child dispatch at `crates/rdocx-oxml/src/content_control.rs:1699` still requires the actual qualified Word child and its permitted SdtOwner before helper admission. Wrong-owner modeled names remain captured verbatim. Empty-child dispatch retains the same predicate at `crates/rdocx-oxml/src/content_control.rs:1737`. The helper's local-name dispatch is therefore reached only after qualified owner admission. Inherited namespace bindings and local overrides are supplied to the same native child parsers as before. Empty children retain their prior construction rules.

Run source handling at `crates/rdocx-oxml/src/content_control.rs:1646` still captures the full raw start/end subtree or empty element, parses with the same inherited bindings, records its original clone and uses the pre-append content index. Thus unchanged run serialization and subsequent edits retain the original snapshot semantics and source ordering. Table, row and cell parsers keep their existing reader-consumption and error propagation paths.

Direct nested SDT handling at `crates/rdocx-oxml/src/content_control.rs:1711` remains in the recursive parser, with the same owner and qualified prefix context. Failed typed projections retain captured raw XML. Unknown and revision children keep their original slots at `crates/rdocx-oxml/src/content_control.rs:1719`. End, EOF and reader errors remain unchanged at `crates/rdocx-oxml/src/content_control.rs:1757`. No resource limit, source quota, typed owner policy or public stack size changes.

Earlier native-resource findings remain resolved in byte-identical carriers: authored tables use the separate existing32 bound before clone/output, while actual unknown anchor wrappers remain uncharged and preserved. Weighted256 accounting still assigns one to a control and sixteen to a typed text box. This correction adds no mixed quota.

## Actual causal and compatibility evidence

Authenticated checkpoint36 SHA-256 `4bee55d99d14295d0576ca2808fdf8e521bcbe603677f557adcef0d3f2d6b3bc` and all3,203 current tracked-source entries. Source33-to34 comparison identifies exactly the single correction carrier. Every retained receipt hash and actual log hash matches.

The genuine source33 B224 failure remains preserved in receipt `4c3a3101caf39ed2419d385e7a2a4200275c44268eeedcccf8e444a5a351a412`. Its actual child exits101 after signal6 SIGABRT with an ordinary-stack parsing overflow, before typed-count output. The outer finite measurement completed while recording that failed child, not a passing B224. Source33 stopped its B-family after that failure. No failed receipt is relabelled.

The isolated source34 B224 successor, receipt `13bc28232816fb4aa87ba263ba538b5c0eac9b42d353dc1d2114f16d19889275`, records three qualified zero rows: all-target native checking, owning release binary build and the exact deciding lifecycle. The actual log proves32 typed tables and224 controls, then complete guarded clone, save, reopen and drop. The inspected driver explicitly sets RUST_MIN_STACK to2097152 for the deciding child.

Affected controls35, receipt `a45205536e096d6a69780104e206963b44a307c2cbd6564955772594a4e64121`, records twelve qualified zero rows and twenty exact controls. These include the nine actual placement/source/inline tests, retained cached-field ownership, opaque inline-child behavior, namespace origin and scope, weighted source accounting, authored/source table boundaries, A15, C32 tables with16 boxes and pure256 controls. Current boundary36, receipt `221f1a934ca5811d28c9efbb85d21013be3e3ebc8bca36889ebf7ee3ee15844b`, records four qualified zero rows:32 tables with240,248 and256 controls, and32 tables with128 controls and8 boxes. All deciding children use the ordinary2MiB stack. Combined34/35/36 records nineteen qualified zero rows and twenty-five exact controls.

The lifecycle control at `crates/rdocx-oxml/src/document.rs:3972` builds the requested owner nesting and iteratively asserts exact typed counts. At `crates/rdocx-oxml/src/document.rs:4115` it validates resources, clones, saves, reparses, checks serialization equality and drops every recursive model. Successful output therefore proves actual native owners and complete lifecycle, not an opaque fallback or preflight-only success. All qualified receipts retain no timeout, gone owned process groups and reaped leaders. This reviewer executed no native runtime.

## Not found

- Correctness: zero findings in qualified dispatch, concrete child construction, owner order, source indices, raw fallback or reader/error handling.
- Contract: zero findings. The correction addresses the measured parser overflow without changing existing32 table or weighted256 control/text-box limits.
- Panics: zero findings introduced by the helper. It adds no unchecked indexing, arithmetic or unwrap on untrusted data. The deciding formerly aborting path now completes on the same ordinary stack.
- OOXML: zero findings. Native child parsers, inherited/local bindings, exact raw source, revisions and source-order slots remain consistent with the previous route.
- Tests: zero findings in the actual causal successor and affected controls. True failed-before evidence remains separate, and actual typed counts plus full lifecycle distinguish the correction.
- Structure: zero findings. One concrete same-file helper performs construction and source bookkeeping. No new crate, module, file, dependency, trait, generic, public type or larger consumer stack is introduced.

## Remaining scope

Complete selected XML owner cardinality and relationship validation, Word/PPT public binding owners, frozen100 compatibility implementation and qualification, CLI/Python resource refusal, final risk/HLD/verification and structured handoff remain due. These finite native observations do not grant complete F-X211 acceptance or infer an arbitrary new mixed policy.

## Review integrity

Read canonical workflow/review rules, approved plan and progress, current in-progress ledger rows and prior three native-resource passes. Compared all3,203 source hashes, sizes and modes, HEAD, branch, physical/logical index and complete binary diff before and after report. Source map remains `53b9215db292f5c6d1c0b09cdcfa5787eddc4c0fc58b7015a86fa4cbf8968a43`. Binary diff remains `d8903c5119db525e836b79c35c809d37961060f2607c6f75a3ac39cd36e07aea`. Logical index remains `d1ac0aa6c587289eb6fc90884f73d985c585b9a037387546047402f222247b59`, physical index `5a949eb5421f440d730a533607236de39b84e42a012d523c6e0412701ed84532`.

Only this report was written. No source, progress, plan, ledger, runtime, Cargo, Git mutation, Office or CP changes were made. Review ends here and returns control and the writer lease to the orchestrator.
