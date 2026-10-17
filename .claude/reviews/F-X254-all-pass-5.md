# F-X254, all, pass 5

**Reviewed**: focused new contract finding at clean source20e2252101797ec160c9c2f163ed5683caff0dc3. This is not a confirmation pass for pass4's nested fallback repair.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, unrelated embedded mutations acquire a new blocking finite XML policy

`crates/rpptx/src/embedded.rs:529` collects every reopened validation issue, and `crates/rpptx/src/embedded.rs:530` rejects the candidate whenever that collection is nonempty. Both replacement and removal use this helper at `crates/rpptx/src/embedded.rs:146` and `crates/rpptx/src/embedded.rs:163`.

F254 adds InvalidXmlChild diagnostics for preserved imported source. A presentation with an unchanged malformed compatibility declaration or invalid finite child in an unrelated slide can still import and save, but now an otherwise valid OLE replacement or removal fails solely because this new category reaches the old unconditional guard. The earlier full-gate embedded failures exposed this mechanism. Correcting the shared fixture's missing namespace makes that fixture valid but does not preserve the unrelated malformed-input contract.

The approved plan at `.claude/plans/F-X254-design.md:145` retains malformed/opaque import while adding public finite diagnostics. Its checked-replacement transaction boundary is stated at `.claude/plans/F-X254-design.md:163`. The explicit current rule at `docs/hld/06-presentationml-model.md:1589` through `:1593` preserves prior validation categories and reserves the new raw/staged finite admission for checked XML replacement. Unrelated embedded mutation is not that entry point. Existing ownership/signature publication rules and unrelated opaque parts remain protected by `docs/hld/04-opc-and-packaging.md:2045`.

Keep malformed imported diagnostics publicly observable, but preserve the prior embedded mutation validation policy. Do not remove existing blocking graph, ownership, signature or other validation categories. A focused regression should pair an unrelated imported finite defect with a valid embedded operation, require the diagnostic to remain after save/reopen, and retain an existing-category refusal/atomicity control. No global diagnostic suppression or relaxed checked-XML admission is warranted.

## Smells

0.

## Nitpicks

0.

## Review boundary

Inspected the clean current source, exact callers, helper, approved contract and current HLD. No tests, builds, compilers, native applications or product helpers were executed. No source or documentation fixes were made. Only this report was written. Review ends immediately and returns control to root for a separate remediation phase.
