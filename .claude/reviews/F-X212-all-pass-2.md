# F-X212, all aspects, pass 2

**Reviewed**: Full story Base `66d06bf3e3f4287afb35dea8217e07f98efa7bfd`, extraction `ea803be1e59ba588d4c8da3f1231d84314cd847a`, and the frozen R2 working semantic source. Four existing Rust files contain 2,215 added and 329 deleted lines. The retained complete diff contains 2,918 lines including the approved plan refinement. This pass independently checks the exact R1 to R2 remediation and its interactions with the previously reviewed full contract.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Pass-1 D1 and D2 are resolved at source level. Runtime acceptance remains pending.

## Authentication and execution limits

All seven R2 source guards and the unchanged pass-1 review bytes matched at review start and immediately before this record. The complete diff is 133,828 bytes with SHA-256 `5cc844c9fb1303da35f69109e87cc50f046f432164cc76cf4c1c0ce8e626925e`. The behavior diff is 114,777 bytes with SHA-256 `12c476ccb5caf0b64629353896c049cad6e05c57186a4a7742ca07e207301a5a`. Their receipt is `.claude/scratch/F-X212-semantic-source-review-r2/receipt.json`.

No Cargo, native UI, formatting, source repair, scratch update, baseline movement or commit occurred during this review. R2 remains uncompiled and untested. Retained genuine Base failures, the original invalid drawing precondition and its corrected genuine failure, and the mechanical extraction's actual unchanged49 result retain their separate authority. This review does not turn any of those results into a semantic-prefix pass. Current runtime controls, final hash attribution and reversal proof, complexity measurements and publication gates remain required.

## Remediation assessment

Pass-1 D1 is resolved in the inspected source. `crates/rdocx-oxml/src/text.rs:10768` retains SpecialCharacter payloads except softHyphen. The qualified empty carriage-return branch at `crates/rdocx-oxml/src/text.rs:3378` records the newline needed by the existing cache-consistency filter. Simple and complex cr, noBreakHyphen and valid positional-tab payloads now follow the same retained-result route. The expanded parser matrix at `crates/rdocx-oxml/src/text.rs:17679` asserts their exact typed variants, cache values, source equality, reparse and changed-cache filtering.

Pass-1 D2 is resolved in the inspected source. The same eligibility helper retains DeletedText ownership without rewriting the public extracted cache string. `crates/rdocx/src/document.rs:1919` traverses a consistent typed result before falling back to a current public string, so deleted-only leaf and nested result graphs no longer bypass accepted-view exclusion. `crates/rdocx-oxml/src/text.rs:17745` checks deleted-only and accepted-text-sibling leaf, simple-outer and complex-outer graphs, once-only child ownership, unchanged public cache strings, exact no-op source and cache-consistency mutation behavior. Ordinary text-only projection and the existing explicit current-string fallback remain intact.

Static inventory independently confirms 53 distinct tuple cases in the native matrix at `crates/rdocx/src/document.rs:39042` and 53 in the CLI matrix at `crates/rdocx-cli/tests/integration.rs:2021`. Native cases invoke both public paragraph_visibility and audit, with live model/package equality. CLI cases preserve exact input bytes and other advisories. The parser has eight visibility control functions. These counts describe source coverage, not executed test results.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Aspect results

| Aspect | Result |
|---|---|
| correctness | 0 findings. Special, deleted, accepted sibling and nested results have coherent source branches and distinguishing controls. |
| contract | 0 findings. Current-source preparation, accepted view, scope-specific paragraph owners, effective-cache filtering and truthful failure advisories retain the approved contract. |
| panics | 0 findings in the reviewed changes. Generated envelope assumptions and parser-owned indexing remain guarded by their actual construction paths. |
| ooxml | 0 findings. Producer-prefix qualification, foreign/shadowed exclusions, first expanded control-slot admission and opaque source preservation remain intact. |
| tests | 0 findings. Both remediation gaps have parser and public native/CLI controls. Semantic execution and full acceptance gates remain pending. |
| structure | 0 findings. Existing files, concrete private modes and actual native/CLI consumers remain the implementation boundary. No new module, trait, generic or dependency was introduced. |

Rechecked the remediation against structural child access independent of stale display filtering, refresh and source patch identity, original-child replacement refusal, existing annotation ownership, nested graph clone accounting, direct-body streak resets and scanner owner/path correspondence. No additional source findings were identified. This ends the review pass without remediation or a confirmation pass.
