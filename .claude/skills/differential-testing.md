---
description: How to compare this workspace against an external oracle. python-docx and python-pptx for the object model, LibreOffice for renders, and the rules that keep an oracle from becoming a second source of bugs.
---

# Skill: differential-testing

`differential` is one of the six test categories in
`docs/hld/12-testing-strategy.md`. It means comparing this workspace against an
external oracle rather than against a recorded expectation of our own.

Use it when the question is "does this match the thing we are a port of", and
use `golden` or `round-trip` when the question is "does this match what we
decided last week".

## The oracles

| Oracle | Answers | Comparison |
|---|---|---|
| `python-docx` | Does the WordprocessingML object model agree | Structural, over the parsed tree |
| `python-pptx` | Does the PresentationML object model agree, including inheritance resolution | Structural, over the parsed tree |
| LibreOffice headless | Does the render agree | Rasterise both and compare within a stated tolerance |
| PowerPoint or Word | Does the file open at all | Manual, recorded as performed or not performed |

The first two are the ones that matter most. This workspace is a port, so the
oracle is the thing being ported.

## Rules

1. **Pin the oracle version and record it.** An unpinned oracle turns its
   upgrade into your regression, at a time you did not choose. The pin belongs
   in the test harness, not in a comment.

2. **Compare the tree, not the bytes, against `python-docx` and
   `python-pptx`.** Attribute order, prefix choice and whitespace are ours to
   decide. `docs/hld/04-opc-and-packaging.md` sets the serialisation contract,
   and a byte diff against a Python writer would fail on decisions we made
   deliberately.

3. **A render comparison needs a tolerance and a reason for it.** Rasterise
   both sides at a stated DPI, compare with a stated metric, and state why the
   threshold is where it is. "Close enough" is not a gate.

4. **Deterministic font mode is mandatory on our side of a render
   comparison.** System fonts differ between machines. See
   `docs/hld/15-build-and-toolchain.md`.

5. **A disagreement is not automatically our bug.** Triage it into one of
   three, and say which in the test or the finding:
   - We are wrong. Fix the code.
   - The oracle is wrong or is doing something out of scope. Record the
     divergence as intentional with a citation, and assert the divergence so a
     later change cannot silently drop it.
   - Both are defensible readings of ECMA-376. Decide, write the decision into
     the relevant `docs/hld/` document, and assert our side.

   The third case is the reason a differential test never just prints a diff.
   An unexplained known-difference list becomes a place bugs hide.

6. **Never make the oracle a build dependency of a published crate.** It is
   test infrastructure. Nothing under `crates/*/src` may reach for it.

7. **No binary fixture files.** `docs/hld/12-testing-strategy.md` holds this
   rule for the whole workspace. Construct inputs in code, including image
   headers with precomputed CRCs. The deck corpus is the one exception, it is
   fetched rather than committed, and it lives outside the published crates.

## Composition and discriminating fixtures

Internal consumer agreement is regression/integration evidence, not external
parity. Before an oracle run, compare the affected internal consumers on the
same input: evaluation, cached state, saved XML, layout/render, native facade,
binding and CLI as applicable. A shared parser or formatter can still receive
different context, defaults, selection, units or refusal policy. Trace those
caller decisions in the consumer map.

Plan a bounded risk-based matrix. Include meaningful zero/one/many collections,
implicit/explicit lookup, legacy/versioned behavior, nested/direct owners and
before/after update and reopen. Each decisive fixture names the plausible wrong
implementation it would reject. Prefer multilevel values where relative/full
context differ, known missing-root controls beside deferred empty items, and
fresh/held handles before reopening. Do not multiply trivial fixtures or demand
every Cartesian combination.

For cached projections, exercise a source edit that changes the cached value
while keeping its owner live or its revision unchanged, when such a publisher
exists. Compare the native value with held and fresh reads, then save/reopen.
Check every affected logical owner when one operation mirrors physical content.
Owner identity and successful serialization alone do not establish freshness.
Compare read and write coordinates on mixed controlled/direct content. An
immutable accessor and a mutable accessor can interpret the same ordinal
differently. Check the selected physical owner and its unchanged neighbors.

Observe the promised layer. A cached number cannot prove the positioned field
text. A reopened package cannot prove live in-memory alias validity. An
unchanged hash proves stability on those samples, not correctness of a new
feature. A reporter's self-catching success or contribution test count cannot
prove every acceptance row. Retain exact expected/actual results and actual
old-source RED/repaired GREEN outcomes. Use the same input and state when
comparing consumers, and record intentional differences explicitly.

For structure-changing APIs, cross applicable operations with destination
contexts. Creation passing its grammar check does not prove move, clone,
replace, unwrap or repetition reaches that check. Use a bounded set of legal
destinations and incompatible parent/payload pairs. Assert refusal without
changing source, destination, relationships or live owner state where promised.
A parser preserving unsupported children as opaque XML cannot certify that a
writer is allowed to introduce those children.

Challenge a repaired guard with a payload hidden one level deeper behind a
context-preserving wrapper. Carry the destination slot through that wrapper
until its meaningful content is checked. Also test legal nesting and elements
that change child context, so a blanket recursive rejection cannot pass. Check
all relevant users of the guard, including unwrap when it shares the policy.

Enumerate semantic publishers, not just direct helper callers. Type conversion,
repetition and a copied emptiness predicate can reintroduce the same invalid
state after move/clone guards are repaired. Tie each applicable publication path
to a test in the existing consumer map. For event-based text rules, inspect all
variants of the pinned parser. Text, CDATA and GeneralRef are different events.
Compare equivalent literal, decimal/hexadecimal and predefined-reference forms
at the same grammar position. Preserve legal whitespace and text inside valid
child elements. Classify document-boundary positions separately. A successful
test of one representation does not establish the others.

For binding parity, call every promised operation through the actual public
binding, with its documented argument types, and inspect the installed result.
Native coverage and a correct stub cannot prove a Python method exists or that
its argument conversion reaches the native operation. Conversely, a runtime
call does not prove public re-exports, __all__, stubs or factory typing agree.
Add a small typed use for each changed API family and inspect visible defaults
where the binding generator displays Ellipsis. Keep real installed stubtest
and strict mypy outcomes distinct, without broad annotations or exclusions
that conceal a mismatch. Exercise the promised
nested owner contexts and at least one atomic refusal where applicable.

For APIs accepting XML bytes, separate tokenization from well-formedness,
namespace validation and schema validity. Include duplicate or malformed
attributes and relevant unbound-prefix cases beside valid raw-byte retention.
Inspect which parser options and attribute iterators enforce the requirement.
An accepted event stream or successful round-trip alone is insufficient.

Distinguish attribute tokenization from name/value validation. Where the
contract requires a well-formed element, use invalid-name, literal less-than
and undeclared-reference cases, such as `<root 1bad="x"/>`, `<root a="<"/>`
and `<root a="&bad;"/>`, under the approved entity/declaration policy. Include
valid escaped values and legal names as positive controls. A colon in a lexical
XML name does not itself require namespace binding. If the contract explicitly
retains `<root a:b="1"/>`, preserve that case. Enforcing namespaces requires its
own approved contract, not a silent side effect of repairing lexical validation.

## Compatibility facts and oracle provenance

When exposing existing imported data through a new API, enumerate the parser's
accepted variants and absence states first. Cover read, untouched preservation,
assignment and clearing where applicable. Do not derive the reader's entire
domain from the new setter's accepted values. Include a discriminator for each
different projection or preservation rule, using bounded cases rather than
repeating equivalent fixtures. For unit conversions, record both the stored
value and its public value so a correctly typed but incorrectly scaled getter
cannot pass.

Before claiming parity, pin the primary reference version and exact source
file/function or schema clause. Trace the specific getter, setter and collection
class. Similar names do not imply identical ordering or conversion behavior.
For numeric conversions, derive discriminating examples from the actual formula:
ties, sign, scale and boundary rejection. Execute a narrow reference probe when
source inspection leaves ambiguity. Do not ask the user to decide an answerable
fact. An intentional departure is a separate documented product decision.

For native-produced comparison evidence, retain the producer and version,
input identity, output identity, production steps and exact observation being
compared. Record hashes and retrieval locations consistent with the existing
no-binary-fixture policy. An existing fixture qualifies only when its origin
and coverage of the promised behavior can be established. Do not invent a
producer version or call this library's own generated output a Word fixture.
Self-generated inputs are useful local regressions, but cannot establish native
output parity. Schema-valid child names alone do not prove application behavior.

Before declaring an oracle bundle unavailable, check its documented independent
producer. HLD15 and CI prepare F-X195 inputs with
`crates/rdocx-py/tests/test_rendering_threads.py --prepare-fx195-inputs` and the
selected `--family`. Use that pinned tool and provenance path when available.
A preparation failure stays a failure. It does not authorize synthetic expected
outputs or removal of the qualification gate.

If the promised oracle or private unreduced input is unavailable, keep that
acceptance row pending. Reduced examples qualify only their stated scope.
Do not replace a native gate with internal round-trip success, restrict the
supported formats to those convenient to test, or mark the full issue resolved.
Any agreed scope change must update the acceptance contract explicitly.

## Where it applies

Differential tests are worth their cost where a port can be subtly wrong rather
than obviously broken:

- Inheritance resolution from slide to layout to master to theme. See
  `docs/hld/07-inheritance-and-resolution.md`. A wrong answer here is a
  plausible-looking wrong colour, not a crash.
- Unit conversion at boundaries, where our constructors truncate deliberately.
- Preset geometry, against the checked-in table `tools/gen-presets/` produced.
- Placeholder inheritance and the empty-placeholder rules.

They are not worth it where we have already decided to differ, such as the
serialisation details in rule 2.

## Relationship to the hash harness

They answer different questions and neither substitutes for the other.

- The **hash harness** asks "did our own output change". It gates every PR in
  M1 through M6 and it has no opinion about correctness.
- A **differential test** asks "is our output right". It runs where it is
  declared in a design plan's test plan.

A harness delta plus a green differential test means output moved toward the
oracle, which is still a delta and still needs declaring.

## Related

- `docs/hld/12-testing-strategy.md`, which defines the six categories.
- `.claude/commands/differential.md`, which runs the corpus.
- `.claude/skills/risk-routing.md`, the oracle-pinning row.

## Execution timing

Pin and prepare applicable oracle inputs during design. Run external oracle
comparisons once in the final integrated sprint union, not per story or again
through completion subcommands. Local implementation uses named in-repo
RED/GREEN regression controls. Pending oracle qualification is recorded in the
existing plan and handoff, never called passed before actual execution. Reuse
only actual observations with unchanged source, tools, inputs and configuration.
