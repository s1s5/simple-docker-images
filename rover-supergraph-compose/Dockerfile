FROM golang:1.18

ENV PYTHONUNBUFFERED=1
ENV PIP_NO_CACHE_DIR=on

RUN apt update && apt install -y python3-pip
RUN pip3 install --upgrade pip && pip3 install watchfiles PyYAML click

ADD https://rover.apollo.dev/nix/latest /opt/
RUN chmod +x /opt/latest && /bin/sh /opt/latest && cp /root/.rover/bin/rover /bin/rover && chmod 777 /bin/rover

WORKDIR /opt
COPY main.py ./

RUN mkdir -p /home && chmod 1777 /home
ENV HOME /home
VOLUME /home

ENTRYPOINT ["python3", "main.py"]
