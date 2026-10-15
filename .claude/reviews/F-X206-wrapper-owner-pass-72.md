# F-X206, caller-selected wrapper repair, pass 72

**Reviewed**: Complete four-carrier11164-byte incremental diff e95cead56bd8c2a8b28bd05e5579f92fcfec34e683200c0d7300f5caaba703e7 against pass71.

**Verdict**: 0 defects, 0 smells, 0 nitpicks in the bounded repair.

## Authentication

ROOT authenticated every2714 byte, size and mode record at a3fcbcad218d66bfd52d0ef17b7ba8fd3b858315a5e71b2cf00133457036f9d2. The four changed carriers are the text paragraph, body and list-style writers and the shape-tree test carrier. All2710 others remain exact. Receipt d12306b7ab157201161a1ba2743373a1c1773d84535a2502eb5600b50ca2866c, actual checks89776b498df207c14cc459fafbc3f48a88fd889c9172cd5cc99ae066a3258a6d and Word pair bb47d77dd2f0bc973a85a2c89f6d2afa7fe9bf3c9af3154455568416836560d3 authenticate. All raw logs match and all owned groups are freshly absent.

## D1 disposition and six aspects

D1 resolved at crates/oxml-drawing/src/text/paragraph.rs:37. Source spelling is retained only when its actual URI equals the requested canonical wrapper's namespace. Presentation wrappers remain finite and diagram text remains dgm:t. The converted wrapper helper at crates/oxml-drawing/src/text/paragraph.rs:77 closes its destination prefix without rebinding an opaque source prefix. It chooses a free local destination spelling when the requested prefix is already foreign-bound. Authored and legacy undeclared fragments retain their existing surrounding-scope contract.

The actual cross-owner controls at crates/oxml-drawing/src/text/mod.rs:908 convert in both directions for alias and default owners while preserving exact opaque p-prefixed payload and repeated save/reparse. The actual shape control at crates/rpptx-oxml/src/shape_tree.rs:3099 proves the saved DrawingML shell becomes a genuine PresentationML text body and retains text on reread. Existing drawing176 inventory passes174 with two historical ignores. Both-crate all-target checks and Clippy pass. The original large Word/PPT reading-order and raster control now passes under pinned Poppler26.01.

Correctness, contract, panics, OOXML, tests and structure have no additional finding in this incremental repair. No trait, module, generic or dependency is added. The helper performs a concrete namespace conversion shared by body and list-style writers.

## Remaining acceptance

The chart test still fails its original package hash before paint, with current0e3b27ea versus historical8edda137. Its immutable failure is retained. Historical expanded comparison reports only two intentional placeholder-name attributes after this correction, but the exact expectation and actual pixel assertions require a separate attribution and test phase. No full native, installed, archive, hash or whole-feature completion is claimed. Only this review record was written during review. Return to orchestration.
