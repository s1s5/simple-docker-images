FROM alpine:20220715

RUN apk --no-cache add bash gnupg haveged tini
COPY encrypt.sh /opt/encrypt.sh
RUN chmod +x /opt/encrypt.sh

ENTRYPOINT ["/sbin/tini", "--", "/opt/encrypt.sh"]
