# F-X195, hash delta attribution, pass 96

**Reviewed**: Actual guarded R68 hash measurement, all49 entries, retained F212 accepted and audit-reversal DOCX/PDF artifacts, current generator and audit source, and canonical versus worker baseline inputs. This is a narrow evidence attribution pass, not a whole-feature review.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in the measured F195 output behavior.

## Authentication and result

Actual receipt SHA256 is6954ceadfc9ae179e3dac0f8d1e2fa9495e4da7ef8468592825496269e746fcf. Actual child exited1, wrapper failed, no timeout, no signals, leader reaped and group ESRCH. The failure remains unchanged. Log SHA256 is725910c039aa9a63724a97cfce33f8cc38cb6c4f6cd4706538448ea3730e380e. All2679 current tracked byte, size and mode guards match f7418cffd1009108e2d41d9c377f7736e586ff612ed46cc07c5d9896edeaf574. Source317 guard remains4c67badc56194d029796b2426a17b1c79293f64f1ecde73c9d28da3758477d4d.

Independently compared all49 actual entries against both the retained F212 measurement actual_entries and the current canonical baseline. Both comparisons are exact equality, with no added, absent or changed keys. The worker baseline at scripts/hash_baseline.json:11, :12 and :14 still holds the pre-F212 values. Canonical /private/tmp/rdocx-s91/scripts/hash_baseline.json:11, :12 and :14 already holds the measured current values. Canonical baseline SHA256 isd316734444e0640ff9c785eb78d7b74a746dbac8671cffb7f209ebd0ea9d4940. Worker baseline SHA256 is9ecae73d5debadd117686a60849088c2f02a16aeb2a62bd936512785930c0bb1.

This is an older worker comparison input, not a new F195 font delta. No new intentional baseline values or reason are justified by this measurement. Root may reconcile the existing accepted canonical baseline through its exclusive baseline ownership. This report does not make that write or convert the failed runtime to a passing check.

## Exact XML attribution

Current word/document.xml is byte-identical to /private/tmp/S91-F-X212-hash-semantic-r5/feature_showcase.docx, SHA256 ada34bba6c97288dce7f60a96b32f2d20c3ea2254f6f82e08c660a39ecfc3b54. Against retained old-audit XML, exactly two effects exist: Accessibility issues4 becomes3 and the complete numbered paragraph Info:4 consecutive empty paragraphs is removed. No font property, other paragraph, source order or package member changes in that retained comparison.

crates/rdocx/examples/generate_all_samples.rs:655 calls the audit, :656 writes its count and :657 writes each issue. Current direct-body visibility and empty-streak logic at crates/rdocx/src/document.rs:29768 and :29780 produces the already approved F212 finding removal. Independent prior attribution and bounded audit-only reversal are recorded in /private/tmp/rdocx-s91/.claude/reviews/F-X212-hash-pass-1.md:47. That reversal restored original49 hashes with an actual child exit0. Its enclosing helper failure after an overly narrow assertion remains qualified separately in the prior report. No new reversal was executed here.

## Exact PDF attribution

Current PDF is byte-identical to the retained accepted F212 PDF, SHA256 b4fe360e1dc26d0f3d848b3d9be0bcba8af613cb417e3d310b8ee17b150f993b. Both retained old-audit and current PDFs have12 pages. All12 MediaBoxes are identical, eleven612x792 and page10 landscape792x612. The first10 inflated content streams are byte-identical. Only physical pages11 and12 change in the old-audit comparison, following the count/list paragraph edit and consequent later content positioning.

Page11 inflated bytes fall31722 to30992, digest a4adb5d02ff36cf10872abc8b8c3f2ced9b817ecb22b79bff8f47a9025c59db4 to4b2498144b286f611df0a291f93cf3fc5db473df15fe4f96a2b167b7c6ad98f1. Page12 falls20301 to19169, digest0ec5a28123bd2adc15c7bd01a330c50dbf54efae9b90a10abfef0fd00f9a5cec toc568cdb9899eb890a7fc8d8b4d53537bb52538fd5d4c31feda224bb4aa4f2dc3. These are not page-count or canvas changes. scripts/hash_harness.py:290 hashes ordered MediaBoxes and inflated page content, which explains the pages-key delta. The complete bytes digest also changes. Resource fingerprint3573cc6e7183b5e912bfde97f11edea4ec44bb40e1b8f066eeb9b4e5e7726d62 remains exact, as does FeatureShowcase page1 PNG and all46 other harness entries.

## Not found and limits

Correctness:0 findings. Contract:0 findings in F195 measured output. Panics:0 findings. OOXML:0 findings. Tests:0 findings in the fail-closed measured comparison. Structure:0 findings. No F195 variable weight, synthesis, selected axes, embedding or geometry regression appears in these49 values. This finite corpus does not replace independent F195 font controls, full acceptance, publication, policy or final integrated gates. No production, baseline, generated artifact, environment or Cargo mutation occurred in this pass.
