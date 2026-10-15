# F-X211, corpus contract, pass 1

**Reviewed**: Two immutable R2 specimens, W-XML-02 and W-LIFE-06, their setup/body/assertion identities and the approved parent/child XML, lifetime and deliberate-divergence contracts. Static review only. No specimen, binding, prototype or test was executed.
**Verdict**: 1 defect in the specimen expectations,0 smells,0 nitpicks. No production defect or final acceptance verdict is assigned by this pass.

## Defects

### D1, the same held regular run must both survive and retire on document append

`/private/tmp/S91-F-X211-frozen-corpus-r2/scripts/W-XML-02.py:6`

This specimen captures the regular Run returned by Document.add_paragraph(...).add_run(...). At line7 it appends another body paragraph. At line9 it requires successful text assignment to that same captured Run, then line10 requires successful Run element XML editing and later saved-file assertions.

`/private/tmp/S91-F-X211-frozen-corpus-r2/scripts/W-LIFE-06.py:8`

This specimen captures the same class of regular Run through the same public operations. Line9 appends another body paragraph. Line10 performs a subsequent table append. Line11 requires editing the captured Run to refuse, and lines13/14 explicitly require add_paragraph as the first invalidator while excluding add_table. Line16 fails if editing succeeds.

These triggers are not mutually exclusive. Both captures are ordinary text runs appended to a fresh body paragraph. Neither specimen identifies a special physical owner, different generation policy, XML-only handle, field root or foreign story. Different literal text and keeping a separate paragraph variable provide no legitimate lifetime distinction. The additional table append in LIFE06 cannot be the actual first invalidator while its error truthfully identifies the prior paragraph append as that first invalidator. XML02's ordinary Run.text assignment happens before any element operation, so an XML-only resolution policy cannot reconcile the conflict.

Under the required conservative immediate Run retirement on Document.add_paragraph, XML02 must refuse at line9. Under append survival for the same Run, LIFE06 cannot truthfully report that paragraph append as its first invalidating operation. No implementation should discriminate on specimen identity, text value, retained Python parent-reference count or intended later use to make both appear successful.

## Smells

None found.

## Nitpicks

None found.

## Proposed resolution for root review

Preserve both immutable scripts, their setup/assertions, exact digests and the100-script denominator. Retain LIFE06's truthful first-cause contract. Review and explicitly admit XML02 as a deliberate non-success caused by its stale held Run setup. Do not inject a re-fetch, edit its body, rerun an adapted body as original credit, suppress the failure or convert expected refusal into correct-file success.

`.claude/plans/F-X211-design.md:104` requires deliberate divergences to remain in the denominator and count as non-success. Line108 permits their actionable-error controls to qualify independently. Line501 reiterates fixed identities, no injected re-fetches/coercions and separate adapted evidence. The existing manifest currently declares divergence null for both specimens. This report proposes a separate reviewed reporting decision, not an edit to that immutable manifest or an already approved classification.

Keep the regular Run XML owner as a hard positive/refusal gate independently. `.claude/plans/F-X211-design.md:495` defines regular Run XML and checked roots. `.claude/plans/F-X211a-design.md:27` requires that actual owner, while line43 names the five-owner installed gate. `.claude/plans/F-X211c-design.md:31` explicitly retains all14 XML owners and named lifetime contracts as hard gates independent of the percentage. Therefore deliberate XML02 non-success alone is insufficient for completion.

Use separately named current-source installed Run XML controls on an actually current captured Run. Prove namespace-complete w:r read/write, qualified/default/alias roots, exact opaque/unrelated source retention, save/reopen target facts, wrong-root/field/trailing/relationship refusals, no-op/failure atomicity and supported local retirement. Separately prove that a Run retired by Document.add_paragraph refuses XML/text mutation atomically with the true first cause. These controls are outside the frozen100 scoring identities and cannot earn successful XML02 credit. Root must explicitly reconcile the existing line43 wording and reviewed divergence interpretation before treating this resolution as adopted.

This proposed resolution preserves the approved capability, immutable metric and first-cause safety contract. It does not lower the denominator or remove the installed regular Run XML gate. No prototype or final integrated result is claimed passed.

## Authentication and review aspects

Freeze receipt SHA is `d3cf81e655dfb1f7d35745487d46e2ceb8c3c826167439cc970911eec3ac21f7`. Manifest SHA is `7501e001301a703988526f76c9961b35b345a1c17f8e1d383e9477d5947b22e1`, complete body-manifest SHA is `2f4348dd55dcd1b8889badd8ca407d9f790cf38327c00ec3ae31dd5b02b3f66e` and allocation SHA is `f6adc336ed82bcfa2a3c034e91513ef58543fb322e4de259c1a6d6d2010ebcbf`. Counts remain100 total,49 Word/51 PPT,24 BASE/14 XML/16 LIFE/46 AREA.

W-XML-02 script SHA is `38464cc7008defe79ce99a1d9355d6ddd69640fa3498bb265b6e0f521836e104`. W-LIFE-06 script SHA is `f8b971dbcd1e9650f7512e8872f6773ec652dd15c03cc6a7ac1ee27395e882c6`. Both actual scripts match manifest and freeze inventory hashes/sizes. Their setup, body and assertion payloads match the body manifest and each declared payload hash. Shared support SHA `7d5a7a1896def281cdf176ec6e19c414c46eafb69631d51ebb4c14cfe7f8eb47` authenticates. The original historical Base17/100 is not remeasured or relabeled.

Correctness and contract: D1 above. Panics: zero findings within this static specimen-contract review. OOXML: zero additional findings, and required regular Run positive/refusal capability remains explicit. Tests: D1 is a specimen-expectation conflict, not runtime proof. Structure: zero findings. No new production construct or product file is proposed.

The canonical workflow/microscope and parent/child contracts were read. Immutable script/manifest/allocation hashes are checked again after writing this report. Only this review file changes. Review ends and control returns to root to select or seek approval for the reconciliation.
