# F-X208 transition, field and CLI focused pass 5

Reviewer: fonts_resume, independent of the F-X208 implementation.

Scope: immutable source-production19 transition authoring and timeline composition, field/date helpers, associated Python bindings and stubs, and footer CLI publication. This is a partial implementation review. It does not qualify the complete feature, native Office paint, subsequent worker changes or installed typing.

## Input authentication

Snapshot receipt `/private/tmp/S91-F-X208-transition-field-cli-source19-snapshot/receipt.json` SHA-256 `86bb7b541757884503f065b0f81d21ed519dd9863de5f42b855948a2b3d5d15d`. All 18 compressed carriers were authenticated against both gzip and uncompressed hashes and lengths. Source19 map SHA-256 `9b2fbbb3c58523a8c156d7c779e86000940d93eaaf3264e3b19bf79fcc70d37f` was authenticated. Citations below refer to the exact uncompressed carriers retained in this capsule, not the advancing worker.

Native lib SHA-256 `c43b24b76132f4f6842bd00c1738993935ee09a03072b448a786dbeabe639a7c`, timeline SHA-256 `9ef3f168cc4e8786eb08d368f6d2078988c0f680b7666399d4eba214a63d0f9a`.

## Finding

D1. Default zoom direction disagrees between authoring and rendering. `crates/rpptx/src/lib.rs:16490` declares the first accepted direction to be the default, and `crates/rpptx/src/lib.rs:16499` lists zoom as `[Out, In]`. Python documents `None` as the effect default at `crates/rpptx-py/src/slide.rs:1547`, and a newly selected zoom can retain absent direction through `crates/rpptx-py/src/slide.rs:1536`. However `crates/rpptx-render/src/timeline.rs:334` resolves absent zoom direction to `in`. These are materially different compositions. At half progress the out branch scales the outgoing page to 0.75, whereas the in branch scales the incoming page to 0.75 (`crates/rpptx-render/src/timeline.rs:272`). A deck authored with the declared default therefore receives the opposite zoom composition. Align the default semantics across layers and add a deciding absent-direction versus explicit-default composition control without materializing an absent XML attribute merely for readback. The existing bounded zoom composition test at `crates/rpptx-render/src/timeline.rs:870` does not compare those cases.

## Six aspects

- Correctness: 1 defect, D1. No other defects found in the bounded field, date, transition mutation or CLI paths.
- Contract: D1 also violates agreement between optional authored direction and static/timeline behavior. No additional contract findings. Optional policy presence remains governed by the separate pass 4.
- Panics and errors: 0 additional findings. Duration conversion rejects nonfinite, negative and overflowing milliseconds before mutation (`crates/rpptx-py/src/slide.rs:1497`). Explicit date arithmetic validates supplied calendar operands rather than consulting the clock (`crates/rpptx/src/lib.rs:15444`).
- OOXML preservation: 0 additional findings. Qualified transition root patching, selected AlternateContent branch editing and same-effect opaque payload retention are implemented at `crates/rpptx-oxml/src/timing.rs:3887`, `:3959` and `:4043`. Type changes with unrepresentable retained payload refuse rather than discard it. Sound, extension siblings and unselected branches remain outside the authored patch.
- Tests: 0 separate defects. D1 needs the discriminating default control described above. Existing native controls cover field cache absence versus empty strings, identity allocation, retained transition siblings, checked directions and all-slide rollback (`crates/rpptx/src/lib.rs:19052` and `:19115`). This review grants no broad test acceptance.
- Structure: 0 findings. Existing carriers contain the implementation. The bindings use owned candidate publication and the CLI validates output paths before staging (`crates/rpptx-py/src/slide.rs:1625`, `crates/rpptx-cli/src/commands.rs:1351`). Field and transition stub signatures match the inspected binding methods (`crates/rpptx-py/python/rpptx/_rpptx.pyi:734`, `:1091`).

## Retained runtime evidence and limits

Authenticated native check receipt SHA-256 `644b28fb73093b85b6f502e4d4d1c75701a35b2b193a125266905c1f7032620b` records one successful all-target check. Native focused receipt SHA-256 `c32cd3c9a7eb972c55076a8148e3e0a73e7631b3fb2af4f1dfbde87556654f12` records three distinct one-pass, zero-failure, zero-ignore field/transition/footer controls. Both bind source18. Source18 to source19 differs only in the Python package `__init__.py`, so these native carrier bytes remain exact. Every referenced raw log hash was independently checked. Their recorded groups were reaped and gone with no signals.

Installed attempt3 receipt SHA-256 `6d1ca4a34302768890ac8e61dc1548ed9b0ab29f94ebe3e5be67a8624dabb278` remains failed. Its raw log hash was authenticated. Six selected tests passed and one placeholder fixture expectation failed. It is not an installed acceptance pass and is not relabelled by this review. No tests or product imports were run during this audit.

Verdict: 1 defect, 0 smells, 0 nits. Return control for a separate remediation phase.
