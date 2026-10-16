# F-X249, correctness, pass 1

**Reviewed**: Frozen nine-file working diff on work/f-x249-codex against 43c54fb01714f28d679085ff927d5994b143b525. 1005 insertions and 45 deletions. All nine source/test/HLD digests match /private/tmp/S95-F-X249-worker-1/frozen-source.json before and after inspection. The approved plan, cited ownership/binding/testing sections and canonical review rules were read.
**Verdict**: 3 defects, 0 smells, 0 nitpicks.

## Defects

### D1, Locally encoded namespace declarations hide selected Word metadata

`crates/rdocx-oxml/src/content_control.rs:449`
`crates/rdocx-oxml/src/content_control.rs:469`

The new value_children reader compares NsReader resolver URI bytes directly with W_NS for both metadata elements and attributes. quick-xml retains lexical namespace declaration references in those resolver bytes. The inherited wrapper supplies normalized semantic bindings, but a declaration on a retained child overrides it with fresh lexical bytes. For example, this direct date child is semantically qualified Word metadata:

```xml
<a:dateFormat xmlns:a="http://schemas.openxmlformats.org/wordprocessingml/2006/mai&#110;" a:val="yyyy-MM-dd"/>
```

The raw comparison classifies it as foreign, so an otherwise supported selected date refuses with a missing/unsupported format. A Word dateFormat with only its val attribute bound through such a local alias similarly loses its value. Locally declared listItem/value/displayText aliases have the same issue. This contradicts qualified alias handling and exactly-once namespace normalization. Normalize the actual resolver URI once before comparison, preserving shadow/default behavior. Add child-local element and attribute declaration controls, plus a literal ampersand-reference lookalike that must remain foreign after one decode. Do not repair this by normalizing or rewriting unselected retained source.

### D2, Conflicting supported non-Word type markers bypass selected-value admission

`crates/rdocx-oxml/src/content_control.rs:433`
`crates/rdocx-oxml/src/content_control.rs:776`

The new conflict scan checks only is_word_element followed by the local SdtType lookup. The actual parser uses is_supported_type_element, which also admits the qualified Word2010 checkbox and Word2012 repeating-section markers. With a selected date or dropdown first and a second qualified w14:checkbox in sdtPr, parse_modelled retains the second type as a Raw slot because control_type is already set. value_children then ignores that slot because its namespace is not Word main. A fill succeeds and changes the selected date/list while retaining a conflicting supported type marker, instead of refusing ambiguous selected type metadata. The same inconsistency applies to the supported w15 markers. Use the actual qualified supported-type admission rule for conflict detection and add atomic refusal controls for these supported cross-namespace conflicts, with foreign same-name controls still preserved. This is bounded selected-edit validation, not a request to reject such imports globally.

### D3, Selected repeating-section type markers are rewritten into the Word namespace

`crates/rdocx-oxml/src/content_control.rs:324`
`crates/rdocx-oxml/src/content_control.rs:338`

The type-name correction runs for every retained type except CheckBox and assumes the correct type namespace is Word main. RepeatingSection and RepeatingSectionItem are supported extension types whose canonical names are w15:repeatingSection and w15:repeatingSectionItem. For a tagged control carrying a legitimate w15 marker and a typed paragraph, the existing text-value route prepares successfully. The new metadata call finds that its type name is not a Word-main element and replaces it with the available Word prefix, such as w:repeatingSection. Serialization then emits a different expanded type name. This changes valid unselected kind semantics during an ordinary selected text edit and violates producer type preservation. Restrict the correction to types actually defined in Word main or resolve the actual type namespace. Add selected extension-kind controls that verify their expanded marker identity survives, without changing the existing checkbox or other unsupported-kind contracts.

## Smells

None found.

## Nitpicks

None found.

## Not found

- Correctness outside D1/D2/D3: no additional finding. Finite civil-date validation covers Gregorian boundaries and canonical selected midnight metadata. Dropdown matching distinguishes absent and empty attributes and refuses conflicting mappings. Metadata updates and bound XML edits stage before native publication.
- Contract outside D1/D2/D3: no additional finding. Individual missing selectors use KeyError. Only selected InvalidValue maps locally to ValueError, leaving bulk and unrelated error families unchanged. Native counts, bulk missing-key/tag-first behavior, strict fill rules and conservative individual publication remain distinct.
- Panics: no finding. Civil-date slicing follows complete ASCII shape checks. The public fullDate path checks its fixed-length ASCII suffix before splitting the ten-byte date prefix. Existing type-state invariants justify the selected mutable payload accesses. No new untrusted index or unchecked arithmetic issue was found.
- OOXML outside D1/D2/D3: no additional finding. Qualified placeholder removal preserves foreign/raw slots and order. Existing fullDate aliases retain their qualified attribute identity. Parser context is stored separately from emitted attributes. No global unselected namespace declaration injection is introduced.
- Tests: no additional finding beyond the two missing cases above. Existing new controls cover finite dates/mappings, value/display ambiguity, metadata-only mutation, held-owner/refusal/no-op behavior, signatures, late binding failure and CLI no-output behavior. Changed F-X241 expectations are limited to the approved Two display and assigned October fullDate, retaining checkbox, legacy, list choices and lastValue checks.
- Structure: no finding. Two additive concrete OXML methods and private context/value records fit the approved boundary. No public field, module, dependency, trait or generic is introduced.

## Evidence boundary

The native seal hash matches c28078a4b6a4ab976c17090374f8da25c6743721068dee731a053660bfde884a. The documented missing-mapping and value-only dropdown differences from Mac Word are intentional normative decisions. Native foreign-property losses do not authorize product losses. Preserving dropdown lastValue is explicitly approved, so this review does not infer full native metadata parity.

No build, test, cache, source, progress or private evidence file was written or executed by the reviewer. The worker's recorded focused passes precede some final added assertions, which still need actual execution. Remaining scoped riders, fresh installed/typing/stub checks, original reporter replay, independent typed XML observations and genuine publication remain required. Only this review file was written. Return to the orchestrator for separate remediation and a new independent review pass.
