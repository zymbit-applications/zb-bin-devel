#!/bin/bash

echo "bootstrapping the zbcli installer. you may be asked for your password."
pushd "$(mktemp -d)" >/dev/null
curl -sSL https://github.com/zymbit-applications/zb-bin/releases/download/installer/installer -o ./zb-installer
chmod +x zb-installer
sudo ./zb-installer $@
rm zb-installer
popd >/dev/null
