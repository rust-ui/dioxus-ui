#!/usr/bin/env bash
set -euo pipefail

die() { echo "❌ ERROR: $*" >&2; exit 1; }

BUILD_DIR="target/dx/dioxus-ui/debug/web/public"

command -v dx      >/dev/null 2>&1 || die "'dx' not found in PATH"
command -v wasmer  >/dev/null 2>&1 || die "'wasmer' not found in PATH"

echo "Building..."
dx build --platform web

[[ -f "$BUILD_DIR/index.html" ]] || die "Build output missing: $BUILD_DIR/index.html"

# WHY: Dioxus produces a pure client-side SPA (single index.html).
# Wasmer's static-web-server returns 404 for any path it can't find on disk.
# Wasmer's CDN intercepts 404 responses before they reach the browser, so
# server-side fallback flags (--page-fallback, SERVER_FALLBACK_PAGE) don't help.
#
# The fix: place a real index.html at every route path so the server always
# finds a file. The WASM router then handles navigation client-side.
#
# Component slugs are extracted from the registry source files (single source of truth).
COMPONENTS=$(grep -rh 'slug: "[a-z]*"' src/registry/ \
    | grep -oE '"[a-z]+"' \
    | tr -d '"' \
    | grep -v "^$") || die "Failed to extract component slugs from src/registry/"

[[ -n "$COMPONENTS" ]] || die "No component slugs found in src/registry/ — check slug: \"...\" entries"

echo "Copying index.html to component routes..."
while IFS= read -r comp; do
    mkdir -p "$BUILD_DIR/components/$comp" || die "Failed to create directory for component: $comp"
    cp "$BUILD_DIR/index.html" "$BUILD_DIR/components/$comp/index.html" || die "Failed to copy index.html for component: $comp"
done <<< "$COMPONENTS"

echo "Deploying to Wasmer..."
wasmer deploy
