# F-X208, focused native implementation, pass 1

Reviewer: Codex fonts_resume. Verdict: 1 defect, 0 smells, 0 nitpicks.

## Reviewed input and limits

Immutable snapshot `/private/tmp/S91-F-X208-native-focused-snapshot-1/receipt.json`, SHA-256 `8c99e0e09afdd24dd4b032b24b1ed5eda1afa56e8d83b2a713b425e8b9dd0969`. All ten compressed artifacts and decompressed byte lengths and hashes authenticated. Native facade source SHA-256 `187122a9db0f7859801556757dafa5271c9258291f102d16b877825373309f6a`. Shape model source SHA-256 `9cbb65a470a618b484a0f7d36519779d1f48c2401fd37b452ccab9edc8d61e13`. Snapshot HEAD `a6f3f1c1c014f510f5ff79827eae02974f574d01`, declared index SHA-256 `5e88f28fc4b7dca24d336c92e23430f72af07d43136069fb3adeb7cdea65353d`.

Focused scope is native footer candidate publication, repeated assignment, field content transitions, displayed start zero through static and timeline assembly, latent footer visibility and qualified placeholder identity cloning. The review inspected the participating existing native facade, layout, render, timeline, presentation and shape-model slices and their controls. Snapshot Python and subsequent theme additions are context only. This is not full F208 review, installed acceptance, native branded/multi-master acceptance, transition review, or completion. No source edits or test replay occurred. Report staging is external so unrelated worker implementation can continue.

## Defects

### D1, equal cached text suppresses an automatic-to-fixed date transition

`crates/rpptx/src/lib.rs:15665`

`fill_latent_body` returns from `LatentContent::Text` solely because `body.plain_text()` equals the requested string. Create an automatic date with `field_type="datetime1"` and `text=Some("same")`, then assign `HeaderFooterDate::Fixed("same")`. The body still contains its automatic `a:fld`. The fixed assignment can become a complete no-op, and `read_slide` reports Automatic because it recognizes the retained datetime field. PowerPoint may refresh that field on open, contrary to the requested fixed mode. The same text shortcut can retain an imported field in a footer requested as ordinary text.

Require content-kind compatibility before treating equal plain text as unchanged. Add a deciding automatic-cache-to-equal-fixed transition, save/reopen mode and field-absence assertions, and exact repeat bytes for the resulting fixed state. Keep the existing idempotence and formatting-preservation assertions. This is a mode correctness and contract defect, not a request to disable the valid same-mode no-op.

## Smells

Zero.

## Nitpicks

Zero.

## Not found

Correctness outside D1: zero additional findings in the inspected slices. Footer mutations validate text and date type before mutation, stage the selected slide or complete deck, and publish after fallible owner work. Existing duplicate and missing-template checks refuse before publication. The actual footer regression proves identical repeated package bytes, reopened state, unique copied field IDs and invalid-text source equality for its supplied control.

Contract outside D1: zero additional findings. Displayed start remains presence-sensitive, including zero, with checked addition. Static pages and timeline pages restore physical one-based page identity while passing displayed values to the existing field evaluator. Qualified layout type governs title suppression. Footer authoring uses slide-owned placeholders rather than an illegal slide-root `p:hf`.

Panics: zero findings in the inspected operation paths. The inspected indexing and expectations follow checked owner selection or construction within staged state. No general untrusted-input panic conclusion is claimed for the full feature.

OOXML: zero additional findings. `CT_Shape::clone_with_id` at `crates/rpptx-oxml/src/shape_tree.rs:1027` resolves actual inherited namespace bindings for the identity scan and changes only the owned ID span. Its deciding control at `crates/rpptx-oxml/src/shape_tree.rs:3492` retains foreign attributes and opaque children, preserves the original, saves/reopens the clone and refuses foreign, undeclared and rebound identity owners. This conclusion is limited to this shape-clone boundary, not every grouped or relationship-bearing footer carrier.

Tests: zero additional findings within the bounded supplied controls. The footer control does not cover the D1 mode transition. Remaining native template, multi-master, date-mode, relationship and installed matrices remain required by the plan and are not credited here.

Structure: zero findings. The inspected slice uses existing carriers, candidate publication, layout assembly, field allocation and namespace helpers without new modules, dependencies or speculative traits.

## Authenticated existing runtime evidence

The snapshot references and raw logs were independently SHA-authenticated. Footer control receipt `9012816db2d72cee9131d34c5f3b2c89230741fff9aec42f5a4711b01a56a579` qualifies one named test with 1 passed, 0 failed, 0 ignored. It does not make the entire historical wrapper successful because the later displayed assertion failed there.

Displayed-start successor receipt `cbec37b4ae0aa311ff9dafc76a07d62e6eda0fc44d164096b3a86b28f1c87973` qualifies its named test with 1 passed, 0 failed, 0 ignored. Clone receipt `66c22d83917893088a8874a63f77883f0376964f89328cc516fd5a371c6c3efb` qualifies its named test with 1 passed, 0 failed, 0 ignored. Binding native-check receipt `964f269eefae16101c15e19862be0c52dcc2d4626ff526312b38a2c6631e805a` records check exit zero, with retained pending unused-helper warning rather than a lint pass.

These rows retain their actual source-production-4, source-production-6 and source-production-8 scopes and recorded natural cleanup. They are not relabelled as full current-source or full-feature GREEN. Prior failed attempts remain failed. D1 was derived from the exact immutable source, without a new runtime claim.
