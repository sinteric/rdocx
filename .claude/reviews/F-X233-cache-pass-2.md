# F-X233, cached table border continuity, pass 2

**Reviewed**: frozen working diff against12fa27101f153614ac60bff324b0ef2cdd586e29,5existing files,93inserted and5removed lines. One production field, an existing-module regression, approved plan/HLD12 refinement and genuine layout archive numeric/date reconciliation. Pass1 D1 and the approved F-X233 contract read before audit. Engine source SHA256 is85ea85a454640670d0a2551b67e2c56bc59acfcbb48c7c056502eb0f40306815.
**Verdict**: 0 defects,0 smells,0 nitpicks

## Defects

None found. Pass1 D1 is resolved.

## Smells

None found.

## Nitpicks

None found.

## Resolved finding and evidence

Cached table source rebinding now retains border_group_before from the
existing paragraph semantics. Source IDs still rebind to the current physical
location, structure IDs retain their established reset behavior and direction
remains retained. The same recursive path handles nested tables. No other
production statement changes.

The regression constructs actual typed-authoring bordered paragraphs without
raw source carriers that would disable caching. It covers direct and nested
cells, both with and without provenance. Each variant proves initial cache
construction, inserts a paragraph before the table and proves two actual
subsequent table-cache hits without rebuilding. The existing comparison helper
compares full page elements, page geometry, backgrounds, fonts and metadata.
Provenance source tables also match the fresh engine after insertion. Thus the
assertion observes the painted bridge and source rebinding rather than merely
checking the metadata field.

The retained actual RED log fails on unequal page elements with old production,
exit101, at the existing geometry comparison. The corrected frozen test passes.
This reviewer independently executed the existing built test binary and
observed its pass without recompilation. The recorded complete layout run
reports352unit passes and1doctest. Original external pass1 draw-operation
artifacts remain unchanged and are not overwritten by this audit.

The plan and HLD12 describe precisely this cache/provenance regression and
retain existing border units, native bounds, inferred-input limitation and
baseline attribution. Only the layout archive tuple/date changes. Independent
read-only execution of the existing archive helpers validates all22genuine
current archives and reproduces every observation in
`/private/tmp/S93-cache-archive-observation-2.json`. Actual layout is
345245compressed bytes,1894202normalized member bytes and15members, exactly
matching its README/catalog tuple dated2026-10-16. Its packaged engine.rs and
README equal the frozen sources. Other21catalog tuples remain unchanged.
The64byte compression allowance, exact member checks and10MiB ceiling remain
unchanged. No archive was rebuilt during this review.

## Not found

- Correctness and cache interaction: no further defect in the retained grouping/source rebinding boundary after D1.
- Contract and tests: the new regression proves actual reuse and full painted equality for the previously missing typed-authoring path, including nested provenance.
- OOXML and panics: this repair retains one existing boolean during transient layout. It introduces no parser, serializer, indexing or arithmetic change.
- Structure and layering: no new product file, module, public API, trait, generic, feature or dependency.
- Metadata and scope: only approved existing plan/HLD12 and measured layout inventory carriers change. No baseline, other source edit or worker ledger change is present.
- Integrated acceptance: final full Rust/native/installed gates will restart after the remediation commit. Their future results and the formal sprint review are not claimed by this pass.

This audit ends here. Return to the orchestrator for commit and final integrated
verification. No remediation was performed during this pass.
