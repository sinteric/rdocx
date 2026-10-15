# F-X195, current runtime, pass38

**Reviewed**: Bindings-authored prospective current-R29 runtime packet, complete
runner, source/producer transition, fresh paired wheel lineage, full104 selection,
typing and native eight-owner Q/C handoff. Independent runtime review only.

**Verdict**: 3 defects, 0 smells, 0 nitpicks. Do not execute this packet.

## Authentication

Packet: `/private/tmp/S91-F-X195-current-r29-runtime-prepared`.
The initially supplied receipt63ad254d was superseded by the author before final
handback. This review authenticates final receipt SHA-256
`dd65ba00c90bd15e2abc04c2948afa93690b5e76788d1a0687863dff92097240`.
Runner remains SHA-256
`f7fdb3281f21ee143fe52b2cab7e8db7b3358c8334f57ab89b31434dd099741e`.
Queue SHA-256:
`51f148fbba984263d69ce80eeef84135488fffa4c24051799572c55620214ce1`.

All final receipt artifacts matched. Current all2679 tracked byte/size/mode guards,
HEAD and semantic index matched. Current317 inputs are the exact tracked subset.
Immutable input and tool hashes/resolved paths matched. Source authorities are
tracked dc070fa626092335410ffe92bb3b3fbe2666ca052c0ef62cccf0abb8cc620173
and317 b0a031f30ee5dabb5896a11200a72afa94b93934128f417d1b829383fa2f90cf.
Python AST checks are syntax evidence only. No helper, product or oracle ran.

## Defects

### D1, spawn precedes the cleanup boundary and can earn false release credit

`/private/tmp/S91-F-X195-current-r29-runtime-prepared/run.py:189`
`/private/tmp/S91-F-X195-current-r29-runtime-prepared/run.py:204`
`/private/tmp/S91-F-X195-current-r29-runtime-prepared/run.py:369`

The actual owned child is spawned before the try/finally lifecycle. PGID capture,
cleanup state construction, local helper construction and timer initialization
also precede that boundary. An interruption after Popen returns but before line204
can leave its process group running without an active receipt or completed row.
The outer failure calculation sees no active record and may evaluate all() over
zero rows as safe to release. This does not preserve the independently reviewed
R29 whole post-spawn closure.

Place spawn and the full subsequent lifecycle inside guaranteed cleanup, retaining
the returned child identity even when metadata writes fail. Record actual group
absence and leader reaping before release, preserve the original exception, and
leave explicit do-not-release state when cleanup cannot be proved. The existing
bounded TERM/KILL and normal-return descendant refusal must remain. This finding
is source-backed, not an executed interruption experiment.

### D2, the full104 artifact adapter requires a nonexistent style SVG reference

`/private/tmp/S91-F-X195-current-r29-runtime-prepared/run.py:248`
`crates/rdocx-py/tests/test_rendering_threads.py:1583`

artifact_inventory unconditionally indexes expectedSVG and expectedSVGSHA for all
eight cases. The current six style artifact producers emit expected PNG,
expectedQuadratic and expectedCubic PNG identities, but do not emit expectedSVG.
The two variable producers separately include the static searchable SVG reference.
Consequently even a genuine full104 success fails with KeyError before creating
its native admission. This is the same class of schema mismatch already found in
prior runtime reviews, now against the actual R29 Q/C producers.

Validate the two concrete schemas explicitly. Require actual product SVG, both
independent Q/C images, physical oracle font and applicable reference identities
for every case. Require expectedSVG for variable cases and authenticate it when
otherwise present. Do not invent a style SVG, weaken Q/C or bypass the mandatory
native gate.

### D3, consumed transition metadata is not checked against current timestamps

`/private/tmp/S91-F-X195-current-r29-runtime-prepared/run.py:286`
`/private/tmp/S91-F-X195-current-r29-runtime-prepared/run.py:300`

The consumer accepts a completed transition by checking recorded hashes and that
its recorded afterMtimeNs exceeds its recorded beforeMtimeNs. It never checks
that the actual selector still has that after timestamp or recorded inode.
current() checks source bytes/modes only, and the per-command guard also omits
transition metadata. Thus a reverted or superseded invalidation can satisfy
admission while the required shared-target recompilation trigger is absent. The
transition producer records those exact fields, but the actual runtime does not
consume them against current files.

Verify every actual selector timestamp and inode against the authenticated applied
transition before nontransition stages and in subsequent command guards. Bind the
completed transition's source/head/index/admission lineage without substituting a
prepared or merely plausible record. Preserve same-byte monotonic behavior and
fail closed when the transition no longer represents the current files.

## Smells

None.

## Nitpicks

None.

## Checked without additional findings

- correctness: No additional findings beyond D2 and D3. The known R19 producer
  keeps its30 qualified commands and final semantic101 distinct. It is not called
  the most recent execution.
- contract: No additional findings. Word56 plus PPT48, fresh paired wheels, four
  typing checks and mandatory exact ignored native SVG1 remain scheduled.
- panics and failure handling: No additional finding beyond D1. Cleanup after
  entering its boundary is bounded and refuses surviving descendants.
- ooxml: No product or XML source changes in this runtime packet.
- tests: No additional findings. Exact collected membership and executed PASSED
  membership must agree. Current physical owners, coordinates, Q/C and .99
  obligations remain in the source, not replaced by runtime metadata claims.
- structure: No new product module, dependency, public wrapper or target.

Fresh wheel members and installed native provenance are linked through exact
build/install prerequisite rows. Stage receipts bind current helper/queue/source
identities and actual logs. The separate latestProducerCleanup schema links an
actual observation to its exact terminal receipt when that receipt lacks process
fields. No successful exit alone supplies owned-process disappearance evidence.
Templates retain authorization false and latestProducer null.

## Limits and handback

The current latest-producer adapter accepts the exact rdocx all-features/all-targets
check schema. ROOT reports a successful R21 check, while the subsequent portrait
test is live. Its eventual terminal must be reflected in a renewed producer
adapter/admission before execution. No check-only record may be relabelled as the
new test terminal or an exhaustive cache history.

No source adoption, transition, Cargo, compiler, build, installation, interpreter,
painter, UI, cache or mtime operation occurred. No full104/native pass or F195
completion is claimed. Only this report was written. Review ends and returns the
canonical reviews writer lease for distinct remediation.
