# F-X166, Picture and final-block comparison revisions

**Status**: completed
**Sprint**: S84
**Size**: L
**Depends on**: F-X145, F-X165

## Problem

Comparison can pair distinct pictures by drawing identity and lose the media
needed to accept or reject. Final paragraph and table changes can put
revision marks on the wrong owner or discard paragraph properties. Issues 254
and 255 require Word-compatible output in both directions.

## Spec reference

- `docs/hld/03-architecture.md`, "Facade conventions".
- `docs/hld/04-opc-and-packaging.md`, "Media" and "Package integrity".
- `docs/hld/08-rendering-spec.md`, "Word revision views".
- `docs/hld/14-development-backlog.md`, "F-X166, Picture and final-block comparison revisions".

## Approach

Review PRs 257, 258, 260 and stacked 261 in order. Align pictures by target
media bytes, preserve both media parts and use revision marks on the correct
paragraph or table owner. Reconcile overlapping comparison and revision code
with F-X145 and F-X165. Reject unsupported paragraph property, mark and
section combinations explicitly instead of writing invalid OOXML. Verify
accept and reject after save and reopen.

## Rejected alternatives

- Pair images only by relationship ID. That identifier has package-local
  meaning and can hide a replacement.
- Serialize a final empty paragraph after every table. It changes the
  acceptance outcome for valid documents.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| differential | Issue 254's three picture cases in both granularities | Word for Mac sees the same replacement and retained media after accept and reject |
| differential | Issue 255's five final-block pairs in both directions | Word opens the package with correct marks, content and paragraph properties |
| regression | pPrChange, paragraph marks and sectPr combinations | Valid forms round-trip and unsupported forms fail clearly |
| package | Save and reopen every comparison result | Relationships, content types and XML child order remain valid |

**Test gate**: differential. Issue 254's three picture cases agree with Word
for Mac, and Issue 255's five final-block pairs open in Word with correct
revision marks, content and properties in both directions.

## HLD impact

- `docs/hld/03-architecture.md`, comparison ownership contract.
- `docs/hld/04-opc-and-packaging.md`, comparison media preservation.
- `docs/hld/08-rendering-spec.md`, final-block revision view behavior.

## Risk routing

- Parser and serializer: preserve unmodelled XML and validate schema order.
- External oracle: read `.claude/skills/differential-testing.md`. Record
  Word for Mac version and exact source packages for every case.

## Hash harness

No existing hash delta is expected. If the new comparison cases affect the
harness, declare the exact entries before recording a separate baseline commit.

## Implementation checklist

- [x] Review and reconcile PRs 257, 258, 260 and 261 in stack order.
- [x] Preserve picture media and final-block revision ownership.
- [x] Verify the complete Issue 254 and 255 matrices in Word for Mac.
- [x] Pass focused tests, risk riders, scoped verification and microscope.

## Open questions

None.
