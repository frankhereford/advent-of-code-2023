FROM ubuntu:latest

# Update Ubuntu packages
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get clean

# normal stuff
RUN apt-get install -y vim aptitude python3-pip magic-wormhole file

# install youtube downloader
RUN python3 -m pip install -U yt-dlp vtt_to_srt3

WORKDIR /application/workdir

ENTRYPOINT ["/application/download/download_video.py"]

