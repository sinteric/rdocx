# F-X211c, actual approximation remediation, pass 1

**Reviewed**: Frozen3346-file map `615d254c5b76bea4e8509b5ade676e92a443b777d461038922a9d2920f8c2bee`, worker and immutable producer capsule. Independently authenticated every mapped body, size and mode in both trees. Only native rpptx lib.rs differs from source9dc60b0b. Its SHA is `17bf43b48cab723f400d7ee7764a032ec5492bb77986741af8cd43341660cb4f`.

**Verdict**: 0 defects, 0 smells, 0 nitpicks within this bounded source remediation. The diagnostic-epoch defect identified by actual review `4396cc573161b48d0dab745d2e6332f01b399805aa0e1a1f5bde9ea6b55fab10` is resolved in source. Installed execution remains uncredited.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Diagnostic epoch and preservation

`crates/rpptx/src/lib.rs:25017` starts fresh layout bookkeeping immediately before each final context's bounded contribution capture. `FontManager.begin_layout` at `crates/oxml-layout/src/font.rs:1141` clears only paragraph trace, layout usage and approximation diagnostics. It does not change loaded physical faces, embedded/caller aliases, font selection precedence, selected-instance caches or font program bytes.

The authoring FontManager is operation-local at `crates/rpptx/src/lib.rs:2859`. The reset occurs before its local capture, after preceding sizing trials, so it does not interrupt an active caller contribution scope. Each successful final context now owns a fresh diagnostic epoch and cannot be suppressed by an earlier sizing message. The subsequent successful measurement, bounded event capture/replay and transactional finish remain byte-identical to the previously reviewed source. Failed trials and capture/replay errors are checked before publishing facts.

Each owner retains its own context facts. Deck-level stable first-encounter Aptos/Aptos Display deduplication at `crates/rpptx/src/lib.rs:2994` and line3024 remains unchanged. Authentic physical requested faces suppress approximation through the existing provider, and Resolution events alone still cannot generate it. No message is fabricated from loaded inventory or a guessed family label.

The reset changes bookkeeping in a temporary manager, not the presentation package. Existing candidate measurement, bounded owner validation, package no-op detection, failed mutation rollback and authoring publication paths remain unchanged. No new resource, dependency, type or selection policy is introduced. The authored-shape saved-source control from corrective pass2 is also unchanged.

## Acceptance boundary

This is a static source review. It does not claim the seven installed controls now pass, infer runtime equality from Clippy or a build, or replace the genuine failed six-pass/one-fail corrective receipt. The actual source-qualified installed successor must still verify approximation ownership, deduplication and repeat/no-op behavior. Final paired provenance, complete unchanged frozen100 and declared native/render gates remain separate.

No source, test, CP environment, runtime, output artifact, dependency, Git state or native UI was changed. No build, tests, installation or runtime diagnostic was executed. Only this canonical review was written under the exclusive lease. Review ends and returns control to orchestration.
