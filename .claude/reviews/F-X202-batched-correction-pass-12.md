# F-X202 batched correction, pass 12

**Reviewed:** Independent bounded correction against immutable batched-correction-r12, receipt985421810a2ef1e16506dbeea0acaed8d1bb308862622eed863197711fe2f83d. Eight complete before/current carriers and incremental diff, approved plan, whole11 findings and retained original failures. Current full source map0df6376ef180fd3e43e0fbb746e018de9be13f68aec5e20634be1a29481c9b5d. No code changes or runtime by the reviewer.

**Verdict:** 0 defects, 0 smells, 0 nitpicks in this bounded correction. Whole11 D1 and D2 are resolved at source within this scope. New installed cases remain due.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Authentication

All45 manifest artifacts authenticate against declared SHA-256 and sizes. All eight preimages match the independently reviewed whole11 source map e03d07eacca427abec1e60a0844e026f82d8ed302789faa9fe1d098c80b88e2d. All eight current copies match the full3148-record map. Receipt HEAD30775a483ae73c06e448dd594b002dbb4ea2881d and index6d68e3492f4a0020077140f47574953b37e8dddaf29248263a37123071e586a7 remain explicitly bound. Every referenced native35/36/37 raw log independently authenticates. This review uses copied carriers, not moving implementation.

## Correction dispositions

### Whole11 D1, physical heading destinations

source/crates/rdocx/src/document.rs:19702 removes the unsupported direct-body/main-part restriction while retaining checked snapshots of source and destination. Existing heading style/outline resolution and ancestry-cycle refusal remain. The complete heading extent now compares physical checked StoryRunRange locations, not a mutable body index. Existing native checked story-range inventory supplies bookmark uniqueness and source order. It rejects unmatched, crossing and duplicate markers before selection.

At :19773 the new bookmark uses add_story_bookmark with the actual destination owner. Source append and destination marker creation remain inside a cloned candidate, and :19793 prepares/reopens before committing. Invalid source text or tooltip cannot leave the new bookmark in the original document. Existing foreign/stale owner checks are not replaced or bypassed. The previously checked physical source provider and the Python owner checks remain unchanged. The correction does not broaden the inventory to opaque unsupported owners.

The new native control covers header, footer and physical table-cell destinations, exact bookmark ownership, repeat reuse and save/reopen. Existing main-heading and related-source controls retain their original assertions. New installed header/body-cell/footer-cell cases exercise the public owner route but are unrun in this capsule.

### Whole11 D2, external tooltip XML refusal

source/crates/rdocx/src/paragraph.rs:1146 validates the tooltip before appending any run or hyperlink. The typed method returns Result, and source/crates/rdocx-py/src/paragraph.rs:529 propagates refusal out of the candidate edit before prepare_scopes, result allocation or publication. The existing no-tooltip add_hyperlink wrapper retains its return contract, with a justified expect for the absent tooltip only.

The native related-owner reconstruction at document.rs:20691 sets source provenance from the checked original paragraph owner. It preserves actual relationship admission for same-owner reconstruction rather than claiming a foreign fragment origin. The selected source is already qualified by story_range_paragraph_snapshot. Valid related external links use their actual owning part relationship. Invalid-tooltip errors preserve the original package and revision because candidate allocations are discarded. New native control inspects escaped valid tooltip, owner-local relationship and reopened text as well as byte-identical invalid refusal. Installed body/header handle checks remain pending.

### Explicit-size malformed picture refusal

source/crates/rdocx/src/document.rs:19423 checks recognized image type and actual raster probing before dimension selection, staging or relationship allocation. This aligns Document/Cell import with the existing checked Run route. Admitted explicit-dimension vector paths are not falsely subjected to the raster probe. Unknown bytes and truncated PNG refuse even when dimensions are supplied. The native deciding control covers body, header and cell package equality. New public document/run/cell held-owner controls are unrun.

### Split preservation and retained refusal boundary

source/crates/rdocx/src/table.rs:1098 removes the blanket continuation-content refusal. This is consistent with approved-plan.md:366, which requires each original member's content at its leftmost resulting cell. The actual algorithm keeps every original member in place, clears merge properties, changes admitted widths and inserts only new empty cells beside it. It does not copy original paragraph, bookmark, control or opaque content into new owners.

Existing formatted_empty_cell at :713 refuses unmodelled cell-property cloning. Topology preflight, bounded vertical-region lookup, legacy hMerge refusal and post-topology validation remain. The complete document split route stages and reopens before publication, and an ordinary-cell no-op returns without committing. Original tcPr producer payload remains on its original owner, while unsafe formatting needed by a new cell refuses atomically.

The old negative for distinct continuation text is correctly migrated into the plan's supported positive. It now proves once-only origin and producer text, leftmost retention, empty new cell and reopen. The additive continuation control proves bookmark/control IDs and opaque producer payload occur exactly once, retains original content and tests repeat no-op and exact reopened output. Existing opaque-property cloning and legacy horizontal-merge negative assertions remain deciding controls. This is payload preservation, not ambiguous cloning or lost origin.

### Fixture and diagnostic changes

The original asymmetric-cell fixture receives only its missing one-column tblGrid. Count mismatch, error and package assertions remain. The accurate diagnostic adds the word alternate to the existing paired text-box topology error and preserves the assertion's intended branch. Neither correction changes a tolerance or replaces a deciding failure with generic success. HLD08 states the actual approved per-member split preservation and leaves other contracts intact.

## Actual evidence and limits

Native37 receipt a1b818379f0a188672d3a36f4f1c646cbd633aa82cd836c7a0c3dc0a92972c4d authenticates six terminal rows: combined native/binding check0, four exact controls each1PASS0FAIL0IGNORE, and changed native/binding all-feature Clippy0. All source guards pass and all owned groups terminate naturally without signals.

Native35 separately retains four passing exact controls for supported heading destinations/external tooltips, malformed story pictures, existing heading links and merged continuation text. Their relevant implementation and integration carrier bytes are identical to source37 except the single named asymmetry diagnostic operand. Reversing that exact text operand reproduces source35 document hash. These are qualified retained passes, not fresh source37 executions. Native35 and36 original asymmetric fixture failures remain red evidence with safe cleanup, followed by the actual source37 successor.

Original public28 and whole regression failures remain retained. The eight newly added installed nodes have no fresh installed result here. No current wheel, native Word/LibreOffice, corpus, deterministic hash, archive, full scoped-suite or feature-completion acceptance is granted by this pass. Stable whole11 review remains bounded authority rather than a renewed confirmation audit.

## Aspect results

Correctness: 0 findings. Contract: 0 findings. Panics: 0 findings. OOXML and raw preservation: 0 findings. Tests and evidence: 0 findings. Structure: 0 findings. No new product file, dependency, module or detached ownership model is introduced by this correction.
