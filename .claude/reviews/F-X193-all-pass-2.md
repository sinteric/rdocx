# F-X193, all, pass2

**Reviewed**: Independent root review of the final working diff against Base404a8ce23195271dd7c2e864eb9629ec6dc4eebd,6 files,143 additions and7 deletions. Scope includes the approved archive-maintenance revision and actual generated archive.
**Verdict**:0 defects,0 smells,0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness: semantic inheritance at `crates/rdocx/src/html.rs:196` retains mark highlight separately from ordinary background, captures the final mark cascade at `html.rs:3335`, and preserves child background precedence at `html.rs:3535`. The production patch is unchanged since pass1.

Contract: the approved plan explicitly requires rederived local archive records. `README.md:42` and `scripts/readme_doctests.py:405` agree with the actual archive:3525710 compressed bytes,50767756 normalized member bytes,37 members. Independent read-only archive validation returned True. No tolerance, inventory, manifest, version or publication behavior changed. Host provenance matches the observed macOS26.6.2 AppleM5Max arm64 platform.

Panics: no new production unwrap, expect, unchecked indexing or arithmetic on document input.

OOXML: no parser or serializer changes. The round-trip test at `html.rs:3956` preserves exact opaque sibling XML and custom-part bytes while asserting formatted run values.

Tests: the contributor reproducer actually failed before production changes. The focused11-test HTML group passed after implementation. The CSS precedence and ordinary background controls at `html.rs:3926` distinguish the behavior. Broader scoped lint, tests, harness and policy gates remain pending and are not claimed by this review.

Structure: one field in the existing concrete private style and existing functions, no new production type, module, file, trait, generic or dependency. HLD03 and HLD14 describe the implemented behavior.
