#!/bin/bash

# Get the container IDs for containers matching the pattern
container_ids=$(docker ps --filter "name=advent-of-code-2023-crt-run-*" --format "{{.ID}}")

# Stop each matching container
for id in $container_ids; do
  docker stop $id &
done
