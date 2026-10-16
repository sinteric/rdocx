#!/usr/bin/env python3
"""Fork-only wrapper for scripts/readme_doctests.py.

The README archive rows and readme_doctests.py follow tensorbee/rdocx
upstream verbatim so upstream syncs never conflict on them. The fork's
extra commits change package contents, so upstream's exact archive-size
check cannot match here. This wrapper runs every upstream check unchanged
except the archive-size comparison, which keeps only the member-count and
10 MiB crates.io limits and reports the size drift as a notice.
"""

from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import readme_doctests as upstream  # noqa: E402


def fork_validate_archive_measurement(package: str, archive: Path) -> bool:
    observed = upstream.archive_measurement(archive)
    recorded = upstream.recorded_archive_measurement(
        upstream.MEASUREMENT_ROWS[f"archive:{package}"]
    )
    if recorded is None:
        print(
            f"README doctest error: malformed archive measurement for {package}",
            file=sys.stderr,
        )
        return False
    compressed, _members, count = observed
    valid = count == recorded[2] and compressed < 10 * 1024 * 1024
    if not valid:
        print(
            f"README doctest error: {package} archive breaks fork limits, "
            f"recorded={recorded!r}, observed={observed!r}",
            file=sys.stderr,
        )
    elif observed != recorded:
        print(
            f"fork notice: {package} archive differs from upstream record, "
            f"recorded={recorded!r}, observed={observed!r}",
            file=sys.stderr,
        )
    return valid


upstream.validate_archive_measurement = fork_validate_archive_measurement

if __name__ == "__main__":
    raise SystemExit(upstream.main())
