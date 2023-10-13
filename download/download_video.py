#!/usr/bin/env python3

import subprocess

def download_youtube_video():
    try:
        subprocess.run(['yt-dlp', 'NfmSjGbnEWk'])
    except Exception as e:
        print(f"Something went wrong: {e}")

if __name__ == "__main__":
    download_youtube_video()
