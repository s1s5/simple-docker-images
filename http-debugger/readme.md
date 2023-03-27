# create self signed key and certificate

- `openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:2048 -out root-ca.key`
- `openssl req -x509 -sha256 -new -nodes -key root-ca.key -days 3650 -out root-ca.crt`

- register root-ca.crt to browser

# run with docker

```
docker run --name http-debugger --rm -u `id -u`:`id -g` --rm -p 8100:8100 -v `pwd`/certs:/certs -v /tmp/http-debugger-cache:/tmp/http-debugger-cache s1s5/http-debugger --cache /tmp/http-debugger-cache --key /certs/root-ca.key  --crt /certs/root-ca.crt
```

`cargo watch -s "mold -run cargo run -- --cache /tmp/http-debugger-cache --key ./certs/root-ca.key  --crt ./certs/root-ca.crt`
