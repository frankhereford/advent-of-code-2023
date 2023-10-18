#!/bin/bash

redis-cli  LPUSH start_queue "$1"
