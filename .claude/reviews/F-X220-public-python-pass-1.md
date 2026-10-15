# F-X220 public Python review pass 1

**Reviewed:** Frozen qualified Python phase on `work/f-x220-codex`, HEAD `38d1697b1d56bf97f7eabc4c02eb6996acf0ae3d`. Twelve public carriers, 1,022 insertions and zero deletions, from checkpoint2 and exact patch SHA-256 `7ac8a9092b69fdbbf59316a0d6270a8de4fb49f352d15f353776cbd9e9be7588`. Seventeen preceding native carriers remain qualified separately, including current provider composition. This pass audits the Python phase, not complete feature acceptance.

**Verdict:** One defect, zero smells, zero nitpicks.

## Defects

### D1, binary-reader preparation treats a short read as complete input

`crates/rdocx-py/src/document.rs:8594`
`crates/rpptx-py/src/presentation.rs:3249`

Both binary-reader branches call read(64MiB+1) once, then return a copy of that one bytes result. A binary reader may return fewer requested bytes without reaching EOF. For example, a reader over a valid font may limit each call to8KiB. The exposed BinaryIO input then feeds only the initial chunk to the native font validator and falsely refuses the valid font as truncated. Conversely, a stream returning a valid sfnt as its first chunk while more bytes remain is accepted without examining its complete input or enforcing the advertised cumulative input bound.

Bytes input and filesystem paths are bounded correctly, but those checks do not repair the reader branch. Read bounded chunks until an actual empty bytes result, check each borrowed result and cumulative length before copying, retain fallible allocation, and propagate invalid reader output or errors without publication. Add deciding public controls in both families for a valid short-reading stream, cumulative over-limit chunks and an invalid later chunk or read error. The PPT authored EmbeddedFont constructor uses this same preparation and needs its successful short-reader path covered. No runtime reproduction or remediation is credited by this review.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Aspects checked

- Correctness: D1. Zero additional findings in native delegation, license and kind matching, derived/explicit Word keys, tri-state settings, snapshot construction or publication.
- Contract: D1 affects the approved bounded binary-reader input. Zero additional findings in actual native-backed Word/PPT APIs, immutable snapshots, four face mappings or explicit caller authorization.
- Panics: zero findings introduced by the Python phase. Borrowed bytes length checks precede fallible copying. Path reads enforce the bound while reading. Snapshot vectors use checked reserve and fallible Python allocation. No unchecked input indexing was added.
- OOXML: zero new findings in this phase. Binding operations use the native graph/settings provider and do not add a Python package or XML editor. Existing native producer-format and preservation qualifications remain separate.
- Tests: D1 lacks short-read and cumulative multi-read controls. Zero additional findings in actual exact installed membership, license/no-op/owner tests, real bytes-allocation refusal or shared-part alias controls.
- Structure: zero findings. Concrete input and snapshot code stays in existing binding carriers, uses real native result types and completed revision owners, and introduces no product file, dependency, trait, generic or second revision engine.

## Publication and ownership audited

Word font listing uses checked native try_fonts, followed by immutable FontDefinition/EmbeddedFont snapshots. Font keys, subset flags and actual license metadata are retained. Descriptive set_font refuses embedded snapshots and delegates to native validation. Removed snapshots are allocated before publication. Word embedding and all three setting setters stage native candidates and reuse `crates/rdocx-py/src/document.rs:3846`, whose exact byte no-op returns before revision publication.

PPT operations at `crates/rpptx-py/src/presentation.rs:1805` enforce explicit license identity and matching authored input kind/license, then delegate to native authoring. Read-only format, part, relationship, stored-length and decoded-byte facts remain native snapshots. At `crates/rpptx-py/src/presentation.rs:3331` decoded bytes are shared per actual resolved part during listing. Font removal allocates its snapshot before publication. The helper at `crates/rpptx-py/src/presentation.rs:3409` compares full candidate bytes, proves unchanged owners and publishes through the completed provider. No-op, native errors and snapshot allocation failures leave the original package and captured owners unchanged.

All new classes are frozen. Tuples and bytes carry detached ownership. Exports and final constructor stubs agree with the actual PyO3 surfaces. Pending CLI and full HLD work are not implementation defects in this explicitly bounded phase.

## Actual evidence authenticated

Checkpoint2 SHA-256 is `d6d9e2b22199255680f6703f829ef12b30f79be10a75b05b480ed120152aeffe`. Authenticated every pinned evidence file and retained receipt/raw-log hash. Final receipt `bef321018369a44b2b2e10f353118b67a9ffa4966b782faeb56b4520393f8fe1` records nine qualified zero-exit commands: fresh offline installation, exact four-node collection and execution, both strict mypy consumers, both complete stubtests, actual allocation controls and alias controls. All command groups are gone and leaders reaped. Owned installed-prefix/source before-after equality is retained. This reviewer performed no runtime or package import.

The actual allocation control wraps CPython's object allocator in an isolated child, refuses one allocation of the font bytes object's exact size and restores the original allocator in finally. Six real MemoryError cases cover listing, remove face and remove record in both families. Full package bytes and held owners remain unchanged, followed by real successful unarmed successors. This is bytes-allocation failure, not a tp_alloc interception or simulated exception. Result SHA-256 is `8358a67207702cc1efdf04dfa6189b240c64b3b16bd984bce3865c517e8ddf5c`.

Three alias controls, results `4db9d503a1d65053320abb854522471cc5e917e66a7cc001dcf857ce801f4230`, prove one Python bytes body for2000 actual PPT aliases, copy-on-write replacement and reachable-source retention. Two Word aliases prove shared-source retention. The2000-reference Word owned snapshot exceeds its real aggregate bound and refuses while package and held owner remain unchanged. These are source-built graph controls, not Office interoperability evidence.

Independently verified all current wheel members, complete RECORD digests/lengths, Python/stub source bytes and native target equality. Word wheel has twelve members and SHA-256 `51ff084247341a5c80a11c6bac41385ef2d2c5345b52ebd3ca7c8b264851469e`. PPT wheel has fifteen members and SHA-256 `27157bec502574eda6cb7861798c888a0dbbc6c25ee7456ed1a15a8b5678040e`. No managed CP installation is claimed. Initial compile, fixture flag expectation, typing narrowing, constructor stub and UV argv failures remain retained as genuine setup/implementation checkpoints, without product RED credit or relabelling. Current binding check, strict Clippy and format successors pass separately.

Current native provider composition receipt `9598e4ed4cb5fc50bf3d84df6037455623d584c2cb3f5f443002ead39b5b0de1` retains actual checks and eleven Word plus eleven PPT controls. Prior native and settings reviews remain authoritative for their bounded carriers. Mechanical existing bibliography font-manager adaptation is not resumed bibliography work.

## Remaining boundary

D1 needs separate remediation and independent re-review. CLI, final HLD/recipes, current archives, full feature/risk gates, required corpus and native font-bearing Word/PowerPoint producer and face-use authority remain due. Office activation is still pending permission. Isolated wheel or source-built alias results do not substitute for native Office or root managed CP acceptance. No feature completion, sprint closure or release credit is granted.

## Review integrity

Read CLAUDE, canonical workflow/review rules, approved plan, relevant native/public HLD references, prior native/settings reviews, progress and current in-progress sprint rows. All3,205 tracked-source hashes, exact twelve-carrier patch, HEAD, branch, complete tracked diff and physical/logical index match before and after review. Full diff SHA-256 remains `6779e35a871815349062945c63c23b046bd5fa3adf6a95d011476ef16a25d6ac`. Physical index remains `0de54845b89f71bb5f690d1247df7eb853a1fae5090625b670c03bea1d5c482d`, logical index `3e55920e60cde135bb08358e9c9b71be1e035f8e11dfdeb9ad888c635062da75`.

Only this review file was written. No source, progress, plan, HLD, tests, ledger, runtime, Cargo, Git mutation, CP or Office edits occurred. This pass ends here and returns control and the writer lease to the orchestrator.
