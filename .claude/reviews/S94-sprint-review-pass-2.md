# S94 sprint review, pass 2

**Reviewed**: Clean `sprint/s94` HEAD `76000663e3e5543f120f5090b940b6d27da6a97c` against main merge base `3a3b0de8be9f8bf4eb588a9a24c565f0af8ee190`. Full integrated scope is 108 files, 10,261 insertions and 480 deletions. Crates touched remain oxml-chart, oxml-cli-support, oxml-drawing, oxml-opc, oxml-py-support, oxml-sml, rdocx, rdocx-cli, rdocx-layout, rdocx-opc, rdocx-py, rpptx, rpptx-chart, rpptx-cli, rpptx-layout, rpptx-oxml, rpptx-py and rpptx-render, including measurement-only README carriers.

This required post-remediation pass follows the actual failed final policy run, not a confirmation of unchanged source. Re-read the canonical review/verification contract, full integrated scope, prior review and bounded successor. Against pass 1's reviewed cc4f3f8, only its committed review file and one existing policy test changed. The remediation itself is one file, nine insertions and six deletions, binary diff SHA-256 `7802503a807d4a99ca90a596557cb39b09840f031b438119b49412028b0dea24`. All prior production, feature tests, manifests, workflow, baselines, crate READMEs and catalog inputs remain byte-identical. Their authenticated prior audit is reused explicitly, without claiming fresh runtime execution.

**Verdict**: 0 blocking, 0 should-fix, 1 nice-to-have. Review exit condition met. The optional item is unchanged from pass 1.

## Blocking

0 findings.

## Should-fix

0 findings.

## Nice-to-have

### N1, retained optional whitespace nit

`scripts/officemath_oracle_harness.py:651`

The same whitespace-only line still has trailing spaces, confirmed by read-only `git diff --check`. It was already recorded in F-X247 pass 2 and sprint pass 1. No runtime behavior or required gate depends on it. It remains optional and does not justify a confirmation review.

## Actual failure and remediation

The final full attempt at 3bb3380 genuinely failed. `/private/tmp/S94-full-closure-3bb33803-1/policy.log:210` records the roadmap test converting `95.2` with `int`, raising ValueError rather than checking the adopted split roadmap. That attempt is not a full verification pass.

At `scripts/test_sprint_workflow.py:8642`, plan sprint identifiers now become tuples of integer components. Backlog identifiers use the same representation at `:8659` and `:8665`. This preserves exact S95.2/S95.3/S95.4 distinctions and lexicographic numeric dependency ordering, without float conversion or integer truncation. The explicitly expected placement set at `:8648` now matches the adopted M24 carryover assignments. Ordinary integer sprint identifiers remain one-component tuples.

The same test still audits F-240 through F-310, requires complete and unique placements at `:8644`, requires unique backlog rows at `:8667`, compares HLD/backlog/plan title and size, refuses archived audited stories, verifies real dependency IDs at `:8702`, checks prerequisite ordering at `:8706`, and detects cycles at `:8714`. No checks, stories, failures or meaningful expected set were silently dropped to obtain green. New cross-cutting repair IDs remain governed by their existing planning and consistency contracts, not misrepresented as part of this native-F-ID test.

The actual complete policy successor at current 76000663 exited zero. Independently hashed `/private/tmp/S94-policy-remediation-76000663-1/policy.log` to `f444706c665433a0a9664bddfdba08263630f8c6b5d543131b1fe420eb89d9f4` and verified its report: `Ran 183 tests`, `OK (skipped=2)`. `/private/tmp/S94-policy-remediation-76000663-1/receipt.json` names this exact source SHA and exit zero. This closes the observed policy error, not the whole final gate.

## All eight aspects

- **Interaction**: 0 findings. The policy parser now agrees with adopted split-sprint rows while retaining dependency ordering. Runtime interactions from pass 1 remain unchanged: distinct FormValues/Signing owner proofs, current-state verification, no-op matched counts, signed stream save/reopen, scalar notes projection and finite equation semantics.
- **Duplication**: 0 findings. Integer tuple conversion appears locally at the two actual input boundaries of one existing test. It does not justify another shared abstraction. Previously audited two-consumer Python/CLI report helpers and one shared strict signer remain unchanged.
- **Layering**: 0 findings. No dependency or crate source changed in remediation. No inverse family dependency enters oxml support.
- **Harness**: 0 findings. No baseline changed across the integrated sprint or successor. Actual 49-hash/seven-pixel qualification remains attributed to its original native producer. The policy repair neither changes rendering nor claims a new output observation.
- **Gate**: 0 findings. Actual policy RED and complete current-source GREEN establish the repaired failure. Prior native, installed, hosted and finite oracle evidence retains exact producer/source applicability. A fresh full gate at the future committed review SHA remains required before closure. Neither the failed 3bb attempt nor this standalone policy run is labeled full green.
- **Docs**: 0 introduced findings. The adopted five S94 completions, seven pending repair IDs, 18 unique future assignments, capacity and 54-to-72-day arithmetic are unchanged from pass 1. The parser now checks the documented dotted M24 placements. The affected eleven consistency checks remain clean. The explicitly classified 30 historical strict flags remain residuals, not global zero discrepancies or newly fabricated work. S95 alone does not close Issue385, and M24's final boundary remains S99.
- **Deps**: 0 findings. Manifests/lock, approved ZIP 9.0.3 migration, ordinary signing consumers, default-off published CLI-support edge and native/WASM defaults are unchanged. No dependency is added by the policy repair.
- **Surface**: 0 findings. No public API, stub, export, exception, XML model or feature flag changed. Approved section imports, private dictionary report descriptors, import-only color enum scope and finite form/math limitations remain as audited.

## Retained evidence and limits

Pass 1 independently authenticated the actual complete native gate at da4a75ac, `/private/tmp/S94-full-native-da4a75ac-4/receipt.json`, SHA-256 `594b2f351f2effb28bae678136732ab4dfadb117b95d7f29231f7c62d3a10992`: 18 closed command results and 22 genuine archives. Default 5,493 passed/68 ignored and all-features 6,579 passed/73 ignored overlap. Same-SHA successful continuations from attempts 2 and 3 remain labeled as continuations. The test repair does not alter archived package inputs, but its updated policy test required the actual standalone successor above. Fresh final full execution remains due.

Actual ordinary installed 877 Word plus 905 PowerPoint tests, strict typing/stubtest, 23 fresh public packages and 27 hosted jobs/four publication guard skips remain producer 9cdbea. Actual CLI, corpus and math observations retain their prior exact producer bindings, rather than being called newly executed at 76000663. The cc4 metadata closure receipt `/private/tmp/S94-final-metadata-applicability-cc4f3f81-2/receipt.json`, SHA-256 `eb8604416a678f171c2a2278ac53c22672cdfa735fa0ada6a30d2730d2cae645`, is not a false whole-snapshot equality claim across the subsequently changed policy file. The additional exact successor consists only of that policy test and the review record, as independently checked here. Relevant product/runtime inputs are unchanged.

Corpus qualifications remain raw Word 1/PowerPoint 0, saved/body success 55/76 and 43/44, named Word requirements 16 compatible plus 16 explicit alternatives, exact three pointers, and successful Cell.merge versus its stale original marker. Math retains 73 finite semantics, actual Pandoc 72 emissions plus original degree fallback, seven native positives/seven real mutation rejections, captured historical six with five negative rejections/radical escape, and bounded 13-formula nonregression. The MathKern supplement and matched-Cambria discrepancy stay diagnosed, not relabeled accepted. One lost original S93 identity JSON, overwritten iterative predecessor PDFs and other recorded preservation limitations remain candid. No unavailable proof is reconstructed by source closure.

## Milestone gate and handback

The M24 end gate at `docs/hld/14-development-backlog.md:2668` requires no unexplained partial capability row and broad authoring, validation, preservation, rendering and binding qualification without Word repair. It is not claimed achieved by S94. The final F-310/S99 boundary remains at `docs/hld/14-development-backlog.md:3304`. Limited native captures do not establish the complete milestone human confirmation.

S94 has zero blocking and zero should-fix review findings within its approved contracts. The implementing orchestrator must commit this record, run the required fresh final full gate and bind external evidence to unchanged relevant inputs before close. No final full result at that future SHA, main merge, tag, contribution closure or release is claimed here.

Only this new untracked review file was written, without staging. No remediation, build, test, cache, GUI or network operation was performed. End of pass and return control to the orchestrator. No clean confirmation pass is requested.
