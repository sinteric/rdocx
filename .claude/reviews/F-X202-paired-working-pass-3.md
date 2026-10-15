# F-X202, paired working, pass 3

**Reviewed**: bounded immutable paired implementation, existing scoped transaction integration and two proposed legacy test migrations. Reference-column remediation is excluded and remains ROOT pass2. Later picture/link work, installed gates and full feature acceptance are excluded.

**Verdict**: 1 defect, 0 smells, 0 nitpicks. The two legacy expectation migrations are statically admissible, with no successor runtime credit.

## Input authentication

Capsule `/private/tmp/rdocx-f-x202-codex/.claude/scratch/F-X202-start/paired-capsule-r3/receipt.json` SHA `372460be6fef85d0634d49372dcc6d9a83117ed2c01cb4b8854df7b34c029555` authenticated. Manifest SHA `7b37e3a420ea13e801a776d69773f2219ff6fbc5772a134014c2401222b823f7`. All 33 declared copied artifacts match size and SHA. Complete document and regression copies match the two corresponding records in the 3,148-entry batch18 source map SHA `a30400815379aaa11b030804024f821307c66b79d54c18810a64fd6e9599194e`.

Read CLAUDE, WORKFLOW, canonical microscope and the approved F202 paired contract at design lines403-415. Reviewed copied source only. No code, source, index or runtime changes.

## Defects

### D1, the passing unequal-run control never creates unequal runs

`/private/tmp/rdocx-f-x202-codex/.claude/scratch/F-X202-start/paired-capsule-r3/source/crates/rdocx/tests/regression_test.rs:401553`

The fallback replacement needle at 401554 lacks the authored text's `xml:space="preserve"`. The same genuine helper/source is explicitly asserted with that attribute by the descendant deciding control at 401619-401621. No source normalization distinguishes the unequal-run seed. Consequently, this unchecked String replacement leaves the fallback unchanged and the supposedly unequal-run branch still uses the original equal runs.

The additional `output.contains("<w:b")` assertion at 401567 does not repair this gap. `<w:body>` satisfies it without any bold property. Actual selector PASS therefore proves its distinct-sibling/three-copy path, but cannot qualify the approved unequal-runs and retained-formatting behavior.

The control must assert the source needle actually occurs, create demonstrably unequal physical run counts, inspect the qualified typed bold property on the intended fallback run, and retain all existing logical count, both-copy and unrelated-content assertions. This is a fixture defect, not proof that the replacement algorithm fails unequal runs. The worker independently confirmed the exact precondition issue after receiving the citation.

## Implementation assessment

The scanner at copied `document.rs:9010` qualifies actual MC and Word carrier routes and records physical representation spans inside their containing AlternateContent. The projection at 9107 separates signature and structural slots. It retains block/control boundaries, accepted paragraph text and table grid/omission/merge topology while excluding run splitting and formatting from correspondence. Nested modeled text-box/alternate routes are explicitly refused rather than mapped by text alone.

At `document.rs:9309`, paired preparation selects the nearest physical enclosing AlternateContent, validates branch cardinality and corresponding ordered signatures, rejects ambiguous identical siblings and reordered/asymmetric signatures, and resolves the selected descendant slot. Each physical replacement independently runs the existing block-local provider and must match the logical selected count. Non-overlapping original ranges are prepared before descending source splices. The existing scoped transaction retains checked physical-source correspondence, expected mismatch and zero-match returns before pairing admission, candidate-only edits, companion preparation, reopen and single publication.

No additional correctness, contract, panic, OOXML or structure defect was established in this bounded implementation. The private types have concrete local purposes, no new file, module, trait or dependency is introduced by this slice, and ordinary global replacement/story inventory remains outside the new pairing route. These conclusions do not qualify the remaining foreign/rebound, opaque, nested-control, late-reopen and native oracle matrix before those deciding controls execute.

## Legacy expectation proposal admission

Unadopted proposal receipt SHA `2cedc9160148dbfd53308387c9479822e4dccf2eb3d01b9e9cf080230868252f`. All three declared artifacts authenticate. Candidate diff SHA `d0035616362b1f66dbe11c7e926e7d3fa1986504620c56b185e37cdf55179f3d`, candidate SHA `01ee182c4dcb55d9038d37726cbd59000f3c1b13fea7bc6539d973ded177cf42`.

The original blanket-refusal expectations predate the explicitly approved paired positive behavior. The first migration, candidate `regression_test.rs:398226`, now asserts logical count1, two physical edited strings, unchanged non-document package members, complete comment ownership and root/reply count, unrelated body text and reopened selected text. It preserves the original comment-bearing operand.

The second migration at candidate `regression_test.rs:398372` tests each original physical owner independently, ordinary and alias-qualified MC, three representations, complete physical counts and reopened selected/unselected owner text. Divergent fallback still refuses with exact unchanged serialized bytes at 398409-398413. The original unrelated single-copy owner positive remains. This matches the approved success/refusal boundary and does not derive expected text from produced output.

Both migrations are statically admissible. Their original actual failures remain failures and no execution of the proposed successors is credited. D1 concerns the separate new unequal-run control, not an objection to these two migrations.

## Actual evidence and limits

Authenticated batch14 check0 and ordinary paired exact1PASS. Batch16 check0, paired distinct-sibling/three-branch selector1PASS and ambiguity/asymmetry/count-order selector1PASS. Its descendant setup failed and remains retained. Batch17 descendant exact1PASS follows only the source-match fixture correction. Its Clippy failure remains retained. Batch18 Clippy0 follows only the redundant-borrow correction, while both legacy blanket-refusal selectors fail with actual Ok(1), as preserved in the copied logs.

Map16-to17 changes only the regression carrier. Map17-to18 changes only document.rs, whose disclosed difference is the needless borrow. The copied full carriers, extracted helper/integration source and evidence are bound by the manifest. Cleanup/source-after matches are recorded in the actual receipts. D1 limits the unequal-run interpretation of the otherwise genuine passing selector.

No public installed, fresh-wheel, strict typing, native Word paired-copy save/reopen, physical paint, whole feature, full scoped or sprint acceptance is awarded. Further required matrix controls remain due. No reviewer runtime was performed.

## Not found

Correctness: zero additional findings. Contract: zero additional findings in the bounded source. Panics: zero findings. OOXML: zero additional findings. Tests: D1 only. Structure: zero findings. Smells: zero. Nitpicks: zero.

Review ends here. Remediation belongs to a distinct implementation phase.
