# F-X209, selective theme slice, pass 6

**Reviewer**: fonts_resume, independent read-only review.
**Reviewed**: immutable selective-theme-source30-capsule, one complete native carrier with selected PPTX/POTX and THMX scheme-application routes, inherited source closure and atomic unsupported-dependency refusal. Fresh master/layout import, unproved image transfer and Python/installed surfaces are excluded.
**Verdict**: 1 defect, 0 smells, 0 nitpicks. This slice is not ready for clean admission.

## Authentication

Receipt SHA `c032246de69131c75950526ff9608bf3b1b628d58f81bfb659bb7f73cb70df6d` authenticates all 13 declared records. Complete diff SHA `ed9356b80036ad97a7cbddf4a487f4d98a6e2010d051b9e756358177788b9522`. Copied before SHA `5926df6530962a30d708bf1541d2e6686e19d54b7c47fc80fcede184ee60e159`, after SHA `ae99f6aa6bb1f3ba3f5d3053a6af47c40722d9a7dfcb304f721e042a3a9c6f56`. Source30 map SHA `4a970bea962ae7450da2a7ff750370157f70aa1cebb75f57791f6e3db3a7b0a6`. The approved F-X209 selective application and preservation contract, canonical workflow and microscope govern this pass.

## Defects

### D1, copied scheme loses inherited markup-compatibility policy

`/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/selective-theme-source30-capsule/lib.rs.after:7973`

`portable_theme_scheme` tracks only namespace declarations from the theme and themeElements ancestors. At lines 7985-7997 it copies the selected scheme and inserts those declarations, but it neither transports nor refuses inherited qualified markup-compatibility policy. `apply_theme` then inserts that scheme beneath the destination's preserved root at line 3682.

A legal source theme can declare `xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"`, `xmlns:x="urn:producer"` and `mc:Ignorable="x"` on its root, with an opaque `x:retained` descendant inside the selected format scheme. The copied subtree retains the x and mc namespace bindings, but loses the inherited Ignorable attribute. The destination root may have no such policy or a different one. Consequently the output no longer carries the source instruction to ignore that retained extension. Related inherited ProcessContent/PreserveElements/PreserveAttributes or MustUnderstand policy also has no transport or refusal path.

The approved design requires the copied full source schemes to retain unknown source facts while preserving the destination root and unrelated content. Namespace closure alone does not preserve this processing meaning. Reconcile the selected subtree's effective qualified policy locally or refuse the unsupported inherited-policy case before publication. Do not overwrite the destination root to repair it. A deciding source-root policy/control must cover the imported scheme, conflicting destination prefix/policy and exact atomic refusal or preserved processing semantics. The current positive control at line 19713 copies an ordinary scheme without inherited MC policy and cannot decide this case.

The implementing worker confirmed no upstream policy transport or rejection exists in the immutable source30 route. Later unfinished master-import work is excluded and does not resolve this finding.

## Smells

None.

## Nitpicks

None.

## Other inspected behavior

The native operation resolves the actual selected PPT source master or one internal package THEME relationship and checks theme content type. Multiple masters require explicit selection. THMX refuses master/import/map operands. All three qualified scheme owners must exist. Edits target distinct destination master theme resources and preserve unrelated destination root and package ranges. Scheme namespace declarations are closed against the actual ancestor scope, including default and alias bindings.

Selected retained relationship references use the existing qualified relationship inventory and exact rewrite provider. Missing references and unsupported external/non-image dependencies refuse against a cloned candidate before publication. The unsafe OLE dependency control compares unchanged whole-package bytes. Image transfer is compiled in the carrier but its positive branch is unproved and outside this pass's acceptance scope. Unfinished import_masters and mapping routes explicitly refuse and remain due, not silently credited.

## Not found

- Correctness: no additional findings beyond D1 in the inspected non-image scheme route.
- Contract: D1 only, concerning inherited source processing semantics.
- Panics: 0 findings in the bounded slice. Relationship indexing is preceded by a short-circuit length guard. Edits are applied in descending original-source range order.
- OOXML: D1 only. Qualified scheme owner and namespace declaration closure otherwise have no findings.
- Tests: no additional findings. The missing inherited-policy deciding case is part of D1. No full PPTX/POTX/THMX or image matrix is inferred from the single positive control.
- Structure: 0 findings. One existing carrier, with no new module, trait, dependency or forwarding framework.

## Actual evidence and limits

Actual GREEN3 receipt SHA `d808c36ac4b2fa035257c28616a40a9419a0820b32cbdebf465e8b604a1c4192` authenticates one exact native test, 1 PASS, 0 ignored. Raw log SHA `6c1ff99b93c6d568f8680eaf4744ef5b3e0fbf1da809745162df636564f521e6`. SourceAfterMatches and ownedLeaseSafeToRelease are true, with natural ESRCH, reaped leader and no signals. Attempts1 and2 exited101 and remain retained rather than receiving runtime credit.

No tests or products were executed by this reviewer. This report does not award fresh master/layout import, positive image transfer, public installed API, render/native Office, lifecycle binding or whole-feature acceptance. Review ends at this report.
