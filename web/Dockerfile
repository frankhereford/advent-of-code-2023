FROM node:current-bookworm

# Update Ubuntu packages
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get clean

# normal stuff
RUN apt-get install -y vim aptitude python3-pip magic-wormhole file

RUN mkdir /application
COPY . /application/web
WORKDIR /application/web
RUN npm install
CMD ["npx", "next", "dev"]