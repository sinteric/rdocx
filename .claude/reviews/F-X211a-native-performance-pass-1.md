# F-X211a, native performance, pass 1

**Reviewed**: Frozen source3 retained-qualification correction against the actual preserved source102 document carrier. One file, crates/rdocx/src/document.rs,110 insertions and15 deletions. Other inherited native checked-owner changes are outside this limited pass.
**Verdict**: 0 defects,0 smells,0 nitpicks in the retained-qualification correction. This is not a clean whole-owner or full-feature verdict. The existing duplicate RootSlot::Raw projection defect and maximal85 remain unresolved.

## Defects

None found within this correction.

## Smells

None found within this correction.

## Nitpicks

None found.

## Aspects checked

- Correctness: zero findings. `crates/rdocx/src/document.rs:19722` passes the exact operation's independently scanned inventory by borrow, rather than reconstructing typed admission for each nested item. That inventory is sorted at line11494. The reuse predicate at line12287 requires the actual Word namespace, sdt local name, nonopaque parent, modeled parent route, exact mapped source start, independently admitted ContentControl kind, non-complex full-equals-scan span, enclosing scan bounds and matching actual SDT context. Missing or mismatched entries fall through to the original qualifier at line12318. A typed parent does not authorize an arbitrary raw child.
- Contract: zero findings. The inventory remains tied to the same immutable source and one snapshot operation at `crates/rdocx/src/document.rs:19680`. Existing standalone StoryItemRef.text and link-text consumers supply empty inventories at lines1687 and19762, preserving ordinary qualification. No cross-document, cross-candidate, persistent or per-edit replay is added. XML source, checked replacement, resource quotas and public APIs are unchanged.
- Panics: zero findings. The new source-offset arithmetic uses checked_sub and checked_add at `crates/rdocx/src/document.rs:12295`. Binary search supplies a valid index into the same borrowed slice before indexing at line12299. New production code adds no unwrap, expect, recursive ownership structure or unchecked source slicing. Existing reader and namespace errors continue to propagate.
- OOXML: zero findings. Namespace closure preserves original payload bytes at `crates/rdocx/src/document.rs:7476`. scoped_story_item at line7511 explicitly records added declarations or the inert complex-field envelope. The inverse mapping at line12292 treats the ordinary root at local offset0 separately, and handles other offsets by removing the recorded insertion length. Actual NsReader namespace and parent frames remain authoritative. Alias/default namespace positives and foreign/raw negatives are checked, and no serializer or child order changes occur.
- Tests: zero findings within the correction. `crates/rdocx/src/document.rs:36615` independently expects visibletail, proves actual old qualification count1 versus reused count0, and forces ordinary qualification for wrong context, shifted offset and out-of-range end. Alias and default namespace cases retain that expected text. At line36654, wrong-owner, invalid modeled ID and foreign-parent raw children are compared with the original path and exact source remains unchanged. Malformed XML retains an error. The unchanged finite checked table transaction at line36679 asserts concrete nested table/control types, actual changed terminal text and save/reopen/drop survival. Returning to the original qualifier would fail the new zero-requalification expectations.
- Structure: zero findings. Existing StoryItemSpan inventory, sorted slice, scanner frames and namespace-closure result are reused. No new cache, type, trait, generic, wrapper, file, module, crate, dependency, feature flag or lifetime framework is introduced. Added private arguments express actual same-source inventory and existing excerpt-offset consumers.

## Exact source and actual evidence

The actual pre-edit carrier at .claude/scratch/F-X211a-start/actual-pre-retained-qualification-document.rs has SHA `b987e7c9d689e70b9e14cd5e1c519814be491be9c81d12351a6ba1928737ea6a`. It is preserved actual bytes, not a reconstructed original. The current carrier SHA is `efcc08775d8af1c287fc6bb8fd801a8b3003bf1aab24d312b1f1c93b4a832a2f`. Their exact textual diff was inspected independently, including the iteration changes and all updated call sites.

Checkpoint3 SHA `f96416fd0cd76b551f30730a366183fa8ea54d07d0b54af32e00462ed83da7b0`, source-map SHA `2f5496c7312689eda7f90327c72c26ba01f113da3db45c6ec740f2cf1ba29d65` and driver SHA `25e3fccc1142d3c97c57363cd2ef52c0a4450b48e09603d395a1d8117cb83ae8` authenticate. All3208 tracked entries match bytes, sizes and modes. HEAD remains564bb645e5e93fbc986b86162a27da9d8a31bbe4. NUL-delimited index SHA remains `9fb6f4a525e2252e4db2e26b2074e74d1394d9ee69154d3457171aafa8d36e6b`. Full inherited working binary-diff SHA remains `c3103bfd8847a28f39844963ddf2ddc663e1d4db153629f6a9ad49ce41e04c10`. These guards were checked before and after report publication.

Receipt3 SHA `a757fd39aaf29191de07170a97de267a3fba3cd4f8381265bc7f6c15884c455e` authenticates20 actual exit0, qualified, non-timeout command rows, each with reaped leader and gone owned group. All raw log hashes authenticate. Membership is all-target check, two deciding debug cases in one command, paired release compilation, nine finite transactions and eight distinct semantic executions. The17 direct release executions use2097152-byte stacks and30-second deadlines. Debug/build commands use8388608-byte stack configuration and are not relabeled ordinary-stack controls. Word binary SHA `e60a9b75002ce1fb8d80b8b6a34d6d9b3166331618dfac6b5ec7c5052d30019f` and OXML SHA `cce24518f6f020bb08affe5617d4ebf24c3a342d118efc3dea9fb783f5a37e17` were independently rehashed. No runtime or tests were run by this reviewer.

The unchanged nine table/control cases retain excluded qualification counts0, retained inventory admissionN and per-item qualification0. Actual tables4/controls32 snapshot times are4246 and4408 microseconds. These are recorded observations, not a general complexity or maximal-depth qualification claim. Source traversal and overlapping text output still occur. No quota, timeout or stack enlargement is used to claim the correction.

## Preserved failures and incomplete acceptance

Original driver1 receipt SHA `3163dd689027a44345b2117ac009428192bc353e24e3b2adb97e79542907d8e7` retains all-target compilation exit101. Driver2 receipt SHA `212106d378e87eeaaba7aa49b6db9530f7d15e4be76ce3e491ecfaf284a4f008` retains all-target exit0 followed by deciding-test exit101. Their receipt status remains running after failure, so actual row exits and logs, not that status label, define the evidence. Every recorded original log hash authenticates. Neither attempt is relabeled successful.

The duplicate sdtContent case at `crates/rdocx/src/document.rs:36657` deliberately records existing visiblephantomtail output only to prove this performance change preserves old admission. It is not a semantic oracle granting correctness to the retained duplicate RootSlot::Raw projection. That known source-backed semantic defect remains assigned to childa and must be independently repaired and reviewed. Inherited namespace97 and excluded-owner102 reviews remain separate. Maximal85, public five-owner XML/local retirement, final scoped acceptance, native Office and full F-X211 are not certified here.

The approved childa/parent contracts, canonical workflow/microscope and cited native owner preservation requirements were read. Only this review file is written. Review ends and control returns to the implementer/orchestrator.
