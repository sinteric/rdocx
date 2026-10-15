# F-X195, core compile repair, pass 3

**Reviewed**: Two additions and two deletions in existing
crates/oxml-pdf/src/writer.rs. Exact remediation diff SHA-256
6635fd26841422cbc10a5cdbc20124c365e5999a6aed2c0d7f62517506da4936.
All nine other frozen source guards remain unchanged from the reviewed core
prefix. R4 source receipt SHA-256
695778be6ec9f49e4ac5e701de19eaea04c8a7d5d5187d2ee1c852e87c9318ed.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Not found

Correctness and contract: crates/oxml-pdf/src/writer.rs:1862 and :1863 use
RoundCap and RoundJoin, the actual pinned pdf-writer0.15.0 enum variants,
also used by the existing writer at :1688 and :1693. This preserves the
reviewed round synthetic stroke intent. No rendering policy or API change.
Panics, OOXML, structure and tests: zero additional findings in these two
variant-name corrections. The existing core pass2 assessment remains
applicable to its unchanged source portions. No runtime acceptance is earned
by this compile repair review. The original provider-word semantic failure
and provider-ppt compilation failure remain separate retained evidence.
Remaining six scoped checks, D1 PDF/raster control and later provider/wiring
implementation and full feature review remain required. No source was repaired
inside this review pass.
