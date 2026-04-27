TAG=$(curl -s https://hub.docker.com/v2/repositories/clux/muslrust/tags/?page_size=100 | jq -r '.results[].name' | grep -E '^[0-9]+\.[0-9]+\.[0-9]+-stable-' | sort -V | tail -n 1)

export TAG=$(curl -s https://hub.docker.com/v2/repositories/clux/muslrust/tags/?page_size=100 | jq -r '.results[].name' | grep -E '^[0-9]+\.[0-9]+\.[0-9]+-stable-' | sort -V | tail -n 1); docker build --build-arg TAG=${TAG} . -t s1s5/muslrust:${TAG}


1.94.1-stable-2026-04-07
1.94.1-stable-2026-04-11
1.94.1-stable-2026-04-13
1.94.1-stable-2026-04-15
1.95.0-stable-2026-04-16
1.95.0-stable-2026-04-18
1.95.0-stable-2026-04-20
1.95.0-stable-2026-04-22
