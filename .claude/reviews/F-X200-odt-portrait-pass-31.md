# F-X200, ODT portrait projection, pass31

**Reviewed**: Independent fonts-authored R20 one-carrier proposal, complete
3309-byte diff, actual R19 command31 failure and existing ODT section writer,
reader and diagnostic predicate. F200 current branch/status, approved plan,
backlog and progress were read. This is an incremental review.
**Verdict**: 0 defects, 0 smells, 0 nitpicks. Runtime is pending.

## Authentication and cause

Proposal receipt SHA is
6b704be484126659bca3854035fb504f40b48aa54e114b64a8f96e494d7e5e3c.
Complete diff SHA is
926324a100094d1a69db65b36453fc674e21defff592a6811ed05c460ac495b8.
Candidate SHA is
43ec99f22503a3cc6313dffa9f163dd33ce8b3df573a298f1bd2dab84d23d9fe.
Before identity matches current odt.rs. The actual terminal R19 result is
failed, first30 stages qualified and final ODT stage failed. It is not a
completed31-stage pass.

`crates/rdocx-oxml/src/document.rs:541` constructs default letter with explicit
Portrait. The serializer at:1707 emits orientation only for Landscape.
The reader at:1092 sets orientation only when that attribute exists.
The current ODT predicate at `crates/rdocx/src/odt.rs:2056` compares the cloned
section structurally with default letter. The unchanged source save/reopen
therefore produces a false final-section loss. The actual failure contains
that one additional leading diagnostic, not a changed numbering result.

## Defects

0 defects.

## Smells

0 smells.

## Nitpicks

0 nitpicks.

## Not found

Correctness and contract have no additional findings. Normalizing omitted
portrait only in the private cloned diagnostic projection preserves source
presence, serialization and rendering. Other section fields and opaque
payloads still participate in the same equality. Existing header/footer
reference handling remains unchanged. The original shared-definition
numbering regression, full diagnostic order and source checks are retained.

Tests have no additional findings. The new inline control proves the actual
default-letter serialization omission and reader representation before
checking ODT diagnostic equality. Landscape, changed margin and raw payload
remain deciding losses across save/reopen with original source bytes intact.
These controls are prepared and do not establish actual compilation or pass.

Panics, OOXML and structure have no additional findings. No production panic,
XML child-order change, parser presence normalization, new module, dependency
or public API is introduced. Test-only unwraps concern authored finite data.
Full F200 acceptance, installed factories, riders and completion remain due.

## Review boundary

Only this review file was written. No source, formatting, Cargo, runtime,
cache, mtime, baseline, UI or intake action occurred. This review ends here
and returns control for separate guarded adoption and focused execution.
