# F-X211a, native slot semantics, pass 1

**Reviewed**: Frozen source8 against the actual pre-correction carrier. One file, crates/rdocx/src/document.rs,144 insertions and23 deletions. This pass audits only first typed SDT-content admission and its affected scanner consumers.
**Verdict**: 1 defect,0 smells,0 nitpicks. Ordinary successor9 is still running. No runtime qualification or whole-feature verdict is supplied here.

## Defects

### D1, direct control paragraph inventory enters raw foreign wrappers

`crates/rdocx/src/document.rs:11179`

The tuple scanner inherits opacity and marks later direct sdtContent slots opaque, but never establishes that other ancestors are modeled. A selected qualified w:sdt containing a foreign f:payload with nested w:sdtContent/w:p, followed by its real direct first w:sdtContent/w:p, therefore projects both paragraphs. The foreign wrapper receives opaque=false at line11179. Its nested content does not meet the direct SDT-parent guard at line11181, so it also remains nonopaque. The raw paragraph meets the immediate-content and nearest-control tests at lines11189 and11199.

The native control parser captures that whole foreign direct child as RootSlot::Raw at `crates/rdocx-oxml/src/content_control.rs:1300`. Only the actual direct qualified first START content receives RootSlot::Content at line1282. Thus the scanner inventory disagrees with the real typed owner and exposes a raw descendant as an editable paragraph. The central opaque_story_event route at `crates/rdocx/src/document.rs:10067` does enforce modeled ancestry, so the affected consumer routes can disagree. The existing direct foreign-content fixture at line36676 does not exercise a foreign wrapper around a qualified nested content slot.

This is a source-backed finding, not a claimed executed failure. Reproduce a separate genuine defining RED, then make the direct paragraph inventory respect actual modeled owner ancestry and the same typed-slot boundary. Preserve the foreign payload byte for byte. Include qualified aliases/default namespaces and nested raw controls so a raw nested owner cannot acquire authority by matching a local name. Do not infer admission merely from the nearest SDT or immediate content name.

## Smells

None found in the bounded correction.

## Nitpicks

None found.

## Not found

Correctness outside D1: no further finding in the inspected first-START guard, EMPTY handling or central event integration. Contract: the central rule matches the native first START slot without changing XML source. Panics: no new unchecked arithmetic or untrusted indexing was found. OOXML: central namespace and opaque-source behavior is retained, with D1 the direct-consumer exception. Tests: the genuine second-slot defining native oracle and namespace/save-reopen controls are meaningful, with the raw-wrapper omission covered by D1. Structure: no new type, module, file, crate, trait, generic or revision framework is introduced by this correction.

## Exact scope and limitations

Actual pre-correction body SHA b66edb6b63508b66efc011816a147a42765c91b8f5b5c628d50f3d5ba47a04ed is retained at .claude/scratch/F-X211a-start/actual-pre-slot-correction-5-document.rs. Current source8 body SHA0d91edbe5ea5ecfd6936510b9ee2aff8bb8abdc33666f6e7feb56de76bc61182 authenticates before and after this review. HEAD564bb645e5e93fbc986b86162a27da9d8a31bbe4 remains unchanged. The exact two-body diff and owning native parser were inspected. No test, renderer, application or installed package was executed by this reviewer. Running ordinary9 logs are not claimed as completed evidence. Maximal85, public Word owners, retirement, scoped completion and parent acceptance remain outside this pass.

Approved child and parent contracts and canonical microscope/workflow were read. Only this review file is written. The audit ends here and returns control to a distinct remediation phase.
