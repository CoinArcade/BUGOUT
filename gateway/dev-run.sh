#!/bin/bash

# Cheater script.  Use from inside the docker
# container, to reduce incremental build times

cargo install --path . --force && gateway