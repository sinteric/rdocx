# F-X216, OPC import preservation, pass 3

**Reviewed**: frozen relationship.rs42723d8fbd5d6d5208ee4b3a81475782edcbf6848b4413dc337c938090b69235 against reviewed eeee39 predecessor and pass2 residual D2.
**Verdict**: 0 defects, 0 smells, 0 nitpicks within the scoped OPC preserving-helper checkpoint.

## Prior D2 resolved

The concrete namespace_uri helper at crates/oxml-opc/src/relationship.rs:625 through :634 applies the existing quick-xml XML1.0 decoded/normalized attribute operation to actual namespace declaration bytes. The helper has three real ownership consumers: public relationship root/direct-record projection at :177, :190 and :203, preserving parser root/direct-record admission at :655, and expanded attribute identity at :691. All compare decoded actual namespace identity instead of raw lexical entity spelling. No guessed prefix, raw-name fallback or arbitrary URI rewriting is introduced.

The actual splice source and declaration bytes remain unchanged. Metadata identity normalization is used only for comparison and duplicate refusal. Conflicting encoded-versus-literal aliases refuse, equal aliases emit one expanded attribute, duplicate expanded input is rejected before helper publication and genuinely different namespace identities remain distinct. The defining control at :1203 covers all four branches. Encoded canonical relationship root and direct child declarations both remain owned at :1185, with raw untouched round trip and checked changed target. This closes the remaining consistent-decoding gap without accepting foreign lookalikes.

## BOM and preservation

The reviewed explicit BOM source origin at crates/oxml-opc/src/relationship.rs:637 onward remains intact. Every parser-relative splice coordinate maps to original bytes, preserving the original left-side BOM through rebind and merge. Source relationship attributes, opaque children, root metadata and namespace closure remain handled through existing scoped methods. Legacy direct items mutation followed by rebuilding serializer remains outside this checkpoint, as already recorded in pass1 and pass2.

## Actual evidence

Independently authenticated successor receipt aad1f5704611ab16707d61d3484311454dd31869f704d6a385aedc539f0048d1 has ten exit0 qualified safe rows with all raw log hashes exact: strict Clippy, release compilation and eight exact controls. These include both new decoded-namespace controls and the six prior BOM/alias/projection/rebind/remove-add/merge controls. No failed predecessor is relabelled.

The worker progress retains RED2 alias failures, RED3 encoded canonical root/direct-record failures and the genuine successor3 borrow compilation failure separately. Actual old failed binaries remain subject to their existing preserved identity records. This audit does not rebuild or infer unavailable historical artifacts.

## Not found and boundary

Correctness, qualified ownership, XML attribute identity, BOM arithmetic, schema/raw preservation, atomic helper behavior, test relevance and structure found no additional bounded issue. Smells:0. Nitpicks:0. Only this canonical review file was written. Frozen carrier hash matched before and after audit. No source fix, test, Cargo, probe, Git operation, managed interpreter or application ran. Whole F216 native graph, globals, public surfaces, corpus and Office acceptance remain outside this verdict. Review ended and returned control.
