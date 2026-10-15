# F-X203 public APIs and properties, pass 4

**Reviewed**: Immutable public-apis-properties-4 capsule. Twelve existing carriers, complete diff2036 additions/179 deletions. Scoped audit of exports, TextReport/counts, validation, JSON template conversion, Settings and custom/app properties. Fragments, equations, controls and later all-story amendments are excluded, even where their test scaffolding appears in the copied carrier.
**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Bound input

Receipt SHA `d39d40052c65e154e4f2a1c355cf467531cead2764c0de7bb4ae2fa8f238650f` and all51 declared artifacts authenticated. All12 copied carrier SHA/size records match the full3111 source map `0c7d70019344a5eb913eb8d077a0d9e2cfe4fbb0d34f2d54226f64aee92e7bbb`. Diff SHA `64514cb077b48426825b898ac8500641c42155694c83dc7254acc188a3989a9d`. HEAD `dd7c627b534301c253615d30e35e8748b455a4d0`, declared index `c203016ed951117303df8410bfd0993697878991ea37fce9a229fc1b0770f17d`. Review reads immutable copies, not evolving worker source.

Paths below are relative to `/private/tmp/rdocx-f-x203-codex/.claude/scratch/F-X203-start/immutable-public-apis-properties-4/source/`.

## Defects

### D1, validation inspects unselected MC choices and omits selected fallback styles

`crates/rdocx/src/document.rs:46019`
`crates/rdocx/src/document.rs:46028`

The shared style-reference scanner suppresses every qualified MC Fallback subtree and accepts every Choice subtree without evaluating Requires or selecting an admitted branch. An unsupported Choice with an undefined Word style therefore produces a phantom missing-style error even when its fallback is valid. Conversely, an undefined style in the fallback selected because no choice is supported is never collected. Multiple choices can also contribute styles simultaneously. This affects the new document/module validation reports and the shared CLI consumer, not just a formatting label. Reconcile selection with the existing qualified MC provider and retain whole-part well-formedness checks and original error ordering.

This is inherited CLI scanner logic relocated into the shared provider, not a newly discovered donor branch change. The worker explicitly discloses the unresolved seam. Existing simple validation controls do not decide it. The approved validation parity and qualified physical-owner contract does not permit incomplete selected-source semantics.

### D2, four new public property types are missing from the stub export list

`crates/rdocx-py/python/rdocx/_rdocx.pyi:20`
`crates/rdocx-py/python/rdocx/__init__.py:120`
`crates/rdocx-py/src/lib.rs:150`

Runtime exports and registration include CustomFileTime, CustomProperty, CustomProperties and AppProperties, and their classes are declared later in the stub. The stub's explicit __all__ omits all four. Static star imports therefore do not expose the same public API as runtime, and the declared strict stub export gate cannot qualify this state. Add these exact existing public names to the export list. No runtime API or immutable snapshot behavior needs changing.

## Smells

0.

## Nitpicks

0.

## Not found

Correctness outside D1: zero additional findings. Template conversion checks bool before int, admits signed64/unsigned64 JSON integers, refuses nonfinite floats, unsupported objects, nonstring keys, cycles and nesting beyond256 before staging. Container conversion uses native dict/list/tuple storage, not arbitrary stringification. Publication compares serialized staged bytes and prepares revisions only on real changes. Error paths leave the live document untouched.

Contract outside D1/D2: zero additional findings. Typed format-specific export results retain native bytes and complete ordered diagnostic fields, including RTF offset/destination. Immutable TextReport and ValidationReport snapshots preserve result ownership. Character counts share the accepted body paragraph visitation used by native word_count and filter Unicode whitespace only when requested. Page count reuses actual deterministic layout and existing caller-font collection, not cached app metadata.

Panics: zero new findings in the bounded paths. CustomFileTime checks length/ASCII and fixed delimiters before bounded slices, validates Gregorian dates and explicit timezone without lexical normalization. Signed32 custom integers and finite floats are checked before mutation.

OOXML: zero additional findings outside D1. Live custom-property writes preserve existing PID/fmtid/string representation and native ordering, immutable reads clone values, Raw is exposed as bytes. App setters clone the latest complete native model, retaining unmodelled metadata through existing staged native setters. Optional tracking reads do not create settings and unchanged writes are no-ops. Native package reservation and serialization remain the ownership authority.

Tests: zero additional findings outside D1/D2. New property/date/atomicity and module-validation controls are meaningful but unrun on a fresh installed candidate in this capsule. Missing MC selection coverage remains part of D1, not implied by simple validation GREEN.

Structure: zero findings. serde_json.workspace=true reuses an existing workspace package. Cargo.lock changes only the rdocx-py dependency edge, with no version/checksum/package changes. No new module, file, trait, feature or speculative abstraction is introduced.

## Evidence limits

Actual-public-and-native-Base-1 retains15 installed failures and four genuine native332/334 failures, plus installed pre/post identity and49-match claimed baseline. Some installed failures concern excluded APIs. The equation constructor precondition failure is not a genuine missing-operation Base and receives no credit here. Actual-installed-remaining-Base-1 retains five AttributeError failures with unchanged installed provenance, three in this review scope and two excluded.

The original native check failure and malformed missing-author HTML fixture failure remain failed evidence. Actual-native-api-2 records four-crate all-target check0. Actual-native-api-3 records HTML14PASS and native7PASS, natural ESRCH cleanup/no signals/source-after exact. The extra native TextReport/Validation controls are genuine simple-case evidence, not MC-branch completeness. Actual-properties-check-1 records current binding all-target check0, cleanup/source guards true. All declared receipt/log hashes authenticated. No fresh installed GREEN, typing/stubtest, corpus parity, native oracle, hash successor or whole-feature completion is asserted. No source changes or runtime commands were performed by the reviewer.
