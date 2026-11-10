# Hanji, zero-radius preset corners, bounded review

**Reviewed**: seven tracked files, 200 additions and 6 deletions against 082edbf6, plus the approved independent custom-fix plan
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Effects checked

Correctness and contract: the private arc flattener validates all coordinates,
radii and angles before recognizing the exact both-zero case. That case is a
point and returns no commands. The caller retains its current pen, so a
following segment and subpath closure use the prior point. No epsilon gate,
production rounding or coordinate clamping is introduced. Positive arcs keep
their existing cubic lowering and segment cap. Negative, single-zero-axis and
non-finite forms keep their existing errors. The guard test proves these
boundaries and unchanged following-arc output, including negative zero.

OOXML and structure: no parser, serializer, generated preset source, retained
XML, transform, color, font, public signature, dependency or binding carrier
changes. No new production module, source file, trait or generic wrapper is
introduced. The official round2SameRect and round2DiagRect defaults contain the
point corners. Accepting them prevents rejection of the whole supported path.
The existing point model, path commands and caller behavior are sufficient.

Tests: the owned drawing gate and the Presentation resolver gate both fail at
the old code with InvalidArcRadius and bounds fallback. The corrected gates
require two real curved corners and closure. The Presentation fixture covers
direct slide and inherited layout shapes, complete authored/default equality,
source text, bounds, paint and stroke. The drawing bounds oracle uses the
existing assert_close only for ordinary floating trigonometric roundoff.
All 147 drawing tests pass with two pre-existing ignored oracle tests. All 142
Presentation layout tests pass with 50 pinned corpus files verified by SHA-256.
Strict changed-crate Clippy, formatting, prose, adapters and 139 repository
policy tests pass with two expected platform skips. All 49 hashes are unchanged.

The 186-name default probe restores exactly two radius failures. All 181
previously successful complete results stay identical. The three independent
circular-arrow formula errors remain. Real PPTX 12 retains seven slides and
restores exactly seven curved shapes. All other resolved shape state and both
unrelated chart diagnostics remain. All 490 actual rendered text runs, including
glyph state, positions and accumulated transforms, are identical. At 96 DPI
page one is pixel-identical, page two changes 240 pixels and pages three through
seven each change six pixels. The input hash is unchanged. This proves a
bounded shared-renderer correction, not native PowerPoint certification.

Both affected packages verify against the reviewed local dependency graph with
offline and allow-dirty flags, and both README examples compile. Source bytes
inside the verified packages match the final positive source exactly. The
drawing archive has 1,230,174 normalized member bytes and 24 members, with
182,751 compressed bytes in the verified repack. Its recorded 182,758 bytes
are within the existing 64-byte tolerance. Presentation layout has 83,782
compressed bytes, 486,090 normalized member bytes and 11 members. Both remain
below 10 MiB. Machine and date carriers match the refreshed README rows.

Publication follows the user's one-useful-custom-fix-PR-per-fork convention.
The independent branch is based on merged main82 and does not contain the
watermark commit. Keep it local while renderer PR 6 is open. A later consumer
must include both reviewed fixes through a main descendant or an explicit
coordinated dependency. No competing PR, merge, auto-merge, release or deployment
is part of this correction. Exact-head CI remains a future publication gate.
