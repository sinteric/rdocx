# F-X216, native composition, pass 1

**Reviewed**: Frozen working composition against claim Base `5b0f63d95e4ea82a05a3082fe74a6d8e19d9f572`. The seal contains 19 changed source, test and plan carriers. Its complete patch has 20,232 lines, with 14,513 additions and 4,274 deletions. This pass examines the approved native composition and its public and CLI publication consumers. Previously reviewed helpers retain only their exact source-qualified evidence.

**Verdict**: 4 defects, 0 smells, 0 nitpicks.

## Defects

### D1, source notes master authority outside the child surface is discarded

`/private/tmp/rdocx-f-x216-codex/crates/rpptx/src/lib.rs:14715`

The source-format path transfers the source master's shape-tree child surface, notes text defaults, background and colour values. It then serializes the note and redirects it to the destination global notes master. There is no transfer or representability refusal for source notes-master root metadata, common-slide metadata, header/footer settings, or the source tree's own group and non-visual properties. The existing model explicitly retains those values at `crates/rpptx-oxml/src/notes_parts.rs:36` and `crates/rpptx-oxml/src/slide_parts.rs:46`. The new tree helper keeps the candidate's own group/non-visual properties while iterating only the master's children at `crates/rpptx-oxml/src/shape_tree.rs:3225`.

A source master carrying a foreign root attribute or common-slide opaque extension therefore loses that authority on successful import. A source `p:hf` with footer disabled also cannot preserve its appearance merely by copying the footer placeholder. The imported note's rendering uses the destination master's header/footer settings at `crates/rpptx/src/lib.rs:23835`, with enabling decisions at `crates/rpptx/src/lib.rs:23990`. A destination that enables the footer can display a source-disabled footer. The one-global-master constraint does not authorize this loss. These source values need a proven local representation or a named atomic refusal before publication.

### D2, inherited root namespaces can reinterpret retained local tree bytes

`/private/tmp/rdocx-f-x216-codex/crates/rpptx-oxml/src/shape_tree.rs:3238`

The tree merge adds every absent master root attribute, including namespace declarations, directly to the existing local tree root. It tests only same lexical-key conflicts. This repeats the namespace leak that the bounded property inheritance helpers were corrected to prevent.

For a valid local `p:spTree` with no default namespace and retained `<opaque/>`, and a source master tree with `xmlns="urn:source"`, the merge succeeds and adds that declaration to the local root. The local opaque element changes from no namespace to `urn:source`, despite unchanged raw bytes. Likewise, adding an absent prefix can change the interpretation of a local namespace-valued opaque attribute. Actual inherited typed groups being closed independently does not protect these existing local raw slots. The writer emits the merged attributes before those slots at `crates/rpptx-oxml/src/shape_tree.rs:2852`. Merge admission must preserve both physical contexts or refuse the conflict, rather than supply a new ambient binding to local opaque content.

### D3, alias attributes can become duplicate expanded root attributes

`/private/tmp/rdocx-f-x216-codex/crates/rpptx-oxml/src/shape_tree.rs:3230`

The same tree-root metadata merge uses lexical attribute names as identities. Consider a local tree declaring `y="urn:meta"` with `y:keep="A"`, and a master declaring `x="urn:meta"` with `x:keep="A"`. Each input independently has one valid expanded `{urn:meta}keep` attribute. The merge adds the master's declaration and `x:keep` because neither lexical key equals `y:keep`. The writer emits both attributes on one root at `crates/rpptx-oxml/src/shape_tree.rs:2852`, producing a namespace-invalid duplicate expanded attribute. Different values are also accepted by this branch.

Use actual expanded attribute identity and preserve the selected owner spelling or refuse incompatible metadata before returning the staged tree. The earlier shape-property helper's expanded-identity checks do not cover this separate composition consumer.

### D4, source raw boundary concatenation can duplicate schema-final extensions

`/private/tmp/rdocx-f-x216-codex/crates/rpptx-oxml/src/shape_tree.rs:3311`

The helper traverses every source raw boundary and appends any byte-distinct raw child to the candidate's final boundary. It does not distinguish the schema-final `p:extLst` slot from repeatable shape members or other opaque events.

A local notes tree ending in `<p:extLst><p:ext uri="local"/></p:extLst>` and a source master tree ending in `<p:extLst><p:ext uri="source"/></p:extLst>` are individually valid. Both survive the byte-equality test and are emitted consecutively by the final raw-boundary loop at `crates/rpptx-oxml/src/shape_tree.rs:2862`. The resulting tree has two schema-final extension-list owners. Appending all earlier raw slots to the end also removes their recorded source boundary relationship. Preserve the schema-final owner and required opaque ordering through an admitted merge, or refuse the unsupported combination atomically. Exact raw-byte deduplication is insufficient schema admission.

## Evidence and guards

The manifest SHA is `87a32df0a6de2b59625291111ad204be7264c0f3c3e5d3f8e039fd072a633b3f`. All 19 immutable current/Base copies match their manifest hashes. The patch SHA is `8bae8ba19e3ea19a11b82a54639355991039c1eb7423c18ff0218600db51847c`. The source-map SHA is `97b035ad949a68a33bcf549009b8cb658e6f4477ded73f2cfcf9433cd3bef1e6`. All 3,296 mapped live bodies, sizes and modes match. HEAD matches the seal.

The actual receipt `F-X216-start/actual-native-import-current-controls-5/receipt.json` matches SHA `c6b6a6420dcc2cb5c6dd242e89aba8447bb9f7f65e13e675d53692163453018f`. Both raw log hashes match. The log has exactly the sealed 25 distinct native control names, all passing with 0 failures and 0 ignored tests. The all-target check and native row have exit 0 and authenticated safe process cleanup. The retained binary matches SHA `f16b11ffef852277d930dadbee9af74fd4b6b25226ffe523a370cd824227154b`. This is the documented 8 MiB debug test environment, not new default-stack evidence. Those controls do not exercise the four composition witnesses above. No commands, tests or binaries were executed by this review.

The declared index guard hashes logical staged entries from `git ls-files --stage -z`. Its current SHA matches the seal at `931922cf4a7d7700479801087ca129adbea2d2b71ca46052bc1f04ac82858d07`. An initial comparison against raw index-file bytes used the wrong representation. That incidental raw-file SHA, `123ec7dada584ab35895cf0b83375d5ab64e7a8f2ac2a1647febd31ab4a37b92`, does not contradict the declared logical guard. The author clarified the representation and the reviewer authenticated the exact declared command. No reviewer Git mutation occurred.

## Smells

0 found in the examined composition.

## Nitpicks

0 found.

## Not found and acceptance limits

No additional finding was established in the examined staged source snapshot, relationship-scope resource transfer, conservative automatic script-font refusal, source theme matrix preparation, sparse line staging, prepared Python result allocation, or CLI path-aware serialization. The retained 25 native controls qualify their exact tested paths only. New notes composition defects above are separate from the prior bounded utility reviews.

Current Office save/reopen, deterministic two-deck comparison, fresh installed Python and types, broader applicable native/CLI gates and formal handoff remain outside this review's acceptance credit. This report does not claim full F-X216 completion. The review pass ends here with no remediation.
