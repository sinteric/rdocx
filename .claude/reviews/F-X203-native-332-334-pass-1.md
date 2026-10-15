# F-X203, native332/334, pass 1

**Reviewed**: Independent partial working review of the immutable constructor and Markdown reuse slice. Seven existing carriers,786 additions and88 deletions. This excludes subsequent Python operations, other F203 APIs, HLD completion and full feature acceptance.

**Verdict**: 3 defects, 0 smells, 0 nitpicks.

## Source identity

Input: `/private/tmp/rdocx-f-x203-codex/.claude/scratch/F-X203-start/immutable-332-334-review-1`.
Receipt SHA `3870d1af9e4c360f0156530a4f428dc145eb1c86e29c64563471f19f60e34aeb` authenticates all41 copied artifacts. Full3111-record map SHA is `d6c222363acb09b16f0831a9a56fcb49bdbdf03af171ef157636087b0168a791`. Complete diff SHA is `23f80c8d7c40b4e094ad69bea91822558f8a5a1e486f8eba5295e0bc66e9fee5`. Recorded HEAD is `dd7c627b534301c253615d30e35e8748b455a4d0`, index SHA `c203016ed951117303df8410bfd0993697878991ea37fce9a229fc1b0770f17d`.

All citations below refer to these immutable `source/` copies, not advancing live worker files. The approved Issues332/334 amendment requires literal whitespace retention and prohibits coalescing across accepted structural owners and hard/page/column breaks. Assessed PR354 is reused with that stronger accepted contract. Its donor test/archive claims provide no local acceptance credit.

## Defects

### D1, accepted structural owner boundaries disappear before coalescing

`source/crates/rdocx-oxml/src/text.rs:6270`
`source/crates/rdocx-oxml/src/text.rs:6291`
`source/crates/rdocx-html/src/markdown.rs:289`
`source/crates/rdocx-html/src/markdown.rs:358`

The field-preserving accepted projection still appends accepted content-control and insertion/moveTo runs directly to the ordinary run vector, without retaining their entering/leaving boundaries. Markdown then collects all runs in the same hyperlink context and merges consecutive equal-mark pieces. For bold literal `left`, a bold accepted inline control or insertion containing `middle`, and bold literal `right`, the code emits one `**leftmiddleright**` span across the physical owner. The approved emitter amendment requires coalescing to stop at accepted-view structural boundaries. Preserve those boundaries only in the explicit Markdown projection, while leaving default accepted-view and HTML semantics unchanged. Add deciding control/revision cases with equal emphasis on both sides and source equality.

### D2, cached field hard breaks bypass the break boundary

`source/crates/rdocx-html/src/markdown.rs:414`
`source/crates/rdocx-html/src/markdown.rs:424`
`source/crates/rdocx-oxml/src/text.rs:1198`

Direct run line/column breaks flush the current marked text and insert empty-piece boundary sentinels. Cached field display handles newline and column-break characters by appending `LINE_BREAK` inside the marked display string instead. The resulting piece merges with adjacent equal-format cached segments. A bold field cache with `one`, a line or column break, then `two` can therefore retain one emphasis span across the break, rather than separately owned spans around the two-space hard break. The existing cached-display provider explicitly exposes these physical break characters, so this is not an absent producer fact. Use the same explicit boundary behavior for cached line/column breaks and test simple and complex cached fields with equal marks before/after, preserving tabs, source XML and page-break behavior.

### D3, an ordinary whitespace-only paragraph loses its literal text

`source/crates/rdocx-html/src/markdown.rs:147`

After collecting original text, the early empty-paragraph branch applies `text.trim().is_empty()` and emits only one newline. An ordinary paragraph with `xml:space="preserve"` text containing spaces or tabs consequently loses those characters. The amendment requires whitespace-only segments to remain undecorated and original literal whitespace to survive. Distinguish a truly text-empty paragraph from a paragraph containing literal whitespace, retain that text outside emphasis, and add a deciding whitespace-only paragraph control. No approved exception authorizes retaining this previous omission.

## Smells

None.

## Nitpicks

None.

## Six aspects

- **Correctness**: D1 through D3. Equal-mark ordinary run coalescing, URL sanitation, empty/nonempty table list state and explicit page-break placement otherwise show no additional finding.
- **Contract**: D1 through D3 violate the amended Markdown preservation/boundary contract. The constructor confines mode15 initialization to fresh WordCompatible profiles across the existing package classes. Minimal creation, imported settings and explicit removal/override remain distinct. No constructor defect found.
- **Panics**: Zero additional findings. New string slicing uses UTF-8 trim boundaries, inline span indices come from local constructed vectors, and fresh settings initialization uses fixed valid values rather than untrusted input.
- **OOXML**: Zero additional findings. Export remains read-only. Fresh typed settings establish one qualified office-URI mode15 before initial serialization. Default `accepted_view()` delegates to the false projection, with equivalent recursive selection and flattening behavior. HTML explicitly selects that unchanged mode. D1 concerns lost export boundaries, not source XML mutation.
- **Tests**: Existing genuine Base and current native results are meaningful for their selected cases. They do not cover the three deciding inputs above. Required new controls are attached to D1 through D3, rather than treating existing passes as evidence for absent cases. No additional test finding.
- **Structure**: Zero findings. Existing carriers and consumers are reused without a new module, dependency, trait or forwarding framework. The explicit projection has a named Markdown consumer and keeps default consumers isolated.

## Runtime qualification and exclusions

Copied actual-native-332-334-2 receipt SHA `beb343d651f2889b5710f243a6b7d434a70ea1917e418d7b38b3884d2851e891` has native rdocx/rdocx-html all-target check0, HTML14 passes and five native332/334 controls passing under the recorded full map. Source-after matches and all owned groups are naturally reaped with ESRCH and no signals. These results do not establish D1 through D3.

Original native Base failures and the first HTML failed attempt remain retained. The installed missing-operation Base and corrected equation setup concern other F203 surfaces and receive no current implementation credit in this partial review. No current installed wheel, native Word opening, modern-versus-legacy layout oracle, output-hash reconciliation, archive/HLD completion or full feature result is claimed. Each intentional constructor and Markdown behavior still requires its separate labelled history and actual declared output qualification.

No source edits or runtime commands were performed. This bounded review ends with this report.
