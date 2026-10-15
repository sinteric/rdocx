# F-X213, proposal, pass 2

**Reviewed**: Immutable R2 external proposal against claim Base `3723dea217efdd1d1ee37201ec73b0fd1b5dab20`, three existing carriers, 1632 complete patch lines. Complete patch 75355 bytes, SHA256 `2c213fbf94e9c5845666af073800dcf7467d550ee711157f25f076536353f7b1`. R1-to-R2 incremental patch 12098 bytes, SHA256 `cdd2059d0378ce704fb15f6d8f0f6a90ce7bdc2026093b1f14c1bce35872d9e3`. Receipt SHA256 `1102a2e476cfb80ca6862a0bcec626c675e967d5881f3ba33bd85f8c11d57d3d`.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

Authenticated all receipt-listed artifact hashes and sizes, all three candidate identities, exact R1 predecessor identities and original Base Git source hashes. Python test carrier remains R1 byte-identical. Read the approved design, cited HLD contracts, PR341 assessment, prior pass and full candidate changes. Citations below use repository paths and candidate line numbers in `/private/tmp/S91-F-X213-implementation-proposal-r2/candidates`.

## Prior findings

- D1 resolved in source. `crates/rdocx/src/field.rs:9329` now closes each retained end run and suffix root using the original paragraph namespace map before single-owner expansion publishes the generated last paragraph. It retains the separate two-owner-to-one closure. The new `toc_single_entry_expansion_closes_paragraph_local_namespaces` discriminator covers locally declared Word aliases and default namespaces, a foreign binding shadowing an outer binding, exact opaque payload/entity spellings, end-marker URI, original identity once and repeated save/reopen.
- D2 resolved in source. `crates/rdocx/src/field.rs:9230` now preserves the balanced distinct-root zero-entry topology while overlaying complete style removal on each eligible boundary. It retains original paragraph roots and unrelated properties. The corrected contributor zero-entry test requires unstyled boundaries. `toc_empty_entry_boundaries_remove_styles_and_allow_later_positive_rebuild` covers one and three old entries, started styles, opaque properties, repeated zero saves/reopens and a later positive rebuild. The latter uses the saved source package as its input and retains the protected marker roots rather than deleting their coverage.

These are static source resolutions, not observed GREEN results.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Aspect results

- Correctness: 0 findings. Reviewed source offsets, distinct and single boundary topology, one-to-many and many-to-one transitions, zero-to-positive behavior, sorted nonoverlapping staged edits, nested field markers and current page-substitution path.
- Contract: 0 findings. The bounded TOC repair retains the approved generated-entry policy, original instruction ownership, marker-only behavior, protected-root fallback and atomic unsupported single-owner refusal. It does not adopt bibliography or expand full navigation.
- Panics: 0 findings. New indexing and slices were inspected against their actual internal generated-source and scanner call context. Runtime panic absence is not claimed.
- OOXML: 0 findings. Reviewed expanded namespace ownership, inherited and local closure, complete started-style removal, foreign shadows, duplicate-property refusal, pPr schema ordering, retained root attributes, opaque style/tab particles and end suffix preservation.
- Tests: 0 findings. Both original Base discriminators remain meaningful. The remediation tests address the prior missing namespace and zero-entry policies without dropping marker/count, opaque source or reopen checks. Fifteen native and two installed definitions are source inventory only, not actual collection or PASS.
- Structure: 0 findings. No new public API, dependency, product file, module, trait or generic. Helpers perform actual source scans, overlays and closure operations in the existing carrier.

## Evidence boundary

No Cargo, native application, installed product, formatter or other runtime was executed in this review. ROOT genuine Base runtime was still pending when assigned. Production remains an external proposal. Syntax and apply-check receipts do not establish compilation. Working microscope, focused corrected native/installed controls, deterministic layout, hash49, applicable risk gates, HLD and final integrated verification remain required. Later F-X200/F-X206 composition must preserve their independently owned publication and provider regions.

Only this review artifact was written. Review pass ended and the review-only lease returned.
