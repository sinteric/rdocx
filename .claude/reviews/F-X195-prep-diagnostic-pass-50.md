# F-X195, prep diagnostic actual failure, pass50

**Reviewed**: Actual diagnostic terminal1 and bounded original PDF xref.
**Verdict**: 1 defect,0 smells,0 nitpicks. No causal rendering evidence.

## Defects

### D1, parser assumes a dense ordered PDF object table

`/private/tmp/S91-F-X195-prep-counterfactual-r31-prepared/run.py:63`
requires object0 free-list offset zero. The retained original uses offset3.
Lines64 through76 further require sixteen live entries in ascending physical
order and equate each physical position with its object number. Original
objects3 and4 are free, and live objects are physically ordered differently.
The actual helper exits1 at line63 before font mutation or raster comparison.

Parse authenticated fixed xref rows by actual object number and live/free
state. Locate object boundaries using sorted actual live offsets, preserve
object identities and free-list entries, and rebuild truthful output xref.
Do not remove input authentication or infer causality from this failed run.
Retain the original helper, receipt and failed diagnostic artifacts unchanged.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Not found

No additional finding is claimed in unexecuted font and raster stages.
Actual failed receipt0ccdeb3cd6f68c1f83f0bd6947e84b97e753277926808cfb16c469c18f8d2168
retains matching post-source guards and doNotReleaseLease false. The version
probe group is gone. The separate full104 session66386 remains live and is
not restarted. No production, oracle or threshold change is authorized.

## Review boundary

Only this review was written. No remediation occurred inside the review.
Return to orchestration for a separate corrected external packet and review.
