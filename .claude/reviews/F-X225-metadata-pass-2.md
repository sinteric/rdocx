# F-X225, archive metadata, pass 2

**Reviewed**: Four existing files added after the clean product review, three README measurement rows and their shared numeric catalog, nine insertions and nine deletions.

**Verdict**: 0 defects,0 smells,0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: the three README values match the recorded actual verified archive observations and the corresponding numeric catalog. All three updates preserve README byte length. Member counts remain24,20 and16.
- Contract: versions remain0.14.1, the date records the actual measurement day, and the existing64-byte compressed-size normalization tolerance and10MiB ceiling are unchanged. No measurement is a publication claim.
- Tests and source identity: the final three archives were verified after metadata changes. The current-member proof records source correspondence and exact normalized member totals, with compression inside the existing tolerance. The other19 packages require their separate unchanged-source bridge, not an assertion that these three measurements prove the whole workspace.
- Structure, panics and OOXML: no new code path, public API, parser or serializer change. The clean product review remains applicable to the unchanged implementation.

## Completion boundary

The measurement application record is `/private/tmp/F-X225-verified-archives/numerical-metadata-application.json`. Final current-source member checks are `/private/tmp/F-X225-verified-archives/final-three-current-members.json`. The full policy successor must finish before handoff. ROOT must remeasure the combined rpptx archive after integrating F-X226, since that separate feature changes the same package source. No automatic semantic merge or unchanged combined-archive claim is permitted.
