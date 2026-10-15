# F-X209, whole working, pass 15

**Reviewed**: immutable source61, 13 complete product carriers against claim `fcbab9c3c1210c071d31471cb1913cc09ee70e7e`, 15,310 diff lines, plus the seven proposed HLD section changes. This is the first whole-feature review, incorporating the earlier bounded passes without awarding unrun acceptance.

**Verdict**: 3 defects, 0 smells, 0 nitpicks. D3 is an inherited provider defect exposed by the current required gate, not an attributed F209 source regression.

## Bound input and evidence

Capsule `/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/whole-working-source61-capsule/receipt.json` SHA `b63db35648fafec97dfe6c6d68d101d0f6de9b54641baec55c8ff7691de87730` authenticated. All 48 declared artifacts match their sizes and hashes. Complete diff SHA `223e7e412089014ce162e42f97edea2da0f01ba5e77597e9104c6b932cadcdbe`. Full 3,131-source map SHA `290596e99b28d1f9ff5e503c4bbeb7bf860faa3c215a1abad3f443a45709f641`. Native facade SHA `078ca7c1d1ebb87c58144b8a44f5853fafffe03feb33a82a22a8f5bd09a3f4ff`.

Read the approved plan, canonical microscope instructions and the referenced DrawingML, PresentationML, binding, rendering, testing and backlog contracts. Checked correctness, contract, panics, OOXML, tests and structure across the complete changed surface. Frozen paths below identify the reviewed bytes, not the continuing worker source.

## Defects

### D1, changing a non-RGB theme choice can discard an unmodelled attribute

`/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/whole-working-source61-capsule/after/crates/rpptx/src/lib.rs:4372`

The replacement admission combines every color choice's attributes into one whitelist at 4374-4378, then replaces the entire selected element at 4386-4389. For example, a qualified empty `a:sysClr val="windowText" lastClr="112233" hue="60000"` passes this guard. `hue` belongs to another color model, is opaque on this owner, and disappears when assignment produces `a:srgbClr`. The previous theme fixes preserved unrelated lexical source and refused edits which discard unmodelled payload. This path defeats that contract even though it rejects unknown attribute names.

Admission must be specific to the selected expanded color owner. A deciding test must use an owner-inapplicable known-schema attribute and assert exact package/source preservation on refusal. Keep ordinary permissive reads and valid absolute RGB replacement.

### D2, physical-owner removal does not prove IDs are unambiguous

`/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/whole-working-source61-capsule/after/crates/rpptx/src/lib.rs:7000`

The guard only compares the first direct child with the selected ID to the selected index. Removing the first of two same-ID members therefore passes. It also does not establish that IDs within a removed group do not occur elsewhere in the owner tree. `collect_subtree_ids` at 7010-7016 and `detach_connectors` at 7047-7049 then act on ID identity, so an ambiguous ID can detach references to a surviving member. The native API must refuse ambiguous ownership before cleanup/publication. This is the same physical scope requirement already enforced for picture replacement, not a request for globally unique deck IDs.

Add direct-first-duplicate and nested collision controls on layout/master owners, retaining complete source, relationship and connector equality on refusal. Check every ID that drives subtree cleanup, rather than only the removed root's first occurrence.

### D3, qualified empty text-body aliases are refused by picture conversion

`/private/tmp/rdocx-f-x209-codex/crates/rpptx-oxml/src/shape_tree.rs:1260`

The genuine current `fx206_picture_placeholder_empty_namespace_aliases_remain_supported` control fails on a qualified `q:txBody` with `d:bodyPr`, empty `d:lstStyle` and empty `d:p`, at the conversion call at `shape_tree.rs:3540`. There is no opaque root attribute in that input. The admission compares the body to a canonical empty reference, while list-style equality includes physical namespace closure, so legal alias-only emptiness is misclassified as payload that a picture cannot retain.

The original opaque-root refusal must remain. Use a qualified source-aware empty-body admission which distinguishes pure namespace closure from meaningful/opaque payload. Do not remove the alias deciding assertion or change global physical equality merely to make this gate green.

Six relevant guard/parser/equality files are byte-exact between claim and current, independently verified against Git blobs and the source hashes in the alias-attribution receipt. That proves unchanged attribution, not a passing claim execution or permission to retain the current failure. This provider defect is separate from the new F209 source defects.

## Earlier findings and remaining surface

The reviewed source contains the earlier selected-source theme admission and lexical edits, absolute RGB transform removal, inherited MC policy closure, actual qualified opaque relationship inventory, owner-specific background replacement guard, and picture exact-one ownership plus selected fallback read repair. Prior passes 1-14 remain the authority for their bounded evidence. None of D1-D2 is closed by those prior controls.

The whole review also examined actual part-backed shape collections and property edits, staged source initialization, ordered master/layout lists, safe imported master/layout/image closure, mapping-only slide relationship updates, complete inbound removal checks, master text-style levels, live theme and script-font views, causal Python owner scopes, font/bullet reuse, public stubs and explicit exports. No additional findings were identified in these areas. No duplicate detached Python model or separate revision map was introduced. The immutable claim already contains ShapeMutOwner::Presentation with a slide index and path at `whole-working-source61-capsule/before/crates/rpptx/src/lib.rs:10481`. Current code extends that existing provider to ShapeOwner, rather than introducing a second representation. The plan's donor-era exact-child description at its line146 is stale against the actual claim prerequisite. An initial provisional concern that this was a newly introduced representation was refuted by the authenticated before carrier and is withdrawn. Retaining and extending the actual causal geometry provider is the applicable prerequisite reconciliation. The F208 dict expectation migration follows the newly approved live ThemeColors mapping contract.

## Supplemental fixture admission

The unadopted footer fixture proposal receipt SHA `2c48c33014fc29a287ffcc74722f684e778bcfad06238fc352020726abf76bd7` and all six declared artifacts authenticated. Diff SHA `cb701b0f31f555cbd5b8e2eecf3cc5a106c6d55f9e1a751ca58376fa7f51ff61` adds only two coherent reopen operations after the fixture privately replaces master package XML. F209 now reads the loaded MasterRecord. Reopening after those private setup writes makes that model coherent while retaining the original policy inputs, subsequent layout model change, presence/materialized paint checks and reopen assertions. Static admission is valid. No successor execution credit is awarded.

## HLD projection

Seven HLD proposals are explicitly external and unadopted. Diff SHA `f4abf984f79e1bf1ae4ac11e2c9821931f2be32ecb9c39f4ac1748015e458139`. Their additions and targeted replacements preserve the prior Word, PPT, caller-font and neighboring story contracts. They describe actual intended reusable branding, physical owners, live theme/style views and selective closure rather than asserting completed gates. Zero additional documentary defects, smells or nitpicks. D1-D2 still need source reconciliation before this proposed intent can support completion.

## Actual qualification and limits

Supplemental native receipt SHA `9067452f8152ea5845c4fafc76d0d423e5318e1ee8d433c27045f10be3519cb3` and all five raw logs authenticated. Four changed-crate all-target/all-feature Clippy exited 0. Support ran 13 passing unit tests and zero doc tests. Native facade ran 146 passing, one failing and five existing ignored tests. OXML ran 37 passing and one failing test. Facade doc tests numbered zero. Full source-after match and natural process cleanup are recorded. These are actual counts, not green whole suites. The footer failure has the admissible unrun fixture successor above. The OXML alias failure remains D3.

Alias attribution receipt SHA `cb63b5744f10f2f7fcc33df0e5fbe96265ba3df9c568f2bc9cbc3689549c30fb` authenticated, including six current-versus-claim blob comparisons and the actual failure log. Earlier bounded Base/GREEN evidence is retained with its own source qualifications. No tests were run by this reviewer.

Current installed wheel, installed API/strict typing/stubtest, native Office/LO and source-derived paint, affected archives, remaining full scoped verification, WASM and current hash qualification are not credited by this review. Their absence is a completion limit, not invented proof of a code defect. Required image-positive and three original branding script acceptance remain mandatory.

## Not found

Correctness has D2. Contract has the required D3 provider failure. OOXML preservation has D1 and D3. Structure: zero additional findings. Panics: zero additional findings. Tests: zero additional defects beyond the explicit deciding gaps for D1-D2 and retained D3 failure. Smells: zero. Nitpicks: zero.

Review ends here. No remediation, source edit, runtime replay or completion claim was performed.
