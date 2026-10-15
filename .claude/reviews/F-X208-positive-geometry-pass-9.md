# F-X208, positive geometry, pass 9

**Reviewed**: Immutable source35 one-carrier correction against reviewed source31, one predicate and 65 regression lines. Receipt SHA-256 96431fb53342fffda73ec1321d1abb286df8529c9d1b9eaa1aa496c9285d8ed2. Authenticated candidate, diff, full source map, execution receipt and both raw logs. Approved atomic footer geometry contract and actual renderer extent predicate are the authority.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None. Pass8 D1 is resolved at `crates/rpptx/src/lib.rs:15804`: effective cx and cy must each be strictly positive, matching renderer admission.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness: missing, zero and negative extents refuse before publication, supported positive geometry retains the existing success path. Contract: default footer, repeated bytes and reopen controls remain. Panics: new test-only unwraps operate on constructed controls, production adds no unchecked access. OOXML: imported zero extents on each axis are actual parsed packages, negative XML is rejected by the existing parser and typed negative authoring is separately exercised. Tests: the expanded geometry control and original footer control each actually pass with no ignores and byte-identical refusal assertions, owned processes terminate safely. Structure: one concrete existing predicate, no new file, type, generic, trait or suppression.

This closes the geometry remediation only. Earlier reviewed date-cache, diagnostic and HLD corrections are unchanged. Final wheel, installed typing, scoped riders and feature completion remain separate worker gates. No confirmation review or runtime replay was performed here.
