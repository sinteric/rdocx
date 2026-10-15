# F-X205, integration composition, pass 20

**Reviewed**: Canonical staged F205 squash on HEAD `9877a722854a0e4ee1389f1e1cbe443465ff6504`, compared with accepted worker `da639bbe`, handoff implementation Head `e71fdfed` and Base `338754a7`. The staged diff contains 62 files, 7729 additions and 243 deletions, including existing reviews and the approved plan. This is an incremental integration review, not a repeat whole-feature review.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Integration identities

Complete staged binary diff SHA `b7936cf3cbc8635b529fff90c43d1dc9c9fcebfbc62aa00ca8a463d37f50fb7d`. Staged product and HLD carriers were compared directly with the accepted worker Git blobs. Every changed carrier under crates is worker-exact except `crates/rdocx/src/document.rs`. That carrier is canonical HEAD byte-exact with only the existing F159 PowerPoint archive expectation changed from `0e3b27ea...` to `79354125ec3c356ad4f5ff1fa3bb773b5938c57a92c249dc13770ab902c62ec6` at line37825. The original Word archive expectation and complete pixel-comparison body remain unchanged. This retains completed F201 Word implementation rather than replacing it with the older worker carrier.

The remaining worker differences are root README, the shared metadata script and HLD02/08/10/14. They contain the expected canonical Word contracts and combined package measurement. No sprint ledger or hash baseline is staged. Unchanged provider carriers remain canonical because the staged path inventory contains no changes to them.

## Contract composition

All eight F205 HLD impacts are present: 02, 05, 06, 07, 08, 10, 14 and15. The bounded changes in HLD02/08/10 retain the canonical F201 section and related-owner authoring, qualified text/image projection and the user-approved half-opacity boundary. Their distinct PowerPoint parity, sampled-fill, script-face and binding sections are retained from the worker. HLD14 changes only the F205 acceptance paragraph relative to canonical HEAD. The F201 entry and F-X219 full-range opacity obligation remain intact. There is no replacement of Word intent with PowerPoint intent.

The metadata AST contains the exact admitted worker tuple values except canonical F201 rdocx-layout and rdocx-oxml measurements, plus the separately measured combined rdocx tuple `(3609198, 51230290, 37)`. All other values retain the applicable unchanged canonical or worker authority. All22 remeasurement dates are worker-exact, including unchanged F201 dates. The root README combined rdocx row agrees with the script. The crate rdocx README remains its canonical delegate, with no staged change.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: zero findings in the integration exceptions and exact product mapping.
- Contract: zero findings. Both approved Word and PowerPoint contracts remain represented.
- Panics: zero new findings. The sole non-worker source exception is a reviewed test constant, not production logic.
- OOXML: zero findings. No semantic source merge override or new serializer behavior was introduced during composition.
- Tests: zero findings. The F159 original failure and exact pixel-comparison successor are retained separately.
- Structure: zero findings. Composition introduces no new abstraction, dependency, module or file outside admitted feature artifacts.

## Actual prefix evidence and limits

Checkpoint `/private/tmp/S91-F-X205-integration-checkpoint` retains prefix1 receipt SHA `ad64a0e6124dee19e3bd7dc3b80603a1520daae2072bbf21955cc3bdfba7edb1` and prefix2 receipt SHA `e8b6c368bb75c07812890abd2e2c10bf919e962a80d03406db3585801aaa4087`. Every row log authenticates against its receipt SHA. Prefix1 native/binding all-target check exited0. Its original F159 expectation failed with exit101 and remains failed evidence. Prefix2 exact F159 golden and pixel selector passed1, failed0 and ignored0. Its actual combined package measurement and hash check exited0, with all49 entries matching. All five owned groups record natural cleanup, ESRCH and safe release, with source-after equality true.

The retained combined archive SHA is `b6506876760b37bb998e47c4aa4798b1bfa8d25620bfe7bdf8cab57cbe6443e2`. All37 member hashes and lengths agree with the measurement inventory. Applying the existing VCS normalizer, which removes the dirty field and zeros the commit identity only for normalized member-size accounting, reproduces exactly `(3609198, 51230290, 37)`. This was a no-verify combined-source measurement, not new verified publication credit. Its source map is `4296b4db5ddb1cf61f3b9a17672e467d88a1ccc123771bb79e8e0ffb2e8261a0`.

Final metadata-policy successors and full sprint verification remain due. Earlier worker gates retain their existing source-qualified authority and are not re-awarded here. This reviewer ran no product, Cargo or acceptance command and changed no canonical source or documentation.
