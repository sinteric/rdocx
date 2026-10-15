# F-X207, authoring origin, pass 40

**Reviewed**: External one-carrier correction diffd3bfde1ad7722036d3f899f6f5a625068e59828871c312c372033d982509ba25, receipt22c12fb628801a3dfe83e90937f7fa46b2ba8227eb074d0323fc6a9e11dd23b4 against frozen worker source94aaa32b34c5fd1dfdb681c071a13970dbba0c488cd444836acd579fc44b22ee.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

Actual worker `/private/tmp/rdocx-f-x207-codex/crates/rpptx-render/src/lib.rs:890` derives the same oriented content box, first-owner direction slice and shared stacker as candidate measure_authoring_text at :461. The candidate changes only the explicit authoring measurement origin, preserving dimensions, original public usable rectangle, selected fonts, page fields and direction slice. Physical extent and fitting use that same local measurement box. Ordinary rendering and checked EMU truncation are unchanged. The source-built test at candidate :2052 checks physical extent invariance and original public usable geometry across different vertical frame origins.

Actual diagnostic log64859ce96a1e47f7d37b94bcce2a86a4cab33768e91a8520be8d810aae37cf9d establishes the vertical Shape repeat width1727199 to1727200 and centered x431800 to431799. No tolerance, expected bounds or original matrix predicate is changed. This review permits guarded adoption and the new unit plus unchanged failed matrix selector. It grants no runtime or native paint acceptance.
