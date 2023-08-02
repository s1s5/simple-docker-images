FROM alpine:20230329

RUN apk --no-cache add bash gnupg haveged tini bash
COPY encrypt.sh /opt/encrypt.sh
RUN chmod +x /opt/encrypt.sh

ENV HOME=/tmp

ENTRYPOINT ["/sbin/tini", "--", "/opt/encrypt.sh"]


