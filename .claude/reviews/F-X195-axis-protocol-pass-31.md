# F-X195, axis fixture protocol admission, pass 31

**Reviewed**: One existing test carrier, two synthetic variable literal arrays.
The 11292-byte proposal is bound to patch SHA256
2f64e91a09ff2bb17ef06ec553e52283f3962611bc01425d30cb02053785ab07.
**Verdict**: 0 defects, 0 smells, 0 nitpicks in the corrected proposal.

## Independent protocol qualification

Read the approved full design, current feature state and progress, prior
passes29 and30, complete candidate diff, generator, manifest and actual
independent reader result. Primary authority is
[OpenType tuple formats](https://learn.microsoft.com/en-us/typography/opentype/spec/otvarcommonformats).
PRIVATE_POINT_NUMBERS selects a private packed point list. The corrected
0xA000 tuple has an explicit zero point count for all four real and four
phantom points. Its serialized19 bytes consist of that count, eight word
X deltas and an eight-zero Y run. Data interval29 and all offsets, padding
and checksums are adjusted consistently. No reserved flag or extra tuple
is introduced.

ROOT independently authenticated every manifest artifact and reconstructed
both inline arrays. Both independent static controls are byte unchanged.
All tables except gvar and head checksum adjustment are byte unchanged.
Every assertion line and the complete production prefix remain exact.
Original points, delta values, metrics, coordinates and CMap expectations
are retained. This is a test precondition correction, not product repair.

The direct pinned Skrifa0.42.1 reader actually compiled and ran successfully
against both R17 and R18 inputs. R17 reports all-points false and draws the
upright rectangle at selected coordinates. R18 reports all-points true and
draws the independently established shifted points590/190, with unchanged
advance600. Read-fonts variations.rs:947 and Skrifa glyf/deltas.rs:118
correspond to that observed interpretation. Complete output is retained at
/private/tmp/S91-F-X195-axis-fixture-protocol-r18/reader-actual-root-2.json.
The first diagnostic compiled but its child timed out during unbounded
iteration. The second independently inspected diagnostic bounds decoding
to the eight authored points. Original diagnostic source remains retained.
Neither execution writes product source or the shared Cargo target.

Prior pass29 missed the tuple point-list precondition. Its structural
checks and actual R17 failures remain historical evidence, but do not prove
a valid-input product defect. This pass supersedes that qualification.

## Defects

None in the corrected proposal.

## Smells

None.

## Nitpicks

None.

## Not found and limits

Correctness, contract, panics, tests and structure have zero findings in this
bounded correction. OOXML is unaffected. Actual corrected product causal
tests remain unrun. No licensed ital/slnt corpus parity, full installed104
paint qualification or complete feature acceptance follows from the direct
reader. Host-painter provenance remains separately unresolved under pass30.
The review writes only this report and ends before adoption or remediation.
