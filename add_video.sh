#!/bin/bash

redis-cli -p 31780 LPUSH start_queue "$1"
