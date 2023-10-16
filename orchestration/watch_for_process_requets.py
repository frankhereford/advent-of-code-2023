#!/usr/bin/env python3

import redis
import time
import subprocess

def poll_redis_for_key(redis_host='localhost', redis_port=6379, start_queue='start_queue', encode_queue='encode_queue', poll_interval=1):
    try:
        r = redis.Redis(host=redis_host, port=redis_port)
    except Exception as e:
        print(f"Could not connect to Redis: {e}")
        return
    
    while True:
        try:
            list_length = r.llen(start_queue)
        except redis.RedisError:
            print("Error reading from Redis. Retrying in a sec.")
            time.sleep(1)
            continue

        if list_length > 0:
            list_values = r.lrange(start_queue, 0, -1)
            print(f"Found the key {start_queue} with values: {list_values}")
            first_value = r.lindex(start_queue, 0).decode('utf-8')


            # Run the Docker commands
            try:
                subprocess.Popen(["docker", "compose", "run", "download", first_value])

                # Remove the processed element
                r.lpop(start_queue)

            except subprocess.CalledProcessError as e:
                print(f"Docker command failed: {e}")
        else:
            print(f"Key {start_queue} not found or list is empty. Polling again in {poll_interval} seconds.")
        

        try:
            list_length = r.llen(encode_queue)
        except redis.RedisError:
            print("Error reading from Redis. Retrying in a sec.")
            time.sleep(1)
            continue

        if list_length > 0:
            list_values = r.lrange(encode_queue, 0, -1)
            print(f"Found the key {encode_queue} with values: {list_values}")
            first_value = r.lindex(encode_queue, 0).decode('utf-8')


            # Run the Docker commands
            try:
                subprocess.Popen(["docker", "compose", "run", "crt", first_value])

                # Remove the processed element
                r.lpop(encode_queue)

            except subprocess.CalledProcessError as e:
                print(f"Docker command failed: {e}")
        else:
            print(f"Key {encode_queue} not found or list is empty. Polling again in {poll_interval} seconds.")
        
        time.sleep(poll_interval)

if __name__ == '__main__':
    poll_redis_for_key()
