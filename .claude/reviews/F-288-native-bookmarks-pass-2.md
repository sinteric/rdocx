# F-288 native bookmarks, pass 2

**Reviewer:** Independent Codex reviewer `/root/review_source_pieces`.
**Reviewed:** Frozen canonical working diff against `2c8dc516`, nine product, test, plan and HLD files, 675 insertions and eight deletions. This includes the complete bounded native-bookmark remediation and the namespace correction after pass 1. No builds, tests or product edits were performed by this reviewer.
**Verdict:** Zero defects, zero smells, zero nitpicks in this bounded remediation. Pass 1 D1 is closed. The separately discovered actual alias namespace loss is also closed. This is not full F-288 native A6, release or sprint acceptance.

## Closure and namespace provenance

Pass 1 D1 was an invalid standalone reparse of part-scoped paragraph output. The revised control places the untouched output inside an ordinary Word document scope and does not inject an alias (`crates/rdocx-oxml/src/text.rs:20275`). That correction then exposed a genuine product error, retained in `/private/tmp/s954-native-named-fields-oxml-final.log`: the newly collapsed field discarded its consumed run's namespace sidecar and left an unbound physical bookmark alias. The earlier standalone failure was a setup error. The later expanded-name assertion was an actual product RED. Neither historical result is relabelled as passing.

The current repair retains the actual paragraph namespace frame only after a real own-name bookmark field was admitted (`crates/rdocx-oxml/src/text.rs:6138`). It does not infer the frame from child bindings or combine local field scopes. After the original root-attribute record is captured, the caller filters bindings already retained there and appends one namespace-only record through the existing root-attribute representation (`crates/rdocx-oxml/src/text.rs:10310`). Namespace-only paragraph declarations had previously been intentionally discarded, so preserving this frame supplies dependencies of both the internal start and the external end. Different aliases and default namespaces are covered without rewriting their physical bytes.

The guard runs before projection changes. It compares actual paragraph bindings with the existing eleven-prefix validation scope plus the generated SDT `w15` namespace (`crates/rdocx-oxml/src/text.rs:6224`). Conflicts decline only the new admission. They do not rebind generated paragraph output or add a new refusal error. This covers generated `w14:paraId`, relationship, compatibility and drawing prefixes as well as canonical Word. The existing root writer checks already emitted lexical attributes, while `bindings_outside_retained_root` removes exact declarations supplied by the original retained root record (`crates/rdocx-oxml/src/text.rs:15004`). The synthetic record therefore does not duplicate those declarations. Local declarations on surviving hyperlinks, runs and opaque children continue to shadow the exact parent frame.

The mixed-alias test resolves `q:bookmarkStart`, `z:bookmarkEnd`, foreign `x:keep` and a default-namespaced opaque child by actual namespace URI after a current-value edit (`crates/rdocx-oxml/src/text.rs:20331`). The alias control distinguishes outer `x` from a local hyperlink shadow, validates both bookmark expanded names and reopens current/default values and accepted axes (`crates/rdocx-oxml/src/text.rs:20246`). Conflicting `w`, `w14`, `w15`, `r`, `mc` and `wp` controls retain the untyped form (`crates/rdocx-oxml/src/text.rs:20388`). These assertions address the discovered namespace failure rather than merely supplying the missing alias in a test wrapper.

## Unchanged bounded rules rechecked

The helper still requires one valid isolated legacy field, pure begin/end runs, an exact own-name pair and unambiguous IDs. Other interior typed boundaries, nested fields, partial pairs and shared boundary content preserve the previous untyped fallback (`crates/rdocx-oxml/src/text.rs:6169`). Namespace metadata does not count as a physical run child. The physical start remains in field raw source and the external end remains a paragraph boundary. The direct, accepted and tracked semantic corrections still subtract only the pure begin's contribution before the ordinary span remap (`crates/rdocx-oxml/src/text.rs:6138`). No new arithmetic or indexing exposure was found.

The actual rich merge consumer still selects whole XML bookmark patching and reparse when a changed authored identity reaches a legacy-bearing paragraph (`crates/rdocx/src/document.rs:21263`). The authored-only selection excludes ordinary preserved native IDs. The `MailMerge1` fragment before `Tail` forces a nonidentity remap and checks both issue-free ranges, their text and repeated reopen (`crates/rdocx/tests/integration_test.rs:14533`). The separate facade control preserves reset defaults while editing current values and updating REF caches (`crates/rdocx/tests/integration_test.rs:14486`). The namespace correction changes no publisher, current/default policy, public API or cache lifetime. The approved plan clarification and HLD 03/04/10/12 describe these bounded rules and preserve external qualification debt.

## Consumed evidence and remaining gates

- `/private/tmp/s954-native-named-fields-frame-final-oxml.log`: 26 affected cases passed in 0.02 seconds, including the final local-shadow extension.
- `/private/tmp/s954-native-named-fields-frame-native.log`: 54 affected facade integration cases passed in 3.27 seconds. Its product source matches this review. The later difference is the local-shadow test extension only.
- `/private/tmp/s954-native-named-fields-frame-clippy.log`: scoped three-crate check completed successfully in 56.62 seconds.
- The original parser semantic RED and actual parser-only rich merge duplicate-ID RED remain in their earlier receipts. Invalid direct-import and empty-text fixture premises remain setup failures.

Reviewed source SHA-256: OXML `text.rs` is `aefef75d0b1983762432ebb3fa3693671b3fe81e0f7b8c538da3b6c7f234342e`. Facade `document.rs` is `ce6ce6a067da7bc8fc1d87b24ac719c0080d60a511e83033f7f6858d4882a5b3`.

The three installed Python parameter cases remain pending the joint rebuilt release artifact. Their source compilation is not runtime qualification. The hash refresh was in progress when this pass consumed evidence, so no hash result is claimed here. The independent native partial package proves the original discrepancy but does not qualify the complete three-kind A6 authoring and Word comparison or the dense-form visual gate. Final performance, memory, release and sprint riders remain pending.

## Not found

Correctness: zero findings. Contract: zero findings. Panics: zero findings. OOXML namespace, physical marker ownership, schema order and opaque preservation: zero findings. Tests: zero remaining local findings within the reviewed scope. Structure: zero findings. Smells: zero. Nitpicks: zero.
