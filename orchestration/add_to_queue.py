import redis
import argparse

def append_to_redis_list(redis_host='localhost', redis_port=6379, key_to_append='start_queue', value_to_append=''):
    # Initialize the Redis client
    r = redis.Redis(host=redis_host, port=redis_port)

    # Append the value to the list
    r.rpush(key_to_append, value_to_append)
    print(f"Appended '{value_to_append}' to the list stored at key {key_to_append}")

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Append a string to a Redis list.')
    parser.add_argument('string_to_append', type=str, help='The string to append to the Redis list.')

    args = parser.parse_args()
    append_to_redis_list(value_to_append=args.string_to_append)
