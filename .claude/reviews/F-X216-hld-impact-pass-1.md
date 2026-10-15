# F-X216, HLD impact, pass 1

**Reviewed**: Applied current-intent changes to exactly eight plan-listed HLD files, 225 added lines and 31 deleted lines, against worker ClaimBase and approved canonical plan `1a0dc6c6d4def67e9b455ef6ace104983f067858332edf2413243fb14d33b5b4`. Applied manifest SHA-256 is `f2e3fae70e879f3168757ebc07a1a854d0c1b1d401a9d1cab4f76638a65ff245`.

**Verdict**: 0 defects, 1 smell, 0 nitpicks.

## Defects

None.

## Smells

### S1, shape-property inheritance omits its retained-source refusal boundary

`docs/hld/05-drawingml-model.md:214`

The new shape-property inheritance contract describes namespace preservation
and conflicting owner metadata, but does not describe its distinct admission
check for unrepresentable metadata on modeled descendants. That refusal does
not require a conflict between local and inherited owners. For example, an
otherwise empty local owner and a lower solid-fill owner with an unsupported
foreign attribute refuse inheritance because the typed owning projection
cannot retain that source attribute. The reverse local case refuses too.

The actual mechanism retains original XML and its projection specifically for
this operation at `crates/oxml-drawing/src/shape_props.rs:279`, rejects edited
source with nested namespace authority at line 302, and compares original
metadata occupancy with the staged typed representation at line 328. Its
explicit local and lower-owner regression is
`crates/oxml-drawing/src/shape_props.rs:1317`. Ordinary shape-property equality
does not include these retained facts, as shown at line 122. The approved plan
explicitly names both the unrepresentable-descendant refusal and the ordinary
equality/serialization boundary.

Document those boundaries alongside the new shape-property operation. Keep
the existing style-materialization paragraph distinct, since its current
source-fact statement applies to shape style, not shape properties. This is a
documentation omission, not a new production defect or a request to broaden
the parser.

## Nitpicks

None.

## Checked current intent

The applied eight-file path set matches the canonical plan impact list. All
actual before and after hashes authenticate. The current worker plan hash is
`1ebac70fb2c01a1a96048f063fead358d2b2857545586715f8f34554545dc1b1`.
The first four implementation boxes are checked while native acceptance and
the final scoped completion boxes remain open. No completion claim is added
to the HLD mechanism prose.

The OPC equality and copying contract now describes complete physical
relationship roots, ordered records, opaque payload and effective namespace
context. It permits normalization only of graph-proven IDs and targets, and
retargets from the original owner to the actual new owner. These match whole
production pass 2 D1 and D2 and their actual scoped preservation provider.
Source-enabled embedded programs, retained font bytes, conflict refusals and
destination-resource preservation match the reviewed consumer activation
rules. No ordinary serializer change or fabricated license authority is
claimed.

The presentation and rendering sections describe one global notes master,
registration before publication, refusal when registration would change
existing unregistered notes, actual source placeholder occupancy, real local
text and fields, omitted absent master samples, skipped-placeholder reference
refusal and source-owned decorations. Notes without a local owner and handouts
retain distinct composition behavior. Source appearance resolves before style
neutralization, and source master or presentation-default resources retain
their actual scopes.

The DrawingML style preparation and neutralization contract retains current
qualified reference choices until direct appearance is staged, then clears
only the validated owned selections. Opaque payload, namespace authority,
legitimate modeled edits and ordinary style equality remain correctly scoped.
Character, paragraph and list inheritance document concrete local precedence
and atomic namespace-conflict refusal.

Python and CLI contracts retain existing positional single-slide calls and
the single Slide result, with explicit diagnostic and batch companions. Error
classes, negative indexing, caller order, prepared returned-object allocation,
foreign-owner refusal, one-based CLI selection and protected input paths match
the reviewed implementation. Testing prose keeps genuine current native
open/save/normal-reopen and source-format appearance evidence mandatory. The
effective helper zero review does not supply native acceptance.

## Backlog integration boundary

Independent byte comparison confirms that only the F-X216 block changes in
worker HLD14. Every unrelated byte, including its older F-X221 block, is
unchanged from ClaimBase. The canonical sprint already has the reviewed
F-X221 Word 0.16.1 and PowerPoint 0.14.1 patch-release contract. Integration
must retain that canonical block and apply only the F-X216 block. This known
branch divergence is not a defect in the scoped worker patch.

## Not found

No scope, dependency ownership, API-default, graph-preservation, notes-content,
font-activation or premature-acceptance defect found in this bounded HLD pass.
Whole production review `d99c3ba60f84139b7244aba14c537ab63903381bb4872675c1c0b0625c05aa5c`
and effective helper review `eeeff53e89fa72715ab5c551b5ed1a62caac885cbbe2b268feb1264afcc495e9`
authenticate. No source edits, worker writes, Git mutations, native UI actions,
builds or tests occurred. Only this review report was written. Native and
feature completion remain separately pending.
