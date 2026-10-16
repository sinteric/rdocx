# F-X230, all aspects, pass 2

**Reviewed**: Frozen post-D1 working diff against599e318de614f3c0b2e9fba0a4bcac6a455695a9,8 files,1099 insertions and16 deletions. Reviewed the actual D1 RED, raw segmentation and emission repair, expanded boundary controls and updated evidence. The worker withdrew its freeze after the new targeted test failed.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D2, New Run parity control assumes a previously unsupported input succeeds

`crates/rdocx/tests/regression_test.rs:32552`

The new inline-control boundary test unconditionally unwraps default comparison
before exercising Word and Character behavior. Its actual successor fails here
with the existing content-control owner-correlation refusal for the PI fixture.
Default and explicit Run behavior must remain unchanged, including existing
atomic refusals. The test cannot require new Run support as a prerequisite for
the granular repair. Compare both outcomes and saved bytes on successful cases,
and compare the named error plus unchanged input bytes on existing refusals.
Then exercise all Word and Character cases independently. Do not widen Run
admission to make this test pass.

## Smells

None.

## Nitpicks

None.

## Not found

D1 remediation: the caller now carries actual raw run boundaries into
shell_unit_cuts and align_between_shells. Grouping consumes their segments,
and the source emitter copies a segment's leading shell before a pure
insertion. This addresses the static source defect. The genuine pre-fix RED
at /private/tmp/F-X230-inline-raw-red.log reports the expected edited-story
reconstruction failure. The current targeted successor stopped at D2 before
the new granular matrix ran, so no successful runtime remediation is credited.

Other correctness, contract, panics, OOXML, structure and measurement aspects
retain the first pass's review boundaries. No additional product finding was
identified in the D1 repair. Its actual post-fix gates remain required.

## Review boundary

Only this review file was written. The pass ends with the withdrawn freeze,
returning the worker lease for the narrow test correction, actual green
successor and a fresh independent review. No source or test was changed by
the reviewer and no pending gate is reported as passed.
