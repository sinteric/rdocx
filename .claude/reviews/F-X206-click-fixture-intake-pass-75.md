# F-X206, actual click fixture preconditions, pass 75

**Verdict**: 3 defects in the click fixture setup, 0 smells, 0 nitpicks in this bounded review.

## Defects

1. crates/rpptx/tests/integration.rs:28017 inserts an https hyperlink using with_first_slide_children, whose line25885 adds internal relationships. The original save at line28020 fails on an unreachable package target before testing external retargeting. Build the original hyperlink with TargetMode External before opening the deciding fixture. Preserve the original URL and all payload/reopen/pruning assertions.
2. crates/rpptx/tests/integration.rs:28089 serializes an imported sound relationship before the required WAV parts are added at line28091. Debug save correctly refuses dangling and unreachable media. Assemble the complete original OPC members and content types before opening or saving the deciding fixture.
3. crates/rpptx/tests/integration.rs:28147 repeats that ordering error for the alias/opaque shared-sound fixture. Add its two original media parts before the first typed save, retaining exact opaque aliases, sound sharing and namespace payload assertions.

## Smells

None in this bounded setup review.

## Nitpicks

None.

## Not found

The actual R25 run passed the seven numbering donor controls. Its PowerPoint integration reported285 passes,16 failures and26 historical ignores. This review addresses only three precondition failures, not the other thirteen failures. Geometry contract reconciliation, foreign namespace input ownership, six lexical assertions, corpus projection and two package hashes remain unqualified. No production admission, baseline change or weakened save validation is justified. Only this review file was written during review. Return to separate exact fixture setup remediation and independent review before runtime.
