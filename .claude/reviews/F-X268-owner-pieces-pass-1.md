# F-X268, owner pieces, pass 1

**Reviewed**: Frozen working tree against `b1906e7cc8a8207c492c17877d35c758c2821079`, six source and test files, 1837 insertions and 85 deletions. Root Codex independently read the implementation and assertions. This is a bounded review of the implemented owner-piece prefix, not the complete F-X268 feature.

**Verdict**: 0 defects, 0 smells, 0 nitpicks within this boundary

## Defects

None found in the inspected prefix.

## Smells

None found in this bounded pass.

## Nitpicks

None found in this bounded pass.

## Not found

Correctness, namespace qualification, panic boundaries, cache admission, fallback, publication and structure produced no additional finding in the inspected implementation. By-value reuse requires equal context and equal block value, with fields, tab stops and bookmarks excluded through the complete typed owner walk. Related parts require exact bytes and current relationship checks. Duplicate physical parts and unsuccessful piece proofs retain the full inventory path. Source errors are not converted into an empty successful inventory.

The new native readers close inherited namespace scope. Existing full readers retain exact-slice XML. Body story fingerprints alone are excluded from the debug address comparison. Cell and text-box identities retain their fingerprints. The native companion now compares full cell identities and table geometry against the whole-reader projection.

The cache stores current block and part vectors with shared concrete records. A prepared candidate does not replace the live cache before revision publication. Qualified insertions and removals preserve neighboring pieces, while count or slot mismatches discard body reuse. Existing form and signing publications retain the authoritative full source path. No semantic equality implementation, public Python signature, package writer or hash baseline is changed by this prefix. The callback in `word_owner_facts` has current body-block and related-part scanner instantiations.

## Evidence

The green development extension is `c7ac74b34e3cf1c9b3ea6537bdd2dcfe7fad7dec8d3d0767375c287119c7106f`. The fresh build exited 0 in 12.80 seconds. Scoped Python-crate clippy exited 0 in 7.16 seconds. Four splice cases passed in 6.47 seconds, and the affected 52-case runtime batch passed in 305.52 seconds. The stricter six-case batch passed in 192.40 seconds. Run, cell and header edits at both 20 and 80 paragraphs each read one piece and perform zero binding whole-inventory rebuilds or whole-source reads.

The native source comparison passed in 0.86 seconds. The field-form exclusion companion passed separately. The isolated archived and current artifacts established identical legacy run retirement for cell and header edits. The incorrect test assumption was corrected to require that exact refusal, rather than broadening handle lifetime.

A compiled cache-off mutant reached functional and reopen assertions, then failed the two-piece bound with 31 reads at size 20 and 97 at size 80. Its pytest child exited 1 as expected. No missing helper or compilation failure was treated as that result. Root compared the restored source and green extension byte for byte with their saved copies. Temporary diagnostic and mutant switches are absent.

## Boundary and remaining work

This does not complete A3 or A4 across all named publishers and contexts. Remaining direct binding assignments and full `WordOwnerIndexes::prepare` queries belong to the planned publication work in mechanism 4. Package staging in mechanism 3 and mechanisms 5 through 8 remain unimplemented. Further provenance and composition controls, current-intent HLD updates, whole-feature microscope, release timing, memory and history measurements, Word, hash and integrated gates remain pending.

One piece read is a projection-phase observation. Typed metadata traversal, native index preparation, cloning, hashing and serialization are separate work. No constant total public-call cost, release parity, measured memory bound or completed sprint gate is claimed by this review.
