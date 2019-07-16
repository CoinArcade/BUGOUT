#!/bin/bash

alias docker-compose='docker run --rm \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v "$PWD:/rootfs/$PWD" \
  -w="/rootfs/$PWD" \
docker/compose:1.13.0'

# This script is useful for environments where you
# do not have access to cargo, gradle etc
docker-compose build judge
docker-compose build gamestates-aggregator
docker-compose build gateway
docker-compose build kafkacat