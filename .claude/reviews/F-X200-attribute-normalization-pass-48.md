# F-X200, attribute normalization, pass48

**Reviewed**: Actual one-call paragraph.rs normalization correction against the approved F-X200 preservation contract and retained four-IO compile remediation.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

Receipt6d49d38e6d8100d8954f3dbd6b0bbebc475ba965b85deaa235bf1e462903fbd3 and correction63bf3cb7bf0577c6d47168c619498a14367c5c6df0a892f2279fe2dea6d36d83 authenticate. All2714 current byte, size and mode guards match source fdea60da85df0793e31902837945c66d1e8eed4e1de206a769a25e357a01d5e5. Only this carrier differs from bb9520. HEAD7c9b46a37a5c40ff3d7a21953dec06571b1e5964 and staged index34a791385771b53d4288429b4b573cdecb5274ec9c9d93db41a4ba86c6f95cc4 remain unchanged. Exact inverse restores the retained preimage.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Six aspects

- Correctness: zero findings. At crates/oxml-drawing/src/text/paragraph.rs:947, normalized_value(XmlVersion::Implicit1_0) replaces deprecated unescape_value. Installed pinned quick-xml0.41.0 primary source events/attributes.rs:84 and :299 routes both to normalized_value_with with identical XML version, depth1 and predefined entity resolver. Authority SHA256c46448f11d7dba312e6ad2177dc31deab2ce51282e9f1e4069d3b35175c18518 matches. Return and error types are identical.
- Contract: zero findings. Only the namespace declaration value interpretation changes spelling. Existing matching, ownership, refusal, raw retention and four IO conversions remain exact.
- Panics: zero findings. No new indexing, unwrap, allocation policy or untrusted-input failure branch.
- OOXML: zero findings. The same XML1.0 normalization and entity handling are retained. Neither source prefixes nor raw attributes are reserialized by this replacement.
- Tests: zero findings. All tests and deciding assertions are byte-exact. This source-equivalent lint correction does not qualify the pending EPUB semantic Base or GREEN.
- Structure: zero findings. One existing API call replacement, no new construct or dependency.

Only this report was written. No execution or whole-feature acceptance is claimed. Review ends and returns control before the separate F-X210 review.
