# F-X206, CLI fixtures, pass 83

**Reviewed**: R31 receipt1969ee23213fdc81ed4cbce9f36d7308bcdc777ee5c46e0041fc02f40490966f, one-carrier diff7cf3fea9ac3b87eb8f7bee5882b46b5ebf68a25b45aee9a99684f6e4cb92e7fd against sourcebcaee6bc6f96d0c25527c26c39e4aa3cbf2e16d45049bf777bf8083fb363db8f.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Defects

None.

## Smells

None.

## Nitpicks

None.

Actual worker `/private/tmp/rdocx-f-x206-codex/crates/rpptx-cli/src/main.rs:43` declares the existing --to conversion flag. Candidate integration.rs:2368 and :2379 correct only its two fixture operands. Success, output format, source preservation, diagnostics and fatal-output atomicity remain asserted.

Candidate integration.rs:1556 and :1570 updates only the two names required by approved native placeholder naming. All other exact JSON structure remains unchanged. No production source or assertion strength changes. This permits guarded test-carrier adoption, the two previously failed selectors and three unrun native scopes. It does not authorize replaying34 CLI passes or changing archive SHA expectations.
