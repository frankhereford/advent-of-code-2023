FROM linuxserver/ffmpeg

# Update Ubuntu packages
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get clean

# normal stuffs
RUN apt-get install -y vim aptitude python3-pip magic-wormhole file fonts-freefont-ttf

WORKDIR /application/workdir

ENTRYPOINT ["/application/crt/run_ffmpeg.py"]

