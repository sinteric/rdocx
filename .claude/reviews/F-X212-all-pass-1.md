# F-X212, all aspects, pass 1

**Reviewed**: Full story Base `66d06bf3e3f4287afb35dea8217e07f98efa7bfd`, mechanical extraction `ea803be1e59ba588d4c8da3f1231d84314cd847a`, and frozen working semantic changes. Four existing Rust files contain 1,953 added and 329 deleted lines. The retained complete diff contains 2,649 lines including the approved plan refinement.

**Verdict**: 2 defects, 0 smells, 0 nitpicks.

## Evidence and limits

All seven source guards in `.claude/scratch/F-X212-semantic-source-review/receipt.json` matched at review start and immediately before this record. The 120,321-byte complete diff matches SHA-256 `9f17655d35e843d72f07e0ec84ed330cb8eb9c449588ef4b4c8df740dd40008c`. The 101,270-byte behavior diff matches `df3e4468ec93548704e3ea58e407cd4bb070cfb8081fb041fd237fb45a401a70`.

This is a source review. No Cargo, native UI, formatting, source repair, baseline update or semantic execution occurred during the pass. Retained actual Base failures and successful mechanical extraction with unchanged 49 hashes remain distinct from the uncompiled semantic prefix. The invalid initial drawing fixture remains an invalid precondition, followed by the separately corrected genuine Base failure. Semantic controls, final hash attribution, complexity measurements and publication gates remain pending.

## Defects

### D1, Special-character-only field results are still classified as empty

`crates/rdocx-oxml/src/text.rs:10761`, `crates/rdocx-oxml/src/text.rs:3243`, `crates/rdocx-oxml/src/text.rs:4487`, `crates/rdocx/src/document.rs:1914`

The new retained-result eligibility predicate handles Tab, Break, Symbol and Drawing but ignores every SpecialCharacter. A complete simple PAGE field whose sole result is `<w:r><w:noBreakHyphen/></w:r>`, or a valid positional tab, parses its typed result and then discards it because this predicate returns false. Its string cache is empty. The equivalent complete complex field also discards its builder result graph. The visibility operation consequently has neither non-whitespace cache text nor result runs to traverse and reports false. These same accepted special characters outside a field explicitly report true. This violates the approved break/tab/special-character contract.

Carriage return needs an additional consistency check. `crates/rdocx-oxml/src/text.rs:2344` gives its typed display a newline, while `simple_result_run_display` at `crates/rdocx-oxml/src/text.rs:3340` does not collect `cr`. Therefore merely adding SpecialCharacter to eligibility still leaves simple carriage-return results excluded by the equality guard at line3248.

The parser matrix at `crates/rdocx-oxml/src/text.rs:17666` tests tab, break and symbol but misses these accepted variants. Add simple and complex cr, noBreakHyphen and valid ptab controls through parser, native visibility/audit and CLI, keeping softHyphen-only false and preserving source bytes and public cache consistency. The finding is established by the source branches, with no runtime result claimed.

### D2, Deleted-only simple-field caches bypass accepted-view exclusion

`crates/rdocx/src/document.rs:1919`, `crates/rdocx-oxml/src/text.rs:3340`, `crates/rdocx-oxml/src/text.rs:3243`

For `<w:fldSimple w:instr="PAGE"><w:r><w:delText>gone</w:delText></w:r></w:fldSimple>`, the existing simple cache collector explicitly includes delText and produces the string `gone`. The new retention gate does not retain this deleted-text-only typed graph. The new native visibility predicate then accepts the nonempty public cached_result before inspecting any accepted typed result. It reports visible, so CLI omits its empty advisory and three such paragraphs interrupt the native empty streak. The approved accepted-view rule excludes deleted text. This is an existing producer limitation newly consumed without qualification, rather than a claim that this diff introduced the cache collector itself.

The deleted control at `crates/rdocx-cli/tests/integration.rs:2050` only places delText directly in a paragraph, and the native field matrix at `crates/rdocx/src/document.rs:39120` tests ordinary field text and whitespace. Neither exposes this bypass. Add the deleted-only simple field, its nested placement in a complete outer field, and a positive accepted-text sibling. Preserve the existing public cache/source contract while deriving visibility from accepted display ownership.

## Smells

0.

## Nitpicks

0.

## Aspect results

| Aspect | Result |
|---|---|
| correctness | D1 and D2. No additional findings. |
| contract | D1 and D2 violate accepted displayed-content semantics. No additional findings. |
| panics | 0 findings in the reviewed changes. |
| ooxml | 0 independent namespace, child-order or opaque-source preservation findings. |
| tests | D1 and D2 identify missing distinguishing controls. No additional findings. Unexecuted semantic gates earn no pass credit. |
| structure | 0 findings. The concrete private modes have actual native and CLI consumers and no new module, dependency, trait or generic parameter. |

The review also checked structural paragraph-slot correlation, direct-body streak resets, current-source preparation and failure advisories, stale graph child access independent of display filtering, child source identity refusal, refresh/save/reparse, mixed annotation ownership, producer-prefix qualification and the bounded retained-size/clone accounting controls. No additional source finding was identified in those paths. Their runtime and output gates remain pending.
