# F-X236, integration reconciliation, pass 1

**Reviewed**: canonical staged diff against current sprint HEAD after F-X234 integration,10files,161inserted and11removed lines. Both approved plans and retained clean source/metadata reviews inspected. Audit is limited to reconciliation, source preservation and combined measurement, not a repeat source review or full gate.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Defects

None found.

## Smells

None found.

## Nitpicks

None found.

## Not found

- Reconciliation correctness: README and catalog agree on the actual combined rdocx measurement3733529compressed bytes,51954601normalized member bytes and37members. Independent archive measurement reproduced that tuple and the repository validator passed with unchanged64byte tolerance and10MiB ceiling. The84member-byte increment over the integrated F-X234 measurement corresponds to the six selective annotation lines. No unrelated measurement or policy changes appear.
- Preserved conversion behavior: canonical working and staged math.rs bytes exactly match reviewed F-X234 source5579e2339da74769981d10e1ab5246ce15a51bd7, SHA-256da3be0b2949493294036f77205d498fdc86b7413bca3d8a6ef68c9fc8e80c9f3. The measured archive contains those same bytes.
- Preserved setter behavior: canonical working and staged run.rs bytes exactly match reviewed F-X236 source796c6cb02497994d7906e8c7e4d1789d8985e86b, SHA-25679005f81af763e9ace17cf6b3542a3ba8758a11e195fc0776c409fd5048eda75. The measured archive contains those same bytes. No new product-source reconciliation exists beyond the reviewed six annotations.
- Documentation and contract: distinct HLD12/15 sections retain both the bounded formula/Pandoc contract and selective non-LTO setter qualification. Both completed plans and their independently clean source and metadata reviews remain present. No source behavior or approval scope was rewritten to resolve the metadata conflicts.
- Archive provenance: the measured archive precedes the resulting self-included README numeric-row update. Direct comparison confirms that row is its only README difference, with identical member-byte length. This is the observation/update step, not a claim of a newly executed final package gate. Both actual implementation members match the reconciled source exactly.
- Structure, OOXML and panics: no new executable reconciliation, type, dependency or XML transformation was introduced. Staged whitespace checks pass and no unresolved conflict appears.

The prior feature gates remain their recorded scoped evidence. This audit does
not claim a full integrated gate or publication. The pass ends here and returns
control to sprint orchestration for handoff consumption and the local batch
commit. No file outside this review was modified.
