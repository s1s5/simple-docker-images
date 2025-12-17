TAG=$(curl -s https://hub.docker.com/v2/repositories/clux/muslrust/tags/?page_size=100 | jq -r '.results[].name' | grep -E '^[0-9]+\.[0-9]+\.[0-9]+-stable-' | sort -V | tail -n 1)

export TAG=$(curl -s https://hub.docker.com/v2/repositories/clux/muslrust/tags/?page_size=100 | jq -r '.results[].name' | grep -E '^[0-9]+\.[0-9]+\.[0-9]+-stable-' | sort -V | tail -n 1); docker build --build-arg TAG=${TAG} . -t s1s5/muslrust:${TAG}
