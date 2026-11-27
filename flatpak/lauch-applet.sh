#!/usr/bin/env bash
set -Eeuo pipefail

APP="com.github.MiracleForge.cosmic-ext-marketwatch"
RELEASE_FILE="Release.md"
DRY_RUN="${1:-}"

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

echo "======================================"
echo " COSMIC MarketWatch - Release"
echo "======================================"

for cmd in just git python; do
    command -v "$cmd" >/dev/null 2>&1 || {
        echo "ERROR: $cmd not found."
        exit 1
    }
done

VERSION="$(python -c '
import tomllib
with open("Cargo.toml", "rb") as f:
    print(tomllib.load(f)["package"]["version"])
')"

TAG="v$VERSION"

echo "Version: $VERSION"
echo "Git tag: $TAG"

if [[ ! -f "$RELEASE_FILE" ]]; then
    [[ -f "CHANGELOG.md" ]] && RELEASE_FILE="CHANGELOG.md" || {
        echo "ERROR: changelog.md not found."
        exit 1
    }
fi

python - "$RELEASE_FILE" "$VERSION" <<'PY'
import re
import sys

file, version = sys.argv[1:]
content = open(file, encoding="utf-8").read()

if not re.search(r"(?<![0-9])" + re.escape(version) + r"(?![0-9])", content):
    print(f"ERROR: version {version} not found in {file}.")
    sys.exit(1)

print(f"OK: version {version} found in {file}.")
PY

if [[ -n "$(git status --porcelain)" ]]; then
    echo "ERROR: uncommitted changes detected."
    git status --short
    exit 1
fi

if git show-ref --verify --quiet "refs/tags/$TAG"; then
    echo "ERROR: tag $TAG already exists locally."
    exit 1
fi

if git ls-remote --exit-code --tags origin "refs/tags/$TAG" >/dev/null 2>&1; then
    echo "ERROR: tag $TAG already exists on GitHub."
    exit 1
fi

echo
echo "All checks passed!"

if [[ "$DRY_RUN" == "--test" ]]; then
    echo
    echo "======================================"
    echo " DRY RUN - TEST COMPLETED"
    echo "======================================"
    echo "Version: $VERSION"
    echo "Git tag: $TAG"
    echo "Release notes: $RELEASE_FILE"
    echo "Build release: READY"
    echo "Install: READY"
    echo "Flatpak bundle: READY"
    echo "Git tag: READY"
    echo "GitHub push: READY"
    echo
    echo "No changes were made."
    exit 0
fi

echo
echo "[1/5] Building release"
just build-release

echo
echo "[2/5] Installing"
just install

echo
echo "[3/5] Building Flatpak bundle"
just flatpak-bundle

echo
echo "[4/5] Creating tag $TAG"
git tag -a "$TAG" -m "Release $TAG"

echo
echo "[5/5] Pushing tag to GitHub"
git push origin "$TAG"

echo
echo "======================================"
echo " RELEASE $TAG PUBLISHED!"
echo "======================================"
echo "Package: $APP.flatpak"
