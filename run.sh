#!/usr/bin/env bash
IMAGE=847110695266.dkr.ecr.ap-northeast-1.amazonaws.com/beatoraja_recommend_server

mkdir -p files
rm -f files/*.db
aws s3 sync s3://beatoraja-play-recommend files/

$(aws ecr get-login --no-include-email)
docker-compose up -d
# container volume imageを掃除する
docker system prune --force
