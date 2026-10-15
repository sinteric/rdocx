# F-X205, spacing fixture, pass 3

**Reviewed**: The immutable review2-fixture-capsule correction, one existing test carrier, three added lines and one removed line. Reviewer: fonts_resume, independent of this correction. This resolves only pass2 D1.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Evidence and scope

Manifest SHA-256: `bdf79fc13b3208f7f4bbd26123bb5cd64d323288dc4679a873ccbc9656fe5d91`. Exact before SHA-256: `1e7505cd92e59dcaf59a27345f7f8914957cca6faa61dc3532e15323f783757d`. Exact candidate SHA-256: `181d61f918855435b449e87b9fa592d78078afb6d6c87b83f000f4778164d6e6`. Both hashes authenticated from the complete copies.

At `crates/rpptx-py/tests/test_documented_examples.py:8943`, the corrected fixture retains the actual authored run and sets its existing public font size before capturing package bytes. This creates a genuine direct character-properties owner for the following spacing attribute injection. The checked injection count, seven lexical operands, expected EMU values, getter no-write assertion, authored spacing update and reopen assertion remain unchanged. No implementation or accepted source-property semantics change.

## Defects

None. Pass2 D1 is resolved in the reviewed candidate.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness: 0. Contract: 0. Panics: 0. OOXML preservation and schema: 0. Tests: 0. Structure: 0.

## Limits

This is a fixture setup review. The seven cases have not been executed against a fresh installed wheel. It adds no runtime, paint, typing or whole-feature acceptance credit. Earlier source corrections retain their prior review authority.
