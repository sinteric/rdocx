# F-X237, all aspects, pass 1

**Reviewed**: Frozen working diff on work/f-x237-codex against Base b271eff8e82a7068e3ba91c111a45a22d55eb215, 9 files, 2500 insertions and 131 deletions.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Not found

All six aspects were checked against the approved plan, cited HLD sections,
current progress, actual source, test oracles and retained execution evidence.

- **Correctness, zero findings.** The native combined provider at
  `crates/rdocx/src/document.rs:22470` retains full item, cell, paragraph XML
  and hyperlink payloads. Physical correspondence at
  `crates/rdocx/src/document.rs:22793` uses source-qualified spans and one
  traversal of each parsed outer table, retaining nested and controlled cells
  without substituting logical grid coordinates. The scalar path at
  `crates/rdocx/src/document.rs:22948` keeps its original checked contract.
  The index at `crates/rdocx-py/src/document.rs:4314` retains every admissible
  old parent until child matching, validates candidates with the legacy
  predicate and selects the earliest old index. Slot and projection classes,
  both-parent cell precedence, unsupported body paths, forward parents and
  checked hyperlink-removal shifts were traced separately. Generated parent
  links are acyclic physical ancestry. No persistent proof cache or historical
  owner accumulation was introduced.
- **Contract, zero findings.** Production changes remain in the two approved
  document.rs files. The sole hidden additive native API has an existing Python
  registry consumer. Publication, generations, selected-owner lookup, ordinary
  Run epoch and first stale cause retain their existing mechanisms. The user
  explicitly accepted the measured fill tradeoff. The former 3x fill failures
  remain failures and are diagnostic, with no replacement threshold invented.
  Open and edit limits remain enforced. Complete Word qualification and the
  remaining scoped riders are still required before completion.
- **Panics, zero findings.** New table identifiers and vector indexes arise
  together from the same bounded physical traversal. Peeked control iterators
  guard their next calls. Dependency-stack indexing is bounded by the generated
  current owner graph, including forward parent references. Namespace frame
  pushes and pops preserve the initial map, and namespace state cannot escape
  an error. Hyperlink shifts retain checked addition. No new unbounded
  recursive route traversal or unchecked external ordinal was found.
- **OOXML, zero findings.** The namespace change at
  `crates/rdocx/src/document.rs:7623` preserves the original decoder and error
  priority, including ordinary attribute errors without declarations and an
  invalid namespace value before a later duplicate attribute. Scalar scope
  includes target declarations. Batch scope retains inherited context and its
  early return before target validation. Empty events retain their distinct
  validation points. Declaration-bearing frames restore shadowed and removed
  bindings after End. Qualified source spans, closure, relationships and opaque
  payloads remain checked. No serializer particle order changed.
- **Tests, zero findings.** The retained original scalar, namespace helpers,
  complete Python registry and first-match predicate provide independent
  comparison paths. The exact-result controls cover mixed slot classes,
  duplicate parents with an earlier child under the later parent, all
  publication variants, shifted and overflowing hyperlinks, physical controls,
  merges, omissions, nested and related-story tables, aliases and foreign
  lookalikes. Namespace tests cover every consumed event boundary, malformed
  attributes and XML, undeclaration, restoration, duplicate requests, empty
  requests and unvisited tails. Installed controls at
  `crates/rdocx-py/tests/test_core.py:6805` and
  `crates/rdocx-py/tests/test_core.py:6845` exercise held owners, stale causes,
  save/reopen, opaque cells, foreign owners, no-op and late allocation rollback.
  Timing controls retain raw samples and independently check saved cell text.
- **Structure, zero findings.** Concrete node and route enums reduce the
  cases in traversal and conditional matching. No new trait, generic parameter,
  dynamic dispatch, dependency, feature, product file or Python method appears.
  Temporary indexes live within one operation. HLD edits are limited to the
  three approved mechanism documents and describe current behavior.

## Evidence inspected

Actual predecessor RED logs show 120 source materializations against a bound
of 1, 2310 scalar traversal steps at 60 cells, and 153 namespace frame copies
against a bound of 2. These are retained in native-red-selected.log,
scalar-red-selected.log and namespace-frames-red.log under /private/tmp/F-X237-.

The current namespace-native-green.log records 5 passed, 0 failed, 0 ignored.
At 60, 120 and 240 cells it records one source build, one owner scan, three
namespace scans, respectively 61, 121 and 241 item scans, and exactly one model
table traversal with each physical cell visited once. Namespace copies remain
2 while requested boundaries grow through 145, 275 and 535. The scalar check
requires one source build, two owner scans and one physical walk.

The combined-py-native2.log records 3 passed, including complete registry
success and refusal equivalence and exact first matches. Its generated 240-cell
control records 494 old owners, 495 new owners, 494 candidate checks, 1230
predicate ancestry visits, 495 parent probes, 747 key insertions and maximum
candidate pool and bucket size 1. This log predates the namespace-only change,
whose independent old-helper differential subsequently passed. A final pure
binding refresh remains a gate, not an inferred pass.

The installed-current-focused.log records 3 passed with no skips. The accepted
release production hashes independently match namespace-performance-provenance.json:
native document.rs 7409ae8cbe59e0be86011180b146441a4af090e65e1560ddcc138f62e4950b9c
and Python document.rs edb4fad3699d136c75002bed356846cdb1ebd38cde762a1d087534e67f53c212.
The original namespace-candidate-timings.json retains fill medians .925389s
and 3.304061s at 120 and 240 cells. The user-accepted-final-timings.json keeps
the old fill comparisons false with enforcement false, while open and edit
comparisons remain true and enforced. The user override is the authority for
accepting this measured tradeoff.

The final archive was independently hashed and measured. SHA256 is
394c7c783f4220cfe2884dd04a2bab43bab482932b8717a7ffc6e93e42c6b311.
The actual and recorded tuple is 3721620 compressed bytes, 51901570 normalized
member bytes and 37 members. All 27 packaged source, test and root README
payload files were byte-compared with current files. The 64-byte tolerance and
10 MiB bound are unchanged. This is measurement evidence, not verified publish.

## Pending acceptance and review boundary

No Cargo job was launched by this review. Full native acceptance, the complete
fresh guarded Word suite, final clippy, WASM, denied-warning docs, policy and
verified patched publication were pending at the review checkpoint. No pending
or predecessor run is credited as their final pass. The worker must finish
these actual gates before preparation.

One temporary diff inspection was mistakenly redirected to
/private/tmp/F-X237-review-py.diff outside the review write scope. This was
reported immediately to the orchestrator, which audited unchanged product
inputs and directed continuation with stdout-only reads. No worker or canonical
product, test, plan or carrier file was changed by this review.

This pass ends when this file is written. The reviewer returns the worker
writer lease to the orchestrator for the remaining gates and preparation.
