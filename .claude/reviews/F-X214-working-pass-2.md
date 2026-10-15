# F-X214, bounded timing correction, pass 2

**Reviewed**: immutable source9 to source11 correction, two existing carriers, 154 added and 40 removed lines. This pass reviews the timing fixes for pass1 D1/D2 and the two readonly lineage call corrections. It does not replay the whole feature audit.
**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Bound input and evidence

Capsule receipt `/private/tmp/rdocx-f-x214-codex/.claude/scratch/F-X214-start/immutable-timing-correction-2/receipt.json`, SHA256 `79c73dc7a466385d2426d339d94d7a75e7cfdfdfa196b82c255668129b3b291d`. All11 records and8 external references independently match their recorded sizes and hashes. Complete diff SHA256 `d7eae82264a6c4b12366f323b7f62fd43b48aa83941944e2815d7dcdb3e35260`.

Source11 map SHA256 `69cdbb764c38327bf62edbf872ff24d4b3f7262c5082810ea51d14e6c22a92c8`. Source10 to11 changes exactly timing.rs and context.rs, with3177 other map records identical. Source9 to10 adds only the public negative test to timing.rs and copies the existing pass1 review. The source9/source11 complete carrier copies match their respective maps. The receipt's3176 unchanged figure covers source9 to11, excluding those two carriers and the added review, rather than counting the source10 to11 comparison.

The approved plan, current in-progress status in both shared ledgers, canonical CLAUDE/workflow/microscope, and previous whole pass1 were read. The existing plan's depth32, URI ownership, source-preservation and linear-work requirements control this bounded correction. No change to product scope or waiver is approved by this report. Source citations below refer to the immutable `.after` timing.rs copy.

## Defects

### D1, empty condition-list root now reaches an unchecked expectation

`crates/rpptx-oxml/src/timing.rs:2660`, `:2666`, `:2718`, `:2737`, `:2738`, and `:2507`.

The corrected presence collector handles both Start and Empty condition-list elements. For an empty stCondLst or endCondLst it captures the self-closing root and forwards it to collect_condition_list_target_presence. That helper initializes its root only in the Event::Start arm. Its first event is consequently Event::Empty and `root.as_ref().expect("root set")` panics.

Concrete public input, with p bound to the PresentationML URI: `<p:timing><p:tnLst><p:par><p:cTn id="1"><p:stCondLst/></p:cTn></p:par></p:tnLst></p:timing>`. The native common-node parser accepts the self-closing list through its existing Empty child sequence check, leaving an empty condition vector. CT_Timing::from_xml then invokes the presence projection, which panics instead of returning the accepted model or a checked error. The same path applies to endCondLst. This is a newly reachable helper precondition violation introduced by passing empty list roots, not an inferred failure from a missing test or compiler error.

Handle an empty list root without calling a Start-only helper, or give that helper an explicit empty-root path. Add public controls for both spellings and preserve the ordinary nonempty explicit-target behavior. Review does not execute a reproducer or modify the source.

## Smells

None independently identified within this bounded scope.

## Nitpicks

None.

## Closed findings and aspects without additional findings

- Pass1 D1: both cursors now admit only concrete schema-selected timing owner transitions with URI qualification. Foreign and qualified unknown owners are captured at their actual opaque boundary. Their descendant common nodes no longer select metadata owners. The retained public negative genuinely fails before this correction and passes afterward.
- Pass1 D2: opaque descendants no longer grow either cursor's namespace stack. capture_element uses an iterative reader/writer and does not clone namespace scopes for each opaque descendant. The adversarial unique-prefix opaque-depth control proves equal selected event visits and constant maximum open depth at256/512 opaque levels, with doubled retained payload bytes and exact public round-trip. Concrete recognized paths remain limited by the first parser's32-owner admission.
- Correctness: the shared transition predicate covers the existing par/seq, behavior, media, and nested child/sub list paths. It keeps the existing modeled-versus-unsupported projection choices and source frame ranges. No additional semantic change was identified in the readonly lineage calls.
- Contract and OOXML: unsupported raw ranges and original source are retained, without promoting qualified descendants through opaque ancestry. No public field, signature, equality rule, namespace policy or schema child order is altered by these corrections.
- Panics: D1 is the reachable new expectation failure. The changed owner-stack expectation follows the nonempty branch guard. No other reachable new panic was identified in this bounded diff.
- Tests: source10 adds only the genuine public metadata negative. Current controls cover source aliases, sublists, opaque bytes, selected-event work, foreign overwrite and the original public nesting refusal. They do not cover self-closing condition-list roots, which is D1.
- Structure: the concrete transition helper is used by both existing timing cursors. The private work counters expose deterministic review/test evidence without a new module, crate, trait, generic or flag. The context.rs changes remove mutable arguments where only a borrowed slice is read.

## Authenticated execution and limits

Actual RED receipt SHA256 `25e21f16af33e4c0e033540bcb2f7e327e9666aa15aa60dbf78b569cc0876f8b` binds source10 `dcf3666559293a87f0b9131aedc9f7ce2c19235f7c9bb96bcba542738664b864`. Its exact selector runs1 test, fails101 with Some(false) versus required Some(true), and records natural safe cleanup and matching source. This is genuine semantic RED, not a setup failure.

Actual GREEN receipt SHA256 `55026335df11e1068dd7904ad7eb9bbcdacc5af30987cf18b13bffe00dd32bb9` binds exact source11. Raw logs show4 timing controls PASS, the original public nesting selector1 PASS, and four-crate all-target/all-feature Clippy exit0. All rows retain original command arguments, terminal exits, source matching and safe cleanup. The earlier Clippy failure receipt remains retained. These controls close the original findings but do not exercise D1, and they do not establish whole-feature acceptance.

Release timing, CLI and isolated Python, fresh installed wheel, corpus, full scoped gates, native PowerPoint, archives, HLD and preparation remain outside this audit's execution credit. No runtime, Cargo, Git mutation, CP, Office or source edits were performed. This pass ends with this report. D1 requires a distinct remediation phase and a bounded successor review.
