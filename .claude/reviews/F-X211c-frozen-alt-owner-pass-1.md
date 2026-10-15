# F-X211c, frozen alt-text owner, pass 1

**Scope**: Unchanged P-AREA-14 saved-owner assertion and its original failure. No source or specimen remediation.

**Verdict**: The original missing Python facade is a real provider defect. The later saved-source assertion independently selects the wrong owner. No additional alt-text persistence defect was found on the exact corrective candidate. Zero smells and zero nitpicks.

## Original failure remains failed

The frozen body assigns Shape.alt_text at `/private/tmp/S91-F-X211-frozen-corpus-r2/scripts/P-AREA-14.py:11`. The first100 actual traceback at `/private/tmp/S91-F-X211c-final-frozen100-attempt-1/cases/P-AREA-14/stderr.log:2` fails there with AttributeError. The required Shape surface is explicit at `docs/hld/10-bindings-spec.md:1027`. That actual attempt did not reach saved-file assertions. It remains a failed attempt, not retrospectively an owner-selector divergence or a successful correct file.

## Invalid later saved-source assertion

At `/private/tmp/S91-F-X211-frozen-corpus-r2/scripts/P-AREA-14.py:16`, `.find('.//p:cNvPr', NS)` selects the first nonvisual owner anywhere in the slide. Line17 then expects the authored rectangle's description and title. The first owner belongs to the slide shape tree, not that rectangle.

The native contract explicitly writes direct physical shape properties at `crates/rpptx/src/lib.rs:21295` and line21308. Native getters read the same owner at line23402 and line23414. Decorative state uses that owner at line21321 and line23426. The added Python fields delegate through existing checked physical shape access at `crates/rpptx-py/src/shape.rs:709`. Placing rectangle metadata on the slide-tree owner to satisfy the selector would violate the model.

A separately constructed read-only in-memory rectangle diagnostic authenticated corrective native SHA `eaca2c75539e045a23a24521702631c1324216b3cd647d3424db186a4d2b50be` before and after. The authored shape_id was2. Its serialized physical path `p:cSld/p:spTree/p:sp/p:nvSpPr/p:cNvPr` carried id2, descr Description, title Title and the qualified decorative extension with val1. Both the generic first-descendant selector and `p:cSld/p:spTree/p:nvGrpSpPr/p:cNvPr` selected id1 with only name and id, without description or title. Reopening the candidate bytes returned shape2, Description, Title and decorative True. The in-memory serialized package SHA was `297c2bfb2e3e263d31db5c65e5dd77bbbf7e459e3ec40062649926cd2c669724`. No package was written to disk.

Disposition: after the actual missing-facade repair is independently qualified, the unchanged later selector is a named deliberate non-success. It cannot become success credit. Do not edit the frozen body, alter the authored owner, or omit the separate physical-owner persistence gate. Original first100 failure and any later corrective attempt remain separate actual results.

## Acceptance boundary

Original body SHA is `f32718c643c2b449aa8d784658c78435af188979a5b37a1834ac72ec318662a7`. The denominator remains100 and declared divergences receive no success credit under `.claude/plans/F-X211-design.md:104`. The original save/reopen support at `/private/tmp/S91-F-X211-frozen-corpus-r2/support.py:30` writes, checks relationships and reopens a package, but was not reached by the original attempt. Original line18 decorative verification was also unreached. The separate diagnostic demonstrates the bounded corrective owner facts only, not execution of those original gates, native application acceptance or full corpus completion.

## Smells

None.

## Nitpicks

None.

Only this canonical review was written. The root-approved concurrent read-only CP diagnostic used python -I -B, printed observations, created objects only in memory and preserved the actual native file hash. No source, specimen, installed runtime, output file, dependency, Git state or native UI was modified. Review ends and returns control to orchestration.
