# F-X210, asset registration proposal, pass2

**Reviewed**: External registration and existing NOTICE-Noto recipe proposal, five font entries and the exact nine approved asset inputs. Approved design, bounded metrics and prior Base-test pass1 were read. This is a distinct review after F-X200 attribute pass48 ended.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication

Receipt SHA256ddb4e0abf20e59ce0c80568d762238b0d817e400a69be3c88ab2ccaf7b538012. Registration diff SHA256d149809e473bfd4c131d32e30b65bc0123da1cb4479906bdcc02051a83cb994b. Candidate bundled_fonts.rs SHA256c772373775d251b3e985da56b081d8b135e44ff54b27ce82d5414391222cf659. NOTICE candidate SHA256e1984046309a0d0d4d4f7c940792f9b72e9c6ac221ce654f2910515d6dad318f and recipe diff b894ac126b9ff462789a03a023ea307bb9618957d18a89a2155f1743f13de9b8 match sealed artifacts.

All2900 actual current source byte, size and mode guards match start map a68e14feeeb908d07808a20c9b80e582a9e008f3bfc90da4f48d859015c6f861. All nine retained input hashes and sizes match the approved inventory, total444812 bytes. This includes the selected3968-byte subset a95e7d7421e30e8ad304a0eb48aa18aefabcfec568a0f652c1039d72da999373. No input is the superseded3916-byte proposal. Registration has not been adopted.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Six aspects

- Correctness: zero findings. Candidate crates/oxml-layout/src/bundled_fonts.rs:73 registers original upright and italic Gelasio variable faces under their genuine family. Candidate :82 registers static Selawik Regular and Bold without a fake italic entry. Candidate :90 registers only the approved Noto Sans Symbols2 subset. This adds five entries to27 existing entries, total32 faces across15 distinct families. Original font bytes and entries remain unchanged.
- Contract: zero findings. Registration does not alter selected coordinates, synthesize asset bytes, claim universal Georgia or Segoe metric parity, or rewrite symbol characters. Module documentation describes bounded substitutes. Font-manager selection mappings remain explicitly outside this proposal and require their reserved source phase. Genuine caller/embedded precedence remains the provider contract.
- Panics: zero findings. Production adds only compile-time include_bytes entries. No new parsing, indexing or runtime path. Missing approved files before adoption is a preparation boundary, not valid semantic RED.
- OOXML: zero findings. No OOXML model, namespace, child order or source serialization changes. Symbol coverage is not represented as original Wingdings outline or advance parity.
- Tests: zero findings. The full existing Base-test suffix is byte-exact. The existing license set now includes Gelasio, Selawik and Noto Sans Symbols2 at candidate crates/oxml-layout/src/bundled_fonts.rs:184 and :195. Notice existence checks include both new core families. Original license inventory checks remain intact. Existing deterministic face-count assertions in font.rs:3450 and :4141 derive their expected count from bundled_font_data rather than hardcoding27. Genuine actual Base and later four-style GREEN remain separate runtime obligations.
- Structure: zero findings. One existing inventory carrier and existing NOTICE carrier. Nine asset paths are precisely the user-approved inventory. No new loader, downloader, module, dependency, alternate pack or duplicated font manager.

## Subset recipe and boundaries

Candidate crates/oxml-layout/fonts/NOTICE-Noto:23 records original source commit, full-source SHA, selected subset SHA, exact17-target repertoire, FontTools4.66.1 recipe and monochrome semantic qualification. The recipe preserves original naming and matches the retained independently reproduced proposal. Original Noto license text remains present, with font metadata retaining original source notices. The core license and notice bytes are copied only from authenticated approved inputs.

NOTICE-Noto needs ROOT's explicit existing-carrier lease extension before application, as the packet states. Both deciding Base controls must have genuine semantic outcomes before production registration or asset copy. The currently live controls supply no result to this review. Registration alone does not change Georgia or Segoe lookup and cannot finish their GREEN. Caller priority, metric/paint tests, Aptos diagnostics, qualified symbols, installed projections, packaging, hash and HLD remain due.

Only this report was written. No asset, source, Cargo, environment or runtime action occurred. Review ends and returns its lease to ROOT.
