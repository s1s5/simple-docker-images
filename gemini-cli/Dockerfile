# docker build . -t s1s5/gemini-cli --push
# alias gemini='docker run -t -i -e GEMINI_API_KEY=${GEMINI_API_KEY} -e HOME=/home/dev -v `pwd`:/work -w /work -u `id -u`:`id -g` -v ${HOME}/.gemini:/home/dev/.gemini s1s5/gemini-cli'
FROM node:24

RUN npx playwright install-deps
RUN npm install -g @google/gemini-cli

RUN apt update \
    && apt install -y build-essential clang lld pkg-config libssl-dev git \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# install mold:https://github.com/rui314/mold
ENV mold_version=2.40.2
RUN cd /tmp && \
    wget https://github.com/rui314/mold/releases/download/v${mold_version}/mold-${mold_version}-x86_64-linux.tar.gz && \
    tar zxf mold-${mold_version}-x86_64-linux.tar.gz && \
    cp -r mold-${mold_version}-x86_64-linux/* /usr/local/ && \
    rm -rf /tmp/mold-${mold_version}-x86_64-linux

ENV PATH="/work/.venv/bin:/home/dev/.cargo/bin:/home/dev/.rye/shims:/usr/local/python/bin:${PATH}"
# ENV PATH="/work/.venv/bin:/home/dev/.cargo/bin:/usr/local/python/bin:${PATH}"
ENV CARGO_TARGET_DIR="./target/gemini"
# ENV PYTHONPATH="/work:/work/.venv/lib/python3.13/site-packages/:/work/.venv/lib/python3.12/site-packages/:/work/.venv/lib/python3.11/site-packages/"

CMD ["gemini"]

