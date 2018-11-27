#!/bin/bash

pacman-key --recv F7FD5492264BB9D0
pacman-key --lsign F7FD5492264BB9D0

cat >> /etc/pacman.conf << "EOF"
[dkp-libs]
Server = https://downloads.devkitpro.org/packages

[dkp-linux]
Server = https://downloads.devkitpro.org/packages/linux
EOF

pacman --noconfirm -U https://downloads.devkitpro.org/devkitpro-keyring-r1.787e015-2-any.pkg.tar.xz
pacman --noconfirm -Syu
pacman --noconfirm -S wii-dev gamecube-dev

cat >> /etc/bash.bashrc << "EOF"
source /etc/profile.d/devkit-env.sh
export PATH="${DEVKITPPC}/bin:${DEVKITARM}/bin:${PATH}"
EOF
