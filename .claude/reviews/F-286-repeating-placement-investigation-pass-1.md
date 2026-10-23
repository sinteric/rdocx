# F-286 repeating type placement, designer investigation, pass 1

**Reviewed**: `sprint/s95.3`, HEAD `f9e0060bd22c372a79683f2b558ddcd7400811eb`. Working status was clean before investigation and remained clean at the last read before this report.
**Reviewer**: Codex, focused investigation session. No implementation or delegated review in this session.
**Verdict**: 1 blocking defect class, existing B2 destination/type-placement admission. 0 separate smells. 0 nitpicks. Remediation belongs to the orchestrator after this report.

This is neither final sprint review pass 5 nor a reset of Q25's authorized bound. Read AGENTS.md, CLAUDE.md, WORKFLOW.md, F-285/F-286 design contracts, relevant HLD03 placement and type provisions, historical B2 sprint/focused reports, the independently clean customXml amendment report and retained progress. CURRENT_SPRINT and BACKLOG mark F-286 done. Those historical completion entries do not discharge this newly demonstrated contradiction. No tracking changes are made here.

The initial customXml payload repair remains independently clean within its reviewed scope. Its recursive payload checking works on the newly examined empty inputs too. The defect is the additional inference that compatible payload proves compatible physical placement of a repeating type.

## Defect

### B2, type publication and relocation omit repeating-type physical admission

Primary citations:

- `crates/rdocx/src/content_control.rs:1553`, item conversion checks section ancestry only.
- `crates/rdocx/src/content_control.rs:1575`, section conversion checks repeating content without checking the addressed shell's physical slot.
- `crates/rdocx/src/content_control.rs:3453`, existing-item recognition precedes ordinary payload admission.
- `crates/rdocx/src/content_control.rs:3460`, ordinary admission tests the payload as Block, regardless of shell placement.
- `crates/rdocx/src/content_control.rs:3134`, relocation extracts content and discards the outer shell's type for admission.
- `crates/rdocx/src/content_control.rs:3011`, recursive SDT admission skips properties, including nested type markers.
- `.claude/plans/F-286-design.md:43`, type changes retain the F-285 shell placement.
- `docs/hld/03-architecture.md:1859`, incompatible destination publication must refuse.

The supplied execution starts with a valid inline rich control inside a paragraph of an existing block repeating item, inside a repeating section. Setting the inline shell to `RepeatingSectionItem` succeeds and writes `w:p/w:sdt/w:sdtPr/w15:repeatingSectionItem`. An enclosing section exists, but the addressed SDT is still inline. This is a successful illegal publication, not a refusal message discrepancy or an inferred failure.

Fresh probes establish the analogous section conversion. A valid inline rich shell contains either a nested empty rich SDT or metadata-only customXml. Each payload passes recursive Block admission because it contains no conflicting concrete child. Section conversion nevertheless creates a repeated item under the paragraph-owned shell. Empty payload compatibility does not turn that slot into a block slot. The same two payloads convert successfully on block shells, which is a necessary positive discriminator against rejecting empty transparent wrappers indiscriminately.

Fresh move and clone probes also start with a valid block repeating section nested inside an ordinary block control. Its existing repeating item contains metadata-only customXml. Moving or cloning that section into a separate host paragraph succeeds, changes bytes and emits an inline repeating section. Its repeated item necessarily inherits the inline slot. The source is deliberately nested to take the actual inline insertion route. Direct body relocation relative to a paragraph is a body boundary operation and would not demonstrate this failure.

Count these publishers once under B2. The payload recursion, ancestry predicate and retained-shell relocation all omit part of the same destination/type-placement invariant. There is no additional duplication smell for the same defect.

## Primary specification and approved boundary

[MS-DOCX 2.5.1.11](https://learn.microsoft.com/en-us/openspecs/office_standards/ms-docx/6b50d4d0-3f5d-4d8d-b507-9879bd342f3e), retrieved during this investigation, requires a repeated-item SDT to be within a repeating-section SDT. It separately limits that item's physical form to block, row or cell. These are independent conditions. The enclosing-section condition does not establish an immediate-parent requirement. F-286 A8 says inside a section at `.claude/plans/F-286-design.md:92`. The shorthand parent wording at `docs/hld/03-architecture.md:1887` must not be used to prohibit intervening legal content or controls.

[MS-DOCX 2.5.1.10](https://learn.microsoft.com/en-us/openspecs/office_standards/ms-docx/b1768ff2-c9fe-4660-86e5-73cd8c94b0a0) requires section content to consist of repeated-item SDTs. It does not itself state a separate inline prohibition in the retrieved text. The section refusal follows from combining that requirement with the item's physical-level restriction: an inline section cannot supply legal block, row or cell repeated items through its inline content slot. This is an inference from the two primary rules, not a quotation of a nonexistent section-level sentence.

Microsoft's [ISO-derived SdtRun documentation](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.sdtrun?view=openxml-3.0.1) identifies paragraph-owned SDTs as inline and lists inline customXml and inline sdtContent among their parent contexts. Thus neither an empty nested SDT nor a customXml wrapper changes the inherited physical level.

F-286 already distinguishes an existing item from a block sequence to wrap at `.claude/plans/F-286-design.md:38`. Keep that distinction. Existing legal row and cell sections/items must retain support. This investigation does not authorize ordinary row/cell sequence wrapping, which is absent from the approved block-sequence conversion contract. Nor does it require validation of arbitrary malformed imported repeating items before preservation, reading or repetition. The demonstrated inputs begin valid and the requested transformations introduce the illegal placement.

## Executed evidence and limitations

All fresh probes used `/private/tmp/s95.3-fx263-py312/bin/python -B`, in-memory ZIP packages and actual public methods. Before each probe batch, the imported extension at `/private/tmp/s95.3-fx263-py312/lib/python3.12/site-packages/rdocx/_rdocx.abi3.so` was hashed and asserted equal to `3958f3a45317ddb22158867a541e42cbb1c7e05a78b6548e458605c9e9e2101f`. No build or installation occurred. These observations qualify that installed artifact, not future repaired source or native Word behavior.

Read `/tmp/s95.3-inline-repeating-item-probe.py` and its `.log`. The recorded execution reports accepted, changed bytes and one inline repeated-item marker, with the same native SHA. The earlier unsuccessful inner-paragraph lookup is not a product observation. The fresh item probe located the tagged addressed control through its own sdtPr, inside a valid pre-existing item, and independently reproduced acceptance with changed bytes.

Fresh observations:

| Input and actual operation | Observed result |
|---|---|
| Inline shell, empty nested rich SDT, set section | Accepted, changed bytes, one repeated-item marker under the paragraph-owned section |
| Inline shell, metadata-only customXml, set section | Accepted, changed bytes, same illegal physical placement, exact customXml bytes retained |
| Block shell, each of those two payloads, set section | Both accepted, changed bytes, one item each, no inline item path. Exact customXml bytes retained |
| Inline and block shells with completely empty content, set section | Both refused with `repeating section has no item`, complete package equality |
| Inline and block shells with a direct run, set section | Both refused with `not a block sequence`, complete package equality |
| Inline rich shell inside valid item/section, set item | Accepted, changed bytes. Defect reproduced |
| Block rich shell inside valid item/section, set item | Accepted, changed bytes. Intervening existing item does not invalidate section containment |
| Valid nested block section with metadata-only item, move and clone into host paragraph | Both accepted, changed bytes, one paragraph-owned section each. Defect reproduced |
| Existing legal block, row and cell sections, set section again | All accepted, retained one item. Block bytes equal, row/cell bytes changed |
| Existing legal block, row and cell items, set item again | All accepted, retained one item. Block bytes equal, row/cell bytes changed |
| Existing legal block, row and cell sections, repeat | All accepted, changed bytes, two item markers each |

This is 21 actual native public calls: 17 successful calls and four atomic refusals, across two bounded probe processes, both exit 0. Exit 0 means diagnostic completion, not that the illegal publications passed their contract. The row/cell re-setting byte changes were observed but not attributed to a specific normalization or treated as semantic failures. These controls checked acceptance and item counts, not exhaustive preservation.

Two additional attempted customXml-ancestor target cases did not find the target in `story_items`. They never called the setter. They supply neither a product refusal nor evidence of valid public addressing for such a descendant. CustomXml ancestor classification below is source-derived. No saved probe or log was modified or created by this investigation.

## Classification and root cause

`parent_slot` at `crates/rdocx/src/content_control.rs:2911` derives a shell's level from its ancestors. It traverses sdt and sdtContent, maps paragraph/inline owners to Inline, table to Table, row to Row, and body/cell/story owners to Block. Table denotes the slot for row-level SDTs. Row denotes the slot for cell-level SDTs. Block includes block content inside a cell. Paragraph placement is still block-level when the shell surrounds a paragraph in a block owner.

`element_slot` at `crates/rdocx/src/content_control.rs:3144` supplies the corresponding destination-owner grammar. Both functions omit customXml from transparent ancestors and return Other there. This is different from `transparent_children_match` at line 2971, which correctly traverses qualified customXml and sdt payloads under an inherited slot. A new physical placement guard cannot simply reuse these ancestor functions unchanged and then blanket-refuse Other, because legal customXml ancestry would become an accidental restriction. The two functions also switch on local names rather than namespace-qualified identity. A physical classifier should recognize qualified Word ancestors, using the namespace flags already present in `XmlElem` at line 2412. No additional foreign-namespace execution failure is claimed here.

`ancestor_repeating_section` at `crates/rdocx/src/content_control.rs:3657` walks all parents and tests qualified Word SDTs with a qualified w15 section marker. That expresses containment, not physical level. Retain this separation. A paragraph encountered along that walk is still an inline discriminator even if a section is farther out.

The customXml amendment at `crates/rdocx/src/content_control.rs:2981` correctly treats its own qualified customXmlPr as metadata. At line 2984 it descends into actual payload, and at line 2991 it descends through SDTs. Empty wrappers fitting several grammars is expected. Tightening that payload predicate to reject all such wrappers would conceal the physical-placement defect and lose legal block support.

The recursive SDT check at line 3002 deliberately skips sdtPr/sdtEndPr as payload. That remains correct for grammar checking. It does not establish that a type marker in those properties is legal at the inherited level. Relocation additionally strips the outer shell down to its content at line 3134, so checking only nested markers would still miss the moved section's own marker.

## Publisher inventory and atomic staging

| Publisher or consumer | Assessment and required treatment |
|---|---|
| `set_content_control_type`, line 1543 | All ten payload variants enter here. Item ancestry and section content branches omit physical admission. Check the proposed type on the retained shell before publication |
| `type_markup`, line 3283, and `replace_control_type_markup`, line 3540 | Private marker construction/replacement, consumed by the setter. They do not know placement and are not separate public defect counts |
| `wrap_repeating_item`, line 3510 | Creates a new typed nested item for ordinary section conversion. Its inherited slot must be legal before wrapping. Payload fit alone is insufficient |
| `insert_content_control`, line 1373 | `plain_or_rich` at line 2121 refuses repeating kinds. It authors only rich/plain placement-specific shells. No independent repeating-marker bypass found |
| `move_content_control`, line 1656, and `clone_content_control`, line 1707 | Republish retained typed shells. Direct block delegation and nested insertion both need destination-aware type/context reasoning in addition to payload grammar. The nested inline bypass was executed here |
| `unwrap_content_control` and `rewrite_control`, lines 1651 and 1775 | Removes one shell and publishes retained descendants. At line 1790 admission checks payload grammar only. Source-derived coverage obligation: evaluate retained repeating types and any section containment actually changed by unwrap. No fresh unwrap failure claimed |
| `repeat_content_control`, line 1600 | Copies the existing first item, freshens identity and appends in the same section. Legal block/row/cell cases executed successfully. This does not turn the preserved-template path into a general imported-schema validator |
| Remove, metadata, value and binding edits | Remove deletes rather than relocates payload. Property edits at line 1761 retain type and slot. Existing value paths do not select a new repeating kind. No independently demonstrated publisher of this physical-placement defect through these paths |
| Python | Type setter at `crates/rdocx-py/src/document.rs:8602` and repeat at line 8617 use the native operations through `edit_content_control` at line 7049. Relocation wrappers invoke actual native move/clone. No independent binding policy should be added. WASM and CLI have no F-286 typed setter |

Native setter work is staged at line 1549. Marker replacement and story writes occur on the candidate at lines 1592 to 1595. `publish_candidate` at line 1739 prepares and reopens before committing. Nested relocation also stages its splices. The defect is successful admission before commit, not partial mutation after a returned error. Reopening well-formed XML does not enforce these physical type constraints. Existing staging is the correct place to retain complete atomic refusal.

Other type variants are excluded from a new repeating-specific slot ban on evidence, not analogy. Microsoft's [ISO-derived SdtProperties contract](https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.sdtproperties?view=openxml-3.0.1) lists all four SDT forms as property owners and includes richText, text, date, comboBox, dropDownList, picture and group. That supports shared properties without imposing the repeated-item restriction on those seven variants. It does not certify every possible payload. Picture retains its existing owned-image/content checks at line 1562. Dates and lists retain their own validated payload rules at line 3242.

[MS-DOCX 2.6.1.1 checkbox](https://learn.microsoft.com/en-us/openspecs/office_standards/ms-docx/3350cb64-931f-41f7-8824-f18b2568ce66) specifies a character-content constraint but does not supply the repeated-item block/row/cell rule. Do not infer an inline-only or block-only checkbox restriction here. This investigation does not claim to qualify that separate content constraint. Citation, equation, bibliography and document-part variants remain outside the authored payload enum under `.claude/plans/F-286-design.md:41`.

## Minimal corrective contract and recommended matrix

Keep the existing payload grammar policy. Add a concrete native check of physical level and relevant section context, using the full source/destination ancestor information. No new public API, general schema validator, trait or file is needed to express the contract.

1. Item conversion requires both section containment and a Block, Table or Row physical slot. Inline refuses even inside a valid item/section. Other refuses only after supported transparent ancestors have been resolved. Do not replace containment with an immediate-parent test.
2. Section conversion must use a physical slot capable of holding legal repeating items. Ordinary block wrapping stays limited to the approved block sequence. Existing legal row/cell item branches remain accepted. Run this physical check before both AlreadyItem and Wrap outcomes, without recursively validating arbitrary malformed imported item contents.
3. For relocation, check the retained outer type as well as retained nested repeating markers at transparent boundaries under the actual destination slot. Resolve customXml/sdt ancestry with inherited context. A real paragraph, table, row or cell remains a physical discriminator rather than being skipped in pursuit of a section ancestor.
4. When a requested edit actually changes the section containment of retained items, evaluate that affected context too. Type replacement can remove a section marker at line 1593, and unwrap can remove its shell at line 1799. This is a source-derived lifecycle obligation, not a newly executed additional failure. Preserve authoring of a section's template shell and the existing A8 operation. Do not retrofit rejection of every temporarily incomplete imported section.
5. Refuse through existing Error::Other/RdocxError staging with complete package-byte equality. Keep metadata, raw XML, existing identities and fresh clone identity behavior. A candidate reopen is supporting serialization evidence, not the placement check.

Recommended regressions in the existing integration entrypoint and Python test file:

| Discriminator | Required observation |
|---|---|
| Valid item/section containing a paragraph with an inline rich shell, set item | Atomic refusal, including a legal run payload. Defeats ancestor-only admission |
| Same setup with a block rich shell inside the existing item, set item | Success despite intervening item/content ancestors. Defeats invented immediate-parent restriction |
| Inline rich shell containing empty nested SDT or metadata-only customXml, set section | Both refuse atomically. Defeats payload-fit-as-placement inference |
| Same two payloads in block shells | Both succeed and retain exact metadata/raw bytes after reopen. Defeats blanket empty-wrapper refusal |
| Bare empty content versus nonempty wrapper with empty payload | Bare content retains existing no-item refusal. Do not conflate item count with descendant text count |
| Qualified customXml, nested sdt and alternating ancestors in each physical slot | Inherit the actual slot. Source-derived cases require a proven public native locator. A failed test lookup is not semantic RED |
| Existing legal block/row/cell sections and items, set/repeat/save/reopen | Retain support, placement, nested content and identities. Repeat adds exactly one fresh first-item copy |
| Valid nested block section or item with ambiguous payload, move/clone into paragraph | Refuse illegal destination type placement even when payload fits inline. Check outer and nested markers |
| Compatible move/clone at legal level and section containment | Success with retained move identity, fresh clone identity and opaque metadata preserved |
| Item moved outside every section, or affected section removed/converted | Discriminate changed containment from unchanged imported preservation. Preserve approved template authoring and A8 |
| Rich/plain/date/list/picture/group/checkbox unaffected legal uses | Retain current supported conversions and variant-specific refusals. Do not spread repeating restrictions by speculation |

The existing A8 test at `crates/rdocx/tests/integration_test.rs:27422` distinguishes section ancestry from a body shell but never contrasts inline and block descendants inside a valid pre-existing repeating item. Its positive fixture initially contains a rich child directly under a section, so it is not the valid initial-state discriminator supplied here. Earlier boundary-reference and customXml matrices test payload grammar, which explains their passing alongside this defect.

## Explicit zeros and handback

0 separate smells and 0 nitpicks. No additional demonstrated panic, layering, raw-byte-loss, binding-surface or atomic-error-mutation defect in this focused investigation. The customXml payload repair is not reclassified as failed. B1/B3/B4 and historical review dispositions are unchanged. No new product choice or direct-parent restriction is proposed.

Only this report is written. No source, tests, plans, existing reports, progress, ledger, state, probe file or log was changed. No build, suite, native UI, agent, commit, push, merge or tag was used. The bounded probes are local API execution, not a Word oracle or final sprint gate. A subsequent source repair will require its own applicable artifact and verification reconciliation. Return control to the orchestrator now, with B2 still blocking.
