#!/bin/sh
# Generate a Homebrew formula for the latest forgejo-cli-plus release.
# Run this after publishing a new release, then commit the result to
# your homebrew-forgejo-cli-plus tap repo.
#
# Usage: ./update-formula.sh [version]
#   version  e.g. "1.0.0" (without v prefix). Defaults to latest release.
set -eu

REPO="stalecontext/forgejo-cli-plus"
API_BASE="https://codeberg.org/api/v1/repos/${REPO}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TEMPLATE="${SCRIPT_DIR}/forgejo-cli-plus.rb"

if [ $# -ge 1 ]; then
    VERSION="$1"
else
    VERSION=$(curl -fsSL "${API_BASE}/releases/latest" \
        | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"v\{0,1\}\([^"]*\)".*/\1/p')
    [ -n "$VERSION" ] || { echo "Error: could not detect latest version" >&2; exit 1; }
fi

printf 'Generating formula for v%s\n' "$VERSION"

DL_BASE="https://codeberg.org/${REPO}/releases/download/v${VERSION}"
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

download_and_hash() {
    local file="$1"
    curl -fsSL "${DL_BASE}/${file}" -o "${TMPDIR}/${file}"
    shasum -a 256 "${TMPDIR}/${file}" | cut -d' ' -f1
}

SHA_LINUX_X86_64=$(download_and_hash "forgejo-cli-plus-linux-x86_64.tar.gz")
SHA_MACOS_X86_64=$(download_and_hash "forgejo-cli-plus-macos-x86_64.tar.gz")
SHA_MACOS_AARCH64=$(download_and_hash "forgejo-cli-plus-macos-aarch64.tar.gz")

OUTPUT="${SCRIPT_DIR}/forgejo-cli-plus.rb"

sed \
    -e "s/VERSION/${VERSION}/g" \
    -e "s/SHA256_LINUX_X86_64/${SHA_LINUX_X86_64}/g" \
    -e "s/SHA256_MACOS_X86_64/${SHA_MACOS_X86_64}/g" \
    -e "s/SHA256_MACOS_AARCH64/${SHA_MACOS_AARCH64}/g" \
    "$TEMPLATE" > "${TMPDIR}/forgejo-cli-plus.rb"

mv "${TMPDIR}/forgejo-cli-plus.rb" "$OUTPUT"

printf 'Formula written to %s\n' "$OUTPUT"
printf '  linux-x86_64:   %s\n' "$SHA_LINUX_X86_64"
printf '  macos-x86_64:   %s\n' "$SHA_MACOS_X86_64"
printf '  macos-aarch64:  %s\n' "$SHA_MACOS_AARCH64"
