# F-X210, consumer Base tests, pass 4

**Reviewed**: External test-only proposal in `/private/tmp/S91-F-X210-consumer-Base-tests-r1-prepared`. Two existing carriers, four named tests, 12112-byte complete diff. Receipt SHA256 `c9121ae7861af21e7caa54d2cb5e79caac0731a08620e4286e18a076cf0caca3`. Complete diff SHA256 `6e2071dd210363c2c48cec2ca564d5eb978b4b25a6e34f7de073ce320bf324a3`.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, proposed measurement assertion references a future field and prevents Base compilation

`crates/rpptx/src/lib.rs:13614` in `ppt-carrier.candidate` accesses `frames[0].diagnostics`. The actual existing `TextFrameLayout` definition at `crates/rpptx/src/lib.rs:215` through222 contains only `slide_index`, `shape_id`, `name`, `autofit` and `layout`. No `diagnostics` field exists. The approved design selects this additional projection for implementation, but the test-only packet expressly requires existing APIs before production adoption.

Both proposed PPT selectors compile the same library test carrier. This reference therefore blocks both before their deciding diagnostic or bullet assertions, rather than producing genuine semantic Base failures. Keep the existing `render_deterministic` diagnostic discriminator for Base and place the measurement projection assertion in the later implementation control phase when its real API exists. Do not add a prototype field merely to manufacture Base. The packet's claim that all four tests use only existing public inputs is currently false.

## Smells

Zero.

## Nitpicks

Zero.

## Authentication and bounded assessment

The worker remains on `work/f-x210-codex`, with F-X210 in progress in both ledgers. Read the approved plan, current progress, provider review3 and workflow microscope contract. All2909 current byte, size and mode guards match provider source map SHA256 `0a46980f5c88eef9654d376daad798b03cc48fbfb6e17108983a8c546a35f781`. Both carrier preimages and complete candidates match the receipt. All listed packet artifact hashes match. Existing carrier bodies remain exact up to the final test-module closing brace, with only appended controls.

Word uses real `Document` layout, explicit symbol and numbering APIs. PPT uses real shape text, bullet and deterministic rendering APIs. The layout walk and multilingual legacy projection exist. The two Aptos controls first require four actual visible contributions with nonzero glyphs and exact bundled Carlito bytes, then distinguish the two requested names and repeat output. The Word control also retains empty-request and later non-Aptos checks. These preconditions are meaningful and must pass before a missing-diagnostic assertion earns semantic RED.

The eight symbol mappings follow the independently measured native rows. Word covers both explicit low-code and PUA spellings, actual list markers, unchanged package/model source and explicit negative contexts. PPT covers real PUA bullets and the existing F0B7 compatibility case. Marker selection distinguishes the fixture's item text from positioned marker output. These establish positioned semantic text and glyph coverage, not independently observed painting. Save/reopen comparisons retain original source semantics. No claim of original-font outline or metric parity is warranted.

## Aspect results and limits

- Correctness: D1 above. No additional finding in the bounded appended controls.
- Contract: D1 violates the existing-API Base requirement. Other assertions fit the approved bounded diagnostic and symbol contract.
- Panics: Zero findings. Indexing and unwraps belong to authored fixtures and explicit setup preconditions, not production untrusted input.
- OOXML: Zero findings. Tests use existing qualified source writers and retain source/package equality rather than rewriting expected input.
- Tests: D1 blocks genuine PPT Base. Other deciding assertions and positive setup are meaningful, subject to actual compilation and execution.
- Structure: Zero findings. Two local collectors serve two actual tests each. No new module, file, dependency, trait or generic is introduced into the repository.

No Cargo, product import, native UI, source edit or runtime execution occurred. Actual Base remains pending. Genuine caller-font suppression, original symbol-face preference, additional PPT negative contexts, companion APIs, CLI propagation and independent PDF/PNG/SVG appearance remain full-feature obligations. This review does not credit those gates or complete F-X210.
