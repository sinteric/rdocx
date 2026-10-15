# F-X195, provider, pass 5

**Reviewed**: All six aspects of the four existing-file provider stage.
The stage diff has 1,352 additions and 59 deletions, 73,996 bytes, SHA-256
cb8e85a2fab8daac3150fd4fddd06ab5aec705986585b94925874bef563fca70.
Receipt SHA-256 ebf5c6bbf527ca26d2e109fa962ce83ea964081778b09a5f3f8b6fb8aa69838b.
The complete eleven-file working diff is 141,256 bytes, SHA-256
19c464fb3cec9f3df53a2726052ef65f2e8df4c11f31ee852fcd3683930ff14a.
All eleven current byte guards match the receipt. The seven nonprovider
files remain identical to the reviewed R5b prefix. HEAD is
2a9bf14072231e46fcb34df70f88cf7aa79fbd79.
**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Defects

### D1, cache eviction still treats ordered events as FontId values

`crates/rdocx-layout/src/engine.rs:3831`,
`crates/rdocx-layout/src/engine.rs:3838`,
`crates/rdocx-layout/src/engine.rs:3845`.

The paragraph, table and header/footer carriers are now Vec<FontTraceEvent>
at :2874, :2895 and :2929, but each final retention predicate still passes
its event directly to current_fonts.contains. current_fonts is a
HashSet<FontId> built at :3822. FontTraceEvent is a distinct enum, with no
Borrow or conversion implementation that makes this lookup valid. The
ordinary engine therefore cannot type-check. These three consumers must
project the selected FontId from both Resolution and Contribution before
checking membership, preserving the existing eviction policy. This is a
source-level finding. No Cargo was run during review.

### D2, the required cross-request warm-cache attribution control is missing

`crates/rdocx-layout/src/engine.rs:21498`,
`crates/rpptx-render/src/text.rs:3738`,
`crates/rpptx-render/src/text.rs:3750`,
`crates/rpptx-render/src/text.rs:3756`.

The approved provider contract requires Aptos then Calibri and reverse
ordering through warm/cold reuse, plus Aptos Display sharing the same actual
fallback. The Word control creates a fresh engine for every requested family.
The PowerPoint control likewise creates a fresh manager/cache per family and
compares only the intentionally Resolution-only legacy projection. It cannot
fail when final Contribution attribution collapses Aptos and Calibri on a
shared warm PowerPoint cache. The new Carlito multilingual and field/marker
controls prove other routes, but do not supply this discriminator. The direct
carrier test at crates/oxml-layout/src/font.rs:3735 records caller-provided
family strings manually, so it cannot cover native request propagation.

Add the bounded request-aware native control in the existing test modules:
reuse the same manager and relevant cache/engine across both request orders,
then revisit each request warm. Assert original request inequality alongside
same fallback bytes and exact final selected-instance/glyph identity. Preserve
the legacy Resolution-only check separately. This is an absent required
source control, not a claim that an unexecuted test failed.

## Smells

0.

## Nitpicks

0.

## Not found

- Correctness: no additional finding in scoped generation, nested/foreign
  refusal, reset, empty/probe exclusion, staged contribution removal or
  whole-carrier validation before replay mutation.
- Contract: no additional finding in the one ordered Resolution plus
  Contribution carrier. Legacy public signatures remain Resolution-only.
  Word final text, field, symbol, marker, ruby and annotation requests and
  PowerPoint text, field, marker and multilingual requests retain private
  original request context rather than reconstructing it from FontId.
- Panics: no additional finding in checked generation, entry and byte bounds,
  capacity accounting, scope range checks or replay admission. The guarded
  scope unwraps follow validated immutable state without an intervening
  operation that can remove the trace.
- OOXML: zero findings. This stage consumes existing typed projections and
  changes neither serialization, namespace ownership nor child order.
- Tests: no additional finding beyond D2. Source controls cover nested,
  stale and foreign scopes, atomic invalid replay, owned-string capacity,
  overflow/empty bypass, final route attribution and ordered replay. The rich
  Word test explicitly preserves cache refusal and distinguishes recomputation
  from a cache hit. Latin Word and PowerPoint cache-hit controls are separate.
- Structure: zero findings. The concrete provider has two actual native
  consumers. No new module, dependency, generic parameter, trait or public
  TextSegment field was introduced.

## Evidence limits

The approved plan, cited rendering, bindings and toolchain contracts, prior
core pass 4, frozen provider diff and source callers were read. Prior actual
Word R3 and PowerPoint R4 Base failures remain distinct evidence. This provider
stage and its new tests are uncompiled. Runtime, no-default-features, WASM,
output-instance controls and the full feature acceptance matrix remain
pending. This pass does not certify the remaining facade/binding wiring,
package or HLD work. No source, scratch, progress or runtime state was changed
inside this review.
