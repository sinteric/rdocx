# F-X216, OPC record metadata query, pass 4

**Reviewed**: frozen relationship.rs114c1cb9674c903ecd34f3468650577b931bc68b33197f965b24135fe2cdd954 and immutable query manifest5fc775d32834425cd3823f9ee624de5e23542270b4f687110e027c487c68efa4 against separately reviewed predecessor42723.
**Verdict**: 0 defects, 0 smells, 0 nitpicks within the new record query and its dependencies.

## Assessment

record_metadata_equal_for_import at crates/oxml-opc/src/relationship.rs:504 compares qualified direct records selected by their actual unqualified decoded Id attributes. It first materializes checked preserving scopes without changing either input. Missing records, including a foreign nested ghost, return a named relationship error rather than matching by position or target value. Assigned ID, Type, Target and TargetMode equality belongs to the caller's documented graph admission and is deliberately excluded from this metadata comparison.

For records without metadata or content, the plain semantic fast path at :557 through :562 permits equivalent package namespace spellings. For opaque metadata or payload, :564 through :586 requires the full decoded root namespace map, actual expanded attribute identities, retained lexical attribute slices in order and exact inner payload bytes. Local namespace declarations remain metadata slices and are not stripped. This conservatively preserves unknown QName/prefix-valued authority instead of assuming name-only equivalence. Entity-equivalent root declarations compare by decoded URI while opaque record values remain lexical. Conservative false results for different opaque spellings do not authorize source loss or destination overwrite.

The query is nonmutating and calls the already reviewed BOM-aware and URI-normalizing parser and preserving rebind helpers. It adds no global serializer fix or transfer architecture. Prior42723 pass3 acceptance remains its own source boundary, not whole updated-carrier execution credit.

## Actual evidence

Authenticated manifest retains current query source, actual receipt91ec19ca5ecc88b16eddbab3c3b51b38c86744a6d4aa0d1356829b84c3187ac4 and two exact qualified safe rows. Strict three-crate Clippy exits0. The exact inline query control at crates/oxml-opc/src/relationship.rs:1434 executes1 passed with0 failures and0 ignores. Both raw log hashes match.

That control distinguishes decoded equivalent namespace binding from foreign authority, refuses a nested foreign ghost and permits plain records with different root prefixes and remapped ID/target. The later font namespace fixture setup failure and unrun table node in the composite receipt are not query failures and earn no acceptance. No whole native or F216 completion claim is made.

## Not found and boundary

Correctness, selected record identity, conservative opaque namespace context, lexical preservation, atomic nonmutation, exception boundaries, relevant regression and structural review found no additional bounded issue. Smells:0. Nitpicks:0. Only this canonical review file was written. Live and retained query source hashes matched the sealed manifest before and after review. Evolving import/lib/public/font/table carriers were not audited. No test, Cargo, probe, source fix, Git operation, CP or native application ran. Review ended and returned control.
