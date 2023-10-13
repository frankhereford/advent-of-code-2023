#!/bin/bash

# Read ID as an argument
ID="$1"

# Run the Docker Compose commands
docker compose run download "${ID}"
docker compose run crt "${ID}"

