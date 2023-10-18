#!/bin/bash

sudo rm -frv ./workdir/*;
docker compose exec redis redis-cli flushall;
