# F-X227 pre-Worker admission review, pass 5

**Verdict: 0 blocking findings, 0 structural smells, 0 nits in the bounded V4 endpoint-admission diff.** Reviewed frozen external wrapper `09107479f21d6f3f445d857ce9d2158877416a4e32c80d842663504fa924a471` against reviewed V3b6616. No implementation, runner, registration, canary or review-triggered runtime was changed or executed. Only this review file was written. The separately authorized, already running current215 core gate continued independently and was not part of this review.

## Endpoint proposal and mandatory owner binding

`/private/tmp/S91.1-F-X227-preworker-v4-remediation-1/admit_worker-v4.py:223` through239 checks HTTPS, absence of userinfo including empty userinfo, an ASCII DNS hostname with bounded labels and total length, and a valid explicit port. The proposal exposes only the public hostname, effective port and whether the host matched the earlier policy list. The full endpoint still passes through the existing recursive value-hashing projection. Paths, queries and authorization values are not exposed by this change.

At318 through321 the proposal binds endpointHosts to the actual frame's endpoint projection. Lines350 through353 require an exact approval object containing those hosts, frame SHA, complete proposal SHA, owner nonce and policy SHA. The complete proposal SHA also binds the displayed port. The host-list match at238 is informational and cannot authorize a Worker. There is no hostname suffix rule or known-host automatic acceptance. Both known and previously unknown valid hosts require explicit ROOT approval of the same frame.

The existing approval custody and proposal-stability checks precede the tool guards, one-job consumption and Worker spawn at342 through363. The diff does not change raw-frame storage or forwarding. It introduces no endpoint network request in the wrapper and does not reinterpret a rejected assignment as admitted.

The pinned official source `/private/tmp/S91.1-F-X227-preworker-v4-remediation-1/JobRunner.cs:64` through69 selects SystemVssConnection from the actual job message and connects using its URL and credential. This supports reviewing the job endpoint itself rather than inferring it from registration or broker hosts. The retained official197ca027 file physically matches `006421e85af890f4ac2192d494992582ca155bfa8996ff94b87d3e152638d6e3`.

## Authenticated relevant controls

The terminal61-control receipt `/private/tmp/S91.1-F-X227-preworker-v4-remediation-1/actual-owned-v4-schema-controls-1/receipt.json` physically hashes to `71c77f2a31121a79ae171bc76e29c1ec927dcda2e4c12323aaa07b7320039087`. The controller hashes to `21fcccdf2587ef358e2cf8c4e9d1827ce9149021b1b409095545d65b13b7bf58`. All61 individual receipt hashes and copied wrapper files were authenticated. The preceding40 schema/expression controls remain present, with21 endpoint controls added.

`owned-v4-schema-controls.py:56` through70 supplies unknown and known hosts, a deceptive suffix, private path/query markers, a valid high port, and malformed scheme/userinfo/port/IP/localhost/Unicode/label forms. Lines73 through86 verify no Worker exists before approval and mutate missing, different, extra, invalid or duplicate host bindings and an extra approval property. The known-host and suffix cases receive no approval and never launch a Worker. The unknown-host positive explicitly asserts policyHostMatch false before exact owner approval.

Lines87 through101 check original-frame hash acknowledgement by the owned fake Worker, correct accepted/refused status, group/leader cleanup, no credential marker in receipts/stdout/stderr, and static diagnostic shapes. The proposal marker check at75 covers endpoint path/query and token-bearing contexts. These are actual owned controls, not an actual server capture or an executed official Worker job.

## Preserved actual refusal and scope

`third-owner-attempt38701589628-endpoint-refusal.json:13` in the same directory identifies validate line225 in the exact reviewed V3b source. That is the endpoint-host predicate. The earlier variable-schema check at V3b217 was passed on that path. This actual third refusal is not a variable-schema failure, and its unretained endpoint value is not guessed. The retained source/authority bridge, preceding source and actual refusal hashes were authenticated. The actual attempt remains refused before Worker and before any corpus case, with no score or current acceptance credit.

The diff is confined to endpoint syntax/projection, exact same-frame owner host binding and a static URI rejection code. Settled identity, setup denials, environment/script restrictions, guards, one-job consumption, privacy defaults and lifecycle cleanup are unchanged and were not broadly reviewed again.

No remaining finding was established in the assigned diff. A successor deployment must pin these wrapper bytes and require the new endpointHosts field in the actual same-frame ROOT approval. This pass earns no registration, actual server-assignment success or actual100 CI acceptance. The review ends on writing this file and returns the canonical writer lease to ROOT.
