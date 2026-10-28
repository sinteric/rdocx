# F-X266, all, pass 1

**Reviewed**: Frozen working diff on `work/f-x266-grok`, Base and HEAD `64a15b2c08393353cc77e03e3761e5f77e1d11eb`. 16 tracked files, 3023 insertions and 246 deletions. Binary diff SHA-256 `99a44acdb917acc93bdcd062f37884d770f6f441489d5969a7f1b24b3c379332`.
**Reviewer**: Codex root, independently of Grok implementation. Grok mechanically executed the bounded review probes and transcribes this report.
**Verdict**: 0 defects, 1 smell, 0 nitpicks

## Defects

No new runtime correctness defect was established in the reviewed repair.

## Smells

### S1, The visit counter omits ordinary work and its budget denominator is unproved

`crates/rdocx-py/src/document.rs:5266`
`crates/rdocx-py/src/document.rs:5636`
`crates/rdocx-py/src/document.rs:5655`
`crates/rdocx-py/src/document.rs:5793`
`crates/rdocx-py/tests/test_core.py:11061`

The normal fast-path qualification loop in `word_inventory_is_canonical` visits owners without incrementing the visit counter. Preparation also walks all owners to count kept scopes and to collect retired scopes without counting those visits. Commit searches owners for the body part without counting the search. These are ordinary qualification, preparation and commit work, not the explicitly excluded debug whole-source comparisons.

The passing metric therefore under-reports the work the approved counter is meant to measure. Audit the other owner lookup, index and projection loops on the qualified path as well, including iterator adapters. Count visits at the actual work, not with one marker beside a scan.

The Python bound then counts visible paragraph, run, table and cell owners after the mutation. The approved budget uses owner and scope-state counts for the qualified operation. The current post-mutation visible count is not shown to be that denominator or a conservative lower bound for it, and queries can materialize handles after the measured operation. Capture the relevant pre-operation counts or establish a conservative lower bound before the operation. Reset measurement after any counting queries. A stricter bound derived from a proved lower bound is acceptable. Do not inflate the denominator or weaken the approved factor.

Rebuild the authenticated debug extension and refresh the affected counter observations after instrumentation. Keep actual raw counts, the denominator derivation and the selected membership. Local counters still do not qualify the final Python timing threshold.

## Nitpicks

None.

## Not found

- Correctness: no additional defect found in the direct reader dispatch, owner route movement, append versus conservative publication or the shared scope-preparation duplicate detection. Recoverable input, owner preparation and Python result creation precede the qualified native edits. The remaining revision checks were traced against the same exclusive borrow.
- Contract: native None versus invalid-path Err behavior is retained. The hidden native providers and debug-only instrumentation earn the published native API rider. Stable Python signatures remain unchanged.
- Panics: no additional unchecked untrusted index was found in the new path setters or selected owner movement. Invalid run and cell paths have the recorded checked tests. The generic debug comparison and guard are not release behavior.
- OOXML: body dispatch and root construction preserve the existing serializer. The permitted w14 comparison strips only the canonical declaration in the root start tag. Original root bindings, used bindings and opaque source retain their owners.
- Tests: the dev extension's direct-read guards compare the same selected bytes with the whole-source reader. The real Base w14 failure is a semantic public-API assertion. Selected scope precedence and missing-path Rust tests ran after the initial zero-selection attempts were corrected. S1 is the remaining work-bound proof gap.
- Structure: no new product module, crate, trait or feature flag. The shared writer generic retains the existing generic CT_Body interface, with Vec and borrowed Vec writer instantiations used by the current serializer and validation test. The debug guard has actual byte-read and inventory-result instantiations.

## Evidence consumed

Root inspected the implementation, consumers, contract and assertions. It did not change product files or run a build.

- `/private/tmp/s95.4-fx266-local-ready.txt` records the real old-source w14 failure and final nine selected pytest passes.
- Successful final dev extension SHA-256 `a89567b2475960789967adf4ba03219ff17fe0c5314f78811e3fa72b41aa02fd`, Python 3.12.13. `/tmp/fx266-fixed-maturin4.txt` succeeds and `/tmp/fx266-py-final.txt` reports nine passes.
- `/tmp/fx266-precedence3.txt` and `/tmp/fx266-runpath5.txt` record actual selected Rust tests. Formatting and changed four-crate all-target/all-feature clippy pass.
- The first two purported repaired-source Python runs followed failed extension builds and loaded the older module. `/tmp/fx266-py-green.txt` and `/tmp/fx266-py-green2.txt` are invalid repaired-source observations, not five-pass credit or new-source semantic failures. They remain separate from the authenticated final run.
- Read-only in-memory probes are retained in `F-X266-pass-1-probe.txt`, `F-X266-pass-1-baseline-probe.txt` and `F-X266-pass-1-current-probe.txt` beside this report. Invalid-text append and insert publication matched the preserved F-X265 runtime prefix, so no new atomicity regression is attributed to this repair. The baseline artifact used its Python 3.14 native-module loading context, and exception-class differences are not API parity evidence. Valid controls pass. These diagnostic probes are not native Office or new acceptance gates.

## Acceptance boundary

Local implementation is not ready for preparation until S1 is resolved and independently reviewed. A1's original release-build mean-of-three and separate median-of-seven measurements remain unrun. Full verification, hash, installed typing and stubtest, package dry-run, WASM and the other final union riders remain pending.

This pass ends here and returns control to the orchestrator. It performs no remediation.
