# Hanji, vertical page stories, correctness, pass 2

**Reviewed**: working implementation after keep-next remediation, same seven source/spec files and approved plan
**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D2, notes are not registered at selected vertical measures

`crates/rdocx-layout/src/engine.rs:2383`

The note registry registers only the raw section's horizontal content width.
The paginator looks notes up using its transposed selected body width. A
vertical paragraph with a footnote reference can therefore retain the reference
while losing the note's reserve and paint. Register every finite selected
vertical measure as well as the existing raw width used by endnote pages.
Keep the registry's existing deduplication and cloned numbering semantics.

## Smells

None.

## Nitpicks

None.

## Not found

D1's keep-next lookahead now uses selected table variants and rebreaks following
paragraphs before pricing them. The dedicated regression exercises a narrower
initial measure and a wider page measure. The source, atomic-item, overlap and
public-carrier checks from pass 1 remain applicable. A final zero-finding
review is still required after note registration and final validation.
