# F-X260, binding cost, pass 3

**Reviewed**: fresh independent focused `/microscope F-X260 --working`, branch `sprint/s95.3`, against HEAD `6f206c834bd81266877887c09479dae0ef59a2e1`. The complete requested twelve-file incremental diff has 1495 insertions and 612 deletions, 2107 changed lines.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in this repair. D1 and D3 remain resolved. The reviewed D2 publication-order defect is resolved in the examined publishers.
**Reviewer**: Codex in this audit session. No implementation or delegated agents.

This is binding-cost pass 3, not sprint review 4. The three-pass sprint review bound remains exhausted. A focused clean result does not close F-X260, any issue or S95.3.

Read AGENTS.md, CLAUDE.md, the complete canonical WORKFLOW and microscope command, the complete approved F-X260 plan including pending intake, its progress record, binding-cost passes 1/2 and the publication design investigation. Read the cited HLD publication, lifetime, binding-test and integrated qualification sections. Confirmed F-X260 remains in-progress in CURRENT_SPRINT and BACKLOG. The investigation supplied concrete challenges, not acceptance evidence. Other dirty intake metadata was excluded from the implementation verdict and write scope.

## Defects

None, 0.

## Smells

None, 0.

## Nitpicks

None, 0.

## D1 and D3 assessment

**D1, resolved by source inspection.** `word_owner_capture` has seven parameters, with the document and removal epochs grouped, and no new lint exemption (`crates/rdocx-py/src/document.rs:4235`). The historical lint failure remains historical. The current supplied cargo check is successful but is not a new clippy execution. This review ran neither compiler nor linter.

**D3, resolved.** Word replacement and property relocation construct complete indexes before installation (`crates/rdocx-py/src/document.rs:6487`, `crates/rdocx-py/src/document.rs:6541`). PowerPoint replacement likewise prepares complete owned path and part-identity keys before scope commit, then moves the inventory and indexes (`crates/rpptx-py/src/presentation.rs:3582`, `crates/rpptx-py/src/presentation.rs:3603`). Local generation updates reserve successor capacity before live mutation. Root textbox append reserves all three map entries. Its post-commit key copies contain only inline root paths of length two or three and a slide integer identity. Deep paths and owned layout/master strings use prepared replacement indexes. No remaining allocating replacement-index rebuild after scope commit was found. This is an ordering assessment, not executed allocator-exhaustion qualification.

## D2 publication assessment

**Element XML, resolved for all nine dispatch kinds.** `replace_xml` validates captured authority and resolves the current owner before reading its snapshot (`crates/rpptx-py/src/presentation.rs:2415`). Exact submitted snapshot equality returns without publication, matching native admission. Otherwise only an ordinary slide shape or graphic frame with a proved scalar change can use the live branch. All other kinds and uncertain inputs mutate a candidate. Post-edit reads, table-presence checks, survivor construction, inventory discovery, revision preparation and complete replacement-index construction occur against that candidate before `publish_owners`.

| Element kind | Publication and lifetime examined |
|---|---|
| Shape | Ordinary slide shape requires equal optional identity and native/projected body topology plus a real scalar witness. Graphic frame additionally requires complete graphicData equality. Group, picture, connector, AlternateContent, template and unproved XML use staged native admission. Native AlternateContent remains read only. |
| Table | Candidate mutation and preparation. Selected physical table/shape authority survives, while grid, cell and text descendants retire. Changed dimensions or cell-body topology cannot use the frame scalar shortcut. |
| Cell | Candidate mutation and preparation. Selected cell survives and its former body/text descendants retire. |
| TextBody | Candidate mutation and preparation. Selected body survives, former paragraph/run descendants retire. |
| Paragraph | Candidate mutation and preparation. Selected paragraph survives, former regular runs retire. |
| Run | Candidate mutation and preparation under the existing selected regular-run survival rule. A field is not a regular-run owner. |
| Slide | Whole-part native admission and all binding preparation on a candidate. |
| Layout | Candidate admission using the layout part and its separate index identity. |
| Master | Candidate admission using the master part and its separate index identity. |

The shared write dispatch is `crates/rpptx-py/src/presentation.rs:2312`. Per-kind survivor policy remains at `crates/rpptx-py/src/presentation.rs:3782`. There is no remaining general fallback after successful live Element mutation.

**The local witness is stronger than topology equality.** `prove_local_shape_xml` uses the same concrete shape/frame parsers as native assignment (`crates/rpptx-py/src/presentation.rs:4012`, `crates/rpptx/src/lib.rs:6272`). Ordinary shape topology distinguishes an absent body from an empty body and counts only regular TextRun::Run entries (`crates/rpptx-py/src/presentation.rs:3959`). Breaks and fields cannot inflate the inventory signature. Name and accessibility witnesses compare current native values with the parsed old values and require different incoming values. Geometry requires agreement between current native fields and the parsed old transform, then a changed stored offset, extent, angle or flip (`crates/rpptx-py/src/presentation.rs:3974`). Thus native successful assignment of a witnessed input changes a modeled value rather than merely a lexical spelling. Shape roots do not take the native paragraph-token equivalence shortcut. Uncertain parse/context cases stage. Submitted original bytes still reach native validation, including frontier, relationship, signature and inherited-MC handling (`crates/rpptx/src/lib.rs:6175`). No typed projection is substituted for admission input.

Frames compare complete graphicData, including retained payload metadata, rather than table presence or dimensions alone. Changed cell payload stages. A table-presence transition is refused before live publication, either by proved preflight or candidate inspection. No XML reconstruction is used as rollback. Local preparation enumerates the selected shape's actual table/cell/body/paragraph/regular-run addresses through the shared projector and resolves their indexed identities (`crates/rpptx-py/src/presentation.rs:3825`). It does not assume slide owners are contiguous. Selected shape authority survives while replaced descendants, text, XML and applicable grid scopes retire. Local commit installs only prepared generation successors (`crates/rpptx-py/src/presentation.rs:3909`).

**All scalar callback routes examined.** Alt text uses a concrete CT_Shape setter preview, including physical attribute equality and validation. Position, size and rotation require stored direct geometry with a conservative connector/group/AlternateContent exclusion. Those proved routes prepare the property revision before calling native. Crop, style removal, connector endpoints, title, decorative, name, preset and theme-effect callbacks stage. Top, width and height use the same explicit geometry classification as left (`crates/rpptx-py/src/shape.rs:365`, `crates/rpptx-py/src/shape.rs:436`). Candidate comparison covers the complete affected part, including connector side effects. No post-live serializer, identity discovery or reconstructive rollback remains in this helper.

**Shared fill and gradient routes examined.** Equal fill exits before preparation. Non-blip shape fill prepares the property revision before the native setter (`crates/rpptx-py/src/dml.rs:198`). Blip, background and cell fills retain candidate publication. Gradient edits, gradient-stop color, pattern colors, solid color and transparency reach this same publisher. Line/cell-border fill retains the existing write_line route. Gradient token removal after successful fill publication uses retain and cannot allocate or reject. No equivalent late property preparation was found among these callers. Native setter rollback and transaction selection remain authoritative.

**Append and Word boundaries remain repaired.** Root textbox capacity, fixed newborn scopes, captured path and returned Python allocation precede native append. Only infallible constructor ID/kind binding and prepared commit follow success (`crates/rpptx-py/src/shape.rs:1885`, `crates/rpptx-py/src/presentation.rs:3650`, `crates/rpptx-py/src/presentation.rs:3747`). The root shortcut still checks missing/duplicate identities on its target slide. Other additions stage.

Plain Word XML prepares its selected successors before native admission and discards preparation on no-op or unsupported local input (`crates/rdocx-py/src/paragraph.rs:737`). The native provider admits namespaces, identity, relationships, plain topology and section ownership before assignment (`crates/rdocx/src/document.rs:27070`). Tail removal prepares the shared removal epoch and snapshot before native removal, and requires the last direct plain body paragraph, contiguous descendants and a section-only suffix (`crates/rdocx-py/src/document.rs:6003`). Other removals stage (`crates/rdocx-py/src/document.rs:11421`). Captures include the removal epoch. Removed numeric-slot reuse cannot revive earlier captures. Prepared indexes remain coherent with tail drain, local run successors, property relocation and full replacement.

The owning exclusive mutable wrapper borrow spans each prepare/native/commit sequence. No Python callback or independent revision mutation intervenes after native success. `commit_scopes` checks authority and expected state before any retirement and allocates nothing afterward (`crates/oxml-py-support/src/lib.rs:484`). Its remaining checks are invariant checks under these private sequences. No externally reachable late refusal was established. Counter exhaustion or allocator fault injection was not executed by this review.

## Independent bounded public probes

Executed only bytecode-disabled, in-memory public Python probes with `/tmp/s95.3-rpptx-venv/bin/python -B`. No persistent probe package, script or output file was created. Each probe process printed the imported extension path and SHA-256. Successful complete batches authenticated all twelve source hashes before and after and rechecked binary identity.

Imported extension: `crates/rpptx-py/python/rpptx/_rpptx.abi3.so`, resolved under this worktree. SHA-256 `51a4ac4f648b00def090e4b23787770fef809b910650ff0d3a21766f50713bbb`.

Observed passing controls:

- Exact shape XML retained saved bytes and a held run. A changed name using alternate p/a QName prefixes succeeded, retired the old run with Element.replace_xml provenance, preserved regular runs around an interleaved break and field, and reopened correctly.
- A changed name combined with foreign content in the preset-geometry frontier refused with exact bytes and the held run unchanged. An actual offset change succeeded and retired the prior run.
- A table frame on an earlier slide, after later-slide owners and another earlier-slide append existed, accepted unchanged-payload name XML, retired a held cell, preserved fresh payload and the unrelated later textbox, and reopened correctly. Table-to-opaque transition refused with exact bytes and the held cell intact.
- An imported deck with unrelated duplicate IDs refused target paragraph growth with exact package bytes and held run intact. A scalar shape-name XML change in the same deck succeeded and retired its former run. This directly challenges the pass-2 D2 reproduction.
- Equal alt text and direct geometry setters preserved package bytes. Actual alt text, title, decorative, name, position, size and rotation changes succeeded. Gradient stop editing, gradient-to-solid and repeated solid used the shared publishers successfully and survived reopen.
- Root and group inherited MC grants admitted a changed name plus an authorized opaque sibling. Rebinding that sibling to an ungranted URI refused with exact bytes and held run unchanged.
- An opaque graphic frame with table-looking XML inside its opaque payload and an entity-escaped URI accepted an unchanged-payload name change, retained its payload and reopened correctly.

Exploratory outcomes remain separate. One compound probe stopped after an incorrect assumption that foreign xfrm must refuse. That input is outside the four explicitly checked native source frontiers, so the assertion is not a demonstrated repair defect or a passing compound control. The corrected preset-geometry frontier control passed. A separate exploratory negative-extent layout was rejected during import, before any fill edit. It supplies no fill-refusal or rollback evidence. Neither stopped invocation is relabelled green.

## Supplied evidence and coverage limits

Consumed current logs without rerunning their commands:

| Current artifact | Observation |
|---|---|
| `/tmp/s95.3-resume-check.log` | rpptx-py all-target cargo check completed successfully. The user identifies the command as locked and offline. |
| `/tmp/s95.3-resume-ppt-install.log` | Fresh Maturin editable dev build and install succeeded using the named virtual environment. This is not a release wheel or timing sample. |
| `/tmp/s95.3-resume-d2-regressions.log` | 10 passed, 1018 deselected. |

Inspected the parameterized nine-kind publication test and empty-body regression in `crates/rpptx-py/tests/test_documented_examples.py:12169`, plus Word tail lifetime and refusal controls in `crates/rdocx-py/tests/test_core.py:9976`. The current table/cell cases grow paragraph topology and retain legal same-topology text-edit controls. Their earlier paused 8-pass/2-fail result remains an actual failed run caused by incorrect test expectations, not two product bugs and not a historical green run.

The ten selected tests do not individually execute every malformed/no-op, allocation, signature, group, chart or lifetime combination described by the investigation. The source audit traces the shared boundaries and the public probes add selected discriminators. Private owner/index entry equality and serial/generation exhaustion were not observed by these public probes. The existing internal Word fault control compares cardinalities rather than every mapping. Those limits are explicit and are not universal memory-failure or native-parity certification.

Local visit counters inspect actual selected traversal/index work and compare small and large unrelated packages. They do not meter every native parser, serializer, hash-map rehash, Python allocation or general fallback. Exact no-op can legitimately visit no mutation owners. No work-counter test or benchmark ran in this review. No current release ratio is inferred from counters, build success or the bounded probes.

## Explicit zero categories and pending acceptance

**Correctness and publication: 0 findings.** No remaining D2 ordering failure established across the examined publishers. **Contract and lifetime: 0 findings in this repair.** Imported duplicate refusal, selected-owner survival, descendant retirement, absent versus empty bodies and mixed regular-run ownership were examined. **OOXML: 0 findings in this diff.** Native admission retains original bytes, namespaces, MC authority, relationships and signature handling. No schema ordering or raw-payload loss introduced by the binding repair was established. **Panics: 0 findings.** The new expectations follow private fixed constructors, validated indices and prepared successor arity. **Tests: 0 new defects or smells found**, with the execution limits above. **Structure and public API: 0 findings.** No new public method, trait, generic, feature flag, crate, module or ownership-history registry. The existing workspace oxml-drawing dependency names concrete parser/transform consumers. HLD 03/10 describe the implemented boundary and its conservative fallbacks.

Inspected all 26 generated SKILL adapters without changing them. Generated markers, canonical source paths and recorded SHA-256 values matched their existing canonical sources. This was a read-only invariant inspection, not regeneration or an execution of the complete verify gate.

A1 through A4 and A6 retain final installed-product obligations. A7 has no remaining finding from this focused repair audit but is not a substitute for integrated qualification. A5/Q10 remains pending: all seven actual installed release Python operations must have strict large/small ratios below 3 at the final source, with unchanged workloads and no timing floor. The old actual release run failed four of seven rows. Native-only timing did not qualify it. Temporary historical binaries/log products were cleared while paused, so their reported outcomes remain historical and cannot authenticate current execution. Rebuilt current dev environments do not replace release acceptance.

Issue398/PR399 A8 through A10 are intake-only implementation and qualification debt, explicitly excluded from this repair verdict. Cold first-call work, exact three release ratios, whole walks, nested/related routes, table-position independence and read/edit/reopen cache correctness remain pending. PR397 is a candidate for existing A5 through A7, not accepted code or replacement timing evidence. Issue400/PR401 belongs to F-X259 intake and remains pending outside this repair. Mere pending approved intake is not counted as a new product defect.

Authentic native Word and the qualified font-oracle bundle remain unavailable. No oracle was invoked. Final integrated suites, hash, release, WASM, packaging and applicable rendering/risk riders remain pending where not qualified at the final source. No feature, issue or sprint closure is authorized by this report.

## Source authentication and stop boundary

All twelve full-file SHA-256 values matched `/tmp/s95.3-resume-review-source.json` before inspection, around the completed probe batches and immediately before report creation. HEAD and pre-existing dirty paths remained unchanged. The exact authenticated values follow.

| File | Before and after SHA-256 |
|---|---|
| `.claude/plans/F-X260-design.md` | `5a6feabf6029061d93cbf54e8869b7ae7801864634db77a4aa738305e7028c9c` |
| `Cargo.lock` | `a5023cee68bd483fd9cd542746d61faea779bd5de8834636ab30a0cb8beeebf0` |
| `crates/rdocx-py/src/document.rs` | `951a68c60e072f9a18475fc70d6cb70952ff0076e08e5ee929270c0869753487` |
| `crates/rdocx-py/src/paragraph.rs` | `c7ec2469d5c8bd07c663841f36e1748036bf3cc3855cb3a09ab86a33d34f3766` |
| `crates/rdocx-py/tests/test_core.py` | `eb6749f0416fd38152a18704e768f2391e1e5efca49e30d4c2b58600b807fc44` |
| `crates/rpptx-py/Cargo.toml` | `29eb2d7f72d6c8ef5db39c61f37278248f4e0636ff1d6b9fc7bec5940568b464` |
| `crates/rpptx-py/src/dml.rs` | `73694a7e685378b85c34bdc36665f1ac24ac9667f587fe5ee0bdfa9eb367674e` |
| `crates/rpptx-py/src/presentation.rs` | `445fd1e879f547a7ef9be0daa489fce752b594467157fe372e0fe5bb30c81028` |
| `crates/rpptx-py/src/shape.rs` | `2fef7a97a554d7925ad98816ceba6c01b2ca33a02fe2318d0de2aa518fea4710` |
| `crates/rpptx-py/tests/test_documented_examples.py` | `2148acf3642870d0fdc0dda9a4db7ae8370d7c7095e101ff3914617bf04d9f31` |
| `docs/hld/03-architecture.md` | `d3a80d0bf2f2daaf7a9b3917e98f6a93a3345a8a83743ecc3b28e0c75fdc4768` |
| `docs/hld/10-bindings-spec.md` | `697df4b4bea6ee602a3fea1f1ea6d2854d6589e4be92fafc622322951761991f` |

Only `.claude/reviews/F-X260-binding-cost-pass-3.md` was written. No source, plan, progress, state, adapter, other review or intake metadata was edited. No agents, broad tests, benchmarks, builds, oracle calls, commits, pushes, merges or tags. Stop after this report and return control.
