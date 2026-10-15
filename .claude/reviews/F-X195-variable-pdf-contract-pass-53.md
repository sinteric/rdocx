# F-X195, variable PDF contract proposal, pass53

**Reviewed**: R35 existing Python test carrier and explicit plan amendment.
**Verdict**: 3 defects,0 smells,0 nitpicks. Unadopted and unexecuted.

## Defects

### D1, new PDF reader dependency is absent from the qualified interpreter

`/private/tmp/S91-F-X195-variable-pdf-contract-r35-prepared/candidate-test_documented_examples.py:5827`
imports pypdf. Actual isolated qualified Python reports find_spec for pypdf
as None. The current test carrier contains no pypdf import. The proposal's
reuse claim is therefore false, and each new PDF check would fail before its
deciding assertions. Reuse the existing qualified PDF parsing mechanisms or
explicitly qualify the dependency and environment before execution. Do not
claim AST parsing proves runtime imports.

### D2, actual/static anchors replace independently expected source owners

`/private/tmp/S91-F-X195-variable-pdf-contract-r35-prepared/candidate-test_documented_examples.py:5871`
through5873 derives both anchor inventories from product PDFs and compares
them to each other. It never derives the expected owner count, roles, origin
or scale from original OOXML and original physical font metrics. The loop at
5882 is not checked against the independent source inventory. A common
placement or omitted-owner error in both PDFs can evade the intended owner
proof. This does not satisfy the proposed source-derived per-owner contract.

Derive original slide/notes body and preview owners independently, as the
existing style fixture controls do. Require exact owner cardinality and
source placement before constructing ROIs. Preserve the qualified source
preview scale and exact unrelated pixels. No pixel registration is permitted.

### D3, glyph-count matching does not bind the validated subset to drawn text

`/private/tmp/S91-F-X195-variable-pdf-contract-r35-prepared/candidate-test_documented_examples.py:5853`
selects resources by glyph count. The validated subset at5863 is not bound
to the reachable Tf/Tj or TJ operations used by the expected owners. An
unused matching resource could satisfy exact outline checks while a different
font paints the actual text. Resource count and matching outlines alone do
not prove caller selection or every used subset glyph.

Resolve each actual owner font through its reachable resource scope and
text operations. Bind actual CID sequence, source first-use mapping and
widths to the authenticated HB/static source before geometric and paint
checks. Do not select the first matching or merely same-sized resource.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Not found

The causal justification for representation-aware paint is independently
established by actual R34. No fitted pixel tolerance, GID exception or
original-font substitution is proposed. Exact repeat, non-PDF comparisons,
full104 membership and native SVG requirements remain. The .99 threshold
and negative-control intent are retained. The explicit test semantic and
plan changes are correctly disclosed, but cannot be adopted with these
source-owner and runtime gaps. No production change is authorized.

## Review boundary

Only this review was written. Read-only isolated dependency discovery ran.
No worker, proposal, package, installed environment, Cargo, cache, mtime,
application, baseline or intake mutation occurred. Return to orchestration
for a separate correction packet and independent review.
