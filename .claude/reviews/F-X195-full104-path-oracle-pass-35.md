# F-X195, full104 path oracle, pass 35

**Reviewed**: External R27 three-carrier proposal, 57715-byte complete diff,
prepared runtime helper and retained native SVG gate. Current F195 branch,
status, approved plan, backlog, rendering contract, progress and pass34 were
read. The proposal author is independent of this reviewer.
**Verdict**: 2 defects, 0 smells, 0 nitpicks. No adoption or execution.

## Authentication

All three before and candidate content hashes, sizes and modes match the
candidate manifest. All26 Word and106 PowerPoint top-level test functions,
including decorators, retain exact ASTs. The selected installed matrix stays
Word56 and PowerPoint48. No production change is proposed. The original
R26 compiler qualification and actual compile receipt were inspected.
The native SVG carrier remains unchanged. Historical failures remain failures.

## Defects

### D1, timeout cleanup can leave owned rendering children running

`/private/tmp/S91-F-X195-full104-path-oracle-r27-prepared/run.py:42`

The timeout loop signals the group only while the leader is live and breaks
when that leader exits at line45. A pytest leader can exit after TERM while
an owned painter or Poppler descendant remains. The helper then reports a
terminal failed stage while the group still runs, violating installed
environment exclusivity for the next admission. Capture the owned group at
spawn, perform bounded group cleanup independently of leader exit, and prove
the owned group is gone before releasing the lease. Do not modify a live
helper or infer termination from an observation timeout.

### D2, searchable SVG does not receive the required cubic reference

`/private/tmp/S91-F-X195-full104-path-oracle-r27-prepared/candidate/crates/rdocx-py/tests/test_rendering_threads.py:1795`

The plan amendment requires searchable SVG to meet0.99 against both the
quadratic and equivalent cubic source paint. This call passes only the
quadratic image into the artifact helper. At line1521 that helper records
one expected PNG. The retained native gate at
`/private/tmp/rdocx-f-x195-codex/crates/rdocx/src/svg.rs:3776` compares actual
SVG paint with only that one expected image. A cubic companion PNG pass is
not an actual SVG versus cubic comparison. Carry and authenticate both
references into the native gate and require both actual scores for each
of the eight existing owners. Preserve current five Thai owners, variable
defaults, all source guards and exact one-passed, zero-ignored gate semantics.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Not found

No additional correctness, OOXML, mutation, production panic or structural
finding. The bounded original/static decoder preserves contour order,
implied midpoints, explicit composite restrictions and hmtx advances. The
quadratic-to-cubic conversion is affine equivalent. Independent fixture
placement is compared with actual reachable PDF owner matrices, rather than
fitted to bitmap ink. PDF retains original font-text paint and its deciding
negatives at the unchanged threshold. The shared raster backend limitation
is explicit. These static observations do not qualify the new placement
discriminator at runtime or prove any of the104 cases.

Actual fresh wheels and installed-source lineage, complete collection and
outcomes, compiler qualification guards and eight artifact identities must
still be independently admitted before execution. No boolean admission field
alone constitutes those proofs. Full feature riders, hash attribution,
packaging, HLD and completion remain due.

## Review boundary

Only this review file was written. No source, Cargo, compiler, imports,
rendering, UI, mtime, cache, intake or publication operation ran. This pass
ends here and returns to the orchestrator for a separate remediation phase.
