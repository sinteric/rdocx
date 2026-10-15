# F-X206, cell-anchor correction, pass30

**Reviewed**: Independent R3 incremental correction after pass29. Nine complete
candidate identities and all2714 current input guards were authenticated.
The369-byte incremental diff removes two lines in one existing carrier.
**Verdict**: 1 defect,0 smells,0 nitpicks. Unapplied and unexecuted.

## Authentication and correction

Read canonical workflow/microscope, current approved F206 contract, feature
status/progress and preceding full nine-carrier pass29. F206 remains running
in work/f-x206-codex. Current source map53debc893276ef3db4baa380c971dc994ba0d835df8406f933335d75dd57c875
matches all2714 actual byte, size and mode guards. No worker changed.

R3 receipt SHAa3855c58c086f87eae9842d6b84fc036d7ff53bfaa14fdadc6859bcc2d920d88,
complete diff4acfa9be1d69b83eb055ad0a513b4656bc58e959fd7cd159923ed548a3e8fbc7
and incremental diffa6640c1ec71a8833886a24cdb6a072ac8304a8375a51f3435c569b75ee7fc412
were authenticated. Eight candidates remain byte-identical to R2. Independently
reconstructing the remaining context candidate from R2 by deleting exactly
the embedded patch-header line and duplicate test attribute reproduces R3.
The field-free Base, namespace model, bindings, stubs and tests are unchanged.
Pass29 D1 is resolved in source. The provisional fixture concern remains
refuted against actual master relationship order, with no fixture change.

## Defects

### D2, two Rust candidates overwrite the same retained check log

`/private/tmp/S91-F-X206-cell-anchor-production-r3-prepared/rustfmt-checks.json:12`
and `:68` assign the same src-table-rustfmt-check.log to oxml-drawing/table.rs
and rpptx-py/table.rs. The first row declares SHA418478be931dd46887805a830bbca3eb78be5352a9a0ab7b5a0ed6eadbd33b31
at line13, but the actual file hashes to the second row's
aa7864edcbf7acd761ff71ef861e834f39ed8b79c48d5a3b16b733fd3cce9cd7.
The second check has overwritten the first artifact. The retained receipt
therefore cannot authenticate all six advertised syntax observations.

Rerun external syntax checks with unique carrier-bound log paths and seal a
separate correction packet. Retain the failed R3 evidence unchanged. Do not
rewrite the old receipt or infer the missing check from another carrier.
Rustfmt syntax observations remain distinct from actual compilation, Base
RED, GREEN and runtime acceptance.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Not found

Correctness, contract, panics, OOXML and structure have no new source findings
in this two-line correction. Tests retain the preceding reviewed assertions,
with no new semantic finding. The evidence defect D2 is the sole new finding.
No compiler or product test ran in this review. Full scoped compilation,
actual field-free Base, native/installed anchor and preservation controls,
fresh wheels, typing, native no-repair, hash and HLD gates remain mandatory.

## Review boundary

Only this canonical review was written. No source, external proposal, Cargo,
cache, mtime, application, baseline or intake mutation occurred. The review
ends and returns control for a distinct remediation phase.
