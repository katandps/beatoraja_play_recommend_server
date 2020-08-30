#!/usr/bin/env bash
$(aws ecr get-login --no-include-email)
docker run -it -v $(pwd):/app/ 847110695266.dkr.ecr.ap-northeast-1.amazonaws.com/beatoraja_recommend_server