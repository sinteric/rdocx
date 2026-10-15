# F-X195, SVG oracle, pass 21

**Reviewed**: Frozen R11-to-R12 incremental test-only delta, one existing
carrier and7207 bytes. Production SVG geometry remains byte-identical.

**Verdict**: 0 defects, 0 smells, 0 nitpicks in this bounded source review.
Runtime qualification and complete feature acceptance remain pending.

## Contract and identity

Read the approved font selection and independent static400/700 test contract,
rendering and deterministic oracle specifications, canonical microscope command,
prior pass20 and actual diagnostic evidence. Current worker is
work/f-x195-codex at2a9bf14072231e46fcb34df70f88cf7aa79fbd79,
with in-progress sprint and backlog status.

Authenticated receipt SHA256
0c9846c85137f29f6630d603f3d8ca8bc44c32d45540a35dff447c080d2c8b0e,
all317 size, mode and byte guards, and316 unchanged inputs.
Source map SHA256 is
e8ebb92abad43cdb74fe394dff99bcac5853f224b2ee05c786f3382c5e063450.
The retained7207-byte delta SHA256 is
0d96d25f8f878967a88347cb1e8684ac095ab768f0583a28b6338ea108bdc816.
Current SVG SHA256 is
6ef09085ffb2b1b7d9d54202a140b1269c62bd124158a7caf5f3ff4ee592a7da.
Independently compared the entire production prefix with the frozen R11 file.
No production change occurred.

## Assessment

The actual R11 Devanagari400 capture retains raw SSIM0.4374027655587927
and natural-length SSIM1. Independent static and variable HarfBuzz advances
match the native190.752 target. The pinned usvg cluster-width calculation
uses maximum constituent advance where its length adjustment needs the sum.
The raw mismatch remains an oracle limitation, not a product pass.

At crates/rdocx/src/svg.rs:2137, the oracle adapter derives natural advance
from independently shaped, hash-qualified static face bytes. Static400/700
selection,24pt sizing and exact HarfBuzz14.4.0 identity are explicit. At:3106,
only a unique adjacent textLength and spacingAndGlyphs pair whose target
matches that independent advance within half a six-decimal serialization unit
is removed from the oracle copy. Nonfinite, nonpositive, unknown and
nonidentity adjustments do not qualify. The explicit target-plus1 control
rejects nonidentity stripping. Raw product SVG and its raw raster score remain.

At:2255, the0.99 comparison is unchanged. The selected700-versus400 error
ratio and distinct-paint discriminators at:2260 through:2281 remain unchanged.
Each script and weight independently executes the width qualification, so
Devanagari700 eligibility is not inherited from400. Failure to qualify a
present textLength stops the test rather than silently using a weaker oracle.
The helper removes its temporary static face and directory before interpreting
the child result. Existing serde_json is reused without dependency changes.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Not found

- Correctness: no incorrect identity condition or changed production geometry.
- Contract: no nonidentity waiver, expected-output copying or threshold reduction.
- Panics: no new production untrusted panic surface. Test failures remain explicit.
- OOXML: no source XML, package, namespace or child-order change.
- Tests: raw failure remains visible and every weight requires independent qualification.
- Structure: no new source file, module, dependency, trait or public API.

## Remaining evidence

Run the actual complete SVG module and all other queued current-source checks.
Independent source review is not paint acceptance. Fresh installed binding
wheels, scoped feature gate, risk riders, hash accounting and full sprint
verification remain required. This pass writes only this review artifact.
