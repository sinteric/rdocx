# F-290, Mail-merge package and data-source authoring

**Status**: completed
**Sprint**: S95.4
**Size**: M
**Depends on**: F-233, F-270, F-278
**Base**: a57be365 on sprint/s95.4
**Design date**: 2026-10-23

## Execution boundary, 2026-10-27

The API, preservation, refusal and acceptance contracts below remain binding. Problem descriptions, approval-time unrun statements and local provider-prefix checkpoints retain their historical source and scope. They do not describe the final integrated execution state. At this checkpoint, the native union and affected corrective cases, installed Python suites and typing, native Word and PowerPoint qualification with explicit source applicability, A7 measured memory and retained-history review, A9, the original Issue407 gate, hash49, lint, WASM, rustdoc, supply chain and policy observations are available. Actual producer identities remain attached to each observation. No old binary is relabelled as a new build.

The remaining integrated acceptance debt at this checkpoint is Cell.add_picture's required flat-cost gate, the active README example batch, the clean22-package publication dry run and the final full-union record. The original failed attempts remain evidence. Status stays approved until the required final acceptance passes. When this plan becomes completed, its final source-bound delivery entry in docs/sprints/AS_BUILT.md supersedes these historical execution-status statements. Completion changes neither the contract nor any threshold. Build-only hosted wheel qualification belongs to release preparation before final release approval. The subsequent F-X270 intake prepares fresh0.16.4/0.14.4 patches within this sprint because the old0.16.3/0.14.3 candidates were already published. It is not an additional acceptance criterion in these eight sprint designs. Publication remains separately approved at the exact reviewed main SHA.

## Problem

`MailMerge` exposes 13 settings members (`crates/rdocx-oxml/src/settings.rs:365`) and does not carry the data source, header source, or ODSO graph. `Document::set_mail_merge_settings` (`crates/rdocx/src/document.rs:33307`) writes those settings only. Rich and flat execution (`crates/rdocx/src/field.rs:2975` and `3020`) consume caller-supplied records and do not retrieve a package source. There is no public package authoring or read API for mappings, recipients, and an embedded or external source, and no offline diagnostic when retrieval is refused.

Primary documents inspected by root on 2026-10-22, not a Word run: Open XML SDK 3.0.1 data source reference (ISO/IEC 29500-1 17.14.9), `CT_Odso` (17.14.25), field map data (17.14.15), and MS-OI29500 3.1.1.3 recipient data part (`https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oi29500/af3dc913-8c08-4843-ab40-495a92170b96`). `UniqueTag` is SDK 3.0.1 (`https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.uniquetag?view=openxml-3.0.1`). Native Word validation of an rdocx custom XML table is not claimed. MS-OI29500 says the recipient relationship URI ends in `/recipientData`. A similarly spelled `/mailMergeRecipientData` URI is a different string. At most one recipient part exists, and the settings part owns that relationship. An external mail-merge source relationship is owned by the settings part, not by the main document.

Dependencies F-233, F-270, and F-278 are done. F-290 local implementation and review 3 are recorded. Delivery stays in progress. Installed Word is 16.114 / 16.114.26101123. No new merge-package comparison has run.

## Spec reference

- `docs/hld/14-development-backlog.md`, "F-290, Mail-merge package and data-source authoring" (lines 3148-3154). Test gate copied below.
- `docs/hld/03-architecture.md`, "What stays put".
- `docs/hld/04-opc-and-packaging.md`, "The package" and "Relationship types".
- `docs/hld/10-bindings-spec.md`, "Native Word facade stability".
- `docs/hld/12-testing-strategy.md`, "Test taxonomy". This story's backlog gate is round-trip, not a native oracle for custom XML interoperability.
- ISO/IEC 29500-1 17.14.9, 17.14.15, and 17.14.25, plus MS-OI29500 3.1.1.3, as named above.
- Existing `MAIL_MERGE_ORDER` in `settings.rs` orders `CT_MailMerge` children, including the `odso` member. The native writer uses that same table, now `#[doc(hidden)] pub const`, plus `ODSO_ORDER` and `RECIPIENT_DATA_ORDER` of the same `&[&[u8]]` type in `settings.rs`. There is still one `CT_MailMerge` order table.

## Approach

No new mail module, trait, generic, feature, or test binary. Package authoring and diagnostics live in existing `field.rs` and `settings.rs`, plus the existing OPC relationship constants file when a constant is missing. Native merge execution stays `mail_merge` on prepared records.

```rust
pub enum MailMergePackageSource {
    Embedded { columns: Vec<String>, records: Vec<Vec<String>> },
    External { uri: String },
}
pub struct MailMergeFieldMapping {
    pub column: u32,
    pub source_name: String,
    pub mapped_name: String,
    pub language: Option<String>,
    pub dynamic_address: Option<bool>,
}
pub struct MailMergeRecipient {
    pub column: u32,
    pub unique_tag: Vec<u8>,
    pub active: bool,
}
pub struct MailMergePackageOptions {
    pub settings: MailMerge,
    pub source: MailMergePackageSource,
    pub mappings: Vec<MailMergeFieldMapping>,
    pub recipients: Vec<MailMergeRecipient>,
}
pub enum MailMergeSourcePolicy { Offline, CallerSupplied }
pub enum MailMergePackageDiagnosticKind {
    ExternalSourceBlocked,
    ExternalSourceUnavailable,
    UnsupportedSource,
}
pub struct MailMergePackageDiagnostic {
    pub kind: MailMergePackageDiagnosticKind,
    pub message: String,
}
pub struct MailMergePreparedData {
    pub records: Vec<std::collections::BTreeMap<String, String>>,
    pub diagnostics: Vec<MailMergePackageDiagnostic>,
}
impl Document {
    pub fn set_mail_merge_package(&mut self, options: MailMergePackageOptions) -> Result<()>;
    pub fn mail_merge_package(&self) -> Result<Option<MailMergePackageOptions>>;
    pub fn remove_mail_merge_package(&mut self) -> Result<bool>;
    pub fn prepare_mail_merge_data(
        &self,
        policy: MailMergeSourcePolicy,
        external_records: Option<&[std::collections::BTreeMap<String, String>]>,
    ) -> Result<MailMergePreparedData>;
}
```

Do not add `MailMergePackageInfo`. The same `MailMergePackageOptions` value is the owned snapshot for input and for a successful read.

`unique_tag` is opaque caller-native identity bytes, stored base64 in the package. It is not a documented application-independent hash. This story does not promise runtime recipient filtering or ODSO mapping execution based on converting those bytes. `prepare_mail_merge_data` returns faithful column-keyed records for an embedded table or for caller-supplied rows. When native mapping or native recipient selection cannot be applied faithfully, it returns no records and an `UnsupportedSource` diagnostic. It does not return a partial record list. Package metadata still round-trips. The existing merge caller may pass its own prepared records into `mail_merge`.

Q7 selected answer, user answer on 2026-10-23: an offline rdocx table plus Word metadata. Store the embedded table in versioned custom XML, schema `urn:rdocx:mail-merge-data:v1`, and write standards-compliant external Word connector metadata. There is no automatic retrieval and no file, HTTP, or SQL execution. Callers supply external records. `Offline` plus `Some(external records)` is `Err`. Do not pretend a Word-native embedded source. ODSO mapping metadata is not the embedded table. Word is not claimed to read the custom XML as an external database. This meets package round-trip and offline diagnostics. It is not a proprietary external engine. The historical automatic-connector alternative stays rejected.

Source policy:

- `MailMergeSourcePolicy` implements `Default` as `Offline`. `prepare_mail_merge_data` still takes an explicit policy. Open, save, and query never retrieve.
- `Offline` never uses `external_records`. `Offline` with `Some(records)` is `Err` under the Q7 answer. That refusal covers an external declaration and an embedded table handed external records. The call does not use those records.
- `CallerSupplied` uses the provided records and never fetches.
- `ExternalSourceBlocked` means the policy denies retrieval. `ExternalSourceUnavailable` means retrieval would be permitted but the caller did not supply records. Both return empty `records` and a diagnostic, not an exception, for an external declaration.
- `mail_merge_package` returns `Ok(None)` when there is no recognized owned mail-merge configuration. That means none of the 13 modeled settings values, no owned store docVar, no qualified known source, no mapping, and no recipient references. Opaque-only residual extension XML, including a leftover `w:mailMerge` or `odso` shell, does not become a configuration. Recognized settings whose required source is missing return `Err`, never `None`. Malformed or ambiguous input returns `Err`, never `None`.
- Unknown imported graph bytes stay untouched. An edit that cannot preserve them refuses. Unknown non-source extensions are preserved.
- Malformed structure or ambiguous ownership is `Err`, not a partial projection. Imported `dataSource` and `odso` `src` values that differ are preserved and are an unsupported projection, not a malformed graph.
- The existing 13-member getter and setter stay scoped to those members and do not erase the graph.
- Copy-on-write applies to embedded custom XML stores. Store identity reuses the F-287 normalized GUID rules for case and braces. It is not a raw byte comparison. The docVar name `rdocx.mailMerge.storeItemId` is exact case. There is no new unbounded source-identity validator. A stale GUID, or a duplicate of the same canonical id, is an ownership `Err`. A recipient part is the exception to generic shared-target copy-on-write, specified below.
- `remove_mail_merge_package` clears all 13 owned settings values, the owned docVar, and the owned known source, mapping, and recipient references. It prunes only provably unshared owned parts. It preserves unrelated and opaque XML nodes and relationships. If only an opaque `w:mailMerge` or `odso` residual remains, a later query returns `None` because the known configuration is gone. A second remove returns `false`. When no recognized configuration is present, remove returns `false`. If unmodelled source-shaped metadata or ambiguous references prevent proving that state without loss, the call returns `Err` before mutation. There is no false partial success, no tombstone, and no second model. A known shared recipient part is preserved and only the owned reference is dropped, and only when that drop is safe.
- Duplicate, foreign, or colliding graphs refuse atomically. Every new `r:id` is allocated without collisions.
- `dataSource`, `headerSource`, and `src` keep their target role and the rest of the retained graph. Only the declared owned subset is edited. The source relationship type is exactly `http://schemas.openxmlformats.org/officeDocument/2006/relationships/mailMergeSource`, `TargetMode` is `External`, and the settings part owns it. Both `dataSource` and ODSO `src` use that type. `headerSource` may use the same relationship type. Ownership is the qualified XML reference, not a global type filter. Preserve `headerSource` and its edge even when the old source edge is shared with it, and allocate a new edge for a new source when that is required. New authoring writes a consistent `dataSource` URI and, where the writer emits it, a consistent ODSO `src` URI. There is one active source. Official Word ODSO `src` precedence when `dataType` is `native` is a fact, and the single source query does not expose an inactive fallback. An explicit full set may normalize known owned source values when the header and opaque graph can be preserved. SDK references: `https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.datasourcereference?view=openxml-3.0.1` and `https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.sourcereference?view=openxml-3.0.1`. Do not confuse the SDK `mailMergeRecipientData` URI with the canonical MS `recipientData` relationship.
- Connection strings and settings are stored and never executed. Credential-bearing URIs are not specially banned. Caller strings are preserved under the XML policy. Diagnostics must not echo those strings back.
- A new URI is an XML 1.0-valid ASCII `file` or `https` absolute declaration. The scheme compare is case-insensitive. Raw whitespace and control characters are refused. Percent triplets must be valid. `https` has a nonempty authority and host. `file` has an absolute nonempty path, and a UNC authority is optional. A userinfo or credential-bearing URI is allowed and stored literally. There is no existence, DNS, or resource check, no full RFC resolver claim, and no new dependency. The value is metadata only and is never fetched. An unsupported new URI or unsupported new settings value makes `set_mail_merge_package` return `Err`. That setter does not return a diagnostic. Only `prepare_mail_merge_data` returns diagnostics. Diagnostic messages stay generic and do not echo a URI, connection string, or query string.
- Imported schemes other than `file` and `https` stay an external raw URI without mutation. They are `UnsupportedSource` only when `CallerSupplied` classification is needed. An `Offline` external declaration is `ExternalSourceBlocked` first, with empty rows, and that block does not need a connector classification.
- No external access occurs on open, save, query, render, or merge.

Embedded table under the selected Q7 custom XML answer:

- Exact XML 1.0 text. Columns are distinct and non-empty. Records are rectangular. Zero rows is valid. Zero columns refuses. Input order is stable. An empty cell is allowed. Escaping uses the existing strict validator. No formula, macro, or SQL payload is executed.
- Q8 selected caps, user answer on 2026-10-23, apply only to owned embedded table XML, counted with markup and escapes: 16 MiB, 100000 rows, and 1024 columns. These are selected payload caps, not a measured benchmark. They do not bound already allocated caller-supplied external rows, and they do not bound mapping or recipient metadata parts. Caller-supplied external rows use the existing flat merge-record shape, conversion, and safety controls. No new global cap is added.
- `mapped_name` stays `String` in Rust and `str` in Python. A new authored value must be one exact canonical role id from the Q16 list, and mapped targets are unique. The options carrier may keep the caller `Vec` sequence. `set_mail_merge_package` canonicalizes XML into all 30 slots. `mail_merge_package` returns logical mappings in canonical Word role order and does not return padding null slots. Empty input mappings emit all 30 null slots in that fixed order.
- A mapped slot serializes `w:type` `dbColumn`. An unused slot serializes `w:type` `null`. Word ignores `mappedName` and reads `fieldMapData` by position. The slot count is only the direct namespace-qualified Word `fieldMapData` children of the qualified ODSO. Foreign names, local names, and children nested under an unknown wrapper do not count. Query derives the logical role from physical slot 0 through 29. Null slots count as positions and are not logical mappings. Unknown raw `mappedName`, attributes, opaque extensions, and nullable entries stay verbatim and are not assigned. Unsafe assignment that cannot keep that opaque XML returns `Err` before mutation. A narrow edit of the existing 13 settings does not rewrite ODSO. This is not a new Word oracle and it does not promise Word execution.
- Embedded `column` is zero-based and must be less than `columns.len()`. `source_name` must equal `columns[column]`. External `column` is only a known `u32` bound. No external dataset length is known. Invalid role, duplicate target, out-of-range column, and name-index mismatch refuse atomically.
- A structurally valid active mapping with an unknown type, or more than 30 mapping records, makes `mail_merge_package` return `Err` for an unsupported projection. Open and save of that source stay unchanged. `prepare_mail_merge_data` then returns `UnsupportedSource` and no rows. Malformed structure and ambiguous ownership stay `Err`. There is no `AmbiguousOwnership` diagnostic. An ambiguous graph always returns `Err`, and no diagnostic variant produces it.
- An explicit full assignment canonicalizes owned known mapping values. It retains opaque XML when position and owner can be preserved. Otherwise it returns `Err` before mutation and does not drop unknowns.
- Recipient `column` is zero-based. `active` is inclusion metadata, not a row cell.

`prepare_mail_merge_data` checks, in this order: Python outer argument types and policy, then native `Offline` plus `Some` as `Err` without using the rows, then an absent owned package as `Err` with the message `mail-merge package is absent` rather than fabricated empty records, then embedded plus `Some` as `Err` regardless of policy, then a malformed or ambiguous graph as `Err`, then external plus `Offline` as `ExternalSourceBlocked` with empty rows, then `CallerSupplied` external whose URI, native alias, mapping, or recipient selection cannot be applied as `UnsupportedSource` with empty rows before any no-records unavailable result, then a valid external source with `CallerSupplied` and `None` as `ExternalSourceUnavailable`, then a valid faithful `Some` as copied column-key rows, including a valid empty row list, then embedded with `None` as the table rows unless native evaluation is required and unsupported, in which case the result is `UnsupportedSource`. A clean embedded table with zero rows is a legitimate empty record list and no diagnostics. A valid `CallerSupplied` `Some` of empty rows is a legitimate empty record list and is distinct from `None`, which is `ExternalSourceUnavailable`. No path returns partial records. Different imported `dataSource` and `src` URIs are `UnsupportedSource` when classification is needed, and `Offline` blocked precedence still wins. They are not labelled invalid OOXML. An embedded owned docVar plus any known external source reference stays the existing ambiguous-ownership `Err`.

Recipient ids stay opaque and are not interpreted. Any recipient selection that is not proved faithful refuses with `UnsupportedSource`. This story does not hash those ids. An identity mapping is faithful only when the actual column names and values need no alias translation and every required column exists. There is no silent native alias engine. The query and the new writer do not implement arbitrary `MERGEFIELD` alias or native filter execution. The role list is an XML fact. It is not a passed Word execution, and this story adds no new Word oracle beyond the selected package round-trip.

`prepare_mail_merge_data` on a clean embedded package under `Offline`, with `None` records and a faithful identity mapping, returns those rows by column name and an empty diagnostic list. The existing `mail_merge` execution of prepared raw column maps stays unaffected.

Complete-package authoring policy, explicit and not a fabricated claim that every historical document requires `mainDocumentType`. The native complete package setter requires `Some(main_document_type)` and `Some(data_type)` with one of `textFile`, `database`, `spreadsheet`, `query`, `odbc`, or `native`. That enum is the schema set named by Open XML SDK 3.0.1 `MailMergeDataValues` at `https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.mailmergedatavalues?view=openxml-3.0.1`. The pinned JSON `CT_MailMerge` sequence is at line 48981 of the same v3.0.1 wordprocessingml schema used by F-288. `dataType` is required on a new complete package. The existing narrow 13-member settings setter and import preservation stay unchanged. The snapshot query retains a wider imported `data_type` as `String`. The new setter does not accept an unknown value. Unknown imported `dataType` values are preserved without assignment. A new complete package with an unsupported source or unsupported settings value returns `Err` from `set_mail_merge_package`. Embedded default settings `formLetters` and `textFile` write valid declared settings and do not pretend Word uses the custom table. Null field-map type is the SDK `MailMergeFieldType` fact at `https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.mailmergefieldtype?view=openxml-3.0.1`. Column name semantics are the SDK `Name` fact at `https://learn.microsoft.com/en-us/dotnet/api/documentformat.openxml.wordprocessing.name?view=openxml-3.0.1`. Position semantics are MS-OI29500 2.1.369 at `https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oi29500/df0bbd4b-c5e4-4836-a65c-f956185a0f8d`. The 30 role ids, in order, are Unique, CourtesyTitle, FirstName, MiddleName, LastName, Suffix, Nickname, JobTitle, Company, Address1, Address2, City, State, PostalCode, CountryorRegion, BusinessPhone, BusinessFax, HomePhone, HomeFax, EmailAddress, WebPage, SpouseCourtesyTitle, SpouseFirstName, SpouseMiddleName, SpouseLastName, SpouseNickname, RubyFirstName, RubyLastName, Address3, Department. No new public enum or file is added.

Embedded ownership and schema closure. Custom XML schema URI `urn:rdocx:mail-merge-data:v1`. Root element is qualified `mergeData`. The sequence is `columns` (`column` with `name`) then `records` (`record` of `cell` text). The namespace constant is used on all children. There is exactly one column node per declared column. Cell rows are rectangular. Count bounds apply. The internal writer is XML 1.0 strict, with no DTD and no entities except escaped literal text. Metadata identifies the authoritative store by the existing document variable `rdocx.mailMerge.storeItemId`, whose name match is exact case and whose value is the store GUID under the F-287 normalized identity rules for case and braces. Use the existing `CT_Settings` / `docVars` owner from F-270 and the F-287 custom XML graph. Do not discover the owned store by namespace alone and do not assume paths. An explicit docVar reference resolves the store id and the properties and schema identity. A stale, malformed, or duplicate variable or store target is `Err` and preserves bytes, including two references that share one canonical GUID. A known variable that already exists without a valid owned graph refuses before mutation and does not overwrite an unrelated user docVar. Only one active source exists. The embedded new writer removes prior owned `dataSource` and `odso` `src` references safely. The external new writer removes its own store variable reference. Unmodelled `headerSource` and its relationship edge, and opaque other children, are preserved even when an old source edge is shared with the header. An imported conflicting external source plus a library variable reference is `Err` for ambiguous ownership, with no unstated priority. Copy-on-write of a bound or shared owned store allocates a new store GUID and updates only this docVar reference, leaving the old store and other bindings intact. Remove prunes only provably unshared owned stores, clears the own reference and the modeled source graph, and preserves known shared and unowned parts. This is a reference in the existing model, not a second form or document model.

Recipient part exception. The package has at most one recipient-data part across both known content types, not merely the one relationship that is referenced. The new writer uses content type `application/vnd.openxmlformats-officedocument.wordprocessingml.mailMergeRecipientData+xml`, root namespace `http://schemas.openxmlformats.org/wordprocessingml/2006/main`, root element `recipients`, relationship type `http://schemas.openxmlformats.org/officeDocument/2006/relationships/recipientData`, `TargetMode` `Internal`, and the settings part as owner. The recipient part has no outgoing relationships. The alternate Microsoft content type and root `wne` hash format is a recognized unsupported projection. Do not invent a hash conversion. Preserve it without assignment, and `prepare_mail_merge_data` returns `UnsupportedSource` when that projection must be classified. If replacing a shared recipient part would need a second copy-on-write part, return `Err` atomically and do not violate the one-part maximum. An existing unrelated recipient part is not overwritten or reclassified by name. Allocate or reuse only a provably owned valid single part, or return `Err`. Removing a known shared recipient part preserves that part and drops only the owned reference, and only when the drop is safe. The primary source is MS-OI29500 at `https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oi29500/af3dc913-8c08-4843-ab40-495a92170b96`, which states both known content types, the namespace, the maximum of one, the settings owner, and internal mode. The canonical relationship ends in `/recipientData`. The SDK `/mailMergeRecipientData` string is a different URI.

Size caps apply only to the owned embedded table XML, including markup and escapes: 100000 rows and 1024 columns, inside the selected 16 MiB encoded cap. Mapping and recipient parts are not that table. Caller-supplied external rows already allocated before the library call are not bounded by this owned-XML budget. They keep the existing flat merge-record controls. The native writer preflights embedded counts and the checked encoded length before allocating an output buffer over 16 MiB. Python `MailMergePackageSource.embedded` conversion stays bounded before a whole `Vec` is built, and the native writer preflights the same cap. It counts outer and row lengths and UTF-8, then the encoded budget, while building. Respect XML escape bloat. A partial temporary allocation is never published. No global package changes.

## Python API contract

Frozen public carrier classes are backed by native values. There is no Python model of the graph. Every new native operation maps to the exact Python method below. No consumer is left as Python-excluded. Existing native flat and rich `mail_merge` stay unchanged. Python already writes `MERGEFIELD` through `Run.add_field`. New Python merge execution is not promised.

All classes are frozen and public on `rdocx` and `rdocx._rdocx`, with `_rdocx.pyi`, `__all__`, and `__init__` imports aligned. Factory-private classes use `__new__(cls, *, _private: _Never) -> Class`. Public input constructors are callable. Fields are readonly. Sequence inputs are copied and frozen. `str` and `bytes` are rejected where the input is a sequence of rows. Bool is not an integer. Defaults are the actual constructor call values. On CPython 3.10 or newer, `inspect.signature` exposes the same names, kinds, order and defaults. Use the existing PyO3 `text_signature` where needed so `()` is not `Ellipsis`. On CPython 3.9, the supported `abi3-py39` wheel has no native class text signature and `inspect.signature` raises `ValueError` for these constructors. Actual calls, keyword-only rules, defaults, explicit `None`, primitive errors, class identity and stubs retain the same contract on both versions. No wrapper or synthesized signature is added.

Root selected this bounded ABI capability amendment under the user's standing recommended-choice instruction after the actual c62 normal-wheel Python 3.9 full run recorded 1183 passed, two constructor-introspection failures and 25 debug-only skips. That earlier full run remains failed under its previous universal metadata expectation. The pinned PyO3 0.29 guide, `class.md` under its text-signature section, documents the `abi3` limitation before Python 3.10. Its `pyclass` macro has no class-level `text_signature` option, and the pre-3.10 heap-type workaround requires the non-limited API. The final tests assert unavailable class metadata on 3.9 and actual complete metadata on 3.10 or newer, with all constructor behavior tests still active. This changes the introspection acceptance expectation, not constructor call behavior or Python 3.9 runtime support.


Shared error and lifetime contract. Strict wrong Python shapes and types raise `TypeError`. Negative or out-of-domain primitives raise `ValueError`. Nonnegative integers outside the `u32`, `u64`, or `usize` extraction range raise `OverflowError`. Bool is forbidden for numeric indices, counts, and sizes. Native semantic, XML, and package errors keep the existing `rdocx_to_pyerr` at `src/lib.rs:117`. Oxml maps to `XmlError`, package and io map to `PackageError`, and Other and Story map to `RdocxError`. Do not introduce another error hierarchy and do not convert every native error to `ValueError`. Factories validate in native code before return. Results are prepared before publishing a candidate. Refusal and no-op preserve bytes, revisions, and live handles. Changed publication follows the existing physical owner policy and never revives dead owners.

```python
class MailMergePackageSource:  # frozen, factory private
    @staticmethod
    def embedded(columns: Sequence[str], records: Sequence[Sequence[str]]) -> MailMergePackageSource: ...
    @staticmethod
    def external(uri: str) -> MailMergePackageSource: ...
    # readonly kind: Literal["embedded", "external"]
    # readonly columns: tuple[str, ...]
    # readonly records: tuple[tuple[str, ...], ...]
    # readonly uri: str | None
    # external getters that do not apply are empty tuples. Embedded uri is None. All inputs are copied.

class MailMergeSettings:  # frozen, public constructor. Wraps existing native MailMerge. No new native settings type.
    def __init__(
        self,
        *,
        main_document_type: Literal["catalog", "envelopes", "mailingLabels", "formLetters", "email", "fax"] | None = "formLetters",
        link_to_query: bool | None = None,
        data_type: str | None = "textFile",
        connect_string: str | None = None,
        query: str | None = None,
        do_not_suppress_blank_lines: bool | None = None,
        destination: Literal["newDocument", "printer", "email", "fax"] | None = None,
        address_field_name: str | None = None,
        mail_subject: str | None = None,
        mail_as_attachment: bool | None = None,
        view_merged_data: bool | None = None,
        active_record: int | None = None,
        check_errors: int | None = None,
    ) -> None: ...
    # all 13 readonly properties keep these types

class MailMergeFieldMapping:  # frozen, public constructor
    def __init__(self, column: int, source_name: str, mapped_name: str, *, language: str | None = None, dynamic_address: bool | None = None) -> None: ...
    # readonly column, source_name, mapped_name, language, dynamic_address
    # column is 0 through u32::MAX and is not bool

class MailMergeRecipient:  # frozen, public constructor
    def __init__(self, column: int, unique_tag: bytes, *, active: bool = True) -> None: ...
    # readonly column, unique_tag, active. unique_tag stays opaque. No hash algorithm.

class MailMergePackageOptions:  # frozen, public constructor. Exact native four fields.
    def __init__(
        self,
        source: MailMergePackageSource,
        *,
        settings: MailMergeSettings | None = None,
        mappings: Sequence[MailMergeFieldMapping] = (),
        recipients: Sequence[MailMergeRecipient] = (),
    ) -> None: ...
    # readonly source and settings. mappings and recipients are immutable tuples.
    # settings None creates the default MailMergeSettings above. It is not a reference clear.

class MailMergePackageDiagnostic:  # frozen, factory private
    # readonly kind: Literal["external_source_blocked", "external_source_unavailable", "unsupported_source"]
    # readonly message: str

class MailMergePreparedData:  # frozen, factory private
    # readonly records: tuple[dict[str, str], ...]
    # readonly diagnostics: tuple[MailMergePackageDiagnostic, ...]
    # each records getter returns fresh owned map copies, so mutation cannot change another getter, the carrier, or the document

class Document:
    def set_mail_merge_package(self, options: MailMergePackageOptions) -> None: ...
    def mail_merge_package(self) -> MailMergePackageOptions | None: ...
    def remove_mail_merge_package(self) -> bool: ...
    def prepare_mail_merge_data(
        self,
        *,
        policy: Literal["offline", "caller_supplied"] = "offline",
        external_records: Sequence[Mapping[str, str]] | None = None,
    ) -> MailMergePreparedData: ...
```

`data_type` on a snapshot query stays a public `str` so a wider imported domain can round-trip. A new full package setter validates the narrow enum and returns `Err` for an unknown value. It does not accept that unknown value. `active_record` and `check_errors` are the exact `i32` range and exclude bool. `None` omits the member. Faithful column rows are returned, or an unsupported diagnostic with no records. There is no retrieval. Keys and values are actual `str` values, with no implicit `str()`. `offline` plus some rows is `Err` and does not use the rows. Embedded plus some rows is `Err` regardless of policy. There is no second row source. Only prepare returns diagnostics.

Python bounded conversion counts outer length, row lengths, and UTF-8, then the encoded budget, while building. It never extracts an unbounded whole vector before rejection.

Python tests in existing `tests/test_core.py`: `test_mail_merge_package_authoring_round_trips_public_api`, `test_mail_merge_data_preparation_is_offline_and_diagnostic`, `test_mail_merge_package_snapshot_and_conversion_are_atomic`, and `test_mail_merge_mappings_use_word_slots`, plus typed constructors and factories for every public type. They cover mutable input copy, nonce paths, shared-store binding copy-on-write, the recipient one-part exception, ownership docVar collision, GUID case, absence, zero rows, a second remove, an opaque-only residual, source-shaped refusal, different external URIs, header-edge ownership, and limit boundaries before publication. Those cases stay inside the existing tests. Typed constructors and factories use existing `crates/rdocx-py/tests/typing_smoke.py`. Behaviour tests stay in `crates/rdocx-py/tests/test_core.py`. No temporary string runner and no new tracked file. The final installed typing, stub, and full suite run only in the final union. Native Rust merge consumption stays unchanged.

Binding files are existing `src/document.rs` and `src/lib.rs` under the Python crate, plus `python/rdocx/__init__.py`, `python/rdocx/_rdocx.pyi`, and `tests/test_core.py`. F-290 does not add a `run.rs` binding. Native and HLD claims retain the files already listed in this plan. README measured package inventory may be recomputed by the existing `README.md` and `scripts/readme_doctests.py` only if a publication archive change requires it. Do not fabricate those values. No Cargo dependency change and no new module.

The prior native planning basis remains 8 to 11 developer-days across the sprint. Q1 Python scope is an explicit planning estimate of 4 to 6 additional developer-days, for an expanded planning total of 12 to 17. That figure is an estimate, not a measurement, and it is not an undisclosed capacity promise.

## Consumer map

| Rule | Entry points and outputs | Shared provider and caller policy | Lifecycle stages compared |
|---|---|---|---|
| Write package | `set_mail_merge_package` | Caller options. Settings part owns source and recipient relationships | Serialize, reopen |
| Read package | `mail_merge_package` | `None` when no recognized owned configuration. Opaque residual XML is still `None`. Recognized settings with a missing required source, and malformed or ambiguous input, are `Err` | Read, reopen |
| Remove | `remove_mail_merge_package` | Clear the 13 owned settings, owned docVar, and owned known source, mapping, and recipient references. Prune only provably unshared owned parts. Second remove and a missing recognized configuration return `false` | Serialize |
| Narrow settings | Existing getter and setter at `document.rs:33307` | 13 members only. Graph preserved | Update, serialize |
| Prepare | `prepare_mail_merge_data` | Explicit policy. No fetch | Query |
| Merge | Existing `mail_merge(prepared.records)` or caller-prepared records | Execution unchanged | Update |
| Python, WASM, CLI | Q1 selected answer: new Rust and Python authoring APIs. Every new native operation is the matching method in `## Python API contract`. WASM and CLI gain no new authoring methods. Existing consumers retain preservation and current-value behavior. Existing `Run.add_field` still writes `MERGEFIELD` | Frozen carriers, four document methods, exports, and stubs. Codex root approved this contract on 2026-10-23. Implementation and final gates stay pending | Reopen |

## Behavior contracts and evidence

| Surface or placement | Exact behavior and boundary examples | Primary source identity and location | Local discriminator or pending evidence |
|---|---|---|---|
| External declaration | URI stored. Settings part owns the relationship. Noncanonical part paths reopen | MS-OI29500 and ISO 17.14.9. Not a Word run | `authored_mail_merge_package_retains_source_mappings_and_recipients` |
| Recipient part | At most one recipient-data part across both known content types. New writer uses the standard content type, main namespace, root `recipients`, relationship type ending `/recipientData`, internal mode, and the settings owner. No relationships leave the part. Alternate `wne` hash form is preserved unsupported. Opaque `unique_tag` round-trips without a hash conversion | MS-OI29500 `https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oi29500/af3dc913-8c08-4843-ab40-495a92170b96`. SDK `UniqueTag`. Not the SDK `mailMergeRecipientData` URI | A1 and A5 |
| Embedded rectangle | Columns `a`, `b`. Rows `["x",""]`, `["&","<"]`. Zero further rows allowed | Q7 selected versioned custom XML. Not a Word-native embedded source | `embedded_merge_records_round_trip_without_external_access` |
| Bounds | 1025th column or 100001st row refuses before allocation under the selected Q8 caps | Q8 answered 2026-10-23. Not a benchmark | A5 |
| Offline external | `Offline` yields empty records and `ExternalSourceBlocked` | Chosen policy | `offline_merge_sources_return_explicit_diagnostics` |
| Unavailable | Policy would allow caller records but `None` was passed | `ExternalSourceUnavailable`, empty records, `Ok` | Same test |
| Fail closed | Native recipient filter or unfaithful mapping | Empty records, `UnsupportedSource`, metadata still present | A3 arm of the diagnostic test |
| Narrow edit | Change one of the 13 settings | ODSO and recipient parts remain | `mail_merge_graph_uses_settings_relationship_scope` |
| Known shared store | Replacement of a known shared owned custom XML store | Copy-on-write under F-287 GUID identity. Known shared and unowned parts stay. A shared recipient part is not copied into a second part | `merge_package_mutations_preserve_unowned_parts_and_refuse_ambiguity` |
| Removal | Provably unshared owned parts, then a second remove, and an opaque-only residual | Owned known configuration is cleared. Those parts are pruned. Opaque residual stays and the next query is `None`. Second remove is `false`. A known shared recipient part stays and only the owned reference drops | Same test |
| Ambiguity | A graph whose sharing cannot be proved, two recipient parts, source-shaped metadata that cannot be cleared without loss, or different external `dataSource` and `src` URIs | `Err` before mutation for an unsafe clear or an ambiguous graph. Different URIs are an unsupported projection, not malformed OOXML | Same test |
| Header edge | `headerSource` shares the source relationship type, and may share the old source edge | The header edge stays. A new source allocates its own edge. Ownership follows the qualified XML reference | Same test |
| GUID case | Store id braces and hex case differ, or the same canonical id is duplicated | F-287 normalized identity. Duplicate canonical id is `Err` before mutation. DocVar name match is exact case | Same test |
| Main document | Source relationship is not parked on the main part | Settings owner | A1 |
| Canonical slots | Out-of-order FirstName and LastName, plus Department, with gaps | All 30 positions. Null padding. Query order is the Word role order. Selection uses position | `author_mail_merge_mappings_use_word_slots` |
| Import projection | Unknown raw mappedName before assignment. Active unknown type or more than 30 records | Verbatim until assignment. Unsupported projection is `Err` on query. Prepare returns `UnsupportedSource` and no rows. Opaque assignment that cannot keep position returns `Err` before mutation | Same test |
| Prepare order | Offline plus rows, embedded plus rows, blocked external, unsupported caller source, unavailable, faithful rows | The priority in Approach. No partial records. Existing `mail_merge` raw column maps stay | A3 |

## Dependency readiness

F-233, F-270, and F-278 are done in BACKLOG. F-290 is pending and unclaimed. No F-288 or F-289 contract is required to implement F-290. Custom XML store mechanics from F-287 may be reused for the selected Q7 embedded schema. That reuse stays inside existing store APIs. Codex root approved this contract on 2026-10-23 at source `a57be365`. Q7 and Q8 are answered. The Python contract is approved and unimplemented. The prior native planning basis remains 8 to 11 developer-days across the sprint. The expanded planning total is 12 to 17 developer-days, an estimate and not a measurement.

## Acceptance criteria

| ID | Input or setup | Expected observation or refusal | Consumers | Local test | Final gate or pending evidence |
|---|---|---|---|---|---|
| A1 | External source, settings, mappings, recipients. Noncanonical paths. Python `set_mail_merge_package` and `mail_merge_package` | Save and reopen. Relationship owner is the settings part. DocVar `rdocx.mailMerge.storeItemId` is the owned-store identity when an embedded store exists | Rust and the Python contract | `authored_mail_merge_package_retains_source_mappings_and_recipients` and `test_mail_merge_package_authoring_round_trips_public_api` | Round-trip gate. Native Word not required for this custom XML |
| A2 | Embedded special characters, empty cell, zero, one, and many rows, rectangular bounds, stable order. Mutable inputs are copied | Exact cells. Zero columns refuse. Selected Q8 caps, including escape bloat, before publication | Rust and Python | `embedded_merge_records_round_trip_without_external_access` | No network in the test |
| A3 | Offline, unavailable, and fail-closed unsupported evaluation. `CallerSupplied` returns faithful column-keyed records, or no records plus `UnsupportedSource` when native mapping or recipient selection cannot be applied. Offline plus some rows is `Err`. Embedded plus some rows is `Err` regardless of policy. An absent owned package is `Err` with message `mail-merge package is absent`. A clean embedded zero-row table and a valid `CallerSupplied` empty row list are legitimate empty results. `None` on a valid external source stays unavailable | Diagnostics as specified. Zero retrieval. No partial native filter result. No fabricated rows for an absent package. Each records getter returns a fresh map copy | Rust and Python | `offline_merge_sources_return_explicit_diagnostics` and `test_mail_merge_data_preparation_is_offline_and_diagnostic` | Round-trip |
| A4 | Narrow settings edit. Unknown imported bytes. Malformed and ambiguous packages. Conflicting external source plus library docVar | Graph kept. Unknown bytes exact. Ambiguity is `Err`. No unstated priority | Rust and Python | `mail_merge_graph_uses_settings_relationship_scope` | Round-trip |
| A5 | Late failure, `r:id` collision, shared store, recipient maximum of one, prune, second remove, opaque-only residual, source-shaped refusal, different external URIs, `headerSource` edge ownership, GUID case, bounds, docVar collision, copy-on-write of a shared owned store | Atomic. Unowned parts and the old store stay. Only this docVar reference moves to the new GUID under F-287 normalized identity. A shared recipient part is not copied. Source-shaped loss and an ambiguous graph refuse before mutation. Limits refuse before publication | Rust and Python | `merge_package_mutations_preserve_unowned_parts_and_refuse_ambiguity` and `test_mail_merge_package_snapshot_and_conversion_are_atomic` | Package safety unchanged globally |
| A6 | Public package API plus existing `mail_merge` on prepared records. Python does not add a merge engine | Existing consumers still open packages. Native Rust merge consumption unchanged. Prepared raw column maps stay the execution input. Archive riders on publish | Rust and Python. WASM and CLI gain no new authoring methods | Existing merge tests stay green | Final union only: `/verify --full`, hash harness, strict mypy and stubtest, WASM, genuine dry-run with archive under 10 MiB |
| A7 | Out-of-order FirstName and LastName plus Department, gaps, empty mappings, invalid role, duplicate target, out-of-range column, name-index mismatch, unknown raw mappedName, and an unrepresentable active mapping | XML has 30 slots, `dbColumn` on mapped slots and `null` on unused slots. Query returns canonical role order without null padding. Negatives refuse atomically. Unknown raw mappedName stays verbatim before assignment. Column selection uses position. Unsupported import projection and opaque assignment that cannot keep position refuse. Open and save of an unrepresentable source stay unchanged | Rust and Python | `author_mail_merge_mappings_use_word_slots` and `test_mail_merge_mappings_use_word_slots` | Package round-trip. No new Word oracle |

Backlog promise: the public package retains its declared source and mappings, and unavailable external data returns diagnostics. Native custom-XML interoperability is intentionally outside the gate.

## Test matrix

| Case | Modes or state combined | Expected difference or agreement | Wrong implementation rejected |
|---|---|---|---|
| C1 | External URI, two mappings, two recipients, odd part names | Owner and URI ending `/recipientData` | A main-part relationship fails |
| C2 | Embedded `&`, `<`, empty cell, 0 rows, 2 rows | Rectangular text and order | A trimmed cell or a fetch fails |
| C3 | `Offline` with and without smuggled records. Unavailable `None`. Unsupported native filter | No silent use. Empty records plus the named diagnostic. No partial rows | A homegrown recipient hash fails |
| C4 | Narrow setter then package read. Foreign unknown element | 13-member change only. Unknown span identical | A graph wipe fails |
| C5 | Collision, known shared store replacement, a shared recipient part, provably unshared removal, a second remove, an opaque-only residual, source-shaped metadata, different external URIs, a shared `headerSource` edge, GUID case, and a truly ambiguous graph | Copy-on-write for the known shared store only. Recipient copy-on-write that would add a second part is `Err`. Prune only proved-unshared owned parts. Query is `None` after a successful clear that leaves only opaque XML. Second remove is `false`. `Err` before mutation when the clear cannot be proved or the graph is ambiguous | A deleted shared or unowned part, a second recipient part, or a dropped header edge fails |
| C6 | `mail_merge` on `prepare` records for a faithful embedded table | Same rows the caller could have supplied. Raw column maps stay | A second merge engine or a native alias engine fails |
| C7 | Empty mappings, then FirstName, LastName, and Department out of order | Thirty null slots, then canonical query order with null padding omitted | A raw caller-vector order on the wire fails |

Omitted: live SQL, HTTP, and filesystem connectors, and any pretence of a Word-native embedded source. A new Python merge engine stays out. The historical Q1 recommendation of native-only scope stays in the question table as history. WASM and CLI gain no new authoring method. A native oracle is not the backlog gate.

## Rejected alternatives

- `MailMergePackageInfo` as a twin of `MailMergePackageOptions`.
- A new mail module.
- Treating `unique_tag` as a portable hash and filtering recipients with it.
- Pretending Word reads `urn:rdocx:mail-merge-data:v1` as its datasource.
- An unsolicited credential-URI ban.
- Silent use of caller records under `Offline`.
- Changing global package size limits. The selected Q8 caps apply only to owned embedded table XML.
- Marking schema child order or absent-source behaviour as Word-verified. Primary citations are the ISO, MS-OI29500, and SDK pages. A Word run has not happened.

## Test plan

Backlog test gate, verbatim: round-trip. A public-created merge package retains its declared source and mappings, and unavailable external data returns diagnostics.

| Timing | Category | Test | Asserts |
|---|---|---|---|
| local scoped | regression | `authored_mail_merge_package_retains_source_mappings_and_recipients` | A1 relationship owner and reopen |
| local scoped | regression | `embedded_merge_records_round_trip_without_external_access` | A2 cells, order, and no I/O |
| local scoped | regression | `offline_merge_sources_return_explicit_diagnostics` | A3 blocked, unavailable, fail-closed, absent-package `Err`, clean embedded zero rows, and a valid empty `CallerSupplied` list distinct from `None` |
| local scoped | regression | `mail_merge_graph_uses_settings_relationship_scope` | A4 narrow edit and settings owner |
| local scoped | regression | `merge_package_mutations_preserve_unowned_parts_and_refuse_ambiguity` | A5 atomicity and pruning, including absence, zero rows, second remove, opaque-only residual, source-shaped refusal, shared recipient maximum of one, different external URIs, `headerSource` edge ownership, and GUID case. These are cases of this existing native test and the existing Python atomicity test. No per-detail mirror file |
| local scoped | regression | `author_mail_merge_mappings_use_word_slots` | A7 slots, query order, negatives, preserved unknown mappedName, position selection, unsupported projection, opaque assignment refusal |
| local scoped | regression | `test_mail_merge_package_authoring_round_trips_public_api` | Public constructors, copied inputs, reopen, and docVar identity |
| local scoped | regression | `test_mail_merge_data_preparation_is_offline_and_diagnostic` | Offline and embedded refusal of supplied rows, fresh record maps, no retrieval |
| local scoped | regression | `test_mail_merge_package_snapshot_and_conversion_are_atomic` | Nonce paths, shared-store copy-on-write, docVar collision, embedded-table limit boundaries before publication |
| local scoped | regression | `test_mail_merge_mappings_use_word_slots` | A7 on the public Python constructors in existing `crates/rdocx-py/tests/test_core.py`, with typed constructor use in existing `crates/rdocx-py/tests/typing_smoke.py` |
| local scoped | typing | typed constructors and factories for every public type | Existing `crates/rdocx-py/tests/typing_smoke.py`. Behaviour tests stay in `crates/rdocx-py/tests/test_core.py`. No temporary string runner and no new tracked file |
| final sprint | risk rider | Integrated `/verify --full`, hash check, strict mypy and stubtest, WASM, genuine publish dry-run with archive under 10 MiB | Once, in the final union. No native oracle mandated for the custom XML table. Scoped public API tests stay on this feature |
| before final freeze | metadata | `python3 -m unittest scripts.test_sprint_workflow.SprintWorkflowTests.test_modern_docx_capability_matrix_has_closed_classifications_and_evidence scripts.test_sprint_workflow.SprintWorkflowTests.test_every_incomplete_modern_docx_row_has_one_live_owner` | DOCX-057 cells, classification, evidence, and owner match the implemented and tested scope. `Y` means that bounded operation exists today. Native Word parity stays explicit debt. Do not mark the row complete only so the assertion passes |
| after completion metadata | metadata | `python3 -m unittest scripts.test_sprint_workflow.SprintWorkflowTests.test_every_incomplete_modern_docx_row_has_one_live_owner` | Backlog and current-sprint status agree with the DOCX-057 owner. A previous green run does not prove the new status. `capability_owner_problems` still rejects an incomplete owner that is done |

No new test binary. Provider dependencies are already done, so this story has no F-288 prefix wait.

## HLD impact

- `docs/hld/14-development-backlog.md`, the implemented F-290 package contract, with final acceptance kept separate
- `docs/hld/02-scope-and-non-goals.md`, row DOCX-057 for F-290 (offline declaration, no connector execution)
- `docs/hld/03-architecture.md`, "What stays put"
- `docs/hld/04-opc-and-packaging.md`, "The package" and "Relationship types"
- `docs/hld/10-bindings-spec.md`, "Native Word facade stability"
- `docs/hld/12-testing-strategy.md`, "Test taxonomy"

## Risk routing

- Parser or serialiser. Reuse `MAIL_MERGE_ORDER` for `CT_MailMerge`. ODSO child order and recipient order are work to add in `settings.rs`. Preserve unknown spans byte for byte.
- Public API. Semver note. Genuine dry-run with archive under 10 MiB, only in the final union. README measured package inventory may be recomputed by the existing `README.md` and `scripts/readme_doctests.py` only if that archive change requires it. Do not fabricate those values. No Cargo dependency change and no new module. The prior native planning basis remains 8 to 11 developer-days across the sprint. Q1 Python scope is an explicit planning estimate of 4 to 6 additional developer-days, for an expanded planning total of 12 to 17. That figure is an estimate, not a measurement, and it is not an undisclosed capacity promise.
- OPC relationships. Settings part owns the exact `mailMergeSource` relationship for both `dataSource` and ODSO `src`, and the exact `recipientData` relationship. `headerSource` may share that source type and is owned by its qualified XML reference. Collision allocation. At most one recipient-data part.
- Compatibility and ordering. Column indexes, recipient order, and record order specified above. Direct qualified `fieldMapData` children only.
- New projection. `None` means no recognized owned configuration, including an opaque-only residual. Recognized settings with a missing required source are `Err`. Unknown graphs are preserved and refuse unsafe edits. Different external URIs are an unsupported projection.
- Multiple parents. Settings part versus main document. Do not relocate the relationship onto the main part.
- Shared consumers. Existing narrow settings APIs and `mail_merge` stay. Rust and Python gain package authoring under Q1, with every new native operation mapped in `## Python API contract`. WASM and CLI gain no new authoring methods and retain preservation and current-value behavior.
- Grammar and empty collections. Zero rows, zero columns, empty cells, empty diagnostics.
- Performance bound. Selected Q8 caps are correctness limits checked before allocation, not a timed benchmark. If a later story adds a timing budget, pin it separately.
- External oracle row is not a backlog mandate for the embedded schema. Do not record a fake Word pass. PyO3 is in scope under the Q1 answer. Scoped affected public API tests and the small typed use run with this feature. Strict mypy, stubtest, the installed full suite, WASM, publish dry-run, full verify, and the hash harness run only in the final union. WASM gains no new authoring method. Workspace tests keep `--exclude rdocx-py --exclude rpptx-py`. Codex root semantic validation on 2026-10-23 approved this contract. Local implementation exists. The final union stays pending.

## Hash harness

Expected unchanged for existing fixtures. New package bytes belong to new tests. No baseline re-recording. Merge execution output changes only when a caller passes different prepared records.

## Exclusive resources

Product source and test files already named in this plan stay the product lease. This amendment adds one existing file:

- `scripts/test_sprint_workflow.py`, only the expected live-owner inventory in `test_every_incomplete_modern_docx_row_has_one_live_owner`, and only when DOCX-057 genuinely leaves the incomplete set. Do not change the closed-classification rules in `test_modern_docx_capability_matrix_has_closed_classifications_and_evidence`. Do not add a done-owner exception. Do not edit this file while the row stays incomplete. The edit follows the F-288 and F-289 leases of the same file.

Before final source freeze, reconcile DOCX-057 honestly. An incomplete row cannot name a done owner. A resolved row cannot keep F-290 as owner. Satisfy that with cells and evidence from the tested scope. `Y` is a complete bounded operation today, not native Word parity. Leave that native debt explicit in the evidence boundary. Do not record a Word pass for the custom XML table.

## Implementation checklist

- [x] Add the types and four methods in existing `field.rs` / `settings.rs` and export them. Native and HLD edits stay on files already named in this plan.
- [x] Bind Python only in existing `crates/rdocx-py/src/document.rs`, `crates/rdocx-py/src/lib.rs`, `python/rdocx/__init__.py`, `python/rdocx/_rdocx.pyi`, and `tests/test_core.py`. No new tracked file, module, or test binary.
- [x] Reuse `MAIL_MERGE_ORDER` for `CT_MailMerge`. Add ODSO child order and recipient order in `settings.rs`.
- [x] Use the existing `mailMergeSource` and `recipientData` relationship types. They are private string constants beside the encoder. No new relationship module was added.
- [x] Keep the 13-member settings API graph-preserving.
- [x] Implement prepare with explicit policy and fail-closed unsupported evaluation.
- [x] Preserve unknown bytes and refuse ambiguous replacement.
- [x] Apply the selected Q8 caps of 16 MiB encoded XML, 100000 records, and 1024 columns. They are not a measured benchmark.
- [x] Escape embedded text through the existing strict XML validator.
- [x] Add the native tests, including `author_mail_merge_mappings_use_word_slots`, to the existing entrypoint and the named Python tests, including `test_mail_merge_mappings_use_word_slots`, to existing `tests/test_core.py`.
- [x] Register every frozen class on public `rdocx` and `rdocx._rdocx`, with `_rdocx.pyi`, `__all__`, and `__init__` imports aligned.
- [x] Identify the owned store only through `rdocx.mailMerge.storeItemId` on the existing F-270 docVar owner and the F-287 custom XML graph.
- [x] Leave Word-as-database behaviour unclaimed.
- [x] Reconcile DOCX-057 before final source freeze. The row is `complete` with owner `-`. Create, read, mutate, remove, and save-reopen are `Y`. Story is `package`. Layout and render are `NA`. Determinism, Native, and Python are `Y`. WASM and CLI are `B`. Native `Y` is the bounded Rust facade and is not Word interop. The evidence boundary is the offline versioned rdocx table plus Word metadata, with no connector, no new merge engine, and no Word-native embedded data source. F-290 left the incomplete-owner inventory. Delivery stays in progress and the plan stays approved. Review 3 is clean. Final debt remains the hash harness, the installed full suite, stubtest, and native Word acceptance owned by the other form stories in the integrated union.

## Readiness

Status stays approved. Delivery stays in progress. DOCX-057 is reconciled to the bounded package operation with owner `-`. Local native and Python package checks are recorded in `.claude/scratch/F-290-progress.md`. The current extension SHA-256 is `2d1656c17a26d4d84247888731be7f78b9cafd45d71d82c6d51dd1dff3ac0fb4`. Review 3 is `.claude/reviews/F-290-whole-feature-pass-3.md` with 0 defects, 0 smells, and 0 nitpicks. Final debt remains the hash harness, the installed full suite, stubtest, and native Word acceptance owned by the other form stories in the integrated union.

## Open questions

| ID | Affected F-IDs and A-IDs | Decision and recommendation | Status and answer/source |
|---|---|---|---|
| Q1 | F-290 A1 through A5. Shared with F-288 and F-289 | Historical recommendation: native Rust package API only. Python methods need exact signatures and tests in a revision before approval. | Answered 2026-10-23 by user answer. Rust and Python new APIs. WASM and CLI gain no new authoring methods. Existing consumers retain preservation and current-value behavior. The exact Python surface is now recorded in `## Python API contract`. Codex root later approved this contract on 2026-10-23. This row keeps the original answer. |
| Q7 | F-290 A1, A2, A3, A4, and A5 | Historical recommendation: an embedded table in an existing custom XML store with schema `urn:rdocx:mail-merge-data:v1`, plus standards-compliant external connector metadata, offline by default, no network, filesystem, or SQL. `CallerSupplied` is the only path for caller records. Distinguish ODSO metadata from embedded rows. Alternative Word-native embedded datasource or automatic connectors needs a compatibility investigation first. Nested historical recommendation: `Offline` plus `Some(records)` is `Err` rather than a diagnostic. | Answered 2026-10-23 by user answer. Offline rdocx table plus Word metadata. Versioned custom XML. No automatic retrieval, file, HTTP, or SQL. Callers supply external records. Offline plus Some external records is Err. Do not pretend a Word-native embedded source. |
| Q8 | F-290 A2 and A5 | Historical recommendation: a pre-allocation cap of 16 MiB encoded owned XML, 100000 records, and 1024 columns for the owned mail-merge payload. Alternative: the user supplies different limits. These numbers are not a measured pass. Broader package limits stay as they are. | Answered 2026-10-23 by user answer. Proposed payload caps: 16 MiB encoded XML, 100000 records, and 1024 columns. Not a measured benchmark. Reconciliation the same day: those caps apply only to owned embedded table XML, including markup and escapes. |
| Q16 | F-290 A1 and A7 | Author Word-compatible canonical slots. | Answered 2026-10-23 by user answer, after the previous contract pass. The chosen text is "Author Word-compatible canonical slots (Recommended)". Primary source is MS-OI29500 2.1.369 at `https://learn.microsoft.com/en-us/openspecs/office_standards/ms-oi29500/df0bbd4b-c5e4-4836-a65c-f956185a0f8d`. Word ignores `mappedName` and interprets `fieldMapData` by position. The slot count is only direct namespace-qualified Word `fieldMapData` children of the qualified ODSO. The 30 role ids and `w:type` `dbColumn` or `null` rules are in Approach. Unknown opaque XML stays verbatim, or assignment returns `Err` before mutation. This is not a Word execution promise. Codex root later approved this contract on 2026-10-23. This row keeps the original answer. |

Q1, Q7, Q8, and Q16 are the only open-question rows of this story. Q2, Q3, Q4, Q5, Q6, and Q9 through Q15 are not decisions of this story. Q15 is answered on F-288 and F-289 and does not add a mail-merge default type here. Q9 through Q14 are answered on F-X264 and F-X267.

## Contract closure

Result: user answers recorded. Codex root semantic validation on 2026-10-23 approved this contract at source `a57be365`. Q16 keeps the 30 Word slots, the source graph, the recipient maximum of one, and the package lifecycle. Local implementation of that surface now exists. Native Word qualification and the final gates stay pending. This approval does not mark the story completed.

Remaining decisions: Q1, Q7, Q8, and Q16 are answered. Those four are the only question rows on F-290. Q2, Q3, Q4, Q5, Q6, and Q9 through Q15 stay on their own stories. Q15 does not change this story. Q16 provenance is the 2026-10-23 user choice "Author Word-compatible canonical slots (Recommended)", after the previous contract pass, with MS-OI29500 2.1.369 as the primary source. No blocked product choice remains. Local implementation evidence exists. Final union evidence stays pending.

Blockers before approval:

- Q1 selects Rust and Python. The signatures in `## Python API contract` are the approved surface and are now bound in the existing Python files. The expanded 12 to 17 developer-day figure is an explicit planning estimate, not a measurement and not a capacity promise.
- Q7 selects versioned custom XML plus Word metadata. It is not a Word-native embedded source and it does not fetch.
- Q8 selects 16 MiB encoded XML, 100000 records, and 1024 columns for owned embedded table XML only. Those caps are not a measured benchmark. They do not bound caller-supplied external rows or mapping and recipient metadata.
- Q16 selects the 30 canonical Word role slots. `mapped_name` stays a string and must be one of those ids. Serialization uses position. The count is only direct namespace-qualified Word `fieldMapData` children. Query omits null padding. There is no new public enum. There is no new Word oracle.
- `MailMergePackageDiagnosticKind` is `ExternalSourceBlocked`, `ExternalSourceUnavailable`, and `UnsupportedSource`. Ambiguous ownership is `Err` and has no diagnostic variant. `set_mail_merge_package` returns `Err` for an unsupported new URI or settings value. Only prepare returns diagnostics.
- `Offline` plus supplied external records is `Err` under the Q7 answer.
- `prepare_mail_merge_data` must not grow a recipient-filter algorithm to satisfy A3. The fail-closed diagnostic is the contract.
- `CT_MailMerge` child order reuses existing `MAIL_MERGE_ORDER`. ODSO child order and recipient order are work to add in `settings.rs`. This draft does not paste a second `CT_MailMerge` sequence and does not claim Word confirmed absent-default or order behaviour.
- `mail_merge_package` returns `Ok(None)` only when no recognized owned configuration is present. Opaque residual XML is `None`. Recognized settings with a missing required source are `Err`. Tests must show both, plus a second remove of `false` and an absent-package prepare error.

Operations: `set_mail_merge_package` proves A1, A2, and A5. `mail_merge_package` proves A1 and A4. `remove_mail_merge_package` proves A5. `prepare_mail_merge_data` proves A3. Existing `set_mail_merge_settings` proves A4. Existing `mail_merge` proves A6 when given prepared rows. No raw XML public API is added. Typed records are serialized and escaped internally.

Projection inventory: no recognized owned configuration, opaque-only residual, external URI, different `dataSource` and `src` URIs, embedded table including zero rows, mappings present or empty, recipients present or empty, unknown sibling bytes, unknown raw mappedName, a structurally valid unrepresentable mapping, and recognized settings whose required source is missing. Read, preserve, assign, and clear are the four methods plus the narrow setter. Read returns logical mappings in canonical role order and keeps a wider imported `data_type` string. Read returns `None` when no recognized owned configuration remains. Preserve keeps unknown mappedName, opaque extensions, `headerSource`, and other schemes without mutation. Assign canonicalizes owned known mapping values into the 30 direct qualified slots and returns `Err` before mutation when opaque XML cannot keep its position and owner. Clear removes the known configuration under the prune rule and does not invent a diagnostic. Copy-on-write applies to embedded custom XML stores under F-287 GUID identity. A recipient part is not copied into a second part. Removal prunes only provably unshared owned parts and preserves known shared and unowned parts. Source-shaped metadata that cannot be cleared without loss, and a truly ambiguous graph, are `Err` before mutation. These boundaries are preservation rules. They do not add a user question id. API shapes stay the signatures in Approach.

## Internal shared conversion

This section records the internal implementation used by the native encoder and by the Python factories. It does not add a public method, a Rust carrier, or a Python class. The stable surface stays four `Document` methods, eight Rust carriers, and seven Python classes.

`#[doc(hidden)] pub struct MailMergeTableBudget` lives in `crates/rdocx/src/field.rs` and is reexported from `crates/rdocx/src/lib.rs`. It has `new`, `Default`, `add_column(&str) -> Result<()>`, `begin_record() -> Result<()>`, `add_cell(&str) -> Result<()>`, `end_record() -> Result<()>`, and `finish(&self) -> Result<usize>`. The struct owns only an encoded counter, a column counter, a row counter, an open-cell counter, a `HashSet<String>` of column names, and a private phase. It has no `Document` field and no retained package state. Each `&str` borrow lasts for that call only. `finish` borrows `self` and returns the encoded total.

`add_column` runs only before the first record. It rejects an empty name, a duplicate borrowed name, and a 1025th column. The duplicate check uses the borrowed name. The owned name is inserted only after the encoded increment fits. `begin_record` rejects zero columns and a 100001st row. The first record counts `</columns><records><record>`. Later records count `<record>`. `add_cell` counts `<cell>`, the escaped text, and `</cell>` only while a row is open and the cell count is still short of the column count. `end_record` requires that exact cell count and then counts `</record>`. `finish` rejects zero columns and an open row. Zero rows are valid. With rows, `finish` adds `</records></mergeData>`. With zero rows it adds `</columns><records></records></mergeData>`. Every increment uses the existing checked add and escaped-length helpers and refuses a projected total above 16 MiB, including the closing markup still required for that phase. The native `encode_merge_table` preflight is this budget. The returned `Vec` capacity is `finish`. The written bytes stay the existing schema and must equal that total. There is no whole-table `Vec` before the count and no second walk that re-encodes an old prefix. The later Python `MailMergePackageSource.embedded` factory is the other consumer. It checks `Sequence` ABC, uses the counted length before allocation, and counts each borrowed UTF-8 string before cloning it. This task does not add that class.

`#[doc(hidden)] pub fn validate_mail_merge_external_uri(uri: &str) -> Result<()>` is the existing new-assignment validator, renamed in place in `crates/rdocx/src/field.rs` and reexported from `crates/rdocx/src/lib.rs`. There is no forwarding wrapper. It runs the strict XML 1.0 character rejection in the `Error::Oxml` family before the file or https `Error::Other`. Callers now are the external arm of `set_mail_merge_package`. The later Python external factory is the other caller. Imported package snapshots wrap the raw URI and do not call this function. Native default settings stay the existing narrow set. Imported preservation and the global one-recipient-part rule stay as already implemented.
