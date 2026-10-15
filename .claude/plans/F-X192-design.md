# F-X192, Complete the remaining Word bibliography catalogue

**Status**: approved
**Sprint**: S92
**Size**: L
**Depends on**: F-282

## Scheduling

The user parked this unfinished S90 carryover at S92. Preserve its worker
branch, dirty worktree, reviews and native evidence. S91 issue and PR repairs
continue without requiring this story to complete. The contract is unchanged.

## Problem

The measured native foundation refuses standard non-APA sparse or non-ASCII
operands at `crates/rdocx/src/bibliography.rs:12169`. Localized APA sparse,
repeated and corporate branches refuse at `bibliography.rs:12005`. Sorting
rejects non-ASCII source operands at `bibliography.rs:7781`. Standard selected
and empty bibliography output refuse at `bibliography.rs:2701` and `:2845`.
Citation modifier, multiple-source and script branches also remain unfinished.
These cannot become unsupported-producer cache retention.

## Spec reference

- `docs/hld/02-scope-and-non-goals.md`, citation/bibliography boundary DOCX-049.
- `docs/hld/03-architecture.md`, What stays put, native bibliography ownership and physical field caches.
- `docs/hld/04-opc-and-packaging.md`, The package, bibliography source graph and atomic publication.
- `docs/hld/08-rendering-spec.md`, Word field caches and deterministic script rendering.
- `docs/hld/10-bindings-spec.md`, native bibliography and Python/WASM preservation boundaries.
- `docs/hld/12-testing-strategy.md`, Word corpus, measured bibliography gate.
- `docs/hld/14-development-backlog.md`, F-X192 complete catalogue contract.

## Approach

Retain the existing public API, concrete formatter ownership and staged
transaction. Extend existing bibliography.rs, field.rs and existing regression
entrypoint. No new dependency is selected. No new module, file, trait, generic
or feature flag is needed for known formatting consumers.

Complete the thirteen remaining dense APA selectors from each own completed
native result:1054,1105,1107,1108,1109,1111,1112,1113,1115,1121,2117,2128,2145.
Do not infer neighboring grammars. Complete all seventeen source kinds and223
ordinary selectors across the eleven other styles, retaining pinned editions,
rich properties and IEEE table geometry. Independently vary sparse/repeated
operands, contributors, corporate/plural names, missing dates and mixed scripts.
Complete grouped citations, suppression, locators, prefixes/suffixes, numeric
assignment, disambiguation, multiple-source/empty selection and all locale
precedence/application-default/filter branches.

Derive actual sorting strings and comparator discriminators before extending
bibliography_sort_key. The installed Harvard XSL assembles Last+initials for
up to four people, Year/no-date and Title without delimiters. APA uses its
formatted contributor representation and source-kind-specific title keys.
IEEE uses numeric RefOrder. Existing tuple Last/Title probes do not establish
those string keys. Retained ICU probes tested partial operands and tuples,
not complete Harvard or APA SortingString values. Their failure does not prove
complete-key ICU failure. The separately authenticated complete-key ICU
counterexample and same-key native controls below establish bounded results.
Default ICU, Core Foundation and ASCII fallback remain prohibited.
The approved implementation route is source-kind-specific native sort-key
assembly plus an independently demonstrated portable comparator and stable
source/citation tie policy. No comparator backend is presumed approved.

Before comparator production edits, run the bounded native key/host study:
fixed first/middle/year/title with varied last names, then reverse title and
vary year independently. Distinct Tag/Guid with equal semantic keys and
independently reversed source/citation order distinguish ties. Concatenation
collision pairs distinguish one string from tuples. Harvard and APA controls
must record exact identities, actual selected style, completed F9 and normal
reopen. Retain numeric styles' separate encounter-order authority.
If a new dependency is demonstrated necessary, revise this plan with exact
versions, data and all dependency riders before adding it. Missing exact
comparator evidence blocks completion and dependent work, not independent
sprint repairs. It is a technical implementation barrier, not a request for
the user to guess a library or waive parity.

Authenticate the existing census rather than repeating broad captures:
`/private/tmp/S90-F282-full-catalogue-oracle-readiness-census-20261009.json`
records223 selectors,17 kinds,3791 records per style,1475 file bindings and
zero census errors. It does not prove arbitrary-input, sparse/context or
comparator parity. The paired Thai/Tibetan Book capture completed F9 and
normal reopen with equal output hashes. Its finite four-source/eight-field
control can be independently audited. The exact own1054/own1105 dense compact
controls now have native F9/save and ordinary no-F9 reopen authority, as
recorded below. Their original prepared working files remain untouched.

Keep source identities, raw XML, namespaces, physical begin/separate/end
owners, rich paragraphs and IEEE cells/grids. Stage the entire update through
update_bibliography_transaction at `bibliography.rs:16186`. One unfinished
recognized standard owner aborts the whole candidate. Preserve locked and
producer extension contracts. Prove public source mutation, repeat update and
library save/reopen against completed controls.

## Rejected alternatives

- Alias adjacent locale grammars or substitute modern style editions.
- Lookup expected native caches in production or execute producer XSL.
- Treat incomplete standard branches as unsupported extensions.
- Assume a Unicode-capable comparator is Word-equivalent.
- Repeat the full dense capture matrix despite existing indexed authority.

## Test plan

**Test gate**: differential, full_bibliography_catalogue_matches_pinned_word.

| Category | Test | Asserts |
|---|---|---|
| differential | full_bibliography_catalogue_matches_pinned_word | Every style/kind/selector row has exact identity, instructions, selection, order, text, rich properties and reopen semantics, without skips/assumed aliases/cache fallback |
| differential | bibliography_sparse_and_repeated_operands_match_word | Missing/repeated properties, roles, names, dates and scripts match independent controls |
| differential | citation_groups_and_modifiers_match_word | Grouping, suppression, locators, prefix/suffix, disambiguation and numeric interaction |
| differential | bibliography_sort_discriminators_match_word | Exact keys, locales, normalization, independent source/citation order and stable ties |
| differential | bibliography_locale_precedence_matches_word | Source, field, omitted/default-zero, explicit default and filtering remain distinct |
| regression | one_unfinished_bibliography_owner_preserves_every_owner | Mixed-owner failure retains package, caches and revision |
| round-trip | bibliography_mutation_and_repeat_updates_reopen_consistently | Public CRUD, repeat update and ordinary save/reopen retain published identity and cache |
| round-trip | bibliography_namespace_aliases_preserve_opaque_xml | Equivalent prefixes produce equal output with retained opaque bytes |
| integration | ieee_bibliography_preserves_table_and_story_geometry | Rich labels/grid/cells and first/interior/end paragraph context |
| golden | bibliography_render_uses_matched_content_and_deterministic_fonts | Declared finite script controls compare matched content and actual rendering |

Pin Microsoft Word for Mac16.113.2 build16.113.26092012. Bind source/native
output hashes, actual update/reopen provenance and normalization classification.
Use existing test files and source-built inputs. No cache implies completion.

## HLD impact

- `docs/hld/02-scope-and-non-goals.md`
- `docs/hld/03-architecture.md`
- `docs/hld/04-opc-and-packaging.md`
- `docs/hld/08-rendering-spec.md`
- `docs/hld/10-bindings-spec.md`
- `docs/hld/12-testing-strategy.md`
- `docs/hld/14-development-backlog.md`

## Risk routing

- Parser/serializer: HLD04/HLD06, schema order, qualified owners and exact
  opaque subtree retention.
- Layout/shaping: HLD08, deterministic fonts, actual finite script controls
  and declared independently reviewed rendering deltas.
- External oracle: exact Word pin and completed-operation source-bound evidence.
- Public API: existing additive pre-1.0 foundation, package dry-run and10MiB
  archive checks. No new public API is planned.
- Crate graph: existing facade dependencies only. No reversed format edge.
- Any earned collation dependency requires design revision with exact MSRV1.93,
  WASM, no-default, graph, data/licence/provenance, supply-chain and packaging
  riders before implementation. No dependency is selected by this approval.

## Hash harness

Expected unchanged for the existing49 entries. Diagnose unexplained deltas.
An intentional change needs separately labelled, declared and reviewed evidence.

## Exclusive file claims

- `crates/rdocx/src/bibliography.rs`
- `crates/rdocx/src/field.rs`
- `crates/rdocx/tests/regression_test.rs`
- Exact listed HLD sections, oracle capture and all Cargo/test execution

## Implementation checklist

- [ ] Authenticate census and completed locale/script owners.
- [ ] Finish remaining own dense APA grammar consumers.
- [ ] Complete eleven style families and sparse/contributor branches.
- [ ] Complete citation, selection and application context.
- [ ] Derive exact style keys, comparator and independent tie authority.
- [ ] Revise before any newly justified dependency.
- [ ] Pass full catalogue plus atomicity, mutation, reopen and rendering gate.
- [ ] Run scoped verification and independent zero-finding microscope.

## Open questions

No unresolved product decision. The user authorized the full deferred contract.
The comparator and own-locale native evidence are explicit technical execution
barriers. This approval does not permit approximation, a new dependency or
completion before those barriers are resolved.

## Bounded native key and tie evidence

Word16.113.2 build16.113.26092012 authenticated ten existing name/title/year
controls and six new four-source Harvard Anglia and APA Sixth controls.
The new controls expose Publisher identity markers outside both installed
sort-key builders. All six explicitly completed F9 and preserve exact bytes
through normal reopen without F9. Their nine field instructions and four
source trees survive, apart from native RefOrder. Immutable input hashes,
complete artifact hashes and installed style hashes are recorded in
/private/tmp/S91-F192-key-tie-study/capture-receipt.json and independently
checked before recording this plan evidence. The read-only study appendix is
.claude/scratch/F-X192-native-key-comparator-study.md.

For both styles, baseline identity order is19,20,17,18. Source reversal yields
20,19,18,17. Citation reversal retains19,20,17,18 despite reversed RefOrder.
These controlled equal-key pairs therefore retain source collection order.
Do not use Tag, Guid or citation encounter order to break their ties. This
result is bounded to the tested Book operands and contexts.

Installed Harvard SortingString at HarvardAnglia2008OfficeOnline.xsl:6608
concatenates contributor representation, Year and Title without delimiters.
The two distinct operand combinations Boundary+A+2020+Collision and
BoundaryA+2+020+Collision produce BoundaryA2020Collision. Their native tie
reverses with source order. This disproves the current independent operand
tuple as a sufficient Harvard key. APA uses its formatted contributor and
source-kind-specific title/year assembly at APASixthEditionOfficeOnline.xsl:5826.
Its tested personal/corporate pair is source-stable, but exact intermediate
formatted-key byte equality was not independently extracted.

The existing17-name controls show style-specific mixed-script order and
suffix-sensitive name-equivalent pairs. They do not license a shared raw
Last-name comparator. Portable comparator parity, nonempty/repeated initials,
all source kinds/locales and remaining dense grammar branches are still
unproved. No dependency or production change is authorized by this finite
study. Preserve the existing exact-evidence implementation barrier.

## Complete-key corporate host discriminator

Four additional native controls vary Harvard/APA and forward/reversed source
collection order while holding citation order fixed. Preserved corporate and
title whitespace makes each style assemble the same complete source-established
keys: ' 王 2020 Same title' and ' Иванов 2020 Same title'. Native Word preserves
the exact leading/trailing U+0020 source operands. All four complete results
place the Chinese-key source before the Cyrillic-key source regardless of
collection reversal. Unique selected entries authenticate source identity.

The full capture receipt is
/private/tmp/S91-F192-complete-key-host-study/capture-receipt.json. Exact raw
source XML, UTF8 keys and assembly facts are complete-key-audit.json in that
same folder. Artifact and installed-style hashes were independently checked.
All four explicit native style/F9 results are byte-identical to their normal
no-F9 reopen/save. All five field instructions and source trees survive,
apart from native RefOrder. Generated controls were saved and closed and the
existing APA6 study restored unchanged.

The earlier Harvard-person Cyrillic-before-Chinese order disappears under
these equal complete-key inputs across styles. Therefore an unconditional
style-specific script bucket is unsupported. Key/context assembly can explain
the prior difference, but this two-key intervention proves neither a strict
comparison sign nor a universal host/backend rule. Complete-key portable
comparator evaluation remains unproved. Retained partial-key ICU failures must
not be misreported as complete-key failures or used to reject a backend that
has not actually received these operands. No production dependency is selected.


## Completed compact APA1054 and1105 authority

Exactly the two existing prepared compact controls were copied, updated
through actual APA Sixth Edition selection and F9/save in Word16.113.2,
then normally reopened and saved without F9. Both whole packages are
byte-identical across normal reopen. Evidence remains outside canonical at
/private/tmp/S91-F-X192-own-dense-completion/study.md, audit-receipt.json,
capture-receipt.json and native-UI-actions.json. Root reverified all24
receipt-bound artifacts. Original immutable inputs, prepared mutable copies
and the existing unrelated study window remain unchanged.

Each completed control retains all18 original instructions and physical
field owners, all17 exact qualified sources apart from native RefOrder,
and uniquely identified entries through exact Title or PatentNumber.
Both citation/entry text sets and entry order match the historical dense
capture. Thai1054 retains all414 rich result runs exactly. Tibetan1105
retains564 runs, exact text/order/boundaries/pPr, but216 complex-script runs
replace Arial Unicode MS with Kailasa across all17 entries. Every other
recorded property matches after this one-name replacement. Do not claim
that compact context caused it or make Kailasa a universal style default.
Use the newly completed rich control as its own authority, preserving the
historical typography distinction. SourceLCID remains absent, separately
from explicit fieldLCID1054/1105 and the UI's English language label.

Saved SHA256 values are624799886a170e40fc12b36be97e86e014f2d4aff44a18d8319363560daf87c1
anddc54ffb47d9046aad001192f925996dd74e7b0b329302a0d00dc11b6f0a14e95,
respectively. This finite completion resolves prepared-only provenance
for these compact controls. It does not retroactively authenticate the
historical recovered dense document's execution, prove arbitrary context
or demonstrate a portable complete-key comparator. Those barriers remain.


## Genuine complete-key ICU counterexample

Bounded complete-key research now measures installed ICU C78.3 against
actual fully assembled corporate SortingString operands. This evidence is
separate from the earlier partial-key3900-option grid. Root authenticated
all five artifacts under /private/tmp/S91-F-X192-complete-key-comparator.
Full source identities, native/reopen hashes, key bytes, runtime/API/header
hashes and resolved locale/options are in measurement-receipt.json,
SHA2561ab6125d27fe5bdb1ce88643491497c2a6dfc4d165bcfed9d007c31bf5ad90b9.

Both installed styles produce exactly the same two full keys in the
corporate controls: " 王 2020 Same title" and " Иванов 2020 Same title".
All four completed Harvard/APA forward/reversed-source controls put the
Chinese source first. ICU en_US resolves valid locale en_US/actual root,
with its recorded default warning. Default Tertiary, targeted Primary and
Identical strengths with normalization enabled each compare the first key
strictly Greater and reverse the native order. Source-stable equal-key
handling cannot explain that strict reversal. This rejects those three
measured configurations of this runtime, not every portable backend or
ICU4X. Do not infer an unmeasured style/script bucket or universal locale rule.

The bounded36 comparisons include five genuine complete Harvard21-source
vectors, three Harvard identity/tie vectors and these four corporate vectors.
There are27 failures and nine tie-vector matches. Complete-key concatenation
collisions and source-stable tie behavior remain separately established.
APA person author0 depends on runtime locale formatting properties, so its
sixteen localized non-Case operands per own control remain explicitly
unproved. Corporate and Case assembly is genuine, not a guessed author key.
No dependency, production comparator, baseline or native recapture follows.
A demonstrated complete formatter/comparator route remains required.

## Exact locale input and same-key author controls

The authenticated installed resource search found no standalone APA locale
packet for1033,1054 or1105. The eight-artifact metadata manifest under
/private/tmp/S91-F-X192-author0-provenance has SHA
ab535351e2591426cf58c38097d7626131d0495fcd83e6e5b2b8a3c4f9444d65.
The exact person-formatting dependency is twelve runtime properties and
DefaultLCID, not only four name-order formats. Installed literal proximity
does not establish their binding. Do not adopt guessed English defaults.
The prepared independent diagnostic style is inert. Installation into the
signed Word application bundle is rejected. No supported current Mac user
discovery route or equivalent diagnostic/native APA packet is established.

Two additional native Harvard controls use Corporate operands that assemble
exactly the earlier person keys, Unicode strings ИвановA2020Same title and
王A2020Same title. Native F9/save places the Cyrillic source first for both
forward and reversed source collections, agreeing with the authentic
21-source person vector. Each complete package is byte identical after
ordinary no-F9 reopening. Raw operands and installed assembly establish key
bytes, not direct observation of a transient SortingString. There is no
author-kind inversion in these exact controls. Earlier whitespace corporate
keys are different complete strings and cannot establish such an inversion.

Root authenticated all47 artifacts under
/private/tmp/S91-F-X192-byte-identical-author-kind. Its final manifest SHA is
35219161dd1c712e2fc14748a5e6bf5ee6a6e39b3065f90524b23add84b33ec8.
The installed Word pin remains16.113.2 build16.113.26092012. No production
comparator, dependency or script ordering follows from these observations.

Root also tested the installed public Core Foundation comparison API against
the twelve retained complete-key vectors. Plain, localized, case-insensitive
localized and nonliteral case-insensitive localized comparisons under en_US,
en_GB and the actual en_GB host locale each match only three tie vectors out
of twelve. The144 measurements retain36 matches and108 differences under
/private/tmp/S91-F-X192-corefoundation-probe. This is a read-only candidate
rejection, not a portable backend or a production dependency. APA person
keys remain outside the complete-key comparator measurements.

## Further bounded complete-key comparator probes

Independently authored generic XSLT1.0 sorting with installed libxslt1.1.35
was measured against fourteen retained complete-key vectors. Omitted lang,
en-US and en-GB each match only five of fourteen, across forty-two actual
exit0 invocations. Installed Harvard and APA sort nodes omit lang. FieldLCID
alone does not establish the host comparison locale. Two additional ICU78.3
en_GB hypotheses, explicit lower-first Tertiary and Shifted Quaternary, also
match only five of fourteen. Resolved locale and warning facts are retained.

Root authenticated all twenty-three retained artifacts under
`/private/tmp/S91-F-X192-portable-full-key-followup`. Generic probe source is
independently authored. No proprietary stylesheet was copied or executed.
The five matches are three tie vectors and two same-key author controls.
All five twenty-one-source and four whitespace corporate vectors still fail.
These reject only the measured settings and runtimes. They do not prove a
portable comparator or rule out unmeasured backends. APA person-key runtime
properties remain a separate evidence barrier. Continue independent stories
while identifying exact collation authority or a demonstrated candidate.

## Host comparison authority and bounded diagnostic

Read-in-place current Word load metadata links MSXML. Its imports identify
CompareStringW, GetThreadLocale and GetLocaleInfoW from WLMKernel. Microsoft
MS-XSLT2.2.11 C0016 identifies Windows MSXML sorting through NLS. This is a
concrete authority lead, not proof of Mac bibliography call arguments or
Mac and Windows equivalence. No binary or producer stylesheet was copied.

Root selected an isolated flags0 comparison diagnostic using the documented
Windows signature as an explicitly unproved Mac ABI hypothesis. The actual
child exited0 within its twenty-second timeout in the default sandbox. All
694 calls returned1,2 or3. Contextual LCIDs1033 and2057 each match only five
of fourteen complete native vectors. The same three ties and two same-key
author controls match, while five person and four whitespace corporate
vectors differ. Source keys, UTF16 units, identities and every return remain
in `/private/tmp/S91-F-X192-host-comparison-authority/host-measurement.json`,
SHA256d7ea8117a025fa05270ebbc52f43cf5950f7d15fe487f11f6fadd65024535a70.

Actual bibliography LCID, flags, preprocessing and transient sort keys remain
unobserved. Successful isolated execution does not establish a supported
Mac ABI. No production comparator, dependency or signed-bundle change follows.
Further probes need concrete sorting-call authority rather than guessed
flag or locale sweeps. Exact APA person-key assembly remains separately open.

## Bounded full-key encoding discriminator

Independently authored unsigned UTF16-unit and signed UTF8-byte ordering
each matched five of fourteen complete native vectors. Actual subprocess
exit0, twenty-second timeout unused, exact input bytes and all twenty-eight
measurements are authenticated at
`/private/tmp/S91-F-X192-encoding-discriminator/invocation-receipt.json`.
Both fixed representations order Cyrillic before Chinese in both contexts,
while the genuine whitespace corporate controls reverse that native order.
This rejects those two simple encoding shortcuts only. It does not identify
actual runtime preprocessing or a portable collation algorithm.

The missing authority remains transient actual SortingString bytes bound to
the native comparison identity, locale/flags, lengths and returns, separately
from exact APA person author0 provenance. Static imports and isolated host
probes do not establish those bibliography call parameters. No producer
stylesheet execution, native process attachment, signed-bundle modification
or speculative dependency is selected by this diagnostic.

## Bounded static comparison branch follow-up

Root authenticated the pinned WLMKernel SHA, selected4654-byte static
CompareStringW range and isolated LCID127 flags0 probe artifacts under
`/private/tmp/S91-F-X192-comparison-abi-static`. The corrected static read and
twenty-second child each exited0. Fourteen exact complete-key vectors still
yield only five native-order matches. Earlier failed reader options and
retired expanded output retain their actual receipts. No binary was copied,
producer stylesheet executed, native process attached or application changed.

Static provider dispatch, UTF16 loads and an invariant-locale branch do not
identify the actual bibliography call, provider, flags or transient operands.
This is another bounded rejection, with no production comparator selection.
Exact APA person runtime properties remain a separate completion barrier.
The documented custom-style route is Windows-specific and does not establish
Mac discovery or an arbitrary style-path setter. No custom style is installed.
Further work needs concrete native authority instead of repeated locale grids.

## Current Mac native member capability, 2026-10-10

A bounded read-only Word UI check found both Bibliography.BibliographyStyle
and Options.BibliographyStyle in the installed Object Browser. The property
signature is String. Two Immediate Window getters returned the displayed
Harvard Anglia style name while the existing APA6-year-varied study remained
open. No setter, stylesheet installation or execution, document save/edit,
macro injection or security change occurred. The agent observed an incidental
SaveEncoding panel change, so whole-application state equality is not claimed.

The3349-byte capability receipt is retained in the existing diagnostic
directory. Root authenticated its input design, input receipt and all four
original file identities. Getter values are native UI observations retained
in the tool transcript, not an independent observation of APA transform
parameters. This establishes actual installed getter support, not absolute
path loading, style discovery, Locals equality, original comparator arguments
or F-X192 acceptance. The inert independently authored diagnostic remains
unexecuted. Any next qualification needs a concrete bounded reversible route
and its own review before execution.
