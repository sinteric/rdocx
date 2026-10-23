# F-287, Custom XML stores and data binding authoring

**Status**: completed
**Sprint**: S95.3
**Size**: L
**Depends on**: F-285

## Problem

Bound controls can already update a store that exists. `docs/hld/04-opc-and-packaging.md` says the document custom XML relationship resolves the item part, and that item's properties relationship resolves the `ds:itemID`. The evaluator accepts namespace-aware absolute child paths with optional one-based indexes. Functions, wildcards, descendant axes and general predicates are out of contract. `resolve_custom_xml_part` and `custom_xml_store_item_count` in `crates/rdocx/src/content_control.rs` implement that lookup. `CT_DataBinding` holds `prefix_mappings`, `xpath` and `store_item_id`.

Nothing public creates a store, item properties, schema references or a binding on a control. `crates/rdocx/src/bibliography.rs` writes one `ds:datastoreItem` for its own identity. That is the existing package sequence to copy from, not a second writer with different relationship rules. Unknown bytes in an existing item are already retained by the value updater. A new API must not become raw package surgery.

F-286 does not have to land first. The binding sits on `CT_SdtPr`, which F-285 also edits, so the drafted wave still runs this story after F-286. The dependency edge itself is only F-285. DOCX-054 is unsupported today.

## Spec reference

- `docs/hld/14-development-backlog.md`, F-287.
- `docs/hld/02-scope-and-non-goals.md`, DOCX-054.
- `docs/hld/04-opc-and-packaging.md`, the document-owned custom XML graph.
- `docs/hld/03-architecture.md`, data binding and staged publication.
- `docs/hld/10-bindings-spec.md`, bound value updates.
- `docs/hld/12-testing-strategy.md`, round-trip of bound custom XML and byte-identical unknown stores.

## Approach

Add store and binding methods on `Document`. Do not add a package-surgery API, a crate or a file.

Rust `Document::add_custom_xml_store(&mut self, item_xml: &[u8], schema_uris: &[&str]) -> Result<String>`. Python `Document.add_custom_xml_store(item_xml: bytes, schema_uris: Sequence[str] = ()) -> str`. The method stages a new item part, a properties part, both relationships and the content-type overrides through the same steps as `create_collection`. Part names use `reserve_preferred_part_name`. An occupied conventional name is not overwritten. The returned value is the new item id, not the part name. `create_collection` itself still returns the part name.

`item_xml` must be one well-formed XML element. Q23 explicitly refuses every DOCTYPE/DTD declaration before allocation, including declarations in the prolog. XML declarations, legal processing instructions and comments remain supported, and valid bytes retain their original representation. Empty bytes, a fragment, or bytes that are not XML return `Error::Other` and do not publish. `schema_uris` is a list of attribute values. An empty list writes `<ds:schemaRefs/>`, which is what bibliography writes. Each URI must be non-empty. Caller order is kept, including a repeated URI. The properties root is `ds:datastoreItem` with attribute `ds:itemID`, then child `ds:schemaRefs`, then one `ds:schemaRef` per URI. URIs are stored and not fetched. The id is the braced uppercase form `{00000000-0000-4000-8000-############}` from the existing allocator.

Rust `Document::set_content_control_binding(&mut self, location: &ContentLocation, store_item_id: &str, xpath: &str, prefix_mappings: Option<&str>) -> Result<()>`. Python `Document.set_content_control_binding(location, store_item_id: str, xpath: str, prefix_mappings: str | None = None) -> None`. The three strings are the fields of the existing `CT_DataBinding`. There is no new binding object. `None` and `""` mean no prefix mappings. The mapping text uses `parse_prefix_mappings`. The xpath uses `parse_xpath`. The store must resolve to one part through `resolve_custom_xml_part`. The selected element must already exist or be an empty element `select_xml_element` can expand. A failed check returns `Error::Other` and does not publish.

Value updates stay on `set_content_control_value_by_tag` and `bind_content_controls`. This story does not grow a second evaluator. After a public bind, those methods must update the new store and the display together, and a failed control must roll back both.

Unknown sibling bytes in the item stay byte-identical across a value update and across save and reopen. Schema references round-trip and are not resolved against a network or a second schema engine. Prefix mappings are stored and used by the existing path walker.

The relationship steps match bibliography, so one private helper in `rdocx` allocates the item part, the properties part, the relationships and the content types. The caller supplies the item bytes and the schema URIs. Bibliography keeps its own item bytes. The helper does not move into `oxml-*`.

Python uses the signatures above and keeps the frozen content-control snapshot from F-285. WASM and CLI gain none.

## Consumer map

| Rule | Entry points and outputs | Shared provider and caller policy | Lifecycle stages compared |
|---|---|---|---|
| Store identity follows the package graph | `add_custom_xml_store` | The relationship walk in `resolve_custom_xml_part`. No fixed filename | Create, reopen, resolve by id |
| Binding uses the existing path subset | `set_content_control_binding` and the value setters | `CT_DataBinding` and the current updater | Bind, fill, refused xpath, rollback |
| Unknown item bytes survive | Value update on a store with an unmodelled sibling | Byte compare of the unrelated span | Update, save, reopen |
| Display and store commit together | Existing tag fill on a newly bound control | One staged package | Success and one rejected sibling control |

## Behavior contracts and evidence

| Surface or placement | Exact behavior and boundary examples | Primary source identity and location | Local discriminator or pending evidence |
|---|---|---|---|
| Store creation | Rust `add_custom_xml_store(&mut self, item_xml: &[u8], schema_uris: &[&str]) -> Result<String>` and Python `add_custom_xml_store(item_xml: bytes, schema_uris: Sequence[str] = ()) -> str`. The return is the braced item id. Empty or ill-formed item bytes, and an empty URI, return `Error::Other` and do not publish. An empty URI list writes `<ds:schemaRefs/>`. | Q18. `create_collection` in `crates/rdocx/src/bibliography.rs` lines 1795 to 1846. That private function returns the part name. This public function returns the id. | A1. |
| Shared helper | The relationship steps match bibliography, so one private helper in `rdocx` allocates the item part, properties part, relationships and content types. Bibliography keeps its own item bytes. The helper does not move into `oxml-*`. | Q18, resolved by reading both call shapes. | C5. Bibliography bytes stay unchanged. |
| Binding | Rust `set_content_control_binding(&mut self, location, store_item_id, xpath, prefix_mappings: Option<&str>) -> Result<()>` and the same Python arguments, with `prefix_mappings` defaulting to `None`. Absolute child paths only. A leading `//`, an empty step, a function, a wildcard, a descendant axis or a predicate other than a one-based index refuses and publishes nothing. `/ns:root/ns:child` and `/ns:root/ns:child[1]` are accepted when the prefix is mapped. | `CT_DataBinding` at `crates/rdocx-oxml/src/content_control.rs` lines 87 to 90. `parse_xpath` at `crates/rdocx/src/content_control.rs` lines 910 to 990. `resolve_custom_xml_part` at lines 710 to 718. | A2 and A4. |
| Value update | Existing tag and bind setters update the store and the display together. A failed sibling rolls both back. Unknown sibling bytes stay byte-identical. An ambiguous store id and an other-namespace decoy refuse. | Q18. | A2, A3, A5 and A6. |

## Dependency readiness

All formal provider dependencies and shared-file leases were integrated before
this implementation. The applicable local gates and final provider riders now
pass with actual producer/source correspondence. S95.3 final release preparation
and its final sprint review remain separate, unfinished obligations.

## Acceptance criteria

| ID | Input or setup | Expected observation or refusal | Consumers | Local test | Final gate or pending evidence |
|---|---|---|---|---|---|
| A1 | Create a store with one element and one schema reference. Also pass empty item bytes, a fragment, and an empty URI | Reopen resolves the same item id. The schema reference is unchanged. A second create does not overwrite the first part. The three invalid inputs return `Error::Other` and publish nothing | Native `add_custom_xml_store(&[u8], &[&str]) -> Result<String>` and Python `add_custom_xml_store(bytes, Sequence[str]) -> str` | `custom_xml_store_round_trips_item_and_schema` | Hash harness unchanged for documents that do not call the API |
| A2 | Bind a rich-text control with `store_item_id`, xpath `/ns:root/ns:child` and a prefix-mapping string | Fill updates the element text and the control display. Reopen shows `w:dataBinding` with those three fields | `set_content_control_binding` and the existing fill | `custom_xml_binding_updates_the_addressed_store` | Round-trip suite |
| A3 | The same item with an unknown sibling and a comment in the unmodelled span | After fill, that span is byte-identical | Store writer | `custom_xml_unknown_bytes_stay_identical` | Round-trip suite |
| A4 | XPath with a function, a wildcard, a descendant axis, and a one-based index | The first three refuse and publish nothing. The index form updates that child | Binding setter | `custom_xml_xpath_stays_inside_the_existing_subset` | None beyond local refusal |
| A5 | Two controls, one path missing | The successful control does not remain half-updated. Document and item match the input | Existing staged fill | `custom_xml_bind_failure_rolls_back_display_and_store` | Round-trip suite |
| A6 | A store id that resolves to two items, and a same-local-name decoy in another namespace | The ambiguous id refuses. The decoy is not metadata | Existing resolver | `custom_xml_identity_matching_stays_namespace_aware` | Existing decoy test stays green |

## Test matrix

| Case | Modes or state combined | Expected difference or agreement | Wrong implementation rejected |
|---|---|---|---|
| C1 | No store, one store, two stores | Ids do not collide and names are not reused | `count + 1` naming that overwrites a hole |
| C2 | Empty selected element versus an element with text | Both end with the filled text. Siblings stay put | A rewriter that serialises the whole item from a tree |
| C3 | Implicit default namespace versus an explicit prefix | The same expanded name selects the same element | A string compare on the prefix |
| C4 | Bind, save, reopen, fill | The binding attributes survive reopen | A binding that exists only on the live object |
| C5 | Bibliography store left untouched | Its item id and bytes stay | A public helper that rewrites the bibliography part |

CLI and WASM stay unchanged. Layout does not read custom XML item bytes except through the control display, which A2 checks.

## Rejected alternatives

- Exposing `OpcPackage` mutation as the public API.
- A general XPath engine. The packaging spec already bounds the path language.
- A new custom-XML crate. The resolver and the bibliography writer are the existing owners.
- Starting this story in parallel with F-286. The file lease on `content_control.rs` is shared even though the dependency edge is only F-285.

## Test plan

**Test gate**: round-trip. Bound controls resolve, update and reopen against the correct store, and unknown store content stays byte-identical.

| Timing | Category | Test | Asserts |
|---|---|---|---|
| local scoped | round-trip | A1 through A6 in the existing `rdocx` integration entrypoint | Id, path, rollback and byte identity |
| local scoped | regression | Existing binding and decoy tests | The evaluator did not grow a wildcard or a function |
| final sprint | rider | full hash harness and installed suite | No delta on packages that do not gain a store |

No new test binary. No network schema fetch.

## HLD impact

- `docs/hld/02-scope-and-non-goals.md`, DOCX-054
- `docs/hld/03-architecture.md`
- `docs/hld/04-opc-and-packaging.md`
- `docs/hld/10-bindings-spec.md`
- `docs/hld/12-testing-strategy.md`
- `docs/hld/14-development-backlog.md`

## Risk routing

- Any parser or serialiser. Properties child order, and a byte-identical unknown span.
- Public API of a published crate. Additive store and binding methods. Final publish dry run.
- WASM or PyO3 bindings. Python is included. Final `rdocx-wasm` check. Python tests stay excluded from the workspace binary.
- Shared semantic behavior with multiple consumers. Bind, fill, display and reopen.
- Grammar, scope, compatibility mode or empty collection. C1 and the empty selected element.
- Crate boundary. No `oxml-*` dependency on `rdocx-*`. The helper, if extracted, stays in `rdocx`.

No external oracle. The test gate is round-trip.

## Hash harness

Unchanged.

## Implementation checklist

- [x] Wait for the F-285 prefix checkpoint and for F-286 to leave the shared files.
- [x] Add a RED test that a missing public create still cannot bind a new store.
- [x] Implement the store and binding methods on the existing resolver.
- [x] Update the HLD list and obtain a zero-finding microscope.
- [x] Accept the actual final source-qualified unchanged hash observation.

## Deviations

This section records the original implementation checkpoint. Accepted provider completion below supersedes earlier pending evidence and readiness statements. Historical failures and missing-input limits are retained.

The F-285 prefix checkpoint is integrated at `e7f3e98f` and the F-286 file lease ended at integration `9237b484`. The dependency-readiness sentence stays as written.

A missing `add_custom_xml_store` is not a semantic RED at compile time. `custom_xml_binding_updates_the_addressed_store` keeps the missing-store fill refusal beside the additive create checks. There is no separate RED commit.

The private helper is `stage_custom_xml_store` in `bibliography.rs`. `create_collection` calls it and still returns the part name. Empty schema properties bytes stay on that path.

## Contract closure

Reviewed against `.claude/reviews/S95.3-design-quality-pass-1.md` R2 on 2026-10-19.

| Finding | Corrected section | Acceptance and test |
|---|---|---|
| R2 `properties` versus `schema_uris` | Approach and the store-creation behavior row. The argument is `schema_uris` | A1 |
| R2 binding representation was unnamed | Approach and the binding behavior row. Three `CT_DataBinding` strings, not a new object | A2 and A4 |

No new product choice. Q18, Q19 and Q21 stay answered. The private helper fact stays the bibliography relationship match.

## Open questions

| ID | Affected F-IDs and A-IDs | Decision and recommendation | Status and answer/source |
|---|---|---|---|
| Q18 | F-287 A1 to A6 | `add_custom_xml_store` returns a new item id through the existing graph. Binding accepts only the current absolute child-path subset. Unknown bytes stay byte-identical. Bibliography shares a private `rdocx` helper because the relationship steps match. The helper does not live in `oxml-*`. | Answered 2026-10-18 by the user. The helper fact was confirmed from `bibliography.rs` lines 1795 to 1846. |
| Q19 | F-285, F-286 and F-287 | This story starts after F-286, which starts after the F-285 prefix checkpoint. | Answered 2026-10-18 by the user. |
| Q21 | F-285, and this story's approval | The F-285 method names. This plan has no separate product choice. | Answered 2026-10-18 on F-285. This plan adopts those names. |

No external issue closes from this plan.

## Codex takeover, approved Q23 and event coverage

On 2026-10-19 the user requested Codex take over the remaining work and close
when qualified. The user explicitly approved refusing DTD declarations. This
is a bounded API decision, not a claim that native Word evidence exists.

| Event | Before root | Inside root | After root | Proof |
|---|---|---|---|---|
| Start / Empty | Admit one root | Validate names and attributes | Refuse second root | Existing root/attribute cases |
| End | Parser rejects unmatched | Matched depth transition | Parser rejects unmatched | Reader end-name checks |
| Text | Literal XML whitespace only | Legal XML characters, no literal CDATA terminator | Literal XML whitespace only | Document-event test |
| CData / GeneralRef | Refuse | Existing decoded-reference and XML character rules | Refuse | Existing reference cases |
| Comment | Enable lexical checking | Enable lexical checking | Enable lexical checking | Comment refusal and retention |
| PI | Legal XML Name, target not case-insensitive xml | Same | Same | Document-event test |
| Decl | Only first event, validate XML 1.0 declaration grammar | Refuse | Refuse | Document-event test |
| DocType | Refuse by Q23 | Refuse | Refuse | Document-event test |
| Eof | Refuse missing root | Refuse unclosed root | Accept exactly one closed root | Existing root cases |

A7 extends admission coverage through
`custom_xml_store_document_events_are_validated_before_publication` and
`test_f287_custom_xml_document_events_are_owned_and_atomic`. Both compare
complete package bytes on refusal and valid store bytes after reopen.

The input is UTF-8, with optional UTF-8 BOM. A declaration may name UTF-8 or
US-ASCII when the whole input is ASCII. Unsupported or contradictory encoding
labels refuse before publication. This follows the actual decoder boundary
instead of treating every syntactically valid encoding label as supported.
Attribute and declaration fields require XML whitespace between quoted fields.

Q23: answered by the user in this session, "Refuse DTD declarations explicitly
(recommended)". No namespace-well-formedness requirement is added. B1 and final
integrated qualification stay pending. Historical reviews are retained.


## Accepted provider completion, 2026-10-22

The applicable unique local checks, independent clean microscopes and actual
provider final riders are accepted with their original producer attribution.
Native Word evidence and Q24 resolve the earlier missing-Word condition for
F-286. Source-qualified full suites pass6732native tests with73ignored,
1061installed Word and1028installed PowerPoint tests. Two added external Word
controls pass separately. Both WASM configurations, minimal layout, lint/docs,
README inventories and22verified publication dryruns pass. All49hashes and
seven deterministic pixels stay unchanged. All seven edit and three read release
ratios are below3. Qualified independent font inputs are real producer outputs.
The50-deck/421-slide corpus has zero dropped shapes. Its below-target SSIM is
an explicitly advisory trend, not a passed target or an automatic failure.

The authoritative retained logs and source/artifact correspondence are in
`.claude/scratch/S95.3-takeover-evidence/`. Only test_core.py adds the exact150
independently reviewed external-test lines to its earlier qualified bytes.
Other324manifest inputs match. No product/native source or baseline changes
in this completion. Historical failed attempts stay failed. Private unreduced
Issue390 remains unqualified under its approved bounded disposition. This
completion does not close a whole external issue or claim a new full-suite run.

At this historical provider completion checkpoint, the eight-story global
gate was not passed. F-X263 new-version packages, local/hosted artifacts and
final independent sprint review were still due. Q25
permits at most two further final review passes,4and5. Publication still follows
sprint acceptance, closure and separate exact-main-SHA final approval.

## Typed-context qualification checkpoint before sprint review pass 5

The existing B2 customXml and typed-context amendments now have clean focused
microscopes, clean35238e5b verified rdocx archive and actual updated hosted
39820083897qualifications. The21other verified archives retain unchanged
inputs and their082aafa9producer. Current native6733composed, installedWord1065
and applicable unchangedPPT1028retain actual artifact/source attribution.
The exact observations, counts, limits and failures are reconciled in
CURRENT_SPRINT's current integrated amendment acceptance and the retained
repeating-context-amendment full-union receipt. Earlier checkpoint paragraphs
remain historical. No duplicate delivery credit. Final sprint pass5 and close
remain due, with publication separately approved at exact clean main.

## Namespace amendment qualification, 2026-10-22

The existing B2 namespace amendment has a clean independent microscope and
actual current artifact qualification at source 01d800f9. Both affected crates
have genuine clean archive verification, with 20 unchanged verified archives
retaining their producer. Actual build-only run 39830343186 passes 27 deciding
jobs with four publication skips. Fresh downloaded wheels pass 172 priorities,
both typing/stub suites and six additional matrix calls. Current local Word
has 1,066 passes. Native acceptance composes 6,734 passes with 73 ignored,
including 4,801 freshly executed affected cases. Exact observations, source
correspondence, failures and limits are reconciled in CURRENT_SPRINT's namespace
amendment qualification and namespace-identity-amendment full-union receipt.
Earlier checkpoints retain their historical meaning. All eight original
deliveries remain unchanged. Final sprint pass 6 and closure are pending at
this checkpoint. Their actual outcomes follow in the numbered review and sprint
tag. Publication is separately approved at exact clean main.
