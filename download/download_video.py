#!/usr/bin/env python3

import subprocess
import sys
import redis
import os
from vtt_to_srt.vtt_to_srt import ConvertFile


def append_to_redis_list(redis_host='redis', redis_port=6379, key_to_append='encode_queue', value_to_append=''):
    r = redis.Redis(host=redis_host, port=redis_port)
    r.rpush(key_to_append, value_to_append)
    print(f"Appended '{value_to_append}' to the list stored at key {key_to_append}")

def download_youtube_video(video_id):
    try:
        # Create a directory for the video
        video_dir = f"/application/media/downloads/{video_id}"
        os.makedirs(video_dir, exist_ok=True)

        subprocess.run(['timeout', '60s', 'yt-dlp', '--write-subs', '--write-auto-sub', '-o', f"{video_dir}/%(id)s.mp4", '-f', "best[height<=?360]", video_id])
    except Exception as e:
        print(f"Something went wrong: {e}")

def convert_subtitles(video_id):
    try:
        convert_file = ConvertFile(f"/application/media/{video_id}.en.vtt", "utf-8")
        convert_file.convert()
    except Exception as e:
        print(f"Couldn't convert subttitles: {e}")

def poll_redis_list(redis_host='redis', redis_port=6379, queue_to_poll='start_queue'):
    r = redis.Redis(host=redis_host, port=redis_port)
    while True:
        _, video_id = r.blpop(queue_to_poll)
        video_id = video_id.decode('utf-8')
        print(f"Got video ID {video_id} from {queue_to_poll}")
        download_youtube_video(video_id)
        convert_subtitles(video_id)
        append_to_redis_list(value_to_append=video_id)


if __name__ == "__main__":
    poll_redis_list()
