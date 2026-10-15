# F-X192, IEEE namespace assertion, pass 27

**Reviewed**: Independent review of one external test-only carrier delta. Receipt6b4ed620a3c3465321c037a1ace129cf012b6b043e13519985f7694b4c763d5c, patch e2f9eecacb934f5ed040b441c5d8e0961471c0946d08102209d184acd426e1d1 and candidate a96bf359c6992790e96d1685cda50a8fc869052b122c9d87ea330986dc6ed110. Candidate is45107744 bytes and unapplied.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Not found

Correctness, contract, panics, OOXML, tests and structure each have zero findings in this incremental assertion correction. Read the approved full F-X192 design, current progress and independent namespace-save pass26. F-X192 remains in progress on work/f-x192-codex. All2702 actual source records match current c15dc6a545e8ecf079ecf87313ebdd47d32c3f72f2e84d29bdda9b1509acac2a bytes, sizes and modes.

The complete candidate reproduces the exact proposed patch against the current existing regression carrier. All13700 raw string literal operands across that carrier remain byte-identical and ordered identically. Every byte outside f192_ieee_own1033_full17_source_order_table_rich_and_reopen remains unchanged. No production, native expected XML, rich result, grid, paragraph property or failure-marker change occurs.

Candidate crates/rdocx/tests/regression_test.rs:399073 qualifies the independently retained input document root as the ns0 Word alias, with no xmlns:w root declaration. Moving this lexical check to the immutable original input retains the genuine alias-sensitive setup. It does not promise that a dirty modeled save keeps the same lexical prefix.

At candidate :399129 the saved document must resolve its first element to the expanded Word document name. Every saved element and attribute still rejects unresolved prefixes. The added field-scoped run counter verifies exactly seventeen simple citation owners, each with more than one qualified Word run, and closes every active owner. This is stronger than the previous global lexical w:r count, which could count unrelated runs and assumed a specific serializer prefix. Existing prior controls already require seventeen exact sources, public CRUD, seventeen exact numeric citation strings, unchanged source bytes, repeat-update stability and zero precondition diagnostics. No arbitrary-prefix acceptance bypass is added.

The subsequent inserted bibliography owner and original recognized unfinished-grammar refusal remain unchanged. Native seventeen-entry table/grid, rich run, first/interior/end context, all physical instructions and repeat/reopen checks remain exact. The new assertion permits a canonical modeled save prefix while preserving expanded ownership and every independent expected value. Fixture preconditions and parser errors remain distinct from genuine grammar Base failures.

No compiler, runtime, product import, source, environment, cache or namespace packet mutation occurred. Only this review was written. Actual current units and the complete original Base queue remain required. This source-only assertion review supplies no semantic RED, formatter acceptance, catalogue completion or comparator evidence.
