# F-X203, checkbox default namespace correction, pass 9

**Reviewed**: The immutable pass8 D1 remediation capsule, two existing carriers. Production changes one admission branch, with four added and three removed lines. The regression carrier adds one 53-line control. No other feature surfaces are reviewed in this pass.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Pass8 D1 is resolved in the inspected correction.

## Bound evidence

Capsule receipt `f519f5e75a1739eac448c9032fee1b98c6192f69a9f5d2b38dc33de3c25a16a3` at `/private/tmp/rdocx-f-x203-codex/.claude/scratch/F-X203-start/immutable-checkbox-default-correction-9/receipt.json` authenticates all eleven declared artifacts. Both incremental diffs reproduce the copied before and after bytes exactly. The two after carriers match the 3111-record source map `3ba2f8927d86c1792985cf1ceb42cd61b6974d9cb7d881c87c7b57d2df7250ff`. Recorded HEAD is `dd7c627b534301c253615d30e35e8748b455a4d0` and recorded index digest is `c203016ed951117303df8410bfd0993697878991ea37fce9a229fc1b0770f17d`. These are capsule identities, not a claim that later worker changes were inspected.

Production diff: `ab4dbce39fc39c5a3a06d7f0584232346f8d6a7fc62a5a5e470820b989039d09`. Regression diff: `03e021b3d4f6dc021f4c4f0d925f8fc709222d7d072b7d7ef22874e302466ab6`.

## D1 disposition

The copied `after.rs:1767` now assigns the empty prefix only to an unprefixed checkbox. The existing binding comparison at `after.rs:1780` still requires the actual Word2010 namespace. Prefixed roots use the same previous path. Ordinary Word control types retain their existing early admission, and other unprefixed extension roots retain refusal. The source-map-qualified namespace helper in `crates/rdocx-oxml/src/numbering.rs:603` records default bindings and removes an inherited binding when the same prefix is rebound. The correction therefore admits the legal default-namespace checkbox without broadening admission to foreign or rebound namespaces.

The added control at copied `after-regression.rs:400975` supplies a genuine default-W14 checkbox, a foreign same-name child and a separate foreign default root. It asserts checkbox versus untyped classification, selected D glyph and Carlito font, owned checked value, exact retained foreign payload, repeated-operation XML equality and save/reopen equality. This control would fail under the prior admission branch. Existing alias and rebound controls remain unchanged rather than being credited as newly executed here.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: zero findings. Namespace admission uses the resolved binding and leaves glyph/font mutation in the existing qualified provider.
- Contract: zero findings against the approved checkbox extension and pass8 D1 correction.
- Panics: zero findings. The changed branch adds no unchecked indexing, allocation publication or arithmetic.
- OOXML: zero findings. This correction adds no serializer changes and retains foreign payload and ordinary type behavior.
- Tests: zero findings in the bounded default-namespace regression. Its selected glyph, font and owned value checks distinguish admission from ordinary literal replacement.
- Structure: zero findings. No new module, file, dependency, trait, generic or forwarding wrapper.

## Runtime qualification and limits

Copied runtime receipt `441f65d4d3950dee586a0699464b0b6a0c0df2b47d440c61088bcb5e43d54491` is byte-identical to the retained actual-checkbox-native-default-1 receipt. Both copied logs are hash-identical to their original attempt logs. Actual native and binding all-target check exited 0. The named default-namespace control ran exactly one test, with 1 passed, 0 failed and 0 ignored. Both owned process groups were reaped with ESRCH, no signals, and source-after equality and safe lease release were recorded. The inherited row label `native-checkbox-original-Base` does not describe this result. Its actual argv and log identify the successful default-namespace control.

This review performed no runtime or source edits. Earlier Base failures and pass8 qualified results remain historical evidence. Installed bindings, physical font paint, the complete checkbox producer and whole F-X203 acceptance are outside this pass.
