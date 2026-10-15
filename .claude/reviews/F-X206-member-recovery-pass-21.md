# F-X206, member recovery, pass21

**Reviewed**: Bindings-authored R1-to-R2 reconciliation, five existing carriers,
165 added and16 removed lines. The complete prepared recovery packet has ten
existing carriers. This is an incremental source review, not adoption or runtime
acceptance.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Independence and exact evidence

I authored the R1 recovery proposal. Independent pass18 reviewed that proposal
and reported two defects. This pass independently reviews the bindings-authored
R2 corrections against those findings and the approved recovery contract. It
is not an independent replacement review of my R1 implementation.

The approved F-X206 contract at `.claude/plans/F-X206-design.md:524` requires
member-local recovery, ordered valid siblings, a count before drawing eligibility,
actual owner diagnostics and unchanged fatal admission/backend failures. F-X206
remains in progress. The canonical workflow and microscope command govern this
review.

Packet: `/private/tmp/S91-F-X206-member-recovery-r2-prepared`.
Authenticated receipt SHA-256:
`7edf8cfc70da90291bd78951e42dde50d2ab5c27e350b91c8e76a0484eab5989`.
Authenticated R1-to-R2 diff SHA-256:
`2a21b081d66f4799342f6967cea274dfea695c76c72d7205007ea1129a0851e7`.
Authenticated complete diff SHA-256:
`2d51b414eb9eb01029f6430c7b581edd353cd215b20c97a8fc90750c708475c0`.
All ten current and candidate byte/size/mode manifest records matched. The
current whole2714-record source map matched with no exceptions, authenticated
against source-after-eof-root.json SHA-256
`e9e7d4be30058031cafeba358cfaf9751cdd36290335c27dce31d5eb130822b4`.

## Pass18 reconciliation

D1 is resolved in the candidate. At
`complete/crates/rpptx-layout/src/context.rs:640`, unplaced shape fill and line
fill now validate supported gradient stop colors without fabricated dimensions.
The early boundaries match the actual concrete-fill consumer at
`complete/crates/rpptx-layout/src/context.rs:1796`: rotation-independent,
flipped, tiled and path gradients stop before color consumption. A colorless
stop stops the scan before later colors, matching the placed consumer's early
unsupported result. Solid, shadow and existing member validation remain intact.
The candidate controls at context.rs:8104 and context.rs:8137 cover both semantic
color errors, fill/line ownership, placed and unplaced outcomes, canonical/alias/
default namespaces, ordered siblings, count, source equality and supported versus
unsupported boundaries. These assertions have not been compiled or executed.

D2 is resolved in the candidate. The timeline companion at
`complete/crates/rpptx-render/src/timeline.rs:67` emits the slide-qualified count,
ordered resolver owner facts and then actual shaping diagnostics. Index admission
still precedes rendering. The facade at `complete/crates/rpptx/src/lib.rs:4881`
consumes precisely that known owner prefix once, retains the prior incoming
owner/state then outgoing owner/state order, and preserves the remaining media,
shaping and composition diagnostics. It does not identify owners through message
text filtering or deduplication. Both incoming and outgoing slides use the same
companion. The direct companion and public two-slide save/reopen controls cover
fallback and omission, exact count-summary multiplicity and owner order. The
public fixture uses real malformed gradient owners and real valid siblings.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- correctness: No additional defect in the two reconciled paths or their ordering.
- contract: No new recovery category, fatal-error downgrade or count waiver.
- panics: No new untrusted indexing or unchecked unwrap in the R2 production delta.
- ooxml: No serializer or parser ownership change in the R2 production delta.
  Namespace and source-preservation assertions remain strict.
- tests: No observed static API/type mismatch in the new controls. The Base
  discriminator uses existing APIs and is field-free, so absence of the proposed
  count field is not its intended failure. Compilation, positive preconditions,
  exact test counts and genuine semantic RED qualification remain actual gates.
- structure: No new file, module, trait, dependency or generic recovery framework.

## Runtime and acceptance limits

No Cargo, compiler, installed interpreter, painter or native application ran in
this pass. Formatter parsing and AST checks recorded by the author are not type
checking or runtime evidence. The prepared resolver6, renderer3, public2, CLI1
and installed14 counts are proposals for actual counted execution, not passes.
Strict corpus count checks, fatal controls, fresh installed wheels, complete
F206 scope and intentional deterministic output attribution remain required.
No baseline, worker source or other tracked record was modified by this review.
