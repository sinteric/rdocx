# F-X216, notes registration, pass 1

**Reviewed**: Bounded facade notes-master registration delta against the preserved normalization predecessor. Frozen capsule `notes-registration-frozen-1/manifest.json`, SHA-256 `4b5cb76ceeda1822953923cd8cfaf4941ff7070585dd30521af23d98de3720db`, seals library, integration tests and unchanged presentation utility. Production changes add one concrete owning helper and invoke it in notes creation and staged batch import. No whole-feature verdict is issued.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: zero findings. `crates/rpptx/src/lib.rs:7521` requires the actual notes-master owner and its main relationship scope. At7542 it requires exactly one typed global notes-master relationship, rejects external targeting and checks resolved target equality with that owner. At7562 a cloned presentation receives the actual relationship ID. Existing notes prevent a newly inserted registration from changing their inheritance at7566. Only the successfully checked model is assigned at7578.
- Contract: zero findings. Notes creation at7586 and staged batch import at8043 now register an existing master as well as a newly created one. The change retains the single-global-master boundary and refuses unsafe legacy inheritance rather than silently changing old notes. Public setters stage the candidate at7515. Batch publication remains after the fallible graph reconciliation. Ordinary notes-text edits and default master generation receive no new semantics.
- Panics: zero findings. `global[0]` at7548 follows the exact-one check. All new fallible model operations propagate named owner errors. No unchecked input arithmetic or new unwrap is introduced.
- OOXML: zero findings. The unchanged utility `crates/rpptx-oxml/src/presentation.rs:1386` reads actual qualified direct PresentationML lists and relationship-namespace ID attributes at1406 and1422. It rejects duplicate lists or nonmatching reference sets at1447. New registration uses the existing schema slot2 at1457, corresponding to `root_schema_boundary` at3380. Existing agreeing lists return unchanged. Namespace aliases are resolved by physical scope rather than matched by prefix spelling. Opaque raw children are retained by the unchanged owner serializer.
- Tests: zero findings in this bounded scope. `crates/rpptx/tests/integration.rs:34825` covers new notes, fresh import, exactly-one registration, valid legacy-unregistered notes refusal and byte-identical refusal, retained destination note/master bytes, source bytes and normal library reopen. At34911 malformed external and ambiguous globals are parser-admission negatives. The later conflicting actual main-root metadata case asserts source and destination byte identity after mutation refusal. These are different proof boundaries and are not conflated. The existing utility control at33857 checks schema position, exact opaque payload, idempotency, conflicting ID refusal and reopen.
- Structure: zero findings. A concrete owning validation helper has two current facade consumers. It adds no generic framework, forwarding wrapper, module or dependency.

## Evidence and limits

Before and after audit, frozen/live library `9c160ec1cd49feb3012e2c72a3f931e22c3a0ff94d746196cf55d5fab2ade0bb`, test `f9d2321f0648bafc7965ca611cb16cc3781c315d060493ea061203ad37ad8f2e` and utility `6e81e68bffec5ff6f77f35f833531d4b114df9b00863b6eb914c180f4a26ecd1` match the sealed bodies, sizes and source-map modes. The actual predecessor `2229b55af11da335976d4f997d16e8f37611d5b79bad95909b9cb82371407d8e`, patch `48ff1218f93b83533310a90951b38b7ff1e69a84cedb84f3985087cd9517ff97`, source map `5399d0d17bdb4e8cc98b77c1b0962d5f286d340ac45317e863239edceeef16ff`, retained integration binary `e5ab2912164950132c646eb6b156c14d97b97a1b5033d6113232227228c1cd15`, receipts and raw logs authenticate exactly.

RED2 receipt `3ef1762e78668dbc7f557505bbb68089042a4e7b84c8aac39fe460093e27d09d` proves the named registration control failed with actual count0 instead of1. Successor4 receipt `f6f55b35903a2b835d8c6cbaebe5e0e6d93b99968a41bd691883a96806b5764f` proves four named tests each1PASS/0FAIL plus strict native Clippy, all natural exit0, no timeout and safe cleanup. Setup compiler failures remain uncredited. Source package `71b9df47d7d6aec67f70fbb988c956ca802b5fc04f70f10af427132bc1321987` and imported package `83b3ec1792b76996f0118a95d3fc0fbb375ea34fc28fa6a9001c2446ae41bfeb` authenticate, each with exactly one qualified main notes-master ID and matching internal relationship.

Earlier native Office appearance failures remain failed evidence. The separately identified materialized-placeholder content defect is outside this registration delta and remains unresolved here. Fresh installed acceptance, final native Office appearance, broad suites, package gates, sprint verification and release are not credited. No runtime or source changes were performed by this review. Only this report was written.
