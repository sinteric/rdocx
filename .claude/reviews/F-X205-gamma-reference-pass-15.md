# F-X205 gamma reference, pass 15

**Reviewed**: Immutable integration-gamma-reference-proposal-3. One existing test carrier, one hunk, 4 additions and 2 deletions. No production changes.
**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Qualification

Manifest SHA `0562150541c630ff67b8e65cdb9bc8bca567b2fa4f4cc46f59f41fa6149a161c` and all five declared records authenticated. Candidate SHA `e8a5d179ce2dbb9faed947eaaae996c6451e661932e5a3dcd6c1d750826e94ea`, diff SHA `0815071f22da0c41bf3237aa03cf07883a4fafd20db39a8e6c52f405cca790a7`.

`/private/tmp/rdocx-f-x205-codex/.claude/scratch/F-X205-start-1/integration-gamma-reference-proposal-3/integration-candidate.rs:18465` independently expresses decreasing red as `1-t^1.875` and increasing blue as `1-(1-t)^1.875`. These follow endpoint interpolation with the channel-directed ratio in the frozen production helper, `reviewed-source-gamma-policy.rs:5`, and the pinned [Microsoft Open Specifications explanation](https://learn.microsoft.com/en-us/answers/questions/2248059/non-preset-a-tilerect-behaves-strange-in-case-of-g). This corrects my pass13 reference-formula approval. The old formula and its actual failure remain retained, with failure log SHA `47e0c5b521888495baccc5ce6c6f9079df10cb5e6c739727bc901edd219c397f`.

`integration-candidate.rs:18452` retains all 256 intervals, exact endpoint colors, both interval endpoints, zero green and the original `1.0e-4` tolerance. The complete diff leaves axis and rotation assertions unchanged. No count, threshold or acceptance predicate is relaxed.

## Findings

Defects: 0. Smells: 0. Nitpicks: 0.

## Not found

Correctness, contract, panics, OOXML, tests and structure: zero findings in this bounded correction. No source adoption, test execution or whole-feature acceptance is credited by this review. Original pass13 and failed execution remain historical evidence.
