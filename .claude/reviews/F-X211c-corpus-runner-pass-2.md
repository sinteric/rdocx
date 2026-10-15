# F-X211c, corpus runner guards, pass 2

**Reviewed**: checkpoint31f6d1c166a45759d4a17d99c53a91b06cb2f1b7a229ac779353b96e1db3a610, runner7d72dfe876ea30a75442796831e96b12c3d2de09b26c4ef149597431b84d36d6 against actual preimage abccfee7789e7d6bfeaeafbd0dd8d4077e70083790d0ed440809b8484ce89909.
**Verdict**: 0 defects, 0 smells, 0 nitpicks within the external guard remediation. No measurement admission.

## Prior findings resolved

D1 is resolved. The current runner's verify_runtime_files at /private/tmp/S91-F-X211c-final-corpus-preparation-1/runner.py:44 authenticates complete admitted physical package/metadata membership, refuses unexpected files and symlinks, and hashes admitted startup hooks and ancillary files. The wheel guard at :149 through :177 authenticates every wheel RECORD member and digest, all ordinary installed wheel members, the actual installed RECORD and only named installer metadata extras. Actual metadata inventory and imported Python/native origins are checked at :218 through :227, including matching module and spec native paths, ExtensionFileLoader, exact sys.path, Pillow membership and metadata roots. Admitted search directories, path-file inputs and pyvenv.cfg are required before probing. Both family guards remain before and after every case, preserving paired runtime identity.

D2 is resolved. verify_corpus now checks exact frozen physical membership and calls the actual external-input verifier at :14 through :22. That verifier at :35 through :43 authenticates actual Arial/Calibri and movie bytes and actual Poppler bytes, resolved path and exact version output. Existing per-case before/after verify_corpus calls therefore check consumed external files every time. The final seal also calls verify_corpus at :306. Movie validation remains explicitly input-only and does not manufacture final embedded/playable output acceptance.

## Defining controls and limits

The preserved actual RED log adf7fd372b1e3c7646264619503d18246a0bd710edcefaf893ed21f10fe8fade records ten controls with seven failures against the preceding guard behavior. Actual current guarded receipt f14d317762f402a62f4fafe1424ac0f8cd73bc3c2efe54eaebcef0ff52fc02ca authenticates one exit0 qualified safe row with ten passing controls. The controls independently reject unlisted runtime modules, unlisted startup .pth files, redirected native and Python origins, and actual consumed font/movie/Poppler drift. Positive complete package/wheel/environment and unchanged external fixtures remain distinguishing controls.

These controls use synthetic dummy files and mocked import/version probes. They do not execute the frozen specimens, CP native extension, real Office, media decoding or final environment. Runtime config remains approved_for_measurement false with null source/build/install/environment fields. Filling and independently authenticating that actual config is still required before a distinct execution admission.

## Not found

No additional bounded issue found in full dirty source capture/index/ancestry proof, paired per-case origin guards, frozen162-file membership, immutable100 bodies/support/allocation and input hashes, fresh first-attempt directories, finite owned process cleanup or refusal to start another case without safe cleanup. Original R3 remains preserved. No retry, hidden refetch, changed denominator, Boolean score or native/full acceptance credit is introduced. W-XML-02 and P-AREA-17 remain deliberate non-successes, and native/movie acceptance remains separately pending. Smells:0. Nitpicks:0.

Only this canonical review file was written. Runner, actual preimage, controls and draft-config hashes matched the checkpoint before and after review. No script, test, corpus, probe, build, Git operation, source change or application ran in this audit. Review ended and returned control.
