#!/usr/bin/env bash
# rban installer — no sudo, nothing outside $PREFIX is touched.
#
#   curl -fsSL https://raw.githubusercontent.com/s-brue/rban/main/install.sh | bash
#
# Uninstall:
#   curl -fsSL https://raw.githubusercontent.com/s-brue/rban/main/install.sh | bash -s -- --uninstall
#
# Installs a single self-contained binary into $RBAN_PREFIX/bin/ (default: ~/.local/bin/).
#
# Environment:
#   RBAN_PREFIX   install prefix (default: ~/.local)
#   RBAN_REF      version tag to install (default: latest release)
#
# Piping a script into a shell runs whatever is served — read it first if you like: this file is short.
set -euo pipefail

REPO="s-brue/rban"
TMP=""

cleanup() { [ -z "$TMP" ] || rm -rf "$TMP"; }
trap cleanup EXIT

download() { # download URL FILE
  if command -v curl >/dev/null; then
    curl -fsSL "$1" -o "$2"
  elif command -v wget >/dev/null; then
    wget -qO "$2" "$1"
  else
    die "curl or wget is required"
  fi
}

say() { printf '==> %s\n' "$*"; }
die() { printf 'rban install: %s\n' "$*" >&2; exit 1; }

main() {
  case "${1:-}" in
    --uninstall) uninstall; return ;;
    "")          ;;
    *)           die "unknown option '$1' (the only one is --uninstall)" ;;
  esac

  local prefix="${RBAN_PREFIX:-$HOME/.local}"
  local dest="$prefix/bin/rban"

  # Detect OS
  local os
  case "$(uname -s)" in
    Linux)  os="linux"  ;;
    Darwin) os="macos"  ;;
    *)      die "unsupported OS: $(uname -s)" ;;
  esac

  # Detect architecture
  local arch
  case "$(uname -m)" in
    x86_64)        arch="x86_64"  ;;
    aarch64|arm64) arch="aarch64" ;;
    *)             die "unsupported architecture: $(uname -m)" ;;
  esac

  # Resolve version: explicit RBAN_REF or latest GitHub release
  local ref="${RBAN_REF:-}"
  if [ -z "$ref" ]; then
    say "resolving latest release..."
    if command -v curl >/dev/null; then
      ref=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
            | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')
    elif command -v wget >/dev/null; then
      ref=$(wget -qO- "https://api.github.com/repos/$REPO/releases/latest" \
            | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')
    else
      die "curl or wget is required"
    fi
    [ -n "$ref" ] || die "could not determine latest release (is the repo public and has releases?)"
  fi

  # The release asset is rban-<os>-<arch>.tar.gz. Releases up to v0.1.1 shipped the Linux binary bare (UPX-compressed,
  # slow to start), so a pinned old RBAN_REF still works.
  local base="https://github.com/$REPO/releases/download/$ref/rban-${os}-${arch}"

  say "installing rban $ref ($os/$arch) into $dest"
  mkdir -p "$prefix/bin"

  TMP="$(mktemp -d)"
  if download "$base.tar.gz" "$TMP/rban.tar.gz" 2>/dev/null; then
    tar xzf "$TMP/rban.tar.gz" -C "$TMP"
    mv "$TMP/rban" "$dest"
  elif [ "$os" = "linux" ] && download "$base" "$TMP/rban" 2>/dev/null; then
    mv "$TMP/rban" "$dest"
  else
    die "could not download $base.tar.gz (is there a release $ref for $os/$arch?)"
  fi
  chmod 755 "$dest"

  # On macOS, clear the quarantine flag so Gatekeeper doesn't block it
  if [ "$os" = "macos" ] && command -v xattr >/dev/null 2>&1; then
    xattr -d com.apple.quarantine "$dest" 2>/dev/null || true
  fi

  local installed
  installed=$("$dest" --version 2>&1) || die "installed, but 'rban --version' failed: $installed"
  say "installed $installed"

  # Warn if prefix/bin is not in PATH
  case ":$PATH:" in
    *":$prefix/bin:"*) ;;
    *) printf '\nNote: %s is not in your PATH. Add it:\n  echo '\''export PATH="%s:$PATH"'\'' >> ~/.profile\n' \
         "$prefix/bin" "$prefix/bin" ;;
  esac
  printf '\nTry:  rban "Hello"\n'
}

uninstall() {
  local dest="${RBAN_PREFIX:-$HOME/.local}/bin/rban"
  if [ ! -f "$dest" ]; then
    say "rban is not installed at $dest"
    return
  fi
  rm -f "$dest"
  say "rban removed (~/.cache/rban.json and ~/.config/rban were left alone)"
}

main "$@"
