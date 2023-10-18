#!/bin/bash

docker compose exec redis redis-cli LPUSH start_queue $1;
