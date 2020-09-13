#!/usr/bin/env bash
IMAGE=847110695266.dkr.ecr.ap-northeast-1.amazonaws.com/beatoraja_recommend_server

$(aws ecr get-login --no-include-email)
docker pull $IMAGE
docker ps | awk '{print $1}' | xargs docker kill
docker run -itd -v $(pwd)/files:/app/files -v $(pwd)/config.toml:/app/config.toml -p 443:8000 $IMAGE