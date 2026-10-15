# F-X213, working implementation, pass 6

**Reviewed**: the full working diff against worker HEAD `3723dea217efdd1d1ee37201ec73b0fd1b5dab20`, eight existing carriers, 1680 insertions and six deletions. This includes both production carriers, all 17 native controls, the two installed controls and the four approved HLD changes.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Authority and scope

Read canonical CLAUDE, WORKFLOW and microscope, the approved F-X213 design, its cited TOC and preservation sections, current sprint and backlog, progress and working pass5. All 2852 current tracked byte, size and mode records produce the supplied sorted JSON digest `c25dc66b72bc691abb92d95aa179f558b7183e67ceba84636ff422fa4e7c4e7d`. Only field.rs and regression_test.rs differ from the previous `235729b8feb0a8bc74bc27eeffc0a888819cb5c0762f6ce97e8f996a2120f31a` map. Current hashes are `63415cd20ac2762c6a26c912d02afeea9a0a7c701c0f277be4945460dbced12f` and `5a6c61541d73ff1f998eac10accc33e155e446340df3f0f82c5bed62bb835226`, respectively. HEAD is unchanged.

Historical authorship includes the original bounded implementation and earlier namespace-inventory correction. The current pass5 remediation was separately authored. This pass inspects the whole final source, with that authorship disclosed, and does not claim independent runtime acceptance.

## Defects

### D1, collapsing the last entry still drops opaque property comments outside a tab stop

`crates/rdocx/src/field.rs:9265`, `crates/rdocx/src/field.rs:9270`, `crates/rdocx/src/field.rs:9285`, `crates/rdocx/src/field.rs:9319`, `crates/rdocx/src/field.rs:9643`.

The new body-event guard only handles events at depth four or deeper while the active property child is tabs. A comment or processing instruction directly inside the last entry's pPr therefore leaves `has_opaque_properties` false. With two clean entry roots and one generated entry, `keep_end_root` remains false. The first entry's properties win, the end root is merged away, and no edit copies its original pPr. Its opaque comment disappears on a successful rebuild.

Use the existing two-entry, one-heading fixture from `crates/rdocx/tests/regression_test.rs:12572`, changing only the last entry's properties to:

```xml
<w:pPr><w:pStyle w:val="TOC1"/><!--end producer token--></w:pPr>
```

The comment is not a generated style or tab fact. The approved exact opaque property preservation at `.claude/plans/F-X213-design.md:43` requires retaining it, for example by keeping its protected root in the supported generic topology. The same distinction applies to a processing instruction in that position. The new controls at `crates/rdocx/tests/regression_test.rs:398723` exercise opaque payload attached to tab stops, not opaque pPr body events on a root removed by two-to-one collapse. This is a separate remaining ownership gap, not a failure of the new tab-stop whitelist.

This finding follows the source scanner and splice. No runtime reproduction was executed during this review.

## Pass5 correction

The previous tab-stop defect is resolved in source. `crates/rdocx/src/field.rs:9504` permits only URI-qualified val, leader and pos attributes. Foreign or unknown owned attributes block replacement. The body-event guard at `crates/rdocx/src/field.rs:9643` blocks comment, PI, CDATA, text and entity-reference payload inside a replaced stop. Unsafe multi-entry boundaries retain generic topology, while an unsafe single-entry boundary retains the existing named atomic refusal at `crates/rdocx/src/field.rs:2267`.

The added negative matrix at `crates/rdocx/tests/regression_test.rs:398723` retains exact stop bytes through repetition and reopen, and checks unchanged bytes after single-entry refusal. The positive matrix at `crates/rdocx/tests/regression_test.rs:398780` keeps empty and started ordinary stops eligible. The original 15 controls remain present, with no assertion weakening.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Six aspects

- Correctness: D1. No additional finding in field balance, changed counts, single or zero entries, adjacent fields or repeated namespace closure.
- Contract: D1 violates opaque paragraph-property preservation. No scope expansion or additional contract finding.
- Panics: 0 findings. Source offsets, generated entry slices, scanner depth and property-root ranges were inspected with their existing construction and validation paths.
- OOXML: D1. No additional namespace ownership, lexical payload or schema-order finding.
- Tests: 0 separate findings. The new matrices discriminate the repaired tab-stop defect and normal overlay path. D1 requires a distinct removed-end-owner control.
- Structure: 0 findings. Existing concrete carriers and private helpers contain the repair, with no new public API, dependency, trait, generic, module or product file.

## Limits

No Cargo, product imports, installed tests, formatter, target, environment or native UI action ran. The live ROOT verification and all remaining installed, deterministic hash, WASM, package and scoped completion gates receive no credit from this review. Earlier genuine Base failures and failed compiler and namespace attempts remain historical evidence. Only this review file was written. The review phase ends here.
