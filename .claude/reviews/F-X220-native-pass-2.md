# F-X220 native pass 2

**Reviewed:** Frozen native remediation on `work/f-x220-codex`, HEAD `e918fa29195153a6d6cbda179a269366033fe6f5`. The complete four-carrier working diff contains 3,327 insertions and 183 deletions. This pass reviews native pass 1 D1 and D2 and affected behavior. The model and manager carriers retain their preceding reviewed hashes.

**Verdict:** Zero defects, zero smells, zero nitpicks in this bounded native pass. Both previous findings are resolved. Full-feature acceptance remains pending.

## Resolved findings

### D1, removal target preflight

`crates/rpptx/src/lib.rs:12130` validates the actual source target before taking the face snapshot or removing the modeled face. Whole-entry removal preflights every removed face at `crates/rpptx/src/lib.rs:12159`, before snapshot construction and publication. The candidate model may be edited locally before validation, but the source is not published until all checks succeed.

`crates/rpptx/src/lib.rs:12226` propagates target validation errors before checking preserved references. A retained opaque reference therefore no longer bypasses target validation. Foreign, missing, external and non-font targets refuse rather than producing a successful removal.

The controls at `crates/rpptx/src/lib.rs:23480` and `crates/rpptx/src/lib.rs:23496` cover both APIs, five invalid target cases and presence/absence of a preserved opaque reference. All twenty scenarios assert an error and exact unchanged package bytes plus presentation XML. Existing valid shared-part and opaque-reference controls remain in the native selection.

### D2, prepared timeline font tier

`crates/rpptx/src/lib.rs:1661` now selects the existing configured manager for the public timeline path. The branch at `crates/rpptx/src/lib.rs:8248` retains prepared embedded and caller tiers rather than rebuilding from the caller-only input slice.

The genuine control at `crates/rpptx/src/lib.rs:23531` embeds Liberation Mono under a distinct uninstalled alias. It verifies selected static font bytes, then compares actual timeline glyph IDs and advances with static output. The signature helper at `crates/rpptx/src/lib.rs:23511` reads both ordinary and MultilingualText runs and requires actual expected text. The control also proves caller override, unchanged source bytes and a distinct missing-flag negative.

The retained genuine RED failed on different actual glyph IDs and advances, not on an empty signature or missing symbol. The earlier helper-empty failure is not credited as the deciding RED.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Not found

Correctness, contract, panics, OOXML, tests and structure produced no further findings in the affected native remediation. All-face preflight, preserved-reference validation and prepared timeline selection resolve the reported failures without changing the reviewed codec, model or manager contracts. Existing decoded-body sharing, source-preserving graph ownership, tri-state flags and caller ranking remain at their reviewed state.

## Authenticated evidence

Checkpoint SHA-256 is `a1004decadbc3b82f506af9ee3f1deebd7fee50546ad18430d2eaf3a1dd99a41`. Current native PPT carrier SHA-256 is `46e966e9b252f194bb4c6918b274d4ac97a2d57e48841b69ad5bbef06243efd8`. The retained patch and current tracked binary diff SHA-256 are `4dbff8e8b8c20ae4b69401c2e97ef8174373228a2f2b13e99c075bc36a80711c`.

Authenticated all four frozen carrier hashes, the patch, five remediation receipts, their source inventories and every actual row log hash. The final successor receipt `eae9eaa5688b2fded2d20439f246e5669f0c7a218b5c28b89a341ed7750448dc` reports eleven native F220 tests and 61 manager tests passed. The final check receipt `25937d7d40f172c41f0f77fe32fb04b2a665607c47bd67dea4b5948a5a8bc1cc` reports the existing F195 all-output control and eleven F207 tests passed, plus all-target/all-feature check, strict Clippy, formatting and diff checking. Both complete source inventories match all 3,199 current tracked files. Their source guards and safe process cleanup receipts are retained, with qualified terminal zero exits.

The face-removal, entry-removal and genuine timeline RED receipts remain actual exit 101 failures with safe cleanup and unqualified results. They are not counted as passes. No runtime command was rerun during this review.

The manager selection explicitly excludes the still-pending F206 physical font oracle. Native qualification uses the recorded 8 MiB stack and retains the default-stack limitation. Final changed-source 49-output checks remain due. Word, Python, CLI, native Office, package archives, HLD and full-feature acceptance are outside this bounded pass and remain required.

## Review integrity

Captured and checked HEAD, branch, physical index, logical index, binary tracked diff and all 3,199 tracked file hashes before review, immediately before writing and after writing. The complete tracked-source aggregate remained `c73b2b5dace641f14512590e278405c862f82f17cc9bc878c8ee200d1169edb7`. Physical index remained `9fb296d122bd2a99880625492750520bdf6c3aaefd633dbe429344b7d2dee883`.

Only this review file was written. No source, plan, progress, HLD, test or formatting edit occurred. No Cargo, CP, Office or public-consumer execution occurred. F-X215 remains frozen. This review ends here and returns to the orchestrator.
