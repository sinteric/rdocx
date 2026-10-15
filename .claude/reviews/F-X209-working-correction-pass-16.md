# F-X209, working correction, pass 16

**Reviewed**: frozen source61-to64 correction, three existing carriers and 15,538-byte incremental diff. Scope is pass15 D1-D3, their deciding controls, the previously admitted footer fixture reopens and the necessary standalone iterator compile correction. No confirmation review of unchanged whole-feature categories.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Pass15 D1, D2 and D3 are resolved within this bounded source review.

## Authentication

Immutable capsule `/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/review15-correction-source64-capsule/receipt.json` SHA `614b83b01091fe679b96e4654a014f87414365da1657b2a85bbd1e93a39cf1ad`. All 36 declared local/reference artifacts authenticate against size and SHA. Diff SHA `7150c8a1a72c6269251e33b71cf5c5a3d70c40de8179a8d6ee4e66eae05ff6eb`.

Source61 map SHA `290596e99b28d1f9ff5e503c4bbeb7bf860faa3c215a1abad3f443a45709f641`, source64 map SHA `35207ac5a12be15af1e84841fcf9f0b41888cd251ecdce3e0b624ac4981e0c87`. Exactly shape_tree.rs, slide_parts.rs and native lib.rs differ across the 3,131-entry maps. All before/after copies match those map records. The other 3,128 records remain exact. Native lib.rs source63 is byte-exact source64, SHA `d2eed1a6a421715d22d59532f8e6ad8267d06d285709ac63a5a4e9905864bb09`.

Read the approved F209 source-preservation/physical-owner contract, canonical microscope rules and pass15. Inspected only immutable copies. No runtime or product edits were performed.

## Defect dispositions

### Pass15 D1, resolved: color admission is specific to its actual schema owner

`/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/review15-correction-source64-capsule/after/crates/rpptx/src/lib.rs:4374`

The guard now pairs the selected color kind with its permitted unqualified attributes. sysClr admits val/lastClr, schemeClr and prstClr admit val, scrgbClr admits r/g/b and hslClr admits hue/sat/lum. Namespace declarations remain harmless. Other qualified or owner-inapplicable attributes refuse before replacing the element. Existing nonempty/opaque payload refusal is unchanged.

The added matrix supplies each genuine qualified choice both with permitted attributes and with one misplaced known-schema attribute. It proves the fixture substitution occurred. Invalid choices retain complete serialized bytes, while valid absolute RGB assignment checks the effective theme value and exact saved/reopened stability. Alias and prior opaque/transformed source controls remain. No broad parser restriction or guessed fallback color was added.

### Pass15 D2, resolved: all cleanup-driving subtree IDs must occur exactly once

`/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/review15-correction-source64-capsule/after/crates/rpptx/src/lib.rs:7003`

Removal counts IDs inside the removed subtree and across the owning tree before timing or connector cleanup. It refuses if any removed ID is repeated either inside that subtree or elsewhere in the owning part. The existing traversal at 9783 includes groups and selected compatibility fallback children. Only the validated count keys then become the set driving cleanup. No global deck uniqueness rule was introduced.

The original master/layout picture-owner control now additionally attempts first-duplicate removal and subtree/fallback collision removal. Its existing complete-byte atomic checks enclose these attempts. Original image read/replacement, hyperlink, timing refusal, valid remove/reorder and source/relationship assertions remain. The first-occurrence loophole and nested collision cited in pass15 are closed.

### Pass15 D3, resolved: empty-body admission uses qualified source shape, not physical equality

`/private/tmp/rdocx-f-x209-codex/.claude/scratch/F-X209-start/review15-correction-source64-capsule/after/crates/rpptx-oxml/src/shape_tree.rs:1245`

The local conversion preflight uses NsReader to admit only a qualified txBody root and the empty DrawingML bodyPr, optional empty lstStyle and one empty p in schema order. It ignores namespace declaration aliases and whitespace, but refuses non-declaration attributes on every admitted owner, foreign/rebound children, extra/nested elements, comments and other opaque events. It neither alters global PartialEq nor broadens ordinary parsing.

The original canonical/alias/default empty-body positive and original opaque-root atomic refusal controls are unchanged. The additional six negative shells decide retained root attributes, meaningful bodyPr attributes, rebound list-style namespace, opaque paragraph attributes, foreign siblings and real nested text. They require the precise original refusal and unchanged serialized source. Legal alias-only emptiness no longer inherits the physical list-style closure equality failure.

## Fixture and compile equivalence

The footer-policy test adopts exactly the previously admitted two reopens after private master package writes. All original policy/presence/paint assertions remain, with the native model made coherent before it is read. Its exact deciding selector passed.

The slide_parts.rs change at copied line837 replaces `text.as_ref().iter()` with `text.iter()` for the same whitespace predicate. This removes standalone test type-inference ambiguity without changing the predicate, events or source policy. The original source62 and source63 compiler failures remain retained. No new file, dependency, public API or lint suppression was added.

## Actual evidence and limits

Authenticated source63 attempt2 receipt SHA `349a32a3a1c294336f8d35747ef3a91bbe0a4bf05020713e971b5a9946658afa`: theme, physical-owner picture/link/removal and footer-policy selectors each ran one passing test. Its standalone OXML compile failure is retained, not a test failure converted to success. The receipt binds source63 and safe owned cleanup. Native lib63 and64 are exact, with the two later OXML changes confined to the reviewed guard/test compilation correction.

Authenticated source64 attempt3 receipt SHA `a08a74bd97262c5a7aabf62fb8277916c452d166679fa1728ff0a86603f18c36`: original alias-positive, original opaque-root refusal and new negative-shell selectors each ran one passing test. Source-after match and safe natural cleanup are recorded. All six raw passing logs authenticate. Each is a separate one-test result, not a full library rerun or an installed pass.

Original source61 whole-native failures, source62 missing-local compiler failure and source63 standalone compiler failures are retained as failed history. Current all-target/Clippy successors were pending when this input was sealed and receive no credit here. Installed/fresh-wheel, archives, WASM/hash, native Office/LO paint and whole-current scoped acceptance remain outside this bounded pass. Wheel61 authenticity does not imply current-source installed acceptance.

## Findings by aspect

Correctness: zero defects or smells. Contract: zero defects or smells in the correction. Panics: zero findings. OOXML: zero findings after the qualified empty-shell and owner-specific attribute corrections. Tests: zero new defects, with the six named outcomes qualified only as stated. Structure: zero findings. Nitpicks: zero.

Review ends here. No remediation or full-feature completion claim follows from this report.
