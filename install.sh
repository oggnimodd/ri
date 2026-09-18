#!/usr/bin/env bash
set -euo pipefail

umask 022

install_root="${CARGO_INSTALL_ROOT:-${CARGO_HOME:-$HOME/.cargo}}"

cargo install --locked --path . "$@"

for binary in ri rr rx ru rd; do
  binary_path="$install_root/bin/$binary"
  if [[ ! -f "$binary_path" ]]; then
    echo "Installed binary not found: $binary_path" >&2
    exit 1
  fi

  chmod 0755 "$binary_path"
done
