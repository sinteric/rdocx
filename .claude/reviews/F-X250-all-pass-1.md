# F-X250, all aspects, pass 1

**Reviewed**: Frozen working diff on `work/f-x250-codex` against `ced7b754c4c84e87782fe560db4162c656aa8fd2`, 12 files, 655 added lines and 11 removed lines. Frozen-source SHA-256 `71f17d5cc2d32ffb7573fe20be35f24ed5e525352d0cd053a8382b4d5c63350e`, exact patch `b856749d247aeb09671f9eb7fedf7fe26a5958efc8e157323ffd80397e7fbd18`. Approved plan, its HLD contract, current CLAUDE/WORKFLOW, microscope instructions and worker progress were read.
**Verdict**: 1 defect, 1 smell, 0 nitpicks

## Defects

### D1, Restart and continuation admission ignore the effective replacement level

`crates/rdocx/src/document.rs:28789`, `crates/rdocx/src/document.rs:28721`, `crates/rdocx/src/document.rs:26787`

The new restart guard and decimal continuation search call `numbering_level`, whose existing projection reads only the abstract definition level. It does not apply a concrete instance's replacement level. Actual rendering selects that replacement first at `crates/rdocx-layout/src/style_resolver.rs:602`.

A valid imported instance can have a decimal abstract level but a modeled bullet replacement at the selected level. `restart_numbering` admits that bullet as numbered, clones the replacement and publishes a restarted bullet instead of the required atomic refusal. If that instance already has a start override, `add_numbered_list_item` also selects it as a decimal continuation and authors another bullet. Conversely, an effective decimal replacement over an abstract bullet is rejected by the restart guard or omitted from the intended continuation search. This is a new consumer misusing an abstract-level inspection provider, not a request to change that existing public provider's semantics.

Use the actual selected instance's effective replacement-or-base format for both new admission paths, preserving the complete original override projection and refusal rules. Add focused imported or source-built replacement-level controls in both directions, including unchanged bytes and held-owner usability on bullet refusal, and a decimal continuation after save/reopen. The six retained local tests do not exercise replacement-level format overrides, so their green result does not cover this defect.

## Smells

### S1, Chart collection repeats whole-body discovery for each paragraph

`crates/rdocx/src/document.rs:26367`, `crates/rdocx/src/document.rs:26372`, `crates/rdocx/src/document.rs:26178`, `crates/rdocx/src/document.rs:26186`

The new bulk collection loops over every paragraph and calls both an ordinal-to-direct-owner search and `paragraph_story_location` for each. That provider searches the body, obtains the story source and scans the full body's physical items every time. The later paragraph snapshot adds another source lookup. Thus an ordinary n-paragraph document with no charts still performs n full-body discoveries, giving quadratic work and repeated source allocations for an empty result. This will scale poorly for the library's large-document use cases.

Reuse one source-qualified body snapshot/discovery pass for the finite collection and preserve the existing direct paragraph/run/drawing authority and namespace closure. Do not replace it with a relationship inventory or naive model-only discovery that loses imported source. A bounded traversal-sensitive control can prove discovery is not repeated per paragraph without a fragile wall-clock threshold. No new module or generic provider framework is needed.

## Nitpicks

0 found.

## Not found

- Chart contract: the explicit direct-main-body boundary is in the approved plan and current HLD. Multiple physical occurrences and shared targets remain separate. Unique internal relationships use the actual document part, content type and target, and data/type inspection delegates to the existing seven-family and optional-subtype providers. Unsupported data is not silently omitted.
- Namespace and OOXML: discovery retains the exact qualified parent chain and once-normalized namespace meaning. Display math chooses existing OfficeMath constructors. New reference and note-body runs carry matching character styles, and missing style definitions are added on the staged candidate without overwriting producer styles.
- Atomicity and ownership: new binding mutations use existing prepared candidate publication. Note removal resolves the current id independently of retired handles. Restart rejects unprojectable override source before publication. Scalar admission rejects bool and callback-only integer impostors before extraction. Start bounds retain the existing u32 domain and note ids the signed32 domain.
- Compatibility and structure: existing public native signatures delegate, published native structs are not extended, and the new concrete Chart snapshot lives in an existing carrier with real owned data. Exports and stubs match the additive binding surface. No new module, dependency, trait, generic parameter or product file was introduced.
- Panics: no additional finding. The helper's inherited final last-paragraph assumption follows a successful append, and new source/index accesses otherwise retain checked error paths.
- HLD: the five modified files match the approved impact list. They preserve final oracle and installed-suite debt rather than claiming worker evidence supplies native acceptance.

## Evidence and limits

Authenticated all 12 frozen file hashes, approved plan and progress hashes and exact binary patch before review. Rehashed the supplied receipt and its 12 command records. Read the actual six-case old-source failure and six-case fixed-source success summaries and the six shared-chart regression success summary. Rehashed the retained wheel, installed RECORD and all 16 recorded installed members. The receipt's unchanged product-source applicability excludes the later test/HLD-only edits honestly.

No product checks, tests, native/oracle operations or builds were rerun by this reviewer. Final integrated reporter bodies, native appearance, complete suites, package and hash/pixel checks remain pending by contract and are not findings merely because they have not run at this checkpoint. The 12 frozen source files remained unchanged at the final recheck. Earlier failed observations remain failed. No canonical worktree or unrelated documentation was touched.

Only this worker review report was written. The review pass ends here and returns the worktree write lease to the orchestrator for a separate remediation phase.
