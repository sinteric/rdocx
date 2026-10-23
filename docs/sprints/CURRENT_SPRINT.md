# Current Sprint, S95.3

**Milestone**: M24, Modern DOCX authoring completeness.

**Goal**: Author the content control lifecycle before typed values and custom XML
bindings. F-285 creates, inspects, edits, moves, clones and removes controls.
F-286 and F-287 then add typed controls and custom XML stores. The original
three stories retain their 12 to 15 developer-day estimate. The user added
review repairs and the live Issue387 through Issue392 and PR393 through PR396
resolution union on 2026-10-18. The expanded seven-story estimate is 28 to 39
developer-days, not an unchanged sprint-capacity promise.

The 2026-10-22 user-authorized refresh adds Issue398/PR399 and Issue400/PR401,
plus candidate PR397 for existing Issue389. These extend F-X260 and F-X259
without adding parallel stories. Allow another 3 to 5 developer-days
provisionally, making the expanded estimate 31 to 44 developer-days.

## Spec references

- `docs/hld/14-development-backlog.md`, F-285, F-286 and F-287 define the creation, typed-control and custom-XML scope, their dependencies and the round-trip or differential gates. M24's complete stability boundary remains F-310 in S99.
- `docs/hld/02-scope-and-non-goals.md`, DOCX-052, DOCX-053 and DOCX-054 name the form rows these stories close.
- `docs/hld/03-architecture.md`, content-control order, shell identity, data binding and fragment ownership define the structures public creation must reuse.
- `docs/hld/04-opc-and-packaging.md`, the document-owned custom XML graph, item properties and exact retention of unrelated store bytes govern F-287 publication.
- `docs/hld/10-bindings-spec.md`, the present content-control read and replacement surfaces define what authoring may expose and what stays internal.
- `docs/hld/12-testing-strategy.md`, round-trip and differential controls for content controls, bound custom XML and byte-identical unknown stores define the integrated observations.

## The wave

| F-ID | Title | Size | Status | Owner |
|------|-------|------|--------|-------|
| F-X259 | Repair reviewed consistency defects and strengthen acceptance gates | M | done | - |
| F-X260 | Restore live handles and bound structural edit cost | L | done | - |
| F-X261 | Resolve Word package preservation and inspection contributions | L | done | - |
| F-X262 | Resolve PowerPoint authoring and table-render contributions | L | done | - |
| F-285 | Content control creation and lifecycle | L | done | - |
| F-286 | Rich, repeating, and typed content controls | L | done | - |
| F-287 | Custom XML stores and data binding authoring | L | done | - |
| F-X263 | Prepare Word 0.16.3 and PowerPoint 0.14.3 patch releases | L | done | - |

## Sequencing note

Rows are listed in intended execution order, not F-ID order. F-X259 prepares
the reviewed corrections and shared workflow now. Its final integrated checks
remain part of this sprint. Then prioritize F-X260, F-X261 and F-X262 before
new authoring. Their source and binding carriers overlap, so they stay serial:
F-X260, then F-X261, then F-X262. PR heads are candidates and have not been
merged or accepted. F-X260 must not trade owner correctness for speed.

The user requests Grok implementation through the shared commands followed by
an independent Codex sprint review. Use `/run-sprint --reviewer codex` and stop
at the review handoff if Codex cannot be invoked in that environment. The Grok
implementer cannot record its own review as the requested Codex review.

The user subsequently asked Codex to take over implementation and resumed that
takeover on 2026-10-22. Codex is the sole product writer. Grok may propose
mechanical documentation in isolation, subject to Codex validation. Original
worker identities and completed independent reviews remain historical facts.
The original three sprint-review passes remain historical. On 2026-10-22 the
user approved Q25, allowing at most two further independent Codex passes,
numbered4and5, after required checks. No pass is reset or counted as clean.
Native Word is now connected and Q24 accepts the documented native differences.
The sprint remains in integration until its remaining acceptance checks pass. The documented independent CI
producer now supplies authenticated F-X195 inputs locally. Their installed
suite outcomes are recorded separately from native Word qualification.

Within the authoring group, F-285 comes first. Complete and integrate F-285
through its scoped dependency-prefix checkpoint before F-286 starts. F-287
starts only after F-286 leaves the shared content-control files. The three
stories stay serial. F-X249's bounded existing-control fill repair does not
replace these authoring contracts.

## Codex implementation checkpoint, 2026-10-22

The latest instruction changes are committed in61306b3b. F-X260's read-handle,
physical-owner and atomic publication repairs are in8d53993f. F-X259's Issue400
TOC range correction and measured archive refresh are ina4e4deff. Independent
feature microscopes are clean. These are local reviewed checkpoints, not
feature completion or final sprint approval. Run state now retains reviewed
for both stories, while this delivery table stays in-progress until acceptance.
Original consumed worker identities and integration receipts remain intact.

Current source passed6732native tests, with73ignored, both WASM configurations,
minimal layout, workspace lint/docs, README validation and dependency policy.
All49hashes and seven deterministic pixel buffers match. Allseven Issue389
release edit ratios and allthree Issue398 read ratios are strictly below3.
Independent font inputs were rebuilt by the pinned CI producer. The full
installed Word suite passes1061tests and the full PowerPoint suite passes1028.
Clean-tree publication dryrun passed for all22publishable archives, each below
10MiB, largest4900978bytes. The pinned PowerPoint corpus rendered50decks and
421slides with zero dropped shapes. Its visual fidelity trend is51/421slides
at SSIM0.95,12.114percent against an80percent target, which is not met.
Completeness and fidelity are distinct observations. No baseline changed.
Native Word final qualification, the private unreduced Issue390 input and
final acceptance remain pending. No fourth sprint review, push, merge or publication occurred.

## Native Word checkpoint, 2026-10-22

Word for Mac16.114(26101123) now authors the external originals through its
native UI and object model. Two installed external differential tests pass,
including both date formats and distinct first-item repetition with formatting,
identity and non-last removal controls. Word also opened and saved all twelve
library-authored control cases without a repair prompt. Those consumer inputs
are separate from original Word oracle files. Source and output hashes,
producer version, VBA sources and exact observations are retained locally.

The user approved Q24: retain clone-first append and last-item refusal,
document native addressed-item cloning and whole-section deletion as intentional
differences, and use equivalent native operations for comparison. The user also
approved Q25: permit at most two further independent sprint-review passes after
required checks, retaining passes1through3 and every finding. No review is
called clean by that approval. B1 disposition still requires final evidence
reconciliation, and the sprint is not accepted by recording the decisions.

Obsolete sprint build products and completed render output have been removed
from `/private/tmp`, reclaiming818.6MiB. Historical failures, measured outcomes
and artifact hashes are retained in ignored evidence. Active build caches,
qualified font producers, pinned viewers and current release artifacts remain
available for the outstanding gates. Unrelated temporary files are untouched.

## Accepted providers, 2026-10-22

Seven provider stories are complete after actual local and provider-final
evidence reconciliation. F-285's earlier prefix now has its final delivery
entry exactly once. Independent native qualification pass2 accepts the bounded
Word evidence under Q24 with zero findings. The missing-Word condition is
addressed. Original-source PowerPoint regressions and the authenticated PR394
functional-parent read close that story's evidence gaps. Capability rows and
live-owner policy match the bounded completed contracts.

F-X263 is the sole remaining story. Its new-version local wheels and CLIs are
built. Installed Word1063tests and PowerPoint1028tests pass. Exact42Word and
44PowerPoint priority cases pass on fresh Python3.9and3.12 environments, and
both installed typing/stub checks pass. Remaining local and hosted artifact
gates are pending. No global eight-story full gate or final sprint
review is called passed. Q25 allows final passes4and5 only after the required
checks. No push, merge, tag, issue closure or publication has occurred.

The prepared-source local checkpoint now passes6732native tests with73ignored,
workspace lint, minimal layout, both WASM graphs, warning-denied docs, README
compilation and22package inventories,192policy tests with2skips and dependency
policy. All49hashes and seven deterministic pixel buffers match. Independent
F-X263 microscope pass1 has zero findings. Genuine clean-tree verified archives
and actual hosted build-only/downloaded-artifact gates remain pending. This is
an implementation checkpoint, not the final eight-story full gate or review.

Clean-source082aafa9also passes the genuine locked22-package verified archive
dryrun. All archives authenticate that source, match the recorded inventories
and remain below10MiB. F-X263 still waits for actual hosted build-only execution
and downloaded artifact qualification. Its canonical source-availability
checkpoint does not complete the story or authorize tags or publication.

## All eight implementations accepted, 2026-10-22

F-X263 now completes the eight-story delivery union. Actual clean-source
verified22-package dryrun and hosted run39792212063 pass. The hosted run has
27successful deciding jobs, four skipped publication jobs and26authenticated
unique payloads. Downloaded host wheels pass172priority calls over Python3.9
and3.12, plus both strict typing/stub suites. Complete local installed suites
pass1063Word and1028PowerPoint. Native6732pass/73ignored, applicable provider
oracles, bothWASM, minimal layout, lint/docs/README,192policy tests with2skips,
dependency policy,49hashes and seven deterministic pixels are source-qualified.
The advisory421-slide SSIM trend remains below target, not passed fidelity.
Historical checkpoint paragraphs above retain what was pending at those times.

All eight stories are done with no carries. Final independent sprint review
pass4, main merge and sprint closure remain due. Publication has not started.
The complete M24 stability gate remains F-310/S99. Issue390's private unreduced
input stays unqualified and the issue remains open. Later roadmap work has
not started. Fresh final release approval is still required at clean main.

## Independent pass4 amendment, 2026-10-22

Pass4 reviewed178d0b06 and reports one blocking B2 finding, zero should-fix and
zero nice-to-have. B1/B3/B4 are accepted within their bounded contracts.
A valid customXml row wrapper could relocate into a paragraph, and the same
name-only admission reached repeating conversion. The eight completed delivery
entries remain historical implementation records, without new story credit.
Sprint acceptance, close and publication are blocked pending the separate B2
amendment, affected source/artifact qualification and independent pass5.
Old Word wheels and the rdocx archive do not qualify that changed source.
Unaffected observations retain their original source applicability.

## Requested release follow-up, 2026-10-22

The user explicitly retains Word0.16.x and PowerPoint0.14.x for both Rust
crate families and matching PyPI distributions. Release preparation belongs
before the final sprint freeze and review. Publication follows acceptance and
closure. Determine the next available patch in each retained line from actual
registry inventories before fixing the versions. Fresh22crate and two PyPI
inventories select0.16.3 and0.14.3, absent across both complete families.
Recheck absence at final publication. The earlier0.17.0 and0.15.0
candidates were a Codex interpretation and are superseded by this correction.
F-X263 records the approved release-preparation contract. This is an
eighth story under the added release request, provisionally3to5additional
developer-days and34to49total, beyond the normal seven-story cadence.
Implementation waits for actual provider checkpoints. Release preparation
needs its approved plan, exact family version alignment, changelog and
contributor inventory, packaging and build-only CI evidence. This request does not mark any current story done or bypass pending
native Word qualification, integrated acceptance or the approved bounded review.
Only `/release` may publish after `/close-sprint`, with separate final approval
at the reviewed clean main SHA. No version carrier or release tag is changed
by recording this intent.

## Definition of done for this sprint

- Integrate F-285 through its scoped dependency-prefix checkpoint before starting F-286 or F-287.
- F-285 public creation and mutation of block, row, cell, paragraph and inline controls survives round-trip with no raw-XML fallback.
- F-286 typed controls pass the pinned native Word comparison under replacement and documented equivalent repetition. Q24 retains clone-first append and last-item refusal as intentional native-default differences. Authentic Word provenance, version and input identity are required. Generated output is not that oracle. Missing evidence stays pending.
- F-287 bound controls resolve, update and reopen against the correct store, and unknown store content stays byte-identical.
- One integrated verification and a clean sprint review. Do not start S96. M24's complete stability gate remains F-310 in S99.
- Issues 264 and 281 stay open. No release tag is prepared by opening this sprint.


## Live contribution acceptance inventory

Read-only intake on 2026-10-18 found six new issues and four open PRs. Source
bodies and PR identities are retained locally in
`.claude/scratch/S95.3-github-intake.json` and
`.claude/scratch/S95.3-pr-intake.json`, with SHA-256
`320da690d0469135e1515db51d3855e47156120749ffd4b52b389f065e3bb8e0` and
`fd0fc0346fbf7ce2cd713fc24cd1a3051d08ef5c6264c14d8b9ad76e72154d43`.
Refresh and compare remote bodies/heads before using them. The durable contracts
below and HLD14 retain the scope if local intake files are unavailable.

A read-only refresh on 2026-10-18 found issue bodies 264, 281, 377, 381 and
387 through 392 unchanged. PR393 remains
`beb9c448d0d99636a799d95f718acccb9ea55646`. PR394, PR395 and PR396 advanced by
one commit each, and that commit only records archive measurements in README
files and `scripts/readme_doctests.py`. Their refreshed heads are
`e9f9fbe7b86388b8702af5757bdfa7e4e4fc6596`,
`5d481eb53f00926896f9de632115f4cb245fc941` and
`c71708541397f96128dea2dc8b9c1a21c7da498a`. Functional parents remain the
previously pinned heads. No issue or pull request was closed, merged or
commented on.

| Issue or PR | Owner | Acceptance and disposition |
|---|---|---|
| [388](https://github.com/tensorbee/rdocx/issues/388), [393](https://github.com/tensorbee/rdocx/pull/393) | F-X260 | Word cell property edit handles and PPT append owners, including fresh/held/nested and stale-collection controls |
| [389](https://github.com/tensorbee/rdocx/issues/389) | F-X260 | All seven structural-edit scaling rows and range-removal API. PR393 alone does not resolve this issue |
| [387](https://github.com/tensorbee/rdocx/issues/387) | F-X261 | Word custom-property iteration, items and length against actual saved/read imported metadata |
| [390](https://github.com/tensorbee/rdocx/issues/390), [396](https://github.com/tensorbee/rdocx/pull/396) | F-X261 | Six reduced observations plus explicit disposition of the unreduced control-owner diagnostic and disclosed asset reinsert regression |
| [391](https://github.com/tensorbee/rdocx/issues/391), [395](https://github.com/tensorbee/rdocx/pull/395) | F-X261 | All five source/API rows, paired text-box authority and approved public compatibility |
| [392](https://github.com/tensorbee/rdocx/issues/392), [394](https://github.com/tensorbee/rdocx/pull/394) | F-X262 | All nine authoring/render rows, native table growth and intentional paint deltas |
| [398](https://github.com/tensorbee/rdocx/issues/398), [399](https://github.com/tensorbee/rdocx/pull/399) | F-X260 | Read-only paragraph, row-cell and cell-paragraph creation, cold and repeated calls, whole walks, table-position independence, nested/related owners and current-state correctness after edits. Candidate PR399 still fails the reporter's cold-call script |
| [397](https://github.com/tensorbee/rdocx/pull/397) | F-X260 | Additional candidate for Issue389. Preserve all seven strict release ratios and the already approved range signature/error. Its unaddressed Word edit costs and first-fill cost stay acceptance work |
| [400](https://github.com/tensorbee/rdocx/issues/400), [401](https://github.com/tensorbee/rdocx/pull/401) | F-X259 | Bound outline-derived TOC entries by an explicit heading range, through native, Python and CLI rebuild. Preserve outline-only, custom-style and suppressed-outline selectors |

Live API refresh on 2026-10-22 found the existing issue bodies and PR393 through
PR396 heads unchanged. New PR heads are PR397
`d9721353314ba969c05bd9f886bdd24d15d51a8d`, PR399
`34dfd42782de08bc331a5f6c819774720bb60067` and PR401
`62124737c428a77e0c4fec52b56665c67f221e7c`. All target base
`b072b9852d5b17c7bcd19889cef058556ce9b67d`, not the current sprint tree.
Issue398 body SHA-256 is
`4e940897286f91d30bf5c2b54497462e50077e89313262db6ea38eebccaf6124`.
Issue400 body SHA-256 is
`656762fcfaf40919000730cb39099c6d3c0da427cacba45360a9cfcd4fa0836f`.
Both have zero comments at refresh. The existing local intake files now retain
the refreshed issues and the three new PR bodies, file patches and commits.
Their refreshed SHA-256 values are
`a03525cd237b2081c105ffe55f36e189134164ef0f6052977fbaaa0d846c364e` and
`26e3454b3c215f8cbda82e7f805f987cb2e105080c4215cc1bf3492734ad1036`.
No candidate is merged or accepted. No external messages or closures occurred.

No external issue/PR was closed, merged or commented on by this preparation.
`/close-sprint` reconciles them only after acceptance and the integrated main
push, retaining specific contributor credit for hadim. Issues264/281 remain
open, Issue381 stays with the S96 form programme, and Issue377's benchmark
invitation retains its existing disposition without new implementation credit.

## Additional acceptance and handoff

- F-X259 must retain individual old-source RED and fixed-source GREEN cases, a reviewed scoped checkpoint and named final rider debt. Its review report remains historical rather than rewritten as proof of repair.
- Each new contribution plan must map every issue row to an observable acceptance criterion, a test and a final evidence owner. Record partial/unreduced/unavailable criteria explicitly. Do not close a whole issue from PR titles or script exit codes.
- Complete detailed prepare-mode designs for the pending F-IDs before implementation. Resolve API/compatibility questions in one consolidated round. Budget expansion and overlapping source carriers must be visible in the wave plan.
- The six designs for F-285, F-286, F-287, F-X260, F-X261 and F-X262 were approved on 2026-10-18. Slide placeholders iterate by ascending idx. Layout and master placeholders iterate in document order, and idx lookup stays a separate compatibility decision. Percentage rounding matches pinned python-pptx 1.0.2 ties-to-even and leaves unrelated truncation in place. Omitted default-paragraph keywords preserve existing values. An individual None clears one property. Clearing the native paragraph-default object preserves run defaults and unrelated opaque source. Content-control creation validates each placement against its schema. Inline controls hold inline content. Unsupported placement and type combinations refuse atomically. F-286 keeps the full differential contract, and missing authentic Word evidence stays pending. The authoring order is the F-285 prefix checkpoint, then F-286, then F-287. F-X259 stays approved and running, with its repairs and final-gate debt intact.
- Checklist reconciliation on 2026-10-18 keeps Q1 through Q19 answered. Q20, Q21 and Q22 are answered the same day. `part_names` is lexicographic. Content controls use `insert_content_control`, the five setters, `remove_content_control` and `unwrap_content_control`. An imported auto line rule reads back as a float, and assigning a `Length` writes an exact rule. F-X261 and F-285 are approved on completed dependencies. F-286 and F-287 are approved against that provider contract and stay unready until the F-285 prefix checkpoint. F-X260 stays running and F-X262 stays approved. F-286 Word differential evidence remains pending.
- Design-quality pass 1 on 2026-10-19 closes R1 through R3 in the plans. F-286 names `set_content_control_type`, `repeat_content_control` and F-285 removal, and `RepeatingSectionItem` is a payload with a parent constraint. F-287 uses item bytes plus schema URIs, and binding takes the three `CT_DataBinding` strings. F-X261 projects every stored line rule through the existing paragraph getters. F-X262 no longer cites the pre-integration F-X259 tree. Q1 through Q22 stay answered. The Word differential stays pending. F-X260 A5, HLD and microscope stay open.
- Final integrated verification includes the new regression and policy union. After final completion edits, rerun affected metadata-dependent checks before adopting evidence. Codex performs the independent final sprint review, with the actual implementing and reviewing identities recorded.


## Reviewed B2 repair checkpoint, 2026-10-22

Independent F-285 custom-XML placement microscope pass1 accepts the repair with
zero findings. It covers row/cell/block/inline wrappers, alternating depth,
metadata retention, identity, atomic refusal and repeating conversion. The
actual final native3958f3a4 passes the complete1064Word suite, exact42priorities
on3.9/3.12, strict typing/stubs and the expanded wrapper matrix. Native union
710library/411integration/twodoctests is composed from actual successful cases,
without relabeling failed aggregate attempts. BothWASM, scoped clippy,
49hashes/sevenpixels and README inventories pass. Policy union is190passes
and two existing skips, not192passes. The earlier192total field is clarified
in the amendment receipt. A new verified rdocx archive, actual updated hosted
artifacts and final independent sprint pass5 remain due. No close/publication.

## B2 typed-context amendment checkpoint, 2026-10-22

Independent designer investigation found that physical repeating-type placement
and enclosing ownership remain separate from payload grammar. The amendment
refuses inline repeating types, orphaned item markers and non-item replacement
children, including relocation and unwrap. Compatible nested ownership, legal
block/row/cell operations and item-only unwrap remain supported. No new story,
API or completion credit is added. Current nativee07f191d passes the full1065Word
suite, priorities and installed typing/stubs. Native integration acceptance
composes411passed cases and one corrected new case. The failed aggregate
fixture attempt stays failed. Scoped clippy, bothWASM and actual policy190passes
with two skips pass. Focused microscope, current verified archive, current
hosted qualification and final independent sprint pass5 remain pending.

Independent F-286 repeating-context microscope pass1 accepts this amendment
with zero defects, smells and nitpicks, plus five additional installed probes.
Actual49hashes, seven deterministic pixels and README inventories pass. The
clean verified archive, updated hosted qualification and final sprint pass5
remain due. No publication approval or sprint closure is implied.

## Typed-context qualification checkpoint before sprint review pass 5

The B2 customXml and typed-context amendments are accepted after both clean
independent focused microscopes and actual current artifact qualification.
Clean35238e5b verifies the affected rdocx archive. The other21genuine verified
archives retain unchanged inputs and their082aafa9producer. Exact7Word0.16.3
and15shared-PowerPoint0.14.3archives remain below10MiB. Actual build-only
[39820083897](https://github.com/tensorbee/rdocx/actions/runs/39820083897)
executes35238e5b with27successful deciding jobs and four publication skips.
All26unique payloads are authenticated. Fresh installed downloaded wheels pass
172priority calls, both strict3.12typing/stub checks and four additional
placement-matrix calls over3.9/3.12. Prior hosted runs retain their identities.

Current local Word1065passes and applicable unchanged PowerPoint1028passes
qualify installed products. Native acceptance composes6733passes/73ignored,
including710library,412integration and twodoctests for affected rdocx. The
411-case successful subset and separately corrected new fixture case remain
distinct from the failed aggregate attempt. Current lint, bothWASM, README,
49hashes and seven deterministic pixels pass. Actual policy is190passes and
two existing skips, plus four applicable final-state checks. No prior failure
is relabeled. Source correspondence authenticates322unchanged inputs and
three requalified inputs in the325-file manifest. Native Word, independent
fonts, strict cost ratios and bounded corpus observations retain their actual
producers. The advisory SSIM target remains missed and private Issue390 stays
unqualified/open. Original delivery records and eight-story totals remain
historical, with no second completion, effort or velocity credit.

Only independent final sprint pass5 and closure remain before the separate
exact-clean-main publication preflight and approval. No release is authorized
by this qualification. Earlier checkpoint sections preserve what was pending
at those times.

## Namespace amendment qualification, 2026-10-22

The same B2 invariant is repaired through both imported TypeElement parser
paths. Modeled staging preserves the original qualified type and its namespace
scope. Newly authored or changed kinds retain canonical authoring. The clean
independent namespace-identity microscope has zero defects, smells or nitpicks,
including 120 additional alias/shadow controls. The process refinement requires
checking transformations before guards, expanded namespace identity and actual
modeled serialization, with compatible edits and atomic refusals.

Actual source-qualified native acceptance composes 6,734 passes with 73 ignored.
Of these, 4,801 passes are freshly executed for the changed OXML serializer and
its Word dependents. They include 739 OXML units and one doctest, 710 Word
library, 412 integration, 2,457 regression and two Word doctests, plus 480
other Word-dependent cases. Unchanged producers qualify the other 1,933 cases.
The actual fresh installed local Word wheel passes all 1,066 tests, both
42-case priorities, strict typing/stubs and three matrices on Python 3.9/3.12.
The unchanged PowerPoint whole-suite observation remains 1,028 passes. Current
lint, both WASM graphs, all 49 hashes and seven deterministic pixel buffers,
22 README inventories and 190 policy passes with two existing skips pass.
Final metadata-dependent policies are checked after these record edits.

Clean source 01d800f9 genuinely verifies both changed rdocx-oxml and rdocx
archives. The other 20 archives retain their unchanged inputs and actual
082aafa9 verification producer. The exact seven Word 0.16.3 and fifteen shared
OOXML/PowerPoint 0.14.3 archives remain below 10 MiB. Actual build-only
[CI run 39830343186](https://github.com/tensorbee/rdocx/actions/runs/39830343186)
executes 01d800f9 with 27 deciding successes and four publication skips.
Its 26 unique raw payloads are authenticated. Fresh downloaded-wheel installs
pass 172 priority calls, both strict typing/stub suites and six additional
matrix calls over Python 3.9/3.12. Source correspondence authenticates 320
unchanged and five requalified inputs in the original 325-file manifest.

The old-native semantic REDs, first OXML aggregate's stale lexical assertion,
sandbox network failure and all earlier failed attempts remain historical.
The corrected OXML suite has 739 passes. Current native and installed results
are real executions, without relabeling previous artifacts at a later SHA.
Native Word, genuine independent font producers, seven strict edit ratios,
three strict read ratios and bounded corpus evidence retain their actual
producers and applicable inputs. The SSIM advisory target remains missed.
The private unreduced Issue 390 input remains unqualified and open.

All eight original delivery records remain complete with no carries or new
story, effort or velocity credit for this amendment. Earlier checkpoint
paragraphs describe their actual producers and the pending state at that time.
At this qualification checkpoint, final sprint pass 6 and closure are pending.
Their actual outcomes follow in the numbered review and sprint tag. Q26 preserves
passes 1 through 5 and permits pass 7 only if needed. Publication requires
separate approval at the exact clean main merge SHA after closure.
