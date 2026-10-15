# F-X201, rich owner stub remediation, pass 3

**Reviewed**: Narrow resolution of rich-owner focused pass 2 D1. Immutable receipt `2d01a5cc10a9e83864dd342c33b7d8a10fb05878bfc73d5ead10be0c58d87619` and correction diff `7e30560b4b40a97c13f611cc9793d0943954736b4faacb999b296b2b59a489b1`. Exactly seven added stub lines in the two existing owner classes. Before, after, patch and runtime carrier hashes authenticated. No broad source review is claimed.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Pass 2 D1 resolved.

## Resolution and six aspects

- Correctness: zero findings. HeaderFooter.tables and add_table at crates/rdocx-py/python/rdocx/_rdocx.pyi:467-468 match runtime document.rs:1326 and :1354. FootnoteEndnote.tables, add_paragraph and add_table at stub:478-480 match runtime document.rs:1488, :1511 and :1563. These methods return tuple[Table, ...], Paragraph and Table as declared. Required rows/cols and the note text default empty string match the runtime signatures.
- Contract: zero findings. The ordinary existing Paragraph/Table types are reused, and no detached replacement owner or changed public call is introduced.
- Panics: zero findings. This correction contains only type declarations and changes no executable path.
- OOXML: zero findings. Runtime source and serializers are unchanged.
- Tests: zero findings in this narrow correction. Actual fresh-wheel strict typing and stubtest remain pending. This source comparison does not claim that either gate passed.
- Structure: zero findings. The patch only completes two existing classes, with no new helper, file, module, dependency or abstraction.

## Evidence boundary

Original stub SHA `722fbe93997890f0e4a37d82f872c9ca305829c2a773248725a4b990f5eba9a3`, corrected stub `fd606eb9db16feb377d5292a54c7d05a008e55368352f4e8e4512229e42cff83`, unchanged runtime document.rs `50afdd02702c2604233d9048c3dcdee7bf23136d74587f22a6d96cb3c0cbeec5`. The diff has only the two insertion spans. Citations refer to these immutable copies. Reviewer fonts_resume did not author the correction. No source edits, typing runs, product imports or native execution occurred in this pass.
