#!/usr/bin/env python3

import subprocess
import sys
import redis
import os
from vtt_to_srt.vtt_to_srt import ConvertFile


def append_to_redis_list(redis_host='redis', redis_port=6379, key_to_append='encode_queue', value_to_append=''):
    # Initialize the Redis client
    r = redis.Redis(host=redis_host, port=redis_port)

    # Append the value to the list
    r.rpush(key_to_append, value_to_append)
    print(f"Appended '{value_to_append}' to the list stored at key {key_to_append}")

def download_youtube_video(video_id):
    try:
        subprocess.run(['yt-dlp', '--write-subs', '--write-auto-sub', '-o', '%(id)s.mp4', '-f', "best[height<=?360]", video_id])
    except Exception as e:
        print(f"Something went wrong: {e}")

def convert_subtitles(video_id):
    try:
        convert_file = ConvertFile(f"/application/workdir/{video_id}.en.vtt", "utf-8")
        convert_file.convert()
    except Exception as e:
        print(f"Couldn't convert subttitles: {e}")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        video_id = sys.argv[1]
        download_youtube_video(video_id)
        convert_subtitles(video_id)
        append_to_redis_list(value_to_append=video_id)
    else:
        print("Please provide a YouTube video ID as an argument.")
