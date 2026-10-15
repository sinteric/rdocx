# F-X227 pre-Worker admission review, pass 3

**Verdict: 0 blocking findings, 0 structural smells, 0 nits in the bounded V2 successor diff.** Reviewed external wrapper `7b599419247a88a10b8d8e5c5eddce3d9a36f58c264725c6c4d7a8afdf3bb835` against retained reviewed ba39. No implementation, runtime, runner, registration or canary was changed or executed. Only this review file was written.

## Wire schema and preserved admission boundary

`/private/tmp/S91.1-F-X227-preworker-v2-remediation-1/admit_worker-v2.py:93` now accepts the official `Mask` member spelling. Lines97 through100 accept only the two literal message kinds `PipelineAgentJobRequest` and `RunnerJobRequest`. The pinned source declares both constants at `JobRequestMessageTypes.cs:7` through8, recognizes the run-service kind at `MessageUtil.cs:9`, and declares `[DataMember(Name = "Mask")]` at `AgentJobRequestMessage.cs:495`, all in the same external remediation directory and attributed to official commit197ca027. The `MaskHints` public C# property is not its wire member name.

The actual diff changes only those wire names, their redacted projection routing at202 through206, and the rejection diagnostic block. Existing setup denials, exact owner identity, script/environment restrictions, same-frame ROOT admission, one-job consumption, raw-frame forwarding and composed cleanup remain unchanged. Unknown kinds, legacy invented `maskhints` and forbidden setup remain refused. This review does not broaden the trusted job policy.

## Diagnostic privacy and boundedness

At `/private/tmp/S91.1-F-X227-preworker-v2-remediation-1/admit_worker-v2.py:353` through374, exception arguments select from a static reason dictionary or `OTHER_REFUSAL`. The actual exception message and arguments are not serialized. Lines375 through381 retain at most eight sites from this wrapper's own file, containing only static function names and line numbers. They do not expose locals, other filenames or traceback messages.

Lines384 through395 reuse the bounded JSON decoder and emit a top-level shape only for dictionaries of at most64 fields. Known names come from a static literal set, unknown names become fixed-size SHA-256 values, and values contribute only their JSON type names. No request values, tokens or raw frame bytes enter this diagnostic. A malformed or unavailable frame suppresses that optional shape. The existing first-frame size and nesting bounds still apply. No concrete additional token disclosure or unbounded diagnostic-output path was established in this diff.

## Authenticated deciding controls

The terminal eight-control receipt `/private/tmp/S91.1-F-X227-preworker-v2-remediation-1/actual-owned-v2-schema-controls-1/receipt.json` hashes to `5c2fd26d5797635e51c6d50c0984814e755d7738fff3db89adace33613725685`. The controller hashes to `75084c36c40e613d0e526bb5dc78bbe1fb9579afd8fceba51ffc7767c1b915ed`. All eight referenced admission receipts were rehashed and all eight copied wrappers match the reviewed successor.

The actual controls at `owned-v2-schema-controls.py:20` through33 supply official Mask spelling, both approved environment mappings, token-bearing variables, endpoint/context credentials and an empty ContextData array. Both legitimate kinds reach same-frame approval and byte-exact fake-Worker forwarding at36 through42. Wrong, empty and null kinds, Snapshot, an unknown secret-bearing key and invented maskhints are rejected before Worker launch. Lines43 through53 check terminal status, cleanup, absence of the credential marker from receipts/stdout/stderr, static reasons and diagnostic field shapes. The positive proposal is also checked for marker absence at37. These are actual owned fake-process controls, not captured server assignments or actual .NET serialization.

The local source-authority files physically match their supplied pins: MessageUtil `9e260d58c1e07296d80c13969171bdf14a9f54ef353762e3bdd0ec22e3133243`, JobRequestMessageTypes `bbcffb705378c10372b143b4793397ecb40a4d5b8a92b9ae264617a7ef96fbb2`, and AgentJobRequestMessage `4a3959d40356ffd30114f37c4ba3377346fa68d39c04401552855c392a9b5035`.

## Historical refusal and handoff

The actual first-attempt receipt `/private/tmp/S91.1-F-X227-owner-ci/attempt-1-retained/receipt.json` physically matches `ae4889d0b05b586936b9212862526bd14ebba45f4bdb892c1747c3ef77e3fd4a`. It records actual CI failure before the real Worker, zero corpus cases, terminal Listener cleanup and ephemeral runner API404. Its exact first rejecting predicate remains unknown because the raw frame was not retained. The V2/Mask corrections are source-supported compatibility repairs, not proof of that historical cause. The ba39 source is retained byte-exact and its prior controls remain historical.

No actual server admission, registration or actual100 acceptance is earned by this pass. A successor deployment must pin these new wrapper bytes and retain mandatory same-frame ROOT admission. No remaining finding was established in the assigned diff. The review ends on writing this file and returns control to ROOT.
