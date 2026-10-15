# F-X211c, actual approximation failure, pass 1

**Scope**: Actual corrective source9dc60b0b installed diagnostic failure, approved owned-font contract and concrete authoring measurement path. Source-only pass2 remains a historical static review, not installed acceptance.

**Verdict**: 1 defect, 0 smells, 0 nitpicks. The shared authoring diagnostic epoch hides a current owner's approximation behind earlier sizing history.

## Defect

### D1. Earlier sizing facts suppress the final owned diagnostic suffix

`crates/rpptx/src/lib.rs:2859` creates one FontManager for refresh. Earlier size selection at line2879 uses it before final owned diagnostics at line2915. `select_autofit_pair` performs concrete measurements at line25112. Those measurements record accepted text contributions outside the inner shaping-cache transaction, including Latin text at `crates/rpptx-render/src/text.rs:860` and multilingual text at line977. They can therefore leave an Aptos approximation in the shared manager's current diagnostic list before the final owner capture.

`crates/rpptx/src/lib.rs:25015` snapshots the existing diagnostic length. The successful-event replay at line25048 uses the same list. The provider's exact request-prefix deduplication at `crates/oxml-layout/src/font.rs:1699` through line1705 suppresses a new message when the earlier sizing fact already exists. The suffix copied at `crates/rpptx/src/lib.rs:25049` is consequently empty. The actual owned result does not publish the earlier fact, because the suffix starts after it. The fallback requested/measured messages do not satisfy the explicit approximation contract at `.claude/plans/F-X210-design.md:31` through line33 or the shared measurement/render obligation at `.claude/plans/F-X207-design.md:104`.

This is an actual reporting defect, not a reason to weaken the test or invent successful-file credit. The corrective run reports six passing controls and one failing control. `crates/rpptx-py/tests/test_documented_examples.py:10252` receives zero Aptos approximation facts. The exact retained log SHA is `691e0d4dc74025dc8bd46cab58d52708147857a83300b45524dacfef1270a6cc`, installer receipt `afaa6d42667b37ca30fa13d5f3da157f2436971bdf6b36aa3851c06f237ecdbb`, and terminal complete-pair receipt `aa095f3298380bd2985f13477b20f4ef142a2b246ff6151da0b84d107c8e5b5a`, all under `/private/tmp/S91-F-X211c-PPT-corrective-producer-2/actual-installed-1`.

The alternate explanation that nested scopes leave only Resolution events is insufficient for this control. Scope finish does remove nested Contribution events at `crates/oxml-layout/src/font.rs:1316`, but accepted text records its final contribution after the cache scope at `crates/rpptx-render/src/text.rs:860`. Those final events can reach the enclosing successful capture. Diagnostic history, not absence of all accepted contributions, explains the current suffix failure. No runtime instrumentation was performed in this review, so the report does not claim an independently printed internal trace.

## Remediation boundary and risks

The existing `FontManager.begin_layout` at `crates/oxml-layout/src/font.rs:1141` clears only paragraph trace, usage inventory and approximation diagnostics. It does not reload physical faces or alter selection precedence. Starting a fresh diagnostic bookkeeping epoch before each final context capture is consistent with current-owner facts, provided it occurs before any active contribution scope and retains the successful bounded capture/replay and error checks. Do not clear an active caller trace or change loaded faces, resolved instances, glyph bytes or selection caches to manufacture a message.

Each owner still needs its independently owned facts. Deck-level stable first-use deduplication at `crates/rpptx/src/lib.rs:2994` and line3024 remains separate from the per-context bookkeeping epoch. Authentic requested faces, probe-only/empty text, repeated refresh/fit-report equality, failed capture and source-byte atomicity need actual successor qualification. This review authorizes no implementation or test rewrite.

## Smells

None.

## Nitpicks

None.

## Acceptance boundary

The relevant source carriers were reauthenticated against frozen map `9dc60b0b97227eb1c11b6fb525ded94e4c9a447bf2034dc336802d5bf103691c`. The earlier zero-finding static pass did not execute installed controls. Its verdict is retained, and this distinct actual failure supplies the newly established defect. The corrected alt-owner control passes in the actual run, but the remaining approximation failure keeps the installed gate failed.

No source, test, specimen, runtime, CP environment, output artifact, dependency, Git state or native UI was modified. No runtime diagnostic, build, test execution or installation occurred. Existing actual logs and concrete source were read. Only this canonical review was written under the exclusive lease. Review ends and returns control for separate remediation.
