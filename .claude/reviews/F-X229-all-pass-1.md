# F-X229, all aspects, pass 1

**Reviewed**: Independent frozen working diff against e81e3145, 9 files, 1476 insertions and 159 deletions. Read the approved plan, cited specification sections, all five original PR374 correctness commits and the complete implementation, tests, HLD and measured package carriers. Grouping commit7ebb5120 remains excluded.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, preserved nested hyperlinks enter an unbounded recursive wrapping path

`crates/rdocx/src/comparison.rs:5626`

The newly admitted whole-paragraph hyperlink path recursively calls
`wrapped_children` for every Word hyperlink it finds, with no depth or work
budget. Its expanded-name scan also copies and indexes the complete remaining
subtree on each level at `crates/rdocx/src/comparison.rs:5576` and
`crates/rdocx/src/comparison.rs:5607`. A paragraph containing a chain of nested
`w:hyperlink` owners around one deepest run therefore produces arbitrarily deep
Rust recursion and retains a quadratic number of indexed XML positions across
the active calls. Inserting or deleting that paragraph can exhaust stack or
memory instead of returning an atomic comparison error.

This route is admitted, not covered by the existing modeled-owner nesting
budget. `crates/rdocx-oxml/src/text.rs:13143` captures nested hyperlinks as raw
children. With no direct run or revision on the outer hyperlink,
`crates/rdocx-oxml/src/text.rs:9297` retains the complete owner in paragraph
`extra_xml` at line9312. The model admission traversal at
`crates/rdocx-oxml/src/document.rs:166` walks runs, controls, revisions and ruby
runs, without counting that raw hyperlink chain. `paragraph_xml` consequently
returns the preserved chain to the new recursive wrapper.

Bound or iteratively process this admitted raw route before recursive wrapping,
or refuse unsupported nested hyperlink ownership atomically. Add a source-built
depth-boundary and beyond-boundary control exercising the actual captured raw
owner path. The existing direct hyperlink and alias controls do not establish
this bound. This finding follows the concrete parser and call path by source
inspection. No process crash or particular crash depth is claimed as an executed
experiment in this review.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness and contract: no additional defect found in next-kept-owner selection,
trailing deletion chains, self-closing paragraph opening, simple-field refusal,
selected hyperlink cleanup or retained-table paragraph removal. Foreign direct
lookalikes and meaningful raw gaps are handled by the recorded hardening. The
grouping algorithm and its intended F-X230 behavior are not imported.

Panics: D1 is the identified new resource failure. No additional indexing,
arithmetic or unwrap defect found. The gap-kind matrix preserves the existing
quadratic alignment bound. The original align matrix is released when align
returns before gap pairing allocates its matrix. Gap matrices are released per
flush, and the bounded owner fixture checks ordered coverage and compatible kinds.

OOXML: no additional child-order, hyperlink wrapper nesting, namespace-selection,
opaque-gap or property reconstruction finding. Cleanup preserves originally empty
hyperlinks and retains opaque resolved inner content. Paragraph removal before a
retained table refuses content and raw gaps rather than silently dropping them.

Tests: inspected genuine Base failure logs and the focused passing successors.
The comparison filter reports52 passes and1 existing ignore, paragraph filter72
passes, and the focused table and resolution filters each1 pass. Independent
reopened link text, target and anchor assertions supplement comparison
postconditions. Source-built property and table controls cover the named repairs.
D1 lacks a corresponding depth control. No broad native or policy gate still
running at the time of this review is credited as passed.

Structure: no new public API, dependency, file, module or trait. The new generic
helpers have current multiple instantiations, the body and control selectors and
the insertion, deletion and move wrapper closures. No additional structural
finding beyond D1's unbounded recursive route.

## Package evidence boundary

Independently read the regenerated current archive. Its normalized tuple is
3710289 compressed bytes,51830060 member bytes and37 members, matching the
recorded policy tuple exactly. Its packaged README exactly matches the current
root README. The current archive SHA256 is
4a2ca1887a016605724e1b6fd3ae7a819169bae64d7dc5eb44fc091fb8c4d7e7.
The worker explained that receipt41a885ee records the preceding archive before
the mechanical README row update. Those phases must remain distinguished in the
completion evidence. No threshold or performance claim was weakened. This
feature-local archive is not combined-source integration evidence.

## Review boundary

Only this review file was written. No source, test, plan, fixture, measurement
carrier or progress record was modified. Remediation and its independent next
pass belong to the orchestrator after this review returns.
