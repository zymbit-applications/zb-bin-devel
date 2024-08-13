#!/bin/bash

set -e

if [ "$(id -u)" -ne 0 ]; then
  echo "zb-install.sh: error: this script must run as root" >&2
  exit 1
fi

echo "zb-install.sh: bootstrapping the zbcli installer"
TMPDIR="$(mktemp -d)"

function cleanup() {
  echo "zb-install.sh: cleaning up"
  popd >/dev/null 2>&1 || true
  rm -rf "$TMPDIR"
  exit
}

trap cleanup EXIT
pushd "$TMPDIR" >/dev/null 2>&1 || true
curl -sSL https://github.com/zymbit-applications/zb-bin/releases/download/installer/installer -o ./zb-installer
chmod +x zb-installer
./zb-installer "$@"
