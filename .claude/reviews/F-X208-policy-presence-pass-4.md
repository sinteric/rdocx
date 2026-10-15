# F-X208, policy presence remediation, pass 4

**Reviewed**: Narrow correction of Python focused pass 3 D1. Immutable three-carrier snapshot receipt `e32a4cca5bb9abb88dab3fe53ddbd9a467d7cf842de36510628fa3a752b1a155`, source-production15. All compressed and uncompressed carrier hashes authenticated. Unrelated transition changes present in the carriers, subsequent implementation, CLI and whole feature acceptance are excluded.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Pass 3 D1 resolved within this source-bound slice.

## Resolution

At crates/rpptx/src/lib.rs:15376, HeaderFooterFlags now preserves each original Option<bool>, rather than collapsing omitted attributes to enabled. The checked slide_header_footer_policy at :2912 resolves the actual slide layout and master, returning layout provenance when a legal layout p:hf container exists, master provenance otherwise, and absence when neither exists. It does not invent a slide-root policy element or materialize any painted placeholder.

At crates/rpptx-py/src/slide.rs:1314, policy_source exposes that legal container provenance. The three optional policy getters at :1326, :1338 and :1350 preserve absent attributes versus explicit false/true. The existing materialized presence and date-mode getters remain separate. source at :1301 reports slide-owned content when present and otherwise delegates to inherited policy provenance, so a local override can coexist with independently readable owner policy. Stub properties at crates/rpptx-py/python/rpptx/_rpptx.pyi:1015 match the runtime optional types.

The absence, disabled/enabled owner policy and local materialized facts no longer have to share a single boolean. Layout container precedence is explicit, not a guessed per-attribute merge. Existing qualified native owner semantics continue to determine which latent placeholders paint.

## Aspects

- Correctness: zero findings. Current physical slide owner validation precedes policy reads, layout lookup is checked, and master parse errors remain errors.
- Contract: zero findings. D1 presence/provenance is resolved without introducing inherited latent painting or changing local setters and automatic date cache behavior.
- Panics: zero findings. Missing and unresolvable slide owners return named errors rather than unchecked indexing.
- OOXML: zero findings. These are read-only projections of modeled legal master/layout flags. Optional values retain physical attribute presence and no source serializer is changed by the getters.
- Tests: zero findings within correction. The new native control at lib.rs:18951 distinguishes missing policy, master optional values, layout precedence and conflicting optional values, retains no local painted fields, and verifies save/reopen. Actual policy row records 1 passed, 0 failed, 0 ignored. Current installed Python and strict typing remain pending, and are not claimed from the native test.
- Structure: zero findings. The policy operation reads current native facts through the existing owner proxy. No module, trait, dependency or state registry is added.

## Authenticated actual evidence

Runtime receipt `fdcc6353ce3ff76b4596d3a899dec9ad8da51146eb386901276b62a204b56d96` binds source-map `e2c077850e93a0cdd27f5be5185ce3d18c1b5fa3e0b915e34eec525f04667f1e`. Policy raw log `ed610c579c438a64c7f5033ebcce20a03759f7eb8b6adf4e25b05bfbeff95d13` authenticated. Its child exits zero and owned group4666 is naturally reaped with ESRCH and no signals. The same receipt also truthfully contains an independently completed transition control, whose raw log authenticated, but its broader transition semantics are outside this remediation review.

Native lib.rs SHA `0cc7c51bada399fd312d9f97e82f0374153b6b4ed0afdfcb16dd75a8d0c880ab`, Python slide.rs `14f6fcc8f182210d314e7264de16916dfe8628fb94cd2a54b7096b97d47f9838`, stub `1e8db63bf4405ae20c84f9dbde40a93fd4c8ea35d445672dfaafef511892270b`. Citations refer to the frozen copies rather than advancing worker bytes. Reviewer fonts_resume did not author this correction. No source edits or runtime were performed in this pass.
