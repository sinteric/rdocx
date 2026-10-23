# F-X261, all, pass 1

**Reviewed**: uncommitted working tree against claim base `45510cec`, 19 files, 2182 insertions and 85 deletions
**Verdict**: 0 defects, 0 smells, 1 nitpick

## Defects

None.

## Smells

None.

## Nitpicks

- `crates/rdocx-oxml/src/text.rs:856`, a body text sequence that looks like `w14:` can add a root declaration. The function already says that match only adds a declaration, and the checked fixture still refuses an unbound prefix.

## Not found

Correctness produced nothing further. The section-profile wrapper binds `w14` before it parses a checked paragraph, and the same declaration is applied again when the main part is parsed. A text-box paragraph format edit publishes through the physical part, and the selected text-box owner scope retires, so the handle that made the edit fails on the next read. Header, footer, note and comment format edits stay on the direct property path. An asymmetric alternate refuses before publication. `part_xml` strips a UTF-8 BOM, refuses NUL and does not decode UTF-16. Plain sfnt faces are copied, and other accepted faces are deobfuscated.

Contract produced nothing. Q1, Q11, Q12, Q13, Q14, Q15, Q20 and Q22 are the public surface. PR395 and PR396 are not merged. Issue 390 stays open because the private unreduced control-owner document is unqualified. The final checklist item stays open for installed suites, deterministic font render, both WASM graphs and the hash harness.

Panics produced nothing. The `part_xml` lookup uses a part name just taken from the package map.

OOXML produced nothing. Style-root children stay ahead of `w:style`. The `w14` binding is a root attribute and does not reorder children. Paired text-box copies change together or not at all.

Tests produced nothing. The reduced rows name the observation they require, including both declaration orders, the unbound refusal, lexicographic part names, the three comment states, default-paragraph line rules, and one-copy versus two-copy text-box edits. A self-produced fixture is not treated as a Word render oracle.

Structure produced nothing. No new trait, generic, crate, module or file.
