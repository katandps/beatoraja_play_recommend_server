#!/usr/bin/env bash
IMAGE=847110695266.dkr.ecr.ap-northeast-1.amazonaws.com/beatoraja_recommend_server

mkdir -p files
rm files/*.db
aws s3 sync s3://beatoraja-play-recommend files/

$(aws ecr get-login --no-include-email)
docker pull $IMAGE
docker ps | awk '{print $1}' | xargs docker kill
docker run -itd --restart=always -v $(pwd)/files:/app/files -v $(pwd)/config.toml:/app/config.toml -p 443:8000 $IMAGE