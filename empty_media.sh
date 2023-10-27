#!/bin/bash

sudo rm -frv ./media/hls/*;
sudo rm -frv ./media/downloads/*;
#docker compose exec redis redis-cli flushall;
redis-cli -p 31780 flushall;
#docker service update --force television_apache
echo "truncate searches cascade;" | docker run -e PGUSER=television -e PGHOST=host.docker.internal -e PGPORT=31532 -e PGDATABASE=television -i postgres psql 
