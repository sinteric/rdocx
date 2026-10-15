# F-X211b, public owners, pass 2

**Reviewed**: Two frozen carriers only, 3 additions and 2 deletions relative to public pass1. Unchanged native14 and other public8 retain their prior independent review. No confirmation review or runtime replay was performed.

**Verdict**: 0 defects, 0 smells, 0 nitpicks. Both public pass1 defects are resolved within this bounded scope. This is not whole-child completion.

## Contract and authentication

Read the canonical microscope/workflow, approved parent and child contracts, prior public pass1 and binding/owner references. Capsule SHA `b79a9b672b91aa84fba3c59517a08b40e767ea9fb02eef38c0991d2e6a594be3` and delta SHA `50ae855bcdf370b2dd99665d000f269cde34f558b18ac6dff4cbcb39aabd088a` authenticated. All 3,208 current source entries matched the frozen map. Comparing it with pass1 proved exactly the two authorized carrier changes.

HEAD remains `0a4f9259c6d98f281075244cdc6b7b39da8edd83`. Logical index SHA `d1c274930f3047dc8ae8da4d21c9512eda688892ebb8b97a5c7624fa7f9034ef` and physical index SHA `84066f11157ba92a0518514e82ddff73d476e50a0a57eed259d2085a3e4e4c09` remain unchanged.

## Repairs

D1: `crates/rpptx-py/python/rpptx/_rpptx.pyi:55` adds Element to the existing explicit export inventory. It matches the existing runtime export and existing class signature. Full installed stubtest passes without an allowlist or suppression.

D2: `crates/rpptx-py/tests/test_documented_examples.py:9584` now checks the established ValueError for a physically absent text body. The preceding two unsupported Element refusals, package equality assertion and following inherited-style positive assertion remain intact. This corrects the test oracle without changing production behavior, materializing a body or weakening the checked XML contract.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Not found

Correctness, contract, panics, OOXML, tests and structure: no new findings in this two-carrier delta. The correction adds no native behavior, allocation path, schema admission, dependency, module or generic construct. The test still distinguishes actual unsupported owners and checks source preservation. Native and other public owner findings were already closed in their separate reviewed states.

## Actual evidence

The sixth wheel and installed-member authentication records matched their capsule pins. Actual wheel SHA is `63f7f46a77d9015fe4acd86ac67ee2df31b2dee60dabb00a2efcfbbc3a10bcf0`. All retained wheel members and all 14 installed-member hashes authenticated. Relative to the fifth wheel, only the stub, generated SBOM and RECORD differ. The native extension and Python runtime package members are identical. Agreement SHA `fa48cfc7243a7739383062b814c2d98964b51a68b03ca061647b862e09032eb9` therefore supports reuse of the exact nine real Element allocation-refusal result, without asserting a new execution.

All five qualified receipt rows and their log hashes authenticated with actual exit 0, no timeout, reaped leader and gone process group. Wheel build receipt SHA is `59bf4c54d92e0e53fe947de1575c1c2477294145a0be39cfbca129ecf4a2126b`. Install receipt SHA is `2f54a67af01428688553692a43e08b10dda4c7c68614e4e90bf7b4f026fa648d`. Installed controls receipt SHA `49e6fa24170745d8725e49fec85e83f43b8b4f366f1534ee863e850fd7882655` records 31 passed and 777 deselected. Strict mypy receipt SHA is `79108e9085015acd3dd47be591f419a97a7c3055a1f55d0b39101b5e8c76491f`. Full stubtest receipt SHA is `a948966cae4b149a4e814b7bd1ce874b2cb55addadb9de1710b8d51bbac73d73`.

Original failed matrix/stubtest receipts and public pass1 remain retained and unmodified. These successors qualify the repairs, not the historical failures. Policy, archives, whole-child native/runtime residuals, Office and parent frozen100 acceptance remain outside this review. No source fixes, tests, target, CP or Office operations were performed.
