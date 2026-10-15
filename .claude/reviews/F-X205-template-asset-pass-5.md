# F-X205, template asset, pass 5

**Reviewed**: immutable template-core capsule, before and after bundled assets, 39 unchanged member names and nine changed member byte streams. Eight geometry members and one core metadata member change. This is an asset and evidence attribution review, not a review of mutable constructor or later paint code.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Contract and authority

The approved F-X205 design requires deterministic fresh metadata at
2000-01-01T00:00:00Z and authenticated Office widescreen master and layout
rectangles. Imported packages and ordinary save remain outside this fresh
asset correction. Capsule manifest SHA
bf8b8cc9d55a5a99b197c211e135a3374f3fdd3b3d134d90850e966ecc02b05c
authenticates both assets and source-map evidence. Before asset SHA
8ecd98d4e52c8ece061cb36c8baa9f0424b362454d820f6df1e425c662b9a057
and after asset SHA
2049d6080142619d9eb16021e9d66c9de6b0663a1e457a006bfe1def23591d24
were independently checked.

## Attribution

The 39 member names are identical, and 30 member byte streams are unchanged.
All eight changed layout/master members differ semantically only in their
placeholder transform geometry. Comparison layout gains an explicit title
transform that previously inherited the master. Removing placeholder
transform elements for comparison leaves the complete XML model identical.
Every placeholder rectangle across the master and all 11 layouts matches
the native authority by layout name and placeholder type/index, including
absent transforms. The native package is
/private/tmp/S91-X208-native-footer-controls/baseline.pptx, SHA
75261bde786a6b768a736a8657f93658061aa279402cd254132c9edb4382e919.
Notes, relationships, themes, identities and nongeometry content are unchanged.

The changed core member has both exact fixed created and modified timestamps.
Creator, lastModifiedBy and description are absent. Remaining neutral metadata
is retained. Runtime actual-template-metadata-GREEN-1 authenticates one
meaningful native pass, zero ignores, source-map SHA
930b78ba39d8f397dc48b3ef1b1f1fa80230e17aa4538aa7d7382b85fa722781
and raw log SHA
5dacb2206d67e39dccf27e1a20bed58f8e2997792aaa35f8022e4e4772f35133.
Source guard matched and the owned process exited naturally and was reaped.

## Defects

0 defects.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Not found

Asset scope, geometry attribution, neutral metadata, member ownership, schema
placement and evidence provenance produced no findings. This pass does not
credit a final hash harness, native render of the changed asset, archive
measurements, mutable constructor behavior or full feature completion.
Intentional geometry and metadata changes still require separately labelled
commits and actual measured output attribution before any baseline movement.
