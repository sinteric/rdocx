# F-X165, Python and CLI tracked revision view

**Status**: completed
**Sprint**: S84
**Size**: M
**Depends on**: F-X143, F-X150, F-X169

## Problem

Native rendering already offers revision views, but Python and CLI callers
cannot select a tracked view consistently. Issue 253 requires old and new text
to appear according to the selected view in PDF and CLI output.

## Spec reference

- `docs/hld/08-rendering-spec.md`, "Word revision views".
- `docs/hld/10-bindings-spec.md`, "Python API shape" and "CLIs".
- `docs/hld/14-development-backlog.md`, "F-X165, Python and CLI tracked revision view".

## Approach

Review PR 256 after the S77 revision inventory. Expose accepted and tracked
selectors through the four Python render methods and CLI convert and render
commands. Map both to the native view without duplicating revision logic.
Reject unknown selectors with a clear error. Keep default behavior consistent
with the existing native default.

## Rejected alternatives

- Make Python silently accept unknown views. A typo would render the wrong
  document without warning.
- Reimplement revision filtering in each binding. Native layout already owns
  that behavior.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| integration | Issue 253 fixture in CLI PDF and text paths | Accepted and tracked outputs show the requested old and new text |
| binding | All four Python render methods | Keyword selection matches native page count and content |
| error | Invalid Python and CLI selectors | Each fails clearly without producing misleading output |
| regression | Existing default render calls | Their output remains stable |

**Test gate**: integration. Issue 253's old and new text appears in the
selected PDF and CLI output under pinned Poppler, with Python parity.

## HLD impact

- `docs/hld/08-rendering-spec.md`, view selection contract.
- `docs/hld/10-bindings-spec.md`, Python and CLI signatures and errors.

## Risk routing

- Binding surface: read `docs/hld/10-bindings-spec.md` and verify every public
  entry point and invalid selector.
- Layout: read `docs/hld/08-rendering-spec.md` and compare selected page count.
- External oracle: read `.claude/skills/differential-testing.md` and pin
  Poppler for the PDF text comparison.

## Hash harness

No hash delta is expected because the default view stays fixed. Check the
selected-view fixture outside the default harness.

## Implementation checklist

- [x] Review PR 256 increment and the existing native selector.
- [x] Wire Python and CLI selection with explicit errors.
- [x] Run Issue 253 accepted and tracked cases plus defaults.
- [x] Pass focused tests, risk riders, scoped verification and microscope.

## Open questions

None.
