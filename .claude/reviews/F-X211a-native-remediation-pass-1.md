# F-X211a, native remediation, pass 1

**Reviewed**: Frozen native source84 to source102 batch in `work/f-x211a-codex`, ten existing carriers, 855 additions and210 deletions. Capsule SHA-256 `66bcbc2af72e52091ccf673a5e58aa5d2921f2863cf59b374dc2c4e905663c65`, patch `bd35a79b443d9f1d3a3295637a0d2d14555d4ad8489fd5213cd8834f18b2ab0b`, source map `32e2fd3ef3c040af5d780aec1ec8f66d402e3abe37bf45f4e8161f710346500a`. All3228 current source entries match the frozen map.

**Verdict**: Three defects, zero smells, zero nitpicks. The batch cannot complete. Actual source102 OXML execution is704 passed and nine failed, not a passing native checkpoint.

## Defects

### D1, raw child closure adds declarations that change retained source and fixed points

`crates/rdocx-oxml/src/text.rs:3313`

The new metadata collection records every inherited prefix used by a raw child, including canonical w that the generated owner already supplies. The emitter at `crates/rdocx-oxml/src/text.rs:3642` then closes every such raw child independently. For an inherited canonical `<w:separator/>`, the first parse gains a metadata slot and the first save changes the raw child to `<w:separator xmlns:w="..."/>`. Reopening sees the declaration locally and no longer retains the metadata slot. This breaks the existing model fixed point and exposed raw slot counts. Other inherited aliases already retained by the enclosing source owner also gain redundant declarations inside otherwise unchanged opaque subtrees.

Actual source102 results demonstrate eight related preservation failures, including `footnotes::tests::a_separator_definition_survives_open_and_save` at `crates/rdocx-oxml/src/footnotes.rs:505`, the checked raw payload count at `crates/rdocx-oxml/src/text.rs:14657`, watermark source preservation at `crates/rdocx-oxml/src/header_footer.rs:1562`, and exact encoded namespace retention at `crates/rdocx-oxml/src/document.rs:4576`. Retain actual lost namespace authority without redundantly closing already supplied bindings or relocating declarations through unchanged opaque payload. The original foreign-w/xsi authority regression must remain a meaningful positive control.

### D2, namespace discovery decodes arbitrary opaque attribute values

`crates/rdocx-oxml/src/text.rs:13612`

The walker decodes every prefixed attribute value before determining whether that expanded attribute contains namespace-dependent values. Source102 newly invokes this walker over every retained run child at `crates/rdocx-oxml/src/text.rs:3315`. An opaque legacy form leaf with `w:val="&undefined;"` now rejects the whole paragraph while collecting namespace metadata, before the established typed legacy-form owner can expose and reject its malformed value through its own validation API. The actual test at `crates/rdocx-oxml/src/text.rs:18420` fails during paragraph parsing rather than reaching `validate_legacy_form_owner().is_err()`.

Namespace discovery must not become a general value parser for retained payload. Decode only the supported namespace-dependent expanded attributes when finding QName or prefix-list dependencies, and retain the existing explicit malformed-value validation boundary. Do not weaken the legacy-form control or suppress the original source102 failure.

### D3, per-child metadata extraction makes flat opaque serialization quadratic

`crates/rdocx-oxml/src/text.rs:3634`

Every call to `write_run_raw_source` scans all raw slots to locate root metadata. `write_run_raw_boundary` at `crates/rdocx-oxml/src/text.rs:3654` calls it once per admitted raw child. A valid run with N flat opaque children and no modeled content therefore performs N scans of N slots, including runs with no metadata at all. The former single boundary pass emitted those children once. This new quadratic work is unrelated to the bounded recursive owner limits and contradicts Issue331's resource admission goal.

Recover the actual retained bindings once at the concrete emission or admission operation boundary and reuse that finite binding slice for each raw child. No larger quota, timeout, global cache or new ownership type is needed. This finding follows directly from the loop structure. The reviewer did not execute a performance probe.

## Smells

Zero additional findings.

## Nitpicks

Zero findings.

## Not found

- Consumed-reader namespace and cursor proof: zero additional findings. `crates/rdocx-oxml/src/document.rs:666` clones the actual quick-xml Reader with its consumed QName stack, restores strict end-name checking, and preserves the original cursor. Concrete qualified routes still prove the expected URI. Only the two legacy consumed p/r APIs use the supplied operation role rather than invent an owning namespace binding. Positive sibling/cursor controls and mismatched-close refusals distinguish this behavior.
- Fallback classification and resource proof: zero additional findings. `crates/rdocx-oxml/src/text.rs:8946` confines unbound local-name run promotion to the legacy consumed operation without actual Word scope. Full-document and actual-start routes retain unbound runs as opaque source. Preflight at `crates/rdocx-oxml/src/document.rs:376` charges the same actual legacy fallback route, respects explicit foreign shadowing and retains the existing16 typed-box,32 table and256 control/weighted boundaries. Tests distinguish shallow actual typed construction,16 versus17 admission, qualified actual source and opaque full-document save/reopen.
- Marker identity: zero additional findings. `crates/rdocx/src/document.rs:3218` limits a/pic relocation to an actual Word drawing wrapper containing one actual qualified wp:inline/anchor child, with namespace-only wrapper attributes and whitespace outside that child. Actual a/pic URI facts remain significant. Child marker eligibility is tied to the same qualified wrapper proof at line3539. Missing, foreign, default-shadowed and opaque wrapper cases remain refusals. No nth-match, value-match or generic semantic fallback was introduced.
- Generated run and cached-field namespace authority: zero additional findings beyond D1 and D2. `crates/rdocx-oxml/src/text.rs:3609` separates actual root attribute dependencies from raw-child storage metadata so a foreign w binding does not rename the generated canonical run. Cached-field admission at line2062 uses the same per-slot closure as emission, preserving foreign field lookalikes instead of recognizing them as owned delimiters. Focused source102 positive controls pass.
- Panics: zero additional findings. No new production unwrap or expect was found in the batch. Reader offsets are computed from the cloned cursor over the remaining slice. Marker parser errors propagate before owner publication.
- Structure and test isolation: zero additional findings beyond D3. The batch uses existing carriers and concrete helpers, with no new product type, trait, dependency, module or file. Three parameterized manual diagnostics receive precise ignore reasons. The former invalid raw-snapshot success test is removed while its original body and failure remain retained. The text-body oracle now checks the actual authoritative typed paragraph instead of the retired duplicate projection.

## Actual evidence and unfinished acceptance

Read the canonical workflow and microscope contract, child and parent plans, current in-progress ledgers, progress, prior public foundation and native slot/resource reviews, actual84 preimages, and the frozen ten-carrier patch. Authenticated terminal receipt `f670447d58cf6daab99c6de8fe345b246b33af28b8500b225a227d3708db2c19` and every listed raw log hash. All owned groups are naturally terminal and reaped. Compilation succeeded. The three selected raw/cache authority, original namespace authority and drawing-marker controls passed. Full OXML failed with704 actual passes and nine failures, log `f0399dcda76f9da5b3f1f9fe6e9b7c0ab1a4ad921bfe7638971c12914cb344b5`. Full Word unit/regression rows were not executed after that failure.

No source changes, fixes, tests, Cargo, CP, Office, Git operations or final full-sprint qualification were performed by this review. Current wheel/install, affected archive/HLD composition, remaining scoped gates and cumulative feature review remain due after remediation. Original failed batches remain failures. Return control to the orchestrator for a separate remediation phase.
