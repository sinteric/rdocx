# F-X207, layout measurement, pass 46

**Reviewed**: external layout measurement diff2bdcd004 against current cb2f source and historical source-qualified actual archive6f991b07.
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, measurement date predates the actual archive

`crates/rpptx-layout/README.md:20`

The proposed new84129/487425/11 tuple retains2026-10-09, while its authenticated archive proof timeWindow is2026-10-11 20:38:06 through20:38:23 UTC. Update the row and the existing ARCHIVE_REMEASUREMENT_DATES entry to the actual measurement date. Preserve unrelated dates.

## Smells

None.

## Nitpicks

None.

## Not found

Archive values, current ordinary source membership, member count and bounded tuple scope produced no findings. Existing actual verified evidence may be retained because every current layout source member matches. No publish replay is required. Review ends before separate date correction and adoption.
