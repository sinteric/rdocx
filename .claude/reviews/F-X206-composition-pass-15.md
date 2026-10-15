# F-X206 and F-X195, composition, pass 15

**Reviewed**: External R3 composition,41 existing carriers,13601 diff lines,
605529 bytes. The incremental R2 to R3 correction is three carriers,
493 diff lines and21093 bytes. The author is bibliography_resume, independent
of this reviewer. The original R2 composition was reviewed by ROOT in pass14.
**Verdict**: 0 defects,0 smells,0 nitpicks in this incremental composition.
No full feature acceptance, source adoption or runtime result is claimed.

## Scope and authentication

Read canonical workflow/microscope, both approved plans, progress, prior
composition pass14 and the complete three-carrier corrective diff. Inspected
current candidate bodies and the retained foundation/provider/highlight seams.
Citations below refer to intended source paths under the unapplied R3
production directory, not adopted worker code.

Packet root is
/private/tmp/S91-F-X206-F195-composed-source-r3-prepared.
Complete diff SHA is
4f28562494a3177138700ea31c5c17046c93364bad851e995c41625a7f975546.
Incremental diff SHA is
0563d23051f5e16be40bbbd02687fb7f191255840651314a8f526f68dfbacbf9.
Candidate manifest SHA is
f93a9ceda57de8b93171ad7c314b44eebbf893707d38fed4dcc615a18a7f8178.
Receipt SHA is
85eb6cc0cc7abac5a0a21afb1936f4154b4220d764dd25fbf6f38bbcce170cf3.

Authenticated all12 receipt artifacts, all reference hashes and all41
candidate hashes/sizes against the manifest and current F206 before bytes.
Only svg.rs, dml.rs and rpptx-render text.rs differ from R2. All other38
candidate carriers are byte-identical to R2. The complete original R2
candidate manifest still matches its retained files. No hardlink alias was
modified by this review.

All2679 F195 and2714 F206 tracked source bytes, sizes and modes match their
R3 maps. Current HEADs remain respectively
2a9bf14072231e46fcb34df70f88cf7aa79fbd79 and
7b1e5ef29464e93ae30e68df1523ce09143578ff.
Actual semantic index bytes from git ls-files --stage -z match each retained
current-index-entries.bin exactly. Their SHA values are
c964c1c38e50c6b8a84c8ea0f8cf0f5f38bd230cd96a8d542ed4c30272c83db1
and c37a36fb85cd0d0055de6dad78d342e20dd2084e45113fc2d1424f7c3338175c.
This establishes current stage-entry equality, not equality with historical
raw index files. Pass14's raw-index representation/provenance qualification
remains. No initial raw-byte equality or unretained initial capture method
is invented. Donor renewal introduces no tracked source change.

## Defects

0 defects.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Resolved pass14 findings

D1 is resolved at crates/rpptx-py/src/dml.rs:532 through745. The second
parameter helper, THEME_COLORS and pymethod block are deleted exactly.
The first highlight-aware implementation remains byte-retained. There is
one parameter helper and theme constant, with separate ordinary and
pymethod impl blocks. FontHighlight remains covered by color reads,
authoring/write dispatch, owner parameter labels, rgb/theme None clearing
and finite brightness publication. The deletion does not select the older
Font-only block or change current F197 extraction semantics.

D2 is resolved at crates/rpptx-render/src/text.rs:53. Language joins the
existing retained-string allocation array and is charged by String capacity
with saturating addition before cache reserve or style clone. Language
remains part of the style key and occurrence request. The native control
at:3960 checks both an oversized-length allocation and a five-character
en-US string with capacity over the16MiB ceiling. It exercises the real
inline/caps and final multilingual shaping path twice in the same manager,
requires no entry/vector reservation, and compares exact accepted glyph
representation, final physical font inventory and requested Aptos
contributions. Physical bytes, face index, coordinates and synthesis flags
are included. This is repeated-manager bypass coverage, not a cache-hit
claim. The existing actual warm/cold caps and provider controls remain
unchanged. Execution of the new control is pending.

D3 is resolved at crates/rdocx/src/svg.rs:3469 through3722. The entire
candidate file is byte-identical to current authenticated F195 svg.rs,
SHA65b45be75daa9803cad8e9ecd8613cd2e4f0e2f7978c5b9090114c5b73506282.
The native mandatory gate again validates all five qualified positioned Thai
owners, actual embedded fvar defaults, implicit default400 eligibility,
explicit700 coordinates on every owner, physical embedded-family/font
identity, source text, x/y owner facts and absence of unwanted synthesis.
The SVG production prefix has not changed from R2. No single substring
stands in for the all-owner proof. This restores the already reviewed
current donor gate without supplying new execution credit.

## Retained seams and six aspects

Correctness and contract: no additional incremental finding. The composition
retains the one selected-instance provider and both bounded trace-aware
caches. It does not replace current F206 owner/geometry code with an older
whole facade. All38 unchanged candidates remain under pass14's source review.

Panics and atomicity: no new finding. The correction adds test assertions,
removes duplicate code and charges allocation before admission. Existing
highlight refusal before publication remains. No new mutation path is added.

OOXML: no new finding. Qualified highlight ownership, raw transform-attribute
refusal and original opaque/source preservation controls remain unchanged.
The SVG-only reader is the exact current donor test helper, not a new
production parser or OOXML serialization policy.

Tests: no new finding. Independently compared all29 Word and25 PPT current
F195 acceptance helper/test ASTs and both typing helper ASTs to the candidate,
all identical. Original104 allocation, mandatory ignored SVG gate and all
caps/highlight controls remain. AST/source equality does not prove collection,
compilation or runtime. Capacity versus length is a genuinely deciding new
cache-admission control, while actual warm-hit attribution still comes from
the separately retained controls.

Structure: no new finding. Existing carriers only, no new dependency identity,
module, crate, generic, trait, provider or registry. One duplicated binding
implementation is removed, rather than retained behind another adapter.

## Limits and handback

Independent review of these three corrections is clean. ROOT still owns
source admission and deciding execution. Neither provider is declared
completed. Full native consumer compilation, installed suites/typing,
physical font and paint placement controls, genuine smcp and fallback
identity, member-local recovery, the actual104 installed success followed
by --exact --ignored native SVG acceptance, WASM, deterministic hash delta
attribution, verified publication/archive/source correspondence, policy and
feature completion remain separate obligations. Existing failed outcomes
remain attached to their original sources. No baseline, archive tuple or
native parity is inferred.

Only this canonical review file was written. No worker, compiler, Cargo,
product import, rendering, UI, cache, mtime, source or intake changes occurred.
The pass ends here and returns the reviews-only writer lease to ROOT.
