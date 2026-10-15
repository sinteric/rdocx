# F-X195, test compile remediation, pass 14

**Reviewed**: Frozen R4-to-R5 supplemental delta on work/f-x195-codex,
HEAD2a9bf14072231e46fcb34df70f88cf7aa79fbd79. Two existing test carriers,
2173-byte patch e4e83e663fbb6ed9442809ad2553c0518a20585c9e2ce0ab71e9592699c40b59.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Contract and identity

Read CLAUDE, WORKFLOW, microscope, the approved F-X195 contract and cited
rendering requirements, current progress and prior acceptance reviews.
F-X195 remains in-progress. This reviewer is independent of the remediation
author. This pass covers only the compile repair, retaining prior reviewed
production and native acceptance obligations.

Authenticated the R5 receipt
d06c014ef8109625591b8c1cd7c35acb6ee538fc458d51f6c86bd2b1ca9382b8,
the retained R4 manifest and both before copies. All317 current input
hashes, sizes and modes match the R5 inventory. Only document.rs and svg.rs
differ from R4, with all other315 inputs and189 other Rust inputs exact.
Reversing precisely the documented edits recovers both complete R4 files
byte for byte. All modified bytes are within existing test code. Production,
dependencies, baseline, font inputs and assertion expectations are unchanged.

## Assessment

The document fixture at crates/rdocx/src/document.rs:34146 now uses the
existing mutable first-run setter rather than a nonexistent paragraph setter.
Document::add_paragraph at document.rs:20129 creates a direct run for the
nonempty caller geometry text. Paragraph::run_mut at
crates/rdocx/src/paragraph.rs:1143 resolves that direct accepted run, and
Run::set_font at crates/rdocx/src/run.rs:655 writes all four font slots through
the existing setter. The explicit fixture expectation cannot conceal a missing
run. The unchanged actual selected family, font bytes and face-index assertion
at document.rs:34161 still rejects a silently ignored caller font. Existing
PDF, raster, SVG, out-of-range and source-preservation comparisons remain.

The imports at crates/rdocx/src/svg.rs:1305 expose existing private helpers
and the base64 Engine trait to the nested test module. No production helper
or API is added. The two digest expressions at svg.rs:1930 and svg.rs:1979
format every digest byte as exactly two lowercase hexadecimal digits, preserving
leading zeroes and all existing 64-character SHA expectations. This avoids
the unavailable digest-output LowerHex implementation under the existing
sha2 version at Cargo.toml:102. No expected source or instance hash changes.

The hb-subset version guard, fixed source hashes, independent static instance
hashes, variation-axis arguments, physical-font identity, analytic synthesis
and raster comparisons remain unchanged. Importing the production SVG helpers
repairs test visibility without replacing those independent controls with
producer-derived expectations. No threshold or geometry assertion is weakened.

## Defects

0 defects.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Not found

Correctness, contract, panics, OOXML preservation, tests and structure were
checked. No findings in any of the six aspects. The fixture expect is limited
to known authored test state, not untrusted document input. No new module,
file, trait, generic, dependency or public behavior is introduced.

## Verification qualification

This is source review, not runtime acceptance. The original layout check0
and Word check101 with ten compile errors remain retained in the immutable
R4 attempt. Later commands were unrun. No Cargo, imports, test execution,
formatting or source remediation occurred during this pass. Fresh compile
and acceptance execution remain required before claiming those gates passed.
The pass ends with this review file, returning the writer lease.
