# F-X204, Shared CLI one-shot editing commands

**Status**: completed
**Sprint**: S91
**Size**: L
**Depends on**: F-X202, F-X203, F-X205, F-X207
**Implementation**: claimed by codex at 9ef5ec193c7ebb5daa49acf1144dd2fa088d441f. Completed provider prefix 4bb17d5fe0dc6187553fa5f8c63aba224f5d621c retains that ancestry. Native, Python and CLI implementation is complete, independently reviewed and scoped verified.

## Problem

Issue307, captured in S91-intake-issues.json on 2026-10-09, requests
scriptable one-shot editing and reports through the two CLIs. Existing Word
replace is single-pair at rdocx-cli/src/commands.rs:1948, even though native
try_replace_all_expected already supplies ordered transactional pairs at
rdocx/src/document.rs:27163 and Python replace_all exposes them at
rdocx-py/src/document.rs:3589. PowerPoint replace is likewise single-pair at
rpptx-cli/src/commands.rs:527 and has no native counted batch counterpart.

The other omissions are command exposure and some real shared capability
gaps. Native Word field operations are separate general and layout-backed
updates. ImageInfo inventory is body-only, and PowerPoint's existing
text_layout excludes table cells, master shapes and SmartArt. A CLI loop
cannot silently call those incomplete surfaces complete. The native providers
must own common public operations that Python and CLI both consume.

Full acceptance requires Word multi-pair counts, regex and JSON reports,
fields update, image extraction/inventory, core/custom metadata, tagged
content-control fill and counts. PowerPoint requires slide add/duplicate/
remove/move/hide/show, notes text/file updates, fit overflow reports, core
metadata and multi-pair replacement. Each command gets an integration test,
help/reference documentation, input-overwrite refusal, nonzero refusal exits
and all-or-nothing expected counts. Footer/theme CLI work belongs to F-X208.

## Spec reference

- docs/hld/10-bindings-spec.md, "CLIs", shared schema-1 reports, path saves,
  literal replacement scopes, read and mutation conventions.
- docs/hld/10-bindings-spec.md, Python API shape, counted replacements and
  field updates around lines390 through411, and presentation text layout.
- docs/hld/10-bindings-spec.md, "The invalidation problem, handled loudly".
- docs/hld/04-opc-and-packaging.md, owner relationships, preservation and
  package-class/path save contracts.
- docs/hld/06-presentationml-model.md, slide/layout/notes ownership.
- docs/hld/08-rendering-spec.md, deterministic layout and fitting facts.
- docs/hld/14-development-backlog.md, "F-X204, Complete shared CLI one-shot
  editing commands" and provider stories F-X202, F-X203, F-X205 and F-X207.

## Approach

### Layering and actual dependencies

Both CLI manifests already depend on their native family and
oxml-cli-support. Word also depends on existing XML/layout/PDF support and
serde_json. Neither depends on a Python binding. Keep that graph. A command
is a native API consumer, not a Python subprocess, embedded interpreter or
reverse Rust-to-Python edge. Python-visible means the same capability is
exposed by the installed Python API, not that Rust invokes Python.

Current backlog formal dependencies list X200, X201, X202, X203, X205, X206,
X207 and X208. The following is the proposed capability-based reduction:

| Command | Existing common API or real gap | Provider and prerequisite |
|---|---|---|
| Word literal replace map | try_replace_all_expected and Python replace_all exist | X204 consumes now, no formatting/section dependency |
| Word regex counts | Regex exists, expected per-pair report does not | X204 owns minimal shared native/Python addition |
| Word fields update | update_fields and update_layout_backed_fields exist in native/Python | X204 common orchestration, no blanket X201 dependency |
| Word image inventory/extract | images is body-only, image_data_for_story exists | X202 common all-story picture inventory checkpoint |
| Word core metadata | Native checked core updates and PyCoreProperties exist | X204 consumes now |
| Word custom metadata and counts | Native custom properties exist, typed Python/count exposure incomplete | X203 common capability checkpoint |
| Word tagged fill | Native by-tag/binding updates exist, Python exposure belongs to X203 | X203 content-control checkpoint |
| PowerPoint slide lifecycle | Native and Python operations already exist | X204 consumes now, no X206 dependency merely for same files |
| PowerPoint notes | Native and Python set_notes_text/notes_text exist | X204 consumes now |
| PowerPoint fit | text_layout exposed but incomplete inventory and needed-scale semantics | X207 common fit/report checkpoint |
| PowerPoint core metadata | Native core properties exist, Python editing exposure belongs to X205 | X205 metadata checkpoint |
| PowerPoint literal replace map | No counted batch common method | X204 owns minimal native/Python addition |
| Footer inspection/theme commands | Separate acceptance | X208 owns, no X204 dependency |

The actual API inventory removes X200, X201, X206 and X208 as blanket
dependencies. Their file overlap is a serialization concern, not a capability
requirement. X202/X203/X205/X207 are real requirements only for the named
capability checkpoints. The final canonical plan may depend on those stories
or their formally reviewed dependency-prefix checkpoints. Do not wait for
every unrelated acceptance item before designing the already-supported
commands. Do not mark X204 complete until the full acceptance has its common
native API and installed Python evidence.

X204 owns its two CLI source files, existing integration entrypoint and README
per family, and the narrow batch/orchestration operations described here if
the provider plans assign them to X204. Shared native and Python files must
be explicitly serialized with their owners. No new module, dependency or
feature flag is proposed. No CLI-only capability is introduced.

### Command grammar and counted replacement

Preserve the existing single-pair replace options. Add --map PATH and --json.
Word also gains --regex. Single pair and map are mutually exclusive. A map
is an ordered JSON array, not an object whose key semantics obscure order:

```json
[
  {"placeholder": "{{name}}", "value": "Ada", "expect": 1},
  {"placeholder": "{{state}}", "value": "ready", "expect": 2}
]
```

Reject unknown keys, wrong types, negative/nonintegral expected counts,
missing placeholder/value and conflicting command-line --expect in map mode.
An omitted or null expect means unrestricted count. Retain duplicate
placeholders as meaningful ordered pairs. Parse and validate the whole map
before mutable access. Empty map behaviour must be documented, with the
proposed result being an empty successful report and unchanged document.
Empty literal placeholders retain each family's checked native refusal.

Exact semantics match existing Word try_replace_all_expected:

1. Apply pairs in array order to one staged candidate.
2. Pair2 searches the result of pair1, not the original input. Therefore
   A-to-B followed by B-to-C may legitimately cascade.
3. Expected count is checked on that pair's current candidate scope.
4. On any mismatch, malformed pair, parser/serialization error or invalid
   replacement, discard the entire candidate. Original native state and
   binding revision remain unchanged. CLI creates no output.
5. On success, publish once and report counts in input order. Zero is valid
   when expect=0. Preserve existing family-specific single-pair unmatched
   behaviour rather than changing it implicitly in the map extension.

Use existing Word signature:

```rust
Document::try_replace_all_expected(
    &mut self,
    pairs: &[(&str, &str, Option<usize>)],
) -> Result<Result<Vec<usize>, ReplacementCountMismatch>>;
```

Add the same semantic operation to native PowerPoint in existing lib.rs,
using a concrete family-local mismatch report/error and one staged clone.
Python Presentation.replace_all exposes ordered tuples with optional expected
count, analogous to Word, and returns ordered counts. No dependency on
rdocx's mismatch type is allowed. Use concrete types in existing files, not
a new common generic replacement engine. Each family has distinct traversal,
escaping, field and notes contracts.

### Regex scope and reports

Regex is explicitly requested under Word. Do not add a PowerPoint regex
engine, dependency or flag by interpreting its many-pairs request broadly.
Word native replace_regex at document.rs:27377 and replace_all_regex:27388
already use the regex crate. Reuse that syntax, including $1/$2 capture
replacement. Python uses the same native regex dialect, not Python re.

Word regex searches the documented literal stories except chart labels:
body/nested tables, headers/footers, footnotes/endnotes, text boxes, content
controls and tracked insertions. It matches within supported paragraph/story
segments, not across unrelated paragraphs or XML nodes. Verify the existing
implementation and retained boundaries with actual tests before promising
cross-run matching or the handling of tracked deletions, note separators and
AlternateContent copies. Do not broaden that scope incidentally.

For --regex with map, compile every pattern and validate every replacement
before staging. Add the smallest native ordered expected-regex counterpart:

```rust
Document::try_replace_all_regex_expected(
    &mut self,
    pairs: &[(&str, &str, Option<usize>)],
) -> Result<Result<Vec<usize>, ReplacementCountMismatch>>;
```

It uses the existing regex replacement logic on one candidate, returns
per-pattern counts, and preserves the literal batch's ordered/atomic
contract. The existing mismatch placeholder field contains the pattern for
this entrypoint and must be labelled correctly in reports. Expose the same
capability in Python through an additive counted method or a compatible
extension to replace_all_regex, preserving its current total-count return
unless an explicit pre-1.0 migration is approved. Do not build an unsafe CLI
loop over committed native mutations.

All reports use current oxml-cli-support::json_envelope at lib.rs:355,
schema=1. Existing inspect keys stay compatible and gain additive fields.
Replacement success contains mode, ordered pair index/placeholder/count/
expected records, total count and output path. Keep explicit null expectations.
Human output has the same facts. Diagnostic/refusal information goes to
stderr and nonzero exit. Do not print progress or prose into JSON stdout.

### Fields, images, metadata, fill and counts

Add Word fields update next to existing toc rebuild. Native field.rs:1809
update_fields(context) evaluates ordinary typed caches, including supported
DATE/SEQ/REF, while :1840 update_layout_backed_fields uses deterministic
layout for PAGE/NUMPAGES/PAGEREF and section fields. Their existing contract
preserves dynamic header/footer PAGE/NUMPAGES caches and unsupported switches.
Do not change it merely to make every cache look updated.

The one command needs one atomic operation over ordinary and layout-backed
updates, not two visible native commits with a late partial failure. Proposed
minimal common native method in existing field.rs stages once, evaluates
ordinary fields, computes layout, applies supported layout-backed caches,
then commits once. Return an ordinary update count plus the existing
LayoutBackedFieldUpdateReport, using a concrete tuple or a justified concrete
report if named fields materially help both current wrappers. Expose it in
Python. Context uses existing FieldEvaluationContext:1355. Supply dates
explicitly or define --now semantics, and preserve diagnostics and unsupported
cache facts. --json reports what was updated, not a claim of complete support.

images extract DIR and inspect pictures consume X202's native owner-aware
inventory. Existing Document.images at document.rs:28939 reports only body
drawings with relationship ID/name/description/EMU size, so it cannot produce
the requested story list alone. image_data_for_story:17879 is the common byte
accessor. Report drawing occurrence identity and story separately from its
possibly shared image part. Extract exact embedded bytes, not reencoded
rendered images, preserve known media extension/content type and deterministic
safe filenames, and avoid network fetches for linked external images.
Precompute the entire output set, validate collisions/input identity, load
all required bytes, then stage and publish together using existing support.
Decide duplicate-part extraction and unsupported/missing-media reporting
explicitly. No partial directory contents after a rejected edit.

Word meta get/set uses PyCoreProperties/native core methods already present
at document.rs:25883 through :25897 and typed custom properties at :25950
through :26041. PowerPoint core methods are lib.rs:1927/:1932, with Python
exposure supplied by X205. Parse all assignments before mutation, use native
validation and preserve unspecified fields and opaque metadata. Custom
key=value type policy must match X203's typed Python contract, rather than
inventing CLI-only untyped conversions. Title/author/subject/keywords are
explicit supported core names. Repeatable assignments must reject duplicates
or document one chosen rule. Save through the package-class-aware path API.

fill --tag NAME=VALUE consumes a tag-only common operation. Native
set_content_control_value_by_tag at content_control.rs:95 exists, but
bind_content_controls:113 falls back to aliases when a tag is absent. Do not
use that fallback for a flag explicitly named --tag. Repeated tag pairs must
stage all updates once and preserve binding custom XML/display ownership.
X203 owns Python exposure. Validate known unsupported or ambiguous control
cases and no-match policy before publishing. Do not add control creation or
tracked-edit authoring, which the provider's backlog explicitly excludes.

Inspect word/character/page counts consume X203's public facts. Existing
word_count at document.rs:29025 counts whitespace tokens in the accepted
body including table/control paragraphs. It is not an all-story locale word
counter. State its scope. Character and deterministic page counts must use
the provider's specified common API, never a CLI-only heuristic or cached
docProps count. Keep existing inspect schema and keys additive. Layout
diagnostics and unsupported stories must be reported honestly.

### PowerPoint slides, notes and fit

Native add_slide:3253, duplicate_slide:2791, remove_slide:2572 and
move_slide:2585 already have Python consumers in slide.rs:575, :688, :670
and :711. Native/Python hidden mutation exists, as does notes_text setter
slide.rs:229. Add thin command parsing and reporting around these operations.
CLI slide numbers are one-based, native indices zero-based, and conversion
must happen exactly once. Proposed --at denotes a final one-based insertion
position including append. Resolve --layout by checked numeric index or an
exact unique name, with clear unknown/ambiguous diagnostics. Add then move
must be a single staged common native operation if --at is supported, also
exposed to Python. No half-added slide on a placement failure.

Slide duplicate retains native placement after the source unless the final
CLI contract specifies otherwise. Remove/move/hide/show use native checked
operations and package lifecycle. Notes set requires exactly one of --text
and --from-file, validates UTF-8 and reads the complete file before mutation.
Do not add undocumented trimming or newline conversion. All mutations use
an output path and refuse to overwrite input.

fit is a read-only report, not an autofit mutation. It exits1 when any
reported text frame overflows,0 otherwise, and uses the shared JSON envelope.
Existing Presentation.text_layout_deterministic at lib.rs:1199 returns
TextFrameLayout with owner slide/index/id/name and actual renderer font_scale
and overflow. Its documentation explicitly excludes table cells, master
shapes and SmartArt. Current font_scale is the renderer's existing scale,
not automatically the additional scale needed to make an overflowing frame
fit. X207 owns the required common all-frame fit analysis, meaningful needed
scale and unsupported diagnostics. CLI and Python consume it. No CLI-only
binary search or fabricated scale. Preserve ownership identities, units and
deterministic font policy in the report, including cell paths if supported.

### Publication and compatibility

Use existing ensure_output_paths_allowed in oxml-cli-support:143 to reject
input identity, aliases/symlinks/hardlinks and unsafe collisions. Keep current
stage_and_publish and AtomicOutputBatch mechanisms. Mutating commands default
to refusing an existing output like current replacements. A later --force
choice must use the existing shared contract and still never replace input.
Extension-based package class, VBA refusal and signatures remain the native
path-save contract. Do not publish success reports before files are published.

## Rejected alternatives

- Invoke Python from Rust. It adds a reverse dependency/runtime requirement
  and violates thin native wrappers.
- Put regex/counting, fit solving or content-control XML only in commands.rs.
  That creates CLI-only capabilities and inconsistent Python behaviour.
- Interpret replacement maps as unordered objects. It obscures ordered
  expected-count and duplicate-pair semantics.
- Apply each pair directly to live native state. A later mismatch leaves a
  partial edit even when the CLI does not save it.
- Reencode extracted images. Extraction must retain exact embedded bytes.
- Report current text layout as every frame and current font_scale as needed
  scale. Existing documented scope and meaning do not support those claims.
- Keep eight blanket dependencies because files overlap. Explicit provider
  checkpoints and serialized ownership express the actual constraints.

## Test plan

**Test gate**: regression. Reproduce every missing operation or reported
defect through the installed public API on the claimed Base before
implementation. Exercise all Issue307 acceptance criteria through save/reopen,
native rendering where applicable, malformed/atomic refusal controls and
preserved unmodelled XML. Python requires accurate stubs, strict typing and
stubtest. CLI requires help/reference and existing integration tests. Use
deterministic fonts and pinned native oracles for output claims. Google
Docs/Slides import is best effort. Reuse prerequisite evidence and test the
integrated behaviour. Require scoped verification and independent
zero-defect, zero-smell review.

| Category | Test | Asserts |
|---|---|---|
| regression | word_replace_map_is_ordered_counted_and_atomic | Cascade, duplicate pairs, per-pair count, late mismatch, unchanged input/output |
| regression | word_regex_map_has_native_scope_and_capture_counts | Capture substitutions, cross-run controls, chart exclusion, invalid later regex and atomicity |
| regression | powerpoint_replace_map_is_ordered_and_includes_notes | Native/Python/CLI count parity, notes, late mismatch and opaque bytes |
| integration | fields_update_reports_general_and_layout_cache_results | DATE/SEQ/REF/PAGE/NUMPAGES, explicit time, retained diagnostics and late failure |
| integration | images_extract_preserves_bytes_and_reports_story_owners | Body/table/header/footer/note controls, shared media, missing/external image and safe filenames |
| integration | inspect_adds_pictures_and_public_counts_compatibly | Existing schema/keys retained, native/Python inventory and count parity |
| integration | word_meta_get_set_preserves_core_custom_types | Every requested core field, typed custom values, invalid atomic refusal |
| integration | fill_tag_updates_all_matching_controls_atomically | Tag-only selection, alias collision, bound custom XML and unsupported control refusal |
| integration | slide_add_resolves_layout_and_insertion_atomically | Name/index, first/middle/append, invalid/ambiguous layout and no half-edit |
| integration | slide_duplicate_preserves_native_slide_and_notes_ownership | Correct placement, IDs and related parts |
| integration | slide_remove_preserves_native_relationship_lifecycle | Reopen, removed/shared parts and invalid index |
| integration | slide_move_uses_one_based_cli_positions | Every boundary, no-op and rejected positions |
| integration | slide_hide_and_show_round_trip | Native/Python values, repeated no-op and opaque content |
| integration | notes_set_text_or_file_preserves_exact_text | UTF-8 multiline source, conflicting options and missing file refusal |
| integration | fit_reports_every_supported_overflow_and_needed_scale | Provider owner inventory, genuine scale, no write and exits0/1 |
| integration | powerpoint_meta_get_set_matches_python_core_properties | All requested fields and retained imported metadata |
| integration | every_new_command_has_help_and_reference_contract | Grammar, defaults, scope, output/JSON/expect and units |
| integration | new_mutations_reject_input_aliases_and_output_collisions | Same path/symlink/hardlink, existing output, late stage failure and no partial files |

Use only current CLI tests/integration.rs per family. Existing Word tests at
:287 inspect schema, :1402 replacement, :1509 namespace refusal, :1546
text-box counts, :1596 controls, :1710 notes/revisions and :3713 structured
reports are controls. Existing PowerPoint :1105 preservation, :1174 output
class, :1254 counted notes and :1523 additive inspect keys are controls.
Existing output refusal helpers :74/:105 Word and :63/:98 PowerPoint apply
to every new output command. Add no test binary.

Shared new native operations get tests in current native modules/entrypoints,
and installed Python tests go into current test_core.py and
test_documented_examples.py. Provider tests are reused with cited evidence.
Add current typing_smoke/stub changes for the new common capabilities. Verify
twice save/reopen and exact unknown XML/opaque package parts. Tests must prove
both native rollback and filesystem refusal, not just absence of output.

## HLD impact

- docs/hld/02-scope-and-non-goals.md, actual CLI and Python capabilities.
- docs/hld/03-architecture.md, only if common native operation ownership changes.
- docs/hld/04-opc-and-packaging.md, extraction/publication ownership contracts if expanded.
- docs/hld/08-rendering-spec.md, fit/report facts supplied by X207, no duplicated solver.
- docs/hld/10-bindings-spec.md, exact CLI grammar/reports, shared Python-visible operations and fields/count scopes.
- docs/hld/14-development-backlog.md, actual prerequisites and acceptance.

Help and CLI references update current main.rs clap descriptions,
crates/rdocx-cli/README.md, crates/rpptx-cli/README.md and the existing root
README CLI examples where affected. No new reference document is proposed.
Source additions earn actual archive record maintenance in the existing
README/script carriers. Root regenerates integrated measurements, not stale
worker tuples.

## Risk routing

- Published native API. Read HLD10 and structural rules, state additive
  pre-1.0 signatures and run actual archive dry-run/size checks for affected
  native/CLI crates, including required measured records.
- PyO3 exposure. Read HLD10, installed source-built parity, strict mypy and
  stubtest, documented workspace binding exclusions and WASM check.
- Parser/serializer and regex/field edits. Read HLD04/HLD06, test actual
  namespaces, schema order, unsupported switches and byte-preserved raw nodes.
- Rendering/layout for field pages, counts and fit. Read HLD08, use
  deterministic fonts and preserve provider diagnostics, with pinned oracle
  evidence for claims that affect output.
- Dependency edges. Read HLD03 and retain native-to-support edges only,
  with no Python runtime reverse dependency or new regex dependency in CLI.
- Unit conversion and index arithmetic. Read HLD01/CLAUDE, preserve
  truncation and checked one-based-to-zero-based conversion.
- External oracle. Read canonical differential-testing, pin versions and
  retain provenance. CLI parsing is not a PowerPoint open-without-repair check.

These add to normal scoped verification and independent microscope, never
replace them. No new file/module/trait/generic/feature flag is needed.

## Hash harness

Expected unchanged for new commands and additive shared operations. They do
not alter existing default output, traversal or rendering. Explicit command
edits naturally change newly generated documents, with counts and expected
bytes asserted in their own source-built regressions. Provider behaviour
changes keep their own labelled reviewed deltas. Do not attribute an
unexplained hash delta to CLI work or move the exclusive baseline.

## Implementation checklist

- [x] Review and reduce actual capability dependencies and assign native/Python provider ownership.
- [x] Capture installed/native Base failures and existing supported controls.
- [x] Finalize map grammar, exact ordered rollback semantics and JSON schema.
- [x] Add common counted batch/regex/field orchestration operations with Python exposure.
- [x] Add thin commands consuming provider image/count/control/meta/fit surfaces.
- [x] Add every command's existing-entrypoint integration test and filesystem rollback controls.
- [x] Update help, existing reference text, stubs and HLD contracts.
- [x] Run focused evidence, applicable riders, unchanged hash and scoped gate.
- [x] Obtain independent microscope over final diff including measured records.
- [x] Prepare only after the full contract is satisfied, with no worker ledger edits.

## Open questions

Resolved by the one-shot operation contracts below. Cascading ordered arrays,
repeated entries and existing parser last-key-wins behavior are explicit.
Counts/exports/picture identities consume completed native providers, and fit
reports consume the single F-X207 solver and physical owner inventory.
No second CLI fitting engine, JSON parser or lifetime implementation.
Serialize the two command files and native/Python additions against their
providers. Formal dependencies remain completion barriers before source claims,
not unresolved product choices or a reason to reimplement their capabilities.

## Resolved one-shot operation contracts


Freeze --map as a UTF-8 JSON array of objects with placeholder:string, value:string and optional expect:nonnegative integer|null. Array order and repeated placeholders are meaningful. Unknown keys, nonobjects, missing strings, booleans as counts, fractional/negative/overflow counts, invalid XML replacement text and command-line --expect combined with --map refuse before candidate publication. Preserve normal serde_json string decoding, never stringify arbitrary JSON values. Duplicate object keys follow existing serde_json::Value last occurrence wins behavior. State this exact rule in help and test it. Repeated array entries remain separate operations. No duplicate-key rejection is claimed.

An omitted/null expect imposes no count requirement. Every pair searches the staged result of previous pairs. A-to-B then B-to-C cascades, and duplicate placeholders are applied twice. An unrestricted zero count succeeds in map mode, including PowerPoint. Existing single-pair behavior stays family-specific: Word permits unmatched0, commands.rs:1948, while PowerPoint refuses zero unless expect0, rpptx-cli/commands.rs:543. Do not apply the single PowerPoint no-match refusal to every map entry.

Empty map returns an empty successful report with total0 and leaves native state and Python revision unchanged. A mutating CLI invocation still publishes the unchanged source through the required output path and reports it only after publication. It must not create an output before map parse/output preflight succeeds. Native Word try_replace_all_expected at document.rs:27163 currently commits its clone even for an empty input, so native publication no-op needs an explicit early return if required. Existing Python expected_replacements already leaves its revision unchanged when no count is positive, rdocx-py/document.rs:1867. Do not manufacture a Python empty-map revision defect.

Use existing Word counted signature. Add family-local PowerPoint `try_replace_all_expected(&mut self,pairs:&[(&str,&str,Option<usize>)])->Result<Result<Vec<usize>,ReplacementCountMismatch>>`, with mismatch fields index/placeholder/expected/found. Do not import rdocx mismatch types into rpptx. Validate all input placeholders before staging. PowerPoint retains its actual native invalid-character escape policy from lib.rs:1838 unless separately approved to change it, while Word rejects invalid XML replacements. A map should not secretly unify those existing family rules.

One candidate is prepared, pairs run in input order, and only the first mismatch or hard failure is returned. Expected mismatch reports that pair's zero-based index, search string, expected and found counts. It returns no successful prefix report and publishes nothing. Hard parse/ownership/serialization/reopen failure is RdocxError/RpptxError in the public binding and nonzero CLI refusal. Late failure preserves complete original native bytes, revision and handles. Counts are logical family counts, not serialized physical branch count. After all counts agree, validate the complete staged package and publish once. Python Word keeps replace_all's tuple-count result. PowerPoint adds the equivalent tuple-count method. No CLI-only batch loop.

Success schema1 payload: operation='replace', mode='literal'|'regex', pairs=[{index,placeholder,value,count,expected}], total_count, output. Expected remains null when unrestricted. In regex mode placeholder is the pattern and is labelled pattern in human output. Total uses checked arithmetic. Stdout contains exactly one JSON object when --json, with no progress prose. Refusal uses existing stderr human diagnostics and nonzero exit, no success-shaped partial JSON. A new structured error envelope is outside this change unless explicitly reconciled across both CLIs.

## Word regex ownership and error route

Add shared native try_replace_all_regex_expected with the same ordered mismatch contract, and additive Python replace_all_regex_expected(pairs)->tuple[int,...]. Preserve Python replace_all_regex's existing total-int return. Compile every regex and validate every replacement before editing any candidate. Syntax is the native Rust regex dialect and captures.expand semantics, not Python re, shell interpolation or PCRE. A valid empty/zero-width pattern is accepted but zero-width matches are skipped and not counted, exactly placeholder.rs:113. Empty literal semantics must also remain family-specific: native Word replace_in_paragraph returns0 for empty placeholder, placeholder.rs:40, while native PowerPoint require_replacement_placeholder rejects it, lib.rs:9783. Do not claim both native families already refuse an empty literal. CLI map parsing may follow those actual family contracts, with expect governing the Word zero result, rather than changing existing single-pair behavior.

Regex matches within native replaceable run stretches: ordinary adjacent text runs may match across runs, but controls, revisions and supported wrappers split stretches. Native placeholder.rs:49 documents those boundaries, and1736 uses captures_at on paragraph text. No matching across paragraphs/cells/stories, field instructions, deleted revisions or wrapper boundary edges. Preserve first matched run formatting and unrelated runs/opaque XML. Native notes rewriting excludes separator/continuation separator/notice and untyped id<=0 entries, placeholder.rs:1419. Charts and comments are not implicitly added: document.rs:27475 lists main/header/footer text-bearing parts, and note rewriting handles footnotes/endnotes. Typed simple-field displayed text is reachable through its supported wrapper, not arbitrary field code.

Physical alternate textbox copies use the existing document-wide raw replacement policy: edit the copies independently, count only the selected logical representation, and do not use the X202 scoped correspondence algorithm as an unrestricted regex rewrite. Existing placeholder.rs:1243 routes regex through rewrite_text_boxes, while native document.rs:27441 can change a part with count0 when only an uncounted copy matches. This differs deliberately from scoped selected-owner synchronization.

Material error gap: legacy replace_regex_in_xml_parts returns usize and uses if-let Ok for raw rewriting at document.rs:27443, with reparse failure ignored at27453. Legacy header loading can likewise skip a failed load at27421. The new counted checked route must return and propagate Result for every touched required owner and staged reopen. Reuse the existing actual traversal/matcher, but do not equate that swallowing helper with an all-or-nothing checked operation. Keep the legacy method's behavior unchanged unless a separate labelled behavior correction is approved. Malformed raw owner tests must prove the new route's original-byte rollback. A checked preflight may reject malformed supported parts before invoking shared edits, but cannot count them as empty matches or silently bypass a changed uncounted copy.

## Image extraction names, bytes and filesystem publication

X202 owns native/Python occurrence inventory. Existing Document.images at document.rs:28939 is body-only, and ImageInfo at30663 lacks physical owner/bytes, so neither is a complete provider. X204 consumes a reviewed owner-aware checkpoint that distinguishes picture occurrence from media part. It must not duplicate scanner identity or silently call unsupported group/VML inventory complete.

Extract each unique internal owning media part once, in the order first encountered in the provider's deterministic occurrence inventory. Occurrence JSON records keep story kind/part/owner, selected physical occurrence, EMU size, description/title/decorative, relationship and media-part/content-type plus its shared extracted file. Distinct source parts remain distinct files even if bytes match. Do not deduplicate by relationship string across owner parts or by byte hash.

Use generated flat names image-000001.<ext>, padded at least6 digits and expanding when needed. Extension comes from a bounded content-type mapping, never unsanitized package path or alternate title. Unknown content type uses bin and remains recorded in JSON. Never join target names containing separators, '..', drive paths or producer-controlled names. Write exact embedded bytes without rasterization/reencoding. External linked image occurrences are retained with status='external', file=null and URL as metadata, without network fetch. Unsupported occurrence ownership gets a named diagnostic or checked refusal from the provider, never a guessed empty entry. Missing internal media/relationship is a hard prepublication error.

Preload all required bytes and precompute every destination and JSON listing before creating/publishing files. Directory must be an existing real directory or a new directory created only after full source validation. Reject a symlink directory target. Use shared ensure_output_paths_allowed at oxml-cli-support/lib.rs:143 to protect input identity, aliases, hard links and collisions. No --force for extraction: StagedOutputSet::with_replace_existing documents that previously replaced outputs remain on late failure, lib.rs:212, while nonreplacing publish removes this invocation's earlier files on failure at257. Therefore the current forced mode cannot satisfy the chosen extraction rollback contract.

Stage all new files with nonreplacing StagedOutputSet and publish as a set. On failure clean this invocation's temporary/new files and remove a newly created directory only if empty, preserving preexisting unrelated files. Existing support rollback is best-effort filesystem cleanup, not a claim of crash-atomic multi-file rename. Tests inject late publication failure and verify no partial set remains under normal recoverable errors. Print JSON listing only after successful file publication. An empty extraction produces an empty listing and no unnecessary new directory. Read-only inventory never saves the document.

## Metadata, fill and count projections

Word meta get returns core title/author/subject/keywords plus immutable typed custom property snapshots from X203, preserving native PID/fmtid/type/order. PowerPoint meta get uses X205's common metadata surface. Meta set parses all assignments before staging, rejects repeated core/custom names and a set/remove collision, clones the latest native properties, changes only supplied fields and retains opaque metadata and absent fields. No fresh-template normalization for imported files. Empty textual core assignment follows the common provider's explicit removal policy, not a guessed count or author default.

Choose repeatable `--custom NAME=TYPE:VALUE` syntax split at the first '=' and then the first ':'. Types are str, int, float, bool, filetime and empty. Str preserves everything after its type colon verbatim. Int is signed32-bit, float finite64-bit, bool accepts true/false only, empty requires no payload. Filetime uses exactly X203's approved checked lexical CustomFileTime authoring policy, without CLI-only timezone normalization. `--remove-custom NAME` is explicit deletion, missing removal is a no-op. Existing Lpstr/Lpwstr updates preserve their native type/identity unless the explicit type requests a change. Raw custom values are read-only reported data, not authored arbitrary XML. This is input syntax over shared typed APIs, not a new property model in commands.rs.

Repeated `fill --tag NAME=VALUE` splits on first '=', preserves remaining text and rejects duplicate tag names. It means tag only, never native bind_content_controls alias fallback, content_control.rs:113. Proposed shared `fill_content_control_tags(&mut self,pairs:&[(&str,&str)])->Result<Vec<usize>>` stages all selectors once and has Python counterpart. Every matching typed main-body control for a tag is updated using existing native checked binding. A requested tag with no match refuses the whole command. Unsupported bound owner fails atomically. Parse/validate all values before candidate editing. Count controls logically, preserve custom XML/display consistency, publish once and report per-tag counts.

Inspect word/character/page counts consumes X203's exact body token/Unicode scalar and deterministic layout provider. Report count_scope='accepted-main-body' plus character_include_spaces=true, page_count_source='deterministic-layout', and provider diagnostics. Do not count all-story display labels or substitute cached app metadata. Pictures are additive inspect keys using X202 identities. Existing schema1 keys and meaning remain unchanged. A unavailable mandatory provider result is an explicit checked error, not invented0.

## Slide, notes and fit conventions

All user-facing slide and numeric layout positions are one-based. --layout accepts either a decimal positive index in1..layout_count or exact unique name. Entirely numeric input is an index, not a layout name. Reject0, overflow, unknown and ambiguous names before mutation. Native layout_name/layout_count exist at rpptx/lib.rs:1939/1944. --at N is the final insertion position in1..old_slide_count+1, default append. Add then placement must be a common staged operation, proposed Presentation::add_slide_at(layout_index,position)->Result<usize>, exposed in Python too. It composes existing add_slide3253/move_slide2585 on one clone and publishes once, not two separately committed CLI operations.

Duplicate N uses native duplicate_slide2791 and inserts directly after source. Remove N, hide/show N use existing checked operations and preserve relationships/notes lifecycle. Move N --to M uses final position in1..slide_count after removal/reinsertion, not a pre-removal gap position. Native move_slide at2585 already defines final zero-based index. N==M is a no-op. Convert each index once, at parsing-to-provider boundary. Mutation commands require a separate output and preserve original input under every refusal.

Notes set N requires exactly one of --text or --from-file. Read complete UTF-8 file before native mutation, retain LF/CRLF and all leading/trailing characters, and do not add a newline. Invalid UTF-8/missing source file/conflicting options refuse before creating a notes part. Existing Presentation::set_notes_text at2616 supplies common lifecycle and Python already exposes notes text. Report selected one-based slide, output and actual text scalar count, not byte count disguised as characters.

Fit is read-only and consumes a common X207 provider, proposed Presentation::fit_report(font_files)->Result<FitReport> with the same Python exposure. No CLI search loop or calls to mutating fit_text/refresh_autofit. Report every admitted frame with stable slide/owner path/shape id/name, table-cell path when applicable, usable bounds, measured overflow/current stored renderer scale, needed_scale and diagnostics. Needed scale is explicitly the hypothetical total font scale from the common shared measurement trials, not a reused current font_scale or an unnamed additional multiplier. If no permitted scale fits, needed_scale=null with reason='unfit-at-floor'. Do not report25 percent as success when it still overflows.

Fit stdout JSON uses schema1, frames and diagnostics, no output file. Exit0 only when all relevant frames were assessed and none overflow. Exit1 when any assessed frame overflows. A mandatory frame that the provider cannot assess yields an explicit unsupported-owner diagnostic and a refusal exit distinct from a successful fit, using the existing CLI generic-error nonzero path. Empty/no-text frames remain accounted for by owner identity or documented empty status, not silently used to reduce an every-frame promise. X207 must settle full owner admission and shared inherited-owner contexts before fit is complete. Existing text_layout_deterministic at1199 excludes cells/master/SmartArt and reports current scale, so it cannot satisfy this provider merely by renaming fields. Fonts use the shared deterministic/caller-font preparation and requested-versus-used diagnostics from X195/X210.

## Fields combined operation

Choose shared proposed `Document::update_all_fields(&mut self,context:&FieldEvaluationContext)->Result<FieldsUpdateReport>` and matching Python method. Report ordinary_updated:usize, layout:LayoutBackedFieldUpdateReport, ordered retained FieldEvaluation/diagnostic facts, total_updated and saved_cache_scope. One outer candidate evaluates/updates ordinary fields first and layout-backed caches second, then prepares/reopens and publishes once. Reuse update_fields1809 and update_layout_backed_fields1840 with their exact supported stored-cache behavior. PAGE/NUMPAGES in headers/footers intentionally retain caches as documented field.rs:1819. No unconditional cache rewrite promise.

`fields update --now YYYY-MM-DDTHH:MM:SS` supplies explicit civil FieldDateTime context. Native FieldDateTime at field.rs:1344 has only year/month/day/hour/minute/second, no timezone or fractional seconds. Accept exactly that civil grammar with Gregorian calendar and time bounds, rejecting timezone suffixes, fractions and leap seconds instead of pretending the native value has an offset policy. Omission means context.now=None, preserving unresolved date caches/diagnostics instead of reading ambient current time. Invalid input refuses before mutation. Derive FILE/FILENAME context from an explicit documented input path policy, not nondeterministic temporary staging names. Layout diagnostics are report data. A hard failure in either phase discards both phases. X204 owns this narrow common orchestration only after explicit shared-file claim, not an unreviewed CLI loop over live operations.

## Completion and evidence boundaries

Each command needs one existing-entrypoint integration test plus its full option/error/output matrix, installed Python/native parity, exact opaque/source retention and every required provider checkpoint. Plans do not prove APIs are present. New native batch/regex/tag-fill/field/add-at orchestration and their Python binding belong to explicit X204 narrow shared-source claims, serialized with202/203/205/207. No new source file/module/dependency or general replacement abstraction is proposed. Expected hash remains unchanged for additive valid-input behavior, but any deliberate correction of legacy swallowed error behavior requires a separately labelled reviewed contract.

Canonical read HEAD: `decfe4b5356da90c22cef6d85b6553905d1de373`. Source SHA receipts:

- `crates/rdocx/src/document.rs`: `3b8eec9fc3980a9dc743be2faef39d1126c819be102a61b58a4f6ae798a59620`.
- `crates/rdocx/src/field.rs`: `63db78f9e205c56f2982c46a5999092c0041e00204256455669d7b6da07001d7`.
- `crates/rdocx-oxml/src/placeholder.rs`: `26ddc9f8912473176bc7f3a0bab2ed4bbe775c555a395524953d913156c25fef`.
- `crates/rpptx/src/lib.rs`: `9e203e0e9e3fe71c8b0c9d070a1a0cffa6f757fbd849c88c46702ed910996b6b`.
- `crates/oxml-cli-support/src/lib.rs`: `24b4104b26df27e800d049378ad31700aa04f99dbd9d840c3ee8d8d31b3b5cc7`.

## Complete stacked PR330 reconciliation

PR [330](https://github.com/tensorbee/rdocx/pull/330) closes Issue307 and
remains this F-ID at head274e3ca82b3db0393f0b3e0833b640ae2bbf9dc8.
PR325 head87d4b71c1ed211ecf34156d39f18a69cb4fd44ad is its authenticated
ancestor and merge base. Six inherited commits belong to X203 once. Ten new
commits and27 feature-only paths belong to this CLI contribution. Complete
source, discussion, stack/base patches and assessment are retained in
/private/tmp/S91-PR330-intake. Root authenticated92 source blobs and128
artifacts, together with25 reused318 blobs and17 artifacts, with no mismatch
in /private/tmp/S91-root-330-318-source-authentication.json.

Adapt the ordered replacement schema, explicit civil date validation, output
identity checks and nonreplacing staged publication. The approved empty-map
no-op and unrestricted absent expected-count semantics govern batch changes.
Use one checked native/Python transaction rather than sequential live CLI
operations for replacement, tag fill and combined fields. Preserve now=None
on omitted --now. The proposed ambient local clock and Unix libc dependency
are not admitted. Extraction consumes X202's complete physical-owner inventory,
uses generated flat names and refuses missing internal media. The proposed
--force producer-name publication is not the approved rollback contract.

Read-only fit consumes X207's complete inventory, Q/S trials and stored-layout
containment with X195/X210 same-pass diagnostics. Unsupported mandatory owners
cannot disappear into successful empty results. Metadata allocation consumes
X203's checked PID and typed property policy. Checkbox native/Python extension
belongs to X203 and qualified symbol rendering to X210. Contributor tests,
archive measurements and proposed copied modules are not local acceptance
evidence or new file/dependency approval. Formal prerequisite completion and
exclusive source claims still apply.
