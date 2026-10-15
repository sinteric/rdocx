# F-X195, SVG test deprecation, pass67

**Reviewed**: External R43 722-byte amendment to one existing test helper. ROOT read the source diff and pinned quick-xml0.41.0 implementation directly.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None. The deprecated decode_and_unescape_value and its explicit replacement decoded_and_normalized_value delegate to exactly the same internal function with XmlVersion::Implicit1_0, decoder, normalization1 and resolve_predefined_entity. The amendment preserves entity, whitespace, decoding and error semantics. No production, test assertion, font coordinate or independent oracle changes.

## Smells

None.

## Nitpicks

None.

## Not found

Correctness, contract, panics, OOXML, tests and structure have zero findings within this existing test-helper correction. There is no lint suppression. Receipt700a6ed5c46e4570ceeea6b97ed95d4f6bc49dbdef51b7cb30e1139bac788515 binds source9ce6da59 and diff e6eb8c544a078de661e3f606c62f13e8eebc03b4e15936fc4a5910c6f7268f91.

## Handback

Only this report was written. Source adoption and actual compiler and lint gates remain pending. The completed native R41 failure remains immutable. This review ends and returns control to orchestration.
