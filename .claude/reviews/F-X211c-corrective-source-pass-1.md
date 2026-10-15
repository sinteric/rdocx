# F-X211c, corrective PowerPoint source, pass 1

**Reviewed**: Frozen corrective source map `790d194117e9e00d7741aeda0949a8ccbfc59c79c89386c6753c32d170cb69ca`, worker `/private/tmp/rdocx-f-x211c-codex` and its immutable producer source capsule. Independently authenticated all3346 bodies, sizes and modes in both trees. Corrective patch SHA `6ae0ff2b0fad6c78f962c7121cae4b69c6968fa661d48ab1a5367eef8c3bbe04`. The prior reviewed Word/report/CI composition is outside this bounded corrective review.

**Verdict**: 2 defects, 0 smells, 0 nitpicks. This corrective source is not yet ready for a zero-finding gate.

## Defects

### D1. The outer font-diagnostic slice misses nested measurement approximations

`crates/rpptx/src/lib.rs:25031` copies the font manager's diagnostic suffix only after `measure_authoring_text` returns. Measurement uses the shaping cache, whose nested scope begins at `crates/rpptx-render/src/text.rs:134`, records the actual contribution at line139, and finishes at line145. That finish truncates the diagnostic suffix at `crates/oxml-layout/src/font.rs:1309`. Contribution events survive into the enclosing capture, but the approximation diagnostics the new slice expects have already been discarded.

Consequently `crates/rpptx/src/lib.rs:25065` finds no approximation fact and retains the earlier requested/measured message at line25076. Request-level filtering at line24976 cannot restore a missing approximation. The approved reporting obligation remains `.claude/plans/F-X210-design.md:31` through line33 and `.claude/plans/F-X207-design.md:104`.

The exact fresh installed corrective run corroborates the defect. `crates/rpptx-py/tests/test_documented_examples.py:10247` expects one owned Aptos approximation, but receives zero. Its raw retained log is `/private/tmp/S91-F-X211c-PPT-corrective-producer-1/actual-installed-1/current-seven-PPT-residual-and-owner-controls.log`, SHA `62bd5fcd61d583156fecde352c39a636032ae4b94da71d2f32d749522f082db3`. The previous frozen P-AREA-11 failure is not resolved by this implementation. No frozen body or expectation should change.

### D2. The alt-text saved-source assertion selects the slide-tree owner

`crates/rpptx-py/tests/test_documented_examples.py:10177` uses the first descendant `p:cNvPr`. A saved slide has its slide-tree nonvisual properties before the authored textbox's properties. The unscoped search therefore inspects the slide-tree owner rather than the shape whose alt_text was assigned at line10165. The actual corrective run fails this assertion with absent descr, while the same test's preceding live getter assertion succeeds.

The saved-source gate must bind the authored physical shape, retaining its shape_id and actual nonvisual child. This is a defective qualification assertion. It does not establish an alt-text provider failure and cannot be dismissed as a successful saved-file check. The original frozen corpus is separate and must remain unchanged.

## Smells

None.

## Nitpicks

None.

## Bounded areas without findings

Paragraph text replacement at `crates/rpptx-py/src/presentation.rs:3853` retains its physical paragraph and retires its replaced Run descendants. Whole-body replacement still retires paragraph and Run descendants at line3857. Existing checked publication and no-op handling remain in use.

Row.cells at `crates/rpptx-py/src/table.rs:789` validates the held row, resolves its current owner path and captures each actual Cell owner. It does not return detached grid objects or resurrect retired rows. Arrangement aliases at `crates/rpptx-py/src/shape.rs:1827` reject disagreeing arguments before candidate mutation. Foreign selection checks retain presentation and physical parent validation.

Theme.name at `crates/rpptx/src/lib.rs:6217` uses the actual owning theme relationship and qualified root. Existing names patch only the unqualified attribute value. Absent names use the parsed opening-tag boundary with escaping. Native parsing precedes candidate publication and unrelated source bytes remain untouched. The test at line30409 exercises absent/present names, foreign shadow attributes, quotes, ampersands, unrelated parts, no-op and invalid-owner refusal.

Alt fields use the existing checked shape mutation provider at `crates/rpptx-py/src/shape.rs:709`. Highlight aliases use the same physical font owner and checked write provider at `crates/rpptx-py/src/text.rs:2218`. CoreProperties.author at `crates/rpptx-py/src/presentation.rs:691` returns absent state instead of inventing an empty value, with matching stub typing. The shared stale formatter at `crates/oxml-py-support/src/lib.rs:121` adds explicit stale wording without removing the captured/current revision or recovery hint.

No additional correctness, preservation, owner-survival, atomic refusal, dependency or structural finding was found in those bounded changes. The two concrete defects above remain blocking.

## Artifact and acceptance boundary

The corrective wheel SHA is `4363529a053dc0c9c3ee1a0dd292095be8166d262843016bfc6cea35f83913d6`. Root artifact admission SHA is `cb93d9696fa56e49a06fdd1a23aae38d74cb38c635598643a86668e2560df7a0`. The actual installer receipt SHA is `a78eaa4130d9fcc870383c7fd9f71628696c1e3bf9a724549e1dab5ce0336825`, status failed, with five passing and two failing focused controls. Source review does not turn its successful build or five controls into complete provider acceptance.

No source, test, corpus, installed runtime, output artifact, dependency or Git state was changed. No build, installation, test execution or native UI action was performed during this review. Existing actual execution logs were read. Only this canonical report was written under the exclusive review-file lease. Return control to orchestration for separate remediation and a fresh review.
