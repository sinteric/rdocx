# F-X208, Python API focused review, pass 3

**Reviewed**: Immutable source-production13 snapshot, receipt `6edd1b8eb357429cc64fdbd76cdb4b67c27e29731d09962a6936002f075f86ca`. All fifteen compressed carriers and their uncompressed SHA, size records authenticated. Focused inspection covers HeaderFooter, deck publication, per-master Theme and collections, Paragraph.add_field, displayed-start bindings, exports and stubs, with their existing native providers. This is a partial review. Transitions, CLI, later implementation and full feature acceptance are excluded.

**Verdict**: 1 defect, 0 smells, 0 nitpicks.

## Defects

### D1, applied-state metadata loses legal owner flag presence and provenance

`crates/rpptx-py/src/slide.rs:1232`, `crates/rpptx-py/src/slide.rs:1292`, `crates/rpptx-py/src/slide.rs:1320`, `crates/rpptx/src/lib.rs:15869`.

The live proxy reads only slide-owned placeholders through slide_header_footer and read_slide. Its source is consequently only slide or absent, and slide_number_present repeats the same boolean as slide_number. Neither this projection nor the public metadata reads the layout/master p:hf presence, explicit attributes or their precedence. A slide without local latent placeholders has identical public state when its relevant owner flags are absent, explicitly false or explicitly enabled. The existing native master_header_footer and layout_header_footer operations can distinguish those inputs at lib.rs:2890 and lib.rs:2904, but that information does not reach the Python proxy.

The approved plan at .claude/plans/F-X208-design.md:113 requires applied state to distinguish absence from false and local override from inheritance. Its live proxy contract at line 336 requires presence/mode/source metadata. Preserve local visible placeholder facts while exposing the separate legal owner policy and provenance, rather than deriving both from one materialized-owner boolean. A deciding imported control needs absent, explicit disabled and enabled/conflicting owner policies, followed by a local override, without mutating any input on read.

The qualified native controls establish that owner-only latent placeholders do not paint. This finding does not request inherited placeholder rendering or a slide-root p:hf element.

## Checked without additional findings

- Correctness: zero additional findings. Compatible automatic cache retention is implemented at native lib.rs:15677-15683. The proxy date setter preserves the existing automatic field type at slide.rs:1359-1364 and sends an absent replacement cache, which the native branch retains. A changed field type replaces the field rather than incorrectly preserving an incompatible cache.
- Contract: D1 above. Zero additional findings within this slice. Deck replacement and per-field local patching use the approved signatures. Displayed-start retains optional state, accepts zero and delegates schema/overflow checks to the native provider. Master views resolve the original part identity on each access, and Theme colors/fonts follow that master rather than a first-theme snapshot.
- Panics: zero findings. Sequence indices and slices use checked normalization, missing owners produce Python errors, and publication preparation is fallible before publish.
- OOXML: zero additional findings. Native staging validates text and field types before committing, retains qualified template handling and complete field identity allocation, and Python views introduce no serializer. The schema and namespace authoring primitives remain covered by the earlier native focused review, not freshly requalified here.
- Tests: zero separate findings. Existing public tests at test_documented_examples.py:8441-8512 cover basic entry points, not the missing policy distinctions in D1. The snapshot binds actual native binding check4 receipt `27f71812e28280154e1e3ce598230a14ae17e1426c134c2f23d4617d0997143d` and raw log `c18ec01c44f1f232f2607bd2fc21ad5c36f129f1a756f63a9524f3ba057b6311`, both authenticated. Its one check row exits zero with natural process-group cleanup. No current Python wheel, installed controls or strict typing pass is claimed.
- Structure: zero findings. Owner proxies hold presentation and physical identity rather than detached mutable data. The sequence helper has current slide, layout and master collection consumers. Native candidates stage once and publish through the existing causal owner mechanism, preserving unrelated owners and invalidating changed text descendants.

## Evidence boundary

Python slide.rs SHA `0f9952a9a34259140999cd6b9c92b19029d285776997a015de3ffd239c6c661d`, presentation.rs `1a4adf0336198e9d446e849c6ff44fbe7b6ce7e923f93a61178957c0f07491af`, text.rs `75382538322e32288b8ad102b8136e4bb554e0bb777f5b1389254715f07be431`, stub `6a396cd98f46895056c7b2576b84c2b0fe27673740f7878927026c71813dc30f`, native lib.rs `e31760a97d4f8077da0e8a1b9a42068a6c9e84bd8c909da69412222233db0bbe`. Citations refer to these immutable carriers, not the advancing worker. Reviewer fonts_resume did not author this F208 slice. No source changes or runtime executions were performed in this review.
