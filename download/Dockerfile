FROM ubuntu:latest

# Update Ubuntu packages
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get clean

# normal stuffs
RUN apt-get install -y vim aptitude python3-pip magic-wormhole file

# Install docker
RUN apt-get install -y ca-certificates curl gnupg 
RUN install -m 0755 -d /etc/apt/keyrings
RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg 
RUN chmod a+r /etc/apt/keyrings/docker.gpg
RUN echo \
  "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | \
  tee /etc/apt/sources.list.d/docker.list > /dev/null
RUN apt-get update
RUN apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

# install youtube downloader
RUN python3 -m pip install -U yt-dlp

COPY ./download_video.py /
RUN chmod +x /download_video.py

WORKDIR /workdir

CMD ["/download_video.py", ["NfmSjGbnEWk"]

