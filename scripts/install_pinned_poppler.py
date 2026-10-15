#!/usr/bin/env python3
"""Build the exact Poppler command-line oracle used by CI."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import stat
import subprocess
import tarfile
import tempfile
import urllib.request


POPLER_VERSION = "26.01.0"
POPLER_SHA256 = "1cb944a4b88847f5fb6551683bc799db59f04990f5d8be07aba2acbf38601089"
POPLER_URL = f"https://poppler.freedesktop.org/poppler-{POPLER_VERSION}.tar.xz"
TOOLS = ("pdftoppm", "pdfinfo", "pdftotext")
MAX_DOWNLOAD_BYTES = 8 * 1024 * 1024
MAX_ARCHIVE_MEMBERS = 2_048
MAX_EXTRACTED_BYTES = 64 * 1024 * 1024


def download_archive(destination: Path) -> None:
    digest = hashlib.sha256()
    written = 0
    request = urllib.request.Request(
        POPLER_URL,
        headers={"User-Agent": "rdocx-ci-poppler-installer/1"},
    )
    with urllib.request.urlopen(request, timeout=60) as response, destination.open(
        "wb"
    ) as output:
        while chunk := response.read(1024 * 1024):
            written += len(chunk)
            if written > MAX_DOWNLOAD_BYTES:
                raise RuntimeError("Poppler archive exceeds the download bound")
            digest.update(chunk)
            output.write(chunk)
    if digest.hexdigest() != POPLER_SHA256:
        raise RuntimeError("Poppler archive SHA-256 does not match the reviewed source")


def safe_extract(archive_path: Path, destination: Path) -> Path:
    destination = destination.resolve()
    destination.mkdir(parents=True)
    member_count = 0
    extracted_bytes = 0
    with tarfile.open(archive_path, mode="r|xz") as archive:
        for member in archive:
            member_count += 1
            if member_count > MAX_ARCHIVE_MEMBERS:
                raise RuntimeError("Poppler archive exceeds the member-count bound")
            if member.size < 0:
                raise RuntimeError("Poppler archive contains a negative member size")
            extracted_bytes += member.size
            if extracted_bytes > MAX_EXTRACTED_BYTES:
                raise RuntimeError("Poppler archive exceeds the extracted-size bound")
            target = (destination / member.name).resolve()
            try:
                target.relative_to(destination)
            except ValueError as error:
                raise RuntimeError("Poppler archive contains an unsafe path") from error
            if member.isdir():
                target.mkdir(parents=True, exist_ok=True)
                continue
            if not member.isfile():
                raise RuntimeError("Poppler archive contains a non-file entry")
            source = archive.extractfile(member)
            if source is None:
                raise RuntimeError("Poppler archive member could not be read")
            target.parent.mkdir(parents=True, exist_ok=True)
            with source, target.open("wb") as output:
                shutil.copyfileobj(source, output)
            target.chmod(stat.S_IMODE(member.mode))

    source_root = destination / f"poppler-{POPLER_VERSION}"
    if not (source_root / "CMakeLists.txt").is_file():
        raise RuntimeError("Poppler archive has an unexpected root layout")
    return source_root


def run(command: list[str]) -> None:
    subprocess.run(command, check=True)


def verify_tools(prefix: Path) -> None:
    for tool in TOOLS:
        executable = prefix / "bin" / tool
        if not executable.is_file():
            raise RuntimeError(f"missing Poppler tool: {executable}")
        result = subprocess.run(
            [str(executable), "-v"],
            check=True,
            capture_output=True,
            text=True,
        )
        lines = tuple(
            line.strip()
            for line in (result.stdout + result.stderr).splitlines()
            if line.strip()
        )
        expected = f"{tool} version {POPLER_VERSION}"
        if not lines or lines[0] != expected:
            raise RuntimeError(f"unexpected {tool} identity: {lines[:1]}")


def authenticated_tool(receipt_path: Path, receipt_sha: str, executable: Path) -> None:
    """Authenticate a fresh source build without assuming a host binary hash."""
    digest = lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest()
    if digest(receipt_path) != receipt_sha:
        raise RuntimeError("Poppler build receipt changed")
    receipt = json.loads(receipt_path.read_text())
    if (
        receipt["status"] != "actual-source-build"
        or receipt["sourceURL"] != POPLER_URL
        or receipt["sourceArchiveSHA256"] != POPLER_SHA256
        or digest(receipt["sourceArchive"]) != POPLER_SHA256
        or receipt["installerSHA256"] != digest(__file__)
        or receipt["configureExit"] != 0 or receipt["compileExit"] != 0
        or receipt["sourceAndCompilerGuardsPassed"] is not True
        or receipt["configureArgv"][0] != "cmake"
        or receipt["compileArgv"][:2] != ["cmake", "--build"]
        or digest(receipt["compiler"]) != receipt["compilerSHA256"]
        or not receipt["compilerVersion"]
        or set(receipt["tools"]) != set(TOOLS)
    ):
        raise RuntimeError("Poppler source-build provenance is not qualified")
    tool = receipt["tools"]["pdftoppm"]
    if tool["path"] != str(executable) or digest(executable) != tool["sha256"]:
        raise RuntimeError("Poppler executable does not match its source build")


def expose_tools(prefix: Path) -> None:
    github_path = os.environ.get("GITHUB_PATH")
    if github_path:
        with Path(github_path).open("a", encoding="utf-8") as path_file:
            path_file.write(f"{prefix / 'bin'}\n")
    github_env = os.environ.get("GITHUB_ENV")
    if github_env:
        receipt = prefix / "source-build-receipt.json"
        with Path(github_env).open("a", encoding="utf-8") as env_file:
            env_file.write(f"F_X195_POPPLER={prefix / 'bin/pdftoppm'}\n")
            env_file.write(f"F_X195_POPPLER_BUILD_RECEIPT={receipt}\n")
            env_file.write(
                "F_X195_POPPLER_BUILD_RECEIPT_SHA="
                + hashlib.sha256(receipt.read_bytes()).hexdigest() + "\n"
            )


def build(prefix: Path) -> None:
    prefix = prefix.resolve()
    if prefix.exists() and (not prefix.is_dir() or any(prefix.iterdir())):
        raise RuntimeError(
            f"Poppler prefix must be empty so the reviewed source is rebuilt: {prefix}"
        )

    runner_temp = Path(os.environ.get("RUNNER_TEMP", tempfile.gettempdir()))
    with tempfile.TemporaryDirectory(prefix="rdocx-poppler-", dir=runner_temp) as work:
        work_root = Path(work)
        archive_path = work_root / f"poppler-{POPLER_VERSION}.tar.xz"
        download_archive(archive_path)
        source_root = safe_extract(archive_path, work_root / "source")
        build_root = work_root / "build"
        configure = [
            "cmake",
            "-S",
            str(source_root),
            "-B",
            str(build_root),
            "-G",
            "Ninja",
            "-DCMAKE_BUILD_TYPE=Release",
            "-DENABLE_UTILS=ON",
            "-DENABLE_CPP=OFF",
            "-DENABLE_GLIB=OFF",
            "-DENABLE_GOBJECT_INTROSPECTION=OFF",
            "-DENABLE_QT5=OFF",
            "-DENABLE_QT6=OFF",
            "-DENABLE_BOOST=OFF",
            "-DENABLE_LIBCURL=OFF",
            "-DENABLE_NSS3=OFF",
            "-DENABLE_GPGME=OFF",
            "-DBUILD_GTK_TESTS=OFF",
            "-DBUILD_QT5_TESTS=OFF",
            "-DBUILD_QT6_TESTS=OFF",
            "-DBUILD_CPP_TESTS=OFF",
            "-DBUILD_MANUAL_TESTS=OFF",
            "-DBUILD_SHARED_LIBS=OFF",
            "-DENABLE_UNSTABLE_API_ABI_HEADERS=OFF",
            "-DRUN_GPERF_IF_PRESENT=OFF",
        ]
        run(configure)
        cache = (build_root / "CMakeCache.txt").read_text()
        compiler = Path(next(
            line.split("=", 1)[1] for line in cache.splitlines()
            if line.startswith("CMAKE_CXX_COMPILER:FILEPATH=")
        ))
        compiler_sha = hashlib.sha256(compiler.read_bytes()).hexdigest()
        compiler_version = subprocess.check_output([str(compiler), "--version"], text=True)
        jobs = max(1, min(os.cpu_count() or 1, 4))
        compile_command = [
                "cmake",
                "--build",
                str(build_root),
                "--parallel",
                str(jobs),
                "--target",
                *TOOLS,
            ]
        run(compile_command)
        binary_root = prefix / "bin"
        binary_root.mkdir(parents=True, exist_ok=True)
        for tool in TOOLS:
            shutil.copy2(build_root / "utils" / tool, binary_root / tool)
        source_archive = prefix / archive_path.name
        shutil.copy2(archive_path, source_archive)
        verify_tools(prefix)
        assert hashlib.sha256(source_archive.read_bytes()).hexdigest() == POPLER_SHA256
        assert hashlib.sha256(compiler.read_bytes()).hexdigest() == compiler_sha
        receipt = {
            "status": "actual-source-build",
            "sourceURL": POPLER_URL,
            "sourceArchive": str(source_archive),
            "sourceArchiveSHA256": POPLER_SHA256,
            "installerSHA256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
            "compiler": str(compiler), "compilerSHA256": compiler_sha,
            "compilerVersion": compiler_version,
            "configureArgv": configure, "compileArgv": compile_command,
            "configureExit": 0, "compileExit": 0,
            "sourceAndCompilerGuardsPassed": True,
            "tools": {tool: {
                "path": str(binary_root / tool),
                "sha256": hashlib.sha256((binary_root / tool).read_bytes()).hexdigest(),
            } for tool in TOOLS},
        }
        (prefix / "source-build-receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")

    verify_tools(prefix)
    expose_tools(prefix)
    print(f"Installed Poppler {POPLER_VERSION} at {prefix}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    default_prefix = Path(
        os.environ.get("RUNNER_TEMP", tempfile.gettempdir())
    ) / f"poppler-{POPLER_VERSION}"
    parser.add_argument("--prefix", type=Path, default=default_prefix)
    arguments = parser.parse_args()
    build(arguments.prefix)


if __name__ == "__main__":
    main()
