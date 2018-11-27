FROM ubuntu:cosmic AS build

ADD https://github.com/devkitPro/buildscripts/releases/download/v20181114/buildscripts-20181114.tar.bz2 \
    /buildscripts.tar.bz2
COPY config.sh /config.sh

RUN apt-get update && \
    apt-get install -yy build-essential && \
    mkdir /opt/{dkp_build,devkitpro}/ && \
    tar -C /opt/dkp_build -xf /buildscripts.tar.bz2 && \
    mv /config.sh /opt/dkp_build/buildscripts && \
    /opt/dkp_build/buildscripts/build-devkit.sh

FROM ubuntu:cosmic

COPY --from=build /opt/devkitpro /opt/devkitpro
