# F-X268, guarded immutable read getter, pass 26

**Reviewed**: New working diff against b7a7618f1916d92a2e55b7f6dc127467ff5bcdcf. Four tracked files, 26 insertions and seven deletions. Diff SHA256 is 5bb1f470c0f016897442a96ddf6c8935d82af49db20330f3eb38650ad338ceb3. Native document.rs SHA256 is 8432000ab37a2997c277d814d274ec7e088331f69b46a16cfcd5afd992e12e6c. All four source hashes match the initial and final freeze.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Independence and scope

Codex flat_cost_design reviewed this new getter diff. The reviewer implemented earlier F-X268 work and independently assessed this design, but did not author the current prototype or canonical implementation. Shared authored the temporary mechanism and Root authored the canonical port and documentation. This is not a confirmation review of pass25. Its unchanged publication, rollback and fallback implementation is consumed as the reviewed base.

The canonical workflow, microscope command, active plan and current source context were read. F-X268 remains in progress. Only this review file was written. The reviewer ran no compiler, test, profiler, renderer, timing or RSS process and made no source repair.

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Source audit

`reconcile_readback` at `crates/rdocx/src/document.rs:9267` still calls the complete `cached_document_read_context` before accessing any child. That unchanged guard qualifies every existing successful ParsedChild, namespace context, exact source partition, current count and private policy. It rejects reads_anew, failed and missing records. Only its subsequent reconciliation loop changes from read_child to fallible initialized-lock and parsed.as_ref access.

For the same immutable self, the old accessor would return Cow::Borrowed pointing to precisely that initialized parsed value. OnceLock cannot replace an initialized value through shared access. The block bytes, staged value, scopes and parsed model cannot mutate through the intervening comparisons and local replacement preparation. No missing proof is filled, private parse is skipped or staged model promoted to source authority. Failure remains fail-closed without unchecked access. The original namespace certificate, prepared clones, complete frame transaction, exact resource validator and rollback are unchanged.

The test addition at `crates/rdocx/src/document.rs:51809` runs under the full guard and asserts both the old borrowed variant and pointer identity with the direct parsed reference. This distinguishes the exact source object from a merely equal value. Existing missing/private/refusal scenarios remain in the same guard matrix. No public API, cache field, generic, feature or production process switch is added. The approved plan and HLD03/HLD12 reflect this bounded getter policy.

## Evidence

Final freeze `/private/tmp/s954-cached-getter-canonical-scoped/freeze.json` has SHA256 063cc8565b724b7c98a879736f400d22d41b62027d09c8aeded91fcfd12472a0. Actual source and diff identities match. Scoped receipt SHA256 is c2c47942e1c83d67d9ddd1d46086b8e5c2d2050442d9a12d37951a60396cea97. Its seven recorded stages and every log hash were authenticated. All-feature/all-target check, actual native unit build, one selected canonical_readback_guard_matrix with ten scenarios plus pointer identity, clippy, fmt, prose and diff checks pass. Actual native binary SHA256 is 75076eff1b2e8559ee036a7a3e2f9381d7b876605cfd435dc62b769b260bb505. The earlier26 selected tests were not rerun and are not relabeled as executions of this source.

The getter-only prototype receipt `/private/tmp/s954-b7-cached-getter-prototype/RESULT.json` has SHA256 79f1d3d590fe6fb0b605ae051b243a6a30788fd90464d4bb12a1bec739b8dcde. It records diagnostic module b8afafb3bf25b6865672ab1925d1575a00fc117e2803eb5b638bff7db815747b, with nondebug/nontest counter configuration. Twelve ON and twelve getter-only OFF windows preserve exact semantic and physical outputs against the v3 ON mechanism. The other three publication optimizations remain enabled. Twenty-two guard observations and eight focused Python controls pass.

The nested raw counter vectors were inspected directly. Complete guard entries and child checks match in all twelve ON/OFF rows. Eliminated actual accessor and reads_anew entries are114/116/118 at25 and280/282/284 at100 for Document, and112/278 for Cell. These are actual function-site reductions. Private original owned moves, clones, original publication admissions, parser attempts and namespace work remain unchanged. The reported guard arrays are call/byte vectors, not ON/OFF omission counts. Diagnostic mechanism observations do not qualify a normal performance gate.

## Remaining qualification

The normal b7 Document picture miss and Cell pass remain their actual outcomes. No retry, threshold relaxation or timing acceptance is inferred. A new authenticated normal module, both picture gates, remaining final performance and counted-work checks, memory/history and integrated riders remain pending. This pass ends after this file is written and releases the review writer lease.
