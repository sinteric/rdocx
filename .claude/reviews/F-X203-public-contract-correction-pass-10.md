# F-X203, public contract correction, pass 10

**Reviewed**: Immutable public constructor and factory stub correction plus separate stories-property fixture operands. Three existing carriers, seven added and seven removed lines. Native implementation changes are absent from this bounded correction.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Evidence and scope

Capsule `/private/tmp/rdocx-f-x203-codex/.claude/scratch/F-X203-start/immutable-public-contract-correction-10/receipt.json`, SHA `0808ce38fe801a6c7678ff633be5c3034ee8ffa18631786ecc869e1d0367e04f`. All fourteen declared artifacts authenticate by SHA and size. Each incremental diff reproduces its before and after carrier exactly. Stub diff `a01e98f32d6e2f942d1d1f2fab2dfc2923534b360b0cbd1196a5ec00ff34cfa5`, fixture diff `e87539cd352275d962eeaf8439e9e77dfec4214fbbd12b651b574bb82a95e436`, typing diff `9fc64e82ec0aca405a42473af0e785d3fb5ac90bd4f782887df1f1402f363070`.

The binding document carrier read for signature comparison is byte-identical to its entry in the previously authenticated 3111-record runtime source map `3ba2f8927d86c1792985cf1ceb42cd61b6974d9cb7d881c87c7b57d2df7250ff`. This is source qualification, not a review of unrelated later implementation.

## Contract checks

Copied `after-_rdocx.pyi:1648` declares CustomFileTime.__new__(cls, value: str) returning CustomFileTime. Binding `crates/rdocx-py/src/document.rs:2642` supplies the existing PyO3 new constructor with the same value operand. Copied stub `after-_rdocx.pyi:1744` similarly maps ContentLocation.__new__ to binding `document.rs:3397`, including story, item_kind and index_path. Neither runtime constructor acquires new behavior or parameters.

Copied stub `after-_rdocx.pyi:1733` and `:1756` use the established impossible private keyword constructor convention rather than a Never-returning ordinary constructor. This preserves class identity for static factory methods while ordinary direct construction remains uncallable in typed code. Existing runtime FragmentConflictPolicy factories at `document.rs:3343` and DocumentFragment.from_range at `:3434` remain unchanged. The same convention already exists for ContentFragment in the stub. No public constructor is introduced for these factory-only classes.

Copied `after-test_core.py:5090` and `:5097`, and `after-typing_smoke.py:438`, access stories as the tuple property supplied by the existing getter at `document.rs:5967`. The original complete snapshot, boundary, imported-content, failure atomicity and reopened-content assertions remain byte-identical. These changes repair setup operands rather than relaxing acceptance predicates.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: zero findings in the corrected declarations and property operands.
- Contract: zero findings. Public factories remain visible and private construction stays unavailable.
- Panics: zero findings. This patch adds no native operations or unchecked arithmetic.
- OOXML: zero findings. Native XML parsing, publication and preservation paths are unchanged.
- Tests: zero findings. Existing deciding assertions remain intact after the tuple-property setup repair.
- Structure: zero findings. No new module, file, dependency, trait, generic or forwarding wrapper.

## Actual failures and runtime limits

Copied runtime receipt `718168e68b33137f7f1813c9d06da306a6b6d0942788afb52ef0dc97bd243807` is byte-identical to the original actual-public-Word-installed-1 receipt. All ten original row logs authenticate against their receipt hashes. The original collection selected 29 nodes. Actual execution was 28 passed and 1 failed because the tuple property was called. Strict mypy failed with three errors, comprising the two Never-constructor class identities and the tuple call. Stubtest failed with eight constructor signature errors. These failures remain failed historical results.

The original provenance and RECORD rows exited 0. Retained installed and after maps are equal, PPT before and installed maps are equal, and RECORD after-install and after-test maps are equal. All recorded owned groups released safely, with source-after equality and safe lease release true. This verifies the retained failed attempt's source and installation closure, without claiming a fresh installation check in this review.

The corrected successor has not run installed controls, mypy or stubtest. This is a static admission review of the bounded correction, not a passing successor gate or whole-feature acceptance. No source or runtime changes were made by the reviewer.
