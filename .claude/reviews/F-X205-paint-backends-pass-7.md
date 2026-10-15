# F-X205, paint backends, pass 7

**Reviewed**: Seven immutable carriers in `paint-source32-capsule`, 1,112 additions and 39 deletions against claim HEAD `338754a70e5c4dffc78d84069b8d0294e9395f2e`. This is a bounded backend review, not a whole-feature review.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Input identity

The capsule is `/private/tmp/rdocx-f-x205-codex/.claude/scratch/F-X205-start-1/paint-source32-capsule`. Its manifest SHA-256 is `7b474674f2db0942557614b23b30937bd3faefa03dd2aeaefd0e9c15c9acc1fd`. All seven complete source hashes and sizes match that manifest. All seven source31 records equal source32 records, so the retained source31 deciding controls qualify these exact paint bytes. Later source34 and source35 changes are excluded.

The review covers shared Grid validation, layout path/pattern generation, PDF resources and conformance, raster texture lowering, SVG lowering, and the affected text/cache consumer. It excludes later formatting APIs, media/export implementation, installed acceptance, and full native paint parity. The reviewer did not author this F205 backend implementation. Earlier inherited provider authority remains separate from this incremental audit.

## Defects

### D1, Grid allocations bypass the shaping-cache byte budget

`crates/rpptx-render/src/text.rs:79` charges heap storage only for Linear and Radial gradient stops. The new Grid paint has an owned colour vector at `crates/oxml-layout/src/paint.rs:31`. The cache clones the complete style at `crates/rpptx-render/src/text.rs:173` while the admission checks use the underestimated byte total at `crates/rpptx-render/src/text.rs:152` and `crates/rpptx-render/src/text.rs:167`.

A resolved run carrying a Grid fill therefore retains its colour allocation without charging it against the 16 MiB limit declared at `crates/rpptx-render/src/text.rs:39`. A 128 by 128 path grid generated at `crates/rpptx-layout/src/context.rs:2040` contains 16,384 four-f64 colours, approximately 512 KiB per cached style before other allocations. Distinct short runs can exceed the advertised cache bound substantially while still satisfying the recorded budget. Charge the owned Grid vector capacity with the same saturating accounting used for other paint allocations. The reported later source35 fix is not part of this immutable input and receives no acceptance credit here.

## Six aspects

- **Correctness**: D1 above. Zero additional findings in the bounded Grid validation and PDF/raster/SVG lowering.
- **Contract**: Zero additional findings in this slice. Imported unsupported geometry remains explicitly diagnosed. Actual product/native pattern and gradient comparison remains a required pending acceptance surface under the plan's paint requirement at `.claude/plans/F-X205-design.md:487`.
- **Panics**: Zero findings. Grid dimensions, colour count and finite bounds are validated before backend allocation. This does not cure D1's retained-allocation accounting.
- **OOXML**: Zero findings in these seven paint carriers. They consume resolved facts rather than rewrite retained XML. Authoring namespace and source-preservation qualification belongs to the separately reviewed producer slices.
- **Tests**: Zero additional findings in the three bounded deciding controls. They prove concrete visible layout paint, raster repeat/alpha and PDF colour/alpha resource ownership. They do not prove independent native crop parity or every SVG output.
- **Structure**: Zero findings. No new module, trait, dependency or forwarding API was introduced in the reviewed slice.

## Evidence and limits

The genuine Base receipt is `actual-path-pattern-Base-1/receipt.json`, SHA-256 `7afb6e28472ae810bb9ced9c513ce81b6e5fd50ed8b384b2a959b11832b5840c`. Its log records an executed test failure with `FX205_VISIBLE_PATH: circle`, zero passed, one failed and zero ignored. It is not a compiler failure.

The scoped check receipt is `actual-paint-check-9/receipt.json`, SHA-256 `bb1c2a38fa27c9d24af6533bce3ee3ce6fef143dcf511b62f0c753885f2db0e3`. The GREEN receipt is `actual-paint-GREEN-2/receipt.json`, SHA-256 `3e3c40d5462b796040767dc2e0e9cbcb5dcc713fe0b269f24e0cf3dd2cf7f6e8`. Its three exact selectors each report one passed, zero failed and zero ignored. Raw log identities and owned-process cleanup are retained and authenticated. No runtime was executed during this review.

The 54 native tile rows were independently compared with the frozen Rust palette decoding. All 54 full 64 by 64 weight arrays and their hashes match the original native-PDF observations in `native-pattern-tiles.json`, SHA-256 `3a1e3bb8eee2afbae968595f27801f88fa207b8f636303f823532b639dfecaec`. The native all-pattern receipt is `bb08f1bc82d2e34ef0742095c4566974c94be6831cdb63d6497b9db000d62a2c`. Its native-save rewrite and lack of a retained exact original archive are explicitly preserved as limitations.

The actual local-export receipt SHA-256 is `81b4fe87ae1ec0e5b127c48113958bdab2022602e548fe1b9e9269f65830354b`. It records a 960 by 540 point PDF and 64-unit pattern cells transformed by 0.125, giving 8-point tiles. This authenticates the native source operand and scale, not product parity. Current product/native crop and gradient-colour comparisons are pending. Newly reported native gradient interpolation differences require their separate source-qualified correction and acceptance assessment. Neither ordinary encoded interpolation nor this review is a native gradient parity claim.

## Smells and nitpicks

Zero smells. Zero nitpicks. Return to orchestration for distinct remediation and acceptance execution.
