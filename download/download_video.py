#!/usr/bin/env python3 

import subprocess
import sys
import redis
import os
import shlex
from vtt_to_srt.vtt_to_srt import ConvertFile
from moviepy.editor import VideoFileClip
import psycopg2


def remove_files_from_video_dir(video_id):
    directory = f"/application/media/downloads/{video_id}/"

    for filename in os.listdir(directory):
        file_path = os.path.join(directory, filename)
        try:
            if os.path.isfile(file_path):
                os.unlink(file_path)
        except Exception as e:
            print(f"Failed to delete {file_path}. Reason: {e}")


def get_another_video(video_id: str) -> str:
    # Connect to PostgreSQL
    conn = psycopg2.connect(
        host="postgres",
        database="television",
        user="television",
        password="television"
    )
    cursor = conn.cursor()

    # Find the most recent search related to the given video_id
    cursor.execute("""
        SELECT s.id
        FROM searches s
        JOIN videos v ON s.id = v.search
        WHERE v.video_id = %s
        ORDER BY s.creation DESC
        LIMIT 1
    """, (video_id,))
    
    search_id = cursor.fetchone()
    
    if search_id is None:
        return None

    search_id = search_id[0]
    
    # Get another video_id related to the same search
    cursor.execute("""
        SELECT video_id
        FROM videos
        WHERE search = %s AND video_id != %s
        LIMIT 1
    """, (search_id, video_id))

    alternate_video_id = cursor.fetchone()

    cursor.close()
    conn.close()

    return alternate_video_id[0] if alternate_video_id else None

def check_video_dimensions(video_id: str) -> bool:
    r = redis.Redis(host='redis', port=6379)
    video_path = f"/application/media/downloads/{video_id}/{video_id}.mp4"
    
    if not os.path.exists(video_path):
        print(f"Video file at {video_path} doesn't exist.")
        return False

    with VideoFileClip(video_path) as clip:
        width, height = clip.size
    
    return width > 180 and height > 144

def append_to_redis_list(redis_host='redis', redis_port=6379, key_to_append='encode_queue', value_to_append=''):
    r = redis.Redis(host=redis_host, port=redis_port)
    r.rpush(key_to_append, value_to_append)
    print(f"Appended '{value_to_append}' to the list stored at key {key_to_append}")

def download_youtube_video(video_id):
    try:
        # Create a directory for the video
        video_dir = f"/application/media/downloads/{video_id}"
        os.makedirs(video_dir, exist_ok=True)

        output_path = shlex.quote(f"{video_dir}/{video_id}.mp4")

        cmd = ['timeout', '60s', 'yt-dlp', '--no-progress', '--write-subs', '--write-auto-sub', 
                        '-o', output_path, '-f', "best[height<=?360]", '--', shlex.quote(video_id)]

        subprocess.run(cmd)

        original_video_id = video_id

        while not check_video_dimensions(video_id):
            print(f"Video {video_id} is too small, skipping")
            alternate = get_another_video(video_id)

            print("alternate video is", alternate)
            if alternate:
                remove_files_from_video_dir(video_id)
                cmd = ['timeout', '60s', 'yt-dlp', '--no-progress', '--write-subs', '--write-auto-sub', 
                            '-o', output_path, '-f', "best[height<=?360]", '--', shlex.quote(alternate)]
                subprocess.run(cmd)

        return original_video_id

    except Exception as e:
        print(f"Something went wrong: {e}")

def convert_subtitles(video_id):
    try:
        convert_file = ConvertFile(f"/application/media/downloads/{video_id}/{video_id}.en.vtt", "utf-8")
        convert_file.convert()
    except Exception as e:
        print(f"Couldn't convert subttitles: {e}")

def poll_redis_list(redis_host='redis', redis_port=6379, queue_to_poll='start_queue'):
    r = redis.Redis(host=redis_host, port=redis_port)
    while True:
        print("about to block at the redis queue")
        _, video_id = r.blpop(queue_to_poll)
        video_id = video_id.decode('utf-8')
        print(f"Got video ID {video_id} from {queue_to_poll}")
        video_id = download_youtube_video(video_id)
        convert_subtitles(video_id)
        append_to_redis_list(value_to_append=video_id)


if __name__ == "__main__":
    poll_redis_list()
