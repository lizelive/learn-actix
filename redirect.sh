#!/bin/sh

docker rm -f jumpbox 2>/dev/null
docker run --rm -it -d --name jumpbox ubuntu bash
socat TCP-LISTEN:2376,reuseaddr,fork UNIX-CLIENT:/var/run/docker.sock 
docker rm -f jumpbox
