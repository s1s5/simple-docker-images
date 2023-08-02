FROM alpine:20230329

RUN apk --no-cache add bash gnupg haveged tini bash

COPY encrypt.sh /opt/encrypt.sh
RUN chmod +x /opt/encrypt.sh

COPY decrypt.sh /opt/decrypt.sh
RUN chmod +x /opt/decrypt.sh

ENTRYPOINT ["/sbin/tini", "--", "/opt/encrypt.sh"]
