# F-X206, compiled inputs, pass 43

**Reviewed**: Distinct compiled-asset coverage audit of R4 `/private/tmp/S91-F-X206-surfaces-Base-runtime-r4-prepared`. New known-producer asset-only negative controls after pass42, with the same sealed runner, source map and five-control queue. This pass does not repeat or replace prior reports.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, known compiled targets can hide different embedded assets

`/private/tmp/S91-F-X206-surfaces-Base-runtime-r4-prepared/run.py:43`

The producer comparison restricts inputs to Rust, files beneath `/src/` and Cargo manifests. The required native graph contains actual compiled inputs outside that filter:

- `crates/oxml-layout/src/bundled_fonts.rs:33` embeds `crates/oxml-layout/fonts/Carlito-Regular.ttf`.
- `crates/oxml-pdf/src/conformance.rs:11` embeds `crates/oxml-pdf/assets/sRGB2014.icc`.
- `crates/rpptx/src/lib.rs:157` embeds `crates/rpptx/assets/default.pptx`.
- `crates/rpptx-layout/src/context.rs:7305` embeds `scripts/pptx-corpus-manifest.tsv` through include_str.

If producer compilation evidence establishes the relevant native targets, but one of these captured asset inputs differs while the compared Rust/manifests agree, it creates no difference seed and no invalidation for its consumer. Missing-target conservatism does not protect targets already classified known. Compiling an earlier asset does not establish equality with the current asset.

Independent isolated metadata checks used the exact prospective temporary source map and synthetic completed-compilation text solely as an in-memory negative fixture. All12 required targets were known, none unknown and no selectors required in the unchanged case. Separately changing only each of the four real tracked producer asset SHA values left the complete derived result identical, including the empty selector list. Changing the Rust font source instead produced the expected layout and reverse invalidation. Synthetic text was not written, admitted as a raw receipt or represented as actual compiler evidence. No product or runtime execution occurred.

Compare complete compiled input ownership, including tracked assets inside selected crate directories and resolved external include_bytes/include_str inputs. Differences must seed their consuming crates and the selected reverse closure. Source snapshots, manifest equality and a known-compiled target cannot replace captured embedded-byte comparison. Retain actual producer/unknown coverage safeguards and exact prospective source projection.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Evidence and retained boundaries

Packet receipt5f11f5ec7f771ce5f235211cdb7dccb04fd35895e4c9f331368121fc092206b5 and runnerda349c2e25724026642d743591ddd56dd14d1bcdd4f6511480aa9010bd9679c6 remain exact. Every actual2714 source byte, size and mode matches53debc893276ef3db4baa380c971dc994ba0d835df8406f933335d75dd57c875. HEAD and staged index authenticate. All sealed artifact hashes match.

Pass42 checked actual Rust-map difference recomputation, changed-manifest refusal and unknown-target reverse invalidation. The present new negative tests exercise the untested known-target embedded-input boundary. Its omission warrants this separate finding without changing the original pass42 evidence. Original five controls, three temporary carriers, native expected XML, owned-group cleanup, source restoration and honest attempted/compiled provenance remain unchanged.

## Not found

Panics, OOXML and structure produced zero findings. Correctness, contract and compiled-input admission tests have D1 above. No additional lifecycle or restoration finding is introduced by this narrow pass.

Future producer receipt, source, seal and authorization remain pending. No actual five-Base result, semantic RED, target transition, native paint, installed acceptance or feature completion is established. A separate remediation phase and independent review are required before actual admission.

Only this report was written. No source, packet, runtime, compiler, Cargo, import, environment, cache, mtime or UI change occurred. The review ends here.
