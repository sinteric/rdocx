# F-X206, runtime admission, pass 20

**Reviewed**: Independently authored R6 packet at `/private/tmp/S91-F-X206-composed-runtime-r6-prepared`, its complete runner, transition, queue and R5-to-R6 schema delta. The reviewer authored R5 admission changes, but did not author this R6 correction. The current source authority and inherited execution lifecycle were checked explicitly.

**Verdict**: 1 defect, 0 smells, 0 nitpicks. No execution or admission.

## Authentication

Authenticated every R6 receipt artifact identity. Receipt SHA-256 is `4a848159511cb3485de4823139e61914edc165df2e41222b90299a9fc64a2d81`. Runner identity is `576941891c949c6fd7719df32791c4b5b454bdeb8a3b37acec5120611012f6c3`. Queue identity is `8967207bf9d4267f7a8bb0ced6c23f1901d86745d90ef2fe364491169a65ea31`. The transition helper remains `4661eb9f7f74a0547b7e7b62d4c7f871894c961db7577306e6581e70b3c00267`.

Independently matched all 2714 current F206 byte, size and mode records to source authority `e9e7d4be30058031cafeba358cfaf9751cdd36290335c27dce31d5eb130822b4`, including semantic index identity and tracked membership. All 36 command dictionaries remain equal to R5. The 317-record SVG projection has exact current values and unchanged membership. Both helpers parse as Python ASTs. No product import or helper execution occurred.

## Defects

### D1, exceptional post-spawn paths remain outside owned-group cleanup

`/private/tmp/S91-F-X206-composed-runtime-r6-prepared/run.py:276`
`/private/tmp/S91-F-X206-composed-runtime-r6-prepared/run.py:277`
`/private/tmp/S91-F-X206-composed-runtime-r6-prepared/run.py:284`

The child is spawned in an owned new session, then the active-process receipt is saved before entering the `child.wait` exception handler. An I/O failure or interruption at that save exits to the outer failed-stage handler without stopping or observing the already spawned group. The normal-return group observation and cleanup at lines 284 through 287 are also outside that handler. The handler therefore covers exceptions during wait, not every exceptional post-spawn path.

The runner can report failure while its pytest, painter or Cargo descendants remain live. Preserve the original error, but put the complete post-spawn lifecycle under bounded owned-group cleanup. Record actual disappearance and leader reaping, including failures of cleanup or observation, with an explicit do-not-release qualification when safety is unproved. Existing normal and timeout group handling must remain. A failed wrapper or leader exit is not disappearance evidence. This finding is source-backed and does not claim that an interruption experiment ran.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Checked without further findings

- **Correctness**: Pass19 D1 is resolved. Every artifact requires its actual SVG hash. The two current variable cases require `expectedSVG` and its hash, while the six style cases require `expected` PNG and its hash. This matches the current source producers and retained native consumer. No future Q/C source change is silently assumed.
- **Contract**: 0 additional findings. Current full104, eight unique owners and the separate exact ignored native SVG gate remain mandatory. The source and composition review authority remains distinct from the independently clean runtime review required for admission.
- **Panics and failure handling**: No additional findings beyond D1. Invalid prerequisites fail closed. Actual child exit, timeout, group observation, raw log, named test and terminal count checks are retained. Group cleanup after an ordinary wait interruption and timeout remains bounded, but does not resolve the uncovered lifecycle above.
- **OOXML**: 0 findings. This packet changes no product or XML source.
- **Tests and evidence**: 0 additional findings. Prerequisites require complete ordered stage rows, exact argv/cwd/helper/queue/source/admission lineage and final guards. Build and install link the two fresh wheel hashes to actual provenance. Acceptance, preservation and typing revalidate installed provenance. The completed immutable104 receipt, detached artifact seal and current317 adapter remain linked to the installed source, wheel and eight physical artifact identities. No empty or merely prepared stage earns pass credit.
- **Structure**: 0 findings. The correction is a small branch on the two actual artifact schemas. No product file, module, dependency, trait or wrapper is introduced.

The transition consumer retains exact reviewed helper, runner, queue and runtime-review identity checks. The transition producer retains those bindings before mtime changes. Actual producer validation still consumes the explicit F200 terminal schema, actual ordered command prefix and raw logs, qualifying semantic failed terminals without promoting partial compiled scope to an exhaustive cache history. The 27 selectors remain conservative source-bound entrypoints. Actual corresponding caps executables remain bound to the binary identities and successful version probes used by the tests.

## Limits and handback

The inert transition template remains preparation only. No producer, cache transition, actual tool probe, build, installation, full104 pass, native paint result, provider completion or feature completion was observed in this review. Future adoption of the separate F195 Q/C oracle proposal requires its own refreshed source projection and artifact schema.

Only this review file was written. No source, scratch, Cargo, compiler, runtime, product import, cache, mtime, baseline, UI or intake operation ran. This review pass ends and returns control for a distinct remediation phase.
