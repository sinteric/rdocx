# F-X206 and F-X195, composition, pass 14

**Reviewed**: Sealed external R2 composition,41 existing carriers,13567
unified-diff lines,601733 bytes. This is an incremental composition audit
against both current donors and prior reviewed provider, foundation,
geometry and caps/highlight contracts. It is not another acceptance review
of unchanged donor implementations.

**Verdict**: 3 defects,0 smells,0 nitpicks.

## Scope and authentication

The unapplied candidate root is
`/private/tmp/S91-F-X206-F195-composed-source-r2-prepared/production`.
All intended source citations below refer to files and lines in that root,
not adopted F206 source. Complete diff SHA256 is
c16157956cac4010242220ea93043867e7de4d9d3f17eb71ac5e3fd836976054.
Receipt SHA256 is
51d6f0c979903de201a9d7658f3babed51f9541ebbc0292dbfedc398ceb331ba.
Candidate manifest SHA256 is
7b16ed1f7102213785de940395bb2e1fd9f4a4353a3997ab3692c09a90986537.

Read the workflow and microscope contract, both approved feature contracts,
relevant provider/namespace/render/binding obligations, progress and prior
foundation, geometry and caps/highlight reports. The specific incremental
questions are shared selected-instance provider reuse, literal/consumer
closure, source-owner preservation, cache/caps composition, namespace-safe
highlight publication, public binding merge and retention of deciding tests.

Independently authenticated all receipt artifact hashes and all41 candidate
hashes/sizes against the manifest and current F206 base hashes. All2679
F195 and2714 F206 tracked bytes, sizes and platform modes match their retained
maps at the beginning and end. Their HEADs remain respectively
2a9bf14072231e46fcb34df70f88cf7aa79fbd79 and
7b1e5ef29464e93ae30e68df1523ce09143578ff.

Current semantic index serialization from `git ls-files --stage -z` exactly
matches each retained current-index-entries.bin. These digests are
c964c1c38e50c6b8a84c8ea0f8cf0f5f38bd230cd96a8d542ed4c30272c83db1
and c37a36fb85cd0d0055de6dad78d342e20dd2084e45113fc2d1424f7c3338175c.
They also coincide with the initial identity fields named indexSHA. Raw index
file hashes use another representation and their difference does not itself
establish changed tracked entries. The packet does not retain an initial
semantic-entry byte capture or establish that field's original capture
command. Therefore no stronger initial/current index-byte equality is
claimed. ROOT must resolve representation/provenance at source admission,
without treating a raw metadata hash difference as changed source bytes.

## Defects

### D1, the merged colour binding duplicates definitions and leaves a new variant unmatched

`crates/rpptx-py/src/dml.rs:532`
`crates/rpptx-py/src/dml.rs:547`
`crates/rpptx-py/src/dml.rs:744`
`crates/rpptx-py/src/dml.rs:758`
`crates/rpptx-py/src/dml.rs:764`

The candidate retains the new highlight-aware parameter helper and colour
pymethod block, then appends the older helper, THEME_COLORS constant and full
rgb/theme/brightness pymethod block again. Identical module constants and
inherent methods cannot coexist. The second parameter helper also has no
FontHighlight arm, although that enum variant is declared at line413.
This is a concrete compile-blocking reconciliation error, not a runtime
uncertainty. Rustfmt parsing and Python AST checks cannot detect Rust name
resolution or exhaustiveness. Reconcile to one canonical colour implementation
while retaining current F197 colour semantics and the new highlight owner,
clear and parameter-qualified error paths. The older duplicate setters also
lack highlight None handling, so choosing them would lose the new contract.

### D2, retained language allocations bypass the bounded shaping-cache budget

`crates/rpptx-render/src/text.rs:48`
`crates/rpptx-render/src/text.rs:142`
`crates/rpptx-render/src/text.rs:177`
`crates/rpptx-layout/src/lib.rs:421`

ResolvedRunStyle now owns an arbitrary Option<String> language, selected from
retained imported lang metadata. It participates in style equality and is
cloned into each cache entry. shaping_cache_entry_bytes charges family and
hyperlink strings but omits language. Consequently an otherwise tiny run
with a language string larger than the cache byte ceiling can be stored
while cache.bytes reports only a small allocation. Distinct oversized
languages can multiply this across4096 entries. Both provider contracts
require the bounded trace-aware cache to retain actual string allocations.
Charge this new owned string and add an oversized-language bypass control,
including a capacity-sensitive case. The existing oversized control at
text.rs:3941 uses hyperlink_url and cannot discriminate this omission.
Retain language in the key and occurrence requests rather than dropping
language-aware shaping to recover the budget.

### D3, composition rolls back the current mandatory variable-SVG owner proof

`crates/rdocx/src/svg.rs:3551`
`crates/rdocx/src/svg.rs:3566`

Against current authenticated F195 svg.rs, this candidate deletes the
namespace-qualified variable_owners reader and the entire all-five-owner
proof in the mandatory installed-artifact gate. It substitutes a single
svg.contains(selector) assertion. The removed proof independently validates
embedded fvar defaults, implicit400 eligibility, explicit700 coordinates on
every owner, physical font hash, actual positioned Thai owner text/x/y and
absence of synthesis/filter/textLength. The current accepted default400
output intentionally has no explicit wght CSS selector. Reinstating the old
string predicate can reject that legitimate authenticated default and lets
one matching substring stand in for all five owners at700. This is a loss
of current deciding coverage and the already reviewed source-default
correction. Preserve the current F195 native artifact gate exactly when
composing the provider. Retained Python function AST equality does not cover
this Rust-only regression.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Six aspects and retained seams

- Correctness: D1 and D2. No additional incremental correctness finding.
- Contract: D2 and D3. No additional scope finding. Whole F195/F206 completion
  remains explicitly pending.
- Panics and failure atomicity:0 new incremental findings. Highlight candidate
  validation precedes assignment and writer output. Existing invariant
  expects in the retained owner suffix are not newly introduced here.
- OOXML:0 new incremental findings. Reviewed qualified highlight ownership,
  foreign lookalikes, transform-attribute refusal, empty/started source and
  schema ordering. The pass13 refusal correction remains present. No new
  source-preservation waiver is accepted.
- Tests: D3. No additional incremental test defect identified. Cold/warm equal
  text caps controls preserve occurrence requests, original logical clusters,
  final physical identities and contribution contexts, but omit the D2
  adversarial language allocation.
- Structure: D1 duplicates existing machinery. No new trait, module, generic
  or dependency identity was introduced by the composition. The documented
  finite FontManager shaping seam retains the existing TextSegment model.

The layout/PDF instance provider consumers and all current FontData literal
carriers are included. Output/conformance/PDF font/raster/writer and common
Word engine donor bytes are exact current F195 bytes. The shared font caps
candidate keeps the current selected-instance provider and original physical
FontData coordinates/synthesis fields. There is no second provider engine.
The entire current F206 owner-address/publication suffix and native bound
Story equality suffix are byte-present in their corresponding candidates.
Current causal shape/support/context carriers outside the packet remain
unchanged. Word staging visibility is intentionally public for the binding
consumer and does not discard current F212 visibility source.

Independently compared all29 Word and25 PPT F195 helper/test ASTs plus both
typing helper ASTs against current F195. They are identical. This establishes
retention, not fresh collection, execution or acceptance. Lock comparison
preserves all302 package identities, versions, sources and checksums. The
candidate lock equals current F195 and introduces only the already approved
existing dependency-list edges. Marker request attribution, final rich-run
requested-family context, language occurrence projection and timeline/notes
render diagnostics were inspected as source routes, without paint credit.

## Limits and handback

No candidate source is adopted. No Cargo, compilation, product import, wheel,
installed test, rendering, UI, cache/mtime transition or intake action ran.
Prior runtime results concern their original frozen sources. Notes/handout,
diagram and timeline independent placement/paint, real selected-face smcp,
the explicitly executed ignored contour gate, all104 installed provider
controls, the mandatory eight-owner SVG gate, typing, WASM, hash attribution,
publication/source correspondence, policy and full feature gates remain due.
No archive tuple, baseline or native parity is inferred from this review.

Only this canonical review file was written. This pass ends and returns the
reviews-only writer lease to ROOT. Remediation is a separate phase.
