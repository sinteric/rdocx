# F-X164, Added shape theme style

**Status**: completed
**Sprint**: S84
**Size**: M
**Depends on**: F-X155, F-X169

## Problem

Newly authored presentation shapes omit the theme style node, so a shape with
no explicit fill or line may render invisibly in external viewers. PR 252
proposes a style after the integrated presentation prerequisites.

## Spec reference

- `docs/hld/06-presentationml-model.md`, "The shape tree" and "Preservation strategy".
- `docs/hld/07-inheritance-and-resolution.md`, "The chains" and "Colour".
- `docs/hld/14-development-backlog.md`, "F-X164, Added shape theme style".

## Approach

Review PR 252's increment against integrated PRs 207, 230 and 234. Emit a
schema-ordered `p:style` for newly created shapes with theme fill and line
references matching python-pptx 1.0.2. Keep imported shapes and their raw XML
untouched. Verify the style does not override explicit fill or line settings.

## Rejected alternatives

- Set a hard-coded fill. It would defeat theme inheritance.
- Rewrite imported shapes. That would risk unmodelled XML.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| differential | Newly authored shape versus python-pptx 1.0.2 | Theme style and schema child order match the accepted structure |
| integration | Reopen in python-pptx and PowerPoint | Shape remains valid and visible |
| rendering | PowerPoint, LibreOffice and rpptx views | Theme fill and line appear, including white text and SmartArt cases |
| regression | Imported shapes and explicit formatting | Existing style and explicit choices remain intact |

**Test gate**: differential. Source-built shapes reopen in python-pptx and
PowerPoint and render visibly in PowerPoint, LibreOffice and rpptx.

## HLD impact

- `docs/hld/06-presentationml-model.md`, newly authored shape serialization.
- `docs/hld/07-inheritance-and-resolution.md`, theme style resolution.
- `docs/hld/08-rendering-spec.md`, any changed default shape appearance.

## Risk routing

- Parser and serializer: preserve unmodelled XML and validate schema child
  order on newly authored shapes.
- Theme and inheritance: read `docs/hld/07-inheritance-and-resolution.md` and
  verify explicit formatting precedence.
- External oracle: read `.claude/skills/differential-testing.md`. Record
  application versions and compare both package XML and rendered appearance.

## Hash harness

No Word hash delta is expected. Recheck affected presentation render manifests
and record any intentional change in a separately labelled commit.

## Implementation checklist

- [x] Review PR 252 against the integrated presentation changes.
- [x] Add ordered style only for newly authored shapes.
- [x] Reopen and render in the required viewers.
- [x] Pass focused tests, risk riders, scoped verification and microscope.

## Open questions

None.
