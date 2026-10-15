# F-X207, autofit preservation correction, pass 8

**Reviewed**: The two-carrier D1 correction in `crates/oxml-drawing/src/text/body.rs` and `crates/rpptx/src/lib.rs`, correction SHA-256 `8310f9bba734e6035f63e47f8507abaf19d646dd2655b4ef3cb0e46834bc3a5f`, frozen source SHA-256 `0057c866571db72ae656afa5c09b77c6dc8817cf3ed986b534f14e8b7e53116f`. Read the surrounding text-body parser and writer, approved preservation contract, preceding whole-feature pass 7 and retained remediation evidence.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, retaining an autofit choice can emit an unbound inherited namespace prefix

`crates/oxml-drawing/src/text/body.rs:553`, `crates/oxml-drawing/src/text/body.rs:572`, `crates/oxml-drawing/src/text/mod.rs:78`, `crates/oxml-drawing/src/text/mod.rs:197`.

The new source-preserving writer retains the original choice prefix both when the modeled value is unchanged and when the choice changes. However, the surrounding text-body parser discards attributes on the outer text-body start, and its writer emits a canonical wrapper without those attributes. A valid imported `q:txBody` declaring the DrawingML binding on that wrapper, containing `q:bodyPr` and `q:normAutofit` with a locally declared opaque attribute, therefore retains the raw `q:normAutofit` but loses the only declaration of `q` when the selected body is serialized. Before this correction the modeled autofit writer emitted the canonical `a:` choice instead. This is a new malformed-output path introduced by raw choice retention, rather than a request to repair every inherited serializer limitation.

The new unit control declares `q` on `bodyPr`, which survives through body-properties attributes, and does not exercise the outer inherited binding. Opaque attributes can likewise rely on a namespace declared outside the captured child. Preserve the bindings needed by retained source, or refuse the operation atomically. Add a discriminating valid imported body with choice and opaque-attribute bindings inherited from the outer text-body scope. Successful refresh and fit must retain the opaque payload and yield namespace-valid saved and reopened XML. Existing pure modeled canonical output and local-declaration controls must remain intact.

## Smells

0 found.

## Nitpicks

0 found.

## Not found

Correctness outside D1, panics, schema order, structure and additional contract findings: 0. The correction preserves unrelated package parts and correctly excludes foreign prefixed numeric-lookalike attributes from modeled values. No source, runtime, baseline or installation was changed during this review. This bounded correction review does not replace the preceding whole-feature review or qualify a current installed wheel. Review ends here for separate remediation.
