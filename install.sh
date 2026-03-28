#!/bin/sh
# Install forgejo-cli-plus (fj) - https://codeberg.org/stalecontext/forgejo-cli-plus
# Usage: curl -fsSL https://codeberg.org/stalecontext/forgejo-cli-plus/raw/branch/main/install.sh | sh
set -eu

REPO="stalecontext/forgejo-cli-plus"
API_BASE="https://codeberg.org/api/v1/repos/${REPO}"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

die() { printf 'Error: %s\n' "$1" >&2; exit 1; }

# Detect OS
case "$(uname -s)" in
    Linux*)  OS="linux"  ;;
    Darwin*) OS="macos"  ;;
    *)       die "Unsupported OS: $(uname -s). Use Linux or macOS." ;;
esac

# Detect architecture
case "$(uname -m)" in
    x86_64|amd64)   ARCH="x86_64"  ;;
    aarch64|arm64)   ARCH="aarch64" ;;
    *)               die "Unsupported architecture: $(uname -m)" ;;
esac

# Only macOS has aarch64 builds; Linux is x86_64 only for now
if [ "$OS" = "linux" ] && [ "$ARCH" = "aarch64" ]; then
    die "No Linux aarch64 build available yet. Use cargo install instead."
fi

ARCHIVE="forgejo-cli-plus-${OS}-${ARCH}.tar.gz"

# Fetch latest release tag
printf 'Fetching latest release...\n'
if command -v curl >/dev/null 2>&1; then
    TAG=$(curl -fsSL "${API_BASE}/releases/latest" | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p')
elif command -v wget >/dev/null 2>&1; then
    TAG=$(wget -qO- "${API_BASE}/releases/latest" | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p')
else
    die "Neither curl nor wget found. Install one and retry."
fi

[ -n "$TAG" ] || die "Could not determine latest release tag."
printf 'Latest release: %s\n' "$TAG"

DOWNLOAD_URL="https://codeberg.org/${REPO}/releases/download/${TAG}/${ARCHIVE}"

# Download and extract
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

printf 'Downloading %s...\n' "$ARCHIVE"
if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$DOWNLOAD_URL" -o "${TMPDIR}/${ARCHIVE}"
else
    wget -q "$DOWNLOAD_URL" -O "${TMPDIR}/${ARCHIVE}"
fi

tar -xzf "${TMPDIR}/${ARCHIVE}" -C "$TMPDIR"

# Install
if [ -w "$INSTALL_DIR" ]; then
    mv "${TMPDIR}/fj" "${INSTALL_DIR}/fj"
else
    printf 'Installing to %s (requires sudo)...\n' "$INSTALL_DIR"
    sudo mv "${TMPDIR}/fj" "${INSTALL_DIR}/fj"
fi

chmod +x "${INSTALL_DIR}/fj"

printf 'Installed fj %s to %s\n' "$TAG" "$INSTALL_DIR"
"${INSTALL_DIR}/fj" version
