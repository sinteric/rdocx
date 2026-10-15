# F-X195, paint diagnosis admission, pass 27

**Reviewed**: External read-only diagnosis and source-independent PDF reference
proposal. All27 artifacts and both original failing logs authenticated.
**Verdict**: 1 defect, 0 smells, 0 nitpicks in reference qualification.

## Defects

### D1, reference painter does not use the admitted Poppler pin

`/private/tmp/S91-F-X195-Word56-paint-diagnosis-r1/reference.py:33`

The independent reference invokes /opt/homebrew/bin/pdftoppm directly.
The actual installed R15 stages use the admitted26.01 path under
/private/tmp/S91-poppler-26.01.0/bin. Retained measurement argv confirms the
hardcoded reference path. The repeated qualification at reference.py:34 and
the memo call these the same pinned painter, which the actual invocation does
not establish. Thus the claimed1.0 static italic and .994173 synthetic italic
scores cannot yet qualify the pinned acceptance oracle. Preserve all original
results. A new source-independent reference measurement must authenticate the
actual reviewed binary and version and render with that path, retaining the
same source anchors, unchanged .99 decision and negative controls. No product
rerender or assertion relaxation is needed to resolve this reference defect.

## Smells

None.

## Nitpicks

None.

## Not found and limits

The decoded selected-axis versus embedded-upright contour discrepancy does
not depend on this raster tool selection. No additional outline finding is
raised here. The exact product stage losing selected outlines remains
unmeasured and requires the proposed causal native tests. Static source glyph
identity and original actual Word26/30 and PPT23/25 failures remain retained.
No source-independent reference proposal receives installed acceptance credit.

This bounded review writes only itself. It does not audit full F195 source,
change source, run a product test, install a wheel or relax any threshold.
The review ends and returns control to separate preparation and remediation.
