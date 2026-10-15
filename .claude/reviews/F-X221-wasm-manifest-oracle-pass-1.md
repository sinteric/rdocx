# F-X221 WASM manifest oracle diagnosis, pass 1

**Scope**: The failed native manifest-control and neighboring static assertions in the seven selected version consumers. No execution, source fixes or broad product review.
**Verdict**: One stale test-oracle defect and one existing HLD contradiction. Zero demonstrated product dependency-boundary defects, zero smells and zero nitpicks.

## D1, WASM control still requires render-only media-validation dependencies

`crates/rpptx-wasm/src/lib.rs:490` requires `dep:gif` and `dep:tiny-skia` in the render feature. Its second loop at line501 additionally requires their optional dependency declarations. Both assumptions predate the approved semantic export provider. The real failing assertion is retained in `/private/tmp/S91-F-X221-implementation-1/rpptx-wasm-version-consumer-runtime.log:14`. This run is failed and does not qualify the sixth or seventh consumer.

`.claude/plans/F-X218-design.md:85` explicitly admits existing pinned tiny-skia, gif, zune-jpeg and tiff as unconditional facade edges for bounded semantic-media validation, including no-default, MSRV and WASM obligations. `crates/rpptx/src/lib.rs:409` actually calls PNG decoding and line434 calls GIF decoding from that existing validation path. These are concrete non-render consumers, not an unused dependency retained merely because compilation succeeds. Current `crates/rpptx/Cargo.toml:45` and line58 declare the approved unconditional edges. Rendering-specific jpeg-encoder, lopdf/miniz, PDF/layout/render remain optional in the render feature at lines25-33. F-X221 changes only this manifest's package version, not any edge or feature.

The minimal test repair is to replace the obsolete gif/tiny-skia render-membership and optional-declaration assertions with exact unconditional workspace declarations, alongside zune-jpeg and tiff. Retain exact assertions for actual optional rendering dependencies, default-template/render/system-fonts native defaults and WASM default=[]/opt-in render/no system-fonts forwarding. Check render's actual lopdf dependency and wasm_js forwarding as part of the concrete current feature list. Do not move gif/tiny-skia back behind render or delete the isolation test.

The seventh `rpptx_is_an_explicit_publication_candidate` control already distinguishes the optional rendering list from all four unconditional validation codecs at `crates/rpptx/tests/integration.rs:21196` and line21205. Its neighboring static assertions agree with current source. The other five controls retain the actual successful predecessor results and their exact version-only scope. This diagnosis gives the unexecuted seventh test no runtime credit.

## D2, older architecture paragraph contradicts the approved current media graph

`docs/hld/03-architecture.md:305` still says gif, jpeg-encoder and tiny-skia are all optional dependencies of the render feature. The same file at line2118 correctly describes unconditional tiny-skia/gif/zune-jpeg/tiff facade validation when rendering and default-template are disabled, matching the approved F218 contract and actual source. Both statements cannot describe current intent.

Update the old paragraph narrowly to distinguish animation's optional rendering consumer from the codecs shared with unconditional bounded media validation. Keep jpeg-encoder render-only, the no reverse dependency boundary and the unchanged audio/video payload policy. This is stale documentation, not evidence of an unauthorized F221 runtime graph change. Root must coordinate that HLD correction against the existing provider impact rather than silently widening a version-only source patch.

## Required retained boundaries

The WASM manifest still has default=[] and render=[rpptx/render], and its facade dependency disables defaults while enabling only default-template at `crates/rpptx-wasm/Cargo.toml:27`. Unconditional image validation does not enable rendering or host-font discovery. No native executable, corpus, no-default graph or WASM build was rerun by this reviewer. Normal remediation must preserve the failed log and qualify the corrected exact existing controls and applicable source checks independently.

Only this report was written. The review ends here and releases the worker review-write lease without source, test, HLD, build, runtime, Git or remote changes.
