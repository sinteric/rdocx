# F-X195, actual oracle provenance audit, pass 30

**Reviewed**: R15 actual Word56 and PPT48 runner provenance and current
PDF painter adapters.
**Verdict**: 1 defect, 0 smells, 0 nitpicks in raster qualification.

## Defects

### D1, actual R15 painter path is not the declared pin

`/private/tmp/S91-F-X195-installed104-preparation-r15/queue.json:13`

Actual run-root-r15-2.py:29 and run-ppt-root-r15-3.py:31 prepend the
queue oracleBin /opt/homebrew/bin. Word test_rendering_threads.py:1304 and
PowerPoint test_documented_examples.py:6166 invoke pdftoppm by PATH without
a version assertion. Thus the actual expanded104 style stages did not
establish the declared26.01 painter. Original receipt outcomes26/30 and
23/25 remain actual executions, with no full raster qualification.

Pass27 incorrectly described these actual stages as pinned26.01. The
source-backed runner audit supersedes that provenance statement, preserving
the old review and all receipts. Diagnosis r1 used the same host painter
path as actual R15, so its specific cross-pin mismatch inference was
unproven. Qualification r3 does authenticate a26.01 reference renderer, but
its comparisons with retained R15 product PNGs mix painter provenance.
Those scores cannot qualify a same-pin product/reference oracle.

Renew future actual raster stages with the explicit26.01 prefix, exact
version/binary authentication and both product/reference painter provenance.
Keep the unchanged .99 criterion, independent source geometry and negative
controls. No expected image may be derived from product subset contours.
Original failures and source/native outline controls stay retained.

## Smells

None.

## Nitpicks

None.

## Not found and limits

The native axis contour tests do not use Poppler and their recorded
assertions remain actual evidence on their named inputs. Synthetic fixture
preconditions are being resolved separately. No product source, queue,
old review or receipt changes in this audit. This review ends here.
