# F-X250, all, pass 3

**Reviewed**: single Chart entry in the existing native stub __all__, against e02c90ff381f549b73008d216ec7da44cd0088f5. Unrelated PPT test amendment excluded.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

0.

## Smells

0.

## Nitpicks

0.

## Assessment

`crates/rdocx-py/python/rdocx/_rdocx.pyi:81` adds Chart to the explicit export list. The class is already declared at `crates/rdocx-py/python/rdocx/_rdocx.pyi:781`, registered by the native module at `crates/rdocx-py/src/lib.rs:202`, and exported by the existing Python facade at `crates/rdocx-py/python/rdocx/__init__.py:199`. This is the missing stub-export alignment expressly required by `.claude/plans/F-X250-design.md:27`, with no new signature, class implementation or runtime behavior.

The actual original ordinary installed stubtest log reports exactly1error: runtime Chart absent from the stub export list. The source change directly corrects that mismatch. Fresh ordinary wheel and current stubtest GREEN remain pending, honestly recorded in readiness. This clean source review does not claim an artifact acceptance result or remove that gate.

Authenticated readiness SHA256e4c3caae32447983c8cd9f6522668262f9df2137235b7ff43f45db487ec3249e, all declared artifacts and original failed receipt/log pins. Combined freeze SHA256eca2bd943a1f611c304aa6d300839c7002d77d00ae0acd77f9bf7ede790ec65f and patch SHA2566b0590b0e839cd0d3a2fe80a1262933fae2c7481e69f368db21fb0d76565ee00 match. All6frozen source/reference entries remain unchanged.

No tests, product imports, builds or native work occurred. Only this report was written for this pass. Review ends and the canonical writer lease returns to root. No confirmation source review is needed absent new material, but actual fresh artifact/stubtest qualification remains required.
