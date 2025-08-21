# docker build . -t s1s5/gemini-cli --push
# alias gemini='docker run -t -i -e GEMINI_API_KEY=${GEMINI_API_KEY} -e HOME=/home/dev -v `pwd`:/work -w /work -u `id -u`:`id -g` -v ${HOME}/.gemini:/home/dev/.gemini s1s5/gemini-cli'
FROM node:24

RUN npm install -g @google/gemini-cli

CMD ["gemini"]
