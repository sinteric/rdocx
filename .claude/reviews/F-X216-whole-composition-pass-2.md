# F-X216, whole working composition, pass 2

**Reviewed**: Whole19-carrier working composition against ClaimBase `5b0f63d95e4ea82a05a3082fe74a6d8e19d9f572`, 12,893 additions and492 deletions. Frozen capsule `whole-composition-frozen-2/manifest.json`, SHA-256 `89a72941c52b04d27cb3ebc93cb91aa845164727767ade7d717522a8bc66dfb6`. Approved plan `1a0dc6c6d4def67e9b455ef6ace104983f067858332edf2413243fb14d33b5b4` remains the contract. Current full source map differs from pass1 only in the facade and existing native integration carrier. The other17 carriers retain their reviewed implementation.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Pass1 D1 and D2 are resolved in the reviewed current composition.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## D1 resolution, complete physical relationship scope

`crates/rpptx/src/lib.rs:14399` captures the actual source and destination relationship owners. When no supported edges remain, equality still compares their preservation-aware scoped XML. For nonempty graphs, each successfully matched edge records only its proven destination ID and target at14466. The source scope is rebound through that actual map and compared with the destination scope at14480. Original opaque root/record payload, order and namespace context therefore participate in equality. Lexically or structurally different authority conservatively does not reuse a destination owner.

The master layout-list exception remains precise: ignored layout edges are removed through the owning preservation operation, rather than ignoring unrelated relationship-owner metadata. Appending a new layout to an adopted master at`crates/rpptx/src/lib.rs:8771` now normalizes the actual current list through `rebind_for_import` before publishing the relationships. This avoids the ordinary serializer discarding a copied master's opaque relationship scope after its first layout is registered.

The actual unit at`crates/rpptx/src/lib.rs:33080` distinguishes owner attributes, owned-record foreign attributes, foreign children, namespace differences and child order. Each altered graph remains equal to itself. The facade control at`crates/rpptx/tests/integration.rs:35099` separately exercises both layout and master metadata: it requires a distinct copied owner, source opaque root/record bytes and order, untouched destination slide, repeated reuse of the copied layout, unchanged source and library reopen. The intermediate master-append failure remains retained and is not hidden behind the initial equality-unit successor.

## D2 resolution, real target assignment and real ID remapping

`crates/rpptx/src/lib.rs:14689` clones each selected relationship and assigns its checked destination-relative Target. External relationships retain their address and mode. At14700 the source-preserving provider receives an empty ID map because those selected source IDs still identify the physical records. Only the subsequent destination merge returns the actual collision ID map for property XML. The notes-master resource path makes the same correction at15000 before its destination merge. Target paths no longer masquerade as relationship IDs.

The direct property control at`crates/rpptx/src/lib.rs:33126` tests presentation-root and nested custom owners, a real destination ID collision, actual resolved media ownership, exact resource bytes, foreign record metadata and unchanged source relationship bytes. The integration control at`crates/rpptx/tests/integration.rs:35184` uses a nested custom notes master with a PNG resource and foreign record metadata. It checks saved target resolution, source-to-note reference remapping, original destination note bytes, source bytes and late invalid-style rollback. The original failed direct controls and failed facade scope control remain separate genuine evidence.

## Whole composition aspects

- Correctness: zero findings. The two new owning corrections compose with the once-staged source package, whole selected candidate, graph visited/copy maps and preallocated jump ownership. Graph reuse now preserves actual scoped authority. Resource transfer preserves the original source owner before destination merge. Source snapshots and candidate publication are unchanged.
- Contract: zero findings. Caller-order selection, repeated-index/map rejection, empty no-op, checked insertion, default destination formatting, opt-in source formatting, supported dependency closure and sections retain the approved behavior. Notes still have one registered global master, actual local placeholder occupancy and content, source appearance materialized before style neutralization, and atomic refusal for references to skipped placeholders. Unrelated destination notes and provider state remain protected. HLD proposals remain unapplied, with no completion claim.
- Panics: zero findings. The corrections add fallible scoped preservation and concrete mapping, not unchecked input indexing. Existing bounds admission, checked IDs and success-only publication retain their reviewed paths. This does not imply qualification of arbitrary recursion depth or every allocation limit.
- OOXML: zero findings. The repair preserves actual raw relationship source rather than introducing a serializer. Root/record metadata, namespace context and physical ordering participate in graph equivalence and copying. Source-scoped style/default inheritance, qualified lexical jump edits and notes schema registration retain the previously reviewed constraints. Foreign and ambiguous unsupported metadata remains preserved or refused before owning publication.
- Tests: zero findings. The new controls genuinely fail on the predecessor and distinguish the two original defects. Repeated facade reuse exercises the master-layout publication interaction, and the nested notes case reaches source-global/main relationship reconciliation. Existing35 native import controls include comments, table/font/global graphs, notes, source/default inheritance, jumps and atomicity. Actual ten tree controls and three named units qualify their stated boundaries. No test result is substituted for native UI or complete source-format metrics.
- Structure: zero findings. No new product file, module, dependency, trait, generic framework or speculative flag is added. Existing concrete graph comparison and preservation providers have actual callers. Public and CLI carriers are byte-identical to their pass1 reviewed source.

## Public and ordinary-path preservation

Python still allocates diagnostics, paths, returned Slide objects and tuples before `publish_owners`, retaining the checked InsertMany survivor map and no-op path. Foreign layout identity precedes stale lookup. Exact native duplicate refusal and binding IndexError remain covered by the corrected existing installed control. CLI still protects both inputs and uses path-aware package bytes before atomic output. Ordinary duplicate modeled edits, comment identity and source XML restoration remain unchanged from the admitted native pass6 and whole pass1. No public error mapping or default ordinary writer changes were introduced by this remediation.

## Authentication and actual evidence

All19 current and ClaimBase copies, reviewed predecessor library, patch `de8d8a21a83a380f2bd667881341306e17b0326df5adac708985831a3fb17fd7`, qualified binary `08b4c4047527d433d3bc7e53660b03b45eb315051558d51b2afdfdd3e1394fbc`, source map `f10b704bf684a5f1cd404228b3532ef625e8c5573b0accf22b5b5640107b2a4a`, all cited receipts and raw log hashes authenticate. All3,296 current source bodies, sizes and modes match before and after audit. The writer explicitly confirmed the freeze. Source-map comparison establishes exactly the two declared changed carriers since pass1.

Successor2 receipt `9b7fc192410fafdb2a2a3b9e93db67d9617161f1291e14cfd152a100e55efd7d` records35 selected native import passes, ten tree passes, three exact unit passes, native all-target check and strict four-crate Clippy, all natural exit0 and safe cleanup. Genuine direct REDs `4b861bc7154911e31ba3192ed28f285d1f2a403b0bab126ee6189d7f3dc412d6`, intermediate facade RED `15c6759945937e93923577a2158d81790acdfc2ae8f6aef493e42b754988b6cf` and custom notes successor `d8c15475ac08f0060d83651438afeaf70e63e2a1cf6fe92b66ae935dcd26c963` retain their real outcomes. These are actual worker evidence, not reviewer executions.

All eight current artifacts authenticate. Source notes remain `71b9df47d7d6aec67f70fbb988c956ca802b5fc04f70f10af427132bc1321987`. Current imported notes are `52b4b76a51e17343517ce5156fcd1267e2ae9e4bf95c64950579ee35f1b012b0`. Current corpus merge packages are `5e58c884df3deb6d0d2962c875729c9dd3ff7e1e5846f8acb2173b495b76bea4` and `c8b81123cd9eece03903d7fc2244c098def2ba6da1967b867573eb136ad2b256`. Prior imported2a0206 and prior corpus packages remain historical outputs and cannot supply native qualification for these changed bytes. Own-render source/imported PNG equality remains exact for its fixture, without independent native parity credit.

## Pending acceptance boundary

Fresh current producer and installed tests/types, actual current source-format metrics, current native PowerPoint package/appearance qualification, affected archives, scoped policy/riders and HLD application remain pending. The historical broad integration failure and unavailable native oracle assets remain failed or absent as recorded. This zero production finding verdict is not feature completion, full workspace acceptance, frozen100 credit or sprint/release readiness. No source changes, tests, builds, CP, UI or Git mutations occurred. Only this report was written. Review ends and returns the implementing writer.
