# F-X211b, native DiagramML text, pass 2

**Reviewed**: Frozen two-carrier remediation relative to native DiagramML text pass1, 116 additions and 0 deletions. Production adds five predicate lines. The existing integration entrypoint adds two deciding controls. Other clean native/public carriers are unchanged.

**Verdict**: 0 defects, 0 smells, 0 nitpicks within the bounded repair. No whole-child acceptance.

## Contract and authentication

Read the canonical microscope/workflow, approved parent/b contracts and pass1's concrete consumer defect. Capsule SHA `4b0538406c2d7cc4a8cd84f0448638b52ba448e15cf983181a04c95d7eac9616`, native14 manifest SHA `dedc0fdb1d195aaa9b5c4a95cfda6afcb72307dea647e4196e65bca16af3a0ea` and exact delta SHA `b966d15e19adb4805dcade2414608d1b487493c40b451722513a1f78a9718f6b` authenticated. Comparing with the preceding native14 manifest proved only `oxml-drawing/src/text/mod.rs` and `rpptx/tests/integration.rs` changed. All current tracked source bytes authenticate to the runtime source map SHA `e8ef9639c90f6d527ec4cca15c449aa164cb1b6fcd0b1dbfcbffecc6f72aac71`.

HEAD remains `0a4f9259c6d98f281075244cdc6b7b39da8edd83`. Logical index SHA `d1c274930f3047dc8ae8da4d21c9512eda688892ebb8b97a5c7624fa7f9034ef` and physical index SHA `84066f11157ba92a0518514e82ddff73d476e50a0a57eed259d2085a3e4e4c09` authenticated unchanged.

## Repair audit

`crates/oxml-drawing/src/text/mod.rs:130` permits official DiagramML only with caller-selected `t`, and official chart only with `txPr`. It does not admit an arbitrary foreign URI or chart `rich`. Existing A/P root handling is unchanged. Namespace lookup still uses actual local declarations combined with inherited bindings. The subsequent body parser still resolves its modeled children through the actual scope. A foreign-bound bodyPr cannot supply the required DrawingML body properties.

This matches the concrete DiagramML caller at `crates/rpptx-oxml/src/diagram.rs:831`. Its prior validator at `diagram.rs:1690` continues to require official DGM t and actual A descendants. It also restores locally declared C txPr for the existing axis/chart-space consumers at `crates/oxml-chart/src/lib.rs:7027` and `crates/oxml-chart/src/lib.rs:10488`. No caller, serializer, opaque retention rule or relationship provider changes in this repair.

`crates/rpptx/tests/integration.rs:31709` tests both concrete tuples with alias/default roots, actual inherited scope, complete text and serialize/reparse. It exercises real CT_DiagramData and CT_ChartSpace consumers, including a locally declared chart wrapper. `crates/rpptx/tests/integration.rs:31788` rejects foreign roots, swapped DGM/C local names, unsupported txBody/rich combinations and foreign-bound required children. The tests distinguish the pre-repair root rejection without weakening the original shared consumer contract.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Not found

Correctness, contract and OOXML: no findings in the finite admission repair. Panics: no new untrusted-input indexing, unwrap or arithmetic is added to production. Test unwraps operate on explicit positive fixtures. Tests: two deciding controls qualify actual supported wrappers and refused alternatives, rather than accepting any namespace with a matching local name. Structure: no new type, trait, generic, module, dependency or forwarding wrapper. Other native/public reviewed behavior is reused, not confirmation-reviewed.

## Actual evidence and limits

The coherent successor receipt SHA `8a4e07269e8ed0b732bc3d04e5d2a913d9c9bad0f285dfc0da19a58b923af866` authenticates actual exit 0, 24 passed, 0 failed, 0 ignored and 350 filtered, ordinary release stack defaults, safely reaped leader and gone process group. Its source and log hashes authenticated. This includes the existing22 owner controls and both new tuple controls.

The focused original receipt SHA `cb32db3fa5b6270eca4591556b17c83895da57105e85a1bd967f7220fa71eec7` remains actual exit 101, with log SHA `edeebf4118191b5f9e3f509b97da3a1e6fc967fc8cd35130ca3c7b6bc55c4972`. The original broad library failure also remains retained. Neither is relabeled as passing. Current strict changed-four-crate Clippy receipt SHA `df96564e810864cbc0228cc62efec433847690ffa8399271a8bb5476a5f6b3e9` and fmt receipt SHA `fcb321266b0102c9d62c3b00b784ac8dd47c57b487447b15be9ff3ae00109905` authenticate actual safe exit 0.

The separate root broad three-crate successor, including the exact original authoritative-empty DiagramML test, was still pending when this bounded review began. The24-control successor does not substitute for that full obligation. Fresh current public/native-backed wheel and remaining policy/archive/Office/parent acceptance are separate gates. No test, source, target, CP or Office operation was performed by this reviewer.
