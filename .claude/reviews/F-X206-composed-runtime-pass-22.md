# F-X206, composed runtime, pass22

**Reviewed**: Bibliography-authored prepared R7 runner, complete transition helper,
36-command queue, prerequisite and artifact adapters, R6-to-R7 cleanup delta and
prior runtime findings. No execution or source adoption.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication and independence

Packet: `/private/tmp/S91-F-X206-composed-runtime-r7-prepared`.
Receipt SHA-256:
`4b1616f53e9eb0828de0d1fc73c10bac5a54146e135671e806bed57b34f664ad`.
Runner SHA-256:
`0b60885e50b3e014d2016aaabe213f6d8bf552504b89adf4bc908eb8d323986e`.
Runner delta SHA-256:
`eddc31c9143a9857ac128870e2ae30b8935aa273b02ec4ebd2ae351cb311533d`.
Queue SHA-256:
`f8c40c245b8b18d642edfd44cad4519605584837547f08473b93f2538fdd37fc`.
Transition SHA-256:
`4661eb9f7f74a0547b7e7b62d4c7f871894c961db7577306e6581e70b3c00267`.

Every receipt artifact and predecessor/review identity matched. The current
whole2714 source records matched byte hashes, sizes and modes, with exact tracked
membership, semantic index and HEAD. The source authority remains
`e9e7d4be30058031cafeba358cfaf9751cdd36290335c27dce31d5eb130822b4`.
All36 command dictionaries are unchanged from R6. The native317 projection has
current exact values and unchanged membership. Both helpers parsed as Python
ASTs. This is static evidence only. The reviewer authored the earlier R6 artifact
schema correction, not the R7 lifecycle correction reviewed here.

## Prior findings closure

Pass20 D1 is resolved. `run.py:279` places the spawn, active receipt save, ownership
observation, wait and normal-return descendant observation inside the same
try/finally lifecycle. The captured group identity comes from this helper's own
new-session child. `run.py:294` executes bounded cleanup before later receipt,
log or result validation can exit. Cleanup checks group existence independently
of leader liveness, attempts TERM and KILL with bounded waits, observes signal0
ESRCH and separately reaps the leader with a bounded wait. Normal-return surviving
descendants cause stage failure after cleanup rather than successful qualification.

At `run.py:304`, failed cleanup or observation remains explicit. Safety requires
both group disappearance and leader reaping without a cleanup exception. Otherwise
the row and top-level receipt retain doNotReleaseLease and active cleanupUnproved
metadata. Original post-spawn exceptions are retained and rethrown after cleanup,
including when the terminal receipt save also fails. An unavailable terminal
receipt is not release evidence. No interruption experiment ran in this review.

Pass16 D1 through D3 remain resolved in the reviewed source: actual producer
schema/logs, same-byte monotonic transition records, tool binary/version probes,
complete ordered prerequisite rows, exact argv/cwd, counts, source completion,
fresh paired wheels and installed provenance are consumed. The current eight-owner
artifact inventory is linked to the immutable actual104 receipt and detached seal,
then to the actual current317 projection. Pass16 D4 is subsumed by the whole-group
lifecycle above.

Pass17 closures remain intact: caps environment executables are matched to the
actual corresponding version-probed binaries, and the transition consumer binds
the exact helper, runner, queue and runtime review identities. Pass19's artifact
schema correction remains intact: two variable owners require their actual
expectedSVG identity, while six style owners require their actual expected PNG
identity. The actual SVG identity is mandatory for every owner. No proposed future
Q/C schema is silently treated as adopted source.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- correctness: No additional defect in the corrected lifecycle or stage lineage.
- contract: No reduction of full104, the eight-owner native gate or caps oracle
  obligations. Conservative27 entrypoint selectors remain explicitly qualified.
- panics: No further uncovered post-spawn exception path in the reviewed lifecycle.
  Observation and receipt failures do not grant completion or lease-release credit.
- ooxml: No product or XML source change in this runtime packet.
- tests: No empty prerequisite, zero-test or old-wheel acceptance route observed.
  Named tests, actual terminal counts and source/provenance checks remain required.
- structure: No product file, dependency, module, trait or public wrapper added.

## Preparation limits and required renewal

This clean review does not authorize execution. The packet still represents the
current pre-recovery composition source, not the pending F195 R29 oracle source,
returned-handle factory integration or member-recovery R2 composition. Before
executing the intended combined result, refresh its actual whole source map,
HEAD/index, native317 projection, test inventory, command counts/selectors and
artifact schemas against the adopted source. Retain this packet as immutable
preparation history. Obtain an actual latest producer and reviewed byte-preserving
transition admission for that refreshed source. Do not relabel current prepared
schemas or old successful artifacts as the future composition.

No Cargo, compiler, process, installation, product import, painter, cache, mtime,
UI or intake operation ran. No full104, native paint, provider completion or F206
completion is claimed. Only this report was written. Review ends and returns the
canonical reviews writer lease.
