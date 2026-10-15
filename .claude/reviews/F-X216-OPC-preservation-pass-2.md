# F-X216, OPC import preservation, pass 2

**Reviewed**: frozen relationship.rs eeee39c6c6d9ae113ee65c088730ef525a75d4501aff4a27d3a7cad86a522f20 against reviewed predecessor98778e1a. Only the existing OPC helper and its defining controls are covered.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D2, Expanded attribute identity still compares non-normalized namespace bytes

`crates/oxml-opc/src/relationship.rs:674` through `:684` and `:376`

The new identity copies reader.resolver().resolve_attribute's bound namespace bytes directly into a String. quick-xml0.41's Namespace::as_ref returns non-normalized declaration values, explicitly documented at its src/name.rs:339 through :359. Entity references are not expanded in these bytes, so equal XML namespace identities can still compare unequal.

A precise valid witness is a left Relationships root with xmlns:z="urn:metadata" and z:mark="left", and a right root with xmlns:y="urn:met&#97;data" and y:mark="right". Both roots use the ordinary package relationships default namespace. The two mark attributes have the same actual expanded namespace/local name after XML attribute normalization. Current expanded_name stores urn:metadata versus urn:met&#97;data, so merge_for_import appends both instead of refusing their conflicting values. The merged root therefore contains duplicate expanded attributes. Equal mark values likewise emit two attributes rather than one.

Normalize/decode the actual namespace declaration URI before constructing expanded identities, preserving original lexical declaration bytes for output. Add exact encoded-versus-literal alias conflict and equality controls, including duplicate expanded input rejection. Do not approximate with raw-name matching or broadly rewrite source XML.

## Prior D1 resolved

The BOM repair at crates/oxml-opc/src/relationship.rs:624 through :640 parses the BOM-free slice with explicit original origin. Recorded start/end, attribute, child, close and record offsets consistently translate into original input coordinates. Rebind and either-side merge preserve the original left BOM and untouched opaque payload. No further bounded BOM defect found.

## Actual evidence

Authenticated successor receipt1ffb85bf6d62ef48358fdabc94174aff7996e55401e6121d1653e9687f49dcea contains eight actual exit0 qualified safe rows: strict OPC Clippy, release compilation and six exact controls. All eight raw log hashes match. The ordinary unencoded alias test and all four original helper controls pass, but they do not cover encoded equivalent namespace declarations.

Authenticated predecessor receipt00df56d5f1523e3b9b0bd28b6acf67c76387aec89438e9984dc978a5ca15f90c preserves valid compilation and two genuine defining exit101 failures with safe cleanup. Its historical binary was overwritten before retention. No retained-predecessor binary claim is made.

## Not found and scope

No additional bounded defect found in direct qualified projection, explicit ID maps, source target/mode preservation, removed/added records, opaque root/record payload, staged rollback, namespace closure or concrete helper structure. Smells:0. Nitpicks:0. Legacy direct items mutation and rebuilding serializer remain outside this helper pass, as recorded in pass1. No whole-import or F216 completion verdict is issued.

Only this canonical review file was written. No test, build, probe, application, Git operation or source correction ran. The frozen worker carrier hash matched before and after review. Review ended and returned control for distinct remediation.
