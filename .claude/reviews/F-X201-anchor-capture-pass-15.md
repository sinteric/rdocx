# F-X201, anchor capture correction, pass 15

**Reviewed**: One DrawingML capture correction and its inline control, plus the four-name stub export amendment. No whole-feature re-audit.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Authority

Immutable receipt `/private/tmp/S91-F-X201-anchor-capture-correction-r1/receipt.json` SHA256 `87cd3c3174cef4ae960ac2ca41a31c0e40443e9e54f1b2cd32a6d01c8c5edb43`. All eight capsule references and five referenced authority receipts authenticate. Drawing before `333cf36d14aad36ed683855c3d4a8059d8fb21c70d72c79d24186efd926d8932`, after `a2ab11b5f936e74302a533bf08bd7bc636190dc48bcd715cb753e5412c7c6741`, exact patch `570defc59fc6bf79205505450a9e0fd03573490ac675366a4ae8cfd36b0df8f5`.

Actual first model and capture attempts remain failed. The corrected full OXML receipt `98c117e90a8f539821388dd2f9ecdd91c851ac7d99acdbeaf374e86bd36f11b3` and full layout receipt `47a8dcee1d8725790282c37c3894b41ed33a3e9a649d9482275473c907ac4b67` record successful children. Retained observations are 671 OXML plus one doc test and 332 layout plus one doc test. Installed typing/export amendment receipt `325795a0486d7a689e2279ae531182b266b9654f7905bf7e82713ec43bd2c259` records eight successful rows. No failed attempt is relabelled.

## Defects

### D1, literal prefix search does not prove an opaque QName prefix is unused

`/private/tmp/S91-F-X201-anchor-capture-correction-r1/after.rs:1877` and `:1880`, corresponding to `crates/rdocx-oxml/src/drawing.rs`.

The new capture filter removes a nondefault inherited binding when its prefix bytes do not occur literally anywhere in the physical anchor. XML character references in attribute values are valid and decode before a QName-valued attribute is interpreted. An opaque attribute `opaque="un&#117;sed:fact"` therefore references `unused:fact`, despite the raw owner containing no literal `unused` bytes. Changing the inherited binding from `xmlns:unused="urn:first"` to `xmlns:unused="urn:second"` changes its QName meaning. Both bindings are currently discarded, leaving identical raw bytes and identical derived anchor equality/projection environments.

A bounded standard-library XML parse verified the decoded value and absence of the literal prefix in that exact input. No product runtime was run. The current inline control at `after.rs:2792` covers only the literal `unused:fact` spelling and therefore misses this case.

This is not a request to weaken derived equality or normalize opaque source. Preserve physical raw bytes and retain the binding whenever decoded opaque values or text might reference it. A conservative filter can keep uncertain environments rather than claim their absence. Add the entity-spelled value discriminator and retain default, literal used foreign, unused scope, and raw-byte controls.

## Smells

Zero.

## Nitpicks

Zero.

## Not found

Correctness outside D1: zero. The patch leaves derived PartialEq unchanged and keeps physical raw XML exact. Default bindings are always retained. Literal referenced namespaces and foreign binding differences have meaningful controls. The existing qualified shape/picture projection paths are not broadened. Panics: zero new unchecked paths. Contract and OOXML: D1 is the sole incomplete opaque-environment preservation proof. Tests: the actual successful suites remain genuine but do not include the identified entity spelling. Structure: zero new module, trait, dependency or framework.

The stub after SHA256 `fa71657aedf98a72f99f9fb79a68c330a7bd404a4c92f7c4c7daff7abfd2bb13` was independently compared with capsule before `79b4297ac3a3f97e3b847a78d7ac004e8084979394d0ecccd33fcd55c41c322d`. It adds exactly FootnoteEndnote, HeaderFooter, SectionHandle and Settings to existing __all__, with no other stub change. Zero findings in that export amendment. Successful installed authority is distinct from the capture source amendment and does not grant future wheel qualification.

## Limits

The newly reported external watermark capability control is not current production and is excluded. No opacity acceptance or whole-feature completion is inferred from it. No source, index, runtime, environment, baseline or native UI mutation occurred. Only this external review artifact was written for the worker to copy under its exclusive lease.
