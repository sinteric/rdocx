# F-X213, namespace inventory, pass3

**Reviewed**: Narrow actual R4 document.rs correction, ten added lines, against clean working pass1 and compiler pass2. Worker HEAD `3723dea217efdd1d1ee37201ec73b0fd1b5dab20`, branch `work/f-x213-codex`.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication and actual failure

Receipt SHA256 `656a218fb651e1795c3115a0fb5f953f22850c068341a1b026c2c5e09eaa04c3`. Correction diff SHA256 `39c515dbb3f9aad1effc60699ca91d496398c31f554ba6064e4792cb068e9440`,906 bytes. Actual document.rs equals candidate SHA256 `98b87ee503438901f1cbbddd851a3223871e4c8f643b782c453554f246c69051`. Removing exactly the added guard recovers preimage SHA256 `a78151d0d94f66ec486d30b8fcc685491bbfce464373ea81fa8722aa5deac37b`.

Authenticated all2852 current byte/size/mode records against source map SHA256 `e26161f3ff3d12080d612d937e385d90af63191ecf3e054e5f357517239b7e0d`. Compared with R3, only document.rs changed. All2851 other records, including every native and installed test, remain exact.

Read the actual failed opaque-boundary control log. It reports `cannot identify retained p nested namespace owner after mutation` at `crates/rdocx/tests/regression_test.rs:398435`, before the TOC splice. This retained failure is not corrected-source GREEN evidence.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Aspect results

- Correctness: zero findings. `crates/rdocx/src/document.rs:13845` admits the retained-source inventory only when the modeled relationship candidate list is empty and a successful parse of the actual main document part equals the complete current CT_Document. Missing source, parse failure or any modeled difference falls through to the existing path. The comparison includes body, root attributes, extra namespaces and retained background fields through derived PartialEq at `crates/rdocx-oxml/src/document.rs:2867`.
- Contract: zero findings. Empty modeled candidates do not mean empty raw relationships. The exact source scanner inventories and reserves any qualified opaque relationship IDs. With no modeled candidates, there is no modeled value to mask. Both actual callers retain their existing allocator/pruning behavior at `crates/rdocx/src/document.rs:13994` and `crates/rdocx/src/document.rs:24690`. The guard neither mutates package bytes nor publishes document changes.
- Panics: zero findings. The guard adds no unwrap, indexing or arithmetic. Parse failure does not establish equality. Relationship scanner failures remain propagated as errors.
- OOXML: zero findings. `crates/rdocx/src/document.rs:5243` uses the existing namespace-aware scanner on exact retained source. It resolves relationship attributes by expanded URI with actual alias/rebinding scope, preserving first-use order and excluding foreign shadows. This correction does not strip declarations, replay opaque properties, relax owner matching or alter child order. Nonempty modeled candidates and dirty stories retain the original checked masking and nested namespace replay at `crates/rdocx/src/document.rs:13851`.
- Tests: zero findings. All deciding test bytes remain unchanged, including opaque pPr/root attributes, entity/CDATA preservation, repeat save/reopen, paragraph-local aliases and zero-entry reconstruction. The existing failing control discriminates the newly admitted path. Corrected focused execution and dirty/nonempty relationship regression execution remain required, with no result inferred from static review.
- Structure: zero findings. One existing function gains a concrete early-return guard. No new API, type, file, module, dependency, trait, generic or forwarding wrapper.

## Limits and handback

The guard proves typed story equality, not universal equivalence of arbitrary producer XML. It uses retained raw XML only for read-only inventory under that equality and an empty mask set. Serialization, dirty-state owner validation and all other save behavior remain unchanged. This is a bounded incremental review, not full runtime or completion credit. Only this report was written. No source, formatting, Cargo, environment, cache or UI action occurred. Review ends and returns the lease to ROOT.
