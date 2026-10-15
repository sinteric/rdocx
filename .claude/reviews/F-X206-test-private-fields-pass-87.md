# F-X206, test private fields correction, pass 87

**Reviewed**: R34 two-line existing test correction, diffb5eecf45ef1290b61caae261f889e7ae84f6c8ab4073753398c29a8e2f9f65da against source04b0c853. Receipta6da5b905df911e4fcc2b19cf59a78171dbf002cd28beb7f6326339a0b805d32.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None in the correction. Review86 missed the cross-crate private-field restriction on struct-update syntax. That assumption is superseded. Failed compilation48f4d494 ran no named tests.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness, contract, panics, OOXML, tests and structure produced no findings. Construct the existing type through its public Default implementation, then assign its public default_run_properties field. This avoids private fields without changing any assertion or production behavior. No new dependency or API. Review ends before separate adoption and execution.
