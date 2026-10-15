# F-X202, paired fixture correction, pass 4

**Reviewed**: immutable test-only correction of pass3 D1 and the two previously admitted legacy expectation migrations. Three regression function bodies only. No production, picture metadata, reference-column or whole-feature review in this pass.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Pass3 D1 is resolved.

## Input authentication

Capsule `/private/tmp/rdocx-f-x202-codex/.claude/scratch/F-X202-start/paired-fixture-correction-capsule-r4/receipt.json` SHA `c6db994a64dfa81c93d6154fee172a858b1d19b90ca02bc81618d5d64cfc65ed`. Manifest SHA `19a847ba50b0b71973132a2f0433e67b252dd6e6e15cfccb3d4a7b6b6ffad2a5`. All 22 artifacts match declared sizes and hashes. The full before/after regression diff contains exactly the six hunks in the three declared function bodies. The copied current source map SHA is `56fd3892445c4f21f9d413930a4263f8d803035b9c0981f918e085f2f70c5776`.

Read canonical microscope rules, approved paired-copy contract and pass3. All inspected test bytes are immutable copies. No source edits or runtime were performed.

## D1 resolution

`/private/tmp/rdocx-f-x202-codex/.claude/scratch/F-X202-start/paired-fixture-correction-capsule-r4/after-regression.rs:401564`

The fallback source needle now includes the actual authored `xml:space="preserve"` and requires exactly one matching source run before mutation. Its replacement genuinely creates two physical runs, with bold only on the retained prefix run. After synchronization, the test finds the actual fallback paragraph while admitting namespace attributes on its qualified `w:p` root, parses it through the existing CT_P fragment provider, and asserts exactly two runs, typed bold Some(true), first text `box ` and second text `PIN`.

The old body-matching `<w:b` predicate is replaced by an exact one-element count plus the typed property assertion. Original logical return1 and two physical `PIN` occurrences remain. Distinct sibling, three-copy, alias, unselected content and reopened logical inventory assertions in the first half of this control remain unchanged. The corrected test genuinely reaches the intended unequal-run and retained-formatting behavior.

## Legacy migration qualification

The two migrations are exactly the previously reviewed success/refusal boundary, with no additional weakened operands or assertions. The comment-bearing positive retains complete non-document package member equality, root/reply comment ownership, logical count1, both physical copies, unrelated body text and reopened selected text. The namespace/owner positive tests each original logical owner independently, canonical and alias MC, three representations, physical copy counts and reopened selected/unselected text. Divergent fallback retains exact complete-byte refusal. The existing unrelated single-copy owner positive remains.

These are an approved transition from blanket refusal to synchronized positive support, not a relaxation of failed asymmetry or opaque ownership tests.

## Actual evidence and limits

Batch20 receipt SHA `75bb5b0229d506293ef439bfafc5271221eb99db23668297d39b5cfa3e874346` authenticates check0 and each legacy migration selector1PASS. Its unequal-run successor failed only while extracting an assumed attribute-free paragraph root. That genuine failure and raw log remain retained. Its picture metadata PASS is outside this pass.

Batch21 receipt SHA `2abe15ef033c3c9960791c3c4411e1d3837fa457f1a519c23b83f85f55d55dab` authenticates the corrected unequal-run exact selector1PASS, unchanged source-after and safe natural cleanup. The source-root extraction correction retains all typed run, bold, text and physical count assertions. Earlier pass3 nondeciding apparent success remains historical and receives no unequal-run credit.

The capsule binds the full source maps but this review examines test corrections only. Other native implementation changes, including separately evolving PictureOptions, are not confirmed by this report. Existing paired production review remains pass3 with its stated scope. No installed/typing, native Word positive save/reopen, rendering, whole feature or full scoped qualification is awarded.

## Findings by aspect

Correctness: zero findings. Contract: zero findings. Panics: zero new findings in these fixture bodies. OOXML: zero findings. Tests: zero findings after the deciding setup and typed checks. Structure: zero findings, using existing dependencies and entrypoint only. Defects: zero. Smells: zero. Nitpicks: zero.

Review ends here. No remediation or full-feature acceptance follows.
