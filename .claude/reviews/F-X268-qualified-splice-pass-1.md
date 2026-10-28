# F-X268, qualified splice, pass 1

**Reviewed**: Working tree against `5384c4da83b325975465736f5594dd26b56d4903`, four files, 325 insertions and 25 deletions. This is a bounded review of the remaining mechanism 1 splice change, not whole-feature approval.

**Verdict**: 1 defect, 0 smells, 0 nitpicks

## Defects

### D1, prefix owner visits are omitted from the work counter

`crates/rdocx-py/src/document.rs:5525`

The new prefix `.any` reads entries from `body_paragraphs[..paragraph]` without calling `note_visited_owner_record`. A splice after a nonempty paragraph prefix therefore performs owner-record work that the counter does not report. The adjacent suffix loop records each visit. This violates the retained F-X266 counted-work contract and can make a performance assertion pass with understated work. Count each actual prefix visit, including a short-circuiting visit. Preserve the eligibility guard and the existing work budget.

## Smells

None found in this bounded pass.

## Nitpicks

None found in this bounded pass.

## Not found

No additional defect was established in the changed namespace eligibility, local removal serialization, paragraph-position selection, refusal behavior, or structure. The structured test retains its counter assertions and functional save and reopen checks. Native typed-body prefix traversal is separate work and must remain visible in final measurements. Zero binding whole-body counters do not prove constant total public-call cost.

## Evidence and boundary

The recorded local observations are native splice eligibility 1 passed, native prefix counting 1 passed, native atomic refusal 1 passed, Python atomic refusal 43 passed, and structured splice edits 2 passed. Scoped clippy and the fresh development build passed. These observations do not discharge D1. Mechanisms 2 through 8, whole-feature microscope, release timing, memory, Word, hash and integrated verification remain pending. The earlier atomic behavior review remains a separate bounded review.
