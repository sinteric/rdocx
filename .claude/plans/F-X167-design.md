# F-X167, Accepted-view exporters and readers

**Status**: completed
**Sprint**: S84
**Size**: L
**Depends on**: F-X151, F-X165, F-X166

## Problem

Accepted-view exports differ on deleted rows, paragraphs and content-control
contents. PR 262's PDF path still leaves a nonempty deleted-mark paragraph
separate in an accepted view. Issue 244 requires accepted outputs to agree
after save and reopen.

## Spec reference

- `docs/hld/08-rendering-spec.md`, "Word revision views".
- `docs/hld/10-bindings-spec.md`, "Python API shape".
- `docs/hld/14-development-backlog.md`, "F-X167, Accepted-view exporters and readers".

## Approach

Review PR 259 after PR 256 and integrated content-control work, then its
stacked PRs 262 and 263. Use one accepted-view projection for HTML, Markdown,
MHTML, plain text, JSON and layout. Suppress deleted rows and paragraphs while
retaining content that acceptance keeps. Correct the known PDF paragraph-merge
gap in PR 262. Keep tracked output and untouched XML stable. Check the Python
bindings failure seen on PR 259 on the combined result.

## Rejected alternatives

- Filter text only at each exporter. Structural row and paragraph deletion
  needs one ownership decision before format emission.
- Trust PR 262's current PDF output. Its nonempty deleted-mark paragraph case
  remains an acceptance mismatch.

## Test plan

| Category | Test | Asserts |
|---|---|---|
| regression | Accepted output versus `accept_all()` after save and reopen | HTML, Markdown, MHTML, text, JSON and PDF agree on retained content |
| regression | Nested tables, deleted rows and content controls | Structural visibility and ordering are consistent |
| regression | Nonempty deleted-mark paragraph and final paragraph | PDF combines or removes content as Word acceptance requires |
| binding | Python test suite after PR 259 | The reported binding failure is green and default view is stable |
| harness | Existing and new render corpus | Any changed entry has an explained, separately reviewed delta |

**Test gate**: regression. Every accepted-view output agrees after save and
reopen, and the Python bindings gate that fails on PR 259 is green.

## HLD impact

- `docs/hld/03-architecture.md`, accepted-view ownership across exporters.
- `docs/hld/08-rendering-spec.md`, deletion and layout visibility.

## Risk routing

- Layout: read `docs/hld/08-rendering-spec.md` and compare PDF geometry under
  deterministic fonts.
- Parser and serializer: preserve unmodelled XML through save and reopen.
- Binding surface: run affected Python binding tests and read
  `docs/hld/10-bindings-spec.md`.
- Hidden cross-crate OOXML methods are additive Rust API. Check the published
  crate archives and their size with the scoped packaging dry run.

## Hash harness

The combined corpus remains 49 of 49 after the accepted-view repairs. The
repaired fixtures contain revisions and do not change the current generated
sample corpus. This feature's behavior change is isolated in its labelled
commit with no hash baseline delta.

## Implementation checklist

- [x] Review PR 259, then PRs 262 and 263 against the integrated base.
- [x] Unify accepted visibility and repair the known PDF paragraph case.
- [x] Compare every export after save and reopen against `accept_all()`.
- [x] Pass Python bindings, focused tests, scoped verification and microscope.

## Open questions

None.
