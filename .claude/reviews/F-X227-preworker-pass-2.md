# F-X227 pre-Worker admission review, pass 2

**Verdict: 0 blocking findings, 0 structural smells, 0 nits in this bounded successor review.** Pass 1's three findings are resolved. This pass reviews frozen wrapper `ba39c74e9b0e0bc02b482b8bfbd256ddd8f6b2c01f9458fc060890198d5a4ce4`, the composed case cleanup and the narrow environment amendment. It does not repeat settled admission policy review or qualify an actual server assignment. No source, runner, registration, canary or target was changed or executed. Only this review file was written.

## Pass 1 remediation

The successful-child return no longer conceals unsafe cleanup. `/private/tmp/S91.1-F-X227-preworker-implementation-1/admit_worker.py:350` through382 gives the Worker group a finite TERM grace, escalates when its leader remains, reaps the leader and polls group disappearance. Lines443 through449 make unsafe cleanup override the earlier return with status1. The separately owned case-session audit at383 through442 checks the fixed lease directory, custody, original leader identity and process group, performs bounded cleanup and refuses an unverified live group. It does not kill a guessed PID.

The actual case supervisor at `/private/tmp/rdocx-f-x227-codex/scripts/sprint_workflow.py:1590` through1652 catches SIGTERM through an exception, blocks that signal across spawn and lease creation, cleans the separate session, reaps its child and records whether lease release is safe. Its function bytes hash to `fa297f952c8507134c1232c281099024eb9a9181608004c11da3abfa45b972eb`. The executor refuses unsafe cleanup at1815 through1816.

The composed controls use that actual supervisor, not a reimplementation. `/private/tmp/S91.1-F-X227-preworker-implementation-1/composed_parent_canaries-3.py:18` through24 constructs an owned Worker, real driver, separate-session case and case child, plus a real intermediate Listener parent. Lines32 through40 wait for the case lease, then exercise parent exit or wrapper cancellation and assert Worker-group disappearance, leader reaping, case-group disappearance and the driver's safe lease release. The pending parent exit does not launch a Worker. Both accepted controls show the actual separate case session gone. This resolves the mandatory before/after parent-death qualification as well as the composed cleanup gap. The earlier failed composed wave remains failed historical evidence.

The omitted empty-array payload is accepted at `/private/tmp/S91.1-F-X227-preworker-implementation-1/admit_worker.py:59` through62. The rich positive at `lifecycle_canaries-remediation-5.py:23` through26 now includes the official `{t:1}` representation. It passes through the same-frame admission and raw-frame preservation checks at91 through101. It remains a fixture derived from the pinned converter schema, not an executed .NET serializer or captured server-message proof.

## Narrow environment amendment

The accepted job environment is confined to the literal single mapping `CARGO_TERM_COLOR=always` at `/private/tmp/S91.1-F-X227-preworker-implementation-1/admit_worker.py:99` through102. The step environment must contain only `F_X227_CONFIG_SHA`, equal to the owner's validated lowercase64hex pin, at135 through142. Literal mapping decoding at73 through85 rejects expression tokens and duplicate keys. No additional executable environment carrier was admitted by this amendment.

The exact positive and eight negatives at `/private/tmp/S91.1-F-X227-preworker-implementation-1/environment_canaries.py:8` through28 cover altered global values, extra global keys, expressions, multiple layers, and wrong, missing, malformed or extra step values. Their fixture uses the pinned TemplateToken mapping representation. The official schema declares the job list at `pinned-IPC-schema/src/Sdk/DTPipelines/Pipelines/AgentJobRequestMessage.cs:208` and step token at `pinned-IPC-schema/src/Sdk/DTPipelines/Pipelines/ActionStep.cs:48`, both under `/private/tmp/S91.1-one-job-CI-security-proposal-1/`.

## Authenticated terminal evidence

All four completed receipts bind the same frozen successor. Every retained wrapper copy matches it:57 protocol copies,18 lifecycle copies,3 composed copies and9 environment copies. The57 embedded admission receipts match their physical files. Referenced individual lifecycle, composed and environment receipt hashes were checked, with sequential and concurrent replay evidence separately corroborated.

- Protocol57, `actual-owned-canaries-6/receipt.json`: `51cbaffcc44f17697effc65f2f3ea33f201d2a87c10213f6ec5e047c9957e5a8`.
- Lifecycle19, `actual-owned-lifecycle-canaries-5/receipt.json`: `382790a5868c87cbb031ad6b5331057c864ddeec1d17d470d6ab6a162bf788ec`.
- Composed3, `actual-composed-parent-canaries-3/receipt.json`: `66356f8eb811c059b269d06e6982fcdbb18eaf6fd024b110040d252cee5f33ab`.
- Environment9, `actual-owned-env-canaries-1/receipt.json`: `70f4b55de68e9427175d9cdc0299ceead664af9fda34dd7294d68793874d9d7b`.

These paths are under `/private/tmp/S91.1-F-X227-preworker-implementation-1/`. The retained pass 1 source physically matches its original `485867bd5ded4b948e58f67f75856a8c5295ac8ef67c48ac50a291a074cfad53` pin. Historical receipts are not relabelled as successor execution.

## Admission boundary and handoff

The actual deployment must retain the reviewed owner identity, script, environment, tools and one-job custody policy, with `case_lease_root` bound to the actual executor output directory. The eventual assignment still requires the same-frame ROOT approval of its credential-redacted projection. The original frame remains in memory and is forwarded unchanged only after that approval. These owned controls and the earlier unchanged managed-apphost loading proof do not establish actual server-assignment success, actual100 CI acceptance, or a general personal-host sandbox.

No remaining finding was established in this bounded review. Return control to ROOT. The review ends when this file is written. No registration or remediation was performed inside this pass.
