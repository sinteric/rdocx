# F-X206, runtime admission, pass16

**Reviewed**: External prepared packet at
/private/tmp/S91-F-X206-composed-runtime-r3-prepared,36 command rows,
146-line runner and its queue, source inventory, carrier coverage, installed
probe and adopted native SVG consumer. No proposed F200 factory union was
reviewed in this pass.
**Verdict**: 4 defects,0 smells,0 nitpicks. Do not execute the current runner
as qualified admission until these defects are resolved.

## Authentication and scope

Read canonical guidance, workflow and microscope, the approved F206 contract,
current progress/status and composition pass15. F206 remains in-progress.
All2714 adopted source byte/size/mode records match the authoritative map
SHA e9e7d4be30058031cafeba358cfaf9751cdd36290335c27dce31d5eb130822b4.
All41 coverage records match that same map. All five packet artifact hashes,
the installed probe and all16 immutable artifact hashes match. Queue SHA is
73a6b16de421e23b39b780ab4afb47366061c46f93336aca62e1ab0cb06b686d.
Runner SHA is
51a2a5610e0b3942f385985e7104a647b54f1c2b9d6ca4d7ef711ee8ae571cbe.
Prepared receipt SHA is
dd71cd840c6fa72aa7e54d2dc0387b86ce94fe2f1bf28f6b0932eb2d7a6f1e23.
These authenticate preparation bytes, not successful execution or producer
qualification.

## Defects

### D1, producer transition and tool claims are unvalidated booleans

`/private/tmp/S91-F-X206-composed-runtime-r3-prepared/run.py:20`
`/private/tmp/S91-F-X206-composed-runtime-r3-prepared/run.py:24`
`/private/tmp/S91-F-X206-composed-runtime-r3-prepared/run.py:25`

Admission requires true producerTerminalAuthenticated,
composedTransitionAuthenticated, toolVersionAndBinaryPinsAuthenticated and
nativeSubsetSourceClosureAuthenticated. It then hashes referenced files
without reading their terminal/source/command/transition/tool schemas.
A hash-bound failed, running, unrelated-prefix or unapplied transition receipt
passes these checks when the flags are true. No actual executed producer
closure is compared with the own source, and no selector/mtime/unchanged-byte
transition result is consumed. Compiler, maturin, Python and native oracle
paths likewise have no mandatory binary/version receipt correspondence.
Consequently the runner can start with foreign shared-target artifacts or
unqualified tools despite its admission claim. Validate actual receipt and
log schemas, terminal outcomes, immutable compiled source scopes and the
applied transition against this own source. Bind executable identity/version
and actual environment to those records. Failed semantic producer terminals
may be legitimate provenance, but a true flag is not that proof.

### D2, prerequisite completion does not prove the prescribed stage ran

`/private/tmp/S91-F-X206-composed-runtime-r3-prepared/run.py:30`
`/private/tmp/S91-F-X206-composed-runtime-r3-prepared/run.py:33`
`/private/tmp/S91-F-X206-composed-runtime-r3-prepared/run.py:35`

The prerequisite validator accepts any completed receipt with matching stage,
source and queue strings whose supplied rows have exit0. Empty rows satisfy
all(), and missing/duplicate/replaced command rows are not rejected. It does
not bind the receipt's helper/admission identity, exact expected argv/cwd,
verified counts/names or source-guard completion to the current stage.
For example a native receipt with no tests qualifies the build stage, or a
build receipt with arbitrary hash-valid wheels qualifies installation.
Require the exact prescribed stage row set and command identities, real
terminal/count validations and successful final guards, with an explicit
schema for legitimate additional provenance rows. Do not require invented
success fields from unrelated historical receipt formats.

### D3, installed SVG handoff is neither linked nor schema-compatible

`/private/tmp/S91-F-X206-composed-runtime-r3-prepared/run.py:40`
`/private/tmp/S91-F-X206-composed-runtime-r3-prepared/run.py:63`
`/private/tmp/S91-F-X206-composed-runtime-r3-prepared/run.py:142`
`crates/rdocx/src/svg.rs:3523`
`crates/rdocx/src/svg.rs:3549`

The acceptance stage generates an eight-owner inventory with all2714 tracked
source records. The actual mandatory native consumer requires a317-record
source map and an admission carrying installedReceipt, two wheels and their
hashes, pairedFreshWheelsAuthenticated, rootActualTerminalAuthenticated and
sourceModesAuthenticatedByRoot. The produced inventory is not that schema.
The installed-svg stage only checks a true artifact flag and nonempty
F_X195_INSTALLED_SVG_ADMISSION environment strings. It does not hash/parse that
actual supplied admission or link its installed receipt, wheels,317-source
projection and eight artifact receipts to the completed current acceptance,
build and install records. Thus passing the generated inventory directly
fails, while supplying an unrelated older native admission can instead
validate older artifacts. Provide an explicit guarded adapter for the native
schema and verify its exact current lineage and source subset before Cargo.
Keep the native gate's eight-owner physical/font/coordinate and .99 controls
strict. This finding is in the handoff, not a request to relax those controls.

### D4, timeout cleanup may release an owned group with live descendants

`/private/tmp/S91-F-X206-composed-runtime-r3-prepared/run.py:79`
`/private/tmp/S91-F-X206-composed-runtime-r3-prepared/run.py:83`

After TERM, child.wait() returning breaks the cleanup loop immediately.
If the process-group leader exits while an owned compiler or oracle child
survives, no group-liveness check or bounded remaining-group cleanup occurs.
The poll-is-none condition also prevents escalation once that leader exits.
The wrapper then records failed timeout but can return Cargo ownership while
its descendants still compile in the shared target. Authenticate the owned
process group, perform bounded group cleanup when descendants survive and
record actual disappearance before releasing the lease. Do not signal an
unrelated process or infer disappearance solely from the leader's exit.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Checked without additional findings

The source closure and preparation hashes are sound. The runner has fresh
attempt directories, real child exit/log hashes, before/after tracked source
byte/mode/size guards and fail-stop behavior. The stage selector strings,
explicit ROOT authorization and own process-group creation are present.

Native count claims are static until their actual test terminals. Named
native groups check count plus expected sentence names. Installed selection
uses explicit current nodes, actual collection-derived parameter counts and
requires104 for the style corpus. Ordinary pytest passed-summary validation
rejects failures and skips. Explicit rootdir addresses the prior relative
collection issue. No collection or actual104 result was obtained in this
review. Both native oracle commands and the separate native SVG command use
--exact --ignored and require one passed,zero failed,zero ignored.

The installed probe checks CPython3.12.13, declared dependencies, both native
extension paths and package/METADATA/WHEEL bytes against the actual wheel
members. That is useful installation correspondence, subject to D2's real
build provenance requirement.

The readiness document explicitly leaves picture placeholders, cell anchor,
internal run links, placeholder names, every arrangement variant, layout
inventory invalidation, remaining route paint, member-local recovery,
native open-without-repair, full native families, WASM, hash attribution,
publication/HLD/policy and full acceptance due. Missing unimplemented scope
is not misrepresented as a passed runtime gate. No source implementation,
OOXML mutation, new product trait/module or rendering behavior is added by
this packet. No additional finding for OOXML or product structure.

## Handback

Only this review file was written. No worker source, scratch, plan, process,
compiler, Cargo, product import, installed environment, cache, mtime, UI or
intake was changed. The pass ends here and returns the reviews-only lease.
