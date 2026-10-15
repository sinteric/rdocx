# F-X209, placeholder and compile correction, pass 21

**Reviewer**: fonts_resume, independent read-only review.
**Reviewed**: Immutable placeholder-WASM-source67-capsule two-carrier source66-to67 correction, plus the separate unadopted used-layout exception proposal. Incremental diffs are 1,592 bytes and 831 bytes respectively. Earlier pass18/19 scope is not re-reviewed.
**Verdict**: 0 defects, 0 smells, 0 nitpicks. Both finite corrections are admissible, with runtime successors pending.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Aspect results

Correctness: 0. Contract: 0. Panics: 0. OOXML preservation: 0. Tests: 0. Structure: 0.

## Source qualification

Master and Layout placeholders now validate their live owner, resolve its actual index and capture the existing Master(index) or Layout(index) ContentPath before constructing the same PlaceholderCollection. This is the identical physical path-capture operation already used by their adjacent shapes getters. It corrects the prior legacy self.path lacking the required physical ShapeOwner segment without adding a separate identity provider, generation, detached model or publication route. The existing Slide placeholder getter is unchanged.

The native import correction removes only the render cfg from the existing CT_CommonSlideData import. The type is already required by unconditional owner_data, owner_data_mut and selective source helpers. Its omission in a non-render build is a compile defect, not an intentional feature boundary. No other import or production body changes. Render-enabled binding artifacts already included this import, so the change does not alter their executable behavior.

The used-layout fixture changes only its imported exception class and raises operand from ValueError to RpptxError. That is the exact existing exception in the authenticated failed operation. Its used|inbound diagnostic match, serialized-byte equality, surviving slide-layout relationship and copied name assertions remain exact. The separate stale-handle expectation remains StaleElementError. No production exception class is changed.

## Authentication and runtime limits

Source correction receipt SHA256 83a2a363e56d370aa721e60ad32e22c17ed9d1a6b8cdfc6f5937ee2236ba0693. Exception proposal receipt SHA256 d7cb8f4be3435990a296defc6de0675c87230ce90c7a6f7de2608923d6c564c2. All 9 local and 5 external records of the source capsule, and all 3 local and 3 external records of the exception proposal, were authenticated by size and SHA256. Source maps66/67 differ in exactly the two declared production carriers.

The original installed3 outcome remains 20 passed and 2 failed. Those failures are missing physical owner in the placeholder getter and actual RpptxError for used-layout refusal. The retained installed3 strict mypy and stubtest successes are predecessor evidence only. The original WASM failure is also retained. Current source67 WASM/check/Clippy and fresh installed successor are pending and receive no execution credit in this static review. Existing native Office font-controlled output, final archive/policy and whole-feature completion remain separate obligations.

No source or runtime changes were made. This review ends at this report.
