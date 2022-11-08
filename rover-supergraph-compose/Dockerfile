FROM golang:1.18

ENV PYTHONUNBUFFERED=1
ENV PIP_NO_CACHE_DIR=on

RUN apt update && apt install -y python3-pip
RUN pip3 install --upgrade pip && pip3 install watchfiles PyYAML click

ADD https://rover.apollo.dev/nix/latest /opt/
RUN chmod +x /opt/latest && /bin/sh /opt/latest && ln -s /root/.rover/bin/rover /bin/rover 

WORKDIR /opt
COPY main.py ./

ENTRYPOINT ["python3", "main.py"]
