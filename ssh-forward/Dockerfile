FROM debian

RUN apt-get update && apt-get install -y ssh && rm -rf /var/lib/apt/lists/*
RUN set -eux; \
    apt-get update; \
    apt-get install -y gosu iproute2; \
    rm -rf /var/lib/apt/lists/*; \
    gosu nobody true

ENV USERNAME=docker-user
ENV GROUPNAME=docker-group
ENV UID=1000
ENV GID=1000
ENV LOCAL_PORT=22022
ENV REMOTE_PORT=22022
ENV REMOTE_HOST=

COPY ./entrypoint.sh /entrypoint.sh
COPY ./healthcheck.sh ./healthcheck.sh

ENTRYPOINT ["/entrypoint.sh"]
HEALTHCHECK --interval=600s --timeout=60s CMD /healthcheck.sh
