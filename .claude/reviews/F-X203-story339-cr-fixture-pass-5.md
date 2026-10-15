# F-X203 story339 carriage-return fixture, pass 5

**Reviewed**: Immutable single expected-string proposal. One existing regression carrier, one replacement line, one added logical newline. Unadopted and unrun.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Bound input

Receipt SHA `54b3e4ee0309d1cd25c036a42ee8c72fb93081a1e97add616ed976999ea47c3c` and all14 artifacts authenticated. Before SHA `fa339baec930b316b3bc9b61f593bc9405cfe3f683c6d76e55a6997db22453bf`, candidate SHA `46848d0e2c87f7098f21e2996e1f64e14ba07bc0cd1e0ef44082031b33125039`, diff SHA `924d8a203e83906b1431a7d5e8a7e78144ceb7eac0e147d1568b2a64e2a2c015`. Independent byte comparison proves exactly one expected-string replacement and no other changed byte.

## Qualification

`/private/tmp/rdocx-f-x203-codex/.claude/scratch/F-X203-start/immutable-story339-cr-fixture-5/after-regression_test.rs:400645` retains the original complete cached-field input, including tab, ordinary/page/column breaks, explicit w:cr and final B. The expected text at400647 now includes the newline contributed by that explicit carriage return. Both original ParagraphRef equality and story-item equality assertions remain at400676 and400677. No input, owner selection, count, source assertion or other expected character changes.

`source/crates/rdocx/src/paragraph.rs:2703` delegates ParagraphRef.text to the accepted paragraph provider. The complete paragraph.rs copy is byte-identical to claim HEAD dd7c627b. `source/crates/rdocx-oxml/src/text.rs:2344` maps CarriageReturn to newline in CT_R::content_text. The approved shared Paragraph projection contract at `source/.claude/plans/F-X203-design.md:604` requires agreement across native, CLI and Python text views, including field control spellings. The correction follows the explicit source operand and unchanged provider, rather than accommodating a new extraction policy.

## Actual evidence and limits

Original story339 Base receipt SHA `0b3b0707f134c26184faa8fe44532280012b2c54ae1a9840111997c6dff8b599` retains1PASS/4FAIL/0IGNORE. Current pre-correction receipt SHA `cadbf6b940060a063f8995f4ac612a1c993e5b9657973ac00bca0b9f1cbdfef1` retains4PASS/1FAIL/0IGNORE. The latter failed assertion specifically observes the extra required newline before B. These are distinct states, neither is a five-control GREEN. The current receipt's status remains running, despite terminal recorded child rows and safe cleanup. Check and selected-MC rows are actual0, but do not prove this successor fixture.

Both receipts record source-after equality and safe owned-group cleanup. All raw log hashes match declared records. No successor runtime, adoption, installed GREEN or whole-feature acceptance is credited. No source changes or runtime commands were performed by the reviewer.

## Findings

Defects: 0. Smells: 0. Nitpicks: 0.

## Not found

Correctness, contract, panics, OOXML, tests and structure: zero findings in this finite expected-string correction. All original input and equality predicates remain intact.
