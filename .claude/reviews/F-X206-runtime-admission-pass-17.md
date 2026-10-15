# F-X206, runtime admission, pass17

**Reviewed**: Independent bindings-authored R4 packet at
/private/tmp/S91-F-X206-composed-runtime-r4-prepared. Full331-line runner,
50-line transition helper,36 unchanged command rows,27 target selectors,
current317 projection, prerequisite and native artifact schemas. This is
incremental runtime admission review, not product completion review.
**Verdict**: 2 defects,0 smells,0 nitpicks.

## Authentication

Canonical guidance, workflow, microscope, approved F206 contract, worker
status/progress and pass16 were read. F206 remains in-progress on
work/f-x206-codex. All2714 byte/size/mode records match actual current source
map e9e7d4be30058031cafeba358cfaf9751cdd36290335c27dce31d5eb130822b4.
Recorded index identity matches. All nine packet file identities match the
sealed receipt362ef5998976b0703cfba783e18e886787c97d04715d10794b4e9478d16a9ffe.
Runner identity is
3cd6d77e8f2db400919ae4be0087476e3bb2cb1539d54416fedd7c1b8dfc472b.
Queue identity is
8c9e4b4bbd8cc35550a19d96b83fa9c6a8a2af6be799f2641fbca85ea83d4301.
Transition helper identity is
df12b1d07755d18e8053571daf56d02fdbbe185946340243fc769742dddcd268.
The317 projection has exact current values, using retained membership only.
All36 original command dictionaries are unchanged. AST parsing of both
helpers succeeded. No helper or product runtime was executed.

## Defects

### D1, actual caps executables are not linked to their version probes

`/private/tmp/S91-F-X206-composed-runtime-r4-prepared/run.py:83`
`/private/tmp/S91-F-X206-composed-runtime-r4-prepared/run.py:224`
`crates/rpptx-render/src/text.rs:4774`

The caps stage authenticates toolPins hb-shape and font-oracle-python,
including their binary hashes and version probe receipts. The actual
HB_SHAPE and FONT_ORACLE_PYTHON environment paths are checked only for
membership in any hashed reference. There is no comparison with those
corresponding toolPins binary paths. Supplying a different hashed executable
through oracleInputs and the environment therefore passes admission while
the version receipts describe executables the tests do not use. This leaves
pass16 D1 incompletely resolved for the two mandatory independent caps
oracles. Require resolved executable correspondence for both environment
paths and their admitted tool identities, as already done for ordinary
Python, Cargo and rustc. Keep physical font and contour input guards.

### D2, consumed transition does not authenticate the reviewed helper identity

`/private/tmp/S91-F-X206-composed-runtime-r4-prepared/run.py:58`
`/private/tmp/S91-F-X206-composed-runtime-r4-prepared/run.py:78`
`/private/tmp/S91-F-X206-composed-runtime-r4-prepared/transition.py:10`

The transition producer correctly binds its helperSHA, runnerSHA, queueSHA
and runtimeReview in its manifest before touching source mtimes. The runtime
consumer hashes the supplied transition receipt and manifest, but does not
compare any of those four manifest identities, or the receipt helperSHA,
with this reviewed packet and admitted review. A completed receipt from a
different transition helper with matching source/head/selectors and plausible
same-byte mtime records satisfies the consumer. The consumer consequently
cannot establish that the recorded invalidation was performed by the exact
reviewed transition, despite declaring that admission prerequisite. Consume
these existing identity fields and bind the actual transition.py file hash.
Retain the current whole-source and actual monotonic-mtime checks. No new
wrapper or generalized producer adapter is needed.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Checked without additional findings

Correctness and contract were checked for all four prior causal findings.
The parsed F200 producer schema corresponds to the actual retained native
runner and receipts. It accepts semantic failed terminals as provenance,
rejects pending receipts and compiler failures, and does not claim that the
partial producer compiled every crate. The conservative27 selectors identify
actual default or declared Cargo entrypoints. This is broader invalidation,
not a minimal producer-graph proof. Exact whole-source/index guards and
monotonic same-byte records remain mandatory.

Pass16 D2 is addressed by exact ordered stage rows, argv/cwd/helper/source/
queue/admission and final guards, log hashes, real count/name checks and
explicit generated installation/provenance rows. Empty prerequisites no
longer qualify. Build/install wheel paths and hashes are linked to the
actual provenance probe. No build, install, collection or104 pass is inferred.

Pass16 D3 is addressed by the completed immutable104 receipt, detached
artifact seal and current317 adapter. Current build/install/acceptance
lineage, all eight unique owner receipts and expected physical artifacts
are linked before the separate native ignored gate. The native consumer's
schema matches the prepared adapter. Its existing .99, physical face,
coordinate and synthesis rejection controls remain strict. No artifact or
native paint success has been observed in this review.

Pass16 D4 is addressed by checking the exact owned group after leader exit,
bounded TERM then KILL cleanup and an ESRCH observation before normal
handback. Persistent groups fail instead of returning a successful receipt.
This was source inspection, not a timeout experiment.

Panics and failure handling were checked. Assertion failures fail admission
rather than becoming gate credit. OOXML has0 findings in this runtime-only
delta, which makes no XML changes. Tests have no additional findings beyond
the admission defects above. Structure has0 findings, with no product type,
trait, module or dependency added. Both producer and transition remain
prepared-only pending actual ROOT authority and immutable terminal records.

Full remaining F206 implementation, source adoption, route paint, member
recovery, provider completion, whole native families, WASM, hash attribution,
publication, HLD and policy remain due. This review does not waive them or
review the reviewer-authored recovery proposal.

## Handback

Only this canonical review file was written. No worker/source/scratch/plan,
Cargo, process execution, product import, cache, mtime, baseline, UI or intake
changed. The pass ends and returns the reviews-only lease to ROOT.
