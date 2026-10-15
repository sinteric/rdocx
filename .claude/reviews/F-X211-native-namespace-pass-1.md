# F-X211, native namespace, pass 1

**Reviewed**: Frozen source97 namespace83 successor. Four changed carriers compared with reviewed source90,158 insertions and21 deletions in the independently reconstructed textual delta. The complete current working diff against canonical F222 HEAD has14 carriers,5499 insertions and171 deletions. This pass is limited to lost drawing-wrapper namespace closure, its direct/selected/cached-field callers and preservation-sensitive controls.
**Verdict**: 0 defects,0 smells,0 nitpicks. Namespace83 is resolved within this qualified native boundary. Maximal85 and full F-X211/public/PPT acceptance remain unresolved.

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Owning correction

`crates/rdocx-oxml/src/drawing.rs:2099` now receives actual drawing scope and actual ancestor scope separately. At line2118 the lost set compares both prefix and namespace URI, so inherited identical bindings remain external while declarations introduced or rebound on the removed drawing wrapper receive closure. Inline and anchor raw owners use the existing required-binding closure at lines2154 and2181. It adds only externally used lost bindings. Local declarations inside the retained owner still win.

The direct run caller at `crates/rdocx-oxml/src/text.rs:3172` supplies its retained run scope. The complex-field result caller at line4811 supplies its actual run scope. AlternateContent captures the qualified Choice scope when that Choice start is read at `crates/rdocx-oxml/src/drawing.rs:520`, then separately supplies drawing scope and Choice scope at line599. It does not mistake a pending resolver pop from a preceding sibling or a newly consumed drawing start for ancestor authority. The retained AlternateContent source remains verbatim and its projected drawing remains nonserialized at `crates/rdocx-oxml/src/text.rs:3197`.

The existing closure helper at `crates/rdocx-oxml/src/text.rs:13444` discovers external element/attribute and recognized namespace-valued attribute dependencies. It respects default namespaces and locally bound foreign prefixes, then changes only the retained root start tag when required. Unknown child payload bytes remain exact. Raw QName values are preserved, not renamed. An already complete owner or unchanged ancestor scope takes the exact-byte passthrough. This is no generic namespace rewrite or new scanner.

## Aspects checked

- Correctness: zero findings in prefix/URI lost-set computation, direct run/cached-field scope forwarding, qualified Choice parent capture or sibling/default rebinding. `crates/rdocx-oxml/src/drawing.rs:2339` discriminates foreign/default bindings on a preceding sibling from the selected owner's local bindings.
- Contract: zero findings. The fix closes declarations lost during actual modeled wrapper regeneration and keeps the existing native ownership and package transaction. `crates/rdocx/src/document.rs:36538` exercises a real PNG relationship, root-only Word declaration, locally scoped picture, nonmutating snapshot, checked replacement and two save/reopens. Media bytes, opaque payload and exact relationship ownership remain asserted. It does not broaden public XML or arbitrary part mutation.
- Panics: zero findings. Production fallible namespace decoding/closure propagates errors, Choice projection retains its existing Option refusal boundary, and no new unchecked indexing or arithmetic is introduced. Parent scopes are borrowed concrete slices rather than recursive ownership copies.
- OOXML: zero findings. Inline/anchor aliases, foreign bindings, default namespaces and recognized QName values remain semantically qualified. Unknown children are retained verbatim. Root-complete source remains exact. See `crates/rdocx-oxml/src/drawing.rs:2289`, `crates/rdocx-oxml/src/drawing.rs:2339` and `crates/rdocx-oxml/src/text.rs:13444`.
- Tests: zero findings. The direct/selected field and literal-whitespace control remains discriminating at `crates/rdocx-oxml/src/drawing.rs:2757`, and the nested cached-field control at `crates/rdocx-oxml/src/text.rs:14198` preserves its exact raw source. Preservation-sensitive effect/decorative controls now construct independent expected root declarations while retaining unchanged payload/entity/typed assertions at `crates/rdocx-oxml/src/drawing.rs:3168` and line3284. The successor does not weaken them to a parse-only or substring check.
- Structure: zero findings. Existing CT_Drawing, namespace helpers and raw source fields are extended for concrete current consumers. No new trait, generic, module, file, dependency, wrapper or framework is introduced. Native document production remains unchanged relative to source90. Its changed carrier contains only the new actual media regression. The OXML document carrier changes only an existing test caller.

## Original failures and actual successor

Original83 retains its genuine unbound-a modeled snapshot failure. Receipt SHA `cbffc3a58f4ee208159b6050e903cfe1150488c1d59c5b9ca0635f5ee04eb7fb` remains separate. Source93's three deciding successes did not qualify all drawing callers. Original94 retains23 passes and3 failures, including the actual AlternateContent parent-scope gap and two expectations that required unclosed raw roots. Original95 retains compilation failure, not test execution. Original96 retains26 passes and one independent expected-root attribute-order mismatch. Source97 corrects only those two expected insertion positions after the existing attributes, preserving all opaque payload checks. None of these originals is relabeled passed.

Final receipt97 SHA `d71b498cdacefc35ae6c6abfdc5c8d95304ff8aa9c9f8389565b1131f168100b` has eight actual exit0, qualified, non-timeout rows with reaped leaders and gone groups. These are affected all-target check, all27 drawing-module controls, paired release compilation and five distinct direct release controls on2097152-byte stacks: lost-wrapper bindings, real local-picture snapshot, nested cached-field drawing, drawing identity collision/repetition and table/cell foreign-w replacement. Every retained log hash authenticates. Current Word executable SHA `eb29514167420f7abe31d17e5df66aafa665408c677a4b33b5d7afcbfcbcd68b` and OXML executable SHA `4adb8a8764ae512c705e5197ede05f83a035e9d33d618bd9f27ae24b006f371a` were independently rehashed. No test or runtime command was run by this reviewer.

## Boundaries and source integrity

Checkpoint97 SHA `bf88518e96d32152701fbd004f9a011f626a8ee11ec82803fd8b2ebe49605822` and source90/source97 comparison SHA `266d08f9adaf77cd87a3b93871c6c2171057d9c69ac07795f31c606e69ed3925` authenticate. All3205 entries in source97 match bytes, size and mode. Source-map SHA is `9434b1b0027cc6848108bfde71d0f16a7628daed11d7d63c0c78e4c72779a8d6`. The only four source-map differences were independently checked against preserved source90 bodies. Source95/96 are not certified as final source.

HEAD remains `00b0d4eaec2b157a1f2555badd5cbbe89e0eb787`, branch work/f-x211-codex. Full binary diff SHA is `2b8bbe54e668d49f2b32b8fc0c601b0b1e17a46df57d11f02256320b908413c4`. Logical index SHA remains `a8c103650a04517f4eb9c9216089c30817da8a2ecefcd5b5a8eb1802a885e183`, physical index SHA remains `03dc45d44ada1ad908a43ab2552901f3843357a21f96a8dc8d6bf5dd24596cab`. Source and diff are rechecked after this report is written.

Maximal85 selected-transaction timeout remains unresolved and is not inferred fixed from these ordinary controls. Existing resource/owner reviews retain their distinct scope. Public Word/PPT owners, generations, frozen100 corpus, native Office, final scoped acceptance and completion are not certified by this bounded report. Only this review file is written, then control returns to the implementer.
