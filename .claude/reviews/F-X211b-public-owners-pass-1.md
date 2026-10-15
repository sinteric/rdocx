# F-X211b, public owners, pass 1

**Reviewed**: Frozen public10 patch against Base0a4f9259, 10 files, 1,579 additions and 175 deletions. Native14 is unchanged from the separately clean native-owners-pass-2 review. This pass audits the public owner/API phase only.

**Verdict**: 2 defects, 0 smells, 0 nitpicks. D2 is a test setup defect, not a production getter regression. This is not whole-child acceptance.

## Contract and authentication

Read the approved parent and child plans, canonical workflow and microscope command, and their physical-owner, namespace, relationship and binding references in HLD04, HLD06 and HLD10. The nine public owners retain the approved `Element.xml: str` and `replace_xml(xml: str) -> None` contract. Legacy `Shape.xml` stays bytes.

The frozen capsule SHA is `a1792427c6bfc64e14a6f345a1e772181c466ccd8da193218a32da6c3f4d16e9`. The exact public patch SHA is `de3d5696835636c2c0888c71444a9c3cce5b8a2009332df3b5a4bab4054b5300`. All 3,208 tracked source entries, including public10 and native14, authenticated against the capsule before report publication. HEAD remains `0a4f9259c6d98f281075244cdc6b7b39da8edd83`. Logical index SHA is `d1c274930f3047dc8ae8da4d21c9512eda688892ebb8b97a5c7624fa7f9034ef`. Physical index SHA is `84066f11157ba92a0518514e82ddff73d476e50a0a57eed259d2085a3e4e4c09`.

The fifth wheel authentication SHA is `c7ef579c2b2b52d4ea38b675cf6b74a0f7cb36b93257ba04c8a7c792c760ee00`. Its actual wheel SHA is `8161164cebb5e04c1ac586bb593f182e8cc40890b5f38af7f62c95426fefe2d9`. All 15 retained wheel members and all 11 installed rpptx package members matched the pinned bytes. No runtime was launched by this review.

## Defects

### D1, extension stub export inventory omits Element

`crates/rpptx-py/python/rpptx/_rpptx.pyi:50`

The extension exports `Element` at runtime through `crates/rpptx-py/src/lib.rs:137`, and the package re-exports it through `crates/rpptx-py/python/rpptx/__init__.py:40`. The stub defines the class at `crates/rpptx-py/python/rpptx/_rpptx.pyi:1641` but omits it from its explicit `__all__`. Full installed stubtest reports the exact missing runtime export and exits 1. This violates the child plan's full-stub compatibility gate. Correct the existing export inventory without suppressing the check.

### D2, absent-body test assumes an unsupported text getter returns empty text

`crates/rpptx-py/tests/test_documented_examples.py:9584`

This new test removes the actual cell text body, correctly checks that both unsupported Element reads refuse without materializing source, then asserts `frame.text == ""`. The established getter uses the optional actual cell body at `crates/rpptx-py/src/text.rs:163` and raises `ValueError("shape has no text frame")` when it is absent at `crates/rpptx-py/src/text.rs:390`. The older absence test at `crates/rpptx-py/tests/test_documented_examples.py:4827` qualifies safe no-op/refusal behavior without promising an empty scalar text read. The current installed matrix reaches this exact new assertion and fails. This is a test setup/oracle defect, not evidence that the production getter regressed. Preserve the failed original and qualify a successor that checks the established refusal and unchanged package/token facts.

## Smells

0.

## Nitpicks

0.

## Aspect audit

- Correctness: No additional findings. `presentation.rs:1968` stages checked native replacement and compares actual serialized packages before no-op/publication. `presentation.rs:2085` retains the selected owner and proven ancestors, retiring unsafe descendants. Table XML retains the table owner but retires grid descendants. Physical identity and destination claims are checked at `presentation.rs:2919`, without reincarnating removed owners from an ID match.
- Contract: D1. Nine getter factories require existing physical owners. Virtual cell/inherited style views cannot manufacture writable XML authority. Existing table-payload transition refusal is explicit at `presentation.rs:2065`. Family helpers preserve unrelated notes/body owners and retain the established ordinary text topology retirement rules at `presentation.rs:3383` and `presentation.rs:3497`.
- Panics: 0 findings. Scope counts and capacities use checked arithmetic. The audited `expect` sites consume internally constructed complete inventory, affected-successor and birth-offset records. No Python allocation follows `publish_owners`' successful token commit at `presentation.rs:3120`.
- OOXML: 0 additional findings. This public phase delegates qualified owner validation, relationship proof and retained XML closure to unchanged reviewed native14 providers. It does not add a permissive parser or a full-XSD claim.
- Tests: D2. The current real allocation controls deny `Element.tp_alloc` once for each actual owner and check unchanged package/held-token facts plus positive successors. They are not synthetic exception injection inside production.
- Structure: 0 findings. Concrete owner records and the nine-consumer Element type are implemented in existing carriers. No new trait, generic framework, module, dependency or forwarding wrapper is introduced.

## Actual evidence and remaining boundary

All four final receipt rows and their log hashes authenticated, with actual terminal exits, no timeout, reaped leaders and gone process groups. `public-owner-controls-final-5/receipt.json` SHA `5f78f042d10c11a18b941acabe5e5f6b20daf00d25564d8c90eead1de9fe4e5e` records 30 passed, 1 failed, 777 deselected, exit 1. It is not a green matrix. `public-stubtest-1/receipt.json` SHA `cad0fc94958adb8b18124520ddc735f795c26e3e6210a399d4680257c8f343f7` retains D1's actual exit 1. Strict mypy passed, receipt SHA `ccab1a28f9c6d2922db7ef387029a681d631f3a4f07318fe70370cdb8827c4cb`. The nine allocation refusals passed, receipt SHA `9cd5515187502932849d59aef0e3a90a7b7ae327c4802e5bc563233d5b86bdc4`, results SHA `c26276619e520e9c0a3d5801e14d30079ad1ddf3ec1c25e4cc3acbc9f152ad62`.

The original installed Paragraph.add_run compatibility failure remains historical evidence. Its current successor is among the actual passing controls, not a rewritten original result. Earlier setup/contract corrections remain retained. Native Office, final child scoped obligations, root composition and the parent frozen100 result are not certified by this bounded pass. No source fixes or tests were performed.
