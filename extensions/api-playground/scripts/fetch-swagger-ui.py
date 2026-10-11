#!/usr/bin/env python3
"""Fetch the Swagger UI distribution into resources/public/swagger-ui.

The API playground's page (resources/public/index.html, ours) loads the Swagger UI assets
from the swagger-ui subfolder. Those assets are third-party code that goes stale quickly, so they
are not committed (the subfolder is in .gitignore). Run this script before `cargo run -p api-playground`:

    python3 scripts/fetch-swagger-ui.py                 # the latest GitHub release
    python3 scripts/fetch-swagger-ui.py --version v5.33.1
    python3 scripts/fetch-swagger-ui.py --target /some/other/folder

What it does: resolves the latest release tag of github.com/swagger-api/swagger-ui (unless
--version names one), downloads that tag's source tarball from codeload.github.com, extracts the
files of its `dist` folder into the target folder - all but upstream's own page (`index.html`,
`index.css`, `swagger-initializer.js`), which this application replaces - and writes
`swagger-ui-version.txt` recording what was fetched. The target folder is emptied
first; as a guard against a mistyped --target, a non-empty folder is emptied only when it holds a
previous fetch (the version file or swagger-ui-bundle.js).

Python 3.8+, standard library only. The Node.js twin is scripts/fetch-swagger-ui.js.
"""

import argparse
import io
import json
import re
import shutil
import sys
import tarfile
import urllib.error
import urllib.request
from datetime import datetime, timezone
from pathlib import Path

REPO = "swagger-api/swagger-ui"
LATEST_RELEASE_URL = f"https://api.github.com/repos/{REPO}/releases/latest"
TARBALL_URL = f"https://codeload.github.com/{REPO}/tar.gz/refs/tags/{{tag}}"
USER_AGENT = "mercury-api-playground-fetch-swagger-ui"
DIST_MEMBER = re.compile(r"^[^/]+/dist/([^/]+)$")
UPSTREAM_PAGE = {"index.html", "index.css", "swagger-initializer.js"}
VERSION_FILE = "swagger-ui-version.txt"
MARKERS = (VERSION_FILE, "swagger-ui-bundle.js")
DEFAULT_TARGET = Path(__file__).resolve().parent.parent / "resources" / "public" / "swagger-ui"


def fetch(url: str, accept: str) -> bytes:
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT, "Accept": accept})
    with urllib.request.urlopen(request, timeout=60) as response:
        return response.read()


def latest_tag() -> str:
    try:
        release = json.loads(fetch(LATEST_RELEASE_URL, "application/vnd.github+json"))
    except urllib.error.HTTPError as e:
        sys.exit(f"Cannot resolve the latest release ({e.code} from the GitHub API) - "
                 f"name one with --version vX.Y.Z (see https://github.com/{REPO}/releases)")
    tag = release.get("tag_name")
    if not isinstance(tag, str) or not tag:
        sys.exit("The GitHub API answered without a tag_name - name one with --version vX.Y.Z")
    return tag


def dist_files(tarball: bytes) -> dict:
    files = {}
    with tarfile.open(fileobj=io.BytesIO(tarball), mode="r:gz") as tar:
        for member in tar:
            match = DIST_MEMBER.match(member.name)
            if match and member.isfile() and match.group(1) not in UPSTREAM_PAGE:
                extracted = tar.extractfile(member)
                if extracted is not None:
                    files[match.group(1)] = extracted.read()
    if not files:
        sys.exit("The tarball holds no dist folder - the upstream layout may have changed")
    return files


def prepare_target(target: Path) -> None:
    if target.exists() and not target.is_dir():
        sys.exit(f"{target} is not a folder")
    if target.is_dir():
        entries = list(target.iterdir())
        if entries and not any((target / marker).is_file() for marker in MARKERS):
            sys.exit(f"{target} is not empty and holds no previous fetch - refusing to empty it")
        for entry in entries:
            if entry.is_dir() and not entry.is_symlink():
                shutil.rmtree(entry)
            else:
                entry.unlink()
    target.mkdir(parents=True, exist_ok=True)



def main() -> None:
    parser = argparse.ArgumentParser(description="Fetch Swagger UI into resources/public")
    parser.add_argument("--version", default="latest",
                        help="a release tag such as v5.33.1 (default: the latest release)")
    parser.add_argument("--target", type=Path, default=DEFAULT_TARGET,
                        help=f"the folder to populate (default: {DEFAULT_TARGET})")
    args = parser.parse_args()

    tag = latest_tag() if args.version == "latest" else args.version
    if not re.fullmatch(r"v?\d+\.\d+\.\d+(-[0-9A-Za-z.]+)?", tag):
        sys.exit(f"'{tag}' does not look like a release tag (expected vX.Y.Z)")
    url = TARBALL_URL.format(tag=tag)
    print(f"Fetching Swagger UI {tag} from {url}")
    try:
        tarball = fetch(url, "application/octet-stream")
    except urllib.error.HTTPError as e:
        sys.exit(f"Download failed ({e.code}) - is {tag} a release tag of {REPO}?")

    files = dist_files(tarball)
    target = args.target.resolve()
    prepare_target(target)
    for name, content in sorted(files.items()):
        (target / name).write_bytes(content)
    stamp = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    (target / VERSION_FILE).write_text(f"swagger-ui {tag}\nfetched {stamp}\nfrom {url}\n", encoding="utf-8")

    total = sum(len(content) for content in files.values())
    print(f"Wrote {len(files)} files ({total:,} bytes) to {target}")
    print("Next: cargo run -p api-playground")


if __name__ == "__main__":
    main()
