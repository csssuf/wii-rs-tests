FROM ubuntu:cosmic AS build

ADD https://github.com/devkitPro/buildscripts/releases/download/v20181114/buildscripts-20181114.tar.bz2 \
    /buildscripts.tar.bz2

# We want to build devkitPPC.
ENV BUILD_DKPRO_PACKAGE=2
# Don't prompt for anything.
ENV BUILD_DKPRO_AUTOMATED=1

RUN apt-get update && \
    apt-get install -yy build-essential && \
    mkdir /opt/dkp_build/ && \
    tar -C /opt/dkp_build -xf /buildscripts.tar.bz2 && \
    /opt/dkp_build/buildscripts/build-devkit.sh
