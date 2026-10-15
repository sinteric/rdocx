# F-X195, core, pass 2

**Reviewed**: frozen core prefix R2 and D1 remediation, 30 added and
12 removed lines against the retained pre-remediation writer/raster
capsule. Prefix freeze SHA256
6493e5a0c1897fc00be25dfc92b9a71cc7ffee19e1ff8e3a71f9b29a729b964d.
All ten source guards were independently authenticated. The prior eight-file
core review remains the scope, with the new analytic PDF control also read.

**Verdict**: 0 defects, 0 smells, 0 nitpicks in the reviewed core phase.
Actual compile, geometry, provider, facade and full feature gates remain
pending. This verdict does not complete F-X195.

## Defects

None. D1 is resolved in source: writer.rs:1846 applies the baseline shear
to the scoped graphics transform of the complete painted run. Ordinary and
multilingual text keep upright text matrices and their original position
math. The synthetic round stroke now follows the same affine composition
as raster. Existing group and ActualText setup stays in place. The new
raster.rs:1494 control compares a decoded PDF glyph with a separate analytic
round-stroked rectangle, allowing one pixel at576dpi for edge rounding.
Its actual RED/GREEN outcome is not yet measured.

## Smells

None found.

## Nitpicks

None.

## Not found

Correctness, contract within this phase, panics, OOXML, tests and structure
produced no further finding. Remaining private ink bounds, provider request
trace, output wiring and scoped verification are explicit unfinished work,
not acceptance credited by this review. No implementation file was changed
during the pass.
