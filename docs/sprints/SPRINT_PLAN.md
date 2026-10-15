# Sprint Plan

Sprint-by-sprint roadmap for the shared OOXML infrastructure, the Word and
PowerPoint families, and the conditional advanced spreadsheet programme.
Sprints are dependency and review boundaries, not fixed two-week containers.
Sprint clocks start at the first `/start-feature` of that sprint, not at a fixed
calendar date.

Each F-ID uses focused tests, applicable oracle cases, `/verify --scoped` and
a zero-finding `/microscope`. Full verification and sprint review cover the
final integrated result at closure. A formal dependency checkpoint completes
a prerequisite from scoped evidence and focused integration checks.

The active roadmap runs through S105, with earlier deferred cutover boundaries
retained in place. The sizing rationale and compression options are in
`docs/hld/14-development-backlog.md`.

M1 to M5 stage the extraction without changing released rdocx dependencies.
M7 to M12 build rpptx. Deferred M6 publication and rdocx cutover follow M12,
then M13 ships the bindings for both.

## Capacity calibration after S03

S01 through S03 completed 15 stories representing 24 estimated developer-days
in 15 recorded actual days. That is 5.00 stories per working week and 1.6
estimated days per actual day. The story-count rate is not used alone because
the remaining plan contains substantially more L and XL work than the first
three sprints.

The remaining 366 estimated developer-days therefore forecast to about 46
active working weeks at the observed weighted rate. Use 45 to 50 weeks as the
current planning range. Keep the existing dependency-defined sprint and
milestone boundaries, and size each implementation wave by dependencies,
exclusive resources, and estimated days rather than filling a fixed calendar
box. Recalculate after S06, when the evidence includes the first post-baseline
L story and the higher-risk layout extraction.

## Goals per sprint

### M1, Preparation and safety net

#### Sprint S01, The safety net

**Goal**: rendering is reproducible across machines, a byte-level baseline
exists for every sample, and the three shipped defects found during the audit
are fixed. Nothing has moved yet.

| F-ID | Title | Size |
|------|-------|------|
| F-001 | Deterministic font mode                      | M |
| F-002 | rust-toolchain.toml                          | S |
| F-003 | Output-stability hash harness                | L |
| F-004 | Caladea licence and the false OFL claim      | S |
| F-005 | Fix the image counter                        | S |
| F-006 | Fix the JPEG standalone-marker walk          | S |

F-001 gates F-003: a baseline recorded against system fonts would not reproduce
on another machine, which would make the harness worthless.

#### Sprint S02, Prerequisites and the pre-churn tag

**Goal**: everything the later milestones depend on is in place, and a
known-good published state is tagged immediately before the extraction begins.

| F-ID | Title | Size |
|------|-------|------|
| F-007 | Resolve core properties through the rel      | S |
| F-008 | Non-consuming setter twins                   | M |
| F-009 | Cache the layout result                      | M |
| F-010 | Reserve crate names                          | S |
| F-011 | Pin unit truncation behaviour                | S |
| F-012 | Tag v0.4.1                                   | S |

F-008 is required by M13 and improves the Rust API independently. F-011 must
land before anyone is tempted to change truncation to rounding.

### M2, Shared infrastructure extraction

#### Sprint S03, oxml-core

**Goal**: the generic types leave `rdocx-oxml` and 323 call sites do not change.

| F-ID | Title | Size |
|------|-------|------|
| F-013 | Create oxml-core                             | M |
| F-014 | New unit types                               | M |
| F-017 | App and custom properties                    | M |

#### Sprint S04, oxml-opc

**Goal**: the package layer is format-neutral and proven against a real pptx.

| F-ID | Title | Size |
|------|-------|------|
| F-018 | Create oxml-opc                              | M |
| F-019 | PresentationML relationship and content types| S |
| F-020 | oxml-opc reads a pptx                        | M |
| F-021 | Zip-slip hardening tests                     | S |

F-015 and F-016 carried from S03, and F-022 joined their deferred cutover. They
are rescheduled to S32.2 after PowerPoint development and shared-crate
publication readiness. Passing the rdocx 0.5.0 release boundary protects that
release but does not make an unpublished implementation available to later
package dry-runs. F-020 converts the plan's central package assumption into a
test.

### M3, Media

#### Sprint S05, oxml-media

**Goal**: stage and prove one crate that owns image sniffing, dimensions, and
naming without changing released rdocx dependencies.

| F-ID | Title | Size |
|------|-------|------|
| F-023 | oxml-media format sniffing                   | M |
| F-024 | Image probing and DPI                        | L |
| F-025 | MediaNamer                                   | S |
| F-026 | native_size with explicit DPI                | S |

F-027 and F-028 move to S32.2. F-027 retains a focused package regression for
the intentional change from trusted extensions to sniffed content types. The
existing hash harness remains unchanged because it does not collect package
content types or media relationship targets.

### M4, Layout primitives

#### Sprint S06, oxml-layout and the line.rs decoupling

**Goal**: the format-neutral layout types are staged in isolation, including
the one file that needs genuine API design.

| F-ID | Title | Size |
|------|-------|------|
| F-029 | Create oxml-layout                           | M |
| F-030 | Decouple line.rs                             | L |
| F-031 | Transform                                    | M |

F-030 is the highest drift risk in the extraction. Own PR, own review, gated
hard on the hash harness.

#### Sprint S07, The PositionedElement extension

**Goal**: the staged shared element type can express a rotated, clipped,
gradient-filled shape without changing released rdocx construction sites.

| F-ID | Title | Size |
|------|-------|------|
| F-032 | Path and PathCommand                         | M |
| F-033 | Paint and Stroke                             | M |
| F-034 | Path and Group arms                          | M |
| F-035 | The walk helper                              | S |
| F-036 | MediaId                                      | S |

Two new arms, not ten new fields. F-035 exists specifically to prevent the
recursion hazard that S09 then tests for.

### M5, PDF backend

#### Sprint S08, The coordinate system

**Goal**: the renderer moves to one global CTM with zero pixel change.

| F-ID | Title | Size |
|------|-------|------|
| F-037 | Create oxml-pdf                              | S |
| F-038 | Golden-PNG harness                           | M |
| F-039 | Global CTM flip                              | L |

F-039 is the single highest-risk change in the plan. It lands before
PresentationML rendering code exists, so a regression has only one possible
cause.

#### Sprint S09, Groups, paths and the recursion fix

**Goal**: nested content renders, and the three collection passes see inside
groups.

| F-ID | Title | Size |
|------|-------|------|
| F-040 | Group rendering                              | M |
| F-041 | Path rendering                               | M |
| F-042 | Rewrite the three collection passes on walk  | M |
| F-044 | ExtGState alpha                              | S |

F-042 is the R3 regression gate. Its three tests are the only thing standing
between this design and PDFs that silently lose fonts, images or links.

#### Sprint S10, Gradients and the rasteriser

**Goal**: both backends render everything the element types can express.

| F-ID | Title | Size |
|------|-------|------|
| F-043 | Gradient shading dictionaries                | L |
| F-045 | Rasteriser: groups, paths, gradients, dashes | L |

F-045 also fixes the dash pattern that all PNG output currently discards.

### M6, deferred shared publication and rdocx cutover

#### Sprint S11, Staged extraction gate

**Goal**: verify the isolated shared crates and continue PowerPoint development
without publishing them or changing released rdocx dependencies.

No publication or consumer-cutover story runs at S11. F-046 through F-051 are
rescheduled to S32.1 and S32.2 after PowerPoint development. S11 is the staged
extraction validation boundary before DrawingML construction begins.

### M7, DrawingML

#### Sprint S12, Colour

**Goal**: a theme colour with a transform stack resolves to the exact RGB
PowerPoint produces.

| F-ID | Title | Size |
|------|-------|------|
| F-052 | Create oxml-drawing and namespace constants  | S |
| F-053 | OrderedRawChildren                           | M |
| F-054 | Colour choices                               | M |
| F-055 | The colour transform stack                   | L |
| F-056 | Colour map resolution                        | M |

F-055's test gate is a table of 40 pairs sampled from real renders. Getting
`lumMod` wrong makes an entire deck the wrong shade.

#### Sprint S13, Geometry and fills

**Goal**: any shape's outline and fill can be described.

| F-ID | Title | Size |
|------|-------|------|
| F-057 | a:xfrm                                       | M |
| F-058 | Guide evaluator                              | L |
| F-059 | a:custGeom                                   | M |
| F-060 | Fills                                        | L |

F-058 is what makes the preset table a data problem in M10 rather than a code
problem.

#### Sprint S14, Lines, effects and text

**Goal**: the DrawingML text vocabulary is modelled.

| F-ID | Title | Size |
|------|-------|------|
| F-061 | Lines                                        | M |
| F-062 | Effects                                      | S |
| F-063 | Shape properties and style references        | M |
| F-064 | DrawingML text model                         | XL |
| F-064a | Text body properties and shell               | M |
| F-064b | Text paragraphs and runs                     | L |
| F-064c | Text bullets                                 | S |
| F-064d | Nine-level list styles                       | M |

F-064 is the umbrella gate. Its implementation is split into F-064a through
F-064d, and the parent closes only after every child closes.

#### Sprint S15, Theme

**Goal**: themes read and write, and rdocx adopts the shared type without
changing behaviour.

| F-ID | Title | Size |
|------|-------|------|
| F-065 | Theme read and write                         | L |
| F-066 | The rdocx Theme adapter                      | S |

F-066's test gate is the hash harness being unchanged. The Word tint and shade
path is deliberately left alone.

### M8, PresentationML

#### Sprint S16, Parts and the shape tree

**Goal**: the corpus round-trips with everything opaque, then with the core
parts modelled.

| F-ID | Title | Size |
|------|-------|------|
| F-067 | Create rpptx-oxml and the corpus harness     | M |
| F-068 | presentation.xml                             | M |
| F-069 | Slide, layout and master parts               | L |
| F-070 | The shape tree                               | L |

F-067's raw round-trip proves the OPC layer and the corpus harness before any
XML modelling exists.

#### Sprint S17, Placeholders, pictures and tables

| F-ID | Title | Size |
|------|-------|------|
| F-071 | Placeholders                                 | M |
| F-072 | Pictures                                     | M |
| F-073 | Graphic frames                               | M |
| F-074 | DrawingML tables                             | L |

#### Sprint S18, The long tail

| F-ID | Title | Size |
|------|-------|------|
| F-075 | Connectors                                   | S |
| F-076 | mc:AlternateContent                          | M |
| F-077 | Notes slides and notes master                | M |
| F-078 | relmap rewrite_rel_ids                       | M |

F-078 is what makes deep copy safe in M11. Without it a duplicated slide's
SmartArt points at the source slide's relationships.

#### Sprint S19, The read facade

**Goal**: open any deck and read it.

| F-ID | Title | Size |
|------|-------|------|
| F-079 | The rpptx read facade                        | L |
| F-080 | Modelled round-trip gate                     | M |

**This is the M8 gate**: all 50 decks round-trip and every one opens in
PowerPoint without a repair prompt.

### M9, Inheritance resolver

#### Sprint S20, The chains

**Goal**: every inherited property resolves.

| F-ID | Title | Size |
|------|-------|------|
| F-081 | ResolveCtx skeleton and placeholder chain    | M |
| F-082 | Effective transform and body properties      | M |
| F-083 | The seven-step list style merge              | L |
| F-084 | Format scheme reference resolution           | M |
| F-085 | Typeface resolution                          | S |

#### Sprint S21, Draw order and the contract

**Goal**: `ResolvedSlide` is frozen and correct.

| F-ID | Title | Size |
|------|-------|------|
| F-086 | Draw order and the flattener                 | L |
| F-087 | ResolvedSlide contract                       | M |
| F-088 | Visual differential tests                    | M |

F-086's test gate is that a rendered slide contains no "Click to edit Master
title style". Placeholders on layouts and masters are templates, never drawn.

### M10, Renderer

#### Sprint S22, Geometry

| F-ID | Title | Size |
|------|-------|------|
| F-089 | Resolve the preset geometry licensing question | S |
| F-090 | Preset table generator                       | L |
| F-091 | Preset evaluation and fallback               | M |
| F-092 | rpptx-render skeleton and RenderInput        | M |

F-089 is a decision, not code, and it blocks F-090. LibreOffice's table is
MPL-2.0 and cannot be used.

#### Sprint S23, Shapes

| F-ID | Title | Size |
|------|-------|------|
| F-093 | Shape geometry, fills and lines              | L |
| F-094 | Rotation, flips and groups                   | M |
| F-095 | Arrowheads                                   | S |
| F-096 | Pictures with crop and tile                  | M |
| F-097 | Backgrounds                                  | S |

Ships slides with shapes but no text.

#### Sprint S24, Text

| F-ID | Title | Size |
|------|-------|------|
| F-098 | Shape text layout                            | XL |
| F-098a | Text content box                             | M |
| F-098b | Paragraph inline resolution                  | L |
| F-098c | Line stacking                                | M |
| F-098d | Text anchoring                               | S |
| F-099 | Bullets                                      | M |
| F-100 | Autofit                                      | M |
| F-101 | Vertical text                                | S |

**The milestone that makes the project real.** F-098 is the umbrella gate for
F-098a through F-098d, which implement the content box, paragraph inline
resolution, line stacking, and anchoring. The parent closes only after every
child closes.

#### Sprint S25, Tables and the fidelity gate

| F-ID | Title | Size |
|------|-------|------|
| F-102 | Table rendering                              | L |
| F-103 | Hyperlinks, fields and diagnostics           | M |
| F-104 | SSIM fidelity harness                        | L |

**This is the M10 gate** and the natural point to cut an early
read-plus-render release if the schedule needs compressing.

### M11, Write API

#### Sprint S26, Slides

| F-ID | Title | Size |
|------|-------|------|
| F-105 | Bundled default.pptx                         | M |
| F-106 | ShapeIdAllocator and MediaStore              | M |
| F-107 | add_slide                                    | L |
| F-108 | validate()                                   | M |

F-108 will save more debugging time than any other story in the backlog.

#### Sprint S27, Shapes and text

| F-ID | Title | Size |
|------|-------|------|
| F-109 | Shape mutation facade                        | L |
| F-110 | add_textbox, add_shape, add_connector, group | M |
| F-111 | add_picture                                  | M |
| F-112 | Text frame mutation                          | L |

#### Sprint S28, Tables and acceptance

| F-ID | Title | Size |
|------|-------|------|
| F-113 | Table facade                                 | L |
| F-114 | remove_slide, move_slide, duplicate_slide    | M |
| F-115 | Slide and presentation properties            | S |
| F-116 | Cross-viewer acceptance                      | M |

**This is the M11 gate**: a generated deck opens clean in PowerPoint, Keynote,
Google Slides and LibreOffice.

### M12, Charts

#### Sprint S29, The data layer

| F-ID | Title | Size |
|------|-------|------|
| F-117 | oxml-sml workbook writer                     | L |
| F-118 | ChartML core types                           | L |
| F-119 | Series and data references                   | L |

F-119's caches are what actually render. A chart written without them is empty
in most viewers.

#### Sprint S30, Axes and plots

| F-ID | Title | Size |
|------|-------|------|
| F-120 | Axes                                         | L |
| F-121 | Bar and line plots                           | M |
| F-122 | Pie, doughnut, area, scatter and radar plots | L |
| F-123 | Data labels and number formats               | M |

#### Sprint S31, Authoring and rendering

| F-ID | Title | Size |
|------|-------|------|
| F-124 | add_chart                                    | L |
| F-125 | Chart rendering: geometry                    | L |
| F-126 | Chart rendering: axes, gridlines and labels  | L |

#### Sprint S32, Chart polish

| F-ID | Title | Size |
|------|-------|------|
| F-127 | Chart colour resolution                      | M |
| F-128 | Preserved chart fallback                     | S |

### Deferred shared publication and rdocx cutover

#### Sprint S32.1, Shared publication readiness

**Goal**: make the completed shared crates packageable, fully gated, and ready
for an explicitly approved publication without publishing from this sprint.

| F-ID | Title | Size |
|------|-------|------|
| F-047 | Packaging include and size gate              | M |
| F-048 | Automate split-family release preparation    | M |
| F-049 | Extend publish.yml to the extracted workspace| M |
| F-050 | CI matrix additions                          | S |

After S32.1, publication runs only through a separate reviewed release plan
with explicit approval. S32.2 cannot start until the registry contains the
approved shared-crate versions and a clean consumer resolves those versions.

#### Sprint S32.2, Released rdocx cutover

**Goal**: after the real shared crates are published through their approved
release plan, move released rdocx consumers onto them and document the cutover.

| F-ID | Title | Size |
|------|-------|------|
| F-X005 | Tag rpptx-v0.1.2                           | S |
| F-015 | rdocx-oxml becomes a facade                  | S |
| F-016 | Length re-export                             | S |
| F-022 | rdocx-opc deprecation shim                   | S |
| F-027 | rdocx adopts oxml-media                      | M |
| F-028 | add_picture_auto                             | S |
| F-046 | rdocx layout and PDF cutover                 | M |
| F-051 | CHANGELOG and migration notes                | S |

**This is the deferred M6 release gate.** The shared crates are real published
dependencies, released rdocx packages pass archive verification, and the hash
harness remains unchanged while a focused regression proves the F-027
content-type change.

### M13, Bindings and tooling

#### Sprint S33, rdocx-py

**Goal**: the handle design is validated against the settled API before it is
reused.

| F-ID | Title | Size |
|------|-------|------|
| F-129 | oxml-py-support                              | M |
| F-130 | rdocx-py core                                | L |
| F-131 | rdocx-py formatting and tables               | L |
| F-132 | Python enums, units and exceptions           | M |
| F-133 | rdocx-py rendering with allow_threads        | S |

#### Sprint S34, Wheels and rpptx-py

| F-ID | Title | Size |
|------|-------|------|
| F-134 | Type stubs and py.typed                      | M |
| F-135 | python-docx parity suite                     | M |
| F-136 | rpptx-py                                     | L |
| F-137 | wheels.yml                                   | M |
| F-138 | PR-time Python job                           | S |

#### Sprint S35, WASM

**Goal**: the wasm crates wrap the real facades and are watched by CI.

| F-ID | Title | Size |
|------|-------|------|
| F-139 | Rewrite rdocx-wasm                           | L |
| F-140 | wasm CI job                                  | S |
| F-141 | to_pdf in the browser                        | M |
| F-142 | rpptx-wasm                                   | M |

F-139 fixes a shipped defect that silently discards every package part except
two. F-140 is why it will not happen again.

#### Sprint S36, CLIs and local packaging

| F-ID | Title | Size |
|------|-------|------|
| F-143 | oxml-cli-support                             | S |
| F-144 | rpptx-cli                                    | L |
| F-145 | rpptx-cli thumbnail and outline              | M |
| F-146 | npm publication                              | S |
| F-X001 | rdocx-cli tests                             | M |
| F-X002 | README example correctness                  | S |
| F-X003 | Deduplicate the sample generators           | S |
| F-X004 | Fix the shared temp path in the test suite  | S |

**This is the v1 implementation gate.** The CLI and local package surfaces are
complete. Registry publication is explicitly deferred. The expanded
incubating Rust family requires a fresh version and reviewed release through
F-X006. npm registry publication requires a separate future story.

#### Sprint S37, Expanded Rust family release

**Goal**: prepare a fresh common incubating version and publish the complete
14-package Rust family through the reviewed release workflow after separate
final approval.

| F-ID | Title | Size |
|------|-------|------|
| F-X006 | Tag the expanded rpptx family               | S |

#### Sprint S38, Contributor integration and stable release

**Goal**: land PR 25 with its contributor credit intact, harden the new Word
composition APIs against package and table-geometry regressions, and make the
stable crate family documentation useful at the point of publication. Prepare
and publish the complete stable family at the breaking pre-1.0 0.5.0 boundary
only after the integrated result passes review and receives separate release
approval.

| F-ID | Title | Size |
|------|-------|------|
| F-X007 | Integrate PR 25 and stable crate documentation | L |
| F-X008 | Tag v0.5.0                                      | S |

F-X008 depends on F-X007. The GitHub PR is merged into the sprint branch so
the contributor remains credited in the pull request and merge record. Only
`/close-sprint` later merges the reviewed sprint to `main`.

#### Sprint S39, Workspace crate documentation and next-minor releases

**Goal**: every one of the 26 Cargo workspace packages declares a useful
README. Each README explains the crate's role, intended audience, relationship
to neighbouring packages, and includes a concrete example in the language or
command surface that users actually consume. Publish those READMEs for every
crates.io-eligible package through separate next-minor stable and incubating
release tags.

| F-ID | Title | Size |
|------|-------|------|
| F-X009 | README coverage for every workspace crate | L |
| F-X010 | Tag v0.6.0 | S |
| F-X011 | Tag rpptx-v0.2.0 | S |

The existing README runner becomes an exact 26-package contract. Published
archives must each contain one intended README, while unpublished Python and
WASM packages receive accurate local usage examples without gaining any new
publication authority. F-X010 publishes the seven stable crates first. F-X011
then publishes the fourteen incubating crates. Each tag has its own full
verification, clean review, and immediate release approval boundary.

#### Sprint S40, Restore pinned CI toolchains

**Goal**: restore a green hosted CI baseline after runner and package-manager
updates exposed unpinned or incorrectly validated external tools. Keep the
reviewed Poppler 26.01.0 rendering oracle and Binaryen 125 optimizer boundary
without changing product output or recorded rendering baselines.

| F-ID | Title | Size |
|------|-------|------|
| F-X012 | Restore pinned CI toolchains | M |

The story installs checksum-pinned Poppler 26.01.0 for every job that executes
its oracle-dependent tests, validates the official Binaryen 125 Linux version
string, and proves the complete pull-request workflow on a hosted runner. It
does not change a crate, release version, published package, or rendering
baseline.

#### Sprint S41, Footnote placement and floating drawing wrapping

**Goal**: land the parts of the external PR 2 contribution that current `main`
still lacks, rebuilt on the anchor architecture that superseded the
contributor's own. Fix the two footnote placement defects, then give anchored
drawings a real wrap model and make body text flow around them.

| F-ID | Title | Size |
|------|-------|------|
| F-X013 | Footnote and endnote placement | M |
| F-X013a | Footnote line advance | S |
| F-X013b | Footnote reservation and splitting | L |
| F-X013c | Endnotes at the document end | M |
| F-X014 | Kashida justification values | S |
| F-X015 | Anchored drawing wrap and alignment model | M |
| F-X016 | Floating drawing placement and text wrapping | L |

The note work lands first, because it is independent of the drawing work and
each child carries its own baseline delta. F-X013 was planned as a single M and
split into three children during its design, when splitting oversized notes and
correcting endnote placement were taken into scope. F-X014 is a one-line parser
widening carried in the same wave because it comes from the same contribution.
F-X015 adds the wrap and alignment surface without changing placement, which
keeps the harness flat and makes F-X016 the single story that owns the rendering
delta for wrapped drawings.

#### Sprint S42, Dependency refresh

**Goal**: take the outstanding semver-compatible dependency updates and measure
what they do to rendered output. Nothing here is a security fix, since the
advisory scan is already clean, so the value is in not letting the lockfile
drift far enough that a later update becomes a large unexplained delta.

| F-ID | Title | Size |
|------|-------|------|
| F-X020 | Refresh the dependency lockfile | S |
| F-X024 | Move the theme adapter into rdocx-oxml | M |
| F-X022 | Tag rpptx-v0.3.0 | S |
| F-X023 | Tag v0.7.0 | S |

Two of the sixteen pending updates are in the font and image decoding path, so
the hash harness decides whether the refresh is a no-op or a declared rendering
delta. It runs first and alone, because a refresh that moved a baseline should
not compete with a release to explain the same delta.

F-X024 then removes the reason the release order was impossible. Scoping the two
release stories exposed a cycle between the trains: `rdocx-layout` depends on
`oxml-layout`, and `oxml-drawing` depends on `rdocx-oxml` through the one
documented architecture exception. With both trains carrying breaking changes,
neither could publish first. Moving the theme adapter into `rdocx-oxml` inverts
that edge, so the dependency runs one way and incubating always publishes
first.

The two release stories then carry S41's work to crates.io in that order.

#### Sprint S43, Robustness and gate coverage

**Goal**: clear the follow-ups S41 and S42 filed. Three are defects that a real
document can reach, and two close gaps in the gates that let the other three
survive as long as they did.

| F-ID | Title | Size |
|------|-------|------|
| F-X018 | Unknown enumerated values must not fail a document open | M |
| F-X017 | Notes broken to their own section's width | S |
| F-X019 | Paragraph-relative later drawings should wrap | M |
| F-X021 | The hash harness should cover PDF output | M |
| F-X025 | /verify must run the release regressions | S |

F-X018 leads because it is the only one where a document fails to open rather
than rendering imperfectly. F-X014 fixed the three kashida values because a real
contribution reached them, and eight more parsers have the same shape.

F-X017 and F-X019 are the two limitations S41 recorded rather than hid, both
narrow and both reachable by a real document. F-X021 and F-X025 are the gates:
one gives the harness PDF coverage it has never had, the other makes `/verify`
run the release regressions that `publish.yml` treats as its publication gate.
The gate stories land last because neither blocks the three defect fixes, and
putting them first would delay work that users can actually hit.

#### Sprint S44, Gate coverage and specification repair

**Goal**: finish the job S43 started. S43 closed two gaps in the gates and
found, in passing, that the records describing those gates had drifted from
them. This sprint puts the two remaining gates where CI can see them and repairs
the documentation that tells every future session what is true.

| F-ID | Title | Size |
|------|-------|------|
| F-X026 | CI must run the release regressions too | S |
| F-X027 | Wire the golden-PNG gate into something | S |
| F-X029 | Path-filtered CI jobs | M |
| F-X028 | Repair the agent-facing documentation drift | M |

Every implementation milestone is closed, so this sprint carries no feature
work. Three of the four exist because S43 went looking at the instruments rather
than the product. F-X029 came out of a review of whether the workspace should be
split into separate repositories. That review also produced F-X030, archived
before the sprint started because the WASM packages are deliberately
unpublished and its stated problem does not exist.

No pair has a hard dependency, so the order is a preference. The one soft
coupling is F-X026 and F-X029, which both edit `ci.yml`. F-X026 first, because it is the narrower
half of a gap S43 half-closed: `/verify` runs the release preflights now and CI
still does not, so a contributor who skips the local gate can move a version
carrier and see a green pull request. F-X027 next, because the golden-PNG gate
is fully specified and wired into nothing, and deciding where it belongs needs a
judgement about the pinned Poppler build that F-X026 does not need.

F-X028 lands last and is the largest, because it is the only one that touches
`CLAUDE.md`, and a story that rewrites the file every other session reads first
should land against a tree the other two have already settled.

## Post-v1 roadmap, S45 onward

v1 shipped at S43 and every implementation milestone closed. S44 repairs the
gates and the records. From S45 the plan resumes at milestone granularity
against `14-development-backlog.md` M14 through M24.

The order is deliberate and each boundary is a stopping point. Stopping after
S45 leaves one chart engine serving both families. Stopping after S51 leaves a
document-automation product. S69 closes the first Word-depth programme. S73
closes from-scratch generation for the five private business documents, and S95
closes the broader modern DOCX authoring boundary. The advanced spreadsheet
programme starts only after S95 and only if its feasibility gate confirms a gap
worth filling in the Rust ecosystem.

| Sprints | Milestone | Stories | Days |
|---|---|---|---|
| S45 | M15, charts beyond PowerPoint | 4 | 12 |
| S46 to S48 | M14, Word collaboration layer | 9 | 28 |
| S49 to S51 | M16, document automation | 15 | 39 |
| S52 to S53 | M17, security and compliance | 7 | 23 |
| S54 to S56 | M18, format breadth | 8 | 26 |
| S57 to S58 | M20, fidelity at scale | 7 | 27 |
| S59 to S64 | M21, presentation depth | 15 | 60 |
| S65 to S69 | M22, Word depth | 12 | 44 |
| S70 to S73 | M23, from-scratch business documents | 24 | 112 |
| S74 to S75, S86, S89 to S95 | M24, modern DOCX authoring completeness | 47 | 219 |
| S96 to S105 | M19, advanced spreadsheets | 21 | 85 |

The table counts milestone stories only. S70 also carries four cross-cutting
Issue 67 and Issue 69 stories estimated at 12 developer-days. S76 through
S85 hold the separate contribution, issue repair and hosted CI programme.

### M15, Charts beyond PowerPoint

#### Sprint S45, One chart engine

**Goal**: make the chart engine serve Word as well as PowerPoint. It is the
cheapest milestone on the roadmap and the only one whose engine already exists
on the format-neutral side of the crate graph.

| F-ID | Title | Size |
|------|-------|------|
| F-156 | Extract oxml-chart | L |
| F-157 | Word chart part and embedded workbook | M |
| F-158 | Document::add_chart | M |
| F-159 | Chart rendering in the Word paginator | M |

F-156 is a file move and nothing else. The hash harness must be byte-identical
across it, and folding a behaviour change into it is forbidden. The other three
are strictly ordered, since each needs the part the one before it writes.

### M14, Word collaboration layer

#### Sprint S46, Comments and content controls

**Goal**: open the collaboration layer at both ends. Comments are the most
requested missing API in this space and content controls are the primitive every
document-assembly product is built on.

| F-ID | Title | Size |
|------|-------|------|
| F-147 | Comment model and part | M |
| F-148 | Comment API | M |
| F-152 | Content control model | L |
| F-153 | Content control binding | M |
| F-154 | Bookmarks and cross-references | M |

Two independent pairs, so they parallelise cleanly. F-154 joins this sprint
rather than the next because F-161 needs bookmarks to resolve `REF` and
`PAGEREF`, and that is two sprints away.

#### Sprint S47, Tracked changes

**Goal**: read, write and resolve revisions. The single most demanded enterprise
capability in this space and the one with no open-source answer in any language.

| F-ID | Title | Size |
|------|-------|------|
| F-149 | Revision model | L |
| F-150 | Accept and reject revisions | L |

Two stories, both L, and deliberately alone in a sprint. F-150 has to reproduce
what Word produces from the same input, which is the kind of correctness that
takes the time it takes.

#### Sprint S48, Revision display and protection

**Goal**: close M14. Show revisions, and read the author's intent about who may
change what.

| F-ID | Title | Size |
|------|-------|------|
| F-151 | Revision display in the renderer | M |
| F-155 | Document protection | M |

A short sprint on purpose. It carries the M14 end-of-milestone gate, which
covers four subsystems built across three sprints.

### M16, Document automation

#### Sprint S49, Fields

**Goal**: evaluate the field codes real documents are full of. Everything in
M16 rests on this.

| F-ID | Title | Size |
|------|-------|------|
| F-160 | Field instruction parser | L |
| F-161 | Field evaluation engine | L |
| F-162 | Field update policy | M |
| F-203 | Reader compatibility corrections | M |

F-160 through F-162 are strictly ordered. F-203 is an independent corrective
story for reader preservation. F-161 also depends on F-154 from S46, which is
why bookmarks landed early.

#### Sprint S50, Templating

**Goal**: turn substitution into generation.

| F-ID | Title | Size |
|------|-------|------|
| F-163 | Template syntax | L |
| F-164 | Loops and conditionals | L |
| F-165 | Repeating table rows and lists | M |

F-163 leads because the tag-split-across-runs problem is the one every naive
implementation gets wrong, and the two after it inherit whatever it decides.

#### Sprint S51, Automation milestone and community release

**Goal**: close M16, add the requested native reader and editor surfaces,
establish a custom reviewed release-notes ceremony, and publish the coherent
incubating and stable trains.

| F-ID | Title | Size |
|------|-------|------|
| F-166 | Mail merge | M |
| F-167 | Document comparison | L |
| F-168 | Watermarks | S |
| F-X032 | Expose complete Word layout results | S |
| F-X033 | Integrate PR 36 ordered body items | S |
| F-X034 | Reviewed release notes for every release | S |
| F-X035 | Tag rpptx-v0.4.0 | S |
| F-X036 | Tag v0.8.0 | S |
| F-X037 | Trace Word glyphs to source paragraphs | M |
| F-X038 | Cache relayout work across document edits | L |

F-167 is the flagship of every commercial library in this category. It is scoped
to body text, tables and list structure, with formatting-only differences
recorded as a diagnostic, which is what keeps it one story. The community API
surface combines complete layouts, source provenance, bounded relayout caches,
and direct ordered body items. The incubating release precedes stable 0.8.0
because `oxml-chart`, the low-level provenance types, and shared font-cache
work must exist on crates.io before the stable dependency graph can publish.
Both release tags retain separate final approval boundaries.

### M17, Security and compliance

#### Sprint S52, Encryption and renderer follow-ups

**Goal**: open the files that currently cannot be opened at all, then close the
community-reported rendering and interactive-layout gaps with exact public
regressions.

| F-ID | Title | Size |
|------|-------|------|
| F-169 | Agile encryption, read | L |
| F-170 | Agile encryption, write | M |
| F-171 | Digital signature verification | L |
| F-X039 | Share layout payloads and transfer reusable engines | M |
| F-X040 | Restart pagination and cache table blocks | L |
| F-X041 | Remove duplicated glyphs at break opportunities | M |
| F-X042 | Prove headers and footers in PDF output | S |
| F-X043 | Reuse bundled-fallback caller-font layouts | M |
| F-X044 | Scale paragraph-cache lookup for editors | M |
| F-X045 | Cache headers and footers transactionally | M |
| F-X046 | Reuse substituted pages exactly | S |
| F-X047 | Attribute empty Word paragraphs | S |

Reading comes first and matters most. A password-protected document is a hard
stop for a user today, where an unsigned one is only a missing assurance.
F-X039 establishes shared ownership and a checked editor handoff before F-X040
retains pagination tails. F-X041 fixes Issue 23 at the common layout layer, and
F-X042 closes Issue 15 with a public DOCX-to-PDF gate. F-X043 through F-X047
close PRs 40 and 41 after retaining their useful editor behavior behind exact
context identity, transactional publication, and bounded memory. The sprint
does not import unchecked engine setters, hash-authoritative reuse, or
unbounded caches from the draft branches.

#### Sprint S53, Security milestone and community release

**Goal**: close M17, harden the dense-form layout reported by the community,
and publish the complete incubating and stable Rust families with reviewed
contributor credit.

| F-ID | Title | Size |
|------|-------|------|
| F-172 | Digital signature creation | M |
| F-173 | Tagged PDF structure tree | L |
| F-175 | Redaction | M |
| F-X048 | Dense form table fidelity | L |
| F-174 | PDF/A conformance | M |
| F-X049 | Tag rpptx-v0.5.0 | S |
| F-X050 | Tag v0.9.0 | S |

F-173 is the one a LibreOffice-based pipeline cannot do well, and the layout
engine already knows the document semantics it needs, because
`audit_accessibility` reads them. F-X048 reimplements Issue 42 and PR 43 on the
current transactional, bounded cache model rather than importing the stacked
draft branch. The two release stories run last and retain separate final
approval boundaries. Incubating 0.5.0 publishes before stable 0.9.0 so every
stable dependency pin resolves from crates.io. Both reviewed changelog sections
credit and link every included external issue and pull request, and each record
receives a maintainer release note for its contributor.

### M18, Format breadth

#### Sprint S54, RTF and caller font aliases

**Goal**: open M18 with the inbound format that blocks the most corpora, and
honour the document-facing family names supplied with caller fonts.

| F-ID | Title | Size |
|------|-------|------|
| F-176 | RTF reader | L |
| F-177 | RTF writer | M |
| F-183 | Image export options | S |
| F-X051 | Honor caller-supplied font family aliases | M |

F-183 rides along because it is a day of work against an entry point every
format in this milestone shares. F-X051 closes Issue 44 and PR 45 on the
hardened reusable-engine path from F-X043. It remains independent of the RTF
reader and writer.

#### Sprint S55, HTML and ODT in

**Goal**: add the two remaining inbound formats, restore the v0.9 editor path
to the reviewed relayout budget, and close the associated migration and
contribution records.

| F-ID | Title | Size |
|------|-------|------|
| F-178 | HTML import | L |
| F-179 | ODT reader | L |
| F-X052 | Restore interactive relayout performance | L |
| F-X053 | Complete layout migration and contribution records | S |

F-178, F-179, and F-X052 are independent. HTML import is the most requested
inbound conversion in every comparable library's tracker, and ODT is
procurement-mandated across European public bodies and read by nothing in
Rust. F-X052 closes the separate performance residual reported in Issue 46
without weakening the exact, transactional, bounded cache contract. F-X053
runs after it, records the `MarkedContent` migration requirement, and closes
Issue 44, Issue 46, and PR 45 with the exact F-X051 and F-X052 evidence.
Issues 39 and 42 remain closed because their reporter confirmed that note
operations recovered and F-X048 fully replaced the dense-form patches.

#### Sprint S56, ODT, EPUB and SVG out

**Goal**: close M18, integrate the six pending reader contributions, and
publish the stable family at its next pre-1.0 beta boundary.

| F-ID | Title | Size |
|------|-------|------|
| F-180 | ODT writer | L |
| F-181 | EPUB export | M |
| F-182 | SVG page export | M |
| F-X054 | Integrate PRs 47 through 52 | L |
| F-X055 | Tag v0.10.0 | S |
| F-X056 | Tag rpptx-v0.6.0 | S |
| F-X057 | Tag v0.10.1 | S |

F-181 and F-182 both fall out of work that exists: EPUB from the outline API,
SVG from the same `PageFrame` the PDF and PNG backends already consume.
F-X054 audits the ordered reader APIs and parser fidelity changes proposed by
PRs 47 through 52 against current main, retaining six distinct contribution
records and authenticated credit to `@pedroassumpcao`. F-X055 runs only after
all four implementation stories complete. Its immutable v0.10.0 attempt
published only `rdocx-opc` and `rdocx-oxml` before package verification
stopped. F-X057 owns the complete stable publication at v0.10.1 with reviewed
notes, compatibility guidance, direct record links, and contributor credit.
It also owns all nine notifications and the six authorized pull-request
closures after the complete publication and release body verify.

The immutable v0.10.0 attempt published `rdocx-opc` and `rdocx-oxml`, then
proved that the stable source graph requires shared layout APIs newer than the
published incubating 0.5.0 family. F-X055 is retained as an archived failed
release attempt. F-X056 publishes the complete shared and Presentation family
at 0.6.0 while stable packages remain at 0.10.0. F-X057 then publishes the
complete stable family at 0.10.1 against those verified shared dependencies.
Each tag retains its own reviewed SHA, notes, full gate, and separate final
approval.

### M20, Fidelity at scale

#### Sprint S57, The Word corpus

**Goal**: measure the Word renderer against documents nobody here wrote. This is
the largest untested surface in the workspace.

| F-ID | Title | Size |
|------|-------|------|
| F-196 | Word corpus | M |
| F-197 | Word SSIM harness | L |
| F-201 | Large document performance | L |

PowerPoint has 50 fetched decks and an SSIM harness. Word has seven samples this
project generates itself, so it can only catch a regression against its own
output and can never catch a disagreement with Word.

#### Sprint S58, Text shaping and incremental layout

**Goal**: close M20 and finish every planned non-spreadsheet capability before
the advanced spreadsheet programme begins.

| F-ID | Title | Size |
|------|-------|------|
| F-198 | Hyphenation | L |
| F-199 | Complex script shaping | L |
| F-200 | Vertical and bidirectional text | M |
| F-202 | Incremental layout | L |
| F-X061 | Support staged dependency checkpoints in run-sprint | S |
| F-X062 | Reuse restart pagination with notes and headers | M |
| F-X063 | Avoid duplicate caller-font byte comparisons | S |
| F-X058 | Shared multilingual text substrate | L |
| F-X059 | Tag rpptx-v0.7.0 | S |
| F-X064 | Accept whole-valued decimal table measurements | S |
| F-X067 | Prime Word fidelity Cargo dependencies | S |
| F-X065 | Expose tracked table grid changes | S |
| F-X066 | Classify legacy VML horizontal rules | S |
| F-X060 | Tag v0.11.0 | S |
| F-X068 | Tag rpptx-v0.8.0 | S |
| F-X069 | Tag v0.11.1 | S |
| F-X070 | Yank incomplete v0.11.0 packages | S |
| F-X031 | Require the CI gate in branch protection | S |

F-198 changes line breaking and therefore every line after the first hyphenated
one, so it lands after the corpus exists to measure it. Expect a declared hash
harness delta, and expect it to be large.

F-X061 makes ordinary and release dependency checkpoints resumable inside one
sprint. F-X062 and F-X063 close the two independently reproduced editor
performance cliffs from Issues 53 and 54. F-X058 then establishes the complete
shared text contract needed by F-198,
F-199, and F-200. F-X059 publishes that contract as the incubating 0.7.0 family
before the stable Word consumers verify against crates.io. F-X064 lands the
first external reader fix, and F-X067 primes the locked Word fidelity graph
before F-X065 and F-X066 land the remaining reader fixes. Together these are
the hardened or directly adopted outcomes of PRs 55 through 58 before the
paused Word text stories resume. F-X060 records the immutable partial v0.11.0
attempt, which published two packages before registry verification exposed the
missing shared source boundary. F-X068 publishes that shared boundary at
0.8.0, and F-X069 publishes the complete stable recovery at 0.11.1. F-X070
then yanks the two incomplete 0.11.0 registry entries after separate approval
without moving the tag. Each release retains the separate final approval
required by `/release`.

F-X031 remains at the non-spreadsheet boundary after the recovery and cleanup.
F-X029 creates the stable
repository-side `ci-gate` in S44. This operational story makes it a required
GitHub check after the Word fidelity and release gates have settled and before
the larger spreadsheet programme starts.

### M21, Presentation depth

#### Sprint S59, Presentation collaboration and security

**Goal**: deepen the package and collaboration surfaces before adding dynamic
rendering behavior.

| F-ID | Title | Size |
|------|-------|------|
| F-217 | Presentation collaboration and navigation model | L |
| F-221 | Presentation encryption and signatures | M |

The sprint adds comments, replies, sections, footer metadata, protected package
handling, and signature policy. Executable payloads are preserved and
inspectable, never run. Binary `.ppt` remains out of scope.

#### Sprint S60, Animation and transitions

**Goal**: turn preserved timing XML into deterministic, bounded behavior.

| F-ID | Title | Size |
|------|-------|------|
| F-213 | Animation and transition timing model | L |
| F-214 | Timeline evaluation and transition rendering | L |

Static rendering remains unchanged. The new timeline path evaluates supported
entrance, exit, emphasis, motion, transition, and morph behavior at explicit
timestamps, with unsupported extensions preserved and diagnosed.

#### Sprint S61, Audio, video and animated export

**Goal**: complete the media timeline from package relationships to output.

| F-ID | Title | Size |
|------|-------|------|
| F-215 | Audio and video package model | L |
| F-216 | Media poster and playback rendering | M |
| F-227 | Animated GIF and video export | L |

Codec handling is bounded to reviewed backends. Unsupported media remains
extractable and visible through poster frames and diagnostics rather than being
dropped.

#### Sprint S62, SmartArt, embedded content, and reader contributions

**Goal**: model the two major opaque PresentationML surfaces without executing
untrusted content.

| F-ID | Title | Size |
|------|-------|------|
| F-218 | Embedded object and macro inventory | L |
| F-219 | SmartArt typed model | L |
| F-X071 | Integrate PRs 61 through 64 | L |

OLE, ActiveX, and VBA remain inventory, extraction, replacement, removal, and
preservation surfaces. SmartArt gains typed inspection and editing. F-X071
independently adopts the reviewed Word
reader contributions from PRs 61 through 64 and hardens namespace, schema-order,
and effective-style edge cases before integration.

#### Sprint S63, ODP, notes and handouts

**Goal**: broaden modern presentation interchange, complete the presenter and
audience output surfaces, and remove two reporter-confirmed Word editor cache
cliffs.

| F-ID | Title | Size |
|------|-------|------|
| F-220 | SmartArt layout and rendering | L |
| F-222 | ODP read and write | L |
| F-223 | Modern presentation package variants | M |
| F-226 | Notes and handout export | M |
| F-X072 | Keep paragraph caching across note references | M |
| F-X073 | Restart ordinary-prose pagination within the aggregate cache | L |

F-220 completes the carried SmartArt validator boundary before F-222 consumes
SmartArt through ODP interchange. ODP uses a declared LibreOffice differential
boundary. Notes pages and handouts reuse the existing master hierarchy and
shared PDF and image backends. Modern macro, template, and slide-show variants
build on the embedded-content inventory from S62.

F-X072 and F-X073 address issues 65 and 66 sequentially. The first keeps
paragraph-cache reads enabled after note-reference paragraphs. The second lets
ordinary prose publish bounded restart checkpoints and charges them against the
existing aggregate cache budget.

#### Sprint S64, HTML and PDF slide import

**Goal**: turn common modern content sources into editable or explicitly
preserved slide content.

| F-ID | Title | Size |
|------|-------|------|
| F-224 | HTML slide content import | L |
| F-225 | PDF page content import | L |
| F-X074 | Tag rpptx-v0.9.0 | S |
| F-X075 | Preserve restart pagination across page-spanning paragraphs | M |
| F-X076 | Tag v0.12.0 | S |

HTML maps a bounded DOM and CSS subset into shapes. PDF imports either a
preserved page graphic or the declared editable text, image, path, and link
subset. Neither path promises arbitrary browser or PDF-engine compatibility.
F-X074 publishes the completed M21 boundary as the exact 15-package
incubating 0.9.0 family after review and separate final approval.
F-X075 is a late cross-cutting correction for Issue 67. It preserves the
recorded pagination pass across page-spanning ordinary prose while retaining
only complete-block restart checkpoints and every existing unsafe-state
exclusion. Its stable package release follows the presentation release under a
separate reviewed release story. F-X076 publishes that exact stable family at
0.12.0 with the reviewed PR 61 through 64 and Issue 65 through 67 contribution
inventory.

### M22, Word depth

#### Sprint S65, OfficeMath

**Goal**: author, convert, lay out, and render modern Word equations.

| F-ID | Title | Size |
|------|-------|------|
| F-228 | OfficeMath model and authoring | L |
| F-229 | OfficeMath layout and PDF rendering | M |
| F-230 | MathML and LaTeX conversion | M |

The equation model preserves unsupported siblings and remains independent of
legacy Equation Editor objects.

#### Sprint S66, Fields and dynamic tables of contents

**Goal**: update the field-driven structures real reports depend on.

| F-ID | Title | Size |
|------|-------|------|
| F-231 | Extended field evaluation | L |
| F-232 | Dynamic table of contents rebuild | L |

TOC, TC, formula, mail-merge control, and barcode fields retain unsupported
instructions and cached results rather than substituting guessed values.

#### Sprint S67, Advanced automation and comparison

**Goal**: deepen the two flagship document-automation workflows.

| F-ID | Title | Size |
|------|-------|------|
| F-233 | Advanced mail merge | L |
| F-234 | Full-story document comparison | L |
| F-235 | Comparison granularity and ignore policy | M |

Mail merge gains nested regions, multiple data sources, images, fragments, and
format hooks. Comparison expands across related stories and adds explicit word,
character, and ignore policies.

#### Sprint S68, Embedded content, forms and building blocks

**Goal**: expose modern package content that is currently preserved but opaque.

| F-ID | Title | Size |
|------|-------|------|
| F-236 | Embedded object and macro inventory | L |
| F-237 | Forms, glossary, and building blocks | L |

Executable objects and macros remain non-executing inventory surfaces. Legacy
form fields are modeled only where they occur inside modern OOXML packages.

#### Sprint S69, Modern Word package and web variants

**Goal**: close Word depth without opening a legacy `.doc` programme.

| F-ID | Title | Size |
|------|-------|------|
| F-238 | Flat OPC and modern Word package variants | M |
| F-239 | MHTML import and export | M |
| F-X077 | Share strict XML lexical validation | M |
| F-X080 | Restore CI release readiness | S |
| F-X079 | Tag rpptx-v0.10.0 | S |
| F-X078 | Tag v0.13.0 | S |
| F-X081 | Tag rpptx-v0.11.0 | S |
| F-X082 | Tag v0.13.1 | S |

Flat OPC, DOCM, DOTX, DOTM, and bounded MHTML share the current document model.
Binary `.doc` and Word 2003 XML remain permanent non-goals. The shared strict
XML lexical validator removes the three copies exposed by S68 without changing
their fail-closed owner contracts. F-X080 restores the hosted package, Pandoc,
and Python binding gates before either release begins. F-X079 publishes the new
shared API as the incubating family at rpptx-v0.10.0 before F-238 consumes it
through the stable graph. The M22 end gate then runs only modern package
fixtures. F-X078 runs last and prepares and publishes the exact seven-package
stable family at v0.13.0 only after that gate is clean. The immutable v0.13.0
attempt published five low-level packages before its registry graph exposed
F-238's newer `oxml-opc` contract. F-X081 publishes the coherent shared 0.11.0
family, then F-X082 publishes the complete stable recovery at v0.13.1. Each
publication uses `/release` and its own separate final approval at the reviewed
SHA.

### M23, From-scratch business documents

#### Sprint S70, Audit, roadmap, and editor performance

**Goal**: replace the provisional Word-completeness outline with an
evidence-backed capability matrix and final sprint plan, establish the private
and sanitized conformance gates, improve the product README, close confirmed
Issue 67, and land the three independently measured Issue 69 corrections.

| F-ID | Title | Size |
|------|-------|------|
| F-240 | Modern DOCX completeness audit and private corpus matrix | L |
| F-241 | Public authoring conformance harness | L |
| F-242 | Root README product and capability overview | M |
| F-X083 | Close confirmed Issue 67 and intake Issue 69 | S |
| F-X084 | Narrow note-part paragraph cache invalidation | M |
| F-X085 | Memoize restart body identities once per layout | M |
| F-X086 | Provenance-safe restart after body-length changes | L |

F-X083 lands first so Issue 67 is not reimplemented and the live Issue 69
evidence is attributed correctly. F-X084 through F-X086 are independent after
that intake and may run in parallel, but only one story may own a rendering
baseline change. F-240 is the planning authority. F-241 and F-242 consume its
approved matrix, and F-240 updates the provisional sprint contents before
S70 closes. The M24 end gate is S95.

#### Sprint S71, Fresh package, styles, and numbering

**Goal**: make a blank document own the complete package, style, theme, font,
numbering, and deterministic identifier foundation required by the private
business-document corpus.

| F-ID | Title | Size |
|------|-------|------|
| F-243 | Word-compatible fresh package profiles | L |
| F-244 | Corpus settings and document properties | L |
| F-245 | Corpus themes, font tables, and embedded fonts | L |
| F-246 | Corpus style authoring | L |
| F-247 | Complete numbering level and instance model | L |
| F-248 | Style-linked numbering, counters, TOC, and REF | L |
| F-249 | Deterministic package identifier allocation | M |
| F-X087 | Portable authored Word charts from PR 71 | L |
| F-X088 | Verify and close Issue 69 after S70 fixes | S |
| F-X089 | Capability-led README family | L |

F-243 and F-249 establish package identity before dependent parts are authored.
F-245 precedes effective styles. F-247 precedes the two-sided style linkage and
numbering consumers in F-248.

F-X087 through F-X089 are user-approved scope exceptions that raise S71 from
the usual maximum of seven F-IDs to ten. F-X087 hardens Kevin Brown's PR 71 on
the sprint branch after F-245 and F-249. F-X088 verifies the completed S70
Issue 69 mechanisms and closes the issue with contributor credit and reviewed
evidence. F-X089 restores capability-led product and crate messaging while
retaining compiled examples, package evidence, and honest boundaries.

#### Sprint S72, Sections, stories, and content ownership

**Goal**: expose ordered sections and one relationship-safe content model for
the body and every related story required by from-scratch generation. The
user-approved issue wave also restores producer-package compatibility,
searchable PDF text, comparison with drawings, CLI automation, Python access,
and a reviewed Python release path. The final contributor wave also integrates
PRs 77 through 80 and restores the deterministic hosted render oracle.

| F-ID | Title | Size |
|------|-------|------|
| F-250 | Ordered mutable section facade | L |
| F-251 | Complete section and page geometry | L |
| F-252 | Rich per-section headers and footers | L |
| F-253 | Container-neutral story editing | L |
| F-254 | Generic insert, move, clone, and remove operations | L |
| F-255 | Part-scoped assets, links, and relationships | M |
| F-256 | Transactional cross-document fragment import | L |
| F-X090 | Accept part-local producer drawing identities | S |
| F-X091 | Serialize unused root default namespaces safely | M |
| F-X092 | Preserve logical reading order in generated PDFs | L |
| F-X093 | Preserve drawings through document comparison staging | M |
| F-X094a | Expose Word collaboration and redline commands in rdocx-cli | M |
| F-X094b | Structured CLI text and layout plus guarded replacement | L |
| F-X094c | Priority rdocx Python collaboration, comparison, layout, and TOC | L |
| F-X094d | rdocx Python sections, styles, rich stories, and hyperlinks | L |
| F-X094e | rpptx Python rendering, comments, and notes | L |
| F-X095 | Integrate PRs 77 through 80 and restore deterministic CI | L |
| F-X096 | Align Python distribution versions and release tags | M |
| F-X094f | Prepare the version-aligned Python release paths | M |

F-253 is the common owner model for every mutation. Section and relationship
work then converges in F-252, while F-256 lands last against the complete
dependency-remapping surface. F-X090 through F-X093 are one-story corrections
for Issues 72 through 75. F-X094a through F-X094f split Issue 76 into reviewable
CLI, binding, and release-preparation contracts. F-X092 is the only wave that
may move the PDF hash baseline and lands after every unchanged-baseline story.
F-X095 adopts the hardened outcomes of contributor PRs 77 through 80 after the
existing S72 reader, CLI, and binding work. It is the only added story allowed
to move the Word XML hashes, and its declared seven-entry delta must leave PDF
and PNG fingerprints unchanged. F-X096 makes the Python distribution versions
independent and reserves tag namespaces that cannot start a Rust publication.
S72 prepares `py-rdocx-v0.13.2` and `py-rpptx-v0.11.0`, then each publication
remains a separate `/release` action with fresh approval at the reviewed SHA.

#### Sprint S73, Tables, rich content, and private corpus gate

**Goal**: close M23 with complete corpus tables, measure-then-size layout, rich
HTML and run content, modeled drawings and text boxes, deterministic field
materialization, and five public-facade generators starting from a blank
document.

| F-ID | Title | Size |
|------|-------|------|
| F-257 | Complete M23 table authoring | L |
| F-258 | Complete M23 row and cell authoring | L |
| F-259 | Container measurement and equal-height layout | M |
| F-260 | Ordered run content authoring | L |
| F-261 | Rich HTML fragments in arbitrary containers | L |
| F-262 | Corpus drawings, text boxes, and watermarks | L |
| F-263 | Layout-backed fields and M23 corpus gate | L |
| F-X097 | Preserve namespace-scoped drawings and complex fields in comparison | M |
| F-X098 | Preserve content-control type payloads | M |
| F-X099 | Expose direct body ownership for story items | M |
| F-X100 | Preserve explicit false table toggles | S |
| F-X101 | Honor run-level page breaks during pagination | M |
| F-X102 | Resolve header and footer pictures in their story scope | M |
| F-X103 | Accept standard TOC switches and report rebuild diagnostics | M |
| F-X104 | Render DrawingML picture transparency | M |
| F-X105 | Separate slide-owned placeholders from master header flags | M |
| F-X106a | Expose indexed content mutation and counted replacement in Python | L |
| F-X106b | Expose paragraph and run formatting mutations in Python | M |
| F-X106c | Expose story mutation, hyperlinks, revisions, fields, and XML in Python | L |
| F-X107 | Clone and remove existing table rows | M |
| F-X108 | Replace an existing picture atomically | M |
| F-X109 | Split text runs at Unicode character offsets | M |
| F-X110 | Control field updates on document open | S |
| F-X111 | Attach portable CLI binaries to Rust releases | L |
| F-X113 | Preserve appended paragraphs in document comparison | M |
| F-X114 | Rebuild TOC entries with document styles and geometry | M |
| F-X115 | Preserve modern comment metadata and identity | M |
| F-X116 | Make Python story reads linear and complete | L |
| F-X117 | Render transparent and large raster pictures safely | M |
| F-X118 | Make notes rendering and replacement safe | L |
| F-X119 | Complete round-three Python authoring and inspection | L |
| F-X120 | Accept fractional DOCX line spacing values | S |
| F-X121 | Adopt PR 123 authored line-chart portability | S |
| F-X122 | Recover the immutable rpptx 0.12.0 release attempt | M |
| F-X112 | Publish the complete S73 package families | L |

The model stories land before layout and conversion consumers. F-X100 lands
before F-258 so the row and cell setters inherit lossless toggle semantics.
F-X101 and F-X109 follow ordered runs, F-X102 precedes corpus drawings, and
F-X103 precedes layout-backed fields. F-X106a through F-X106c serialize the
second Python binding round, then the row and image mutations consume that
surface. F-X113 and F-X117 are independent corrections. F-X114 follows the
field and private-corpus layout foundations. F-X115 follows exact comment
anchoring, F-X116 follows the complete story binding and split-run paths, and
F-X118 establishes notes mutation before F-X119 closes the round-three binding
surface. F-X120 and F-X121 are independent contributor corrections. F-X122
repairs the failed immutable incubating release attempt and prepares its patch
family. F-263
remains the private-corpus milestone gate. F-X111 prepares portable CLI assets
before F-X112 releases the exact two Rust and two Python families after F-X113
through F-X122 are complete, using separate approvals.
PR 101 contributes the F-X100 explicit
false table-toggle fix and the F-X103 TOC `\\z` support. Its current two-commit
shape is incorporated directly where conflict-free or through a reviewed
hardened equivalent, with contributor credit retained. The five client
documents and their renders remain private and uncommitted.

### M24, Modern DOCX authoring completeness

#### Sprint S74, Complete text, table, section, and settings semantics

**Goal**: expand the corpus-proven primitives into complete public paragraph,
run, table, international typography, page, and document-setting authoring.

| F-ID | Title | Size |
|------|-------|------|
| F-264 | Complete paragraph property authoring | L |
| F-265 | Complete run property and inline authoring | L |
| F-266 | International and vertical typography | L |
| F-267 | Complete table style and conditional formatting authoring | L |
| F-268 | Floating and advanced table layout | L |
| F-269 | Complete section page semantics | L |
| F-270 | Complete settings and web settings authoring | L |
| F-X123 | Accept producer TOC style variants | S |
| F-X124 | Make content cloning linear and explicit | M |
| F-X125 | Compare table grid changes | M |
| F-X126 | Preserve drawings through comparison acceptance | M |
| F-X127 | Collapse adjacent page break requests | S |
| F-X128 | Preserve Word paragraph and revision identities | M |
| F-X129 | Tolerate unmatched notes placeholders | S |
| F-X130 | Show package depth, footprint, and speed | L |

Each model change carries its own parser, writer, public API, and render gate.
F-266 consumes the completed paragraph and run properties, while F-268 consumes
the table and conditional-style surface. F-X123 through F-X129 resolve Issues
124 through 132 before those planned surfaces expand. Issues 126 and 132 share
F-X124 because they exercise the same public clone call. Issue 68 is an
answered roadmap question and requires no implementation story.
F-X130 follows F-264 through F-270 so all 27 package READMEs and both PyPI long
descriptions can present the completed S74 surface with reproducible feature,
footprint, and performance evidence.

#### Sprint S75, Contribution intake and table pagination correction

**Goal**: restore the hosted Python binding gate, integrate the open
contribution wave and its unassigned issue reports, then finish Issue 138's
table pagination correction before one reviewed mid-milestone merge.

| F-ID | Title | Size |
|------|-------|------|
| F-X134 | Keep Python story hyperlink snapshots linear | S |
| F-X135 | Integrate PRs 146 through 151 and resolve unassigned reports | L |
| F-X136 | Fix table row breaks and footer-only pages | L |

F-X134 restores the hosted Python binding gate before F-X135 integrates the
six remaining contributor branches and repairs open reports without a pull
request. F-X136 finishes the distinct Issue 138 line-height and row-splitting
report after F-X135's spacing and table-margin changes. F-X133 and F-271
through F-277 were carried to the then-planned S76 at the approved cutoff.
The unstarted S76 stories are now scheduled from S86 after repair intake.

#### Sprint S76, Contribution intake and core repair

**Goal**: review the open contribution wave at its own commits, reconcile
overlapping Word and package changes, and land the first integrated repair
prefix. A PR is evidence for a story, not a substitute for its acceptance gate.

| F-ID | Title | Size |
|------|-------|------|
| F-X137 | Package and CLI safety contribution wave | L |
| F-X138 | Word story and content contribution wave | L |
| F-X139 | Word identity and comparison contribution wave | L |

F-X137 reviews PRs 174, 178, 182, 185 and 197. F-X138 reviews PRs
177, 179, 180, 191, 195, 202, 210 and 211 in that dependency order.
F-X139 reviews PRs 183, 184, 190, 193, 198 and 205. Integrate one reviewed
contribution at a time on `sprint/s76`, separating behavior changes from
file moves and reconciling semantic overlap manually.
Every integrated prefix reruns the relevant focused checks, then the final
combined result takes `/verify --full` and `/sprint-review`.

#### Sprint S77, Binding and CLI contribution completion

**Goal**: integrate the remaining reviewed rendering, layout, Python,
presentation and CLI contributions on the completed S76 prefix, with one
combined verification and review boundary.

| F-ID | Title | Size |
|------|-------|------|
| F-X140 | Rendering and layout contribution wave | L |
| F-X141 | Word Python contribution wave | L |
| F-X142 | Presentation Python contribution wave | L |
| F-X143 | Revision listing and CLI story contribution wave | M |

F-X140 reviews PRs 175, 188, 196, 199, 200, 206 and 207. It owns the
first labelled and reviewed hash baseline update. F-X141 owns a second,
separate baseline update for native common styles and any reviewed TOC output
delta. No other S77 story may move the baseline.
PR 196's Presentation fidelity gate and PR 206's MSRV and Test failures must
be resolved on the replayed integrated result before sprint closure. F-X141
reviews PRs 176, 187, 194, 201, 203 and 212. PR 203 follows 201. F-X142
reviews PRs 173, 181, 189, 192, 208 and 209. PRs 208
and 209 follow 189. F-X141 also completes the named section-default, native
common-style and refreshable TOC gaps beyond those PRs. F-X142 completes the
full Issue 169 checklist, including the API and rendering items outside its
PR set. F-X143 reviews PRs 186 and 204, with 204 following
S76 PR 198. Replay only each stacked PR's incremental diff after its
parent lands, run focused checks on the replayed result, and require a rebase
and green head CI for any original PR selected for a direct merge. Close a
superseded PR only after its coverage is verified on `main`. Binding smoke,
package, rendering, documentation and release regression checks run on the
combined result, followed by `/verify --full` and `/sprint-review`.

#### Sprint S78, Word producer and comparison repair

**Goal**: land the new Word preservation and comparison contributions, then
finish the identity, producer and redline acceptance matrices.

| F-ID | Title | Size |
|------|-------|------|
| F-X151 | Word preservation and comparison PR intake | L |
| F-X144 | Identity and producer matrices across operations | L |
| F-X145 | Comparison options and redline completion | L |

F-X151 reviews PRs 214, 228, 229, 232, 233 and 239 against the S77 prefix.
F-X144 follows that integration and completes Issues 157, 159 and 160.
F-X145 then completes Issue 161, including edited-side comments and rebuilt
TOCs. Rebase every old head against its actual parent, review only its
incremental diff, and rerun the combined matrix and full sprint gate.

#### Sprint S79, Word layout, CLI diff and binding repair

**Goal**: finish the Word layout and CLI defects, then close the remaining
Word Python binding gaps before the production fixture gate.

| F-ID | Title | Size |
|------|-------|------|
| F-X146 | Word line height and inline picture spacing | L |
| F-X152 | Full-story CLI diff and count repair | M |
| F-X153 | Word Python supplemental contribution | M |

F-X146 reviews PRs 222, 225 and 237 and completes Issue 162 and Issue
226's rich-line symptom. F-X163 completes its plain-line symptom after PR
242. PR 222's current head reports a green CI run, and the integrated
fidelity cases must still pass after rebasing.
F-X152 reviews PR 236 and completes Issue 227. F-X153 reviews PR 220,
including its Issue 168 comment and run operations. The Word fixture and
focused layout oracles run on the integrated result before `/verify --full`
and `/sprint-review`.

#### Sprint S80, Word checklist and presentation preservation

**Goal**: complete the Word Python production checklist and land the new
presentation preservation and drawing contributions on a reviewed prefix.

| F-ID | Title | Size |
|------|-------|------|
| F-X147 | Complete rdocx Python production checklist | L |
| F-X154 | Presentation text and preservation repair | M |
| F-X155 | Presentation drawing API contribution | L |

F-X147 completes every Issue 168 item after F-X141 and F-X153 or records a
reviewed scope decision with a documented fallback where the reporter permits
one. F-X154 reviews PRs 218 and 223 and completes Issues 215 and 216.
F-X155 reviews PRs 219, 221, 224, 230 and 234. PR 219 follows PR 189,
and PR 234 follows PR 207. Their shape, line, effect and binding overlap
requires manual reconciliation and a fresh CI run.

#### Sprint S81, Presentation composition and deck checklists

**Goal**: finish the presentation authoring surface and prove the complete
Issue 169 and Issue 217 deck checklists against independent viewers.

| F-ID | Title | Size |
|------|-------|------|
| F-X156 | Presentation slide and table contribution | L |
| F-X148 | Complete rpptx Python production checklist | L |
| F-X157 | Complete deck-chain authoring checklist | L |

F-X156 reviews PRs 231, 235 and 238. PR 231 follows PR 181. F-X148 checks
all Issue 169 items, including shape hyperlinks, anchored comments and
built-in table styles, after F-X142 and F-X156. F-X157 completes Issue 217,
including outer shadows, connectors without theme effects, line ends, preset
geometry, cross-deck import and scoped replacement. Both checklist stories
require Python API, package validation, python-pptx reopening, LibreOffice
render comparison and explicit scope decisions for any unsupported operation.

#### Sprint S82, Production acceptance and issue closure evidence

**Goal**: turn the reporter's Word and deck fixtures into repeatable acceptance
checks, then reconcile every open issue criterion against the integrated result.

| F-ID | Title | Size |
|------|-------|------|
| F-X149 | Word fixture and workflow acceptance gate | L |
| F-X158 | Presentation fixture and workflow acceptance gate | L |
| F-X150 | Reconcile issue closure evidence | M |

F-X149 owns the 18 by 7 identity matrix, 11 by 8 producer matrix and complete
DOCX workflow. F-X158 owns the deck workflow and cross-viewer acceptance for
Issues 169, 170, 215, 216 and 217. F-X150 records criterion-level evidence
for the 22 issues present in its original snapshot, with Issue 158 last. A
missing criterion keeps its issue open. The new issues and contributions from
1 October are scheduled below, ahead of the deferred feature work.

#### Sprint S83, Word package validity and tolerant reads

**Goal**: make newly authored Word documents open in Word and accept the
reported style, drawing ID and measurement producer variants. Review one
separate baseline change for document validity before the tolerance work.

| F-ID | Title | Size |
|------|-------|------|
| F-X159 | Word document validity baseline | M |
| F-X160 | Tolerant style, drawing and measurement reads | L |

F-X159 reviews PR 240 and owns this sprint's only hash baseline update.
F-X160 reviews PRs 248, 249 and 250 against that prefix. It closes Issues
243, 246 and 247 only after both Issue 243 reporter cases and every affected
mutator or read path pass on the integrated result. Word opening, exact XML,
round-trip, CLI and Python checks accompany the deterministic hash gate.

#### Sprint S84, Integrated contribution and issue repair

**Goal**: finish the current contribution queue and every remaining open issue
criterion in one integrated sprint. Run scoped feature checks and dependency
checkpoints during implementation, then one full verification and sprint review
on the combined result.

| F-ID | Title | Size |
|------|-------|------|
| F-X169 | Reconcile live contributions and open issue contracts | L |
| F-X161 | Compact Word XML and namespace preservation | L |
| F-X162 | Per-paragraph section width and pagination | M |
| F-X163 | Plain-line trailing-space fit | M |
| F-X164 | Added shape theme style | M |
| F-X165 | Python and CLI tracked revision view | M |
| F-X166 | Picture and final-block comparison revisions | L |
| F-X167 | Accepted-view exporters and readers | L |
| F-X170 | High-level Word style formatting API | M |
| F-X168 | Current issue and contribution closure evidence | M |

F-X169 classifies all 31 open PRs against integrated main before implementation.
The 19 older heads need patch-equivalence and contributor disposition checks.
The 12 newer heads belong to F-X161 through F-X167. Complete F-X169 at a
scoped dependency-prefix checkpoint so F-X161, F-X164 and F-X165 may start.

F-X161 reviews PR 251 first and owns its labelled 20-entry OOXML hash delta.
F-X162 follows F-X161 with PR 241 and a separate section-layout baseline.
F-X163 follows F-X162 with PR 242, its separate hash delta and golden pixel
manifest. The three baseline changes are exclusive and reviewed in order.
F-X164 reviews PR 252 independently after intake, with python-pptx and viewer
checks. F-X165 reviews PR 256, then F-X166 reconciles PRs 257, 258, 260 and
261. F-X167 follows F-X165 and F-X166 with PRs 259, 262 and 263. F-X170
addresses Issue 264, opened after the 2 October intake, independently. F-X168
checks every open issue against the final integrated result. Formal
prerequisites complete through scoped checkpoints before their consumers
start. These checkpoints do not run the full workspace or sprint review.

PR 265 arrived on 3 October after F-X162 completed. It targets the same section
ownership defect as PR 241. F-X168 adopts its distinct mixed-orientation,
multipage table and content-control regression on the integrated F-X162 fix,
then records a superseded disposition for PR 265. Its head has no CI run and
reports incomplete local full-gate coverage, so the integrated sprint gate
supplies the missing evidence.

The ten F-IDs represent about 29 to 38 estimated developer-days, above the
normal sprint size by request. Keep their approved designs, individual
microscope reviews and scoped verification. Run the union of risk riders,
source-built round trips, reporter fixtures, the 18 by 7 identity and 11 by 8
producer matrices, Python and CLI checks, deterministic Word and PowerPoint
comparisons, the hash harness, `/verify --full` and `/sprint-review` once on
the final integrated S84 result. After `/close-sprint` merges that result to
main, close all 32 open PRs and nine open issues through the 3 October refresh
with evidence links. Feature work resumes at S86.

#### Live contribution and issue inventory, 2 October 2026

The 2 October intake reported 31 open PRs and eight open issues. Every PR in
that intake targets
`main`, has a green latest head CI run, has no submitted review, and is marked
conflicting with current `main`. PRs #179 and #259 retain failed historical
runs in the rollup, but their latest checks pass. Head CI does not prove that
the combined result passes. The source overlap column shows one current PR
with overlapping source files, not every possible conflict.

| PR | Base | Head branch and SHA | Latest CI | Review | Mergeability | Stack | Source overlap | F-ID | Issues |
|---|---|---|---|---|---|---|---|---|---|
| [#174](https://github.com/tensorbee/rdocx/pull/174) | `main` | `fix/cli-output-overwrite-and-closed-pipe` / `0f108ffb90` | green | none | conflict | - | #256 (`crates/rdocx-cli/src/main.rs`) | F-X137 | [156](https://github.com/tensorbee/rdocx/issues/156)*, [166](https://github.com/tensorbee/rdocx/issues/166)* |
| [#177](https://github.com/tensorbee/rdocx/pull/177) | `main` | `fix/body-readers-reach-content-controls` / `367c48ace8` | green | none | conflict | - | #263 (`crates/rdocx/src/document.rs`) | F-X138 | [160](https://github.com/tensorbee/rdocx/issues/160) |
| [#178](https://github.com/tensorbee/rdocx/pull/178) | `main` | `fix/atomic-library-save` / `4e582fcb09` | green | none | conflict | - | #193 (`crates/rdocx/src/document.rs`) | F-X137 | [164](https://github.com/tensorbee/rdocx/issues/164)* |
| [#179](https://github.com/tensorbee/rdocx/pull/179) | `main` | `fix/direct-body-index-coordinates` / `128e61bdee` | green | none | conflict | - | #191 (`crates/rdocx/src/document.rs`) | F-X138 | [163](https://github.com/tensorbee/rdocx/issues/163) |
| [#180](https://github.com/tensorbee/rdocx/pull/180) | `main` | `fix/text-box-rewrite-keeps-all-children` / `242e9984b3` | green | none | conflict | - | #211 (`crates/rdocx-oxml/src/placeholder.rs`) | F-X138 | [160](https://github.com/tensorbee/rdocx/issues/160) |
| [#182](https://github.com/tensorbee/rdocx/pull/182) | `main` | `fix/story-splice-namespace-facts` / `46c128e3c9` | green | none | conflict | - | #263 (`crates/rdocx/src/document.rs`) | F-X137 | [157](https://github.com/tensorbee/rdocx/issues/157) |
| [#183](https://github.com/tensorbee/rdocx/pull/183) | `main` | `fix/toc-rebuild-unbound-word-prefix` / `1bed176b32` | green | none | conflict | - | #193 (`crates/rdocx-oxml/src/text.rs`) | F-X139 | [159](https://github.com/tensorbee/rdocx/issues/159) |
| [#184](https://github.com/tensorbee/rdocx/pull/184) | `main` | `fix/compare-producer-noise` / `83b92bbf69` | green | none | conflict | - | #211 (`crates/rdocx-oxml/src/placeholder.rs`) | F-X139 | [159](https://github.com/tensorbee/rdocx/issues/159), [160](https://github.com/tensorbee/rdocx/issues/160) |
| [#185](https://github.com/tensorbee/rdocx/pull/185) | `main` | `fix/rewritten-part-roots-and-comments-part` / `88a4838704` | green | none | conflict | - | #193 (`crates/rdocx/src/document.rs`) | F-X137 | [160](https://github.com/tensorbee/rdocx/issues/160) |
| [#190](https://github.com/tensorbee/rdocx/pull/190) | `main` | `fix/compare-diagnostics-instead-of-refusals` / `4e0be1e8ea` | green | none | conflict | #184 | #211 (`crates/rdocx-oxml/src/placeholder.rs`) | F-X139 | [159](https://github.com/tensorbee/rdocx/issues/159), [160](https://github.com/tensorbee/rdocx/issues/160), [161](https://github.com/tensorbee/rdocx/issues/161) |
| [#191](https://github.com/tensorbee/rdocx/pull/191) | `main` | `fix/comment-anchoring-run-index` / `573d8c670a` | green | none | conflict | #179 | #211 (`crates/rdocx/src/document.rs`) | F-X138 | [163](https://github.com/tensorbee/rdocx/issues/163), [172](https://github.com/tensorbee/rdocx/issues/172) |
| [#193](https://github.com/tensorbee/rdocx/pull/193) | `main` | `fix/table-row-identity-attributes` / `7a8841fbcb` | green | none | conflict | #183 | #211 (`crates/rdocx-oxml/src/footnotes.rs`) | F-X139 | [159](https://github.com/tensorbee/rdocx/issues/159) |
| [#195](https://github.com/tensorbee/rdocx/pull/195) | `main` | `fix/replace-reaches-content-controls` / `bb76c4bd35` | green | none | conflict | #184 + #180 | #211 (`crates/rdocx/src/document.rs`) | F-X138 | [160](https://github.com/tensorbee/rdocx/issues/160) |
| [#197](https://github.com/tensorbee/rdocx/pull/197) | `main` | `fix/template-saved-as-document-content-type` / `8cdfd4a913` | green | none | conflict | - | #263 (`crates/rdocx/src/document.rs`) | F-X137 | - |
| [#198](https://github.com/tensorbee/rdocx/pull/198) | `main` | `feat/cli-compare-options-comment-dates` / `ff24553e60` | green | none | conflict | - | #263 (`crates/rdocx/src/document.rs`) | F-X139 | [161](https://github.com/tensorbee/rdocx/issues/161) |
| [#202](https://github.com/tensorbee/rdocx/pull/202) | `main` | `fix/replace-reaches-notes-and-tracked-insertions` / `5c3d81cf90` | green | none | conflict | #195 | #211 (`crates/rdocx-oxml/src/footnotes.rs`) | F-X138 | [160](https://github.com/tensorbee/rdocx/issues/160) |
| [#205](https://github.com/tensorbee/rdocx/pull/205) | `main` | `fix/comparison-marker-unit-boundaries` / `7704245c99` | green | none | conflict | - | #211 (`crates/rdocx-oxml/src/text.rs`) | F-X139 | [161](https://github.com/tensorbee/rdocx/issues/161) |
| [#210](https://github.com/tensorbee/rdocx/pull/210) | `main` | `fix/word-text-boxes-read-and-counted-once` / `7d8e03b7d1` | green | none | conflict | #195 | #211 (`crates/rdocx/src/document.rs`) | F-X138 | [160](https://github.com/tensorbee/rdocx/issues/160) |
| [#211](https://github.com/tensorbee/rdocx/pull/211) | `main` | `fix/text-inside-simple-fields-smart-tags-custom-xml` / `bce7ebcd26` | green | none | conflict | #202 | #202 (`crates/rdocx-oxml/src/footnotes.rs`) | F-X138 | [160](https://github.com/tensorbee/rdocx/issues/160) |
| [#241](https://github.com/tensorbee/rdocx/pull/241) | `main` | `fix/word-section-width-per-paragraph` / `09dd97d614` | green | none | conflict | - | #263 (`crates/rdocx-layout/src/engine.rs`) | F-X162 | - |
| [#242](https://github.com/tensorbee/rdocx/pull/242) | `main` | `fix/word-plain-line-fit-hangs-trailing-space` / `14358319b4` | green | none | conflict | #222, already integrated | #263 (`crates/rdocx/src/document.rs`) | F-X163 | [226](https://github.com/tensorbee/rdocx/issues/226) |
| [#251](https://github.com/tensorbee/rdocx/pull/251) | `main` | `fix/rewritten-parts-keep-word-layout` / `2ae5d6189f` | green | none | conflict | - | #193 (`crates/rdocx-oxml/src/footnotes.rs`) | F-X161 | [245](https://github.com/tensorbee/rdocx/issues/245) |
| [#252](https://github.com/tensorbee/rdocx/pull/252) | `main` | `fix/rpptx-added-shapes-get-the-theme-style` / `966b83ae7c` | green | none | conflict | - | #178 (`crates/rpptx/src/odp.rs`) | F-X164 | [244](https://github.com/tensorbee/rdocx/issues/244) |
| [#256](https://github.com/tensorbee/rdocx/pull/256) | `main` | `feat/revision-view-python-cli` / `ad088becbf` | green | none | conflict | - | #198 (`crates/rdocx-cli/src/main.rs`) | F-X165 | [253](https://github.com/tensorbee/rdocx/issues/253) |
| [#257](https://github.com/tensorbee/rdocx/pull/257) | `main` | `fix/compare-changed-picture` / `e6562e7821` | green | none | conflict | - | #211 (`crates/rdocx/src/document.rs`) | F-X166 | [254](https://github.com/tensorbee/rdocx/issues/254) |
| [#258](https://github.com/tensorbee/rdocx/pull/258) | `main` | `fix/compare-final-table-and-paragraph` / `42bd34f231` | green | none | conflict | - | #261 (`crates/rdocx/src/revision.rs`) | F-X166 | [255](https://github.com/tensorbee/rdocx/issues/255) |
| [#259](https://github.com/tensorbee/rdocx/pull/259) | `main` | `fix/html-markdown-export-tracked-changes` / `5bc41cb280` | green | none | conflict | - | #263 (`crates/rdocx-html/src/lib.rs`) | F-X167 | [253](https://github.com/tensorbee/rdocx/issues/253) (follow-up) |
| [#260](https://github.com/tensorbee/rdocx/pull/260) | `main` | `fix/reject-paragraph-property-change-keeps-mark` / `39a9c9d97b` | green | none | conflict | - | #261 (`crates/rdocx/src/revision.rs`) | F-X166 | [255](https://github.com/tensorbee/rdocx/issues/255) (dependency) |
| [#261](https://github.com/tensorbee/rdocx/pull/261) | `main` | `fix/compare-final-paragraph-keeps-properties` / `cdfcefbe95` | green | none | conflict | #258 + #260 | #260 (`crates/rdocx/src/revision.rs`) | F-X166 | [255](https://github.com/tensorbee/rdocx/issues/255) |
| [#262](https://github.com/tensorbee/rdocx/pull/262) | `main` | `fix/accepted-view-drops-deleted-paragraphs` / `32a12735ac` | green | none | conflict | #259 | #193 (`crates/rdocx/src/document.rs`) | F-X167 | [253](https://github.com/tensorbee/rdocx/issues/253) (follow-up) |
| [#263](https://github.com/tensorbee/rdocx/pull/263) | `main` | `fix/accepted-view-drops-deleted-rows` / `6252b10a8d` | green | none | conflict | - | #259 (`crates/rdocx-html/src/lib.rs`) | F-X167 | [253](https://github.com/tensorbee/rdocx/issues/253) (follow-up) |

The 19 older open heads, #174 through #211 in the table, predate the
completed S76 integration commits. The S76 AS_BUILT entries explicitly list
every one of these PRs and say that unique behavior commits were replayed while
measurement-only tails and duplicate ancestors were omitted. The integration
commits below are ancestors of the current sprint branch, and GitHub reports no
later revision to these PR heads. This is behavioral adoption, not identical
patch text. Their final disposition is superseded by the integrated work after
the combined S84 gates pass. Do not replay their stale branches.

| Older PRs | Current disposition | Integrated evidence | Final check |
|---|---|---|---|
| #174, #178, #182, #185, #197 | Behavior integrated, archive measurement tails omitted | `26b3f880`, F-X137 in `AS_BUILT.md` | CLI, package and save regressions on final main |
| #177, #179, #180, #191, #195, #202, #210, #211 | Behavior integrated, duplicate ancestors and measurement tails omitted | `c8975fb4`, F-X138 in `AS_BUILT.md` | Word story, replacement and Issue 160 matrix on final main |
| #183, #184, #190, #193, #198, #205 | Behavior integrated, measurement tails omitted | `992930ce`, F-X139 in `AS_BUILT.md` | Identity, comparison and CLI regressions on final main |

If a final gate exposes an omitted behavior, reconcile it under an approved
F-ID and revise this disposition before any PR closes. Contributor closure
comments must name the adopted integration commit and the final test evidence.

For pending work, review and replay #251 first, then #241, then #242, with
separate hash baseline commits and deterministic render evidence. #242 includes
#222, already integrated. Review #252 after its already integrated presentation
style prerequisites. Review #256 before the comparison and accepted-view set.
Then review #257, #258, #260, #261 in that order, with #261 depending on both
#258 and #260. Review #259, #262, #263 in that order, with #262 stacked on
#259. Rebase each head onto the reviewed prefix and reconcile shared files
against both approved plans. PR #195 also carries #180's code, so its older
stack must be compared incrementally. PR #257's comment supplies Word for Mac
picture evidence, while #260's comment explains #261's marker-order dependency.

The current conflicts require manual semantic reconciliation in
`rdocx-oxml` serializers, `rdocx-layout` line and section code,
`rdocx/src/comparison.rs`, accepted-view exporters, shared regression test
entrypoints and their HLD sections. Recheck the exclusive `hash_baseline.json`
updates from #251, #241 and #242 one at a time. #242 also changes the golden
pixel manifest. No current head CI job is failing. Missing GitHub reviews,
conflicting bases, and the untested combined result block direct PR merges.

| Open issue | PR coverage | Remaining acceptance and closure evidence | F-ID |
|---|---|---|---|
| [#158](https://github.com/tensorbee/rdocx/issues/158) | Earlier contribution waves, #251 and #242 | Keep both SHA-bound fixtures, the 18 by 7 identity and 11 by 8 producer matrices, and both complete workflows green on integrated main. Close only after #160 and #226 and all child criteria pass. | F-X169, F-X168 |
| [#160](https://github.com/tensorbee/rdocx/issues/160) | Earlier #177, #180, #184, #185, #190, #195, #202, #210, #211 and #251 | Recheck every body walker location, packed fields, no-op comments bytes, `mc:Ignorable` binding, and all 11 by 8 producer cells on serialized output after #251. | F-X161, F-X168 |
| [#226](https://github.com/tensorbee/rdocx/issues/226) | Integrated #222 and pending #242 | Prove plain and explicit `w:rtl=false` line breaks at 1.70 and 2.10 inches, with hanging space and no punctuation or space at line start, against pinned Word or LibreOffice output. | F-X163, F-X168 |
| [#244](https://github.com/tensorbee/rdocx/issues/244) | #252 | Added shape has python-pptx's `p:style`, visible fill and line in rpptx, LibreOffice and PowerPoint, reopens in python-pptx, and accepts theme effect index zero. | F-X164, F-X168 |
| [#245](https://github.com/tensorbee/rdocx/issues/245) | #251 | Edited package keeps untouched parts and regions, compact rewritten XML, one root `w` declaration, valid namespaces and the individually reviewed 20-entry OOXML hash delta. | F-X161, F-X168 |
| [#253](https://github.com/tensorbee/rdocx/issues/253) | #256, related #259, #262, #263 | Python PDF and page render plus both CLI paths select tracked or accepted view, default accepted, reject bad selectors, and give old plus new text versus new only. Check accepted exporters after the follow-ups. | F-X165, F-X167, F-X168 |
| [#254](https://github.com/tensorbee/rdocx/issues/254) | #257 | All three picture cases work at run and word granularities, retain both media parts, and accept or reject to the correct image, caption and size. Repeat the contributor's Word for Mac check on the integrated result. | F-X166, F-X168 |
| [#255](https://github.com/tensorbee/rdocx/issues/255) | #258, #260, #261 | All five final-table pairs work at run and word granularities, accept and reject to exact paragraphs, tables and properties, and open in Word with inserted or deleted table marks. | F-X166, F-X168 |

The 3 October refresh found [#264](https://github.com/tensorbee/rdocx/issues/264),
a new open feature request with no PR. A [3 October response](https://github.com/tensorbee/rdocx/issues/264#issuecomment-5967685523)
explains the active S84 implementation and keeps the issue open. `StyleBuilder` and
`Document` already create, update and manage styles, but simple style formatting
still requires raw `CT_PPr` and `CT_RPr` values. F-X170 owns a focused facade
for common paragraph and run settings, with a save and reopen example. F-X168
must verify the API before closing #264. The PR count and all recorded heads
remain unchanged at 31.

The later 3 October refresh found [PR #265](https://github.com/tensorbee/rdocx/pull/265),
a draft alternate to PR #241. It targets `main` from
`fix/mixed-section-measure` at `177dd7cbc4`, has no CI jobs or submitted
reviews, and GitHub reports it mergeable but blocked as a draft. It overlaps
`crates/rdocx-layout/src/engine.rs` with F-X162 and PR #241, with no stacked
dependency. Its production section selection is already covered by F-X162's
`main_story_item_sections` and the section-width tests. F-X168 will adopt the
new multipage, mixed-orientation table and content-control test on the
integrated implementation, run it under deterministic fonts, and cite the
combined gate before closing #265 as superseded. This makes 32 open PRs and
nine open issues at the latest refresh.
It addresses no open issue directly.

| New open issue | PR coverage | Remaining acceptance and closure evidence | F-ID |
|---|---|---|---|
| [#264](https://github.com/tensorbee/rdocx/issues/264) | None | Show convenient style creation and management through the existing `Document` surface, configure common paragraph and run formatting without constructing `CT_PPr` or `CT_RPr`, and prove that an applied style survives save and reopen. | F-X170, F-X168 |

An issue without a direct PR is #158, the umbrella acceptance contract. All
eight issues from the 2 October intake stay open until every criterion passes on integrated main. The
S84 consolidation exceeds one normal sprint. F-X169 records live intake,
F-X161 through F-X167 and F-X170 implement the remaining slices, and F-X168 closes the
evidence ledger last.
Each F-ID requires focused tests, applicable oracle and risk checks, scoped
verification and a zero-finding microscope. After all increments integrate,
one `/verify --full`, the hash harness and a clean `/sprint-review` precede
`/close-sprint`. The final issue closure check uses both full fixture workflows
and matrices, binding and CLI checks, deterministic render comparisons and the
combined hash baseline. Close every remaining PR and issue after the reviewed
result reaches main and its hosted CI gate passes. The S84 main push exposed a
missing pinned LibreOffice setup in the macOS Python job. F-X171 repairs that
hosted gate before the prepared closures are posted.

#### Sprint S85, Hosted CI repair and contribution closure

**Goal**: restore the required hosted CI gate on the S84 main result, then
reconcile all 32 reviewed PRs and nine issue contracts against the green
integrated result. Keep the published `s84` tag fixed and start the planned
feature work in S86.

| F-ID | Title | Size |
|------|-------|------|
| F-X171 | Pin LibreOffice for macOS Python acceptance | S |

The S84 local full gate and manual viewer evidence passed. The hosted
presentation binding job failed only because it had no `soffice` executable.
Install the exact reviewed LibreOffice image on that job, verify its digest and
build identity, and rerun the full Python suite and aggregate CI gate. Close
the S84 PR and issue queue only after `main` is green. This one-story repair
boundary avoids moving the already published `s84` tag.

#### Sprint S86, Rich related-story foundations and unified releases

**Goal**: establish rich header, footer, and note authoring on the existing
story model, remove repeated canonical-prefix rebinding from retained
elements, and prepare and publish one combined CLI and Python release per
family for Issue 266.

| F-ID | Title | Size |
|------|-------|------|
| F-X133 | Stop rebinding a canonical prefix on every retained element | S |
| F-271 | Uniform rich header and footer editing | L |
| F-272 | Rich footnote authoring | L |
| F-273 | Rich endnote authoring | L |
| F-X172 | Unified release artifacts and provenance | L |
| F-X173 | Prepare unified rpptx 0.13.0 family | M |
| F-X174 | Prepare unified rdocx 0.15.0 family | M |

F-X133 is independent and may proceed alongside F-271. F-272 establishes
the note-authoring model that F-273 extends. The other 11 stories from the
original S76 inventory move together to S87 in dependency order. Issue 266
adds a release pipeline story followed by the incubating and stable release
preparations. After `/close-sprint` merges the reviewed result to `main`, each
tag requires its own final approval at that merge SHA.

#### Sprint S87, Release archive evidence repair

**Goal**: repair the Linux package footprint checks exposed by the S86 main
CI run, then verify a clean release source for both Issue 266 families. The
previously planned related-story work moves to S88 because publication must
follow a reviewed green main merge.

| F-ID | Title | Size |
|------|-------|------|
| F-X175 | Refresh CLI archive evidence after release hardening | S |

The repair updates the two CLI archive measurements after the reviewed Windows
stack correction. Hosted CI, local package evidence, the full gate and a clean
sprint review must pass before `/close-sprint S87`. The two family publications
then follow through separate `/release` approvals at the S87 main merge SHA.

#### Sprint S88, Release inventory and version repair

**Goal**: repair the tag-only asset inventory failure before either registry
publishes Issue 266 packages. Preserve the failed `rpptx-v0.13.0` tag, prepare a
new PowerPoint version and the stable Word version from a reviewed main merge,
and run a build-only rehearsal and the complete gate before fresh approvals.

| F-ID | Title | Size |
|------|-------|------|
| F-X176 | Repair unified release inventory and respin PowerPoint | M |

The tag workflow's publication dry run does not retain `.crate` archives on a
fresh target directory. F-X176 checks the unpacked package inventory instead,
adds a clean-target regression, refreshes version carriers and notes, and
proves both selected release families before `/close-sprint S88`. The related
story wave moves to S89 so publication follows this focused repair.

#### Sprint S89, Related stories and safe fragment reuse

**Goal**: complete note separators and numbering policy, cross-story range
markers, deterministic fragment dependency remapping, and public glossary
creation. These four stories establish the shared story and package semantics
needed by later fields, templates, forms, and collaboration work.

| F-ID | Title | Size |
|------|-------|------|
| F-274 | Note separators, markers, and restart policy | L |
| F-275 | Cross-story bookmarks, ranges, and annotations | L |
| F-276 | Complete fragment conflict and dependency policy | L |
| F-277 | Glossary and building-block creation | L |
| F-X177 | Accept unified fontdb source features | S |

F-274 closes the note policy left by rich footnotes and endnotes. F-275 makes
paired markers valid across stories. F-276 then remaps a full fragment's
conflicting dependencies without dangling IDs, and F-277 uses that transaction
for building-block insertion. The sprint gate includes save-reopen checks for
relationships and untouched XML. This wave moved from S87 through S88 to leave
room for the release repairs.
The user added PR 269 during S89. F-X177 preserves downstream fontdb feature
unification and uses a separate wave between F-276 and F-277 because archive
evidence and HLD files are shared. The four authoring contracts retain their
dependency order.

#### Sprint S90, Fields, pagination, and navigation

**Goal**: provide a complete field construction surface and deterministic
field results across stories, then compose captions, cross-references, indexes,
bounded native citations and bibliography results.

| F-ID | Title | Size |
|------|-------|------|
| F-278 | General simple and complex field builder | L |
| F-279 | Pagination field materialization across stories | L |
| F-280 | Captions, sequences, and complete cross-references | M |
| F-281 | Indexes and tables of figures and authorities | L |
| F-282 | Citations and bibliography authoring | L |
| F-X179 | Correct multi-paragraph comment threads from PR 271 | S |
| F-X180 | Correct cell nil and none border precedence | S |
| F-X181 | Ignore page and column breaks inside table cells | S |
| F-X182 | Honor direct table alignment | M |
| F-X183 | Correct table margins and legacy positioning | M |
| F-X184 | Safe comment ownership during content removal | L |
| F-X185 | Expose comment anchor text and story location | M |
| F-X186 | Move comment anchors without losing threads | M |
| F-X187 | Scoped paragraph and cell text replacement | M |
| F-X188 | Preserve comment ownership when replacing or removing whole stories | L |
| F-X190 | Preserve cached text in multi-run complex field story snapshots | M |
| F-X191 | Ignore namespace declarations in numbering reader completeness | S |
| F-X189 | Prepare Word 0.16.0 and PowerPoint 0.14.0 families | L |

F-278 is the field substrate. F-279 uses the S89 note policy, and F-280 uses
S89 range markers. F-281 follows the field, pagination, and caption work.
F-282 follows F-278 and delivers its revised measured-subset contract.
F-283 carries the composed numbering and navigation result to S91. The S90
gate checks delivered field caches and page targets against pinned Word controls.
F-X178 is carried out of the S90 execution at the user's instruction to leave
Issue 264 alone. Its backlog row remains pending for separate work. F-282
covers the measured native source-authoring and formatter subset. F-X192 owns the
remaining full catalogue work in S91. F-283 is carried to S91.

F-X179 integrates PR 271 only after every Issue 270 criterion passes. It has
no dependency and preserves Issue 264 and F-X178 for separate work.

The user added Issues 272 and 273 with PRs 274 and 275 during S90.
F-X180 and F-X181 validate each full issue before accepting its contribution.
Their source and regression files overlap F-282 and F-283. Pause F-282 at a
saved checkpoint, run these independent fixes in separate waves, then resume
the bounded F-282 contract. F-283 is carried to S91. Source and Cargo ownership never overlap. Reconcile both
approved contracts at integration and check table geometry and bibliography
output. Issue 264 remains excluded.


Newly opened Issues 277 and 276 add F-X182 and F-X183. Run direct alignment
before legacy positioning, with F-X182 completed at a scoped dependency
checkpoint before F-X183 starts. Keep F-282 paused through these exclusive
source and hash-baseline waves, then resume its bounded S90 contract. F-283 is carried to S91. Each rendering
delta is separately declared and reviewed. Issue 264 remains excluded.
PR 279 supplies F-X182. PR 280 is stacked on it and supplies F-X183, including new Issue 278 for unset side margins. Validate both complete issue sets before contributor disposition.


Issue [281](https://github.com/tensorbee/rdocx/issues/281), raised by `hadim`,
adds roadmap feedback to this sprint intake. Record its unified CLI/Python
workflow, template filling, preservation-safe edits and rendering needs in the
existing F-184 decision at S96. The [maintainer response](https://github.com/tensorbee/rdocx/issues/281#issuecomment-6058792996)
answers scheduling and staged-delivery questions and requests concrete workflows
and preservation needs. Keep the discussion open for the F-184 decision. S90 delivers the documented
assessment, while spreadsheet implementation retains its affirmative decision
barrier.

Issues [282](https://github.com/tensorbee/rdocx/issues/282), [283](https://github.com/tensorbee/rdocx/issues/283), [284](https://github.com/tensorbee/rdocx/issues/284) and [285](https://github.com/tensorbee/rdocx/issues/285), reported by `hadim`, add F-X184 through F-X187. The intake was read against canonical 0a775842a8fd12f088bf4f2b3d0a049ddc3e976b. All four were opened on 2026-10-08 at 13:35 UTC and have no matching contribution PR at intake. Subsequent [PR 286](https://github.com/tensorbee/rdocx/pull/286), by `hadim`, targets Issue 285 at head `08361f6af99110cbae71bf9f0498a12feadc01db` and is included in F-X187 contribution assessment. Its patch must satisfy the approved scope and integrated verification before acceptance or closure. Their approved plans retain the complete issue criteria. Issue numbers 282 and 283 are distinct from existing feature IDs F-282 and F-283.

Pause F-282 only at an explicit saved external checkpoint. Run F-X184, F-X185, F-X186 and F-X187 in exclusive waves 11, 12, 13 and 14, respectively. F-X184 uses completed F-X179 and F-271. Complete each formal dependency before starting its consumer. F-X187 has no formal dependency but follows F-X186 because document, comment, binding and existing test entrypoints overlap. These waves exclusively own shared source, Cargo execution and HLD edits. Resume the revised measured-subset F-282 contract afterward. Remaining bibliography work is F-X192 in S91, and F-283 is carried to S91. Draft batch planning does not waive implementation barriers or the final integrated full gate. Issue 264 and F-X178 remain excluded. GitHub closure waits for verified sprint close and complete issue acceptance.

Subsequent [PR 287](https://github.com/tensorbee/rdocx/pull/287), by Hadrien
Mary (`hadim`), targets Issues 282, 283 and 284 at immutable head
`e4b216934eb3abc58ea1a42d72a902f90aa8e120`, based on
`20888b7a2c636e322ad23dc611f494beaac1c09b`. It is included in F-X184 through
F-X186 contribution assessment. Existing F-X184 acceptance retains atomic
partial-cut and fragment-detach refusal. Assess reusable anchor projection,
typed binding and movement changes against the full F-X185 and F-X186 plans,
without replacing the completed ownership protections or accepting the PR's
stated note and header removal gaps. The stacked F-X179 contribution is
already integrated. Read the exact contribution and preserve contributor
attribution, with PR reconciliation only after complete integrated acceptance
at verified sprint close. No additional F-ID or scope reduction is introduced.

Issues [288](https://github.com/tensorbee/rdocx/issues/288) and
[289](https://github.com/tensorbee/rdocx/issues/289), reported by `hadim` on
2026-10-08 at 16:18 UTC, are included in S90 acceptance assessment. Issue288
covers note removal, all header and footer replacement variants, shared story
references and building-block removal with complete comment thread and
companion preservation or atomic refusal. Compare every criterion with the
completed F-X184 implementation and add any missing implementation and tests
through the feature lifecycle before acceptance. Issue289 requires nested
block-control paragraph snapshots to carry accepted text and XML, including
comment anchor endpoints and other checked two-segment path consumers. Assess
it under F-X185's existing checked paragraph snapshot contract. Nested
paragraph enumeration is optional in that issue and does not change the
ordinary story inventory contract. Neither issue is closed from similarity to
an existing fix. Exact integrated evidence and full issue acceptance remain
required. The seven observed PR heads remain unchanged. Issue264 remains
excluded.

Issue292 is included in F-X188 with documented comment omission from
building-block transfers and physical glossary/main owner isolation.
The user corrected the exclusion to Issue281, which remains open for F-184.
Issue291 is included through F-X190, whose planned, verified multi-run field
snapshot correction runs in exclusive wave16 after F-X188 and before F-282 resumes. Issue264 remains
excluded. Assess updated stacked contributions selectively against complete
issue acceptance before merging or closing any record.

F-X188 owns exclusive wave15 after F-X187 and before F-282
resumes under its revised measured-subset contract. Its approved Issue288 contract completes the remaining replacement,
glossary and shared-reference ownership work. Issue289 is part of F-X185.

The user approved F-X189 for new Word0.16.0 and shared/PowerPoint0.14.0
release preparation. Existing unified versions are published at S88 and their
tags remain immutable. Run exclusive wave18 only after all included S90
implementation dependencies complete, including the revised F-282 measured
subset and F-X191. Prepare exact carriers, reviewed family notes and authenticated
contribution inventories over each complete previous-tag range, including
S89. Current local package/binding evidence precedes final integrated full
verification and review. The final reviewed and pushed sprint SHA must pass
the hosted build-only rehearsal before close. Publication follows close
through separate final exact-main-SHA approvals for each family, with shared
packages published before their new Word consumers. This is a user-approved
release preparation boundary, not completion of M24 or a1.0 decision.
Issue264 remains excluded and Issue281 remains open for F-184.

S90 contribution intake is frozen by the user at PR293. Do not search for
or add later issues or pull requests to this sprint. Finish the existing
measured bibliography, numbering reader and release preparation scope.
Remaining catalogue work and unfinished navigation move to S91.
Issues264 and281 remain open.

PR [293](https://github.com/tensorbee/rdocx/pull/293), contributed by Pedro
Assumpcao (`pedroassumpcao`) at head
`fd112a7ac3333709f62746b7065c5cc1b4be0eed`, adds F-X191. The user approved
its workflow records. Exclusive wave17 corrects the namespace-only numbering
reader flag while preserving declarations, foreign attributes, opaque children
and the separate authoring completeness contract. It has no formal dependency,
but shared source and Cargo ownership remain exclusive. Complete F-X191 before
F-X189 in wave18. Assess only the reader fix and focused tests, preserving
current archive policy rather than adopting contributor platform overrides.
Issue264 remains excluded and Issue281 stays open. PR disposition waits for
complete integrated acceptance at verified sprint close.

The user moved remaining bibliography work to F-X192 in S91 and explicitly
carried F-283 to S91 to publish completed S90 work. F-282 must satisfy its
revised measured-subset design, actual admission and atomic refusal contracts,
independent review and scoped gates. Its current 210-of-223 dense APA checkpoint
is not full catalogue parity. F-X189 depends on completed revised F-282 and
F-X191, not the carried S91 stories. Final integrated verification, sprint
review, package checks and exact-SHA publication approval remain required.

#### Sprint S91, Contribution repairs

**Local checkpoint, 2026-10-15**: all29 top-level stories and four delivery
children are complete. A release-workflow amendment qualifies the exact37
Word and41 PowerPoint bare-wheel priority checks after the prior local review. The final local full gate passes, including778 Word
and854 PowerPoint Python cases,104 font cases within those suites, the
separate native-eight check,49 matching hashes,22 verified archives, README
and supply-chain checks. The unchanged100-script report qualifies95 correct
files, all42 hard gates,51 native clauses and24 documentation examples. Its
five documented non-successes remain in the denominator. Historical missing
oracle cases are uncredited. Integrated sprint reviewpass3 is clean. Hosted build
rehearsal, main merge, publication of both families and humanized issue/PR
reconciliation retain their separate gates. Discussion264/281 stay open.
The S92 carryovers remain deferred. AS_BUILT records the actual evidence.

**Goal**: finish completed stories and all included issue/PR repairs. The user
parks the separate S90 bibliography and navigation carryovers at S92. Intake
is closed at forty issues and thirty PRs, excluding only the requirements
discussions264 and281.

| F-ID | Title | Size |
|------|-------|------|
| F-X193 | Preserve semantic mark highlights through nested HTML runs | S |
| F-X194 | Complete Python table text and indentation APIs | M |
| F-X195 | Honor caller fonts and variable styles across render paths | L |
| F-X196 | Validate visible paragraph content without false empty warnings | S |
| F-X197 | Unify Python colour inputs and font colour formats | L |
| F-X198 | Preserve alternate text-box copies during scoped replacement | M |
| F-X199 | Model namespace-qualified empty table cell margins | S |
| F-X200 | Complete Python Word formatting and list parity | L |
| F-X201 | Complete Python section stories and page authoring | L |
| F-X202 | Complete Word tables pictures and internal links | L |
| F-X203 | Complete Python Word document operations and validation | L |
| F-X204 | Complete shared CLI one-shot editing commands | L |
| F-X205 | Complete Python PowerPoint text fill and export parity | L |
| F-X206 | Complete PowerPoint shape authoring and live handles | L |
| F-X207 | Persist PowerPoint autofit and implement fit_text | L |
| F-X208 | Author PowerPoint footer fields themes and transitions | L |
| F-X209 | Edit PowerPoint masters layouts and themes | L |
| F-X210 | Extend metric-compatible fonts and bullet fallbacks | L |
| F-X211 | Complete Python script compatibility and safe XML access | L |
| F-X212 | Align native accessibility with visible paragraph content | M |
| F-X213 | Preserve TOC entry boundary paragraphs during rebuild | M |
| F-X214 | Bound recursive PowerPoint parsing and traversal | L |
| F-X215 | Create charts in Python and replace existing chart data | L |
| F-X216 | Import complete slides and merge decks with source formatting | L |
| F-X217 | Import HTML and Markdown through public Word surfaces | L |
| F-X218 | Export presentations as semantic HTML and Markdown | L |
| F-X220 | Embed fonts through both native and public package families | L |
| F-X221 | Prepare Word 0.16.1 and PowerPoint 0.14.1 patch releases | L |
| F-X222 | Restore linear Word story range inventory and empty owners | M |

**Current delivery checkpoint**: twenty-six of twenty-nine active stories are
completed and integrated, F-X193 through F-X210, F-X212, F-X213, F-X214, F-X215, F-X217, F-X218, F-X220 and F-X222. Remaining issue and PR repairs follow
completed dependencies and exclusive source boundaries. CURRENT_SPRINT.md
records active owners and execution states. F-X192 and F-283 remain at S92 with their contracts,
dirty worker work, reviews and required evidence retained. Standalone authoring
work remains in later sprints. No design question remains outstanding.

F-X209 completes its reviewed scoped integration in7349282b. Part-owned master,
layout and theme edits pass native PowerPoint and LibreOffice acceptance, current
installed logo and typing checks, verified archives and finite measurement policy.
Independent integration review30 preserves completed Word contracts. The composed
six-crate native and binding check and all49 approved hashes pass.

F-X203 completes its reviewed scoped integration in4d992a37. Native and binding
all-target/all-feature checks, exact chart paint, all49 hashes, source-qualified
archive metadata and all21 Rust README cases pass. Original failures and their
qualified successors remain evidence. Its document operations and native
Markdown, fresh Word-compatible settings, story tabs/breaks and checkbox repairs
address the frozen issue and PR slices without duplicating contributor work.
Final full sprint verification, review and GitHub closure remain due.

F-X200 and F-X207 complete their scoped integration checkpoint with seven
actual composed checks, including both exact native regressions and all49
approved hashes. F-X200 owns the two reviewed Word XML namespace changes.
All other completed providers retain their qualified acceptance. The durable
resolution evidence is in AS_BUILT.md and the current status is in
CURRENT_SPRINT.md and BACKLOG.md. Final integrated full verification,
sprint review and verified sprint close remain outstanding. No GitHub issue
or PR closure is claimed by scoped completion.

**Frozen resolution scope, intake closed by the user**: all included issues and
PRs remain in S91, except [Issue264](https://github.com/tensorbee/rdocx/issues/264)
and [Issue281](https://github.com/tensorbee/rdocx/issues/281). Keep those two
requirements discussions open. The latest inventory contains forty open issues
and twenty-eight PRs, thirty-eight included issues after exclusions. The previous
nineteen PRs and twenty-six included issues retain their existing owners below.
Issue339 andPR342 extend F-X203 all-story text acceptance. Issue340 andPR341
are assigned to approved F-X213, a bounded TOC rebuild repair separate from
the parked full F-283 contract. Immutable source and review assessment is
complete. Correct identified preservation gaps before adoption and prove
acceptance locally.
Reconcile only frozen records at implementation, integration and final close
checkpoints. Intake stays closed. Matching issues and PRs share one acceptance
owner and implementation.
The 2026-10-11 09:50:39 UTC refresh found no new records. PR342 advanced
to `9fd0b8da374b8c641e7ab89c0bd3d76bd2f4519a`. Its updated contribution
remains assigned to F-X203. Its immutable update has been assessed against
the existing approved contract, with remaining acceptance controls retained.
All thirty-eight included issues and twenty-eight PRs have the acceptance
owners in this resolution map. The 2026-10-12 02:53:59 UTC refresh found no
new or changed records.
The 2026-10-11 18:43 UTC refresh found no additions or changed PR heads.

| Included issues | Included PRs | S91 acceptance owners | Resolution work |
|---|---|---|---|
| No linked issue | [294](https://github.com/tensorbee/rdocx/pull/294) | F-X193 | Semantic HTML highlights, completed. |
| [295](https://github.com/tensorbee/rdocx/issues/295), [300](https://github.com/tensorbee/rdocx/issues/300) | [317](https://github.com/tensorbee/rdocx/pull/317) | F-X194 | Cell text frames and signed removable table indentation, completed with scoped acceptance. |
| [296](https://github.com/tensorbee/rdocx/issues/296), [297](https://github.com/tensorbee/rdocx/issues/297) | [315](https://github.com/tensorbee/rdocx/pull/315) | F-X195 | Caller fonts and variable styles across layout and output, completed with scoped acceptance. |
| [298](https://github.com/tensorbee/rdocx/issues/298) | [317](https://github.com/tensorbee/rdocx/pull/317) | F-X196, F-X212 | CLI validation and native accessibility complete with scoped acceptance. |
| [299](https://github.com/tensorbee/rdocx/issues/299) | [317](https://github.com/tensorbee/rdocx/pull/317) | F-X197 | Shared Python colour inputs and font colour formats, completed with scoped acceptance. |
| [301](https://github.com/tensorbee/rdocx/issues/301) | [317](https://github.com/tensorbee/rdocx/pull/317) | F-X198, F-X202 | Atomic refusal and synchronized admitted alternate-copy editing completed with scoped acceptance. |
| No linked issue | [302](https://github.com/tensorbee/rdocx/pull/302) | F-X199 | Qualified empty cell margins, completed. |
| [303](https://github.com/tensorbee/rdocx/issues/303), [321](https://github.com/tensorbee/rdocx/issues/321) | [320](https://github.com/tensorbee/rdocx/pull/320), [336](https://github.com/tensorbee/rdocx/pull/336) | F-X200, consumes F-X197 | Word formatting, lists and native shared numbering counters, completed with scoped acceptance. |
| [304](https://github.com/tensorbee/rdocx/issues/304) | [328](https://github.com/tensorbee/rdocx/pull/328), [343](https://github.com/tensorbee/rdocx/pull/343) | F-X201 | Section stories and page authoring, including checked related-story picture media ownership, completed with scoped acceptance. |
| [305](https://github.com/tensorbee/rdocx/issues/305) | [322](https://github.com/tensorbee/rdocx/pull/322), [343](https://github.com/tensorbee/rdocx/pull/343) | F-X202, consumes F-X194 and F-X198 | Word tables, pictures, internal links and alternate-copy synchronization, with atomic related-story Run picture insertion, completed with scoped acceptance. |
| [306](https://github.com/tensorbee/rdocx/issues/306), [332](https://github.com/tensorbee/rdocx/issues/332), [334](https://github.com/tensorbee/rdocx/issues/334), [339](https://github.com/tensorbee/rdocx/issues/339) | [325](https://github.com/tensorbee/rdocx/pull/325), [342](https://github.com/tensorbee/rdocx/pull/342), [354](https://github.com/tensorbee/rdocx/pull/354) | F-X203 | Document operations, validation, Markdown emphasis and list/table boundaries, fresh Word-compatible settings, qualified all-story tabs/breaks and atomic visible control replacement, completed with scoped acceptance. |
| [307](https://github.com/tensorbee/rdocx/issues/307) | [330](https://github.com/tensorbee/rdocx/pull/330) | F-X204 | Shared CLI editing commands after their API providers. |
| [308](https://github.com/tensorbee/rdocx/issues/308) | [318](https://github.com/tensorbee/rdocx/pull/318) | F-X205, consumes F-X194 and F-X197 | PowerPoint text, fill, properties, media and export parity, completed with scoped acceptance. |
| [309](https://github.com/tensorbee/rdocx/issues/309), [323](https://github.com/tensorbee/rdocx/issues/323) | [326](https://github.com/tensorbee/rdocx/pull/326), [335](https://github.com/tensorbee/rdocx/pull/335), [317](https://github.com/tensorbee/rdocx/pull/317) | F-X206 | Shape authoring, live handles, group transforms and diagnosed member-local render recovery, completed with scoped acceptance. |
| [310](https://github.com/tensorbee/rdocx/issues/310) | [329](https://github.com/tensorbee/rdocx/pull/329) | F-X207 | Autofit and fit_text with shared font measurement, completed with scoped acceptance. |
| [311](https://github.com/tensorbee/rdocx/issues/311) | [324](https://github.com/tensorbee/rdocx/pull/324) | F-X208 | Footer fields, themes and transitions, completed with scoped acceptance. |
| [312](https://github.com/tensorbee/rdocx/issues/312) | [327](https://github.com/tensorbee/rdocx/pull/327) | F-X209 | Masters, layouts and themes, completed with scoped acceptance. |
| [313](https://github.com/tensorbee/rdocx/issues/313) | [319](https://github.com/tensorbee/rdocx/pull/319) | F-X210, consumes F-X195 | Metric-compatible fonts and bullet fallbacks, including the approved 3968-byte checkbox subset, completed with scoped acceptance. |
| [314](https://github.com/tensorbee/rdocx/issues/314) | Contributions above | All applicable S91 owners | Umbrella checklist, each provider credited once. |
| [316](https://github.com/tensorbee/rdocx/issues/316), [331](https://github.com/tensorbee/rdocx/issues/331), [337](https://github.com/tensorbee/rdocx/issues/337) | [333](https://github.com/tensorbee/rdocx/pull/333), [336](https://github.com/tensorbee/rdocx/pull/336), [338](https://github.com/tensorbee/rdocx/pull/338) | F-X211, consumes completed providers | Frozen script corpus, safe XML, residual handles and bounded native control/text-box parsing. |
| [340](https://github.com/tensorbee/rdocx/issues/340) | [341](https://github.com/tensorbee/rdocx/pull/341) | F-X213 | Entry-owned and single-paragraph TOC rebuild, retained opaque properties and namespace closure, completed with scoped acceptance. |
| [344](https://github.com/tensorbee/rdocx/issues/344) | [355](https://github.com/tensorbee/rdocx/pull/355) | F-X214 | Bounded PowerPoint parsing and traversal, completed with scoped acceptance. |
| [345](https://github.com/tensorbee/rdocx/issues/345), [346](https://github.com/tensorbee/rdocx/issues/346) | [356](https://github.com/tensorbee/rdocx/pull/356) | F-X215 | Python chart creation and preservation-aware data replacement, approved pending implementation. |
| [347](https://github.com/tensorbee/rdocx/issues/347), [348](https://github.com/tensorbee/rdocx/issues/348), [349](https://github.com/tensorbee/rdocx/issues/349) | [357](https://github.com/tensorbee/rdocx/pull/357) | F-X216 | Complete slide graph import, source formatting and deck merge, completed and integrated locally, scoped acceptance qualified. Remote resolution follows final verification and publication. |
| [350](https://github.com/tensorbee/rdocx/issues/350), [351](https://github.com/tensorbee/rdocx/issues/351) | [359](https://github.com/tensorbee/rdocx/pull/359) | F-X217 | Public Word HTML and CommonMark import, completed with scoped installed, CLI and native Word open/save/reopen/render acceptance. Remote closure follows verified sprint close. |
| [352](https://github.com/tensorbee/rdocx/issues/352), [353](https://github.com/tensorbee/rdocx/issues/353) | [358](https://github.com/tensorbee/rdocx/pull/358) | F-X218 | Semantic presentation HTML and Markdown export, approved pending completed F-X205 and F-X206. |

PR343 entered at `c6a78f683f327be8d7674af509874f74945080f3`, based directly
on main `733ce6f7b2b816c5760692abf5c99d68b29ae897`. Its full immutable
contribution is assessed at /private/tmp/S91-PR343-intake/assessment.md.
Reuse checked native story-media allocation and lookup under F-X201, then
compose actual Run picture insertion under F-X202. Its tests cover embedded
relationships, not drawing insertion or rendering. Preserve current owner
identity and validate stale, foreign and unsupported runs before one staged
allocation and insertion, with no orphan media after refusal. Required note,
cell, text-box, namespace, saved drawing and native render controls remain.
Do not transplant contributor archive measurements or count shared work twice.

PR317 spans several owners and is reconciled once across their accepted
slices. PR326 and PR335 share group-transform changes, adopted once in F-X206.
New PR336, head `c6ab986008782e8a376a9cdd0bda2afd4847cf47`, closes Issues331
and321. Parsing belongs to F-X211 and numbering to F-X200. Its additional
inline-wrapper and cross-control nesting policy has been assessed. Preserve
the approved qualified256 control limit and native per-level numbering
contract. Do not adopt the additional64-wrapper refusal or incomplete
numbering restart shortcuts. Reuse only conforming changes and verify locally. It creates
no duplicate F-ID. Contribution claims do not replace local test evidence.

PR336 advanced to `935ac165afcd29d34454568ba3b2287a4bac52dc` at the
2026-10-10 07:27:15 UTC refresh. Incremental source assessment is complete.
Its proposed64-control and512-element revision limits do not supersede the
approved contract by intake. Relative REF changes must be reconciled with
F-X200 and F-283 once, retaining numbering and navigation ownership.

F-X192 and F-283 move to S92 at the user's instruction. Their full contracts
and existing work are preserved. F-X200 remains in S91 and provides the
authoritative numbering prerequisite for later F-283 work.

**Approved execution order**: wave 1 is complete. Wave 2 is F-X194, F-X212 and the disjoint core-font phase of F-X195.
CLI and binding wiring waits for existing file owners. Wave 3 is F-X197,
wave 4 completes F-X195 wiring, wave 5 F-X210, wave 6 F-X206,
wave 7 F-X200 and F-X207, wave 8 F-X201 and F-X208, wave 9 F-X203,
wave 10 F-X205, wave 11 F-X202 and F-X209, wave 12 is deferred with F-283, wave 13 F-X204,
and wave 14 F-X211. Independent wave 15 is F-X213 after completed F-279.
Its field.rs, regression_test.rs and test_core.py leases serialize against
F-X200 and F-X206 edits and integration. It may prepare in a separate worker
while F-X195 owns Cargo and the shared installed environment. F-X192's bibliography evidence barrier applies in S92. Parallel members require completed dependencies and disjoint file
leases. Cargo and hash-baseline changes remain serialized.

The following paragraphs retain historical intake checkpoints. This current
coverage table and execution order supersede their older inventory counts,
PR draft descriptions and preliminary sequencing. Keep delivery status current
within the current rolling inventory. Close an issue only after its full acceptance is proved
at verified sprint close. Reconcile unmerged contributor PRs explicitly as
integrated or superseded against actual delivery evidence.

The 2026-10-09 intake includes [PR294](https://github.com/tensorbee/rdocx/pull/294)
and Issues [295](https://github.com/tensorbee/rdocx/issues/295),
[296](https://github.com/tensorbee/rdocx/issues/296),
[297](https://github.com/tensorbee/rdocx/issues/297),
[298](https://github.com/tensorbee/rdocx/issues/298),
[299](https://github.com/tensorbee/rdocx/issues/299) and
[300](https://github.com/tensorbee/rdocx/issues/300). These are all open records
at intake except the explicitly excluded Issues264 and281. PR294, by
`pedroassumpcao`, is assessed at head
`63cd119dcb8fa9c367bf93141f79780db7f64886`. Issues295 through300 are reported
by `hadim`. Contribution acceptance requires the full story contract and
integrated evidence, not adoption of a PR's test claims.

F-X194 groups the two Python table API gaps, Issues295 and300, while retaining
separate save/reopen and render controls for each family. F-X195 groups the
caller-font and variable-instance reports, Issues296 and297, because all text
measurement and output paths must agree on the chosen face and instance.
The grouping does not reduce either issue's acceptance criteria.

Use the approved execution order above and the canonical sprint run state.
Complete formal prerequisites before starting their consumers. Shared source,
bindings, tests and the hash baseline have one owner at a time. New stories require approved designs, scoped gates and clean
microscope reviews. The original nine-story estimate was 23 to 29 developer-days. The expanded
twenty-two-story scope requires a new estimate after detailed design, not an
unchanged two-week duration. The inserted boundary moves the previous S91
through S104 to S92 through S105. S90 delivered history remains intact.

Issues264 and281 are excluded from S91. F-X178 remains pending at S92 and
requires a separate scope decision. Issue281 remains open for F-184 at S96.
No spreadsheet implementation or publication is authorized by this intake.
S90's intake freeze remains its completed delivery boundary.

**Current contribution intake**: rolling S91 intake is open at the user's
latest instruction. The 2026-10-11 10:56:28 UTC refresh contains twenty-two
open PRs and twenty-eight included open issues, excluding discussion Issues264
and281. New PR343 joins F-X201 and F-X202 without a duplicate story.
All included records have acceptance owners in the current resolution map.
Fold later contributions into their existing owner where applicable, preserving
complete acceptance and avoiding duplicate work. The earlier2026-10-10 freeze
is superseded. Bibliography F-X192 and navigation F-283 remain parked in S92.

Reuse each already assessed immutable contribution once, across its mapped
owners. Retain contributor attribution, linked issues, exact accepted heads
and locally verified evidence. Complete uncovered acceptance under the
approved designs. An integrated source change still requires its affected
feature gate and independent review. Final verification and sprint review
must cover the actual completed integrated result before close.

**Sprint gate**: complete all twenty-one active contribution-repair stories
and their included issue and PR acceptance. This includes HTML mark inheritance,
editable Python table text and signed indentation, matching caller-font layout
and renders including variable bold/italic, content-aware empty-paragraph
diagnostics, consistent Python colours and the bounded TOC entry repair.
Bibliography F-X192 and navigation F-283 retain their full contracts in S92.
Verify installed bindings, round trips, native rendering controls and declared
hash deltas. Run the full integrated gate and sprint review once over the
completed result. Reconcile GitHub records only at verified sprint close.

Issue [301](https://github.com/tensorbee/rdocx/issues/301), by `hadim`, entered
at the opening refresh and adds F-X198 for alternate text-box ownership. It
follows completed F-X187 and runs in its own exclusive wave. This expands the
initial seven-story intake to eight without weakening acceptance or carrying
new scope away from this sprint.

[PR302](https://github.com/tensorbee/rdocx/pull/302), by `pedroassumpcao`, entered
the rolling refresh on 2026-10-09 at head
`30e4a93df7549990c5838da9c95866d19f1247b0`, based on
`733ce6f7b2b816c5760692abf5c99d68b29ae897`. It is a draft with no linked issue
or discussion at assessment. F-X199 adopts its narrow qualified empty-cell
margin correction and tests, then verifies preservation and public save/reopen
locally. This is independent scope, not a duplicate of the table authoring
or validation issues.

Wave 1 permits separate source worktrees for F-X193, F-X198 and F-X199.
Their source and test file sets are disjoint when F-X198 retains its approved
regression_test.rs boundary. Shared HLD completion edits, delivery records and
integration are serialized. Cargo commands queue against one shared target.
F-X194 follows wave 1 because its native evidence uses integration_test.rs and
regression_test.rs. Later source conflicts remain exclusive, and the exclusive
hash baseline retains its designated owner. Independent draft and review work
can continue while implementation and Cargo resources are occupied.

**Expanded intake, 2026-10-09**: Issues303 through313 are included as
F-X200 throughF-X210, respectively. Issue314 is their umbrella checklist and
adds no duplicate implementation story. The new open PR inventory still
contains only PR294 and PR302 at their already assessed heads. Preserve the
existing four active workers and their evidence while drafting the added scope.
The issue-specific HLD14 contracts retain all requested acceptance. No work
is silently deferred from S91. Designs must map shared criteria to the existing
F-ID first, with later contributor PRs attached to that owner.

F-X200 reuses F-X197 colours. F-X202 reuses F-X194 indentation and F-X198
physical alternate safety, then adds the synchronized editing demanded by
Issue301. The latest Issue305 body delegates Issue300 and Issue301 to those
existing owners. F-X203 reuses F-X196 and F-X201 validation. F-X205 reuses F-X194 and
F-X197. F-X207 consumes caller-font measurement from F-X195. F-X210 contains
only the new metric-compatible face, diagnostics and bullet mapping criteria
from Issue313. Its caller-font and variable-font acceptance remains F-X195.
F-X209 follows F-X208 as explicitly required by Issues312 and311. F-X204 CLI
commands follow their Python-visible API providers, and do not duplicate the
footer/inspection CLI owned by F-X208. Record detailed file conflicts and
waves before claiming any added worker. All new stories are provisionally L,
with measured estimates pending design. The optional font pack needs a design
decision before adding any new crate or distribution. No publication is part
of this sprint.


PR [315](https://github.com/tensorbee/rdocx/pull/315) entered the next rolling
refresh at immutable head `af388f255d3b178aaa0aef03313034b815c71eb2`. It closes
Issues296 and297 and belongs to existing F-X195, with Issue313 crediting that
same implementation. No new story is created. Read the complete changed
files, comments and reviews before claiming font work. Reuse accepted loader,
render-path and instance changes, then implement uncovered approved criteria.
Its stated omissions include synthetic italic, bundled variable instances and
caller-font handout/PDF-A paths. Its fallback on subset failure, directory
precedence and synthesis geometry must be reconciled against the approved
plan. Contributor gate claims are not local evidence. Archive measurements
will be generated from actual integrated source.

Issue [316](https://github.com/tensorbee/rdocx/issues/316) adds F-X211 for
roughly100 first-draft Python scripts with per-area CI success metrics,
validated element XML read/replace across the requested owners, remaining
stable handles, operation-named invalidation and actionable-error audits.
Known font, link, section and selected handle repairs remain with their
existing F-IDs. Measure the integrated provider result once and avoid a
second implementation of those fixes.

PR [317](https://github.com/tensorbee/rdocx/pull/317), head
`3d056f46e97e9cc1caf2969b5096eeb6775a5fa0`, attaches to existing
F-X194/F-X196/F-X197/F-X198 for Issues295/298/299/300/301. Its tcPr anchor
belongs to F-X206 and paired success informs F-X202. Assess the full patch
against these contracts. Its added native accessibility behavior and
three-key feature_showcase delta belong to F-X212 under a separate labelled
reviewed scope.
No contributor test, archive or baseline record substitutes for local gates.

The 2026-10-10 13:26 UTC rolling refresh retains 19 open PRs and 26 included
issues, excluding #264 and #281. PR #294 advanced to
81494509dc27cbcdd3c6d31067089175cdc2313f with archive measurement changes
only. Authenticated HTML production matches completed F-X193 exactly.
Its later description update adds validation notes without changing source.
Retain the integrated tests and locally verified archive measurements, with
no reopened story or duplicate work. S91 remains six completed, three in
progress and thirteen approved. Final integrated verification and review
remain pending.

The 2026-10-10 15:05 UTC refresh retains 19 open PRs and 26 included
issues. No PR head advanced or new included record appeared. Issue264 has
only a timestamp update and remains excluded with Issue281. PR338 remains
assigned to F-X211 for Issue337, with stacked PR336 changes assigned once
across F-X211 and F-X200.

F-X197 installed scope passed all155 Word and497 PowerPoint cases, both
strict typing checks and both recursive stub checks. The full Word suite
then passed353 cases and failed one old malformed-colour error assertion.
Its two legacy border and shading expectations have been migrated to the
approved ValueError contract and independently reviewed with zero findings.
The corrected scoped rerun passed all652 cases and both families' typing
and stub checks. The complete Word and PowerPoint suites are running.
Remaining risk riders and formal completion are pending. F-X195 native
revision controls are prepared and independently reviewed, with native
execution and Python
consumer wiring still required. Full bibliography acceptance remains F-X192.
S91 remains six completed, three in progress and thirteen approved stories.
Final integrated verification, sprint review and closure remain pending.

#### Sprint S92, Bibliography and navigation carryovers, templates and forms

**Goal**: resume the parked F-X192 bibliography and F-283 navigation contracts
after S91. Preserve the existing template, control and form programme for
capacity review when S92 opens. This scheduling decision opens no new sprint
and changes no acceptance contract.

| F-ID | Title | Size |
|------|-------|------|
| F-X192 | Complete the remaining Word bibliography catalogue | L |
| F-X219 | Qualify full-range text watermark interoperability | M |
| F-X223 | Qualify native Word embedded multi-face interoperability | M |
| F-283 | Complete numbering-aware navigation fields | L |
| F-X178 | Clearable direct run formatting setters | S |
| F-284 | Stable container-wide template grammar | L |
| F-285 | Content control creation and lifecycle | L |
| F-286 | Rich, repeating, and typed content controls | L |
| F-287 | Custom XML stores and data binding authoring | L |
| F-288 | Legacy form field creation | M |
| F-289 | Modern Word form authoring | L |
| F-290 | Mail-merge package and data-source authoring | M |

Restore the missing historical native PowerPoint timeline movie/frame and
SmartArt PNG oracle inputs as later fixture infrastructure. Their four existing
missing-input checks remain explicitly uncredited in S91. Regeneration must
use actual independent producer output, with a separate infrastructure design
when resumed. It adds no repair scope to the frozen S91 issue/PR set and does
not waive applicable current source, schema, save/reopen or rendering gates.

F-X178 is the explicit S90 carry for the user-excluded Issue264. Its next
eligible planning target is S92. It remains pending and requires a separate
scope decision before implementation, preserving the exclusion in S90 and
the inserted S91.

F-X192 resumes its retained unfinished worker branch and native evidence.
F-283 consumes completed S91 F-X200 numbering without duplicate work.
These two stories are parked, not completed. Issues264 and281 remain open.

F-284 builds on S89 fragment transactions. F-285 establishes the control
lifecycle before F-286 and F-287 add typed values and bindings. F-288 uses the
S90 field builder. F-289 composes controls, bindings, ranges, and protection
into a complete form. F-290 may proceed alongside the form work after S90.
The sprint gate checks public-created templates and forms through save-reopen,
with explicit diagnostics for unavailable external mail-merge data.

#### Sprint S93, Collaboration authoring

**Goal**: create complete Word revisions, comments, permission ranges, and
comparison results, with caller-controlled identity and time instead of
ambient state.

| F-ID | Title | Size |
|------|-------|------|
| F-291 | Tracked insertion and deletion authoring | L |
| F-292 | Property revisions and move ranges | L |
| F-293 | Complete comments and modern comment metadata | L |
| F-294 | Permission ranges and protection integration | M |
| F-295 | Comparison output as complete revisions | L |
| F-296 | Collaboration identity and deterministic time policy | M |

F-291 establishes revision ownership. F-292 and F-293 then cover property,
move, and comment records across stories. F-294 uses the S92 form and
protection surface. F-295 composes complete comparison revisions, and F-296
closes identity and clock inputs. The sprint gate covers Word differential
results, accept and reject views, and byte-identical output in deterministic
mode.

#### Sprint S94, Drawings, diagrams, and embedded content

**Goal**: complete the visible and packaged Word object surface across every
valid insertion point without raw compatibility wrappers or executable payload
execution.

| F-ID | Title | Size |
|------|-------|------|
| F-297 | Complete Word drawing anchor and effect authoring | L |
| F-298 | Shapes, text boxes, groups, and connectors in Word | L |
| F-299 | AlternateContent, VML, and SVG compatibility authoring | L |
| F-300 | Charts at arbitrary Word insertion points | M |
| F-301 | SmartArt and diagram authoring in Word | L |
| F-302 | Embedded objects, icons, and alternative-format parts | M |
| F-303 | Drawing and embedded-content layout completion | L |

F-297 through F-302 build independent object families on the shared
relationship model. F-303 is their integrated deterministic layout and render
gate.

#### Sprint S95, Package extensibility and modern DOCX end gate

**Goal**: close package extension, accessibility, conformance, determinism,
resource, binding, documentation, and stability boundaries for modern DOCX
authoring.

| F-ID | Title | Size |
|------|-------|------|
| F-304 | Typed package extensibility facade | L |
| F-305 | Attached templates, web extensions, and task panes | L |
| F-306 | Executable compatibility attachment and signature rules | M |
| F-307 | Complete accessibility authoring and audit | L |
| F-308 | Fully modeled and losslessness diagnostics | L |
| F-309 | Strict, transitional, and repair-free conformance | L |
| F-310 | Determinism, resource limits, bindings, and stability gate | L |
| F-311 | Positioned paragraph frame placement | M |
| F-312 | Run visual effect render projection | L |

Package ownership precedes extension and executable-compatibility surfaces.
F-308 consumes every modeled family, F-309 validates its output, and F-310 is
the milestone-wide public API and operational gate. F-311 and F-312 close the
remaining positioned-paragraph and run-effect render checks. A 1.0 decision remains
separate from completing this sprint.

### M19, Advanced spreadsheets

#### Sprint S96, Spreadsheet decision, corpus and core model

**Goal**: decide whether a material Rust ecosystem gap still exists, then build
the ownership model only if that decision is affirmative.

| F-ID | Title | Size |
|------|-------|------|
| F-184 | Advanced spreadsheet go or no-go | S |
| F-204 | Spreadsheet corpus and compatibility matrix | M |
| F-185 | Workbook and worksheet model | L |

F-184 is a true go or no-go gate. It reassesses Calamine,
`rust_xlsxwriter`, `umya-spreadsheet`, `xls`, and any credible successor at S96,
then classifies each proposed feature as preserved, modeled and editable, or
executable. If the ecosystem provides the complete required lifecycle by then,
M19 is archived rather than implemented.

[Issue 281](https://github.com/tensorbee/rdocx/issues/281) supplies user demand
for a unified CLI/Python workflow, template filling, preservation-safe edits
and rendering. Include integration, installation and maintenance costs in
F-184, including reuse of a maintained engine behind a consistent facade.
S96 through S99 stage reading and loss-aware editing before calculation and
pivots. Rendering and distribution retain the S105 boundary. An earlier
distributable reader/editor requires a reviewed roadmap and release decision.

The reporter's [follow-up](https://github.com/tensorbee/rdocx/issues/281#issuecomment-6063901190)
defines a Google Sheets review workflow: read and change cell text, edit a
shared-formula column and preserve reviewer comments. Ordinary spreadsheet
reading is already covered in that workflow. The differentiators are one
CLI/Python shape and installation, Rust performance and safety, loss-aware
editing and later rendering without Excel. The requested first cut preserves
all unmodeled parts, including threaded comments and `commentsmeta*`, data
validation, conditional formatting and defined names. It reads values and
types, shared strings, formats and styles, merged ranges, formulas and cached
results. Edits include values, formulas and styles, rows and columns, sheets,
widths and heights, frozen panes and auto-filters, with atomic output. Shared
formula structure must survive. The decision must choose explicit stale-cache
invalidation and recalculate-on-load, or a separately bounded evaluator for
arithmetic, references, SUM, AVERAGE, MIN, MAX and IF. Chart authoring is not a
requested first-cut need. Rendering is a desired second step. These are inputs
to F-184, not S90 implementation or an approved earlier release boundary.

The [maintainer commitment](https://github.com/tensorbee/rdocx/issues/281#issuecomment-6064533994)
confirms preservation tests and the review workflow first, then reading and
styled edits with explicit stale-cache handling. A bounded evaluator and
rendering follow, with chart authoring deferred. Pivot refresh, Power Query
and scripting remain longer-term goals. F-184 must reconcile the scheduled
stories and any proposed reader/editor distribution boundary with that order
before approving implementation. S90 records this direction without claiming
that the conditional spreadsheet programme or an earlier release is approved.

#### Sprint S97, Styles, tables and structured references

**Goal**: model the indexed formatting and structured data semantics that
ordinary business workbooks rely on.

| F-ID | Title | Size |
|------|-------|------|
| F-186 | Shared strings, styles and number formats | L |
| F-189 | Formula parser | L |
| F-205 | Excel tables and structured references | L |

F-189 lands here because structured table references are part of the formula
grammar rather than a string convention. The sprint does not yet calculate
formulas.

#### Sprint S98, Advanced worksheet objects

**Goal**: cover the visible and interactive worksheet surface before the
streaming package boundary freezes it.

| F-ID | Title | Size |
|------|-------|------|
| F-206 | Advanced worksheet objects | L |

This includes comments, hyperlinks, rich text, drawings, grouping, panes,
sparklines, page breaks, and modern image cells. External content stays offline
unless an explicit bounded policy allows retrieval.

#### Sprint S99, Streaming read and write

**Goal**: prove that advanced workbooks remain bounded at the package boundary.

| F-ID | Title | Size |
|------|-------|------|
| F-187 | Reader | L |
| F-188 | Writer | L |

Both carry an asserted memory ceiling rather than a hoped-for one. A 100 MB
fixture is the gate, not a smoke test. Unsupported package parts and
relationships remain attached through unrelated typed edits.

#### Sprint S100, Calculation and sheet features

**Goal**: calculate ordinary and modern formulas, then expose the features that
depend on their results.

| F-ID | Title | Size |
|------|-------|------|
| F-190 | Calculation engine | L |
| F-191 | Charts in spreadsheets | M |
| F-192 | Conditional formatting and data validation | M |

F-190 is differential against the values Excel stored in the pinned corpus,
including dynamic arrays, spill ranges, and structured references. Unsupported
functions retain cached values with diagnostics. F-191 reuses `oxml-chart`
rather than creating a spreadsheet-only chart engine.

#### Sprint S101, Pivots and the Data Model boundary

**Goal**: move PivotTables from opaque preservation to typed local refresh and
define the boundary around proprietary analytical models.

| F-ID | Title | Size |
|------|-------|------|
| F-193 | Pivot cache and table model | L |
| F-207 | Pivot recalculation engine | L |
| F-208 | Slicers, pivot charts, and Data Model boundary | L |

Worksheet and table-backed pivots refresh locally and regenerate their cache
and visible cells. Slicers and pivot charts follow that refresh. OLAP, Power
Pivot, VertiPaq, and DAX state is preserved and inspectable but is not executed
under this milestone.

#### Sprint S102, Power Query language and package model

**Goal**: understand and evaluate M independently of external data access.

| F-ID | Title | Size |
|------|-------|------|
| F-209 | Power Query package and M language | L |

The package model preserves queries, connections, load destinations, and
refresh metadata. The language boundary covers the pure transformations needed
by the corpus before credentials, connectors, or network policy enter the
runtime.

#### Sprint S103, Power Query execution

**Goal**: refresh a bounded, useful connector set without weakening privacy or
offline determinism.

| F-ID | Title | Size |
|------|-------|------|
| F-210 | Power Query execution and connectors | L |

The first allowlist covers workbook tables, CSV, JSON, and HTTP. Credentials,
privacy levels, source combination, timeouts, byte limits, caching, and query
folding are explicit contracts. Proprietary and tenant-bound connectors remain
preserved with diagnostics.

#### Sprint S104, Office Scripts-compatible automation

**Goal**: automate the same workbook model through a versioned and sandboxed
TypeScript surface.

| F-ID | Title | Size |
|------|-------|------|
| F-211 | Office Scripts artifacts and ExcelScript surface | L |
| F-212 | Sandboxed Office Scripts runtime | L |

Scripts remain external artifacts associated with workbooks. The runtime does
not pretend to provide OneDrive, SharePoint, Power Automate, or Microsoft tenant
identity. It does provide bounded workbook, range, table, chart, pivot, and
query automation with atomic failure.

#### Sprint S105, Rendering, distribution and advanced end gate

**Goal**: close M19 as a headless advanced spreadsheet engine rather than a
file-format crate.

| F-ID | Title | Size |
|------|-------|------|
| F-194 | Sheet rendering | L |
| F-195 | rxlsx distribution | L |

The end gate runs one representative advanced workbook through read, edit,
formula and pivot recalculation, selected Power Query refresh, sandboxed script
automation, save, reopen, and deterministic PDF rendering. Every unavailable
execution surface remains preserved and diagnosed. Distribution follows the
shape M13 established only after that complete lifecycle passes.

## Future release boundaries

These are the planned points where a coherent crate family may publish. They
do not authorize a tag or publication by themselves. The sprint that reaches a
boundary adds an explicit release F-ID, selects the exact version from the
reviewed public API diff, and uses `/release` with its separate final approval
at the fully verified SHA.

| Boundary | Eligible publication | Why this point is coherent |
|---|---|---|
| End of S58, M20 | Stable `rdocx` family. Publish the incubating shared family first only if a shared crate version or stable dependency pin moved. | The Word corpus, shaping, pagination, and incremental layout gates have all settled, so the stable family can describe one measured fidelity boundary. |
| End of S64, M21 | Incubating `rpptx` family. Publish the stable family too only when the reviewed dependency diff requires new shared pins. | Collaboration, security, timing, media, SmartArt, ODP, handouts, HTML import, and PDF import form one complete presentation-depth boundary. |
| End of S69, M22 | Stable `rdocx` family at v0.13.0 through F-X078. Publish the incubating shared family first only if a shared crate version or stable dependency pin moved. | OfficeMath, fields, dynamic TOC, automation, comparison, embedded content, and modern package variants complete the planned Word-depth boundary. |
| End of S73, M23 | Stable `rdocx` family after the five-document public-API-only conformance gate passes. | This is the first boundary where the reference business documents can be authored from `Document::new()` without raw XML or a base template. |
| End of S95, M24 | Stable `rdocx` family after the modern Word authoring capability matrix is closed. Publish the incubating shared family first only if a shared crate version or stable dependency pin moved. | This boundary completes the planned modern DOCX authoring surface, lossless extensibility, accessibility, strict-package, determinism, binding, and stability gates. |
| End of S105, conditional M19 | The new `rxlsx` distribution family defined by F-195, plus only the existing families whose reviewed dependency pins moved. | F-195 is the first point where the conditional spreadsheet programme has a complete facade, CLI, WASM, Python, rendering, and advanced lifecycle gate. |

No intermediate sprint publishes merely because one subsystem compiles. A
security fix may still justify a separately planned patch release, but ordinary
feature work waits for the next boundary above. If F-184 archives M19, the S105
boundary disappears with the programme and no spreadsheet release namespace is
created.

## Cross-cutting

F-X001 through F-X004 are scheduled in S36 as the final cross-cutting v1
hardening wave. F-X007 and F-X008 handle the external Word contribution and
its stable-family release without rewriting the completed milestone history.
F-X013 through F-X016 carry the surviving half of the external PR 2 rendering
contribution, whose anchored-drawing placement was overtaken by the M7 anchor
work before it could land.
F-X031 carries the external branch-protection mutation deferred from F-X029 to
the Word fidelity boundary before the two depth milestones begin.

## Future ideas

These are discovery candidates, not scheduled work. They have no F-IDs, sizes,
dependency promises or delivery order. Each candidate needs a scope decision,
design plan and acceptance boundary before it can enter the backlog.

### Spreadsheet breadth

- Extend the advanced spreadsheet milestone with CSV, TSV, JSON and ODS output
  and broader structured-data import. Legacy binary XLS remains permanently
  excluded. XLSB requires its own future demand and architecture decision.
- Add worksheet copy and move operations, advanced paste options, named-range
  mutation and structured data import and export.
- Treat VBA and XLM as compatibility surfaces. Preserve, inventory, extract and
  optionally remove their projects and signatures without executing them.
- Consider DAX and VertiPaq execution only after the M19 Data Model preservation
  boundary has a licensed representative corpus and a separate security and
  performance decision.
- Add OLAP pivot refresh, proprietary Power Query connectors, custom connector
  SDK support, and broader query folding only against named consumers and
  reproducible service fixtures.

### Modern spreadsheet extensibility

- Preserve and inspect web-extension, task-pane and content-add-in parts,
  manifests, permissions and external resource locations. HTML, CSS and
  JavaScript assets should remain external web application content.
- Preserve namespaced custom-function formulas, cached results and add-in
  associations. The calculation engine should report unavailable custom
  functions instead of replacing their cached values.
- Preserve Python cell formulas, source, result previews and service metadata.
  Keep cloud execution outside the core library and expose the dependency as a
  diagnostic.
- Broaden the Office Scripts-compatible API only through versioned conformance
  fixtures. Microsoft-hosted script storage, tenant identity, Power Automate,
  and administrative policy remain integrations rather than local runtime
  claims.
- Add trusted custom-function hosts only after a separate capability and
  isolation design. Until then, retain namespaced formulas and cached results
  and report the unavailable function.

### Conversion and output

- Define a format-priority decision that weighs corpus prevalence, procurement
  needs, implementation risk and maintenance cost before adding another reader
  or writer.
- Consider XPS and PCL only when named users or a representative corpus justify
  them. Legacy binary Office formats remain excluded.
- Add a platform-neutral print-layout contract before considering operating
  system printer integration. PDF and image output should remain the portable
  default.
- Add per-page and per-shape rendering with explicit size, resolution, quality,
  compression, transparency and page-range controls across document families.

### Product surfaces

- Explore a self-hostable HTTP service for conversion, rendering, inspection
  and validation while keeping every operation available through local crates.
- Consider optional adapters for document summarisation, translation, grammar
  checking and presentation localisation. External model providers must remain
  outside the core document model.
- Add a browser viewer and lightweight editing surface only if the WASM APIs can
  remain the single implementation rather than creating a second document
  engine.

### Guardrails

Future breadth must preserve the properties that distinguish this workspace:

- Pure Rust implementations with no Office runtime or hidden conversion
  service.
- First-class native, CLI, Python and WASM entry points where the feature is
  technically available.
- Deterministic bundled-font rendering and declared output-baseline changes.
- Verbatim preservation of unmodelled XML and schema-correct child order.
- Shared OPC, DrawingML, chart, layout and rendering infrastructure rather than
  one implementation per document family.
- Offline operation by default, with network access isolated behind explicit
  adapters.

Cross-sprint semantic prerequisite: complete F-X200 in S91 before F-283 in S92. The authenticated
Word numbering controls require an explicitly reviewed revision to the old
independent-numId contract. Navigation consumes that final authoritative
result once, preserving source identity and proving its own field switches.

PR [318](https://github.com/tensorbee/rdocx/pull/318) entered rolling intake at
immutable head b1aa842bf531a65c70a49d4ef282033d26e64020, based on733ce6f7.
It closes Issue308 and is attached to existing F-X205. It contributes text,
fill, table, metadata, media and export binding surfaces plus the existing
16:9 template repair. Its stated App/custom-property omission remains part
of F-X205 acceptance. Shared colours retain F-X197 ownership, cell text
retains F-X194 ownership and structural-handle improvements retain F-X211
ownership. Assess all24 changed files and the actual native template before
reuse. Contributor tests, archive values and artifact hashes are claims to
verify locally, not completed sprint evidence. No additional F-ID or asset
inventory is created by this contribution.

## Rolling contribution reconciliation, 2026-10-09

PR [319](https://github.com/tensorbee/rdocx/pull/319), immutable head
`9a36bc966d1e12e4e9e2f38b777889bd687c5dd9`, closes Issue313 and attaches
to existing F-X210. Caller fonts and variable instances remain F-X195 and
PR315. Its proposed four modified static Gelasio faces and new Noto Sans
Symbols2 subset differ from the explicitly approved eight-file inventory.
Capture and assess their sources, license terms, metrics, glyph mappings,
italic selection and diagnostic lifecycle before selecting any contribution.
The existing asset approval does not authorize that different inventory.
Its twelve claimed rendering deltas are contribution evidence, not accepted
local baselines. Root retains exclusive baseline ownership.

PR [320](https://github.com/tensorbee/rdocx/pull/320), immutable head
`252acccbba9adab9264cfb109c4b1d720ed79d4a`, closes Issue303 and attaches
to existing F-X200. Its colours remain F-X197. Assess its tri-state setters,
tab ownership and enums, defaults preservation, list helpers and installed
binding behavior against the full approved contract before reuse. Its body
explicitly leaves native shared numbering counters unchanged.

Issue [321](https://github.com/tensorbee/rdocx/issues/321) therefore maps to
that same F-X200, which already owns the native shared-definition counter
change and its authenticated Word evidence. It adds no duplicate F-ID or
second implementation. Its explicit two-instance continuation, first-use
startOverride and sub-level restart criteria are covered by the same native
state contract and require local regressions and explained sample changes.
F-283 retains its F-X200 completion prerequisite. Do not close Issue303 or
Issue321 based only on the contributor's narrower binding patch.

The refreshed intake contains seven open PRs and23 open issues. Issues264
and281 remain excluded. Contributor test, measurement and baseline claims
never substitute for local verification.

## Next rolling refresh, 2026-10-09

PR [322](https://github.com/tensorbee/rdocx/pull/322), immutable head
`6b5e2e352047357299d1f366f807f4661037430e`, closes Issue305 and attaches
to existing F-X202. Its table grid edits, pictures, links and handle policy
are contribution input for the full existing contract. Its explicit nested
table exclusions and main-document-only picture inventory do not satisfy
all qualified owners automatically. Indentation remains F-X194. Successful
alternate-copy text replacement remains F-X202 after F-X198 safety, reusing
the separately captured PR317 slice. Stable-handle residuals remain F-X211.
No new F-ID is created, and contributor gates are not local evidence.

PR318 advanced from the retained assessed b1aa842 head to
`bfdd6ab732e8c0003ea8c6110e3c891fb098d5b3`. The immutable comparison shows
one new commit changing only the PowerPoint chart artifact SHA in a Word
native test. Complete changed source, Git OID/bytes/SHA and refreshed reviews
are captured independently. Keep the prior assessment, with this addition
attached to F-X205's template acceptance. Measure the actual locally authored
artifact after adopting approved geometry. Do not copy the contributor's
expected hash or rewrite immutable cross-viewer acceptance evidence.

This refresh has eight open PRs and23 open issues, with all scope linked to
existing owners. Issues264 and281 remain excluded.

## Advanced contribution heads reconciled, 2026-10-10

The09:01:30UTC refresh retained18 open PRs and26 included issues. Issues264
and281 remain excluded. Root authenticated complete manifests and237 source
records and inspected exact deltas for PR318 at4fc65a5, PR328 at381f766 and
PR333 atb228d5d. They remain F-X205/Issue308, F-X201/Issue304 and
F-X211/Issue316 respectively. Existing plans now include spacing boundary and
atomicity controls, default story styles, and qualified XML context controls.
No duplicate story, contributor gate credit or completion is created. Full
immutable heads and evidence are recorded in those three canonical plans.

## PR338 and further PR333 head intake

The09:35:24UTC refresh contains19 open PRs and26 included issues. Issues264
and281 remain excluded. NewPR338 atc189dbd73b9a61543ed64a03d2305d0017bbf498
closes existing Issue337 and joins F-X211, including its stacked PR336 work
once. PR333 advanced to193819ca0a1dcf3415017c0be1186fe8df6bcb86 and retains
F-X211/Issue316. Root authenticated255 source records and inspected exact
incremental deltas. Complete source assessment is retained in the existing
F-X211 plan. Intake grants no contributor gate credit or arbitrary new nesting
policy. The approved256 control policy and complete textbox resource,
namespace, discovery and public compatibility acceptance remain mandatory.

## S91 intake and verification checkpoint, 2026-10-10 11:43UTC

The latest read-only GitHub refresh at11:25:57UTC found no additional or changed
records. All19 PRs and26 included issues remain mapped above, with Issues264
and281 excluded. PR338 and PR333 source assessment is complete in F-X211's
existing plan. Assessment does not close their acceptance gates.

F-X194 is completed and integrated with authenticated native, installed
Python, typing, WASM, verified22 package, unchanged49-entry hash and141 policy
gates. Two publication-dependent policy cases remain skipped. Its integrated
source exactly matches the independently reviewed worker.

F-X212 has clean semantic, hash attribution, lint, HLD and carrier reviews.
Actual checks cover521 native unit tests,365 integration tests across the
original run and qualified retries,2331 regressions,2 doctests,3 CLI unit
tests and68 CLI integrations. Original failures remain recorded. Its updated
hash check matches all49 entries, with exactly the three approved audit keys
changed from the prior baseline. Verified22 packaging is running and policy
checks remain. No scoped completion or final sprint gate is claimed.

F-X197's three review findings are resolved and independent pass2 is clean.
Compiled support, fresh installed bindings, rendering, typing and remaining
scoped gates are pending. F-X195's core/provider source reviews are clean,
with runtime gates and remaining facade/provider wiring pending. F-X192 is
claimed for isolated own-locale regression preparation after completed F-282.
Its portable comparator and APA key authority remain technical barriers.

Five stories are completed and integrated, four are in progress and thirteen
remain approved. Final integrated verification, sprint review and close
preflight remain outstanding. The sprint is not ready for close.

## S91 TOC repair integration checkpoint, 2026-10-11

F-X213 is scoped-verified and integrated at4b5eb59b. Issue340 andPR341
acceptance passes for entry-owned TOC rebuild, opaque properties, namespace
closure, repeated rebuild and save/reopen. Independent integration pass17
is clean. Native and installed controls, typing, bothWASM, unchanged49
hashes, policy, verified22 archives, docs and README consumers pass.
The worker handoff is consumed. Its branch and scoped evidence remain
available. Final integrated sprint verification, review and delivery ledgers
remain due, so this feature is reviewed rather than counted completed.
Eight stories are completed, one is integrated and reviewed, four are
running, and eight approved stories await their prerequisite checkpoints.
GitHub reconciliation follows the sprint-close workflow. Issues264 and281
remain excluded and open.

## S91 rolling intake, 2026-10-11 18:56UTC

Authoritative GitHub refresh /private/tmp/S91-intake-refresh-20261011T185631Z
contains40 open issues and22 open PRs. New Issues344 through353 enter S91.
Issues264 and281 remain excluded and open, including the changed discussion281.
No existing PR head changed. The included total is38 issues and22 PRs.

| Issues | Acceptance owner | Required resolution |
|--------|------------------|---------------------|
| 344 | F-X214 | Bound recursive PowerPoint parsing and traversal |
| 345 and346 | F-X215 | Create charts in Python and replace existing chart data |
| 347,348 and349 | F-X216 | Import complete slides and merge decks with source formatting |
| 350 and351 | F-X217 | Import HTML and Markdown through public Word surfaces |
| 352 and353 | F-X218 | Export presentations as semantic HTML and Markdown |

All original acceptance owners remain. This intake adds five stories, not
duplicate implementations in the existing Python or CLI stories. Reuse native
providers and contributor slices once. Twenty-six active stories now comprise
eight completed, one integrated and reviewed, four running, eight approved
pending and five registered for design. No new implementation starts before
approved contracts and completion of its actual prerequisites.

## New intake wave admission, 2026-10-11

All five new issue designs are approved. The user approved the exact
pulldown-cmark0.13.4 parser dependency for F-X217. ROOT adopted the explicit
depth32 policy for F-X214, existing supported chart-data scope for F-X215,
transactional graph/section/default import options for F-X216, canonical
CommonMark round-trip/template policy for F-X217 and semantic export/resource
options for F-X218. No draft blocks these plans.

| Wave | Story | Prerequisite and resource boundary |
|------|-------|------------------------------------|
| 16 | F-X214 | No feature dependency, shape-tree/layout/renderer carriers wait for the F-X206 and F-X207 source checkpoint |
| 17 | F-X215 | Completed F-X197 and F-X206, charts/Word/PPT shared authoring carriers exclusive |
| 18 | F-X216 | Completed F-X206 and F-X215, graph import/PPT/CLI carriers exclusive |
| 19 | F-X217 | Completed F-X200 and F-X203, Word import/outbound/binding/CLI carriers exclusive |
| 20 | F-X218 | Completed F-X205 and F-X206, PPT semantic export/binding/CLI carriers exclusive |

Hash baseline remains ROOT-exclusive. Ordinary implementation never runs
concurrently in a shared source, test, manifest or HLD owner. Final full
verification and review cover the integrated26-story result.

## Rolling intake, PR354

The 2026-10-11 19:57 UTC refresh found [PR354](https://github.com/tensorbee/rdocx/pull/354) at head5a2c657eb705ab53048ab5d4e729b9014d6e0c0e. Its compatibility-mode and CommonMark export fixes address Issues332 and334 already assigned to F-X203. Reuse this contribution within that existing story, with no duplicate implementation owner. The exact retrieved patch is c4814b20267e16c75dd36a972a0e87eb7b8e2c07a3889614315125fc2a2f9b0d at /private/tmp/S91-PR354-intake/diff.patch. Read-only contribution assessment is complete at /private/tmp/S91-PR354-intake/assessment.md, SHA7c7dab495057d1bcc0920e94af90231c6aaab76c068f3b1e6780f46c8807d19a. Consumer reconciliation remains pending. Preserve original edge whitespace and terminal hard breaks, retain field boundaries, and change list state only for emitted nonempty table output. Reuse the constructor and Markdown hunks without copying contributor archive numbers. The approved original issue acceptance remains required, including cases the PR explicitly leaves out. Contributor results do not waive local pinned oracle or output-delta review gates.

Intake now contains40 open issues and23 open PRs. Issues264 and281 remain excluded discussion records. No issue or existing PR head changed in this refresh. This intake does not mark F-X203 implemented or complete.

## Rolling intake, PR355

The 2026-10-11 20:23 UTC refresh found [PR355](https://github.com/tensorbee/rdocx/pull/355) at head4005c4e51c3e2ce00d9c14dde1a2163fcaf8c5c1. It fixes Issue344 and joins the existing approved F-X214 parsing and traversal story. The exact contributor patch is7546f000296c4e6fedde1e7d860faea81ed2ebb145f432010b0e33d3d8e3fa4a in /private/tmp/S91-PR355-intake/diff.patch. Reuse its parser and regression work after reconciling the approved depth32 contract, URI ownership, mixed cycles, source retention and authored traversal safety. The donor depth16 limit and reported gates do not replace the approved acceptance or local verification. Read-only contribution assessment is in progress.

There are24 open PRs and38 included open issues. Discussion issues264 and281 remain excluded. No existing PR head changed. This intake adds no duplicate story and marks no implementation complete.

PR355 contribution assessment is complete at /private/tmp/S91-PR355-intake/assessment.md, SHAf8c78341918d25fe20908a0e1845eae23c0cc36f1d6c0ba7601e243b5246a15e. Reuse the same-reader parser mechanisms, iterative condition-target walk, empty timing-node error and eight contributor tests. Reconcile the donor local-name counting and omitted empty owners, recursive parsing and nested raw copies, depth16 policy, and incomplete grouping/manual-tree traversal guards with the approved F-X214 contract. No contributor archive measurements or reported test results are adopted as local evidence. Implementation remains at its existing exclusive source boundary.

## Rolling intake, PR356, PR357 and PR358

The 2026-10-11 20:46:45 UTC refresh captured three new PRs and no new or changed issues or existing PR heads.

[PR356](https://github.com/tensorbee/rdocx/pull/356) at head e82a2c0c0492517d918c48ee0bde93d291e1b369 joins existing approved F-X215 for Issues 345 and 346, charts creation and data replacement. Its exact captured patch is a15fdd633c8ab55446a7e63cf6c50036f9ed172701a6c044fb649c1c0d75837b at /private/tmp/S91-PR356-intake/diff.patch. Read-only contribution assessment is complete at /private/tmp/S91-PR356-intake/assessment.md, SHA eb542fe436252ded0f2ab12e999abf5723ae1f73429279af166b25f09f6b9a56. Reuse chart projection/data checks and creation/replacement tests. Reconcile workbook and raw ChartML preservation, shared chart ownership, qualified relationship lookup and prepared binding publication. Localize types into the approved existing carriers. Native Office and full local acceptance remain due. Reuse qualifying contributor code and tests within the original approved acceptance and existing carriers. Contributor test claims, archive measurements, omitted acceptance and new product files do not replace local verification or grant additional structural approval. Implementation retains its dependency and exclusive source boundaries.

[PR357](https://github.com/tensorbee/rdocx/pull/357) at head 16ee45f3b7d521edb344c4d59105654907bf8a97 joins existing approved F-X216 for Issues 347, 348 and 349, complete slide import, source formatting and deck merge. Its exact captured patch is d1203c60f78a383f36b4f9cf6986fef08abc9b45fe9da40b08daed8133702b62 at /private/tmp/S91-PR357-intake/diff.patch. Read-only contribution assessment is complete at /private/tmp/S91-PR357-intake/assessment.md, SHA b8670bafe72fb39a1d15e39c5c55a9f364b81a39a4c587960607450e77d3eb8b. Reuse graph copying, source-format comparison and graph/merge controls. Complete classic comments, batch-wide sharing, duplicate selection/map rejection, qualified comment identity/remapping, sections and transactional returned handles. Preserve the approved diagnostics and source-format contract. Reuse qualifying contributor code and tests within the original approved acceptance and existing carriers. Contributor test claims, archive measurements, omitted acceptance and new product files do not replace local verification or grant additional structural approval. Implementation retains its dependency and exclusive source boundaries.

[PR358](https://github.com/tensorbee/rdocx/pull/358) at head 649fa8ed6be75624ea5c1c82433c5a02c935fa89 joins existing approved F-X218 for Issues 352 and 353, semantic Markdown and HTML export. Its exact captured patch is dcf904ae0f6001e4ebfdbe83f28421394604e5fb4b62f728715efc0314dbbb45 at /private/tmp/S91-PR358-intake/diff.patch. Read-only contribution assessment is complete at /private/tmp/S91-PR358-intake/assessment.md, SHA 6fd0fb900c70ad64a58d099166d5937741ccb26f2fd0e55f1c5815c36334a346. Reuse source-order traversal, escaping and inline semantic-output tests. Complete inherited bullet/run semantics, usable Markdown images, safe embedded/adjacent assets, internal links and diagnostics. Use the approved existing exporter carriers and retain full source-authority and validator acceptance. Reuse qualifying contributor code and tests within the original approved acceptance and existing carriers. Contributor test claims, archive measurements, omitted acceptance and new product files do not replace local verification or grant additional structural approval. Implementation retains its dependency and exclusive source boundaries.

The current intake contains 27 open PRs and 40 open issues, of which 38 are included. Discussion issues 264 and 281 remain excluded and must stay open. These contributions add no duplicate story and do not mark an implementation complete. Exact refresh evidence is /private/tmp/S91-intake-refresh-20261011T204644Z/receipt.json.

## Rolling intake, PR359

The 2026-10-11 21:07:23 UTC refresh found [PR359](https://github.com/tensorbee/rdocx/pull/359) at head 7567b4031cc2c247a74ebb61b45cf5ee367f739c. It joins existing approved F-X217 for Issues 350 and 351, Word HTML and Markdown import. Exact patch c6fe3fd750a6ec23391626b161ee6b2a92eff28a244cae904510e6337733ff4f is captured at /private/tmp/S91-PR359-intake/diff.patch. Contribution assessment is complete at /private/tmp/S91-PR359-intake/assessment.md, SHA6fc5a95a1e5a96ff9b114223778a17f435b05842873ae469d732462373638933. The seven new commits are isolated in f217-only.diff, SHAb025943ebe9bbf1882a1957ec01f04ca457a28549f7227e055c13f9f87ea1ef9. Reuse requires the approved exact dependency/features, bounded decoded resources, complete admitted constructs, preserved template graphs and transactional owner publication. The branch is stacked on PR354, whose export/constructor work stays with F-X203 and must be reused once rather than duplicated. The approved exact pulldown-cmark =0.13.4 dependency and original linked-issue acceptance remain controlling. Contributor omissions and reported checks do not replace local acceptance. No source adoption or duplicate story is recorded.

There are 28 open PRs and 38 included issues. Only discussion Issue264 changed in this refresh and remains excluded alongside Issue281. No included issue or existing PR head changed. Exact refresh evidence is /private/tmp/S91-intake-refresh-20261011T210723Z/receipt.json.

## S91 intake closed, latest user decision

The user closed new issue and PR intake on 2026-10-12. The current S91 scope is frozen at38 included issues and28 PRs from the authenticated 05:40:03 UTC snapshot, /private/tmp/S91-intake-refresh-20261012T053947Z/receipt.json. Issues264 and281 remain excluded and open. Later arrivals belong to a later sprint. This decision supersedes rolling intake instructions for S91. Finish all existing acceptance owners and reconcile their PRs against integrated results without reopening intake.

## Approved S92 text watermark follow-up

On 2026-10-12 the user approved the temporary F-X201 DrawingML text opacity
boundary at 0.5, with named atomic refusal for other values and existing VML
full-range support retained. F-X219 in S92 must replace this restriction with
a faithful full-range representation after actual Word and LibreOffice
qualification. This is a deferred implementation follow-up from existing
F-X201 evidence, not new issue or PR intake.


## Bounded intake and minor releases, 2026-10-12

The user explicitly folds Issue360 and PR361 plus updated PR357 and PR359 into S91. The authenticated18:41UTC check adds one issue/PR pair and advances only those two existing heads. Included scope is39 issues and29 PRs, with discussion264/281 excluded and open. This admits these exact records, without an automatic rolling-intake loop.

F-X220 owns font embedding across native Word/PowerPoint, both Python packages and CLIs, real producer-format/native font-use qualification, bounded decoding, source preservation and explicit license policy. Its admitted PR361 head is a5795f2219d1cde5c613890369df3d88a84fc242. F-X216 consumes PR357 incremental16ee45f3 to57011087. F-X217 consumes PR359 incremental7567b403 toe974ea22, preserving completed semantic mark/code/owner contracts. Existing assessments and failed runs remain retained, and contributor reported gates are not local qualification.

F-X221 prepares both patch families in the existing lines after all28 repair stories complete: Word crates/Python0.16.1 and shared/PowerPoint crates/Python0.14.1. Current primary registry and GitHub CLI receipt verifies absent0.16.1/0.14.1 across both exact families, PyPI and remote tags. Preparation uses existing exact7/15 crate allowlists, new matching wheels/metadata, meaningful authenticated family notes and reviewed-SHA build-only rehearsal. No versions are changed yet and no publication is claimed. Final full sprint verification, clean sprint review and close remain due. The user authorizes publication and final addressed issue and PR reconciliation after successful publication. Exact reviewed main provenance, verified roles, publisher registration and artifacts remain mandatory, shared/PowerPoint first.

Twenty of28 active stories are complete. F-X204, F-X215 and F-X217 are running in isolated worktrees with disjoint file leases. F-X211, F-X216, F-X218, F-X220 and F-X221 remain approved and pending. Bibliography, navigation, standalone authoring and full-range text watermark work remain deferred to S92. Required evidence and carried dirty work are retained.


| Included issue | Included PR | Owner | Resolution contract |
|---|---|---|---|
| [360](https://github.com/tensorbee/rdocx/issues/360) | [361](https://github.com/tensorbee/rdocx/pull/361) | F-X220 | Explicit licensed font embedding, listing/removal/settings, both families and public surfaces, native producer/use and preserved imported font graphs |
| 347,348,349 | [357](https://github.com/tensorbee/rdocx/pull/357) | F-X216 | Existing full slide-import contract, incremental head57011087 assessed without redoing accepted provider work |
| 350,351 | [359](https://github.com/tensorbee/rdocx/pull/359) | F-X217 | Existing full Word import contract, incremental heade974ea22 adapted without losing semantic mark or code whitespace |

### Final release preparation boundary

F-X221 depends on every included repair F-X193 through F-X218 and F-X220 plus F-X222. It follows all source-sharing implementation waves and remains exclusive for versions, locks, manifests, notes, archive measurements and HLD release prose. Native and installed repair acceptance is not deferred to publication. Final versioned wheels are rebuilt once the complete source graph is fixed, then reviewed-SHA hosted artifacts and actual final integrated gates decide closure. The next commands after closure are /release rpptx-v0.14.1 followed by /release v0.16.1, under the user's explicit corrected patch publication authorization after exact-main-SHA and all release checks. Only /close-sprint merges main and creates the sprint tag. Only /release creates release tags or starts publication.

## User-admitted PR363 and Issue362 in S91

F-X222, Restore linear Word story range inventory and empty owners, M, is approved pending in S91. It resolves [Issue362](https://github.com/tensorbee/rdocx/issues/362) through assessed [PR363](https://github.com/tensorbee/rdocx/pull/363), immutable head0b9a3cccc1b31bac7ec4108c923faa176b072c47. The full five-carrier patch and two commits are authenticated against actual base733ce6f7 and all five resulting Git blobs. Comments/reviews contain no additional requests. Reuse the three native runtime carriers, adapt current physical source/namespace contracts, prove measured scaling and empty-owner behavior, and measure local archive metadata independently. Do not copy the donor's README/policy measurements.

Included scope is40 issues and30 PRs. Issues264/281 remain excluded and open, and no automatic rolling intake loop is reopened. There are29 stories:20 completed,4 running and5 approved pending. F222's completed dependencies are F203 and F213, with Word source handover after F204. It does not reopen completed F203 or deferred bibliography/navigation. F221 now depends on F222 and follows all28 included repairs before preparing the two minor release families. Final full verification, sprint review and close preflight remain due.

## S91 parallel delivery children

F-X211 remains the existing acceptance owner, with F-X211a, Word checked XML
owners and resource admission, and F-X211b, PowerPoint checked XML owners,
implementing in isolated worktrees. F-X211c, Integrated Python compatibility
and causal errors, follows both and its completed provider dependencies. The
parent closes only after all three children close. This delivery partition
keeps the frozen40 issues and30 PRs, all existing acceptance and the immutable
100-script metric. Bibliography, navigation and standalone authoring stay S92.

**Approved native Word font boundary, 2026-10-13**: F-X220 retains four API face kinds and genuine own-renderer and PowerPoint face selection. Native Word acceptance follows its documented one-font-per-name behavior, recording actual synthetic bold and empty native saved bold parts. The user moved native Word multi-face interoperability to F-X223 in S92. This does not move Issue360 or PR361 out of S91. Their native, public, CLI, preservation and authorization gates are qualified at the scoped F-X220 checkpoint. Final integrated sprint gates remain due.


## Current dependency-prefix admission, 2026-10-13

F-X211a, F-X215 and F-X218 complete at the reviewed current dependency-prefix checkpoint after root policy admission. Current paired execution measures32 Word and53 PowerPoint controls, all85 passing. Typing/fullstubs qualify through the explicit final-annotation-only bridge. Five verified packages, strict nine-crate Clippy, both WASM graphs and49 unchanged hashes qualify their actual recorded scopes. Original failed runner, genuine stub failure and all prior failed commands remain separate from successful successors.

**Policy checkpoint**: Actual README inventory/examples and immutable registry-graph successors pass at source106b, receipt17b34fb9e839ce65672ad613f2f978654a64e22a6c3346687d3e0b1dbf068982. Root authenticates both exit0 logs and safe cleanup. Earlier137 observed passing policy cases and2 platform skips remain historical. Original141 failures and focused inventory failure08f234 stay retained. This is the scoped composite, not a replayed full141 module.

Twenty-six of29 top-level stories are complete. Parent F-X211 remains running. F-X216 and compatibility child c are now claimed and running in separate worktrees. F-X221 remains approved pending all repair prerequisites. Word child a and PowerPoint child b are complete. No frozen100 score, whole2392 regression green, decoded-media/render authority, full workspace or sprint review is credited here. Intake stays frozen40 issues/30 PRs with264/281 excluded and open. Bibliography, navigation, standalone authoring, historical oracle regeneration and approved Word multi-face interoperability remain S92. No remote closure, main merge, tag, publication or push is claimed.


## F-X216 scoped completion, 2026-10-14

Complete slide import, source formatting and deck merge are integrated locally at501144b4. Issues347,348,349 and PR357 are locally addressed by the completed story. The validated handoff is consumed, current defining regression and scoped acceptance pass, and independent integration review reports zero defects and zero smells. Admitted native PowerPoint source comparisons, graph survival and two save/reopen cycles retain their exact documented boundaries.

Twenty-seven of29 top-level stories are complete. Parent F-X211 and its compatibility child c remain running. F-X221 remains approved pending all repair prerequisites. Final full integrated verification, both explicitly ignored focused appearance riders and sprint review remain due. Four missing historical oracle assets remain uncredited in S92. Intake stays frozen40 issues/30 PRs, with264/281 excluded and open. No remote closure, main merge, tag, publication or push is claimed.


## F-X211 failed candidate corrective partition, 2026-10-14

The complete immutable first candidate records72 assertion successes and28
failures, not a completed90-percent correct-file gate. F-X211d owns the
exposed Word XML/alias/ancestor/style residuals in a separate worktree. c
keeps PPT and shared causal/reporting fixes and final composed qualification.
The parent retains every original acceptance condition and now has four
delivery children. Twenty-nine top-level stories and the frozen40 issues/30
PRs are unchanged. Preserve all failed candidates and never stitch a best-of
retry rate or rewrite the corpus. Publication and close remain gated.


## S91 Word compatibility repair checkpoint, 2026-10-14

F-X211d is complete and integrated through its scoped dependency-prefix gate.
Children a, b and d are complete. Child c owns the final fresh paired bindings,
complete unchanged100 measurement and qualified native/rendered acceptance.
Twenty-seven of29 top-level stories remain complete, with parent F-X211 and
release preparation F-X221 unfinished. Bibliography, navigation and standalone
authoring remain in S92. Intake stays frozen40 issues/30 PRs, with discussions
264 and281 excluded and open. Final full verification, focused appearance
riders, sprint review and authorized publication/closure remain due.
