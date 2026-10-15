# F-X221 legacy stale-message oracle, pass 1

**Scope**: Read-only diagnosis of the actual clean Python3.9/3.12 priority and compatibility failures against the frozen F-X221 worker. No product audit, corpus replay or test execution.
**Verdict**: One test-oracle defect, zero demonstrated product defects, zero smells and zero nitpicks.

## D1, exact-message helper retains the pre-c formatter

`/private/tmp/rdocx-f-x221-codex/crates/rpptx-py/tests/test_documented_examples.py:253` constructs an exact expected message beginning with `{kind} handle`. The shared provider at `/private/tmp/rdocx-f-x221-codex/crates/oxml-py-support/src/lib.rs:121` deliberately begins with `stale {element_kind} handle`. Its native assertion at line731 uses that same literal prefix. The independently clean `.claude/reviews/F-X211c-shared-stale-wording-pass-1.md:5` authenticates the exact two-string change and unchanged typed fields, revisions, first invalidator, recovery hint and token policy. The current owner/causal contract is retained at `docs/hld/10-bindings-spec.md:60`.

Both original runtime logs identify only this literal leading-word mismatch at the first helper call: `/private/tmp/S91-F-X221-implementation-1/clean39-runtime71.log:34` and `/private/tmp/S91-F-X221-implementation-1/clean312-runtime71.log:34`. Actual kind, captured revision0, current revision1, first cause SlideCollection.add_slide, committed revision1 and recovery path all match. Both runs remain actual70-pass/1-failure candidates, not qualified71-pass runs. This is an outdated exact oracle, not evidence that slide-inventory invalidation is incorrect.

The minimal correction is adding the exact literal `stale ` to the helper's expected first string at line253. Retain exact string equality at line259, the StaleElementError type, every captured/current/committed revision, first-cause spelling, recovery text and all caller selectors. Do not change the product formatter, use a substring assertion or suppress the failed test.

## Other callers affected

Static AST inspection finds exactly four calling test functions in the same existing entrypoint:

- `test_structural_append_retires_slide_inventory_and_preserves_existing_owners`, line262, helper call270.
- `test_stale_shape_text_and_table_paths_report_exact_recovery`, line280, helper call322 across14 concrete owner paths.
- `test_nested_group_paths_report_exact_recovery`, line410, helper call449 across8 nested owner paths.
- `test_fx216_foreign_layout_precedes_stale_collection_without_publication`, line10020, helper call10051, with its existing add/add_at/single/diagnostics/batch parametrization. Its foreign-before-stale ordering and exact whole-package/owner assertions at lines10049-10058 must remain intact.

All use the same shared formatter, so the single expected-prefix correction applies to every listed call without changing their behavioral contracts. The old Word assertion found at `crates/rdocx-py/tests/test_core.py:1016` uses an unanchored regex, not this exact helper, and is not demonstrated broken by the prefix. No unrelated oracle edit is justified here. The later callers were not reached by the reported first failure and receive no inferred runtime credit. Normal remediation should qualify the original failed selector and these existing callers on the current installed release candidate, preserving both original logs.

This review ends here and releases the canonical review-write lease. Only this report was written. No source, tests, expected values, runtime, build, Git, native or remote state was changed.
