#!/usr/bin/env python3

import subprocess
import sys
import os

def download_youtube_video(video_id):
    try:
        subprocess.run(['yt-dlp', '-o', '%(id)s.mp4', '-f', "best[height<=?360]", video_id])
    except Exception as e:
        print(f"Something went wrong: {e}")

def run_docker_compose(video_id):
    try:
        os.chdir('/application')
        subprocess.run(['docker', 'compose', 'run', 'crt', video_id])
    except Exception as e:
        print(f"Couldn't run Docker Compose: {e}")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        video_id = sys.argv[1]
        download_youtube_video(video_id)
        # run_docker_compose(video_id) # this is a hack
    else:
        print("Please provide a YouTube video ID as an argument.")
