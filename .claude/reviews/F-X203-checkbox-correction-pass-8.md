# F-X203 qualified checkbox correction, pass 8

**Reviewed**: Immutable-checkbox-correction-8, three existing complete before/after carriers. Incremental diff408 additions/31 deletions. Bounded native checkbox model, staged setter and deciding controls only.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Bound evidence

Receipt SHA `a846f30709347e428020bb814debb4397d4eb6344398e6c7f7ed884297082522` and all23 declared artifact hashes authenticated. Diff SHA `55cf4101603abbfd756c91adcc2292ae504bcaaa67b3155b5b6ebd11645ebf1d`. All three preimages match genuine Base2 map records, including the reconstructed test preimage. All three after-images match full3111 GREEN5 source map `8e9d388ee243fd98eeaced8a6a7b8e4ae2d2a308abde643a552f959a01b5ccb4`. Base2 map SHA `b13e4079655f191a666a0d265620624f64b9f002c9c0436c82649d45749defda`.

Paths below are relative to `/private/tmp/rdocx-f-x203-codex/.claude/scratch/F-X203-start/immutable-checkbox-correction-8/after/`.

## Defects

### D1, a valid default-namespace Word2010 checkbox is not selected as a checkbox

`crates/rdocx-oxml/src/content_control.rs:1768`
`crates/rdocx/src/content_control.rs:79`

is_supported_type_element explicitly refuses a checkbox QName without a colon, before consulting its namespace bindings. A valid imported `<checkbox xmlns="http://schemas.microsoft.com/office/word/2010/wordml">` therefore remains untyped even when checked, checkedState and uncheckedState are correctly Word2010-qualified. This is an expanded-name-equivalent spelling of the supported prefixed checkbox. Selecting its tag and setting true takes the ordinary ControlValue path, writes literal true and omits the selected glyph/font pair. Its checkbox payload is left unchanged.

The new namespace-aware checkbox_child helper itself handles default namespaces, but root admission prevents reaching it. Existing controls cover prefixed aliases and rebinding, not this default-namespace owner. Resolve the requested type by the actual in-scope Word2010 namespace for both prefixed and unprefixed element names, while keeping foreign/Word2012 lookalikes untyped. Add a real default-owner deciding case that checks selected glyph/font, checked value, foreign attributes and repeat/save/reopen. This is an uncovered legal spelling of the approved qualified checkbox contract, not a demand to admit unrelated foreign types.

## Smells

0.

## Nitpicks

0.

## Not found

Correctness outside D1: zero additional findings. The selected glyph/font pair is prepared before mutation, with D800 rejected as non-scalar and U0000 rejected by XML-character admission. Selected state fonts populate all direct script-family slots while theme references are cleared, preserving other run properties. Canonical bound boolean and display glyph are distinct values.

Contract outside D1: zero additional findings. All selected controls are preflighted before staged document/package work. Late malformed state or custom-XML failures cannot publish earlier selected display changes. Unsupported picture/group kinds refuse. Ordinary type names and other model properties retain their previous writer path.

Panics: zero new findings in the bounded validated paths. Type payload and checkbox properties are proven before their guarded access. Raw attribute positions come from the same retained source buffer that is subsequently patched.

OOXML outside D1: zero additional findings. Qualified direct child and attribute selection uses actual Word2010 URI, rejects duplicates, and preserves foreign/rebound child bytes. Existing checked value edits splice only the owned lexical value. Missing owned value receives a fresh locally declared attribute prefix avoiding inherited bindings and existing raw occurrences. A missing checked child has its own local declaration. Raw sibling order and foreign attributes remain intact.

Tests: zero additional findings outside D1. Custom checked/unchecked glyphs and distinct fonts, repeated no-op source equality, late D800/U0000 atomic refusal, aliases/local rebinding, foreign type distinction, missing-value prefix collision and reopen are meaningful controls. D1 is not covered by these prefixed inputs. Physical font paint remains a later gate.

Structure: zero findings. Existing three carriers contain concrete staging/data structures and one namespace-aware source probe, with no new module, trait, dependency or public abstraction.

## Actual evidence and limits

Copied runtime receipts were independently compared byte-for-byte with their retained original attempt receipts. Base2 receipt SHA `9ecc85f7b7c2199f9000f9c39e3a9a81bcebd9cdcd4131113884eafb95305bac` records the genuine two deciding failures,0PASS/2FAIL/0IGNORE. The preceding invalid font_h_ansi fixture compiler failure is preserved and receives no regression credit.

GREEN4 receipt SHA `9149c0194105740778bbeecf29d350bb95b97e7e0f1769ee840a37e638b5d5d9` records native/binding check0, three native checkbox controls PASS and existing OXML model9PASS. GREEN5 receipt SHA `e3a8be911d573dbab74e3bab89550653201d76eea4fffba00fa97d8b64b20f38` records check0 and the new missing-value collision/repeat/reopen control1PASS. Source-after/cleanup guards are true. Earlier successes remain qualified prior evidence for unchanged branches, not fresh GREEN5 execution of all tests. The intermediate ordinary-model failure8PASS/1FAIL and previous compile attempts remain retained. Generic historical wrapper labels do not supersede actual argv/results.

No installed, selected physical-font paint, full story, corpus, hash or whole-feature completion credit is granted. No worker source, assertions or runtime state were changed by the reviewer.
