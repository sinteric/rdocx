# F-X210, Base tests, pass1

**Reviewed**: Test-only start packet in /private/tmp/S91-F-X210-start-r1, one existing Rust carrier,155 added lines. Worker branch work/f-x210-codex, HEAD7bbc960e54d2b5acf6763c8d6d13b3796c1a5a7a. Approved design, bounded metric decisions, current progress and cited deterministic rendering and test taxonomy were read.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication

Receipt SHA25645519022be245f7968d163869b5e4d6d5acdbb4a4f34e59beb82bae665172389. Test-only diff SHA25636bd1fda250970e568ade4829dbfbdbf897aaf117aaa999daad68a62e8fff1af. Current source map SHA256a68e14feeeb908d07808a20c9b80e582a9e008f3bfc90da4f48d859015c6f861 matches all2900 tracked byte, size and mode records. Only bundled_fonts.rs differs from the before map. Production and original tests are an exact retained prefix. Index SHA2567aef5efe3c9b04d5d50070222990762ac9fd664bbbec4f59fa2d13631c1437be matches the receipt. All packet artifact hashes and nine retained asset hashes match, total444812 bytes. No asset was copied or registered.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Aspect results

- Correctness: zero findings. The four requested styles independently select the proper upright or italic Gelasio source and400/default or700 coordinates at crates/oxml-layout/src/bundled_fonts.rs:266. Selawik selects genuine regular or bold source bytes while requiring synthetic italic only for italic requests at crates/oxml-layout/src/bundled_fonts.rs:338. Single-glyph reference advances are derived from authenticated original face bytes, with the selected Gelasio weight applied before reading hmtx. The assertions do not compare substituted bytes against themselves.
- Contract: zero findings. The approved original Gelasio variable faces, static Selawik faces and nine-asset inventory are respected. No genuine Selawik italic is invented at crates/oxml-layout/src/bundled_fonts.rs:328. The one-glyph advance controls do not claim Segoe bold reference parity, identical kerning, line pitch, wrapping or universal metric compatibility. Those remain separate approved gates.
- Panics: zero findings. Fixture read, parse, metadata and coverage failures are explicit test preconditions at crates/oxml-layout/src/bundled_fonts.rs:230. Axis indexing follows an exact length assertion at crates/oxml-layout/src/bundled_fonts.rs:254. There is no production panic path or new untrusted-input API.
- OOXML: zero findings. No OOXML, package or serializer code changes. No source XML or stored character mapping is touched by this stub phase.
- Tests: zero findings. Exact module paths are bundled_fonts::tests::fx210_georgia_uses_pinned_gelasio_faces_and_selected_styles and bundled_fonts::tests::fx210_segoe_ui_uses_pinned_selawik_faces_without_inventing_italic. Existing public module declaration at crates/oxml-layout/src/lib.rs:3 and private test module at crates/oxml-layout/src/bundled_fonts.rs:148 establish these paths. FontManager, selected FontData fields, shaping and ttf-parser APIs exist in current source. Each selector requires actual collection of one test, with no ignored test. Current Georgia and Segoe mapping selects Caladea and Carlito respectively at crates/oxml-layout/src/font.rs:2698, so the intended first deciding assertions are the approved-family assertions at crates/oxml-layout/src/bundled_fonts.rs:272 and crates/oxml-layout/src/bundled_fonts.rs:347. Input or compiler failure earns no semantic RED. A first-style Base failure does not prove the three later styles. All four styles need actual GREEN execution.
- Structure: zero findings. One concrete private fixture-reading helper supports both actual tests. No new file, module, asset, dependency, trait, generic or forwarding-only wrapper. The retained-input override allows genuine Base without prematurely copying approved assets, then normal execution reads the same original package files.

## Limits and handback

This is review of two initial selection and metadata stubs, not full F-X210 acceptance. Aptos diagnostics, caller priority, qualified symbols, kerning and line metrics, Word/PPT paint, installed projections, packaging and hash obligations remain pending. No compiler, native runtime, metric-reference comparison or GREEN result is inferred. Only this review was written. Source and execution leases return to ROOT.
