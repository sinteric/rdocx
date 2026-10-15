# F-X211a, native remediation, pass 4

**Reviewed**: Frozen source113 native scope and provenance remediation against retained source107, including the signature-only correction after source110. Capsule `8649373936bb1f378140cba77d12a0792e5ea952ae9e2b00a898691b65f268a3`, cumulative patch `0a7f4d63fc99a29ecd3d283781f9ddf22d2beac769ca1d4b40a411b719332daa`, complete3,228-source map `16627e8f0201e3dda64d8bbdbc8e965167cb602f0f0b483fe95eda3ca1d88d75`. Current text source SHA is `3488afa8b27ba28c62a6a9f49ce39df5117ec2f29840b388952782da8ba2759d`. This is a native remediation review, not a whole-child completion review.

**Verdict**: One remaining defect, zero smells, zero nitpicks. Ordinary opaque preservation, writer wp/r authority and continuity have passing successors. Mixed-use source-local lexical precedence remains incorrect.

## Defects

### D1, earlier synthetic namespace spelling overrides proven local source bytes

`crates/rdocx-oxml/src/text.rs:3807`

The emitter retains the first lexical value for each prefix. The later local-origin flag at line3810 records that a prefix came from the actual run root, but never replaces that earlier spelling. Parsing appends the synthetic retained-binding record first at `crates/rdocx-oxml/src/text.rs:3429`, then the actual lexical local-binding record at line3444 through3468. Complete namespace facts were already decoded and normalized by `crates/rdocx-oxml/src/numbering.rs:619`.

A valid run declaring `xmlns:x="urn:lexical&#x2F;producer"`, with both an opaque sibling `<x:opaque/>` and the retained raw drawing used by the existing local-scope control, therefore constructs two records for x. The opaque sibling creates an earlier synthetic `urn:lexical/producer` record. The drawing creates the later flagged record carrying the original `urn:lexical&#x2F;producer` bytes. First-seen selection keeps the synthetic spelling, so output no longer preserves the proved source-local declaration. The local flag suppresses inherited-scope omission but does not repair precedence.

The actual strengthened control at `crates/rdocx-oxml/src/text.rs:14640` uses only a raw drawing. It has no sibling that constructs the earlier synthetic record, so its exact local-spelling assertion cannot distinguish this mixed-use case. Retain decoded complete bindings for authority and the actual local lexical record for source spelling. A meaningful mixed opaque-child/raw-drawing control must preserve local bytes and payload through repeated save/reopen while continuing to distinguish newly constructed inherited closure. This finding is a static consequence of the concrete ordering and first-seen condition. The audit executed no runtime probe.

## Smells

Zero additional findings.

## Nitpicks

Zero findings.

## Not found

- Original opaque preservation: zero additional findings. Source108 actual `document::tests::fx211_captured_namespace_origin_and_sibling_scope` and `header_footer::tests::fx201_selected_drawingml_suppresses_only_its_inactive_vml_watermark` pass without weakening their raw assertions. Writer protection now applies to actual authored consumers rather than every prefix spelling. Original source106 failures remain retained, not relabeled as successes.
- Generated drawing and relationship authority: zero additional findings. `crates/rdocx-oxml/src/text.rs:3821` identifies actual authored inline/anchor consumers. wp is protected for their canonical drawing elements, while r is protected for authored pictures whose blip inherits it. Chart payloads declare their own r scope at `crates/rdocx-oxml/src/drawing.rs:1989`. Producer-root aliasing at `crates/rdocx-oxml/src/text.rs:577` checks the target's actually established canonical scopes. Existing expanded-name, embed-ID and conflicting namespace-valued refusal controls remain intact.
- Local-origin metadata, position masks and splitting: zero findings beyond D1. The private flag at `crates/rdocx-oxml/src/text.rs:2625` joins the existing position mask. Updating boundaries at line2846 preserves all flags. Namespace-only and flagged local records follow both split pieces at line2975, while ordinary producer attributes follow the actual source side. The three-cycle header/split control at line14531 checks authored picture authority, opaque producer authority and absence of duplicated ordinary attributes. ROOT_BINDINGS scope uses actual header/document root wp authority at header_footer.rs350 and document.rs3938, and restores its thread-local state on drop. No public field or global cache is added.
- QName discovery and selective decoding: zero additional findings. `crates/rdocx-oxml/src/text.rs:3903` discovers dependencies using complete actual bindings before narrowing injected declarations. `record_external_bindings_used` at line13878 decodes only supported actual MC and xsi namespace-dependent values. Ordinary opaque values are not decoded. Earlier D2 is not waived, and the original malformed legacy-form control is covered by the actual passing source109 OXML suite.
- Metadata scanning: zero additional findings. Run emission at `crates/rdocx-oxml/src/text.rs:3490` and cached-field admission at line2080 prepare bindings once per operation. Each raw slot receives the prepared slices rather than scanning the entire metadata vector again. The actual512/1024 preparation-visit control passes. This does not claim that every preexisting mixed-boundary loop is linear.
- Cached-field admission: zero additional findings. The private helper returns the original root/complete-binding pair and populates a caller-local vector cleared at entry. Cached-field validation at `crates/rdocx-oxml/src/text.rs:2107` uses the same complete bindings and localized source slices as emission. Foreign field lookalikes retain producer authority. The new signature does not publish partial native owner state.
- Panics and structure: zero additional findings. The changes use existing concrete carriers and flag storage. No new product file, module, dependency, trait, generic, wrapper, quota or stack-limit exception is introduced. Fallible namespace processing still precedes checked owner publication. No new production unwrap or expect was found in this bounded remediation.

## Actual execution and signature continuity

Authenticated every current mapped source body/hash/size/mode, the actual source107 preimage, cumulative patch and signature equivalence record `3de111af2ac324e233520adc0bc1b6bc71a3b5587cb0d1724df8944772c92834`. Independently reconstructed the complete source110-to113 diff from the actual preserved110 body and current113 body. It equals the retained signature patch exactly. The only semantic changes move the localized vector from a tuple result to fresh cleared caller-local output. Actual source108,109 and110 production prefixes are byte-identical. Source109's718-test behavior credit is retained through this bounded signature equivalence, not asserted as a current113 full-suite rerun.

Source108 receipt `59bfa51204585848f799f4a82976a642577518fd4aa51a8c8dc17bd23a1545de` has passing original two opaque controls and the three-save/split continuity control, then the genuinely failed inherited lexical oracle. That original failure remains explicit. Source109 receipt `27220d0ba28a364a2de09328a6cc936e38623ab80cb2a81c7575f27f3b1c51ea` authenticates nine terminal successful rows, including six focused authority controls and the actual full OXML718 passed, zero failed, zero ignored result.

Source110 receipt `8f020fe492eb39e95e49c671cd81dcb4bbf89848fa393da415157e5b147b97a7` authenticates the strengthened local/inherited payload, exact local raw bytes, newly authored inherited URI closure and save/reopen fixed-point control. The source112 strict-Clippy tuple-complexity failure remains retained. Source113 receipt `a1fb271fd5bd283132ad117c680b9b72ef6eb64d356c7510959d25248e90dd09` authenticates five terminal zero rows: cheap all-target check, current strict four-crate Clippy, release compilation and the exact strengthened local/inherited plus raw/cache controls. All inspected raw logs authenticate and owned groups are naturally gone and reaped without signals.

## Integrity and boundary

Read canonical workflow/microscope, child and parent acceptance, in-progress state, progress and prior native passes1 through3. Worker HEAD remains `7f93a1e22b42cf93646953c5f1d3d34ddd6a9f5c`. Logical index remains `7daa6ae31de3979a75bf6339ec855ff8c6f7da9360956fdef2dc8a66513e869b`, complete HEAD-relative diff `19196b9bf2c838044745361e7847555818118865aa54131d4fbd90714a5fe32f`. Every3,228 frozen body remains exact before and after this report.

Word library/regression target-only work is outside this audit. No result from it is inferred here. Current installed bindings, hash/WASM/archive/HLD composition, whole-child review and final sprint acceptance remain distinct unfinished gates. This report modifies only its canonical review file, after root granted the writer gap. No source fix, runtime, target, CP, UI, index, Git reference, ledger or baseline change occurred. Review ends and returns control for a separate remediation phase.
