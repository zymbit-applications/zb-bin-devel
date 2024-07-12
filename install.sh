#!/bin/bash

curl -sSL https://github.com/zymbit-applications/zb-bin/releases/download/installer/installer -o /tmp/zb-installer

sudo chmod +x /tmp/zb-installer

sudo /tmp/zb-installer

sudo rm /tmp/zb-installer
