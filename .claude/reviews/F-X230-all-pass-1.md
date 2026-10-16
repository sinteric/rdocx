# F-X230, all aspects, pass 1

**Reviewed**: Frozen working diff against599e318de614f3c0b2e9fba0a4bcac6a455695a9,8 files,893 insertions and8 deletions. Read the approved design, its cited architecture, bindings, preservation and testing sections, implementation, regression controls, progress and actual package receipt.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, Inline-control grouping omits preserved-child segment cuts

`crates/rdocx/src/comparison.rs:6832`

The new inline-control grouping call supplies zero for every alignment segment.
The caller explicitly admits interleaved Run and RawXml content at
`crates/rdocx/src/comparison.rs:6429`, checks that the two original raw-boundary
inventories agree at6435, then removes those raw children from the run vectors
at6438. That equality check does not make the boundaries safe to cross.

For example, take one inline control containing plain runs `red green ` and
`blue yellow`, separated by a preserved processing instruction, and replace
their text with `cyan magenta ` and `black white`. Both raw-boundary inventories
match, so this path is admitted. Grouping can fold the whitespace and emit all
deletions before all insertions across the processing instruction. The existing
source emitter at `crates/rdocx/src/comparison.rs:6921` copies that instruction
when it consumes the second original run, before the grouped insertions. The
instruction consequently changes its position relative to accepted text, or
the package reconstruction postcondition refuses an otherwise supported edit.
The approved contract requires opaque and marker boundaries to stop grouping
inside controls as well as paragraphs.

Carry the actual inline-control raw boundaries into alignment segmentation,
preserving original default and Run behavior. Add a genuine failing-before
control for a PI and a foreign raw child between changed runs, plus bookmark
and comment markers where admitted. Check successful Word and Character
comparison, boundary-separated revision order, exact physical raw ownership
and both independently reopened resolutions. This is a static finding. No
runtime reproduction or observed crash is claimed by this review.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness and contract outside D1: the strict anchor rule uses joined word
counts across Character units and retains equal one-word anchors. Adjacent
formatting transitions split blocks and prohibit folding. Paragraph shell cuts
include opaque children only for Word and Character, leaving Run cuts unchanged.
The Issue372 test independently pins exact markup and11 revisions, while Run
retains its existing whole-run output. No additional finding is present.

Panics and resources: no new unbounded recursion or additional LCS. Block
construction, fixed-neighbor predicates, whitespace-stretch accumulation and
emission each scan bounded disjoint ranges. Temporary vectors are linear in
alignment length. Indexes come from the same internal alignment and unit
vectors, including the empty-alignment path. No further finding is present.

OOXML and tests outside D1: external hyperlink targets and per-character bold
are independently inspected after resolution and reopen. Paragraph bookmark,
comment, raw XML and PI tests inspect retained bytes and separate wrappers.
Existing field, ignore, drawing and reconstruction controls remain active.
The new inline-control control uses only plain text, so it does not cover D1.

Structure and documentation: concrete private helpers, existing files and
existing comparison types satisfy the structural rules. No new public API,
dependency, module, trait, generic or feature flag appears. HLD edits describe
current intended grouping and preserve the explicit Run boundary.

Measurement: independently authenticated archive SHA256
c4e98c413cac53e35409c987c0b646a90e91bcc7df65077b8188e08e49d618d4
and tuple3720153 compressed bytes,51880402 normalized member bytes,37 members.
The packaged root README exactly matches the working README. The recorded
3720160 compressed bytes differ by7 within the unchanged64-byte allowance.
The actual archive is below10MiB. Before and after phases remain distinct.

## Review boundary

Only this review file was written. Pending native, clippy, hash and policy runs
are not credited as complete. The review ends here and returns control for a
separate remediation phase and fresh independent pass.
