# F-X211b, qualified preset geometry fixture, pass 1

**Reviewed**: The existing preset geometry regression fixture and actual source15 root DrawingML test result. No source changes are made in this pass.

**Verdict**: One test defect, zero production defects, zero smells, zero nitpicks.

## Defects

### D1, the alias fixture has no actual namespace binding

`crates/oxml-drawing/src/shape_props.rs:494` supplies q:prstGeom, q:avLst and q:gd with no xmlns:q binding and no inherited scope. Its unconditional modeled preset assertion at496 therefore assumes ownership from the local name of an arbitrary unbound prefix. The approved namespace contract requires actual DrawingML ownership, with its separately retained conventional unbound a/p legacy route. The alias fixture must declare q as the official DrawingML main namespace. Its retained opaque-child spellings, preset/adjustment assertions and save/reparse checks must stay meaningful. Preserve the original failed source/log, and add an explicit unbound or foreign alias opacity control if the deciding existing namespace tests do not already cover it.

Root actual DrawingML suite executes179 passing controls, this one failure and2 ignored controls, exit101 with no timeout, gone owned group, reaped leader and no cleanup signals. Log SHA is8da7787b8fe16d5a4754855b8febff2cdf130c0c6699714a1ba6710069d71a7c, receipt is /private/tmp/S91-F-X211b-root-drawing-tests-1/receipt.json. This is an existing test setup/contract defect, not proof that arbitrary unbound q children should become modeled again. Do not broaden production namespace admission to satisfy it.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Review boundary

The unchanged before/after fixture carrier hash is 2930d323e9cc883ab23568040fd0088f3cd28f0cb42e51c79cf5c5bacf3ad00d. This bounded test review makes no whole-feature acceptance, successful successor or native schema completeness claim. All typed/raw child and child-order requirements remain in scope. No source fix, test run, target, managed Python, UI, ledger, baseline, branch or commit action occurs in this review pass. Only this canonical review artifact is written. Review ends and returns control for separate remediation.
