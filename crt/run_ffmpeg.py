#!/usr/bin/env python3

import subprocess
import sys

def run_ffmpeg(video_id):
    try:
        print("hi")
        subprocess.run(['sleep', '10'])
    except Exception as e:
        print(f"Something went wrong: {e}")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        run_ffmpeg(sys.argv[1])
    else:
        print("Please provide a YouTube video ID as an argument.")
