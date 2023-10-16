FROM linuxserver/ffmpeg

# Update Ubuntu packages
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get clean

# normal stuffs
RUN apt-get install -y vim aptitude python3-pip magic-wormhole file fonts-freefont-ttf git binutils

# install amazon's efs-utils
WORKDIR /root
RUN git clone https://github.com/aws/efs-utils
WORKDIR /root/efs-utils
RUN ./build-deb.sh
RUN apt-get -y install ./build/amazon-efs-utils-*.deb

WORKDIR /application/workdir

ENTRYPOINT ["/application/crt/run_ffmpeg.py"]

