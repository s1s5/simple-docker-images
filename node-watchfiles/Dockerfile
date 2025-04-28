# Use an official base image with both Python and Node.js installed
FROM python:3.12-slim

# Install Node.js
RUN apt-get update && \
    apt-get install -y curl && \
    curl -fsSL https://deb.nodesource.com/setup_16.x | bash - && \
    apt-get install -y nodejs && \
    apt-get clean

RUN pip install --no-cache-dir watchfiles
