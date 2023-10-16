#!/bin/bash

# Load environment variables from the env file
source env

# Mount the EFS filesystem
sudo mount -t efs $EFS_ID ./workdir
