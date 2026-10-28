# F-X268, warm picture inventory read reuse, pass 24

**Reviewed**: Working diff against ef048ffbd462719ddbf6977ba0547f8276dfb6e0. Seven tracked files, 363 insertions and one deletion. Canonical freeze0649fc35593a03502b64f6311917e63ebf52c5c68ccea63079ebb20b16598f02, diff2d6a8fc806584ec1711db40be307fc517d395c8a3cacef98b9cf58ee07aadd8a and all seven path hashes match the actual tree. Native document.rs SHA is b55581cb484df35479cc2262e13a99641f23a77fe182c704984abe007b387029.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Independence and scope

Codex review_source_pieces performed this independent implementation review. This reviewer authored earlier F-X268 producer-proof work and temporary diagnostic/profile infrastructure that identified the warm inventory cost. Shared authored the inventory mechanism and temporary controls, and Root ported the canonical implementation, tests and documentation. This reviewer did not implement the inventory mechanism. During preparation the reviewer identified an assemble.part=None fixture assumption, corrected by the implementer before compilation. That setup correction is not a semantic RED.

The canonical workflow, microscope command, active F-X268 contract and current progress were read. The feature remains in progress. This pass covers the new inventory diff and its interactions with existing proof publishers and both picture insertion consumers. It does not repeat the previous clean union review or independently qualify final timing, memory, native rendering or sprint acceptance. Only this review file was written. No compiler, test, renderer, profiler, formatting or source repair was run during this review.

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Source and contract audit

The approved bounded policy is at `.claude/plans/F-X268-design.md:608`. `MainPartBlocks::picture_free_paragraph_ends` at `crates/rdocx/src/document.rs:9124` requires the exact current part partition and an existing empty-frame context. It checks recorded root/body declarations and bindings, reads only successful already populated ParsedChild facts and restricts output to a direct Paragraph with reusable private facts. Missing proofs remain missing. Staged writer models supply no read authority.

`plain_picture_free_paragraph` at `crates/rdocx/src/document.rs:10607` checks the complete paragraph and run side collections. Opaque children, alternate drawings, fields, private markers, controls, revisions, equations, rubies and hyperlinks decline. Its run-content admission is limited to text, tabs and breaks. No new retained field, public API, trait, module, feature or process switch is introduced. The paragraph map is local to one inventory call.

The complete inventory remains at `crates/rdocx/src/document.rs:29719`. Main and related sources retain their order, with the existing related-source provider at `crates/rdocx/src/document.rs:26508`. The prepared main read shares the validated writer and body-span fallback at `crates/rdocx/src/document.rs:24885`. Owner traversal, optional batch discovery, inherited and target namespace handling and closure occur before the skip. The exact main-part identity and whole physical paragraph start/end check at `crates/rdocx/src/document.rs:29816` prevents nested or related paragraphs from borrowing direct-body facts. The original fragment parser remains at the same per-paragraph error-order position after the optional skip at `crates/rdocx/src/document.rs:29826`.

Both binding consumers still run full candidate.pictures before locating the returned occurrence and preparing conservative publication: `crates/rdocx-py/src/document.rs:15520` and `crates/rdocx-py/src/table.rs:2208`. No target-only early return or shortened atomicity validation was substituted.

## Literal frame provenance

ParsedChild records namespace scopes rather than a second literal head/tail copy. The full-frame authority is composed with the existing immutable constructors, as the approved clarification states at `.claude/plans/F-X268-design.md:614`.

- `MainPartBlocks::clone` at `crates/rdocx/src/document.rs:8932` copies the identical head, tail and frame with the same immutable child Arcs.
- `assemble` at `crates/rdocx/src/document.rs:9398` compares the actual final head/tail against the kept frame. Changed frames call fresh_context_piece at9436. That constructor at8628 clears the parsed-read lock and every contextual proof.
- `edited` at `crates/rdocx/src/document.rs:9063` preserves unchanged frames for body edits and refreshes all children when the tail changes at9100.
- `source_blocks_in` at `crates/rdocx/src/document.rs:25177` carries neighbors only after exact old head-prefix and tail-suffix qualification. Changed children receive new actual reads.
- The producer builder at `crates/rdocx/src/document.rs:25011` creates new source scans under its actual frame. The leading-whitespace bootstrap refreshes every carried child at24996.
- `install_carried_blocks` at `crates/rdocx/src/document.rs:25341` installs an exact part-qualified record. It does not reframe child proofs. Layout child-scan constructors start with empty parsed-read locks.

The additional namespace-probe assemble caller at `crates/rdocx/src/document.rs:34505` supplies the same kept frame and inherits the same invalidation. No actual publisher examined transplants an existing parsed child into a different literal frame. Direct private-field mutation retaining old proofs would violate this constructor invariant and is not evidence of a reachable stale publication.

## Acceptance evidence examined

The canonical scoped receipt at `/private/tmp/s954-warm-inventory-canonical-scoped/scoped-results.json` is hash-bound by the freeze. All recorded child exits and log hashes were checked. Sixteen unique exact native selections each actually ran one test and passed. The new five controls at `crates/rdocx/src/document.rs:51372` through51562 demonstrate successful existing reads, staged-only/missing and stale refusal, no lazy fill, real assemble and tail-edit invalidation, private/opaque fallback, real parser restoration under force-off, and unchanged AlternateContent picture bytes and dimensions. The frame fixture first proves part=None refusal, then publishes the actual output before checking that old child reads remain unavailable.

The other eleven selected controls retain actual picture refusal atomicity, source/writer separation, retained canonicalization, namespace boundaries, alternate owner selection, duplicate drawing identity, one-source inventory and existing counted traversal behavior. The all-target/all-feature check, clippy, fmt, prose and diff checks all exited0. These are consumed outcomes, not executions by the reviewer.

Temporary actual normal-cfg mechanism evidence is separately preserved in `/private/tmp/s954-warm-picture-proof-prototype/RESULT.json`. Twelve ON and twelve OFF whole-public windows exercised both pinned Document and Cell calls at25/100, including both warm prefixes. Exact saved/reopened parts, media, relationships and returned occurrences agree. Actual skips are33 and100, while the real disabled shortcut restores exactly that many fragment-parser attempts in each window. The eight existing Python controls also passed, including unrelated unreadable attributes and held-state atomic refusal. Temporary diagnostic exports and environment switches were not ported. Neither these opt1 observations nor the native proof counts are a normal release timing result.

The existing plan and exactly five approved HLD files describe current intent, fallback and pending qualification consistently. No promised79-flat, five-linear,252-call, Original407, memory or final union obligation was removed. Actual successor normal timing and applicable remaining riders remain pending. Earlier failed normal measurements remain failures until their actual successor gates pass.

## Not found

Correctness, contract conformance, panic and bounds handling, OOXML preservation and namespace context, source/writer authority, physical owner identity, failure order and atomicity, discriminating tests and evidence provenance, structure, and HLD scope produced zero findings in this bounded pass.
