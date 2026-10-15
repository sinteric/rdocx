# F-X210, remaining export routes, pass 14

**Reviewed**: pass13 D1/D2 remediation, exactly two current carriers and12648-byte incremental diff, plus complete surrounding notes, handout and timeline diagnostic assembly.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication and authority

Worker work/f-x210-codex remains at HEAD `7bbc960e54d2b5acf6763c8d6d13b3796c1a5a7a`. Read approved plan, companion policy, current progress and pass13. The current packet is `.claude/scratch/F-X210-remaining-routes-r1/r2`. Receipt SHA256 is `d89aae1f2a40c3ddb4c6466efba429ea6e8656e581559b7484f5df3135adb943`. Incremental delta SHA256 is `f8c501264a518cce3735dc864d26a880420aeeabdb484f97cb039ed56a207b7d`. Authenticated every2909 current byte, size and mode record against map `967978d24b5976f31bc7179a589b99875d871a50fb6aa492e84d33c018524da6`. Only rpptx/src/lib.rs and rpptx-py/tests/test_documented_examples.py changed relative to cab15d, and all2907 other records remain exact. HEAD and staged-index digest `7aef5efe3c9b04d5d50070222990762ac9fd664bbbec4f59fa2d13631c1437be` match the packet.

The reviewer authored earlier companion and HLD proposals, not this separately authored remaining-route remediation. Unchanged HLD relies on independent clean HLD pass11. This pass does not assert independent self-review of earlier authored text or whole-feature runtime acceptance.

## Defects

0 defects. Both pass13 findings are resolved in source.

D1: `crates/rpptx/src/lib.rs:11050` captures the actual manager font facts after successful export layout and removes only their appended suffix from the ordinary diagnostic vector. The inspected renderer at `crates/rpptx-render/src/lib.rs:337` appends exactly that current sink, without an intervening manager mutation, so the subtraction and split have a concrete invariant. Notes and handout exporters retain the ordinary vector and merge the separate actual facts through `crates/rpptx/src/lib.rs:10292`. The seen vector starts with the original preview's current font facts. First occurrence order is retained and equal facts are deduplicated, while ordinary warnings are appended unchanged. Different selected-family facts remain distinct output decisions, rather than being discarded under a broader requested-name prefix rule.

D2: `crates/rpptx/src/lib.rs:4885` begins the actual timeline manager lifecycle before the incoming and outgoing resolution/layout operation. `crates/rpptx/src/lib.rs:5034` appends that operation's accepted font facts to the returned frame. It does not copy an unrelated earlier deck or notes warning, consult a global last-output registry or perform a second render.

The native companion entrypoint now explicitly changes main-slide requests to Calibri before testing Aptos-only notes and handout surfaces. Its source-only negative and positive assertions separate preview, notes and handout contributions, require one owned warning, repeat equality, actual package reopen and unchanged source. The installed control at `crates/rpptx-py/tests/test_documented_examples.py:6877` likewise clears the slide's Aptos request and requires main-slide diagnostic absence before demanding the note-specific fact. These discriminate the stale surface-vector defect. The timeline positive, repeated-output equality and notes-only non-leakage assertions discriminate the missing timeline projection. A direct merge control retains duplicate ordinary warning facts even when their text resembles a font fact.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Six aspects

- Correctness: 0 findings. Notes, handout and timeline now propagate the same operation's actual font facts. Suffix separation follows the inspected renderer invariant. First-use equality dedup is limited to accepted font facts.
- Contract: 0 findings. Output-owned companion payloads, original byte APIs and error behavior remain unchanged. Different selected-face facts are retained. No current-output registry or inferred second-render diagnostic was introduced.
- Panics: 0 findings. The new suffix split length is bounded by the same renderer's appended sink. Existing checked render errors precede output publication. Test-only unwraps retain fixture preconditions.
- OOXML: 0 findings. Main-slide font edits and notes/handout fixtures are actual qualified native owners. Repeated render, package XML and reopen assertions preserve source. The production changes do not edit the package or stored character codes.
- Tests: 0 findings. Notes-only and handout-only requests remove the former masking preview warning. Equal preview/surface facts are counted once. Covered main-slide timeline cannot inherit notes-only Aptos. Ordinary warning multiplicity is retained. Actual execution is still required.
- Structure: 0 findings. Existing concrete exporters and the current font sink remain the sole producers. The private helper performs actual ordered merge/dedup work for both current exporters, with no new type, trait, generic, module, file or dependency.

## Limits and handback

ROOT session61717 is running current nine-crate and six-native verification. This review does not infer its terminal result or any installed PASS. No Cargo, product imports, compiler, formatter, target, installation, environment, source or native UI action ran. Fresh installed companion/typing, licensed actual-painted-CID controls and all remaining full font, metric, symbol, packaging, hash, WASM, scoped and completion obligations remain separate. Only this canonical review file was written. Review ends and returns its lease to ROOT.
