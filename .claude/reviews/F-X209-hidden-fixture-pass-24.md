# F-X209, hidden fixture diagnostic correction, pass 24

**Reviewer**: fonts_resume, independent read-only review.
**Reviewed**: Immutable hidden-fixture-source70-proposal-r3. One existing test-carrier proposal. The sole r2-to-r3 difference is the rebound-owner expected error variant and exact diagnostic.
**Verdict**: 0 defects, 0 smells, 0 nitpicks. Pass23 D1 is resolved statically. The finite fixture proposal is admissible for successor execution.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Aspect results

Correctness: 0. Contract: 0. Panics: 0. OOXML: 0. Tests: 0. Structure: 0.

## Resolution and preservation

The rebound negative now expects InvalidValue with the exact existing p:nvPicPr schema-order diagnostic. This is the unchanged capture_non_visual_child path when a foreign cNvPr does not advance the boundary and the following qualified cNvPicPr appears at boundary0. The assertion remains a strict named refusal, not a generic is_err check. No parser behavior is changed.

Exact comparison of r2 and r3 candidates confirms that only this assertion variant/message changed. All QName-only substitutions, root expanded-namespace setup checks, numeric entities, missing/false/visible inputs, foreign attributes, foreign-before-valid source preservation, opaque entity bytes and repeated serialization assertions remain byte-identical to the reviewed r2 proposal. Production remains frozen source70.

## Authentication and limits

Receipt SHA256 158d13e5414da9f2eb7d7c9aa54c49053e56c0698e67d5e38f5635a739fed139. All three records authenticated by exact size and SHA256. The original preimage is byte-identical to source70 picture.rs, SHA256 a576b3c42d8d1f40b821d3eebf2dea2592ccc63a874320540bfe6362cd50ae78. Candidate SHA256 78a10f0915507e21998ce495acc6676960c6410a728fa6b2d1b1d107d6a971f8.

This is unadopted, unrun fixture admission only. Original source70 reader failure and resolver pass, as well as r2's incorrect proposed error, remain distinct retained evidence. No successor native, installed, Office, whole-feature or publication credit is granted. No source or runtime changes were made. This review ends at this report.
