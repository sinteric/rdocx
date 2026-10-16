# F-X231, all aspects, pass 1

**Reviewed**: Independent working diff against 9a63569a, nine files, 826 insertions and 13 deletions. Includes the parent-owned one-line roadmap assertion correction already committed canonically at 64d1b50f.
**Verdict**: 1 defect, 1 smell, 0 nitpicks

## Defects

### D1, the installed PPT test fails the ordinary complete binding suite

`crates/rpptx-py/tests/test_documented_examples.py:10552`

The newly collected test indexes FX231_INSTALLED_INPUT unconditionally. The ordinary complete Python suite does not establish that variable or generate its native-produced input, so it raises KeyError before testing the package. A selected manual invocation with the variable set cannot prove that the normal mandatory binding suite remains green. Provide a source-built, reproducible input path for ordinary suite execution while retaining the explicit native-import provenance check in the focused installed qualification. Do not hide the test through an unrecorded skip or weaken its run/link assertions.

## Smells

### S1, a test helper only forwards

`crates/rdocx/src/html.rs:4794`

fx231_word_markdown forwards to Document::from_markdown and unwrap without adding a shared invariant or scenario. This violates the repository's no-forwarding-wrapper structural rule and requires another lookup to understand each fixture. Inline the existing native constructor at its call sites.

## Nitpicks

None.

## Not found

Correctness: inspected pending whitespace source properties, trailing and leading collapse, whitespace-only intermediate spans, hyperlink identities, NBSP visibility and explicit-break transitions. No product defect found.

Contract: native-only presentation import remains intact, Word implementation remains unchanged, unsupported implicit layout remains diagnosed, and donor attribution and behavioral deltas are recorded.

Panics: no new production unwrap, indexing or unbounded allocation. Test indexing is over source-built, explicitly checked scenarios, apart from D1's uncontrolled environment requirement.

OOXML: existing serializer owns output. New tests inspect reopened run properties and link targets and preserve opaque extension and untouched template bytes.

Tests: exact donor partitions, broader Unicode spacing, full selected source properties, NBSP-only content, explicit diagnostics and Word/CLI counterparts are covered. D1 blocks ordinary suite acceptance.

## Evidence boundary

Focused native and installed checks were reported by the worker. Broad changed-crate, policy and pinned-oracle checks remain pending. No uncompleted gate is credited by this review. The pass ends here. Remediation and a fresh independent pass are required before preparation.
