# F-X216, effective appearance, pass 1

**Reviewed**: Frozen test-only successor against producer4, two existing carriers, 464 added lines and zero deleted lines. Worker HEAD is `5b0f63d95e4ea82a05a3082fe74a6d8e19d9f572`. The frozen manifest SHA-256 is `83869f62700ccbeaa234777a77ae5df9dda941575cf2c6e890be518b55540629`.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Contract and scope

Read the canonical workflow, microscope command, F-X216 design contract and progress record, with rendering and test-strategy references. F-X216 remains in progress. This pass audits only the effective appearance comparison and its test authority. It does not replace the whole production review, native PowerPoint acceptance, scoped gate or feature completion.

The reviewed additions are `crates/rpptx/src/lib.rs:38263` through its appended notes control and `crates/rpptx/tests/integration.rs:35303` through its appended corpus and wrong-theme control. No production statement is changed by these additions.

## Authentication and runtime evidence

Independently authenticated all 3296 current tracked bodies, sizes and modes against the frozen source map, plus actual HEAD and logical Git index. Removing only the two appended test sections reconstructs the exact producer4 bodies. The library preimage is `f752ae130a010be78f4c26a7076a46c3a7d86e1e29a405da7d6c0d213973eee4`, 1524240 bytes. The integration preimage is `855479388460b8edc9696c4d87d08093b06bc2f8ca0946ecca481893e86fcfe7`, 1365660 bytes. The other 3294 tracked bodies are identical. This bridge supports retaining the separately qualified production wheel without rebuilding for these test-only additions.

Authenticated all seven retained receipts and their raw logs, both frozen binaries and all three final output files. Independently listed both frozen binaries and confirmed the actual exact named selectors. Final successor receipt `97c65e9c02468dc388732666a0acd3c3058a92c8602aa760b17f310112229b18` contains two exit-zero rows, each with one named passing test and zero failures. Both terminal records report the owned process group gone, leader reaped and safe lease release. Earlier compilation, color-format and language-fact failures remain retained and are not relabeled as successes.

The corpus output contains twelve paired source-page records, six in each import direction, plus the authored source and wrong-theme negative. The notes output records the actual source note page against imported page two. Actual source-built notes and wrong-theme source bytes are checked against exact hashes before comparison.

## Effective appearance authority

`crates/rpptx/tests/integration.rs:35382` calls the public deterministic renderer and compares actual resolved input plus positioned output. Four transformed shape corners use explicit 17-decimal formatting. Remaining floating-point model facts use Rust's round-trip Debug formatting without rounding, cropping or tolerances. Complete shape geometry, fill, line, endpoints, effects, image placement and text or table content remain in the resolved-shape comparison. Resource identities are replaced only after recording the actual media bytes and content type.

Both positioned-element helpers resolve every used glyph run's actual FontId against the returned FontData. They compare font-byte SHA-256, family, face index, bold and italic selection, variation axes and synthetic flags. Glyph IDs, advances, offsets, clusters, origin, size, logical text, script, direction, bidi level and paint remain in the run facts. Provenance spans and result-local IDs are removed, rather than pretending those identities are source appearance. Groups retain transform, clip, opacity, effects and ordered children. Images retain rectangle, actual bytes and content type. Other painter variants retain their complete Debug facts. Non-drawing marked-content identities do not change paint.

The authored positive must match the source exactly. The destination-theme negative must differ, with separate resolved Georgia versus Calibri and red versus black assertions at `crates/rpptx/tests/integration.rs:35513`. These are discriminating controls rather than only matching two successful outputs.

## Notes language boundary

Only the notes appearance helper excludes language tags at `crates/rpptx/src/lib.rs:38321`, under the integrator's admitted nonvisual appearance projection. It retains the actual shaped glyph, script, bidi, font-byte and painter facts. The control separately asserts and records the five source `None` and imported `Some("en-US")` observations, along with identical logical text. It does not conceal that metadata difference. The additional real local `fr-FR` source mutation is imported and its resulting local language observations must remain exact, including source immutability. The integration helper retains language tags without this notes-only projection.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

No correctness, contract, test-authority, panic-handling or structural finding within this bounded test-only diff. Test unwraps deliberately fail the controls on missing resources. No production XML parser or serializer changes are present in this successor. Native Office rendering and save/reopen comparisons remain separately required and uncredited by this report.
