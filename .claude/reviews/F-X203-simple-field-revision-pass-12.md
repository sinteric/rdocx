# F-X203, simple-field accepted revisions, pass 12

**Reviewed**: Immutable three-carrier correction to accepted simple-field projection, its existing HTML mechanism assertions and one native regression. This bounded pass excludes the remaining moving F203 implementation.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication

Receipt `6cbdf1d725af8d07376544bbe778e922f13006f6646c31ffe2b465092eff4ee9` at `/private/tmp/rdocx-f-x203-codex/.claude/scratch/F-X203-start/immutable-simple-field-revision-12/receipt.json` authenticates all twenty declared artifacts. Each of the three diffs reproduces its copied before and after bytes exactly. All three after carriers match the final3111-record source map `a79c74055495ce7f8c0831a48e4a9c5cd818f50674a072fd70ba253d6bafeda8`. HEAD and index recorded in the runtime receipts remain `dd7c627b534301c253615d30e35e8748b455a4d0` and `c203016ed951117303df8410bfd0993697878991ea37fce9a229fc1b0770f17d`.

Text diff SHA `238f5f8bb66ddd99f9114855e438659eb97abf5ed6f610495fcad92103bb8fc5`. HTML test diff `f839abd5fba8162a58b6b626f256cd0c36f5306286c4e0c585f30c767749bf65`. Native regression diff `c4e3878df7498692e691984af5fc6897115d7275f04a2742d6233f9370d7d400`.

## Semantic disposition

Copied `text-after.rs:6346` stops borrowing an unprojected paragraph solely because field-owner preservation is requested. At `:6423`, both projection modes resolve a simple field through the existing wrapper parser and recursive accepted-view provider. The default accepted mode therefore retains its previous semantics. The Markdown mode now reads accepted insertions, excludes deleted results and retains physical field boundaries using offsets before and after the recursively projected runs. Nested boundaries are translated by the current output-run offset. Existing source runs and XML remain unchanged, and the enclosing hyperlink association follows the existing simple-field projection behavior.

Copied `html-after.rs:493` replaces an assertion about a surviving internal RunContent::Field with assertions about nonempty valid boundary indices and equality to the existing accepted-text provider. This is consistent with the approved physical formatting-boundary contract rather than a promise to keep an internal raw Field in a read-only semantic projection. The complete original literal Markdown expectations and original serialized-source equality remain unchanged at `:498`. Those assertions still distinguish coalescing or whitespace regressions.

Copied `regression-after.rs:400409` adds a real fldSimple with accepted inserted bold text and deleted text between two separate bold runs. It asserts accepted document text, three independent Markdown emphasis spans, exact package preservation and matching saved/reopened text, Markdown and bytes. This is a deciding observable control for the omission, not an implementation-shaped boundary count alone.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: zero findings. Recursive accepted content and physical boundary offsets remain consistent.
- Contract: zero findings. Field formatting boundaries and accepted revision visibility are preserved through the shared semantic provider.
- Panics: zero findings. The changed projection adds no unchecked access or untrusted arithmetic beyond existing bounded output offsets.
- OOXML: zero findings. The projection is read-only and both original and new controls assert exact saved source.
- Tests: zero findings. Replaced mechanism assertions retain literal output and source equality, supplemented by a deciding native accepted/deleted-field regression.
- Structure: zero findings. No new file, module, dependency, trait, generic or forwarding wrapper.

## Actual outcomes and limits

All retained runtime row logs authenticate against their receipt hashes. Runtime2 receipt `8f6d1ccf75bf070f5113854b50f7b3f8bb80af3f932cff650755e21ce332153e` records native/binding check0 and the new control failing because its initial expected document text omitted the existing terminal newline. Runtime3 receipt `a774be40f3935f1a27b1cc33581177a14308c41c04a0d3143819f8ae45250f04` records the corrected new control1PASS, original exporter parity1PASS and HTML13PASS1FAIL at the obsolete raw-Field assertion. Runtime4 receipt `027401b220deb262aa3ac2357d7619b007abcc963cf5295deb57c14fe6ab4af3` records HTML14PASS0FAIL0IGNORE after the assertion correction. Original failures remain failures in the record.

Authenticated runtime source maps2,3,4 match the corresponding receipt digests. Relative to final source, map2 differs only in the two test carriers. Map3 differs only in the HTML test carrier. Thus the check and two successful native controls qualify the unchanged production projection, while the final HTML14 result qualifies the final mechanism assertion. They are a finite composite, not one fresh all-green execution. All recorded groups reached ESRCH safely, with source-after equality and safe lease release true.

No full regression, installed wheel renewal, complete F203 producer or whole-feature acceptance is claimed. This reviewer changed no source and executed no product or runtime gate.
