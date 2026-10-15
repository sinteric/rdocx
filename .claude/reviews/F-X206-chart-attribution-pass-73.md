# F-X206, chart package expectation attribution, pass 73

**Reviewed**: Read-only exact package comparison and proposed single fixture SHA literal replacement at crates/rdocx/src/document.rs:36530.

**Verdict**: 0 defects, 0 smells, 0 nitpicks in the proposed attribution.

ROOT independently authenticated the retained historical and current ZIP bytes at SHA8edda1371d1da30b937108fc1cfa3366467c9dbeb90934391c160b532781eef1 and0e3b27ea9a0750a4df4818d8d3f2a36234e31e56fa7b287b3d93a6517e88f158. All44 member names/order match, with42 byte-identical members. Expanded XML comparison with comments, processing instructions, ordered children, attributes, text and tails proves ppt/presentation.xml semantic identity. ppt/slides/slide1.xml changes only cNvPr id2 name Placeholder 2 to Title 1 and id3 name Placeholder 3 to Subtitle 2. Both text-body wrappers now retain the historical correct PresentationML owner. No chart data, workbook, relationship, geometry or text change remains.

Measurement37691122463a5c5d80fcf64a8a070317386cb51f993398b826f70893c30223cf additionally attributes lexical changes to local namespace closure and nine unchanged default paragraph attribute-order swaps. The naming change is explicit in the approved F-X206 design at line30. The proposed new package expectation records this explained behavior and serialization change, without accepting an unexplained output delta.

Only the PowerPoint fixture package SHA may change. Word package SHA, pinned Poppler26.01,150DPI, fixed crop,750 by450 dimensions and exact zero differing pixels remain unchanged. The original chart failure stopped before these paint assertions. Updating the expectation earns no paint or full-feature acceptance until the actual selector reaches and passes those deciding assertions.

Correctness, contract, OOXML, panics, tests and structure have no additional finding in this proposal. Only this review record was written during review. Return to distinct one-literal adoption and exact chart execution. No hash-harness baseline update is authorized by this fixture attribution.
