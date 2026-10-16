# F-X247, Preserve the common LaTeX inputs reported in Issue365

**Status**: completed
**Sprint**: S94
**Size**: L
**Depends on**: F-X234, F-X235

Approved after S93 closed and S94 opened from main3a3b0de8be9f8bf4eb588a9a24c565f0af8ee190. Source citations were prepared against the unchanged runtime inputs of S93. No S94 runtime evidence is claimed by this design. Establish the exact feature Base when claiming its worktree.

## Problem

The exact Issue365 formula now preserves its supported symbols and script bases, but the follow-up adds a different, finite73-input contract. Existing LatexParser::parse_command recognizes a bounded subset at crates/rdocx/src/math.rs:2025. Unknown commands can leave detached scripts with explicit loss diagnostics. Current MathML text/style handling collapses several token roles at529, spacing only recognizes thin width at549, and non-Bar fraction export diagnoses/discards the type at1602 and2897. Existing native types already represent upright runs, text, no-bar fractions and delimiters. Current fraction layout explicitly paints Linear/Skewed as a slash sequence at crates/rdocx-layout/src/math.rs:1125, so using Linear for tfrac would not satisfy stacked-fraction acceptance.

The reporter's summary prints FAIL unconditionally. Product behavior cannot make that unchanged literal label disappear. Preserve its actual original output and qualify a separately identified summary-only correction, with independent semantic assertions that cannot pass merely because diagnostics were hidden.

## Spec reference

- docs/hld/14-development-backlog.md:8146, F-X247, including its differential gate and finite73-row acceptance.
- docs/hld/10-bindings-spec.md, native OfficeMath authoring and additive MathML/LaTeX conversion paragraphs at1641 and1700. Normalized argument semantics, ordered diagnostics and owner-bound document properties.
- docs/hld/08-rendering-spec.md, Word OfficeMath layout at2318. Deterministic font selection, glyph ink, OpenType MATH and bounded fallback.
- docs/hld/12-testing-strategy.md, rdocx MathML and LaTeX conversion at3008 and OfficeMath native qualification at1544. Independent semantic mutations and unchanged geometric/raster bounds.
- docs/hld/04-opc-and-packaging.md, package serialization, XML1.0 validity and retained unmodeled XML. docs/hld/06-presentationml-model.md, Preservation strategy at1231, as required by parser risk routing.
- docs/hld/15-build-and-toolchain.md, Packaging at335 and Native Office font consumption. Authentic caller bytes and verified archives.
- Authenticated reporter comment: https://github.com/tensorbee/rdocx/issues/365#issuecomment-6200740130. Original fetched JSON is /private/tmp/S93-close-live/365-comments.json, SHA2564e78dfbaec4dd74ae41f011e8817d416c0ba13f8553ffa756cfae011a6905bba.
- Private research inputs: /private/tmp/S94-F-X247-oracle-research.md and /private/tmp/S94-F-X247-fraction-style-scope-note.md. These are evidence inputs, not competing canonical plans.

## Approach

### Finite grammar and unchanged public surface

Keep the four native signatures unchanged:

```rust
pub fn equation_from_latex(input: &str) -> Result<MathConversionResult<MathArgument>>;
pub fn equation_from_mathml(input: &str) -> Result<MathConversionResult<MathArgument>>;
pub fn equation_to_latex(expression: &MathArgument) -> MathConversionResult<String>;
pub fn equation_to_mathml(expression: &MathArgument) -> MathConversionResult<String>;
```

Keep Python Paragraph.add_equation(source, *, format) and existing result/diagnostic types unchanged, at crates/rdocx-py/src/paragraph.rs:734. No new public method, expression variant, model field, trait, generic parameter, crate, feature flag, font asset or product file is proposed. Implement in existing math.rs carriers and existing test entrypoints. Use the existing OfficeMath oracle script/manifest with a separately named fx247 fixture. Preserve all historical fixture rows and identities.

Use two small private finite lookups in crates/rdocx/src/math.rs:

```rust
fn latex_symbol(command: &str) -> Option<char>;
fn latex_symbol_name(character: char) -> Option<&'static str>;
```

They map exactly the named symbol spellings in the73 inputs, retaining aliases le/leq, ge/geq, ne/neq and to/rightarrow. Reverse output chooses one documented spelling per equivalent character. Do not make the grammar depend on an external process or add a general TeX registry. Existing commands outside this increment keep their current behavior and diagnostics.

Add only this bounded scanner method if existing read_raw_group cannot preserve the required text correctly:

```rust
fn read_literal_group(&mut self) -> Result<String>;
```

It counts existing input/event/text/depth limits, preserves literal whitespace, handles the existing escaped literal brace/backslash forms, and refuses unsupported embedded commands or unmatched braces explicitly. It is used for the plain braced operatorname/mathrm/text operands needed here. It does not expand macros, load packages or interpret arbitrary TeX. Existing recursive mathematical groups remain owned by parse_required_group. A script following a command binds to that one resulting atom, preserving the F-X234 rule at crates/rdocx/src/math.rs:1928.

### Existing typed carriers

- Ordinary symbols remain MathExpression::Run(MathRun). Lowercase Greek keeps mathematical italic intent. Uppercase Gamma/Delta/Sigma/Omega and relation/operator symbols use existing MathStyle::Plain where required by independently observed native styling. Preserve existing pi/to/infty/lim normalized trees and their historical controls. Compare logical codepoints separately from optional writer styling defaults in the independent oracle.
- Named functions sin/log/ln/exp and operatorname use the existing upright-function convention normal=Some(true), style=None. Existing lim keeps this exact convention. A function name stays one run and one script base. Reverse built-ins use their named command, other plain upright names use operatorname{...}. Supported MathML normal mi plus its U+2061 application marker maps to this function carrier only in the proved function-name context. An out-of-context application marker remains diagnosed. Native OMML nor already exists in the supported run properties, with no invented metadata.
- mathrm uses style=Some(MathStyle::Plain), normal=None. Its bounded plain operand remains one upright mathematical run. Canonical reverse uses mathrm{...}. MathML export uses normal mi without an application marker. New function/operator carriers export normal mi followed by U+2061, so a normal mi with that proved marker reimports as a function while an unmarked normal mi reimports as a plain mathematical run. Existing lim import/export conventions remain a named historical exception and its existing exact tree stays unchanged. It is distinct from a normal-text run and from default mathematical italic letters.
- text uses normal=Some(true) plus style=Some(MathStyle::Plain), the existing nor+sty=p combination observed in Pandoc's native OMML text. Preserve the complete String, including trailing space in text{if }. Canonical reverse uses text{...} with bounded literal escaping. Do not use an opaque raw sibling as a hidden semantic marker. The source text and formatting fields at crates/rdocx-oxml/src/math.rs:475 already support this distinction.
- MathML text output uses mtext, not mi, for the text carrier. For a boundary space, use an explicit supported .333em boundary mspace with the non-whitespace mtext content. Import the proved mtext/boundary-mspace shape back to one normal text run containing the logical boundary space. The exact Pandoc text example contains both lexical boundary whitespace and an explicit .333em marker. Count that one rendered boundary once, rather than doubling it. Do not trim interior spaces or deduplicate arbitrary adjacent mspaces. Unknown whitespace/attribute patterns retain diagnostics. Explicitly test native xml:space=preserve behavior already written at crates/rdocx-oxml/src/math.rs:2088.
- binom is MathDelimiter::new("(", ")", vec![...]) around one MathFraction with fraction_type=FractionType::NoBar, empty delimiter separator and grow=true. Both types already exist at crates/rdocx-oxml/src/math.rs:634,664,1576. MathML uses stretchy parentheses and mfrac linethickness=0. LaTeX emits binom for this exact canonical shape. Other no-bar or delimiter topologies use the existing generic exports or retain explicit diagnostics.
- dfrac, tfrac and frac all construct the existing MathFraction::new(numerator, denominator), fraction_type=Bar. Canonical reverse is frac. They remain stacked bar fractions in both inline and display OfficeMath, never Linear/Skewed. This is an explicit OfficeMath normalization chosen to meet the reporter's actual criterion that dfrac/tfrac become fractions. Forced TeX display/text style is not preserved or claimed. No per-fraction style field is proposed solely to exceed that criterion.
- comma, semicolon and quad remain distinct spaces. Preserve current comma as U+2009/.167em. Adopt the observed bounded OfficeMath spacing approximations semicolon as U+2004 and quad as U+2001. Reverse these to semicolon and quad respectively. MathML import accepts the observed .278em semicolon and1.0em quad widths into those canonical characters. MathML export uses .278em for the semicolon token and1.0em for quad. It explicitly documents that native OMML uses a Unicode approximation, rather than asserting exact TeX5mu paint. Text boundary .333em remains a separate contextual pattern, so it cannot silently consume a semicolon token next to text. Literal text spaces remain owned by the text carrier, not mistaken for explicit mathematical semicolon spacing. Unknown widths and negative spacing retain existing diagnostics.

Existing MathArgument and MathFraction contain private preservation fields, so they are not externally constructible complete public struct literals. This draft nevertheless avoids adding fields. It does not infer that any future optional defaulted property would automatically require a source break. Any later proposed carrier change must show its concrete consumer need and actual compatibility impact first.

### Fraction style decision and independent authority

The reporter explicitly requests dfrac/tfrac to become fractions and binom to be a no-bar fraction in parentheses. He does not explicitly request preservation of forced TeX style. Microsoft primary documentation lists dfrac/tfrac and says explicit displaystyle/textstyle/scriptstyle/scriptscriptstyle commands do not affect imported OMML. That supports investigating an OfficeMath normalization, but does not prove Word imports dfrac and tfrac identically. This design claims no such observation. Reference: https://learn.microsoft.com/en-us/office/math/latex, Fractions and binomials and Limitations. MathML support: https://learn.microsoft.com/en-us/office/math/mathml.

Actual pinned Pandoc3.11 MathML distinguishes dfrac/tfrac using mstyle displaystyle=true/false, but its DOCX writer collapses frac/dfrac to Bar and writes tfrac as Linear. Its annotation-free reverse loses both forced styles and the binomial no-bar distinction. Therefore copy neither the slash mapping nor a lexical reverse expectation into acceptance. The approved comparison is normalized OfficeMath structure: three stacked Bar fractions with identical operands, and separately measured native rendering of the saved canonical DOCX. mstyle wrappers encountered in this exact fixture normalize by the documented OfficeMath policy. Other unsupported style properties remain diagnosed. Exact TeX forced-style fidelity is outside this bounded increment and is not advertised by the API or HLD.

Fresh native controls must independently open/export the product-authored canonical DOCX containing frac/dfrac/tfrac, then measure clear stacked bars, numerator/denominator ownership and separation. This qualifies the chosen OfficeMath representation against Word, not Word's LaTeX importer. If native normal save/reopen or render refuses the canonical Bar controls, the gate fails. Direct Word LaTeX-import style identity remains unclaimed and is not necessary to the reporter's stated fraction criterion.

### Glyph and layout scope

Reuse deterministic caller-font math layout. The integrator approved a bounded revision after actual Word controls showed missing Greek advances and relation glue. Extend the finite covered mathematical-italic Greek mapping in crates/rdocx-layout/src/math.rs and apply the selected face's actual MATH italic correction at true ordinary-math atom boundaries for covered mapped Latin and Greek glyphs. Boundaries are an operator, explicit math space or run end. Do not add inter-letter spacing to identifiers or functions, do not apply mathematical corrections to upright or normal-text runs, and preserve pi's historical handling. Add conventional medium4mu binary and thick5mu relation glue for only the newly admitted finite operator characters, in the existing layout_text carrier. Preserve old pi and ASCII behavior and retain their old native controls. Logical strings, public types, dependencies and files remain unchanged. Full mapped-run cmap coverage remains mandatory. phi U+03D5 and varphi U+03C6 remain distinct.

Qualify actual native and product glyph advances and the selected MATH face independently. Use authentic installed Cambria/Cambria Math bytes with face indices, file hashes and producer pins. Do not install or redistribute fonts. Native matched-font qualification and bundled deterministic fallback remain separate controls. Missing glyph/MATH coverage keeps explicit stable diagnostics or the documented bounded fallback.

The integrator also approved clamping the MathLeading shift of generically run-only display equations at a lower bound of zero, retaining its existing upper bound. Empty and multiple run lists, upright text and spacing use the same structural rule. Mixed, fraction, script, radical, n-ary and delimiter frames retain their existing adjustment. The rejected global clamp moved actual binomial and canonical-fraction controls downward and remains captured in the private failed-attempt evidence. Actual short-run ink lies below MathLeading within the font ascender, so a negative adjustment incorrectly moves its baseline upward. Independently measured PDF baselines show the cause, rather than a fixture-specific coordinate correction. A regression must prove short displays never move upward while tall constructs retain positive leading. Existing six-case and per-expression controls remain mandatory.

For a Bar fraction whose selected physical face supplies MATH constants, operand advances and actual italic corrections own the horizontal width. The integrator approved removing the synthetic .06em-per-side fallback padding from this path after independent native PDF rule spans and consecutive numerator positions exposed the discrepancy. NoBar stack/delimiter clearance and the non-MATH fallback retain their original .06em-per-side padding. The rejected both-zero attempt failed actual binomial width qualification and remains retained. Apply the Bar rule generically to physical MATH faces, including nested and canonical Bar fractions, without font-name or fixture exceptions. Retain failed predecessor receipts and verify all old and new controls.

A physical MATH Bar whose direct numerator or denominator expression sequence contains a Bar fraction receives2mu clearance per side. MathArgument sequences are transparent grouping boundaries. Do not descend into scripts, radicals, delimiters or other structural children for this classification. The independently captured old nested-fraction native PDF has inner/outer bar widths6.96/9.36points, while removing all nested clearance made the current outer bar7.046875points. The integrator approved2mu as this bounded observed normalization, not an invented OpenType MATH constant or a universal Office claim. Ordinary glyph/radical/script operands remain zero-padding, while NoBar and non-MATH retain their previous clearance.

Both PDF font-box and actual raster-ink geometry remain at1.0point, with150DPI and0.99 block-luminance SSIM. Actual Word PDF font-size quantization is evidence to investigate, not permission to remove a failing criterion. Any normalization must first show independent physical font-size, glyph metrics and baseline evidence and receive review. Never calibrate product coordinates to fixture bounds.

Remediation controls retain one physical italic correction at a script attachment, with explicit private measured-box accounting for an already-applied terminal advance. Ordinary identifiers gain no internal correction. Superscript, subscript and combined controls use authentic positive Cambria MATH values for x, f and alpha. Normal text and pi keep their existing treatment. Completed MathML trees are validated after contextual boundary-space synthesis, including exact at-limit and one-byte-over-limit cases. semantics wrappers recognize supported mstyle content before canonical stacked-fraction normalization, independently of annotation order.

### Finite73 acceptance map

The ordered source list is /private/tmp/S93-Issue365-73-inputs.json. Its compact UTF8 JSON list SHA256 is7288e75fd28f588e1af81625ea762e1a9d0a01362027293a63827f222f091eba. Keep all73 strings and order byte-for-byte in the authenticated original script. The table gives required normalized semantics, not permissive substring checks. R means a logical run, U an upright mathematical run, T a normal-text run retaining whitespace. Structural/script expectations refer to the existing concrete types, not a second product AST.

| Row | Exact input | Required normalized semantic value |
|---|---|---|
| 1 | `\alpha + \beta` | R(α), plus, R(β) |
| 2 | `\pi r^2` | R(π) then Superscript(base=r,script=2) |
| 3 | `\theta` | R(θ), U+03B8 |
| 4 | `\lambda` | R(λ), U+03BB |
| 5 | `\mu` | R(μ), U+03BC |
| 6 | `\sigma` | R(σ), U+03C3 |
| 7 | `\omega` | R(ω), U+03C9 |
| 8 | `\varepsilon` | R(ε), U+03B5 |
| 9 | `\phi` | R(ϕ), U+03D5 |
| 10 | `\varphi` | R(φ), U+03C6 |
| 11 | `\Delta x` | U(Δ), U+0394, then x |
| 12 | `\Sigma` | U(Σ), U+03A3 |
| 13 | `\Omega` | U(Ω), U+03A9 |
| 14 | `\Gamma` | U(Γ), U+0393 |
| 15 | `a \le b` | Original left/right operands with symbol ≤ U+2264 |
| 16 | `a \leq b` | Original left/right operands with symbol ≤ U+2264, same semantic alias as row15 |
| 17 | `a \ge b` | Original left/right operands with symbol ≥ U+2265 |
| 18 | `a \geq b` | Original left/right operands with symbol ≥ U+2265, same semantic alias as row17 |
| 19 | `a \ne b` | Original left/right operands with symbol ≠ U+2260 |
| 20 | `a \neq b` | Original left/right operands with symbol ≠ U+2260, same semantic alias as row19 |
| 21 | `a \approx b` | Original left/right operands with symbol ≈ U+2248 |
| 22 | `a \equiv b` | Original left/right operands with symbol ≡ U+2261 |
| 23 | `a \sim b` | Original left/right operands with symbol ∼ U+223C |
| 24 | `a \propto b` | Original left/right operands with symbol ∝ U+221D |
| 25 | `x \in A` | Original left/right operands with symbol ∈ U+2208 |
| 26 | `A \subset B` | Original left/right operands with symbol ⊂ U+2282 |
| 27 | `a \times b` | Original left/right operands with symbol × U+00D7 |
| 28 | `a \cdot b` | Original left/right operands with symbol ⋅ U+22C5 |
| 29 | `a \div b` | Original left/right operands with symbol ÷ U+00F7 |
| 30 | `a \pm b` | Original left/right operands with symbol ± U+00B1 |
| 31 | `a \mp b` | Original left/right operands with symbol ∓ U+2213 |
| 32 | `A \cup B` | Original left/right operands with symbol ∪ U+222A |
| 33 | `A \cap B` | Original left/right operands with symbol ∩ U+2229 |
| 34 | `a \to b` | Original left/right operands with symbol → U+2192 |
| 35 | `a \rightarrow b` | Original left/right operands with symbol → U+2192, same semantic alias as row34 |
| 36 | `a \Rightarrow b` | Original left/right operands with symbol ⇒ U+21D2 |
| 37 | `a \leftarrow b` | Original left/right operands with symbol ← U+2190 |
| 38 | `\infty` | ∞, U+221E |
| 39 | `\partial f` | ∂ then f, U+2202 |
| 40 | `\nabla f` | ∇ then f, U+2207 |
| 41 | `\forall x` | ∀ then x, U+2200 |
| 42 | `\exists x` | ∃ then x, U+2203 |
| 43 | `\degree` | degree symbol °, U+00B0, independent native/symbol authority because Pandoc rejects this command |
| 44 | `\ldots` | ellipsis …, U+2026 |
| 45 | `\cdots` | centered ellipsis ⋯, U+22EF |
| 46 | `\lim_{x \to 0} x` | upright lim with Subscript(base=lim,script=x→0), then x, preserving existing F-X234 tree |
| 47 | `\sin x` | upright function sin, then x |
| 48 | `\log x` | upright function log, then x |
| 49 | `\ln x` | upright function ln, then x |
| 50 | `\exp x` | upright function exp, then x |
| 51 | `\operatorname{tr} A` | one upright operatorname run tr, then A |
| 52 | `\mathrm{d}x` | one plain mathematical run d, then default mathematical x |
| 53 | `\text{if } x` | T("if "), trailing U+0020 retained, then x |
| 54 | `\binom{n}{k}` | Delimiter(parentheses,grow=true) containing NoBar fraction(n,k) |
| 55 | `\dfrac{1}{2}` | Bar fraction(1,2), explicit canonicalization of dfrac |
| 56 | `\tfrac{1}{2}` | Bar fraction(1,2), explicit canonicalization of tfrac, no slash |
| 57 | `a\quad b` | a, U+2001 explicit quad, b |
| 58 | `\frac{1}{2}` | Bar fraction(1,2) |
| 59 | `\sqrt{2}` | Radical(base=2,hidden degree) |
| 60 | `\sqrt[3]{x}` | Radical(degree=3,base=x) |
| 61 | `\sum_{i=1}^{n} i` | Nary sum, lower i=1, upper n, base i |
| 62 | `\prod_{i=1}^{n} i` | Nary product, lower i=1, upper n, base i |
| 63 | `\int_0^1 x\,dx` | Nary integral, bounds0/1, base x followed by U+2009 and dx |
| 64 | `\oint` | Nary contour integral ∮, retained empty operands |
| 65 | `\left( x \right)` | Delimiter(parentheses) containing x |
| 66 | `\overline{x}` | overline accent applied to x |
| 67 | `\hat{x}` | hat accent applied to x |
| 68 | `\bar{x}` | bar accent applied to x, preserved distinct admitted spelling as appropriate |
| 69 | `\vec{v}` | vector accent applied to v |
| 70 | `\dot{x}` | dot accent applied to x |
| 71 | `a\;b` | a, U+2004 explicit bounded semicolon approximation, b |
| 72 | `M = \frac{4\pi^{2} a^{3}}{G T^{2}}` | M equals Bar fraction: numerator4 × Superscript(π,2) × Superscript(a,3), denominator G × Superscript(T,2) |
| 73 | `M = \frac{4π^{2} a^{3}}{G T^{2}}` | Identical normalized tree to row72 using original Unicode π input |


### RED, observer and evidence sequence

1. At the eventual clean S94 base, preserve the original fetched comment JSON, extract its fenced reproduction verbatim and record raw-script SHA256. Validate the extracted CASES array against the73-list digest above, without rewriting source or order. Run that exact script in a fresh installed candidate-source Word wheel environment before product edits, capture exit/stdout/stderr, the actual lost list and generated fractions.docx. Baseline may already pass the S93 formula rows. Do not copy the reporter's release58/73 loss count as a fresh result.
2. Preserve a second observer whose only reproduction-script change makes the unconditional summary prefix depend on lost count. Keep both M-formula prints/assertions, the full73 CASES and the three original fraction sources unchanged. Record the exact one-line diff and distinct SHA256. Its acceptance is zero actual losses and no FAIL, not merely exit0.
3. A separate independent ZIP/XML semantic observer inspects every selected inserted equation after DOCX save/reopen and exact normalized source values. It uses hand-authored expected codepoints/properties/topology from the table, not production conversion, export, AST normalization or copied diagnostic suppression. Assert all73 rows, not just the two printed mass formulas. Bind each result to input index, source string, wheel/source SHA, selected OMML and complete diagnostic list.
4. Run actual pinned Pandoc3.11 forward MathML and DOCX and annotation-free reverse for all73 rows. Preserve actual stdout/stderr/XML and independently classify only declared canonicalization differences. Actual pre-read evidence exists under /private/tmp/S94-F-X247-oracle-inputs/complete/pandoc-receipt.json. It has72/73 emitted MathML/OMML per context. Row43 degree exits0 with warning/raw TeX, so use independently authored Unicode expectation plus native normal-save/render for that row. Never replace degree input to manufacture73 Pandoc successes. Never count synthetic placeholder files as oracle output.
5. Execute the public native and freshly installed Python conversion/save/reopen routes against the same finite inputs. Require zero unexpected conversion diagnostics. Preserve opaque siblings byte-for-byte, original item boundaries and old S93 exact-formula behavior. Keep glyph/font render diagnostics separate and truthful.
6. For changed paint, create source-built native controls for variant Greek, relations/arrows/degree, functions versus mathrm/text, boundary spaces, thin/semicolon/quad spacing, binomial frame, and canonical frac/dfrac/tfrac. Pin actual Word/Poppler/app/font/source/PDF identities, native normal save/reopen and exact observation methods before claiming passes. Capture missing native evidence as pending, not a self-render substitute.

## Rejected alternatives

- Map tfrac to FractionType::Linear because Pandoc writes it. Existing renderer produces a slash, contrary to the required stacked fraction.
- Add a public forced-style field before establishing a required semantic distinction. The actual reporter requires fractions and an explicit documented OfficeMath canonicalization meets that bounded criterion.
- Claim Microsoft documentation proves identical Word dfrac/tfrac imports. It documents ignored explicit style commands, which supports only a narrower inference.
- Declare all73 passed because Pandoc exit0 or the original script exits0. Degree has a real warning/raw-TeX fallback, and the script's FAIL summary is unconditional.
- Use source annotation echo, token presence, product self-normalization or absence of diagnostics as the sole oracle. They can all hide wrong script bases, fraction operands or text spacing.
- Add general TeX support, macro expansion, new product files or an external production converter. None is required by the finite contract.
- Hide logical text as opaque XML or merge all normal-looking run roles. Existing supported run properties can express this design while preserving meaningful normal text.


The supplementary script-boundary capture is a diagnostic, separate from the seven finite paint gates. The duplicate terminal/script italic advance has a genuine positive-font RED and GREEN regression and retained predecessor/current PDFs. Its correction reduces accumulated width by1.97656pt. The supplementary current-native comparison still fails derived width by1.142188pt. Existing script attachment does not apply physical MATH MathKern, which the actual Cambria table contains. Full MathKern implementation is outside this approved increment. Retain this failing diagnostic and do not advertise a supplementary native pass. Independent successor review must assess this bounded limitation against the finite contract.

The historical13-family raw native PDF and producer receipt were not located. Recorded-vector runtime and mutation checks are distinct from historical raw-PDF/raster replay. Fresh Word16.114 captures use the exact historical Caladea-request recipe and a separate explicit Cambria variant. Word substitutes Cambria in the Caladea-request export. That original-source product PDF is byte-identical between genuine baseline and current wheels under the same caller-font route. Current deterministic product geometry matches that route. The matched-Cambria variant retains a preexisting8.833383pt lower-bound discrepancy, while current width improves toward native. These observations do not establish full13-family matched-font fidelity or reproduce historical producer exports.

## Test plan

The **test gate** is **differential**, verbatim from F-X247. Existing test carriers are crates/rdocx/src/math.rs, crates/rdocx-oxml/src/math.rs, crates/rdocx-layout/src/math.rs, the existing rdocx regression entrypoint, crates/rdocx-py/tests/test_core.py and scripts/officemath_oracle_harness.py with its existing JSON manifest. Add modules/cases to existing integration entrypoints, no additional linked test binary.

| Category | Proposed test/control | Required assertion |
|---|---|---|
| regression | fx247_finite_symbol_table_and_case_aliases | Exact named finite mappings and reverse canonical spellings. Upper/lower Greek and phi variants are independently distinct. Case-changed/unknown commands retain explicit diagnostics. |
| regression | fx247_all73_have_exact_normalized_semantics | All original73 strings and order, exact table semantics, no unexpected diagnostics, separate script/operand ownership, no source annotation shortcuts. |
| regression | fx247_upright_roles_and_text_boundary_spaces | Named/operator functions, mathrm and text use the declared existing property combinations. Boundary and interior spaces survive. U+2061 outside a supported context is not silently dropped. |
| regression | fx247_canonical_stacked_fractions_and_binomial | frac/dfrac/tfrac all reopen as Bar with correct operands. binom reopens as NoBar in parentheses. Forced style limitation is asserted as intentional, and Linear remains slash behavior for existing authored content. |
| regression | fx247_spacing_is_distinct_and_round_trips | Thin/semicolon/quad differ in logical codepoint, canonical MathML width policy and reverse spelling. Native advances are measured, text space is not doubled or folded into a math-space command. |
| round-trip | fx247_public_native_and_python_equation_reopen | All73 selected equations survive native and fresh installed Python save/reopen, opaque siblings remain exact, unrelated parts/item order remain unchanged. |
| differential | fx247_actual_pandoc_3_11_finite_matrix | Actual binary identity and full per-row artifacts.72 emitted cases versus independently retained degree failure. Declared normalized style/spacing divergences have assertions, not an unexplained known-difference list. |
| differential | fx247_native_word_matched_font_paint | Independently measured native source and product geometry/raster for changed glyph/text/space/binomial/fraction paint. Existing1.0point geometry,150DPI and0.99 block-luminance bounds where applicable. Diagnose fonts before comparisons. No threshold relaxation. |
| regression | fx247_semantic_observer_mutation_sensitivity | Actual mutated saved OMML swaps fraction operands, turns binom to Bar, changes phi to varphi, detaches pi² from its base, changes upright text to italic, removes trailing text space and collapses semicolon/quad to thin. Each specific observer fails for its actual semantic reason. |
| regression | fx247_rendered_mutation_sensitivity | Actual rendered group/ink change moves selected control more than1point or overlaps numerator/rule/denominator. Composite native gate must fail, with signed residuals disclosed. Do not assume every case or SSIM alone rejects1.01points. |
| regression | fx247_converter_resource_caps_and_atomic_refusal | Keep byte/event/depth/node/row/column/text/diagnostic/output caps at crates/rdocx/src/math.rs:19. Added text scanners and aliases cannot bypass caps, create invalid XML or publish a partial equation on failure. |
| regression | existing F-X234/F-X235 and historical OfficeMath gates | Exact original formula, mass formula,13 historical expression families and original manifest/tool identities remain unchanged. Preserve actual historical mutation outcomes. |

Mutation tests alter real input packages or actual rendered output. A test that merely changes a helper's expected value, checks command spelling or mirrors the parser lookup is insufficient. The independent observer's expectations remain source-authored and pinned separately from the production matcher.

## HLD impact

- docs/hld/08-rendering-spec.md
- docs/hld/10-bindings-spec.md
- docs/hld/12-testing-strategy.md
- docs/hld/14-development-backlog.md
- docs/hld/15-build-and-toolchain.md

These files document current canonical fraction/text/spacing intent, native glyph/font evidence, public behavior and the finite completed boundary. They must not claim unrestricted TeX, exact forced-style fidelity or that Pandoc supports degree. This list is the future completion work list, not authorization for a second HLD record.

## Risk routing

- Parser/serializer. Read HLD04 and HLD06 Preservation strategy. Require prefix-tolerant math reads, fixed/schema-ordered writes, malformed/resource-boundary refusal and byte-identical capture_element/opaque-sibling round trips in the existing carriers.
- Layout/pagination/shaping. Read HLD08 Word OfficeMath layout. Deterministic bundled mode for baselines, authentic caller-font qualification for native controls, correct variant/face coverage and preserved logical text. Preserve existing geometry/raster thresholds and attribute any intentional paint change separately.
- External oracle. Apply canonical .claude/skills/differential-testing.md. Pin actual Pandoc3.11, current native Word and Poppler/font identities, exact source/output bytes, declared comparison normalization and real negative controls. Keep historical3.10, Word16.104/Poppler26.01 and S93 Word16.113.2/Poppler26.09 identities separate.
- Existing public behavior of published crates. Read HLD10 and structural rules. Add no public signature or field. Record pre-1.0 semantic expansion of bounded conversion and canonicalization. Run scoped denied-warning rustdoc, verified workspace publication dry-run and actual affected archive inventory/size assertions, preserving10MiB and64byte measurement policies. Metadata updates may touch existing README/catalog rows only when actual measurements require them and under an exclusive lease.
- Python/WASM consumers. Read HLD10. Fresh installed Word wheel tests and strict typing/stub agreement must cover the public existing add_equation path. Check both rdocx-wasm and rpptx-wasm graphs on wasm32-unknown-unknown. Final workspace tests exclude both Python extension crates. No new Python/WASM surface is implied.
- Units/spacing. Read HLD01 units and CLAUDE Things that are deliberately wrong. Preserve truncating unit constructors, finite dimensions and the documented OfficeMath approximation. No new generic unit layer.
- Dependency graph, bundled assets and new constructs: not matched by the proposed diff. No dependency, new font, asset, flag, trait, generic, crate, module or product file is planned. If implementation proves one necessary, revise and ask under the repository rule before adding it. No second implementer is invented.

Run focused cargo check on the actually changed crates while iterating, focused meaningful tests/oracles, independent zero-finding microscope and /verify --scoped F-X247 before worker preparation. The integrated S94 result runs /verify --full, union risk riders and sprint review once at final integration.

## Hash harness

Expected unchanged across all49 shared entries and the deterministic pixel manifest. Existing generated sample DOCX inputs contain no typed OfficeMath, as recorded by F-X235. New73-row/native controls are separate source-built fixtures. F-X247 has no baseline write lease. Any unexpected shared delta stops the gate and requires attribution, not rerecording.

Conversion outputs for newly admitted commands and explicit canonical fraction/text/spacing rules are intentional behavior changes. Record them in a separate labeled behavior commit and keep file moves out of it. Historical supported formula/manifest outcomes remain pinned. No source archive or policy threshold changes are anticipated. Measure actual affected archives before revising numeric catalog entries.

## Waves and shared resources

F-X234/F-X235 are completed dependencies. F-X247 can be designed alongside F-X238 through F-X241, but implementation scheduling must honor actual file leases after all five designs are approved.

F-X247 owns the three existing Rust math.rs carriers, current OfficeMath script/manifest and focused existing math regression modules. Its installed Python tests share test_core.py and paragraph.rs consumer code with F-X238 and potentially F-X241. F-X238/F-X239 already share formatting.rs/slide.rs and integrate serially. F-X240 signing and F-X241 forms may touch document.rs, lib.rs, Cargo feature policy, CLI and common tests/publication metadata. None grants F-X247 permission to modify those carriers incidentally.

Prefer a separate conflict-free math wave from Python parity integration if existing test_core.py or regression entrypoint ownership cannot be partitioned safely. Native Office export sessions, the hash baseline, shared HLD paragraphs, actual archive catalog measurements and installed wheel environments are exclusive resources. Use independent worktrees and output directories per worker, no simultaneous writes in one worktree or environment. Root integrates and updates ledgers once. Do not allocate a new test binary or a second corpus/tracker to manufacture parallelism.

## Implementation checklist

- [x] After S94 opens, reconcile this draft with all five canonical designs, confirm dependencies done and receive one consolidated approval.
- [x] Authenticate exact original comment/script/73 inputs and run fresh current-source RED, preserving original unconditional summary and output.
- [x] Record the one-line conditional-summary observer separately and independent semantic expectations/mutation controls.
- [x] Implement finite symbol/name/text/space/binomial/canonical Bar rules in existing carriers with unchanged caps and public signatures.
- [x] Qualify actual Pandoc72/73 observations and degree's independent authority without rewriting input or suppressing known losses elsewhere.
- [x] Execute public native and fresh installed Python save/reopen, opaque preservation, strict typing and WASM checks.
- [x] Capture actual native matched-font glyph/text/spacing/binomial/fraction controls and honest fallback diagnostics, preserving historical evidence.
- [x] Complete applicable publication/hash/risk riders, independent zero-finding microscope and scoped gate, then prepare structured handoff.

## Open questions

No unresolved blocking product question remains. The orchestrator approves the concrete approach and risk riders in this plan under the authorized S94 scope. Native and external controls are future gates until actually executed.
Existing carriers and explicit dfrac/tfrac normalization to stacked Bar fractions are approved under the reporter's bounded criterion. Forced TeX style is not claimed. Retain actual Pandoc limitations, original diagnostics, semantic mutation controls and future native font/paint qualification. Revise the plan if actual evidence disproves the representation.

## Approved sprint waves and exclusive resources

Wave1 runs F-X238 and F-X247 in separate worktrees. F-X238 owns Word parity tests and the existing binding naming/stub lanes, plus its PowerPoint XML-divergence pointer. F-X247 owns native math/conversion, the existing OfficeMath oracle, Word core math tests and the sole published-package numeric catalog lease in this wave. Neither edits the other's test entrypoint or source files.

Wave2 runs F-X239. Wave3 runs F-X241. Wave4 runs F-X240. These stories serialize because their actual edits share binding carriers and the source file scripts/readme_doctests.py for mandatory genuine archive measurements. F-X240 follows the completed form surface for public fill/sign integration tests. Operational source leases add no speculative backlog dependency.

Each worker updates exactly its listed HLD impact under feature-specific semantic section leases before its prepared handoff. The integrator coordinates shared HLD sections and numeric catalog ownership. Shared package observations use genuine archives, and combined observations run again after integration. Workers never update sprint delivery totals. The integrator alone writes AS_BUILT and SPRINT_TRACKER. Keep complete installed environments and target/build caches isolated or exclusively leased. The hash baseline remains exclusive, with no approved new baseline move.
