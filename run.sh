#!/bin/sh

docker build . -t artificial-friend:latest
docker run --rm artificial-friend:latest
