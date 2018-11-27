#!/bin/bash

export BUILD_DKPRO_PACKAGE=2
export BUILD_DKPRO_AUTOMATED=1
export BUILD_DKPRO_INSTALLDIR=/opt/devkitpro
export MAKEFLAGS="${MAKEFLAGS} -j$(grep -c ^processor /proc/cpuinfo)"
