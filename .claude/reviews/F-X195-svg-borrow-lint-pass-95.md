# F-X195, incremental SVG borrow lint, pass95

**Reviewed**: Five adopted test-helper call changes in existing svg.rs, plus their rustfmt reflow. The prior whole-feature authority remains unchanged. This pass covers only the incremental borrow lint repair.

**Verdict**: 0 defects, 0 smells, 0 nitpicks.

## Authentication and equivalence

Proposal SHA `33de318e1406c540457aa6e903c472ff27ffcc288aadbd53b90cc729584ae6f4`, adoption SHA `1f3528b0099bac75a229b858bc13583309d96d48beef1abbdce3095306e2f9eb` and patch SHA `f2afd36fb683a46d45a70c7455ec0d5c6fb59e5b6bd5846a65fb45982c546178` authenticate the retained packet. Preimage SHA `57a9ac51630420b944fb1e95d1d4d5f9f2eab02bc8355557291c151b3a298dcf` is the earlier formatter candidate. Exactly five rasterise_with_resvg calls replace cloned one-element slices with std::slice::from_ref. The unformatted candidate SHA is `64245b547bed11d03b48dc085c15493c04f3cd98d3089097568af2641d094f75`.

Actual adopted svg.rs SHA `2985b98e2411e3ddc7be437bf892d3ac8190d538f58e12c6bb8eaeb0d525fe24` matches the adoption. Candidate-to-actual lexical comparison permits only whitespace and one optional trailing argument comma added by rustfmt. All other tokens and literal/comment bytes are exact. All source and tracked snapshot records match actual bytes, sizes and modes. The sixth unrelated font.clone slice remains unchanged.

## Defects

None.

## Smells

None.

## Nitpicks

None.

## Not found

- Correctness: zero findings. `crates/rdocx/src/svg.rs:2313`, :2345, :2380, :2383 and :3925 supply the same one font value. `rasterise_with_resvg` at :3364 takes an immutable font slice, copies its font data into the oracle database and reads face_index and id. It neither mutates nor retains a mutable reference to the argument. Eliminating the temporary FontData clone therefore preserves face bytes, axes, identity, resolver ownership and paint inputs. Subsequent reference-font mutation still acts on its separately cloned value.
- Contract: zero findings. No product geometry, selected-instance policy, original-font oracle, independent Q/C references, ROI or threshold changes. Existing eight-owner artifact requirements remain unchanged.
- Panics: zero findings. The replacement constructs a valid one-element borrowed slice and adds no bounds checks, indexing or unwrap paths.
- OOXML: zero findings. No source XML, namespace, order or serialization operation changes.
- Tests: zero findings. Assertions, negative controls and existing paint inputs are identical. Original source-qualified runtime receipts remain immutable, with the explicit test-helper equivalence recorded here. The actual failed Clippy run remains failed. This review does not claim fresh Clippy or test success.
- Structure: zero findings. No new file, module, dependency, API or abstraction.

## Handback

Only this report was written. No source, Cargo, environment or cache changes occurred during review. Actual post-adoption verification remains ROOT work. This pass ends and returns its review writer lease.
