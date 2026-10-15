# F-X195, full104 path oracle, pass36

**Reviewed**: Independent bibliography-authored R28 packet, full69359-byte
four-carrier diff, R27 precursor and R28 delta, installed104 runner,
retained native SVG gate and independent physical/HarfBuzz source records.
**Verdict**: 2 defects,0 smells,0 nitpicks. Prepared only, no execution.

## Authentication and scope

Canonical workflow/microscope and approved F195 contract, status/progress,
pass35 and proposed oracle amendment were read. F195 remains in-progress.
Current worker HEAD is2a9bf14072231e46fcb34df70f88cf7aa79fbd79.
All current2679 source records match the actual f135a8bee authority.
All four candidate before identities match actual files. Every artifact in
the sealed receipt matches its content hash. Receipt identity is
7a571c85fa3f384743947f4bff2cd0642e77527cf17ef343632fbf3a0e227dc6.
Complete diff identity is
70abd9105a267acf006ca17da19cf4a9d146a5689a5f3d7ca1505b93cd0f7eaa.
Runner identity is
48f35bb8a6f1d91806721df39f2f5769bb586bfe1450ade687985be3954c3344.
All26 Word and106 PPT top-level test ASTs and decorators equal R27.
The expected adoption map has no difference outside the four candidates.
The source projection has317 records. Native SVG production prefix is exact.
These are source/preparation facts, not compiler, installed or paint credit.

## Defects

### D1, variable reference compares scalar pen positions with offset glyph origins

`/private/tmp/S91-F-X195-full104-path-oracle-r28-prepared/candidate/crates/rdocx-py/tests/test_rendering_threads.py:965`

The new mandatory reference compares each expected SVG scalar x with the
HarfBuzz accumulated advance plus that glyph's dx. The retained five-owner
static SVG has scalar pen positions before mark positioning. Its nonzero
HarfBuzz mark offsets therefore make this precondition fail even though the
existing physical static SVG is valid. Equal scalar/glyph cardinality does
not establish identical scalar-position and glyph-origin semantics.

Independent read-only recomputation from the authenticated retained Thai
HarfBuzz receipt and raw-svg-r21-root-1/static-400.svg finds glyph8 actual
171.744 versus proposed171.936, glyph13 actual216.504 versus216.408, and
glyph17 actual256.704 versus257.880. At700, glyph11 actual208.728 versus
208.392 also differs, with dy4 font units. These exceed the required
0.000001 precision. Both variable artifacts stop before Q/C rendering, so
this packet cannot complete the unchanged104 matrix and eight-owner gate.

Qualify the actual source scalar/cluster pen-position representation
separately from glyph contour origins. Apply independently retained HB
advance and dx/dy exactly once when painting the authenticated static glyph
IDs. Preserve complete text/cluster order, all five owners,400 defaults,
700 explicit coordinates and physical source checks. Do not relax the
precision assertion, remove marks or replace the independent path oracle
with product geometry. No helper, HB or painter was executed in this review.

### D2, interruption bypasses owned-group cleanup

`/private/tmp/S91-F-X195-full104-path-oracle-r28-prepared/run.py:63`
`/private/tmp/S91-F-X195-full104-path-oracle-r28-prepared/run.py:69`

Timeout and normal leader-return paths now correctly inspect the captured
owned PGID independently of leader liveness. The child.wait block catches
only TimeoutExpired. KeyboardInterrupt or another exception during wait
escapes to the outer BaseException handler, which writes failed status and
rethrows without cleaning or observing the owned group. The detached pytest,
Poppler or painter group can remain live after the wrapper exits and reports
failure. This still violates installed-environment lease handback safety.

Use the same bounded captured-group cleanup for exceptional exits after
spawn, record its actual disappearance/reaping result and preserve the
original error. A failed observation must retain an explicit do-not-release
qualification. Do not infer cleanup from leader exit or a failed wrapper.
No interruption experiment or signalling was performed in this review.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Checked without additional findings

Correctness and contract were checked across original R27 and R28. Pass35
D2 is addressed structurally for six style artifacts and the proposed two
variable artifacts: both Q/C PNG paths/hashes reach the native consumer,
which independently decodes and requires .99 against each actual SVG paint.
The original static-text comparison remains too. D1 above blocks the new
variable preparation, not the legitimacy of requiring both curves.

The source decoder preserves contour order, winding, implied midpoints,
physical hmtx and bounded explicit composite transforms. Quadratic-to-cubic
conversion is algebraically equivalent, and the one compound variable path
preserves topology. Unsupported source forms fail instead of gaining oracle
credit. No product outline/shaper/subsetter is used for reference contours.

The original five positioned Thai owners, both weight physical sources,
default-axis versus explicit-selector rules, faceindex0, normal/synthesis
rejection and final-owner negatives remain in the retained owner helpers
and native gate. Both variable default and nondefault acceptance must still
run. No actual score for any proposed new reference has been measured.

PDF continues using independently authored original/static font text and
the absolute binary/hash/version-pinned Poppler26.01.0 for both product and
reference. Source-derived OOXML placement is checked against reachable
composed PDF owner matrices, retaining notes-body and preview separately.
Q/C controls retain12degree shear,1/24em round stroke, actual advances and
deciding wrong-style/origin/shear negatives. The .99 threshold is unchanged.
The shared resvg/tiny-skia backend limitation remains explicit.

The runner preserves Word56 and PPT48 exact selection and complete named
outcomes, rejects skip/error outcomes, retains failures and requires actual
104 before enumerating eight real artifact reports. Native SVG is still a
separate ROOT serial --exact --ignored gate requiring1passed0ignored.
Wheel/build/install/interpreter and source lineage remain ROOT admission
obligations. Hash-bound source and artifacts do not independently prove
fresh wheels or actual104 success. No admission has been supplied or run.

Panics have0 additional findings beyond the fail-closed preconditions.
OOXML has0 findings in this test/plan-only packet. Tests have the actual
variable precondition defect above. Structure has0 findings, with no new
product file/module/dependency/API. The union proposal preserves unrelated
F206 consumer content and carries the same native gate correction, but its
runtime admission remains separate. Full riders, hash attribution,
publication, HLD and feature completion remain due.

## Review boundary

Only this canonical review was written. No source/adoption, scratch, Cargo,
compiler, product imports, painter, UI, cache, mtime, baseline or intake
changed. The review ends and returns its write lease to ROOT.
