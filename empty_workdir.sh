#!/bin/bash

sudo rm -frv ./workdir/*;
#docker compose exec redis redis-cli flushall;
redis-cli flushall;
mkdir workdir/hls
docker service update --force television_apache
