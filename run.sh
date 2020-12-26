#!/usr/bin/env bash
IMAGE=847110695266.dkr.ecr.ap-northeast-1.amazonaws.com/beatoraja_recommend_server
mkdir -p files

$(aws ecr get-login --no-include-email)
docker-compose down
docker system prune --force
docker-compose up -d

#migrate
docker-compose run builder diesel migration run