# F-X252, correctness, pass 1

**Reviewed**: Working diff against b8d064f304f09e8451198c31b3d91b62b6e23c80, six files, 1139 insertions and 10 deletions. Approved revised plan SHA256 fc46d27703d00857b4e0ec8be2fe844445ad68f2536d6d2efef174c526dac86f, in-progress rows, progress, canonical microscope and applicable HLD contract reviewed. Frozen manifest SHA256 58b0ea99b1a781218e5b41dfa607e564cc49f83551753917c98241afc851da52 and all six source/test/HLD/plan hashes authenticate, unchanged at closing checks.

**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Defects

### D1, retained Choice Requires bindings can be reinterpreted

`crates/rdocx/src/document.rs:7869`
`crates/rdocx/src/document.rs:7876`
`crates/rdocx/src/document.rs:8013`

The retained MC element island fingerprint records prefixes in colon-valued attributes, and bare prefix tokens only when the attribute itself has the MC namespace. mc:Choice instead has the unqualified Requires attribute. Its bare prefix list therefore contributes no resolved namespace meaning.

For example, retain the imported raw island `<mc:AlternateContent><mc:Choice Requires="x"><w:r><w:t>choice</w:t></w:r></mc:Choice><mc:Fallback><w:r><w:t>fallback</w:t></w:r></mc:Fallback></mc:AlternateContent>` at its original direct paragraph route. Source x names urn:s95:opaque-a. In a checked replacement, change an ordinary run scalar outside the island and rebind x on the paragraph to urn:s95:opaque-b, leaving the island bytes, occurrence and all element/attribute expanded names unchanged. No QName with a colon or qualified MC prefix-list attribute mentions x. The old and incoming island keys consequently compare equal, although the Requires application-namespace condition changed.

This violates the approved retained-island interpretation requirement. Part3 clause 7.6 gives Requires its unqualified prefix-list form, while clauses 9.3 and Annex A describe the namespace-based choice condition. Fingerprint that existing declaration's resolved meanings without introducing MC execution or a recursive unknown-Word policy. Add the paired unchanged-binding scalar positive and changed-binding refusal. This is a source-confirmed gap, not a claimed executed product reproduction.

### D2, retained full ancestry per frame makes opaque depth quadratic

`crates/rdocx/src/document.rs:7932`
`crates/rdocx/src/document.rs:8022`

Every visited Start element clones the complete parent path, appends one owned expanded-name entry, then retains that entire vector in its frame until the element closes. Deep foreign descendants below an already admitted typed owner are traversed rather than captured as direct exceptions. Existing lexical XML validation and Word admission scan these opaque branches iteratively. Their modeled nesting budgets do not bound arbitrary opaque element depth.

A namespace-complete paragraph containing an admitted w:customXml with 10000 nested x:n elements in a foreign namespace, plus a changed ordinary run, therefore retains roughly 50 million path entries. Each entry contains two owned Strings and an index, at least 56 bytes of inline storage on this 64-bit target, before the duplicated string allocations. A roughly hundred-KiB XML fragment can thus demand several GiB in this new admission scan. Arc namespace contexts do not prevent that path growth. The selected source's preexisting model resource check does not bound incoming opaque depth.

Keep traversal state proportional to actual input or apply an explicit bounded work budget before this allocation growth. This does not require interpreting or rejecting the nested foreign vocabulary as invalid Word. Add a bounded deep-opaque control that detects the actual path-storage/work regression while preserving ordinary delegated nested routes. No stress execution was performed by this reviewer.

## Smells

None beyond the concrete resource defect above.

## Nitpicks

None.

## Not found

The finite host grammar matches the independently authenticated 33 Word plus two math expanded names. It does not use the renderer's traversal list as schema authority. Existing checked parsing enforces known pPr order in addition to the new frontier check. Exact selected snapshot no-op remains before new admission. Owner lookup, relationship checks, identity checks, staging and candidate reopen remain in the original checked path.

Simple Ignorable grants resolve at the declaration element, accumulate URIs, preserve ancestor grants under blank/local lists and later prefix rebinding, and restore sibling context. Actual source ancestor extraction excludes the replaced paragraph's own declarations and does not qualify preceding sibling MC declarations as ancestors. Existing namespace normalization is reused exactly once. Wrong attribute namespaces and unknown Word children receive no foreign-namespace waiver.

Direct imported exceptions retain physical ordered routes, raw bytes and the recorded relevant meaning. Original exceptions cannot move into an admitted wrapper because their required old routes disappear. Indexed maps avoid repeated cross-island raw matching. Stable repeated islands remain individually routed. Nested typed content and existing drawing/math providers remain delegated, as required by the revised contract. Unsupported qualified MC attributes preserve their own raw values and token meanings at their route without freezing an entire SDT owner.

New native and installed test source covers the 30 independently authored finite pairs, grammar helper membership with explicit helper-only scope, copied/moved/reinterpreted exceptions, marker/math positives, typed SDT metadata around retained MC attributes, signed bytes/live owners, stale/foreign owner and late relationship failures. The two defects above are outside those existing controls. No new public API, OXML modification, module, file, trait, generic, feature or dependency was introduced.

## Authority and boundary

Root primary reauthentication SHA256 70c1a8cbf792824e961552a2c818e7d1d5a722d046e0296289e7b832059f5253 and independent input-pair SHA256 d14099c003e1235d676185a1b94b7bbbe16b0eab1eb510fc379aa5b8256393cb authenticate. Those are finite input/schema authority, not current product execution or new native Word evidence. Newly retained MC Requires declaration meaning is part of the existing approved interpretation requirement, not an expansion of new MC support.

No product test, build, cache, UI or runtime gate was executed by this reviewer. No source, HLD, plan, test, progress or private proof was modified. Only this report was written. Worker source-guarded checks and scoped completion remain separate and are not claimed complete. Return the sole worktree lease for a separate remediation phase.
