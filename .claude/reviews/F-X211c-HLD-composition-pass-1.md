# F-X211c, HLD composition, pass 1

**Reviewed**: Seven canonical working HLD files, 67 additions and 11 deletions, against the approved F-X211c plan and composed Word/PPT source contracts.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Scope and source custody

The seven changed files are exactly the approved impact list at
`.claude/plans/F-X211c-design.md:52`. This pass reviews current-intent prose,
not the completed feature, full corpus metric or native acceptance.

Selected PPT table, shape, presentation, font and context bodies were
independently hashed against the sealed source capsule
`/private/tmp/S91-F-X211c-final-corpus-remediation-2/source.json` with SHA
40b2659c618bb378170c2cd31cc03864df4adf27259d31ab57726e2cda107826.
The current text carrier instead matches the authenticated documentation-only
successor source a72651cb4370bdb5b3b78c2740b6a5c486732e234b97b8c9a3cdc2b223f574e2.
Its declared delta moves the existing auto-number Rustdoc block to the correct
method. No runtime-source equivalence was inferred from a source prefix.

## Not found

- Contract drift in the bounded compatibility exclusions at `docs/hld/02-scope-and-non-goals.md:577`.
- Ownership drift in `docs/hld/03-architecture.md:5`, `docs/hld/04-opc-and-packaging.md:1877` and `docs/hld/10-bindings-spec.md:61`. Current Word inventory and captured hyperlink authority distinguish physical owner state from public-field equality. Manual records retain unique-match resolution. Regular Run invalidation and Story snapshot authority remain conservative. Source comparison includes `crates/rdocx-py/src/document.rs:5186` and the distinct captured/manual branches at lines 5196 and 5247.
- PPT surface drift at `docs/hld/10-bindings-spec.md:1028`. Checked Row cells, paragraph text publication, absent versus empty author, theme-root edits and arrangement reference conflicts agree with their actual owners. Source comparisons include worker `crates/rpptx-py/src/table.rs:789`, `text.rs:852`, `presentation.rs:691`, `slide.rs:1875` and `shape.rs:1827`. Native shape metadata dispatch selects the actual physical leaf at worker `crates/rpptx/src/lib.rs:21295` rather than the first unrelated nonvisual owner.
- Font diagnostic drift at `docs/hld/06-presentationml-model.md:504`. Final rendering starts a usage epoch without clearing loaded font faces. Current diagnostics derive from contributing physical faces, deduplicate requests and suppress a matching physical requested family. Source comparisons include worker `crates/oxml-layout/src/font.rs:1141`, `font.rs:1655`, `font.rs:1682` and `crates/rpptx-render/src/lib.rs:315`.
- Acceptance dilution at `docs/hld/12-testing-strategy.md:4195` and `docs/hld/14-development-backlog.md:7541`. The immutable 100 denominator, original bodies, hard XML/lifetime/native obligations, candidate-bound observations, retained failures and independently admitted named non-successes remain binding. Pending facts leave the full metric unset. The approved c contract retains these requirements at `.claude/plans/F-X211c-design.md:31`. This review does not supply any missing output, runtime or Office fact.

The seven-file diff adds current behavior and qualification contracts without
claiming the forthcoming original100 successor has run or passed. No source,
ledger, runtime, native UI or remote action was performed in this review.
