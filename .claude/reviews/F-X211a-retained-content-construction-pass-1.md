# F-X211a, retained content construction, pass 1

**Reviewed**: Exact actual source27 to frozen source28 production delta in `crates/rdocx-oxml/src/content_control.rs`, one carrier,20 additions and10 deletions. Compared the preserved pre-edit carrier and constructor-only patch, not the complete child diff. Existing public-only Word work is disjoint and excluded.

**Verdict**: Zero defects, zero smells, zero nitpicks in this bounded construction repair. Source28 qualifies the recorded normal-stack SAVE and controlled valid-root transaction, not original85, public ownership, frozen100 or whole F-X211a acceptance.

## Defects

Zero findings.

## Smells

Zero findings.

## Nitpicks

Zero findings.

## Aspects checked

- Correctness: zero findings. The new concrete `append_retained_sdt_content` at `crates/rdocx-oxml/src/content_control.rs:1715` moves actual Some(control) into ContentControl and None into RawXml, then pushes exactly once. All six affected sites at1749,1762,1768,1783,1798 and1805 retain the same parser result, raw capture and append order. No modeled descendant is invented from a typed parent. The nested-control Option fallback remains identical to the prior explicit branch.
- Contract: zero findings. Recursive reader traversal, `SdtOwner` context and qualified prefix calculation at1738 are unchanged. Actual owner admission and all resource limits are unchanged. The existing modeled leaf construction routine at1628 is untouched. The reviewed clone19 implementation is byte-equivalent and remains separate. No quota, deadline, larger stack, source rewrite or public API change is introduced.
- Panics: zero findings. The new routine performs only owned enum construction and the existing Vec push. It adds no indexing, unwrap, unsafe code, arithmetic or recursive parsing. `#[inline(never)]` at1714 isolates the large retained/nested enum construction from each recursive parser frame. Actual bounded controls establish the measured improvement, without a universal stack-safety claim or a new admission policy.
- OOXML: zero findings. Foreign/unowned Start/Empty captures retain their exact raw bytes. Other raw children still record revision position using the pre-append content.len at1766 and1796, then append one identical raw entry. Non-element events still use the same event_to_raw conversion at1805. Inline source indices and original run/raw pairs in the unchanged modeled branch retain their pre-append positions. Root slots, content attributes, prefixes, whitespace, revisions and writer order are untouched. Failed nested control parsing still preserves the original raw fragment, with no new error masking or fallback rule.
- Tests: zero findings within this repair. Actual affected content-control controls cover all seven carriers, foreign/placement ownership, exact retained lexical bytes, parser refusal/cursor behavior, duplicate content/root slots, local picture namespaces and selected paragraph/run XML atomicity. The existing standalone SAVE and controlled transaction assert the actual32-table/256-control typed path, leaf data, source identity, saved/reopened model and drops. These are genuine source-qualified successors to the preserved SAVE/transaction failures, not new mirrored implementation tests.
- Structure: zero findings. One routine in the existing carrier owns concrete construction and push for two real retained variants across six current sites. This directly removes their large enum construction from the recursive frame. No new type, file, module, crate, dependency, generic, trait, cache, registry or forwarding API is introduced.

## Exact source and capsule authentication

Worker HEAD remains `564bb645e5e93fbc986b86162a27da9d8a31bbe4`, branch work/f-x211a-codex. Checkpoint SHA-256 is `51276d1843a644cc2ffaaf41a0de55bb45e59f36420cb32dcee5ac9ed1558354`. Exact patch SHA-256 is `71702db8328415c19a4f03599c85430f05b9695e1336e5057596c0f534302d56`.

Actual preserved pre-edit carrier SHA-256 is `e3f8c53f1a08ea49bca0ee7f8d1e5e9744fe2d5fd077ee4a0bd0f7760302ef2f`, matching the prior reviewed clone19 carrier. Actual retained/current source28 content_control SHA-256 is `0eeaa034f0c0b0a042b490e74b8b9f077796665656687cc82ead5225c6f5ceaa`. Retained/current document.rs SHA-256 is `50974fd23b02114a96597145b31e2f524292c430338529cf66a744725a7ba929`, unchanged between27 and28. The complete source27/28 maps differ in only content_control.rs.

Source28 map SHA-256 is `a434b8e128d56f0008cc36aa67955d062a3a6c55a4919e159fa4ef3094c30120`, containing3,208 records. All13 current native carrier hashes, sizes and modes match this map before and after the review. The complete map is historical runtime evidence while disjoint Python carriers may evolve, not a false current whole-tree freeze claim.

## Actual bounded successor

Receipt SHA-256 is `b9127526522ea0604af474ed7882f6bd681e5f8ead0f68f58dac825059fa9e30`. Independently authenticated all14 raw logs and recorded source/binary pins. All rows exit0, have no timeout, reaped leaders and gone owned groups. Two rows are preparation, affected all-target compilation and paired release test-binary compilation, without behavioral test credit. Standalone SAVE and full controlled transaction each pass one control. Ten affected filters pass19 controls. Total actual runtime controls are21.

The driver at `.claude/scratch/F-X211a-start/retained-construction-28.py:25` sets2MiB before every runtime case, with30-second per-case deadlines. Its earlier8MiB environment covers compilation only. Runtime claims do not inherit that larger setting. Standalone SAVE reaches opaque-source reparse-after, finishes save/reopen and both drops. The controlled transaction reaches clone, flush, profile validation, selected insertion, source update, comment reconciliation and candidate reopen/commit, then saves/reopens and checks the changed leaf across the complete typed32/256 path. Its actual runtime is3.01 seconds. No dormant relationship mode or extra row/property repair is executed.

Current and retained source28 Word binary SHA-256 is `de30f85e75a1a714de8c1cbe3a8187dec52c76c8de1a16e06e1505b1e80a05f6`, OXML binary is `33bcf57dbcdcf07cd3df72330d0ff140b8b8f01066237181e92319090d6d4ac6`. Both retained copies independently rehash correctly. This audit runs no tests, Cargo or target command.

## Originals and acceptance boundary

Source27 SAVE negative receipt `959b7a285a11da48f26c772ce08a3ae963dea0825c4494a8ccfbbdd6109bf122` retains actual exit-6 at opaque-source reparse-before after both modeled collection markers complete. Source20 transaction negative receipt `4f5228676ba6dbd8622135440c650be4fa46164d8e4322cd8a8e325e5ecf30d2` retains its reparse boundary. Their immutable logs, source records, safe cleanup and retained binaries authenticate. No collector failure is inferred, and no visitor repair is smuggled into this delta.

Standalone21 parse/equality/drop positive receipt `0a8c4e0fc1a68a910e298259ddd3071f79b7ae3ef21652cb1f52e3b3e9e5e59f` and standalone25 serialize/reparse/drop positive receipt `499124800feb19d02512e0505beb799b6924508e180d40a6b63e5ef131044440` retain their distinct caller contexts. Both logs and receipt hashes authenticate. Source21's executable authenticates against retained source20 bytes. Source25's old Word target executable was overwritten and no retained25 Word copy was found, so this report does not claim it remains rehashable. Its OXML bytes match the retained source20 OXML binary. Historical positive metadata/log proof remains distinct from current source28 artifact proof.

Original85 is unchanged and unqualified. The controlled valid-root successor does not rewrite or relabel that original test, frozen corpus denominator or first-draft result. Public five-owner/local retirement, installed bindings and allocation controls, whole-child strict/scoped gates, root composition and remaining required native acceptance stay pending. No F-X211a handoff or completion is created by this bounded zero review.

## Review integrity

Read canonical workflow/microscope, parent/childa contracts, progress and actual parser/package/source-preservation providers. Coordinated native freeze before audit. Guarded all13 native carriers, retained checkpoint/source/patch/receipt/binaries and HEAD/branch/logical index before and after. Current physical index is separately unchanged. Existing F218 tracked source, complete diff, HEAD and physical/logical index remain exact. Only this root-authorized review artifact in the existing F218 worker is written. No source fixes, runtime, tests, CP, target, Office/UI, network, ledger, branch, baseline or commit mutation occurred. Review ends here and returns control.
