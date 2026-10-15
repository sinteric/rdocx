# F-X211, native performance, pass 1

**Reviewed**: Frozen source102 compared with exact source101. One changed carrier, crates/rdocx/src/document.rs, 44 insertions and8 deletions. This bounded pass covers excluded-owner qualification only.
**Verdict**: 0 defects,0 smells,0 nitpicks. The excluded-owner correction passes this review. Retained-control repeated text qualification, maximal85 and full F-X211 acceptance remain unresolved.

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Aspects checked

- Correctness: zero findings. The actual nested-owner predicate short-circuits qualification in Start and Empty at `crates/rdocx/src/document.rs:11271` and line11354. XML events, source spans, namespace resolution and frame traversal continue. Excluded paragraph frames remain opaque at line11417, so their closing tags do not enter the selected owner's paragraph inventory or clear its complex-field state. Retained direct owners and leaf controls still receive qualification.
- Contract: zero findings. Selected-owner text uses existing nested_owner_depth at `crates/rdocx/src/document.rs:12280` and line12364. No schema quota, resource budget, accepted owner type, public API or transaction staging is changed. The checked table replacement and save/reopen controls at line36608 retain exact owned payload expectations and concrete typed-chain survival.
- Panics: zero findings. No new unchecked slicing, indexing, unwrap, recursive stack or arithmetic is introduced in production. Existing fallible NsReader and prefix handling remain active. The End path at `crates/rdocx/src/document.rs:12419` retains balanced qualified owner-depth decrement. Foreign or previously opaque descendants cannot expose selected text while depth is nonzero.
- OOXML: zero findings. Inherited opacity is preserved at `crates/rdocx/src/document.rs:12284`. The reader still traverses every excluded subtree and maintains namespace scopes at line12340. Qualified nested cell/text-box starts and ends remain paired, including foreign descendants. No serialization or raw-byte rewriting is introduced. Opaque and rebound nodes retain exact spans in the independent fixture at line36562.
- Tests: zero findings. `crates/rdocx/src/document.rs:36557` independently expects the selected outer text beforeafter7, the separate inner text excluded, exact owner kinds and exact foreign/rebound bytes. It also requires zero excluded qualification counts, so restoring the old qualifier path fails the deciding expectation. The unchanged finite transaction at line36584 proves concrete table/control chains and changed terminal text after checked replacement, save, reopen and drop. Existing paragraph, namespace, field-offset, AlternateContent and run-tab controls remain distinct semantic evidence at lines37540,37629,37668,37763 and37856.
- Structure: zero findings. Existing scanner state and concrete frames are reused. No cache, framework, trait, generic, wrapper, file, module, dependency or feature flag is added. The Empty diagnostic context now reuses the already computed nested-owner predicate, without changing its measurement meaning.

## Actual evidence and limits

Receipt102 SHA `f3d9a72c9a02ff2bcf4fbb64c3564ffa7d7e5550e494f05d0c088ecf63438c87` authenticates all18 actual exit0, qualified, non-timeout rows with reaped leaders and gone groups. These comprise affected all-target check, the deciding debug control, paired release compilation, nine finite transactions and six semantic controls. The latter15 executions use2097152-byte stacks. The compilation/debug commands use8388608-byte stacks and are not relabeled ordinary-stack executions. All raw log hashes were checked. No tests or runtime were run by this reviewer.

The same nine tables1/2/4 by controls8/16/32 cases retain exact outputs and independently show excluded scan/text qualification counts zero. Retained leaf admission remainsN and retained per-item text admission remainsN(N-1)/2, namely28/120/496. This report does not credit a fix for that second cause or infer maximal85 success. Receipt101 SHA `ae2d0d551be037137c332b0a41cec45dd824e547f9897d53c0e28a4349132186` and all its11 original log hashes authenticate the earlier measured work. The original maximal85 timeout remains unresolved.

Current Word binary SHA `734d708e2aaed142ece6e9ed36f72ee1d704b31d4eb967b677d27bbfec3f2edc` and OXML binary SHA `4adb8a8764ae512c705e5197ede05f83a035e9d33d618bd9f27ae24b006f371a` were rehashed. Retained source101 Word SHA `2c3c7f27911c46b74611bc56ef9b739fdf331bad65d71268649ff56d2d4a2004`, its OXML binary and retained source90/source97 binaries also authenticate. Existing native97 namespace review remains applicable and unchanged. The generic receipt phase string retains earlier local-namespace wording, while the specific namespace97 evidence and transfer checkpoint identify the actual current boundary.

## Exact delta and source integrity

Source101 document bytes were independently reconstructed in memory by reversing the two scanner short-circuits, restoring the text Start/Empty qualifier paths, removing only the new deciding test and restoring the original Empty diagnostic expression/declaration position. Reconstructed size2048415 and SHA `92ae71cb32d2bdebc8398ea039eca6960a888981e1ee3f703f9ec75404021b50` exactly match the preserved source101 map. Source102 document size2051365 and SHA `b987e7c9d689e70b9e14cd5e1c519814be491be9c81d12351a6ba1928737ea6a` authenticate. No other tracked source-map entry changes between101 and102.

All3205 source102 entries match bytes, sizes and modes before and after this report. Source-map SHA is `1b769d752f0977ca64f348e0bc0d4b8238d927dfe75053b364ce2b49102aa936`. Transfer checkpoint SHA is `6f3c1ecab66a81bc5df95a603ff355fbfaf33c777d938b42f65f9a476e4bedb7`. HEAD remains00b0d4eaec2b157a1f2555badd5cbbe89e0eb787 and branch remains work/f-x211-codex. NUL-delimited index-stage SHA remains `0bf0ff82a27877f3020371a3eee54dc3b520d394eb6a4c93e647a97937f9634b`. Current binary diff SHA remains `d5e248e43e200409906d9b4a9464109d645f90c2253c215a073bf33164c2cf94`. Only this review file is written.

The approved plan, canonical workflow/microscope and cited native ownership/preservation contract were read. This is not a whole-feature review or completion claim. Word public XML/lifetime, PPT native/public XML, final frozen100 composition, native Office and final scoped acceptance remain outside this pass. Control returns to the orchestrator after this report.
