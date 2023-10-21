#!/bin/bash

sudo rm -frv ./media/hls/*;
sudo rm -frv ./media/downloads/*;
#docker compose exec redis redis-cli flushall;
redis-cli flushall;
#docker service update --force television_apache
