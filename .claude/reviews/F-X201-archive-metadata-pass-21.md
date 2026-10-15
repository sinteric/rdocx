# F-X201, archive metadata, pass 21

**Reviewed**: Four existing measurement carriers, three README rows and three tuple replacements. Immutable proposal178147133364ebe6b234dace082c8ea55e90a26f80e93f656487e5aefec181e6, patch ef5ce293c5fd83cfe5ba8fe269c80a8a207e63267e96421a4a6125a92d160d1a.
**Verdict**: 0 defects, 0 smells, 0 nitpicks

## Assessment

All proposal before and candidate hashes authenticate. Three retained verified archives independently match archive hashes, every recorded tar member hash, compressed size, normalized VCS member bytes and member count. Tuples are rdocx-oxml482692/2989568/32, rdocx-layout322015/1776016/15 and rdocx3607968/51224296/37. All are below10MiB. The source archive evidence00cad9bf and four-row verified publish receipt2229c441 are separate from the pending metadata-only package successors.

README edits at README.md:42, crates/rdocx-oxml/README.md:20 and crates/rdocx-layout/README.md:21 retain exact source lengths and recorded dates and performance observations. scripts/readme_doctests.py:405 updates only the same three actual archive tuples. The nineteen other tuples remain untouched. Existing measurement normalization at scripts/readme_doctests.py:1292 and compression tolerance at:1337 remain unchanged. Pending final metadata remeasurement and policy execution receive no pass credit here.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

Contract, measurement provenance, test policy and preservation checks found no issue. The proposal introduces no new product file, dependency, abstraction or behavior. Production implementation and native render qualification remain separate prior reviews.
