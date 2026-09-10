#!/usr/bin/env bash
# sync-profile.sh - refresh the vendored assets/profile.toml from the
# source-of-truth resume repo (https://github.com/GitanElyon/resume).
#
# Priority:
#   1. Sibling checkout ../resume/resume.toml (your local layout)
#   2. GitHub raw URL (works anywhere with network)
#
# Usage:
#   ./scripts/sync-profile.sh
set -euo pipefail
cd "$(dirname "$0")/.."

DEST="assets/profile.toml"
RAW_URL="https://raw.githubusercontent.com/GitanElyon/resume/main/resume.toml"

if [[ -f "../resume/resume.toml" ]]; then
  if cmp -s "../resume/resume.toml" "$DEST"; then
    echo "sync-profile: already up to date (sibling checkout)."
  else
    cp "../resume/resume.toml" "$DEST"
    echo "sync-profile: copied ../resume/resume.toml -> $DEST"
  fi
elif command -v curl >/dev/null 2>&1; then
  curl -fsSL "$RAW_URL" -o "$DEST"
  echo "sync-profile: downloaded $RAW_URL -> $DEST"
elif command -v wget >/dev/null 2>&1; then
  wget -qO "$DEST" "$RAW_URL"
  echo "sync-profile: downloaded $RAW_URL -> $DEST"
else
  echo "sync-profile: no sibling checkout and neither curl nor wget found." >&2
  exit 1
fi
