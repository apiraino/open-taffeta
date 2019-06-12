#!/bin/bash

# How to reproduce locally a Travis CI build
# source: https://stackoverflow.com/a/49019950

RANDOM=$$
RAND=$( echo $RANDOM )
BUILDID="build-$RAND"
# check latest instance available on
# https://hub.docker.com/r/travisci/ci-garnet/tags/

# Trusty
# INSTANCE="travisci/ci-garnet:packer-1515445631-7dfb2e1"
# Xenial
INSTANCE="travisci/ci-sardonyx:packer-1558623664-f909ac5"

echo "Running $BUILDID"
docker run --name $BUILDID -dit $INSTANCE /sbin/init

# Then enter the container
# docker exec -it $DOCKER_CONTAINER bash
# or
# docker exec -it $BUILDID bash -l

# su - travis
# curl https://sh.rustup.rs -sSf | sh
# (choose nightly)

# Then execute the commands from a Travis CI build
