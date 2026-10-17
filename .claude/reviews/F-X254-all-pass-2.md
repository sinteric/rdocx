# F-X254, all, pass 2

**Reviewed**: complete frozen working diff against810af7306e409ad167d512dc370ca4f61df76e4d,10changed files,1555insertions and18deletions. Separate D1 remediation follows immutable pass1.
**Verdict**:0defects,0smells,0nitpicks.

## Defects

0defects. Pass1 D1 is resolved.

## Smells

0smells.

## Nitpicks

0nitpicks.

## Remediation assessment

`crates/rpptx/src/lib.rs:1792` records identities only for supported modeled shape kinds and retains one inherited owner route. `crates/rpptx/src/lib.rs:1811` accepts nonvisual identity only from the qualified correct nonvisual child of that actual enclosing shape. Foreign or opaque shape lookalikes do not create physical owners. Namespace declarations are excluded from attribute identity resolution. IDs must be unqualified, nonzero u32 values. Duplicate nonvisual records make the identity unavailable at `crates/rpptx/src/lib.rs:1855`, instead of borrowing the last record.

The post-scan parent-first interning at `crates/rpptx/src/lib.rs:1954` handles identities encountered after descendant content and binds enclosing group identity as well as leaf kind/id. Missing or ambiguous owner identity propagates the unavailable sentinel. Parent keys are established earlier by preorder, including the explicit part-root key, so the indexed ancestry lookups have an established source invariant. Retention is bounded to supported physical shape records and shared identifiers, with no all-node opaque inventory or copied ancestor vectors.

`crates/rpptx/src/lib.rs:2030` now requires valid equal physical ancestry identities in addition to the prior unique route, raw bytes and namespace/MC meaning. `crates/rpptx/src/lib.rs:2044` applies the same restriction to retained complex compatibility metadata. Whole-part id substitution and swaps can no longer transfer an old opaque exception merely by preserving its ordinal. Typed content changes do not enter the identity key.

The added Python regression `crates/rpptx-py/tests/test_documented_examples.py:11698` covers fresh ID, swapped IDs and enclosing group ID refusals with saved-byte, snapshot and held-owner checks, paired with a same-owner text edit and unrelated namespace-declaration positive. HLD04 accurately states the owner-identity boundary. Existing finite grammar, MC scope, ordinary save report-only behavior, old diagnostic enforcement, bindings/CLI atomicity, public enum impact and final acceptance debt remain consistent with the approved plan. No additional findings arose from the full final diff.

## Evidence and custody

Frozen manifest SHA256 b8adf4e7d0a9e687a22038180f8d15e7aa7667c7dea911ade6fd503e6a6f7d5d. All13entries, including pass1, plan and progress, matched before and after this review. Current binary patch SHA2560de2b2fda580bc53171a086f2f44f7ae1921954589a27dae86d1269fcf62c4e1 matches retained readiness. Readiness SHA256 dce511fdb22370b6d8c662d5616e8bba95d43460609228d346d65c337f098d5f and its five referenced command receipt hashes authenticate.

Retained old-wheel identity JUnit records4tests with3failures and0errors/skips. Current author evidence records successful focused native, clippy, wheel and46case installed qualification with unchanged source. These are inspected author observations, not reviewer reruns. Earlier source epochs and CLI evidence remain explicitly historical/applicable only to their unchanged routes. Final integrated ordinary suites, original reporter accounting, typing/stubs, native and external controls, rendering corpora, docs and publication remain pending. None is claimed passed here.

No test/build/native execution or product/docs/progress mutation occurred. Only this report was written. Review ends and writer lease returns to root. No confirmation pass is required absent new material.
