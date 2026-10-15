# F-X195, binding ownership conversion, pass22

**Reviewed**: R12-to-R13 incremental delta in one existing Word binding
carrier,1534 bytes,12 added and8 removed lines. This is a bounded source
review of the two layout match arms.

**Verdict**: 0 defects,0 smells,0 nitpicks. R13 compilation and complete
feature acceptance remain pending.

## Contract and authenticated source

Read the approved F-X195 caller-font measurement contract and cited rendering
and binding specifications, canonical microscope command, previous pass21 and
progress. Worker work/f-x195-codex remains at
2a9bf14072231e46fcb34df70f88cf7aa79fbd79 with in-progress feature status.

Authenticated all317 current source size, mode and byte guards. The source map
SHA256 is22fbd127ec816a75de616216c6f47fb688cd178e7ae4878fca208cbbe54d97af.
Receipt SHA256 is70bfbac3ee20c7959f1fc5b2a483b1e43ee868da89898f088640fa7cde63ec7a.
The1534-byte delta SHA256 is
198a1e77f5a7374b8b21059008c877e561ed3e4bd36383e9a469fcffc169473c.
Current document.rs SHA256 is
e240aed52e59f867d6a5014b4868efaa83c1b670ee2b1e46ab503e49003b3905.
Independently inverse-replaced exactly the two added Arc arms and recovered the
entire retained before file, SHA256
0c3646a12e16c3dc91489e7d62cfc22495e962a709558fc91b7ec53f019a40bc.
No other source change is part of this remediation.

## Assessment

At crates/rdocx-py/src/document.rs:3493 and:3542, the caller-font branch now
owns the already computed WordLayoutResult through Arc::new. The other match
branch remains the native cached Arc returned by layout_deterministic at
crates/rdocx/src/document.rs:28356. The caller-font native operation returns an
owned WordLayoutResult at crates/rdocx/src/document.rs:28615. The explicit
conversion therefore unifies the concrete match type without cloning the
layout bundle, preparing fonts twice, repeating layout or changing cache policy.

Both native errors still propagate through the same question-mark path before
the Arc is constructed. Input collection, detached work, font discovery choice,
fragment/page access, page bounds and Python output signatures remain unchanged.
The result is consumed read-only through Arc dereference in the existing loops.
The conversion introduces no XML or document mutation.

Authenticated the actual R12 Word compilation failure log SHA256
4a17e742d2a457f49c445acfd46fcf1984f989c0d46a625c583b9d9ec0a1f226.
Its two E0308 diagnostics identify these exact incompatible owned-versus-Arc
match arms. This is compile-failure evidence, not a semantic RED or runtime
pass. Existing SVG25/counter1/support-check results retain their bounded
source-equivalent qualification. R13 Word/PPT checks, fresh paired wheels and
installed caller-font acceptance still require actual execution.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Not found

- Correctness: the actual native return types now agree with the match result.
- Contract: exactly one configured layout and unchanged omitted-input behavior.
- Panics: no new indexing, unwrap, overflow or untrusted panic path.
- OOXML: no namespace, child-order, package or opaque-content change.
- Tests: original controls and tolerances are unchanged, execution is pending.
- Structure: no new file, module, trait, generic, dependency or forwarding API.

This review writes only this artifact and claims no complete feature gate.
