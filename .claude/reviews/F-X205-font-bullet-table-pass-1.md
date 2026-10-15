# F-X205, font bullet and table slice, pass 1

**Reviewed**: Seven complete existing source6 carriers, 1033 additions and 6 deletions in the immutable capsule diff. This is an early bounded review. Fill, properties, export and fresh-template completion are excluded.
**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Authority

Immutable capsule `/private/tmp/rdocx-f-x205-codex/.claude/scratch/F-X205-start-1/production6-capsule`, manifest SHA256 `157309e299816884957ae3eaf46f98a7509d003f547b81cfa144348576335b10`, source map `17757435b0729ad99eb7cb38b60eeadd67b90674b68672d80e21dc1624ea526b`, complete diff `6bcb997a3b4629b51778e10da04dd0d794c209c1a9e2d824148a491e3ff16977`. All seven source hashes and sizes authenticated. HEAD `338754a70e5c4dffc78d84069b8d0294e9395f2e`.

Actual referenced check receipt `8716d47d139e09b7cfc26dde5086f43efbeef7efe66cf99a8ff7aaf49a2e4588`, original language/table receipt `0a0d6e37b37058d4a73c14acb15a07203b2ea1d3a1699e4cd5e2bb4274b7a445`, and corrected table receipt `4d863169c0cde0933896d52c32c1d9db4c31d23a25a450e8310c59dd49cb1c24` were read. All four row logs authenticate, each group is gone. Check and language rows pass. Original table fixture failure remains failed, and the genuine table-selection successor passes separately. No installed or visually meaningful renderer qualification is invented.

Citations below refer to complete files inside the immutable capsule, unless explicitly stated otherwise.

## Defects

### D1, imported universal-unit character spacing is reported as absent

`crates/rpptx-py/src/text.rs:2084` and `:2088`, `crates/oxml-drawing/src/text/paragraph.rs:929`.

The parser admits direct TextPointValue::UniversalMeasure, including an imported spc="1pt". The new spacing getter returns a Length only for Centipoints and sends every other case to None. A present direct source spacing therefore appears absent and inherited. The approved getter contract is Length or None, with None denoting absent direct source. Existing canonical native layout at `crates/rpptx-layout/src/context.rs:3144` already distinguishes and converts UniversalMeasure.

Convert admitted unit-bearing spacing to the checked existing EMU/Length convention, or explicitly refuse an unsupported direct value. Do not silently report its absence. Add an imported direct universal-unit read, unrelated edit and reopen control alongside the integer truncation cases. Current public Base cases only use authored integer127 and do not detect this path.

### D2, bullet-size documentation falsely equates None with follow-text

`crates/rpptx-py/src/text.rs:1153` and `:1158`.

The public getter documentation describes None as following text, but an explicit point-sized bullet also returns None. The final approved plan deliberately allows that narrower fractional-size getter while retaining imported point-size source, and explicitly forbids claiming that None proves follow-text for that raw kind. State the boundary truthfully in the public documentation. Do not erase or convert the point-sized source merely to match the fractional getter. Existing cloning preserves it on unrelated bullet edits, which should remain the behavior.

## Smells

Zero.

## Nitpicks

Zero.

## Not found

Correctness outside D1 and the D2 documentation mismatch: zero findings in the inspected font, bullet and table slice. Baseline setter validates the finite bounded interval through its range predicate, spacing performs signed division before checked narrowing, numbering start checks its schema range, and typeface updates retain existing metadata. Language changes target only the unqualified owned lang attribute and retain foreign attributes. The source preservation control covers raw foreign language and font metadata, repeated save, and clearing.

OOXML: zero additional findings. Font and bullet source updates retain other owned choices and existing serializers. Bullet colour uses the shared RGB extraction and existing colour transformation preservation. Table style checks a bounded braced GUID before native mutation. Table formatting clones and stages the candidate with unchanged owner proofs, preserving grid and cell lifetime rather than treating formatting as structure.

Panics: zero new unchecked production panic paths found. Contract: the unimplemented remainder of F205 and final paint/oracle obligations are outside this slice, not waived. Tests: genuine check/language/table evidence is retained as described, fresh wheel, strict typing and source-outline/native paint remain pending. Structure: zero speculative new traits, modules, dependencies or forwarding-only APIs. The concrete helper closures have multiple present consumers.

No source, index, runtime, environment, native UI or baseline changes were made. Only this external review artifact was written for the sole worker writer to copy. Subsequent disjoint implementation is excluded.
