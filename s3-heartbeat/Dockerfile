FROM python:3.12-alpine

ENV PYTHONUNBUFFERED=1

RUN mkdir -p /opt
WORKDIR /opt

COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt

COPY main.py ./

ENTRYPOINT ["python", "main.py"]
