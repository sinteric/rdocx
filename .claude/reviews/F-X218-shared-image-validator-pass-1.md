# F-X218, shared image validator, pass 1

**Reviewed**: Mechanical relocation against actual preserved pre-edit bytes in the F-X218 worker. Two existing carriers,335 insertions and348 deletions. lib.rs adds333 lines. html.rs replaces its call and removes the original validator. This is a bounded prerequisite review, not whole-feature acceptance.
**Verdict**: 0 defects,0 smells,0 nitpicks within the relocation.

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Aspects checked

- Correctness: zero findings. `crates/rpptx/src/lib.rs:328` retains checked pixel arithmetic and the original limits. The relocated validation at line342 retains PNG chunk extents/CRC/complete decoding, JPEG scan termination and bounded decoding, GIF frame consistency and aggregate bounds, TIFF page/buffer bounds and BMP physical pixel extents. Every branch, decoder call, bound and error literal matches the actual pre-edit implementation.
- Contract: zero findings. `crates/rpptx/src/html.rs:624` calls the same validation with the same data and detected format, then maps the returned static message through the existing semantic_error with the original source path. The result still preserves original owned image bytes and export output. No decoder, format, limit, dependency, version or renderer behavior is added by this relocation. The later XML caller remains separately unqualified.
- Panics: zero new findings. Existing checked lengths, bounds and decoder options remain identical. The mechanical move introduces no new indexing, unchecked arithmetic, allocation, unwrap or expect. Failure construction moves to the caller with the same message and source path. This pass does not independently certify every pre-existing decoder implementation.
- OOXML: zero findings. This relocation changes no XML parsing, schema order, namespace lookup, relationship resolution or package publication. The existing source/resource identity control verifies that export does not mutate its input.
- Tests: zero findings within scope. Six existing media/source review regressions and the exact unchanged-export control executed successfully. They exercise actual complete five-format bodies, corrupt/truncated bodies, dimensions and source identity. No new mirrored implementation test or invented failing-before control is necessary for a behavior-preserving move. These controls do not qualify the later checked XML image relationship caller.
- Structure: zero findings. Four existing constants and four existing functions move into the existing crate root. Only the shared validation entrypoint becomes crate-private. Its concrete static-message result lets each actual caller attach its own error context inline. No new module, file, trait, generic, registry, forwarding wrapper or dependency is introduced.

## Independent source comparison

Actual pre-edit lib.rs SHA is `ae8c298f7ea8ab003514ef999395f1e8bfbf1618485d34836ac73297511ab2e2`. Actual pre-edit html.rs SHA is `892552689e80c020c26300cea000b4cde538cea4bd3e5a945ab4239610f779b5`.

Current lib.rs SHA is `7c337c93122fd52c43733fe61871582e632902f8bea10208f4445878bf5039da`. Current html.rs SHA is `093d0d340f0738a23f5559b7d2bb3cbf0e442343edcc6f0acb02e2367263b6e7`.

Root independently authenticated every capsule artifact and reconstructed the full two-carrier transformation. Only removal of helper path arguments, static-message error construction, concrete result types, crate-private entrypoint visibility and the inline original HTML error mapping are semantic edits. Normalizing formatting whitespace and optional trailing call commas makes the expected and actual entire carriers identical. The complete ordered string-literal sequence is separately identical, including spaces inside messages.

All3,203 current tracked file hashes authenticate. Only the two named carriers differ from the actual pre-edit snapshot. HEAD remainsa86e1f9142e602c6e34d4801136c8b33efb652d1, branch work/f-x218-codex, with the same logical index. Capsule SHA is `12c94d3de14a88acea3a5a9840490acd5bd25a9999c841629fe79112d77a866d`.

## Actual finite qualification

Receipt SHA `4f850a1a4d4c8fb2dfba54de97dd7a08f5bc52b193c49eb5496a298aeb71f384` authenticates seven actual exit0 rows with exact source guards, no timeout, gone owned groups and reaped leaders:

- Locked/offline default rpptx all-target check.
- Locked/offline rpptx library check without default features.
- Existing fx218_review_ integration filter,6 passed,0 failed,0 ignored.
- Exact fx218_export_source_and_resources_are_unchanged,1 passed,0 failed,0 ignored.
- Default rpptx library Clippy with warnings denied.
- Formatting check over the two changed carriers.
- Hash harness,49 entries match.

All seven raw log hashes were independently authenticated. Existing integration dead-code warnings remain recorded. Library Clippy is not all-target/all-feature Clippy. Runtime used the existing8MiB test-stack setting, so no ordinary2MiB consumer claim is made. Proof/schema setup assertions remain setup failures rather than behavioral regressions. No tests or Cargo command ran during this review.

## Acceptance boundary

The original F-X218 approved contract, canonical workflow/microscope and source/package/testing references were read. This pass certifies only the mechanical relocation and its finite evidence. Native PowerPoint authority, current package measurements, installed binding composition, remaining risk riders, whole F-X218 completion, F-X211b image acceptance and final sprint verification remain pending. Historical archives and installed wheels are not relabeled as builds of the relocated source.

Only this review file is written. Worker sources remain frozen and all existing canonical in-flight sources, plans and prior reviews remain untouched. Review ends here and returns control to the orchestrator for explicit shared-source composition.
