# docker buildx build --platform linux/amd64,linux/arm64 . -t s1s5/ssh-client --push
FROM debian

RUN apt-get update && apt-get install -y ssh && rm -rf /var/lib/apt/lists/*
RUN set -eux; \
    apt-get update; \
    apt-get install -y gosu; \
    rm -rf /var/lib/apt/lists/*; \
    gosu nobody true

ENV USERNAME=docker-user
ENV GROUPNAME=docker-group
ENV UID=1000
ENV GID=1000

COPY ./entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
