#!/usr/bin/env python3
"""Sprint state machine.

Holds the resumable state of a sprint in `.claude/scratch/SNN-run.json`, and
enforces the transitions that `.claude/WORKFLOW.md` describes as law. The point
is that "an agent may not merge" is checkable rather than aspirational.

Subcommands:
    init SNN                     create the run state from CURRENT_SPRINT.md
    status [SNN]                 print the current state
    set-phase SNN PHASE          advance the sprint phase
    mark-feature SNN F-ID STATE  advance one feature's state and its worker facts
    record-review SNN N ...      record a sprint-review pass result
    record-verification SNN ...  record a /verify result
    validate-handoff PATH ...    check a worker handoff before integration
    close-preflight SNN          the checks /close-sprint requires
    release-notes TAG            validate or render reviewed release notes
    unified-release-family TAG      validate matching Rust and Python versions
    published-unified-release TAG DIR
                                 verify exact selected registry entries
    python-release-artifacts TAG DIR
                                 validate one selected Python release set

Exit codes: 0 ok, 1 refused, 2 usage.
"""

from __future__ import annotations

import argparse
import hashlib
from email.message import Message
from email.parser import Parser
import json
import os
import re
import signal
import stat
import string
import subprocess
import sys
import tarfile
import time
import urllib.error
import urllib.request
import tomllib
import unicodedata
import zipfile
from html.parser import HTMLParser
from pathlib import Path

try:
    from . import prose_check
except ImportError:
    import prose_check

SCHEMA_VERSION = 2

REPO = Path(__file__).resolve().parent.parent
SCRATCH = REPO / ".claude" / "scratch"
HANDOFFS = REPO / ".claude" / "handoffs"
CURRENT_SPRINT = REPO / "docs" / "sprints" / "CURRENT_SPRINT.md"
BACKLOG = REPO / "docs" / "sprints" / "BACKLOG.md"
SPRINT_TRACKER = REPO / "docs" / "sprints" / "SPRINT_TRACKER.md"
AS_BUILT = REPO / "docs" / "sprints" / "AS_BUILT.md"
PLANS = REPO / ".claude" / "plans"
REVIEWS = REPO / ".claude" / "reviews"
CHANGELOG = REPO / "CHANGELOG.md"

SPRINT_RE = re.compile(r"^# Current Sprint, (S\d+(?:\.\d+)?)$", re.MULTILINE)
VALIDATION_ONLY_RE = re.compile(
    r"^\*\*Validation-only\*\*:\s*yes\s*$", re.MULTILINE | re.IGNORECASE
)
WAVE_ROW_RE = re.compile(
    r"^\|\s*(F-[A-Za-z0-9]+)\s*\|\s*([^|]+?)\s*\|\s*([SMLX]+)\s*\|\s*"
    r"(pending|in-progress|done|archived)\s*\|\s*([^|]*?)\s*\|$",
    re.MULTILINE,
)
BACKLOG_ROW_RE = re.compile(
    r"^\|\s*(F-[A-Za-z0-9]+)\s*\|[^|]*\|[^|]*\|\s*([SMLX]+)\s*\|\s*"
    r"(pending|in-progress|done|archived)\s*\|$",
    re.MULTILINE,
)
FID_RE = re.compile(r"^F-[A-Za-z0-9]+$")
SPRINT_ID_RE = re.compile(r"^S\d+(?:\.\d+)?$")
HANDOFF_FIELD_RE = re.compile(r"^\*\*([A-Za-z][A-Za-z -]*)\*\*:\s*(.+?)\s*$", re.MULTILINE)
SEMVER_COMPONENT_RE = r"(?:0|[1-9][0-9]*)"
RELEASE_TAG_RE = re.compile(
    rf"^(?:(?:rpptx|py-(?:rdocx|rpptx))-)?v{SEMVER_COMPONENT_RE}\."
    rf"{SEMVER_COMPONENT_RE}\.{SEMVER_COMPONENT_RE}$"
)
PYTHON_RELEASE_TAG_RE = re.compile(
    rf"^(?:(?P<stable>v)|(?P<incubating>rpptx-v)|py-(?P<legacy>rdocx|rpptx)-v)(?P<version>{SEMVER_COMPONENT_RE}\."
    rf"{SEMVER_COMPONENT_RE}\.{SEMVER_COMPONENT_RE})$"
)
PYTHON_RELEASE_PLATFORMS = (
    "manylinux_2_28_x86_64",
    "manylinux_2_28_aarch64",
    "musllinux_1_2_x86_64",
    "macosx_10_12_x86_64",
    "macosx_11_0_arm64",
    "win_amd64",
)
PYTHON_RELEASE_METADATA = {
    "rdocx": {
        "readme": "crates/rdocx-py/README.md",
        "summary": (
            "Native DOCX creation, editing, comparison, layout, and "
            "rendering for Python"
        ),
        "keywords": ("docx", "word", "ooxml", "documents", "pdf"),
    },
    "rpptx": {
        "readme": "crates/rpptx-py/README.md",
        "summary": (
            "Native PPTX creation, editing, comments, notes, and rendering "
            "for Python"
        ),
        "keywords": ("pptx", "powerpoint", "ooxml", "presentations", "pdf"),
    },
}
PYTHON_RELEASE_CLASSIFIERS = {
    "Development Status :: 4 - Beta",
    "Intended Audience :: Developers",
    "Programming Language :: Python :: 3",
    "Programming Language :: Python :: 3 :: Only",
    "Programming Language :: Python :: Implementation :: CPython",
    "Programming Language :: Rust",
    "Topic :: Office/Business",
    "Topic :: Software Development :: Libraries",
}
PYTHON_RELEASE_PROJECT_URLS = {
    "Homepage, https://github.com/tensorbee/rdocx",
    "Repository, https://github.com/tensorbee/rdocx",
    "Issues, https://github.com/tensorbee/rdocx/issues",
    "Changelog, https://github.com/tensorbee/rdocx/blob/main/CHANGELOG.md",
}
RELEASE_NOTE_PLACEHOLDER_RE = re.compile(
    r"\b(?:TBD|TODO|FIXME|CHANGEME|PLACEHOLDER)\b|\?\?\?|\[insert\b",
    re.IGNORECASE,
)
RELEASE_NOTE_HEADINGS = (
    "Highlights",
    "Added",
    "Fixed",
    "Compatibility",
    "Contributors",
)
COMMONMARK_BLOCK_TAGS = (
    "address|article|aside|base|basefont|blockquote|body|caption|center|col|"
    "colgroup|dd|details|dialog|dir|div|dl|dt|fieldset|figcaption|figure|footer|"
    "form|frame|frameset|h1|h2|h3|h4|h5|h6|head|header|hr|html|iframe|legend|"
    "li|link|main|menu|menuitem|nav|noframes|ol|optgroup|option|p|param|search|"
    "section|summary|table|tbody|td|tfoot|th|thead|title|tr|track|ul"
)
COMMONMARK_BLOCK_TAG_RE = re.compile(
    rf"^ {{0,3}}</?(?:{COMMONMARK_BLOCK_TAGS})(?:[ \t/>]|$)",
    re.IGNORECASE,
)
COMMONMARK_COMPLETE_TAG_RE = re.compile(
    r"^ {0,3}(?:"
    r"</[A-Za-z][A-Za-z0-9-]*[ \t]*>|"
    r"<[A-Za-z][A-Za-z0-9-]*(?:[ \t]+[^<>]*)?[ \t]*/?>"
    r")[ \t]*$"
)
COMMONMARK_BOUNDED_RAW_HTML = (
    (re.compile(r"^ {0,3}<\?"), re.compile(r"\?>")),
    (re.compile(r"^ {0,3}<![A-Z]"), re.compile(r">")),
    (re.compile(r"^ {0,3}<!\[CDATA\["), re.compile(r"\]\]>")),
)
HTML_VOID_ELEMENTS = frozenset(
    (
        "area",
        "base",
        "br",
        "col",
        "embed",
        "hr",
        "img",
        "input",
        "link",
        "meta",
        "param",
        "source",
        "track",
        "wbr",
    )
)
COMMONMARK_REFERENCE_DEFINITION_RE = re.compile(
    r"^ {0,3}\[(?:\\.|[^\[\]\r\n]){1,999}\]:"
)
COMMONMARK_REFERENCE_TITLE_RE = re.compile(
    r"^ {0,3}(?:"
    r'"(?:\\.|[^"\r\n])*"|'
    r"'(?:\\.|[^'\r\n])*'|"
    r"\((?:\\.|[^)\r\n])*\)"
    r")[ \t]*$"
)
COMMONMARK_AUTOLINK_RE = re.compile(
    r"<(?P<label>"
    r"(?:[A-Za-z][A-Za-z0-9+.-]{1,31}:[^<>\s]*|"
    r"[A-Za-z0-9.!#$%&'*+/=?^_`{|}~-]+@[^<>\s@]+)"
    r")>"
)
COMMONMARK_ESCAPABLE_PUNCTUATION = frozenset(string.punctuation)
MARKDOWN_ESCAPE_GUARD = "\N{WORD JOINER}"

# Every field a worker must have filled in before /integrate-feature will look
# at its branch. A missing one means the worker stopped early, and integrating
# on the strength of a half-written handoff is how unreviewed code lands.
HANDOFF_REQUIRED = (
    "F-ID",
    "Owner",
    "Branch",
    "Worktree",
    "Base",
    "Head",
    "Design plan",
    "Microscope",
    "Verify",
    "Hash harness",
    "Test gate",
)

PHASES = (
    "design",
    "questions",
    "implementation",
    "integration",
    "verification",
    "review",
    "ready_to_close",
    "blocked",
    "closed",
)

FEATURE_STATES = (
    "pending",
    "draft",
    "approved",
    "running",
    "reviewed",
    "completed",
    "blocked",
    "carried",
)

FEATURE_TRANSITIONS: dict[str, set[str]] = {
    "pending": {"draft", "approved", "blocked", "carried"},
    "draft": {"approved", "blocked", "carried"},
    "approved": {"running", "blocked", "carried"},
    "running": {"reviewed", "blocked", "carried"},
    "reviewed": {"running", "completed", "blocked"},
    "completed": set(),
    "blocked": {"draft", "approved", "running", "reviewed", "completed", "carried"},
    "carried": set(),
}

MAX_REVIEW_PASSES = 3

# Worker facts `mark-feature` records alongside the state. The CLI flag is the
# lowercase name with hyphens, so `--integration-commit` writes
# `integration_commit`.
WORKER_FIELDS = (
    "wave",
    "branch",
    "worktree",
    "base",
    "head",
    "handoff",
    "integration_commit",
)


def die(msg: str) -> None:
    print(f"sprint_workflow: {msg}", file=sys.stderr)
    sys.exit(1)


def protect_markdown_escapes(source: str) -> str:
    """Protect visible CommonMark escapes before classifying raw HTML."""
    protected: list[str] = []
    index = 0
    while index < len(source):
        if (
            source[index] == "\\"
            and index + 1 < len(source)
            and source[index + 1] in COMMONMARK_ESCAPABLE_PUNCTUATION
        ):
            # The format-only guard cannot collide with Markdown syntax and
            # does not make an otherwise empty section meaningful.
            protected.append(MARKDOWN_ESCAPE_GUARD)
            index += 2
            continue
        protected.append(source[index])
        index += 1
    return "".join(protected)


def markdown_heading_lines(lines: list[str], prefix: str) -> list[tuple[int, str]]:
    """Return exact Markdown headings outside non-rendered Markdown contexts."""
    headings: list[tuple[int, str]] = []
    fence: tuple[str, int] | None = None
    in_comment = False
    raw_html_end: re.Pattern[str] | None = None
    raw_html_until_blank = False
    for index, line in enumerate(lines):
        without_newline = protect_markdown_escapes(line.rstrip("\r\n"))
        if fence is not None:
            marker, minimum_length = fence
            close = re.fullmatch(rf" {{0,3}}({re.escape(marker)}+)[ \t]*", without_newline)
            if close is not None and len(close.group(1)) >= minimum_length:
                fence = None
            continue
        if raw_html_end is not None:
            if raw_html_end.search(without_newline):
                raw_html_end = None
            continue
        if raw_html_until_blank:
            if not without_newline.strip():
                raw_html_until_blank = False
            continue

        visible: list[str] = []
        cursor = 0
        while cursor < len(without_newline):
            if in_comment:
                end = without_newline.find("-->", cursor)
                if end < 0:
                    visible.append(" " * (len(without_newline) - cursor))
                    cursor = len(without_newline)
                    continue
                visible.append(" " * (end + 3 - cursor))
                cursor = end + 3
                in_comment = False
                continue
            start = without_newline.find("<!--", cursor)
            if start < 0:
                visible.append(without_newline[cursor:])
                break
            visible.append(without_newline[cursor:start])
            visible.append(" " * 4)
            cursor = start + 4
            in_comment = True

        visible_line = "".join(visible)
        opening = re.match(r"^ {0,3}(`{3,}|~{3,})", visible_line)
        if opening is not None:
            marker = opening.group(1)
            fence = (marker[0], len(marker))
            continue
        raw_html = re.match(
            r"^ {0,3}<(?P<tag>script|pre|style|textarea)(?:[ \t>]|$)",
            visible_line,
            re.IGNORECASE,
        )
        if raw_html is not None:
            tag = raw_html.group("tag").lower()
            close = re.compile(rf"</{re.escape(tag)}\s*>", re.IGNORECASE)
            if close.search(visible_line[raw_html.end() :]) is None:
                raw_html_end = close
            continue
        matched_bounded_raw_html = False
        for start_pattern, close in COMMONMARK_BOUNDED_RAW_HTML:
            start = start_pattern.match(visible_line)
            if start is None:
                continue
            if close.search(visible_line[start.end() :]) is None:
                raw_html_end = close
            matched_bounded_raw_html = True
            break
        if matched_bounded_raw_html:
            continue
        if COMMONMARK_BLOCK_TAG_RE.match(visible_line) is not None:
            raw_html_until_blank = True
            continue
        if COMMONMARK_COMPLETE_TAG_RE.fullmatch(visible_line) is not None:
            raw_html_until_blank = True
            continue
        if visible_line.startswith(prefix):
            headings.append((index, visible_line.rstrip()[len(prefix) :]))
    return headings


class MarkdownOutsideRawHtml(HTMLParser):
    """Collect Markdown source that is not contained by a raw HTML element."""

    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.hidden_tags: list[str] = []
        self.text: list[str] = []

    def handle_starttag(
        self, tag: str, attrs: list[tuple[str, str | None]]
    ) -> None:
        del attrs
        tag = tag.lower()
        if tag not in HTML_VOID_ELEMENTS:
            self.hidden_tags.append(tag)

    def handle_endtag(self, tag: str) -> None:
        tag = tag.lower()
        for index in range(len(self.hidden_tags) - 1, -1, -1):
            if self.hidden_tags[index] == tag:
                del self.hidden_tags[index:]
                break

    def handle_data(self, data: str) -> None:
        if not self.hidden_tags:
            self.text.append(data)


def closing_markdown_delimiter(
    source: str, opening_index: int, opening: str, closing: str
) -> int | None:
    """Find one escaped, balanced Markdown delimiter within bounded source."""
    depth = 1
    index = opening_index + 1
    while index < len(source):
        character = source[index]
        if character == "\\":
            index += 2
            continue
        if character == opening:
            depth += 1
        elif character == closing:
            depth -= 1
            if depth == 0:
                return index
        index += 1
    return None


def markdown_without_link_destinations(source: str) -> str:
    """Keep rendered labels while removing inline and reference destinations."""
    visible: list[str] = []
    index = 0
    while index < len(source):
        if source[index] == "\\" and index + 1 < len(source):
            visible.append(source[index : index + 2])
            index += 2
            continue
        image = source[index] == "!" and source[index + 1 : index + 2] == "["
        label_start = index + 1 if image else index
        if source[label_start : label_start + 1] != "[":
            visible.append(source[index])
            index += 1
            continue
        label_end = closing_markdown_delimiter(source, label_start, "[", "]")
        if label_end is None:
            visible.append(source[index])
            index += 1
            continue
        suffix_start = label_end + 1
        suffix_end: int | None = None
        if source[suffix_start : suffix_start + 1] == "(":
            suffix_end = closing_markdown_delimiter(source, suffix_start, "(", ")")
        elif source[suffix_start : suffix_start + 1] == "[":
            suffix_end = closing_markdown_delimiter(source, suffix_start, "[", "]")
        if suffix_end is None:
            visible.append(source[index : label_end + 1])
            index = label_end + 1
            continue
        visible.append(source[label_start + 1 : label_end])
        index = suffix_end + 1
    return "".join(visible)


def markdown_code_span(source: str, index: int) -> tuple[int, int] | None:
    """Return the content bounds for a code span beginning at index."""
    run_end = index
    while run_end < len(source) and source[run_end] == "`":
        run_end += 1
    marker = source[index:run_end]
    close = source.find(marker, run_end)
    while close >= 0 and (
        source[close - 1 : close] == "`"
        or source[close + len(marker) : close + len(marker) + 1] == "`"
    ):
        close = source.find(marker, close + len(marker))
    if close < 0:
        return None
    return run_end, close


def markdown_without_inline_code(source: str) -> tuple[str, bool]:
    """Remove code-span syntax and report whether a span renders code content."""
    visible: list[str] = []
    index = 0
    while index < len(source):
        if source[index] != "`":
            visible.append(source[index])
            index += 1
            continue
        bounds = markdown_code_span(source, index)
        if bounds is None:
            run_end = index
            while run_end < len(source) and source[run_end] == "`":
                run_end += 1
            marker = source[index:run_end]
            visible.append(marker)
            index = run_end
            continue
        run_end, close = bounds
        marker = source[index:run_end]
        code = source[run_end:close].replace("\n", " ")
        if code.startswith(" ") and code.endswith(" ") and code.strip():
            code = code[1:-1]
        if contains_meaningful_rendered_character(code):
            return "", True
        index = close + len(marker)
    return "".join(visible), False


def escape_html_syntax(source: str) -> str:
    """Escape code text only for the internal HTML visibility pass."""
    return source.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def protect_inline_code_from_html(source: str) -> str:
    """Keep HTML-like code-span content visible to the Markdown pass."""
    protected: list[str] = []
    index = 0
    while index < len(source):
        if source[index] != "`":
            protected.append(source[index])
            index += 1
            continue
        bounds = markdown_code_span(source, index)
        if bounds is None:
            run_end = index
            while run_end < len(source) and source[run_end] == "`":
                run_end += 1
            marker = source[index:run_end]
            protected.append(marker)
            index = run_end
            continue
        run_end, close = bounds
        marker = source[index:run_end]
        protected.append(marker)
        protected.append(escape_html_syntax(source[run_end:close]))
        protected.append(marker)
        index = close + len(marker)
    return "".join(protected)


def protect_markdown_code_from_html(section: str) -> str:
    """Protect fenced and inline code before parsing surrounding raw HTML."""
    protected: list[str] = []
    prose: list[str] = []
    fence: tuple[str, int] | None = None

    def flush_prose() -> None:
        if prose:
            protected.append(protect_inline_code_from_html("".join(prose)))
            prose.clear()

    for line in section.splitlines(keepends=True):
        without_newline = line.rstrip("\r\n")
        if fence is not None:
            marker, minimum_length = fence
            close = re.fullmatch(
                rf" {{0,3}}({re.escape(marker)}+)[ \t]*", without_newline
            )
            if close is not None and len(close.group(1)) >= minimum_length:
                protected.append(line)
                fence = None
            else:
                protected.append(escape_html_syntax(line))
            continue
        opening = re.match(r"^ {0,3}(`{3,}|~{3,})", without_newline)
        if opening is not None:
            marker = opening.group(1)
            info = without_newline[opening.end() :]
            if marker[0] == "~" or "`" not in info:
                flush_prose()
                protected.append(line)
                fence = (marker[0], len(marker))
                continue
        prose.append(line)
    flush_prose()
    return "".join(protected)


def markdown_prose_and_code(section: str) -> tuple[str, bool]:
    """Remove non-rendered block syntax and report visible fenced code."""
    prose: list[str] = []
    fence: tuple[str, int] | None = None
    reference_continuation: str | None = None
    for line in section.splitlines(keepends=True):
        without_newline = line.rstrip("\r\n")
        if fence is not None:
            marker, minimum_length = fence
            close = re.fullmatch(
                rf" {{0,3}}({re.escape(marker)}+)[ \t]*", without_newline
            )
            if close is not None and len(close.group(1)) >= minimum_length:
                fence = None
            elif contains_meaningful_rendered_character(without_newline):
                return "", True
            continue
        if reference_continuation is not None:
            if not without_newline.strip():
                reference_continuation = None
                continue
            if reference_continuation == "destination":
                reference_continuation = "title"
                continue
            if COMMONMARK_REFERENCE_TITLE_RE.fullmatch(without_newline) is not None:
                reference_continuation = None
                continue
            reference_continuation = None
        opening = re.match(r"^ {0,3}(`{3,}|~{3,})", without_newline)
        if opening is not None:
            marker = opening.group(1)
            info = without_newline[opening.end() :]
            if marker[0] == "~" or "`" not in info:
                fence = (marker[0], len(marker))
                continue
        reference = COMMONMARK_REFERENCE_DEFINITION_RE.match(without_newline)
        if reference is not None:
            reference_continuation = (
                "destination"
                if not without_newline[reference.end() :].strip()
                else "title"
            )
            continue
        prose.append(line)
    return "".join(prose), False


def markdown_without_block_markers(source: str) -> str:
    """Remove list and quote markers that do not represent section text."""
    visible: list[str] = []
    for line in source.splitlines(keepends=True):
        while True:
            quote = re.match(r"^ {0,3}>[ \t]?", line)
            if quote is None:
                break
            line = line[quote.end() :]
        while True:
            marker = re.match(r"^ {0,3}(?:[-+*]|[0-9]{1,9}[.)])(?:[ \t]+|$)", line)
            if marker is None:
                break
            line = line[marker.end() :]
        task = re.match(r"^\[[ xX]\](?:[ \t]+|$)", line)
        if task is not None:
            line = line[task.end() :]
        visible.append(line)
    return "".join(visible)


def contains_meaningful_rendered_character(source: str) -> bool:
    """Accept alphanumeric text and pictographs, but not Markdown syntax alone."""
    return any(
        character.isalnum() or unicodedata.category(character) == "So"
        for character in source
    )


def has_meaningful_visible_text(section: str) -> bool:
    """Return whether one bounded section renders meaningful HTML or Markdown."""
    section = protect_markdown_escapes(section)
    section = protect_markdown_code_from_html(section)
    section = COMMONMARK_AUTOLINK_RE.sub(lambda match: match.group("label"), section)
    parser = MarkdownOutsideRawHtml()
    parser.feed(section)
    parser.close()
    prose, fenced_code = markdown_prose_and_code("".join(parser.text))
    if fenced_code:
        return True
    prose, inline_code = markdown_without_inline_code(prose)
    if inline_code:
        return True
    prose = markdown_without_link_destinations(prose)
    prose = markdown_without_block_markers(prose)
    return contains_meaningful_rendered_character(prose)


def render_release_notes(changelog: str, tag: str) -> str:
    """Validate and return one reviewed changelog section body."""
    if not RELEASE_TAG_RE.fullmatch(tag):
        raise ValueError(
            f"{tag!r} is not a release tag, expected vX.Y.Z, rpptx-vX.Y.Z, "
            "py-rdocx-vX.Y.Z, or py-rpptx-vX.Y.Z"
        )

    lines = changelog.splitlines(keepends=True)
    release_matches = [
        index
        for index, heading in markdown_heading_lines(lines, "## ")
        if heading == tag
    ]
    if len(release_matches) != 1:
        raise ValueError(
            f"CHANGELOG.md must contain exactly one `## {tag}` heading, "
            f"found {len(release_matches)}"
        )

    start = release_matches[0] + 1
    later_sections = [
        index
        for index, _ in markdown_heading_lines(lines[start:], "## ")
    ]
    end = start + later_sections[0] if later_sections else len(lines)
    body_lines = lines[start:end]
    while body_lines and not body_lines[0].strip():
        body_lines.pop(0)
    while body_lines and not body_lines[-1].strip():
        body_lines.pop()
    body = "".join(body_lines)
    if not body.strip():
        raise ValueError(f"`## {tag}` has no release-note body")
    if RELEASE_NOTE_PLACEHOLDER_RE.search(body):
        raise ValueError(f"`## {tag}` contains a placeholder token")

    headings = markdown_heading_lines(body_lines, "### ")
    found = [heading for _, heading in headings]
    for required in RELEASE_NOTE_HEADINGS:
        count = found.count(required)
        if count != 1:
            raise ValueError(
                f"`## {tag}` must contain exactly one `### {required}` heading, "
                f"found {count}"
            )
    required_positions = [found.index(required) for required in RELEASE_NOTE_HEADINGS]
    if required_positions != sorted(required_positions):
        raise ValueError(f"`## {tag}` required headings are out of order")

    for heading_index, (line_index, heading) in enumerate(headings):
        if heading not in RELEASE_NOTE_HEADINGS:
            continue
        next_line = (
            headings[heading_index + 1][0]
            if heading_index + 1 < len(headings)
            else len(body_lines)
        )
        section_text = "".join(body_lines[line_index + 1 : next_line])
        if not has_meaningful_visible_text(section_text):
            raise ValueError(f"`## {tag}` section `### {heading}` is empty")

    return body.rstrip() + "\n"


def cmd_release_notes(args: argparse.Namespace) -> int:
    try:
        notes = render_release_notes(CHANGELOG.read_text(encoding="utf-8"), args.tag)
    except (OSError, ValueError) as error:
        die(f"release notes: {error}")
    if args.render:
        sys.stdout.write(notes)
    else:
        print(f"release-notes {args.tag}: ok")
    return 0


def validate_python_project_metadata(
    metadata: object, distribution: str, artifact_name: str
) -> None:
    """Validate the user-facing metadata embedded in one Python artifact."""
    if not isinstance(metadata, Message):
        raise TypeError("metadata must be an email message")
    contract = PYTHON_RELEASE_METADATA[distribution]
    if metadata.get("Summary") != contract["summary"]:
        raise ValueError(f"{artifact_name} has incomplete project summary")
    description_content_type = metadata.get(
        "Description-Content-Type", ""
    ).partition(";")[0].strip().lower()
    if description_content_type != "text/markdown":
        raise ValueError(f"{artifact_name} lacks a Markdown long description")
    if metadata.get("Author") != "Atul Sharma":
        raise ValueError(f"{artifact_name} lacks the reviewed project author")
    keywords = tuple(
        keyword.strip()
        for keyword in metadata.get("Keywords", "").split(",")
        if keyword.strip()
    )
    if keywords != contract["keywords"]:
        raise ValueError(f"{artifact_name} has incomplete project keywords")
    if set(metadata.get_all("Classifier", [])) != PYTHON_RELEASE_CLASSIFIERS:
        raise ValueError(f"{artifact_name} has incomplete project classifiers")
    if set(metadata.get_all("Project-URL", [])) != PYTHON_RELEASE_PROJECT_URLS:
        raise ValueError(f"{artifact_name} has incomplete project URLs")
    description = metadata.get_payload()
    for required in (
        f"# {distribution}-py",
        "## Installation",
        f"python -m pip install {distribution}",
        "## Quick start",
        "## Type checking",
        "## Project links",
    ):
        if required not in description:
            raise ValueError(
                f"{artifact_name} long description lacks {required!r}"
            )
    reviewed_description = (REPO / contract["readme"]).read_text(encoding="utf-8")
    normalized_description = description.replace("\r\n", "\n")
    normalized_reviewed = reviewed_description.replace("\r\n", "\n")
    if normalized_description.rstrip("\n") != normalized_reviewed.rstrip("\n"):
        raise ValueError(
            f"{artifact_name} long description differs from the reviewed "
            f"{contract['readme']}"
        )


RELEASE_CRATES = {
    "rdocx": (
        "rdocx-opc", "rdocx-oxml", "rdocx-layout", "rdocx-html",
        "rdocx-pdf", "rdocx", "rdocx-cli",
    ),
    "rpptx": (
        "oxml-core", "oxml-opc", "oxml-media", "oxml-layout",
        "oxml-drawing", "oxml-pdf", "oxml-sml", "oxml-cli-support",
        "oxml-chart", "rpptx-oxml", "rpptx-chart", "rpptx-layout",
        "rpptx-render", "rpptx", "rpptx-cli",
    ),
}


def validate_unified_release_family(tag: str) -> dict[str, object]:
    """Check the exact selected Rust and Python version carriers for a new tag."""
    match = PYTHON_RELEASE_TAG_RE.fullmatch(tag)
    if match is None or match.group("legacy") is not None:
        raise ValueError(f"{tag!r} is not a unified release tag")
    distribution = "rdocx" if match.group("stable") else "rpptx"
    version = match.group("version")
    workspace = tomllib.loads((REPO / "Cargo.toml").read_text(encoding="utf-8"))
    if distribution == "rdocx" and workspace["workspace"]["package"]["version"] != version:
        raise ValueError("stable workspace version differs from release tag")
    dependencies = workspace["workspace"]["dependencies"]
    selected = set(RELEASE_CRATES[distribution])
    publishable = set()
    lock = tomllib.loads((REPO / "Cargo.lock").read_text(encoding="utf-8"))
    locked = {(package["name"], package["version"]) for package in lock["package"]}
    for member in workspace["workspace"]["members"]:
        manifest = tomllib.loads((REPO / member / "Cargo.toml").read_text(encoding="utf-8"))
        package = manifest["package"]
        carrier = package["version"]
        member_version = workspace["workspace"]["package"]["version"] if isinstance(carrier, dict) and carrier.get("workspace") is True else carrier
        registry = package.get("publish")
        if member_version == version and (registry is None or registry is True or isinstance(registry, list) and "crates-io" in registry):
            publishable.add(package["name"])
    if publishable != selected:
        raise ValueError(f"selected publishable crates differ: missing={sorted(selected - publishable)}, extra={sorted(publishable - selected)}")
    for crate in (*RELEASE_CRATES[distribution], f"{distribution}-py"):
        manifest = tomllib.loads(
            (REPO / "crates" / crate / "Cargo.toml").read_text(encoding="utf-8")
        )
        carrier = manifest["package"]["version"]
        actual = workspace["workspace"]["package"]["version"] if isinstance(carrier, dict) and carrier.get("workspace") is True else carrier
        if actual != version:
            raise ValueError(f"{crate} is {actual}, expected {version}")
        if crate in selected and (crate, version) not in locked:
            raise ValueError(f"Cargo.lock lacks {crate} {version}")
        if crate in dependencies and dependencies[crate]["version"] != version:
            raise ValueError(f"workspace dependency {crate} differs from {version}")
    project = tomllib.loads(
        (REPO / "crates" / f"{distribution}-py" / "pyproject.toml").read_text(encoding="utf-8")
    )
    if project["project"]["version"] != version:
        raise ValueError(f"{distribution} Python distribution differs from {version}")
    return {"tag": tag, "distribution": distribution, "version": version}


def validate_release_package_inventory(tag: str, directory: Path) -> None:
    """Check the unpacked packages left by a workspace publication dry run."""
    family = validate_unified_release_family(tag)
    version = str(family["version"])
    expected = {
        f"{crate}-{version}" for crate in RELEASE_CRATES[str(family["distribution"])]
    }
    actual = {path.name for path in directory.glob(f"*-{version}")}
    if actual != expected:
        raise ValueError(
            f"selected package inventory differs: missing={sorted(expected - actual)}, "
            f"extra={sorted(actual - expected)}"
        )
    for name in expected:
        package = directory / name
        if not package.is_dir() or not (package / "Cargo.toml").is_file():
            raise ValueError(f"dry-run package is incomplete: {name}")


def cmd_unified_release_family(args: argparse.Namespace) -> int:
    try:
        result = validate_unified_release_family(args.tag)
    except (KeyError, OSError, TypeError, ValueError) as error:
        die(f"unified release family: {error}")
    print(f"unified-release-family {result['tag']}: {result['distribution']} {result['version']}, ok")
    return 0


def verify_published_unified_release(tag: str, directory: Path) -> dict[str, object]:
    """Confirm both registries expose the selected exact version and Python bytes."""
    from hashlib import sha256

    family = validate_unified_release_family(tag)
    distribution = str(family["distribution"])
    version = str(family["version"])
    expected_python = {
        path.name: sha256(path.read_bytes()).hexdigest()
        for path in directory.iterdir()
        if path.name.startswith(f"{distribution}-{version}-")
        or path.name == f"{distribution}-{version}.tar.gz"
    }
    if len(expected_python) != 7:
        raise ValueError("reviewed family assets lack seven Python distributions")

    def fetch_json(url: str) -> dict[str, object]:
        request = urllib.request.Request(url, headers={"User-Agent": "rdocx-release/1"})
        with urllib.request.urlopen(request, timeout=30) as response:
            value = json.load(response)
        if not isinstance(value, dict):
            raise ValueError(f"invalid registry response from {url}")
        return value

    pending = set(RELEASE_CRATES[distribution])
    for attempt in range(20):
        for crate in tuple(sorted(pending)):
            try:
                result = fetch_json(f"https://crates.io/api/v1/crates/{crate}/{version}")
                entry = result.get("version")
                if isinstance(entry, dict) and entry.get("num") == version:
                    pending.remove(crate)
            except (urllib.error.HTTPError, urllib.error.URLError):
                pass
        try:
            project = fetch_json(f"https://pypi.org/pypi/{distribution}/{version}/json")
            urls = project.get("urls")
            if not isinstance(urls, list):
                raise ValueError("PyPI release lacks a file list")
            actual_python = {
                entry["filename"]: entry["digests"]["sha256"]
                for entry in urls
                if isinstance(entry, dict)
            }
            python_ready = actual_python == expected_python
        except (KeyError, TypeError, urllib.error.HTTPError, urllib.error.URLError):
            python_ready = False
        if not pending and python_ready:
            return family
        if attempt < 19:
            time.sleep(30)
    raise ValueError(
        f"registry publication incomplete: crates={sorted(pending)}, "
        f"PyPI exact files={python_ready}"
    )


def cmd_published_unified_release(args: argparse.Namespace) -> int:
    try:
        result = verify_published_unified_release(args.tag, Path(args.directory))
    except (OSError, TypeError, ValueError) as error:
        die(f"published unified release: {error}")
    print(f"published-unified-release {result['tag']}: both registries, ok")
    return 0


def validate_python_release_artifacts(tag: str, directory: Path) -> dict[str, object]:
    """Validate the exact selected Python distribution artifact set."""
    match = PYTHON_RELEASE_TAG_RE.fullmatch(tag)
    if match is None:
        raise ValueError(
            f"{tag!r} is not a Python release tag, expected "
            "vX.Y.Z or rpptx-vX.Y.Z"
        )
    distribution = "rdocx" if match.group("stable") else ("rpptx" if match.group("incubating") else match.group("legacy"))
    version = match.group("version")
    if not directory.is_dir():
        raise ValueError(f"Python release artifact directory is missing: {directory}")

    expected_wheels = {
        f"{distribution}-{version}-cp39-abi3-{platform}.whl"
        for platform in PYTHON_RELEASE_PLATFORMS
    }
    expected_sdists = {f"{distribution}-{version}.tar.gz"}
    expected_names = expected_wheels | expected_sdists
    entries = sorted(directory.iterdir(), key=lambda path: path.name)
    actual_names = {path.name for path in entries}
    if len(entries) != len(actual_names):
        raise ValueError("Python release artifact directory has duplicate names")
    if actual_names != expected_names:
        missing = sorted(expected_names - actual_names)
        unexpected = sorted(actual_names - expected_names)
        raise ValueError(
            "Python release artifact set differs from the exact family, "
            f"missing={missing}, unexpected={unexpected}"
        )
    if any(not path.is_file() for path in entries):
        raise ValueError("Python release artifact set contains a non-file entry")

    for name in sorted(expected_wheels):
        path = directory / name
        distribution = name.split("-", 1)[0]
        platform = name.removesuffix(".whl").rsplit("-", 1)[1]
        dist_info = f"{distribution}-{version}.dist-info"
        metadata_name = f"{dist_info}/METADATA"
        wheel_name = f"{dist_info}/WHEEL"
        with zipfile.ZipFile(path) as archive:
            names = archive.namelist()
            metadata_records = [
                member
                for member in names
                if member.endswith(".dist-info/METADATA")
                or member.endswith(".dist-info/WHEEL")
            ]
            if sorted(metadata_records) != sorted((metadata_name, wheel_name)):
                raise ValueError(
                    f"{name} has unexpected distribution metadata records"
                )
            if names.count(metadata_name) != 1 or names.count(wheel_name) != 1:
                raise ValueError(f"{name} lacks one exact METADATA and WHEEL record")
            metadata = Parser().parsestr(
                archive.read(metadata_name).decode("utf-8", errors="strict")
            )
            wheel = Parser().parsestr(
                archive.read(wheel_name).decode("utf-8", errors="strict")
            )
        if metadata.get("Name") != distribution:
            raise ValueError(f"{name} has project name {metadata.get('Name')!r}")
        if metadata.get("Version") != version:
            raise ValueError(f"{name} has project version {metadata.get('Version')!r}")
        validate_python_project_metadata(metadata, distribution, name)
        expected_tag = f"cp39-abi3-{platform}"
        if wheel.get_all("Tag", []) != [expected_tag]:
            raise ValueError(
                f"{name} has wheel tags {wheel.get_all('Tag', [])!r}, "
                f"expected {[expected_tag]!r}"
            )

    for name in sorted(expected_sdists):
        path = directory / name
        distribution = name.split("-", 1)[0]
        root = f"{distribution}-{version}"
        metadata_name = f"{root}/PKG-INFO"
        with tarfile.open(path, mode="r:gz") as archive:
            members = archive.getmembers()
            names = [member.name for member in members]
            if any(
                member.name.startswith("/")
                or ".." in Path(member.name).parts
                or not (
                    member.name == root or member.name.startswith(f"{root}/")
                )
                for member in members
            ):
                raise ValueError(f"{name} contains a path outside {root}/")
            if names.count(metadata_name) != 1:
                raise ValueError(f"{name} lacks one exact PKG-INFO record")
            metadata_member = archive.getmember(metadata_name)
            if not metadata_member.isfile():
                raise ValueError(f"{name} PKG-INFO is not a regular file")
            metadata_file = archive.extractfile(metadata_member)
            if metadata_file is None:
                raise ValueError(f"{name} PKG-INFO is not a regular file")
            metadata = Parser().parsestr(
                metadata_file.read().decode("utf-8", errors="strict")
            )
        if metadata.get("Name") != distribution:
            raise ValueError(f"{name} has project name {metadata.get('Name')!r}")
        if metadata.get("Version") != version:
            raise ValueError(f"{name} has project version {metadata.get('Version')!r}")
        validate_python_project_metadata(metadata, distribution, name)

    return {
        "tag": tag,
        "version": version,
        "distributions": (distribution,),
        "wheels": len(expected_wheels),
        "sdists": len(expected_sdists),
    }


def cmd_python_release_artifacts(args: argparse.Namespace) -> int:
    try:
        result = validate_python_release_artifacts(args.tag, Path(args.directory))
    except (OSError, UnicodeError, ValueError, tarfile.TarError, zipfile.BadZipFile) as error:
        die(f"python release artifacts: {error}")
    print(
        f"python-release-artifacts {result['tag']}: "
        f"{result['wheels']} wheels, {result['sdists']} source distributions, ok"
    )
    return 0


def state_path(sprint: str) -> Path:
    return SCRATCH / f"{sprint}-run.json"


def load(sprint: str) -> dict:
    p = state_path(sprint)
    if not p.exists():
        die(f"no run state for {sprint}. Run `init {sprint}` first.")
    data = json.loads(p.read_text(encoding="utf-8"))
    if data.get("schema_version") != SCHEMA_VERSION:
        die(
            f"run state schema {data.get('schema_version')} != {SCHEMA_VERSION}. "
            "Delete the file and re-init."
        )
    return data


def save(sprint: str, data: dict) -> None:
    SCRATCH.mkdir(parents=True, exist_ok=True)
    state_path(sprint).write_text(
        json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )


def git_head() -> str:
    """Return the commit whose evidence is being recorded or checked."""
    result = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=REPO,
        check=True,
        capture_output=True,
        text=True,
    )
    return result.stdout.strip()


def parse_current_sprint() -> tuple[str, list[dict]]:
    if not CURRENT_SPRINT.exists():
        die(f"{CURRENT_SPRINT} not found")
    text = CURRENT_SPRINT.read_text(encoding="utf-8")
    m = SPRINT_RE.search(text)
    if not m:
        die("CURRENT_SPRINT.md has no parseable `# Current Sprint, SNN` heading")
    features = [
        {"fid": r[0], "title": r[1], "size": r[2], "tracker_status": r[3], "owner": r[4]}
        for r in WAVE_ROW_RE.findall(text)
    ]
    if not features and not VALIDATION_ONLY_RE.search(text):
        die(
            "CURRENT_SPRINT.md has no parseable wave rows. An intentionally "
            "empty sprint must declare `**Validation-only**: yes`."
        )
    return m.group(1), features


def backlog_statuses() -> dict[str, str]:
    if not BACKLOG.exists():
        die(f"{BACKLOG} not found")
    return {r[0]: r[2] for r in BACKLOG_ROW_RE.findall(BACKLOG.read_text(encoding="utf-8"))}


def completed_record_problems(sprint: str, fid: str) -> list[str]:
    """Return missing or stale durable records for one completed feature."""
    problems: list[str] = []
    current_id, current_features = parse_current_sprint()
    current = {feature["fid"]: feature for feature in current_features}
    feature = current.get(fid)
    if current_id != sprint:
        problems.append(f"CURRENT_SPRINT.md says {current_id}, run state says {sprint}")
    elif feature is None:
        problems.append(f"{fid} has no CURRENT_SPRINT.md row")
    else:
        if feature["tracker_status"] != "done":
            problems.append(
                f"{fid} is completed in run state but "
                f"'{feature['tracker_status']}' in CURRENT_SPRINT.md"
            )
        if feature["owner"] not in ("", "-"):
            problems.append(
                f"{fid} is completed but CURRENT_SPRINT.md owner is "
                f"'{feature['owner']}'"
            )

    plan = PLANS / f"{fid}-design.md"
    if not plan.exists():
        problems.append(f"{fid} has no design plan at {plan.relative_to(REPO)}")
    elif "**Status**: completed" not in plan.read_text(encoding="utf-8"):
        problems.append(f"{fid} design plan is not completed")

    if not AS_BUILT.exists() or not re.search(
        rf"^### {re.escape(fid)},", AS_BUILT.read_text(encoding="utf-8"), re.MULTILINE
    ):
        problems.append(f"{fid} has no AS_BUILT.md entry")

    tracker_text = (
        SPRINT_TRACKER.read_text(encoding="utf-8") if SPRINT_TRACKER.exists() else ""
    )
    if not re.search(
        rf"^\|\s*{re.escape(fid)}\s*\|\s*{re.escape(sprint)}\s*\|",
        tracker_text,
        re.MULTILINE,
    ):
        problems.append(f"{fid} has no {sprint} row in SPRINT_TRACKER.md")
    return problems


def completed_owner_problems(data: dict) -> list[str]:
    """Return completed features whose run-state owner was not cleared."""
    return [
        f"{fid} is completed but run-state owner is '{feature['owner']}'"
        for fid, feature in data["features"].items()
        if feature["state"] == "completed" and feature.get("owner") is not None
    ]


def cmd_init(args: argparse.Namespace) -> int:
    sprint, features = parse_current_sprint()
    if args.sprint != sprint:
        die(f"CURRENT_SPRINT.md says {sprint}, you asked for {args.sprint}")

    exists = state_path(sprint).exists()
    if exists and args.resume:
        # The point of --resume is that an interrupted /run-sprint picks up
        # where it stopped rather than redoing landed work. Adopt the limits
        # and canonical feature metadata from this invocation, while keeping
        # every state, worker fact, and evidence record.
        data = load(sprint)
        data["max_review_passes"] = args.max_review_passes
        data["max_workers"] = args.max_workers
        for f in features:
            feature = data["features"].setdefault(
                f["fid"],
                {
                    "state": "pending",
                    "size": f["size"],
                    "title": f["title"],
                    "owner": f["owner"] if f["owner"] != "-" else None,
                },
            )
            feature["size"] = f["size"]
            feature["title"] = f["title"]
        save(sprint, data)
        print(
            f"resumed {sprint}, phase={data['phase']}, "
            f"{len(data['features'])} features, {len(data['reviews'])} review pass(es)"
        )
        return 0
    if exists and not args.force:
        die(
            f"run state for {sprint} already exists. Pass --resume to continue "
            "it, or --force to throw it away and recreate."
        )

    data = {
        "schema_version": SCHEMA_VERSION,
        "sprint": sprint,
        "phase": "design",
        "max_review_passes": args.max_review_passes,
        "max_workers": args.max_workers,
        "features": {
            f["fid"]: {
                "state": "pending",
                "size": f["size"],
                "title": f["title"],
                "owner": f["owner"] if f["owner"] != "-" else None,
            }
            for f in features
        },
        "reviews": [],
        "verifications": [],
    }
    save(sprint, data)
    print(f"initialised {sprint} with {len(features)} features, phase=design")
    return 0


def cmd_status(args: argparse.Namespace) -> int:
    sprint = args.sprint or parse_current_sprint()[0]
    data = load(sprint)
    print(
        f"{sprint}  phase={data['phase']}  "
        f"max-review-passes={data.get('max_review_passes', MAX_REVIEW_PASSES)}  "
        f"max-workers={data.get('max_workers') or 'unbounded'}"
    )
    counts: dict[str, int] = {}
    for fid, f in sorted(data["features"].items()):
        counts[f["state"]] = counts.get(f["state"], 0) + 1
        owner = f" ({f['owner']})" if f.get("owner") else ""
        wave = f" w{f['wave']}" if f.get("wave") is not None else ""
        print(f"  {fid:<8} {f['size']:<2}{wave} {f['state']}{owner}  {f['title']}")
        if args.workers:
            for field in WORKER_FIELDS:
                if field != "wave" and f.get(field):
                    print(f"      {field:<19} {f[field]}")
    summary = ", ".join(f"{k}={v}" for k, v in sorted(counts.items()))
    print("  " + (summary or "features=0"))
    if data["reviews"]:
        last = data["reviews"][-1]
        print(f"  reviews: {len(data['reviews'])} passes, last blocking={last['blocking']}")
    return 0


def cmd_set_phase(args: argparse.Namespace) -> int:
    if args.phase not in PHASES:
        die(f"unknown phase {args.phase}. One of: {', '.join(PHASES)}")
    data = load(args.sprint)
    if data["phase"] == "closed":
        die(f"{args.sprint} is closed. Phases do not move after that.")
    data["phase"] = args.phase
    save(args.sprint, data)
    print(f"{args.sprint} phase={args.phase}")
    return 0


def cmd_mark_feature(args: argparse.Namespace) -> int:
    if not FID_RE.match(args.fid):
        die(f"{args.fid} is not a valid F-ID")
    if args.state not in FEATURE_STATES:
        die(f"unknown state {args.state}. One of: {', '.join(FEATURE_STATES)}")
    data = load(args.sprint)
    f = data["features"].get(args.fid)
    if f is None:
        die(f"{args.fid} is not in {args.sprint}")
    old = f["state"]
    allowed = FEATURE_TRANSITIONS[old]
    if args.state != old and args.state not in allowed:
        die(
            f"{args.fid}: {old} -> {args.state} is not a legal transition. "
            f"From {old} you may go to: {', '.join(sorted(allowed)) or '(nothing)'}"
        )
    f["state"] = args.state
    if args.owner:
        f["owner"] = args.owner
    if args.clear_owner:
        f["owner"] = None
    for field in WORKER_FIELDS:
        value = getattr(args, field, None)
        if value is not None:
            f[field] = value
    save(args.sprint, data)
    print(f"{args.fid}: {old} -> {args.state}")
    return 0


def cmd_record_review(args: argparse.Namespace) -> int:
    data = load(args.sprint)
    head = git_head()
    bound = data.get("max_review_passes", MAX_REVIEW_PASSES)
    if args.passno > bound and not args.extend:
        die(
            f"pass {args.passno} exceeds the bound of {bound}. "
            "Another pass means the sprint is not ready. Carry work, or pass "
            "--extend and record why in the review file."
        )
    data["reviews"].append(
        {
            "pass": args.passno,
            "blocking": args.blocking,
            "should_fix": args.should_fix,
            "nice_to_have": args.nice_to_have,
            "head": head,
        }
    )
    save(args.sprint, data)
    verdict = "clean" if args.blocking == 0 else f"{args.blocking} blocking"
    print(
        f"{args.sprint} review pass {args.passno} recorded at {head[:12]}: {verdict}"
    )
    return 0


def metadata_reuse_problems(data: dict, head: str, record: dict) -> list[str]:
    """Authenticate a finite delivery-only range against the latest actual full run."""
    full = [(i, item) for i, item in enumerate(data["verifications"])
            if item["scope"] == "full"]
    if not full:
        return ["metadata reuse requires an actual full verification attempt"]
    index, latest = full[-1]
    source = record.get("source_head", "")
    if (record.get("full_attempt") != index or not latest["passed"]
            or latest.get("head") != source):
        return ["metadata reuse must anchor the latest passing full attempt"]
    if not re.fullmatch(r"[0-9a-f]{40}", source) or not re.fullmatch(r"[0-9a-f]{40}", head):
        return ["metadata reuse requires full commit identities"]

    def git(*args: str) -> str:
        return subprocess.run(["git", *args], cwd=REPO, check=True,
                              capture_output=True).stdout.decode("utf-8")

    try:
        if git("status", "--porcelain"):
            return ["metadata reuse requires a clean worktree"]
        git("merge-base", "--is-ancestor", source, head)
        ledgers = {"docs/sprints/" + name + ".md" for name in
                   ("CURRENT_SPRINT", "BACKLOG", "SPRINT_TRACKER", "AS_BUILT")}
        own_reviews = {f".claude/reviews/{data['sprint']}-sprint-review-pass-{review['pass']}.md"
                       for review in data.get("reviews", [])}
        plans = {f".claude/plans/{fid}-design.md" for fid in data["features"]}
        if git("rev-list", "--min-parents=2", f"{source}..{head}"):
            return ["metadata reuse requires a linear delivery-only commit range"]
        for commit in git("rev-list", "--reverse", f"{source}..{head}").splitlines():
            if prose_check.check_text(git("log", "-1", "--format=%B", commit), "commit message"):
                return [f"metadata reuse refuses invalid commit prose: {commit}"]
            parent = git("rev-parse", f"{commit}^").strip()
            paths = git("diff", "--name-only", "--no-renames", "-z", parent, commit).split("\0")
            for path in filter(None, paths):
                # Deletions, symlinks and mode changes cannot be delivery metadata.
                after_entry = git("ls-tree", commit, "--", path).split()
                before_entry = git("ls-tree", parent, "--", path).split()
                if not after_entry or after_entry[0] != "100644":
                    return [f"metadata reuse refuses deletion or non-regular file: {path}"]
                if before_entry and before_entry[0] != "100644":
                    return [f"metadata reuse refuses mode change: {path}"]
                after = git("show", f"{commit}:{path}")
                if prose_check.check_text(after, path):
                    return [f"metadata reuse refuses invalid delivery prose: {path}"]
                if path in own_reviews and not before_entry:
                    continue
                if not before_entry:
                    return [f"metadata reuse refuses new file: {path}"]
                before = git("show", f"{parent}:{path}")
                if path in ledgers:
                    if path.endswith("AS_BUILT.md") and not after.startswith(before):
                        return ["metadata reuse requires append-only AS_BUILT.md"]
                    if path.endswith("CURRENT_SPRINT.md") and not re.search(
                            rf"^# Current Sprint, {re.escape(data['sprint'])}$", after, re.M):
                        return ["metadata reuse cannot change the current sprint identity"]
                    continue
                if path in plans:
                    old_lines, new_lines = before.splitlines(keepends=True), after.splitlines(keepends=True)
                    if len(old_lines) != len(new_lines):
                        return [f"metadata reuse refuses semantic plan edits: {path}"]
                    for old, new in zip(old_lines, new_lines):
                        if old == new or (old.startswith("- [ ] ") and new == old.replace("- [ ] ", "- [x] ", 1)):
                            continue
                        if (re.fullmatch(r"\*\*Status\*\*: (approved|in-progress|reviewed)\n?", old)
                                and new == "**Status**: completed" + ("\n" if old.endswith("\n") else "")):
                            continue
                        return [f"metadata reuse refuses semantic plan edits: {path}"]
                    continue
                return [f"metadata reuse refuses non-delivery change: {path}"]
    except (subprocess.CalledProcessError, UnicodeError) as error:
        return [f"metadata reuse cannot authenticate commit range: {error}"]
    return []


def cmd_record_verification(args: argparse.Namespace) -> int:
    data = load(args.sprint)
    head = git_head()
    record = {"scope": args.scope, "passed": args.passed,
              "harness": args.harness, "head": head}
    source = getattr(args, "reuse_from", None)
    if args.scope == "metadata":
        if not source:
            die("metadata verification requires --reuse-from with the full source SHA")
        full = [i for i, item in enumerate(data["verifications"]) if item["scope"] == "full"]
        record.update(source_head=source, full_attempt=full[-1] if full else None)
        problems = metadata_reuse_problems(data, head, record)
        if problems:
            die("; ".join(problems))
    elif source:
        die("--reuse-from is only valid for metadata verification")
    data["verifications"].append(record)
    save(args.sprint, data)
    print(f"{args.sprint} verification recorded at {head[:12]}: "
          f"{args.scope}, passed={args.passed}")
    return 0


def closure_evidence_problems(data: dict, head: str) -> list[str]:
    """Return stale or missing evidence, with failures superseding prior green."""
    problems: list[str] = []
    if not data["reviews"]:
        problems.append("no sprint-review pass recorded")
    else:
        review = data["reviews"][-1]
        if review["blocking"] != 0:
            problems.append(f"last review pass has {review['blocking']} blocking finding(s)")
        if review.get("head") != head:
            problems.append("latest sprint review covered "
                            f"{review.get('head') or 'an unrecorded HEAD'}, current HEAD is {head}")
    acceptance = [item for item in data["verifications"] if item["scope"] in ("full", "metadata")]
    if not acceptance or not acceptance[-1]["passed"] or acceptance[-1].get("head") != head:
        problems.append(f"no passing `/verify --full` recorded for current HEAD {head}")
    elif acceptance[-1]["scope"] == "metadata":
        problems.extend(metadata_reuse_problems(data, head, acceptance[-1]))
    return problems


def cmd_validate_handoff(args: argparse.Namespace) -> int:
    """Check a worker's `.claude/handoffs/F-XXX-ready.md` before integration.

    This is a shape check, not a trust check. It catches the half-written
    handoff. Whether the claims inside are true is what /integrate-feature
    verifies against the branch.
    """
    path = Path(args.path)
    if not path.exists():
        die(f"{path} not found. The worker never ran `/complete-feature --prepare`.")

    fields = dict(HANDOFF_FIELD_RE.findall(path.read_text(encoding="utf-8")))
    problems: list[str] = []

    for key in HANDOFF_REQUIRED:
        if not fields.get(key):
            problems.append(f"missing or empty field: **{key}**")

    if fields.get("F-ID") and fields["F-ID"] != args.fid:
        problems.append(f"handoff is for {fields['F-ID']}, you asked for {args.fid}")

    microscope = fields.get("Microscope", "")
    if microscope and not ("0 defects" in microscope and "0 smells" in microscope):
        problems.append(
            "Microscope must cite a pass reporting '0 defects, 0 smells'. "
            f"It says: {microscope}"
        )

    verify = fields.get("Verify", "")
    if verify and "fail" in verify.lower():
        problems.append(f"Verify records a failure: {verify}")
    expected_verify = f"/verify --scoped {args.fid}"
    if verify and expected_verify not in verify:
        problems.append(
            f"Verify must record `{expected_verify}, pass`."
        )

    checkpoint = fields.get("Checkpoint", "")
    external = fields.get("External gates", "")
    test_gate = fields.get("Test gate", "")
    pending = any("external ci pending" in value.lower() for value in (verify, test_gate)) or bool(external)
    if checkpoint and checkpoint not in ("external-ci-source-availability", "sprint-final-acceptance"):
        problems.append("unknown handoff checkpoint")
    if pending and checkpoint != "external-ci-source-availability":
        problems.append("pending external CI requires its source-availability checkpoint")
    final = fields.get("Final checks", "")
    final_pending = any("final sprint checks pending" in value.lower() for value in (verify, test_gate)) or bool(final)
    if final_pending and checkpoint != "sprint-final-acceptance":
        problems.append("pending final sprint checks require their final-acceptance checkpoint")
    if checkpoint == "sprint-final-acceptance":
        if not re.fullmatch(r"[A-Za-z0-9/][A-Za-z0-9 ./_,:()=-]{2,400}, pending", final) or final[:-9].strip().lower() in ("checks", "gates", "final checks", "final sprint checks"):
            problems.append("final checkpoint requires named actual final checks, pending")
        if verify != f"{expected_verify}, local checks pass, final sprint checks pending":
            problems.append("final checkpoint must distinguish local checks from pending sprint acceptance")
        if (not test_gate.endswith(", pass") or len(test_gate[:-6].strip()) < 3
                or re.search(r"\b(?:complet(?:e|ed)|pending)\b", test_gate, re.I)):
            problems.append("final checkpoint requires named actual local regression checks, pass")
        if external:
            problems.append("final checkpoint cannot replace the external CI checkpoint")
    if checkpoint == "external-ci-source-availability":
        if not re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9 ./_-]{2,240}, pending", external) or external[:-9].strip().lower() in ("ci", "external ci", "gates"):
            problems.append("external checkpoint requires named actual gates, pending")
        if verify != f"{expected_verify}, local checks pass, external CI pending":
            problems.append("external checkpoint must distinguish local checks from pending scoped acceptance")
        if not test_gate.endswith(", pass, external CI pending") or re.search(r"\bcomplet(?:e|ed)\b", test_gate, re.I):
            problems.append("external checkpoint requires actual local test checks with external CI pending")

    if problems:
        print(f"validate-handoff {path}: REFUSED", file=sys.stderr)
        for p in problems:
            print(f"  - {p}", file=sys.stderr)
        return 1

    print(
        f"validate-handoff {args.fid}: ok, "
        f"{fields['Base'][:12]}..{fields['Head'][:12]} on {fields['Branch']}"
    )
    return 0


def cmd_close_preflight(args: argparse.Namespace) -> int:
    data = load(args.sprint)
    problems: list[str] = []
    head = git_head()

    if data["phase"] == "blocked":
        problems.append("sprint is blocked. Resolve what blocked it and re-run.")

    # /integrate-feature deletes each handoff it consumed. One still sitting
    # here means a worker was prepared and never integrated.
    stale = sorted(p.name for p in HANDOFFS.glob("F-*-ready.md"))
    if stale:
        problems.append("unconsumed worker handoffs: " + ", ".join(stale))

    incomplete = [
        fid
        for fid, f in data["features"].items()
        if f["state"] not in ("completed", "carried")
    ]
    if incomplete:
        problems.append(
            "features neither completed nor carried: " + ", ".join(sorted(incomplete))
        )
    problems.extend(completed_owner_problems(data))

    problems.extend(closure_evidence_problems(data, head))

    # The durable delivery record must agree with the run state. This catches a
    # completion that updated the JSON but missed one of the canonical ledgers.
    statuses = backlog_statuses()
    for fid, f in data["features"].items():
        want = "done" if f["state"] == "completed" else None
        if want and statuses.get(fid) != want:
            problems.append(
                f"{fid} is completed in run state but '{statuses.get(fid)}' in BACKLOG.md"
            )
        if want:
            problems.extend(completed_record_problems(args.sprint, fid))

    if data["reviews"]:
        last_pass = data["reviews"][-1]["pass"]
        review = REVIEWS / f"{args.sprint}-sprint-review-pass-{last_pass}.md"
        if not review.exists():
            problems.append(
                f"latest sprint review file is missing: {review.relative_to(REPO)}"
            )
        elif not re.search(
            r"^\*\*Verdict\*\*:\s*0 blocking,",
            review.read_text(encoding="utf-8"),
            re.MULTILINE,
        ):
            problems.append(
                "latest sprint review file does not report zero blocking: "
                f"{review.relative_to(REPO)}"
            )

    if problems:
        print(f"close-preflight {args.sprint}: REFUSED", file=sys.stderr)
        for p in problems:
            print(f"  - {p}", file=sys.stderr)
        return 1

    print(f"close-preflight {args.sprint}: ok, ready to close")
    return 0


COMPATIBILITY_COUNTS = {"BASE": 24, "XML": 14, "LIFE": 16, "AREA": 46}
COMPATIBILITY_MANIFEST_SHA = "7501e001301a703988526f76c9961b35b345a1c17f8e1d383e9477d5947b22e1"
COMPATIBILITY_BODY_SHA = "2f4348dd55dcd1b8889badd8ca407d9f790cf38327c00ec3ae31dd5b02b3f66e"
COMPATIBILITY_FREEZE_SHA = "d3cf81e655dfb1f7d35745487d46e2ceb8c3c826167439cc970911eec3ac21f7"
COMPATIBILITY_DIVERGENCES = {"W-XML-02", "P-AREA-17"}


def compatibility_artifact(reference: dict) -> bytes:
    """Authenticate retained bytes without executing their contents."""
    path = Path(reference["path"])
    digest = reference["sha256"]
    if not path.is_absolute() or not re.fullmatch(r"[0-9a-f]{64}", digest):
        raise ValueError("compatibility evidence needs an absolute path and SHA-256")
    raw = path.read_bytes()
    if hashlib.sha256(raw).hexdigest() != digest:
        raise ValueError(f"compatibility evidence changed: {path}")
    return raw


def compatibility_obligations(record: dict) -> list[str]:
    """Keep each frozen item and its explicit semicolon-separated clauses."""
    declared = record.get("additional_native_obligations", [])
    if isinstance(declared, str):
        declared = [declared]
    if not isinstance(declared, list) or any(not isinstance(text, str) for text in declared):
        raise ValueError("frozen native obligations must be texts")
    return [clause.strip() for text in declared for clause in text.split(";") if clause.strip()]


def compatibility_observations(facts: list) -> None:
    if not isinstance(facts, list) or not facts:
        raise ValueError("qualification needs concrete typed observations")
    for fact in facts:
        if not isinstance(fact, dict) or type(fact.get("observation")) is not str or (
            not fact["observation"].strip()
            or fact.get("kind") not in ("source-model", "save-reopen", "native", "render", "lifetime", "schema")
            or "expected" not in fact or "actual" not in fact
            or not isinstance(fact.get("evidence"), list) or not fact["evidence"]
        ):
            raise ValueError("qualification needs concrete typed observations")
        for reference in fact["evidence"]:
            compatibility_artifact(reference)


def compatibility_git_source(root: Path) -> dict:
    """Capture actual clean Git identity and every tracked working byte."""
    def git(*args):
        return subprocess.check_output(["git", "--no-optional-locks", *args], cwd=root)
    if git("status", "--porcelain", "--untracked-files=no").strip():
        raise ValueError("compatibility clean Git source has tracked changes")
    entries = [entry.split(b"\t", 1) for entry in git("ls-tree", "-r", "-z", "HEAD").split(b"\0") if entry]
    tracked = set(filter(None, git("ls-files", "-z").decode().split("\0")))
    if tracked != {name.decode() for _, name in entries}:
        raise ValueError("compatibility clean Git tracked paths differ from HEAD")
    object_format = git("rev-parse", "--show-object-format").decode().strip()
    files = {}
    for metadata, encoded_name in entries:
        mode, kind, oid = metadata.decode().split()
        name = encoded_name.decode()
        path = root / name
        if kind != "blob" or mode not in ("100644", "100755") or path.is_symlink() or not path.is_file():
            raise ValueError("compatibility source must contain regular tracked files")
        raw = path.read_bytes()
        git_object = hashlib.new(object_format, b"blob " + str(len(raw)).encode() + b"\0" + raw).hexdigest()
        if git_object != oid or stat.S_IMODE(path.stat().st_mode) != int(mode, 8) & 0o777:
            raise ValueError("compatibility clean Git working bytes or mode differ from HEAD")
        files[name] = {"sha256": hashlib.sha256(raw).hexdigest(), "size": len(raw),
                       "mode": stat.S_IMODE(path.stat().st_mode)}
    return {"head": git("rev-parse", "HEAD").decode().strip(),
            "tree": git("rev-parse", "HEAD^{tree}").decode().strip(),
            "index_sha256": hashlib.sha256(git("ls-files", "--stage", "-z")).hexdigest(),
            "files": files}


def compatibility_process(argv: list[str], cwd: Path, env: dict, timeout: int, log: Path) -> dict:
    """Bound this case session and leave an auditable lease for the outer job."""
    signals = []
    timed_out = False
    child = None
    pending_error = None
    old_handler = signal.getsignal(signal.SIGTERM)
    lease_path = log.with_suffix(".lease.json")
    lease = {"argv_sha256": hashlib.sha256(json.dumps(argv).encode()).hexdigest(), "cleanup": None}
    def interrupted(signum, frame):
        raise InterruptedError("compatibility case supervisor interrupted")
    signal.signal(signal.SIGTERM, interrupted)
    with log.open("xb") as output:
        try:
            # Do not expose the small spawn-to-lease interval to SIGTERM.
            old_mask = signal.pthread_sigmask(signal.SIG_BLOCK, {signal.SIGTERM})
            try:
                child = subprocess.Popen(argv, cwd=cwd, env=env, stdout=output, stderr=subprocess.STDOUT,
                                         start_new_session=True)
                identity = subprocess.check_output(["/bin/ps", "-p", str(child.pid), "-o", "uid=,pgid=,lstart="],
                                                   env=dict(os.environ, LC_ALL="C"), timeout=5).decode().strip()
                lease.update(pid=child.pid, pgid=child.pid, process_identity=identity)
                fd = os.open(lease_path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
                with os.fdopen(fd, "w") as stream:
                    json.dump(lease, stream)
            finally:
                signal.pthread_sigmask(signal.SIG_SETMASK, old_mask)
            child.wait(timeout=timeout)
        except BaseException as error:
            timed_out = isinstance(error, subprocess.TimeoutExpired)
            pending_error = None if timed_out else error
        finally:
            signal.signal(signal.SIGTERM, signal.SIG_IGN)
            if child is not None:
                for sig in (signal.SIGTERM, signal.SIGKILL):
                    try:
                        os.killpg(child.pid, sig)
                        signals.append(sig.name)
                    except ProcessLookupError:
                        break
                    try:
                        child.wait(timeout=5)
                    except subprocess.TimeoutExpired:
                        pass
                gone = False
                for _ in range(50):
                    try:
                        os.killpg(child.pid, 0)
                    except ProcessLookupError:
                        gone = True
                        break
                    time.sleep(0.1)
                result = {"exit_code": child.returncode, "timed_out": timed_out,
                          "cleanup": {"groupGone": gone, "leaderReaped": child.returncode is not None,
                                      "leaseReleaseSafe": gone and child.returncode is not None, "signals": signals}}
                lease.update(cleanup=result["cleanup"], exit_code=child.returncode)
                lease_path.write_text(json.dumps(lease, indent=2) + "\n")
            signal.signal(signal.SIGTERM, old_handler)
    if pending_error is not None:
        raise pending_error
    if child is None:
        raise ValueError("compatibility case process did not start")
    return result


def compatibility_execute(config: dict) -> dict:
    """Execute the original corpus once, with a separate approved input overlay."""
    if config.get("status") != "admitted":
        raise ValueError("compatibility execution configuration must be admitted")
    expected_identity = config["github_identity"]
    stable_identity = {"GITHUB_REPOSITORY", "GITHUB_REPOSITORY_ID", "GITHUB_ACTOR", "GITHUB_TRIGGERING_ACTOR",
                       "GITHUB_EVENT_NAME", "GITHUB_REF", "GITHUB_SHA", "GITHUB_WORKFLOW_REF",
                       "GITHUB_WORKFLOW_SHA", "GITHUB_JOB"}
    dynamic_identity = {"GITHUB_RUN_ID", "GITHUB_RUN_ATTEMPT"}
    identity = {key: os.environ.get(key) for key in stable_identity | dynamic_identity}
    if set(expected_identity) != stable_identity or any(identity[key] != value for key, value in expected_identity.items()) or (
        identity["GITHUB_REPOSITORY"] != "tensorbee/rdocx" or identity["GITHUB_ACTOR"] != "mantissaman"
        or identity["GITHUB_TRIGGERING_ACTOR"] != "mantissaman" or identity["GITHUB_EVENT_NAME"] != "workflow_dispatch"
        or identity["GITHUB_JOB"] != "owner-compatibility" or identity["GITHUB_SHA"] != config["head"]
        or identity["GITHUB_WORKFLOW_SHA"] != config["head"]
        or not str(identity["GITHUB_REF"]).startswith("refs/heads/")
        or identity["GITHUB_WORKFLOW_REF"] != f"tensorbee/rdocx/.github/workflows/ci.yml@{identity['GITHUB_REF']}"
        or any(not re.fullmatch(r"[1-9][0-9]*", str(identity[key])) for key in dynamic_identity)
    ):
        raise ValueError("compatibility execution GitHub identity is not admitted")
    root = Path(config["source_root"]).resolve()
    before = compatibility_git_source(root)
    for key in ("head", "tree", "index_sha256"):
        if before[key] != config[key]:
            raise ValueError(f"compatibility execution source changed: {key}")
    if before["files"] != json.loads(compatibility_artifact(config["source_manifest"])):
        raise ValueError("compatibility execution tracked source changed")
    build = json.loads(compatibility_artifact(config["build_provenance"]))
    if build.get("status") != "admitted" or build["current_source_manifest_sha256"] != config["source_manifest"]["sha256"]:
        raise ValueError("compatibility build provenance is not admitted for this source")
    capture = json.loads(compatibility_artifact(build["build_source_capture"]))["files"]
    def provider(name):
        return name.startswith(("crates/", ".cargo/")) or name in ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "build.rs")
    current = {name: pin for name, pin in before["files"].items() if provider(name)}
    built = {name: {key: pin[key] for key in ("sha256", "size", "mode")} for name, pin in capture.items() if provider(name)}
    if not current or current != built or set(build["products"]) != {"rdocx", "rpptx"}:
        raise ValueError("compatibility compiled provider graph differs from the actual build")
    for reference in build["products"].values():
        compatibility_artifact(reference)
    probes_sha = hashlib.sha256(json.dumps(config["installed_probes"], sort_keys=True).encode()).hexdigest()
    if build["installed_probes_sha256"] != probes_sha:
        raise ValueError("compatibility installed products do not match admitted build provenance")
    manifest_raw = compatibility_artifact(config["manifest"])
    body_raw = compatibility_artifact(config["body_manifest"])
    if hashlib.sha256(manifest_raw).hexdigest() != COMPATIBILITY_MANIFEST_SHA or (
        hashlib.sha256(body_raw).hexdigest() != COMPATIBILITY_BODY_SHA
    ):
        raise ValueError("compatibility execution requires the original frozen manifests")
    manifest = json.loads(manifest_raw)
    records = manifest["records"]
    ids = {row["id"] for row in records}
    if len(records) != 100 or len(ids) != 100 or any(
        sum(row["area"] == area for row in records) != count for area, count in COMPATIBILITY_COUNTS.items()
    ) or sum(row["family"] == "W" for row in records) != 49 or sum(row["family"] == "P" for row in records) != 51:
        raise ValueError("compatibility execution allocation must remain the frozen 100")
    frozen = Path(config["manifest"]["path"]).parent
    support = frozen / "support.py"
    if hashlib.sha256(support.read_bytes()).hexdigest() != manifest["support_sha256"]:
        raise ValueError("compatibility frozen support changed")
    overlay = json.loads(compatibility_artifact(config["runtime_overlay"]))
    changed = {"fixtures/native-notes.pptx", "fixtures/transition-sound.pptx"}
    if overlay.get("status") != "admitted" or set(overlay["replacements"]) != changed:
        raise ValueError("compatibility runtime overlay must admit exactly the two revised native inputs")
    for ref in overlay["native_producer_evidence"]:
        compatibility_artifact(ref)
    if not overlay["native_producer_evidence"]:
        raise ValueError("compatibility overlay requires actual native producer evidence")
    closure_raw = compatibility_artifact(config["frozen_closure"])
    if hashlib.sha256(closure_raw).hexdigest() != COMPATIBILITY_FREEZE_SHA:
        raise ValueError("compatibility frozen executable closure changed")
    closure = json.loads(closure_raw)["files"]
    closure_by_path = {row["path"]: row for row in closure}
    inputs = {"support.py": support.read_bytes()}
    for row in closure:
        name = row["path"]
        if not name.startswith("fixtures/"):
            continue
        if name in changed:
            revision = overlay["replacements"][name]
            if revision["original_sha256"] != row["sha256"]:
                raise ValueError("compatibility overlay lost original native input identity")
            raw = compatibility_artifact(revision["replacement"])
        else:
            raw = compatibility_artifact({"path": str(frozen / name), "sha256": row["sha256"]})
        inputs[name] = raw
    if not changed.issubset(inputs):
        raise ValueError("compatibility frozen executable fixture closure is incomplete")
    for row in records:
        name = row["script_path"]
        if name != f"scripts/{row['id']}.py" or name not in closure_by_path or closure_by_path[name]["sha256"] != row["script_sha256"]:
            raise ValueError("compatibility script identity changed")
        inputs[name] = compatibility_artifact({"path": str(frozen / name), "sha256": row["script_sha256"]})
    guards = config["runtime_guards"]
    if not guards:
        raise ValueError("compatibility execution requires installed products and external tool guards")
    def authenticate():
        for reference in (config["manifest"], config["body_manifest"], config["frozen_closure"], config["runtime_overlay"]):
            compatibility_artifact(reference)
        if support.read_bytes() != inputs["support.py"]:
            raise ValueError("compatibility original frozen support changed")
        for row in records:
            compatibility_artifact({"path": str(frozen / row["script_path"]), "sha256": row["script_sha256"]})
        for name in inputs:
            if not name.startswith("fixtures/"):
                continue
            reference = overlay["replacements"][name]["replacement"] if name in changed else {
                "path": str(frozen / name), "sha256": closure_by_path[name]["sha256"]}
            compatibility_artifact(reference)
        for reference in overlay["native_producer_evidence"]:
            compatibility_artifact(reference)
        for path, digest in guards.items():
            compatibility_artifact({"path": path, "sha256": digest})
        if compatibility_git_source(root) != before:
            raise ValueError("compatibility source changed during execution")
    if set(config["interpreters"]) != {"W", "P"} or set(config["installed_probes"]) != {"W", "P"}:
        raise ValueError("compatibility execution requires both installed products")
    probe = ("import hashlib,importlib,json,pathlib,sys;"
             "from importlib.metadata import distribution;"
             "m=importlib.import_module(sys.argv[1]);d=distribution(sys.argv[1]);"
             "print(json.dumps({'module':str(pathlib.Path(m.__file__).resolve()),'version':d.version,"
             "'files':{str(pathlib.Path(d.locate_file(f)).resolve()):hashlib.sha256(pathlib.Path(d.locate_file(f)).read_bytes()).hexdigest() "
             "for f in d.files if pathlib.Path(d.locate_file(f)).is_file() and not str(f).endswith(('.pyc','.pyo'))}},sort_keys=True))")
    def installed():
        for family, product in (("W", "rdocx"), ("P", "rpptx")):
            python = config["interpreters"][family]
            if python not in guards:
                raise ValueError("compatibility interpreter is unpinned")
            result = json.loads(subprocess.check_output([python, "-I", "-B", "-c", probe, product], timeout=30))
            if not result["files"] or result["module"] not in result["files"] or result != config["installed_probes"][family] or any(guards.get(path) != digest for path, digest in result["files"].items()):
                raise ValueError("compatibility installed product identity changed")
    authenticate()
    installed()
    output = Path(config["output"]).resolve()
    if output.exists() or output == root or output.is_relative_to(root):
        raise ValueError("compatibility execution needs a fresh external output directory")
    timeout = config["case_timeout_seconds"]
    if type(timeout) is not int or not 1 <= timeout <= 3600:
        raise ValueError("compatibility case timeout must be finite")
    output.mkdir(parents=True)
    execution = output / "frozen-execution"
    execution.mkdir()
    for name, raw in inputs.items():
        target = execution / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(raw)
    def artifact(path):
        return {"path": str(path), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}
    attempts = []
    error = None
    try:
        for row in records:
            authenticate()
            work = output / "cases" / row["id"]
            work.mkdir(parents=True)
            script = execution / row["script_path"]
            env = {key: value for key, value in os.environ.items() if key not in (
                "PYTHONPATH", "PYTHONHOME", "PYTHONSTARTUP", "PYTEST_ADDOPTS", "DYLD_INSERT_LIBRARIES", "BASH_ENV", "ENV")}
            env.update(config["environment"])
            # Preserve the frozen direct-script assertion traceback. The sole
            # import-path override is this authenticated support directory.
            env.update(PYTHONPATH=str(execution), PYTHONNOUSERSITE="1")
            result = compatibility_process([config["interpreters"][row["family"]], "-s", "-B", str(script)],
                                           work, env, timeout, work / "process.log")
            attempts.append(dict(id=row["id"], family=row["family"], area=row["area"], attempt_number=1,
                                 script_sha256=row["script_sha256"], **result, evidence=[artifact(work / "process.log")]))
            (output / "attempts.json").write_text(json.dumps(attempts, indent=2) + "\n")
            if not result["cleanup"]["leaseReleaseSafe"]:
                raise ValueError("compatibility process cleanup is unsafe")
        authenticate()
        installed()
        for name, raw in inputs.items():
            if (execution / name).read_bytes() != raw:
                raise ValueError("compatibility execution altered a frozen body or runtime input")
    except BaseException as failure:
        error = f"{type(failure).__name__}: {failure}"
        raise
    finally:
        receipt = {"source_kind": "git", "dirty": False, "head": before["head"], "tree": before["tree"],
                   "index_sha256": before["index_sha256"], "attempted": len(attempts), "error": error,
                   "assertions_exit_zero": sum(row["exit_code"] == 0 and not row["timed_out"] for row in attempts),
                   "full_acceptance_percent": None, "qualification_status": "pending",
                   "runtime_overlay": config["runtime_overlay"], "historical_score_relabelled": False}
        receipt["github_identity"] = identity
        receipt["build_provenance"] = config["build_provenance"]
        receipt["missing_non_runtime_history"] = [row for row in closure if not row["path"].startswith(("scripts/", "fixtures/"))
                                                   and not (frozen / row["path"]).is_file()]
        (output / "execution-receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
    return receipt


def cmd_compatibility_execute(args: argparse.Namespace) -> int:
    """Keep execution and subsequent independently admitted reporting in one job."""
    try:
        if not 0 <= args.qualification_wait_seconds <= 1800:
            raise ValueError("compatibility qualification wait must be finite")
        config_raw = compatibility_artifact({"path": str(Path(args.config).resolve()), "sha256": args.config_sha})
        config = json.loads(config_raw)
        receipt = compatibility_execute(config)
        output = Path(config["output"])
        deadline = time.monotonic() + args.qualification_wait_seconds
        packet_path = output / "owner-admitted-report-packet.json"
        while not packet_path.exists() and time.monotonic() < deadline:
            time.sleep(1)
        if not packet_path.exists():
            raise ValueError("current CI attempts still await independent file/native qualification")
        if packet_path.stat().st_size > 10_000_000:
            raise ValueError("compatibility qualification packet exceeds the bound")
        packet = json.loads(packet_path.read_text())
        attempts = output / "attempts.json"
        if packet["attempts"] != {"path": str(attempts), "sha256": hashlib.sha256(attempts.read_bytes()).hexdigest()}:
            raise ValueError("compatibility reporting must use this actual CI attempt")
        provenance = json.loads(compatibility_artifact(packet["provenance"]))
        if provenance.get("github_identity") != receipt["github_identity"]:
            raise ValueError("compatibility reporting must bind the actual CI run and attempt")
        if provenance["source_manifest"] != config["source_manifest"] or any(
            provenance[key] != receipt[key] for key in ("head", "tree", "index_sha256", "source_kind", "dirty")
        ):
            raise ValueError("compatibility reporting source differs from this actual CI execution")
        result = compatibility_report(packet)
        (output / "qualified-report.json").write_text(json.dumps(result, indent=2) + "\n")
        # Never upload raw captured output, Office resources or saved documents.
        # The job summary contains only this fixed count/identity projection.
        summary = {key: result[key] for key in ("denominator", "by_area", "documentation_subset", "total",
                                               "full_acceptance_percent", "threshold_met")}
        summary["github_identity"] = receipt["github_identity"]
        raw = json.dumps(summary, indent=2)
        if len(raw.encode()) > 16384:
            raise ValueError("compatibility report summary exceeds the upload bound")
        if any(not re.fullmatch(r"[WP]-(BASE|XML|LIFE|AREA)-[0-9]{2}", case_id)
               for row in [summary["total"], summary["documentation_subset"], *summary["by_area"].values()]
               for case_id in row["unexpected_failure_ids"]):
            raise ValueError("compatibility report contains an unreviewed output identity")
        with Path(os.environ["GITHUB_STEP_SUMMARY"]).open("a") as stream:
            stream.write("## Current unchanged compatibility corpus\n\n```json\n" + raw + "\n```\n")
        print(json.dumps(summary, sort_keys=True))
        return 0 if result["threshold_met"] and not result["total"]["unexpected_failure_ids"] else 1
    except (KeyError, TypeError, ValueError, OSError, subprocess.SubprocessError) as error:
        print(f"compatibility-execute: REFUSED: {type(error).__name__}", file=sys.stderr)
        return 1


def compatibility_named_non_successes(packet: dict, source_sha: str, attempts_raw: bytes, by_id: dict) -> set[str]:
    """Admit only reviewed, candidate-specific frozen assertion conflicts."""
    if packet.get("named_non_successes") is None and packet.get("non_success_admission") is None:
        return set()
    raw = compatibility_artifact(packet["named_non_successes"])
    facts = json.loads(raw)
    admission = json.loads(compatibility_artifact(packet["non_success_admission"]))
    attempts_sha = hashlib.sha256(attempts_raw).hexdigest()
    if admission.get("status") != "admitted" or admission.get("disposition_sha256") != hashlib.sha256(raw).hexdigest() or any(
        value.get("source_manifest_sha256") != source_sha or value.get("attempts_sha256") != attempts_sha
        for value in (facts, admission)
    ):
        raise ValueError("named non-success admission does not bind these facts/source/attempts")
    assertions = {
        "W-AREA-19": (18, "assert any('Gelasio' in x for x in names)"),
        "P-AREA-02": (40, "assert len(updates)==1"),
        "P-AREA-14": (17, "assert nv.get('descr')=='Description' and nv.get('title')=='Title'"),
    }
    rows = facts["cases"]
    if not isinstance(rows, list) or not rows or any(not isinstance(row, dict) for row in rows):
        raise ValueError("named non-successes require explicit reviewed case facts")
    declared = set()
    for row in rows:
        case_id = row["id"]
        if case_id not in assertions or case_id in declared:
            raise ValueError("named non-success identity is unreviewed or duplicated")
        attempt = by_id[case_id]
        cleanup = attempt.get("cleanup", {})
        if attempt["exit_code"] != 1 or attempt["timed_out"] or any(
            cleanup.get(field) is not True for field in ("groupGone", "leaderReaped", "leaseReleaseSafe")
        ) or cleanup.get("signals") != []:
            raise ValueError("named non-success needs an actual safely completed assertion failure")
        reference = row["failure_evidence"]
        if reference not in attempt["evidence"]:
            raise ValueError("named non-success failure log is not this original attempt's evidence")
        text = compatibility_artifact(reference).decode()
        frames = re.findall(r'File "([^"\n]+)", line (\d+), in <module>', text)
        line, assertion = assertions[case_id]
        if len(frames) != 1 or not frames[0][0].endswith(f"/scripts/{case_id}.py") or frames[0][1] != str(line) or (
            not text.startswith("Traceback (most recent call last):\n")
            or f"\n    {assertion}\n" not in text or not text.endswith("\nAssertionError\n")
        ):
            raise ValueError("named non-success did not reach its reviewed frozen assertion")
        for field, kind in (("source_model", "source-model"), ("save_reopen", "save-reopen")):
            compatibility_observations(row[field])
            if any(fact["kind"] != kind for fact in row[field]):
                raise ValueError("named non-success companion observation has the wrong scope")
        declared.add(case_id)
    return declared


def compatibility_report(packet: dict) -> dict:
    """Report first attempts. Qualification is separate from script exit status."""
    manifest_raw = compatibility_artifact(packet["manifest"])
    body_raw = compatibility_artifact(packet["body_manifest"])
    if hashlib.sha256(manifest_raw).hexdigest() != COMPATIBILITY_MANIFEST_SHA:
        raise ValueError("compatibility manifest is not the frozen S91 manifest")
    if hashlib.sha256(body_raw).hexdigest() != COMPATIBILITY_BODY_SHA:
        raise ValueError("compatibility bodies are not the frozen S91 bodies")
    manifest = json.loads(manifest_raw)
    records = manifest["records"]
    ids = {row["id"] for row in records}
    if len(records) != 100 or len(ids) != 100:
        raise ValueError("compatibility denominator must be exactly 100 distinct cases")
    for area, count in COMPATIBILITY_COUNTS.items():
        if sum(row["area"] == area for row in records) != count:
            raise ValueError(f"compatibility allocation changed: {area}")
    if sum(row["family"] == "W" for row in records) != 49 or sum(
        row["family"] == "P" for row in records
    ) != 51:
        raise ValueError("compatibility family allocation must remain 49 Word and 51 PPT")
    attempts_raw = compatibility_artifact(packet["attempts"])
    attempts = json.loads(attempts_raw)
    if len(attempts) != 100 or {row["id"] for row in attempts} != ids:
        raise ValueError("compatibility attempts contain duplicate or missing case identities")
    by_id = {row["id"]: row for row in attempts}
    for record in records:
        attempt = by_id[record["id"]]
        if attempt["attempt_number"] != 1 or type(attempt["attempt_number"]) is not int:
            raise ValueError("compatibility credit is limited to each immutable first attempt")
        if attempt["script_sha256"] != record["script_sha256"]:
            raise ValueError(f"compatibility script changed: {record['id']}")
        if attempt["family"] != record["family"] or attempt["area"] != record["area"]:
            raise ValueError(f"compatibility attempt identity changed: {record['id']}")
        if type(attempt["timed_out"]) is not bool or (
            attempt["exit_code"] is not None and type(attempt["exit_code"]) is not int
        ):
            raise ValueError("compatibility exit status must be an actual integer or null")
        for reference in attempt["evidence"]:
            compatibility_artifact(reference)
        if not attempt["evidence"]:
            raise ValueError(f"compatibility attempt has no retained evidence: {record['id']}")
    provenance = json.loads(compatibility_artifact(packet["provenance"]))
    # Historical dirty captures and actual clean Git executions are distinct.
    if provenance["source_kind"] == "working-copy":
        if provenance["dirty"] is not True:
            raise ValueError("compatibility working-copy source must identify the actual dirty candidate")
    elif provenance["source_kind"] == "git":
        if provenance["dirty"] is not False:
            raise ValueError("compatibility clean Git source cannot claim dirty bytes")
        current = compatibility_git_source(Path(provenance["source_root"]))
        for key in ("head", "tree", "index_sha256"):
            if current[key] != provenance[key]:
                raise ValueError(f"compatibility clean Git identity changed: {key}")
        if current["files"] != json.loads(compatibility_artifact(provenance["source_manifest"])):
            raise ValueError("compatibility clean Git tracked bytes changed")
    else:
        raise ValueError("compatibility source kind must be working-copy or git")
    for key, size in (("base", 40), ("head", 40), ("index_sha256", 64)):
        if not re.fullmatch(rf"[0-9a-f]{{{size}}}", provenance[key]):
            raise ValueError(f"compatibility source identity is invalid: {key}")
    for reference in provenance["evidence"]:
        compatibility_artifact(reference)
    source_raw = compatibility_artifact(provenance["source_manifest"])
    source_sha = hashlib.sha256(source_raw).hexdigest()
    if not provenance["evidence"] or not json.loads(source_raw):
        raise ValueError("compatibility source/build/install evidence is missing")
    dependencies = packet["dependencies"]
    required = {"F-X211a", "F-X211b", "F-X211d", "F-X215", "F-X217", "F-X218", "F-X220"}
    required.update(f"F-X{number}" for number in range(200, 211))
    if set(dependencies) != required:
        raise ValueError("compatibility provider dependency identities are incomplete")
    for reference in dependencies.values():
        compatibility_artifact(reference)
    divergences = COMPATIBILITY_DIVERGENCES | compatibility_named_non_successes(packet, source_sha, attempts_raw, by_id)

    # Until a separate reviewed fact packet is admitted, no correct-file score
    # follows from assertion exits, hash gates or package inventories.
    qualified = set()
    full_metric = None
    declared = {row["id"]: compatibility_obligations(row) for row in records if row.get("additional_native_obligations")}
    native_inventory = [
        {"id": case_id, "obligation": text, "obligation_sha256": hashlib.sha256(text.encode()).hexdigest(), "status": "pending"}
        for case_id, texts in declared.items() for text in texts
    ]
    if packet.get("qualification") is not None:
        facts_raw = compatibility_artifact(packet["qualification"])
        facts = json.loads(facts_raw)
        admission = json.loads(compatibility_artifact(packet["qualification_admission"]))
        if admission.get("status") != "admitted":
            raise ValueError("compatibility qualification is not admitted")
        if admission["qualification_sha256"] != hashlib.sha256(facts_raw).hexdigest() or (
            admission["source_manifest_sha256"] != source_sha
            or admission["attempts_sha256"] != hashlib.sha256(attempts_raw).hexdigest()
        ):
            raise ValueError("compatibility admission does not bind these facts/source/attempts")
        if facts["source_manifest_sha256"] != source_sha or (
            facts["attempts_sha256"] != admission["attempts_sha256"]
        ):
            raise ValueError("compatibility facts belong to a different candidate")
        fact_rows = facts["cases"]
        if len(fact_rows) != 100 or {row["id"] for row in fact_rows} != ids:
            raise ValueError("compatibility facts must evaluate all 100 unchanged cases")
        for row in fact_rows:
            if row["status"] not in ("qualified", "unqualified", "pending"):
                raise ValueError("compatibility fact status is invalid")
            for reference in row["evidence"]:
                compatibility_artifact(reference)
            if row["status"] == "qualified":
                proof_fields = ("source_model", "save_reopen")
                if not row["evidence"]:
                    raise ValueError("correct-file credit needs actual source/model and save/reopen facts")
                for field in proof_fields:
                    compatibility_observations(row[field])
                attempt = by_id[row["id"]]
                if attempt["exit_code"] != 0 or attempt["timed_out"] or row["id"] in divergences:
                    raise ValueError("a failed or divergent first attempt cannot earn correct-file credit")
                qualified.add(row["id"])
        # Admission must enumerate the defining owner, safety and native/render
        # authorities. Their concrete proofs remain subject to independent review.
        hard_gates = facts["hard_gates"]
        required_gates = {
            row["id"] for row in records
            if row["area"] in ("XML", "LIFE") or row.get("additional_native_obligations")
        }
        required_gates.add("P-AREA-10")
        if not required_gates.issubset(hard_gates):
            raise ValueError("all current-owner XML, lifetime and declared native/render hard gates are required")
        for gate in hard_gates.values():
            if gate["status"] not in ("qualified", "unqualified", "pending"):
                raise ValueError("compatibility hard gate status is invalid")
            for reference in gate["evidence"]:
                compatibility_artifact(reference)
            if gate["status"] == "qualified":
                if not gate["evidence"]:
                    raise ValueError("qualified hard gates require retained actual observations/evidence")
                compatibility_observations(gate["observations"])
        native_facts = facts["native_obligations"]
        if set(native_facts) != set(declared):
            raise ValueError("qualification must bind every frozen native obligation")
        native_ready = True
        for case_id, texts in declared.items():
            proofs = native_facts[case_id]
            if not isinstance(proofs, list) or len(proofs) != len(texts):
                raise ValueError(f"native obligation proofs are missing: {case_id}")
            for text, proof in zip(texts, proofs):
                digest = hashlib.sha256(text.encode()).hexdigest()
                if proof["obligation"] != text or proof["obligation_sha256"] != digest:
                    raise ValueError(f"native obligation identity changed: {case_id}")
                if proof["status"] not in ("qualified", "unqualified", "pending"):
                    raise ValueError("native obligation status is invalid")
                if proof["status"] == "qualified":
                    compatibility_observations(proof["observations"])
                else:
                    native_ready = False
        native_inventory = [dict(id=case_id, obligation=proof["obligation"],
                                 obligation_sha256=proof["obligation_sha256"], status=proof["status"])
                            for case_id, proofs in native_facts.items() for proof in proofs]
        complete = all(row["status"] != "pending" for row in fact_rows)
        gates_ready = all(gate["status"] == "qualified" for gate in hard_gates.values())
        if complete and gates_ready and native_ready and admission["status"] == "admitted":
            full_metric = len(qualified)
    def counts(selected: list[dict]) -> dict:
        selected_ids = {row["id"] for row in selected}
        exits = {row["id"] for row in selected if row["exit_code"] == 0 and not row["timed_out"]}
        return {"attempted": len(selected), "executed": sum(row["exit_code"] is not None for row in selected),
                "assertions_exit_zero": len(exits), "correct_file": len(selected_ids & qualified),
                "deliberate_divergence": len(selected_ids & divergences),
                "unexpected_failure_ids": sorted(selected_ids - exits - divergences)}
    documentation = {f"W-BASE-{i:02}" for i in range(1, 18)} | {f"P-BASE-{i:02}" for i in range(1, 8)}
    return {"denominator": 100, "body_manifest_sha256": COMPATIBILITY_BODY_SHA,
            "source_manifest_sha256": source_sha, "provenance": provenance,
            "by_area": {area: counts([row for row in attempts if row["area"] == area]) for area in COMPATIBILITY_COUNTS},
            "documentation_subset": counts([row for row in attempts if row["id"] in documentation]),
            "native_obligations": native_inventory,
            "total": counts(attempts), "full_acceptance_percent": full_metric,
            "threshold_met": full_metric is not None and full_metric >= 90}


def cmd_compatibility_report(args: argparse.Namespace) -> int:
    try:
        result = compatibility_report(json.loads(Path(args.packet).read_text()))
    except (KeyError, TypeError, ValueError, OSError) as error:
        print(f"compatibility-report: REFUSED: {error}", file=sys.stderr)
        return 1
    print(json.dumps(result, indent=2, sort_keys=True))
    if args.check and (
        not result["threshold_met"] or result["total"]["unexpected_failure_ids"]
    ):
        print("compatibility-report: qualification incomplete or acceptance failed", file=sys.stderr)
        return 1
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)

    p = sub.add_parser("init"); p.add_argument("sprint"); p.add_argument("--force", action="store_true"); p.add_argument("--resume", action="store_true"); p.add_argument("--max-review-passes", type=int, default=MAX_REVIEW_PASSES); p.add_argument("--max-workers", type=int, default=None); p.set_defaults(fn=cmd_init)
    p = sub.add_parser("status"); p.add_argument("sprint", nargs="?"); p.add_argument("--workers", action="store_true", help="show branch, worktree and handoff per feature"); p.set_defaults(fn=cmd_status)
    p = sub.add_parser("set-phase"); p.add_argument("sprint"); p.add_argument("phase"); p.set_defaults(fn=cmd_set_phase)
    p = sub.add_parser("mark-feature"); p.add_argument("sprint"); p.add_argument("fid"); p.add_argument("state"); p.add_argument("--owner"); p.add_argument("--clear-owner", action="store_true"); p.add_argument("--wave", type=int); p.add_argument("--branch"); p.add_argument("--worktree"); p.add_argument("--base"); p.add_argument("--head"); p.add_argument("--handoff"); p.add_argument("--integration-commit"); p.set_defaults(fn=cmd_mark_feature)
    p = sub.add_parser("record-review"); p.add_argument("sprint"); p.add_argument("passno", type=int); p.add_argument("--blocking", type=int, required=True); p.add_argument("--should-fix", type=int, default=0); p.add_argument("--nice-to-have", type=int, default=0); p.add_argument("--extend", action="store_true"); p.set_defaults(fn=cmd_record_review)
    p = sub.add_parser("record-verification"); p.add_argument("sprint"); p.add_argument("--scope", choices=["fast", "scoped", "full", "metadata"], required=True); p.add_argument("--reuse-from"); p.add_argument("--passed", action="store_true"); p.add_argument("--harness", default="unchecked"); p.set_defaults(fn=cmd_record_verification)
    p = sub.add_parser("validate-handoff"); p.add_argument("path"); p.add_argument("--fid", required=True); p.set_defaults(fn=cmd_validate_handoff)
    p = sub.add_parser("close-preflight"); p.add_argument("sprint"); p.set_defaults(fn=cmd_close_preflight)
    p = sub.add_parser("release-notes"); p.add_argument("tag"); mode = p.add_mutually_exclusive_group(required=True); mode.add_argument("--check", action="store_true"); mode.add_argument("--render", action="store_true"); p.set_defaults(fn=cmd_release_notes)
    p = sub.add_parser("unified-release-family"); p.add_argument("tag"); p.set_defaults(fn=cmd_unified_release_family)
    p = sub.add_parser("published-unified-release"); p.add_argument("tag"); p.add_argument("directory"); p.set_defaults(fn=cmd_published_unified_release)
    p = sub.add_parser("python-release-artifacts"); p.add_argument("tag"); p.add_argument("directory"); p.set_defaults(fn=cmd_python_release_artifacts)
    p = sub.add_parser("compatibility-report"); p.add_argument("packet"); mode = p.add_mutually_exclusive_group(required=True); mode.add_argument("--check", action="store_true"); mode.add_argument("--render", action="store_true"); p.set_defaults(fn=cmd_compatibility_report)
    p = sub.add_parser("compatibility-execute")
    p.add_argument("config")
    p.add_argument("--config-sha", required=True)
    p.add_argument("--qualification-wait-seconds", type=int, default=1800)
    p.set_defaults(fn=cmd_compatibility_execute)

    args = ap.parse_args()
    if getattr(args, "fid", None) and not FID_RE.match(args.fid):
        die(f"{args.fid} is not a valid F-ID")
    if hasattr(args, "sprint") and args.sprint and not SPRINT_ID_RE.match(args.sprint):
        die(f"{args.sprint} is not a valid sprint id, expected SNN")
    return args.fn(args)


if __name__ == "__main__":
    sys.exit(main())
