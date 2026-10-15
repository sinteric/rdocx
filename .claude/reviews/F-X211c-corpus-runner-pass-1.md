# F-X211c, corpus runner, pass 1

**Reviewed**: external unexecuted R3 derivative abccfee7789e7d6bfeaeafbd0dd8d4077e70083790d0ed440809b8484ce89909, checkpoint082a4b02878c0c58f72cb50fc357e308d54668897e26ad5b9308e6b9eaa359e8 against retained original194a3b395474aa00b54dd86981a4f657ed3bb6172dee1f14a10549068eeb5561.
**Verdict**: 2 defects, 0 smells, 0 nitpicks. No execution admission.

## Defects

### D1, Installed runtime membership and actual import origin are not authenticated

`/private/tmp/S91-F-X211c-final-corpus-preparation-1/runner.py:112`, `:148` and `:151` through `:159`

The environment guard authenticates only the listed installed_files and pillow_files keys. It does not require the actual distribution/package pathset to equal the admitted complete inventory. Wheel comparison omits RECORD and all other dist-info members except METADATA and WHEEL. The probe authenticates distribution metadata/root but never imports or locates the actual Python and native binding modules, despite importing importlib.util. An extra installed module or startup .pth file can redirect actual execution while every selected file hash, package version and metadata root remains unchanged. The runner can consequently record an attempt against a different native producer than the admitted one. Complete pair member/RECORD/current native origin guards are explicit prerequisites of this final measurement. Authenticate the admitted complete inventory, wheel RECORD/member proof and actual imported module/native origins before and after cases. Retain the existing full wheel hash and separately reviewed source/build/install receipts rather than treating metadata lookup as runtime-origin proof.

### D2, Actual external inputs and Poppler are guarded only once

`/private/tmp/S91-F-X211c-final-corpus-preparation-1/runner.py:165` through `:170` and `:217` through `:226`

Preflight authenticates actual external Arial/Calibri files, movie bytes and Poppler command once. Each case later calls only verify_corpus and verify_env. The freeze receipt inventories the external-input JSON metadata but not these actual external files. Frozen support.py:122 through :137 reads the Office font/movie paths directly, while :107 executes the external pdffonts command. A change to one such file between cases is therefore neither refused before the next case nor detected after the previous case. The unchanged metadata is not the consumed byte identity. Reuse one bounded external-input/tool verifier before and after every attempt and at final seal, including the actual command/version identity. Preserve input-only movie provenance and withhold final embedded/playable-output credit. No original capture or specimen rewrite is required.

## Not found

The working-copy source branch authenticates Base ancestry, current HEAD, staged path/mode/blob-origin membership, complete immutable source capsule membership and actual copied/live bytes. It does not substitute Base Git blobs for dirty implementation bytes. Both-family source/environment guard calls surround each case. The draft refuses approved_for_measurement false and cannot run with its null fields.

Fresh case directories prevent overwriting attempts, attempt_number is fixed1, no retry/refetch path is introduced, and W-XML-02/P-AREA-17 remain non-success in denominator100. Exit assertions remain distinct from the null full acceptance metric and pending native/movie obligations. Owned process groups have finite TERM/KILL cleanup, leader reap and group-absence proof, and no next case is admitted after unproved cleanup. No additional defect found in these bounded areas. Smells:0. Nitpicks:0.

## Evidence and boundary

No corpus, interpreter probe, build, application, Git mutation or fix ran during this review. Only this canonical review file was written. Runner, retained original, draft config and draft report packet hashes remained exact before and after review. Original R3 and its Base observations remain historical, no current metric or native qualification was manufactured. Review ended and returned control for a distinct remediation phase.
