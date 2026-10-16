# F-X229, all aspects, pass 2

**Reviewed**: Independent frozen working diff against e81e3145, 9 files, 1577 insertions and 159 deletions. Rechecked the complete correctness contract and pass1 D1 remediation, captured-owner tests, HLD clarification and final package carriers.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None. D1 is resolved. At `crates/rdocx/src/comparison.rs:5621`, a Word
hyperlink encountered inside a hyperlink returns an explicit unsupported-owner
error before the next recursive call. The initial call supplies false, and the
only nested call supplies true at `crates/rdocx/src/comparison.rs:5634`.
There can consequently be only two active wrapper calls, one paragraph and one
direct hyperlink. The namespace and span readers remain iterative. Active copied
subtrees and indexed positions are linear in the input, rather than growing with
an arbitrary captured hyperlink chain.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness and contract: no additional finding. Direct hyperlinks retain the
approved support boundary, nested captured hyperlinks now refuse atomically,
and the implementation does not add nested-owner support. Table-adjacent marks,
trailing chains, self-closing paragraphs and selected hyperlink cleanup retain
the previously reviewed behavior. The grouping subset remains excluded.

Panics: no remaining new recursive-depth finding. D1's guard does not rely on
the modeled nesting budget. Its actual captured raw-owner route is asserted at
`crates/rdocx/tests/regression_test.rs:25125`. Depth1 remains admitted, while
depth2 and257 receive the explicit refusal for insertion and deletion at run,
word and character granularities. Byte equality after refusal is asserted at
`crates/rdocx/tests/regression_test.rs:25158`. No arbitrary crash depth or
unperformed crash experiment is credited.

OOXML: no additional finding. Expanded-name selection leaves foreign lookalikes
opaque. Direct and aliased hyperlink shells preserve wrapper placement, ordered
content and relationships. Raw comments, processing instructions, meaningful
gaps, retained-table refusal and originally empty hyperlink behavior remain
covered without weakening preservation or schema ordering.

Tests: inspected the actual new RED and GREEN receipts. The pre-fix RED failed
at depth2 because only the later reconstruction postcondition rejected it,
without the required bounded-owner refusal. The source-built raw-owner test
then passes with the guard. Its loop covers all declared depths, directions and
granularities without skips. Existing direct, alias and foreign-control passing
receipts remain distinct. No running broad native, clippy, hash or policy check
is credited as a future pass.

Structure: no additional finding. The boolean carries the two concrete current
owner contexts and directly bounds the existing helper. No new public type,
file, trait, module or dependency. Gap alignment retains its reviewed quadratic
bound and per-gap allocation lifetime. Existing multiple generic instantiations
remain the body/control selectors and inserted/deleted/moved wrapper closures.

## Package evidence

Independently recomputed the current after-carrier archive SHA256 as
fb3d1fb87066d2999b032cb329cc7e6005aa244052aa21184962bf48580aa040,
exactly matching /private/tmp/F-X229-archive-D1-final.json. Its tuple3711139
compressed bytes,51833489 member bytes and37 members exactly matches the
recorded policy tuple. The packaged README exactly matches the current root
README. Before-carrier and pre-D1 receipts are explicitly separate evidence.
The unchanged64-byte tolerance and10MiB limit remain intact. Combined-source
integration still requires its own remeasurement and review.

## Review boundary

Only this review file was written. No implementation, test, plan, progress,
fixture or measurement carrier was changed. The orchestrator resumes the
remaining actual gates and preparation after this pass returns.
