# F-X207, retained stub wheel provenance, pass 13

**Verdict**: 0 defects, 0 smells, 0 nitpicks in the exact artifact qualification.

Actual Maturin packaging completed all four child rows successfully, then the two-member-only observer refused the regenerated SBOM. ROOT independently compared the old and fresh wheel members. Only the intended stub, its RECORD and the SBOM differ. The native extension is byte-exact and every other member is exact. Recursive JSON comparison proves the SBOM differs only at metadata.timestamp and serialNumber. Component, dependency, license and source identities are unchanged.

The fresh wheel06d5b683949b4ea2a448f467a57593b5558956b2d9be653a1912a80dfb13d49a is retained. Its new timestamp2026-10-11T20:10:37.469729000Z and UUIDdcd844f5-72af-4150-a12d-5171b1c9bfcf belong to this packaging attempt. Preserve the original failed observer receipt754de2e89f67583fd6312973549ec09037797372ab2af44bd045b6e0f2be11fc. Qualify and reuse the existing wheel with these two exact SBOM field changes and validated RECORD, rather than rebuild or discard evidence.

## Defects

None in this exact artifact qualification. The prior observer's two-member-only assumption explains its retained failure.

## Smells

None.

## Nitpicks

None.

## Not found

No additional native, typed-source or dependency change. Installed PowerPoint strict typing, stubtest and complete member/origin renewal remain required. No installation or new build was performed during review. Only this review file was written. Return to separate retained-artifact qualification and installed verification.
