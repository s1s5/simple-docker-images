# custom images
# dropbox-upload
upload file to dropbox
- `docker run --rm -i s1s5/dropbox-upload -t dropbox_token -d /dropbox/path < filename`
- `tar czvf - -C directory ./ | docker run --rm -i s1s5/dropbox-upload -t dropbox_token -d /dropbox-path.tar.gz`

# dropbox-download
download file from dropbox
- `docker run --rm -i s1s5/dropbox-download -t dropbox_token -s /dropbox/path > filename`

# dropbox-heartbeat

# s3-download-to-tar
Download from s3 and tar all files whose path starts with prefix
- `docker run --rm -i s1s5/s3-download-to-tar --base-url s3://bucket/prefix/ --aws-access-key-id AKIA*** --aws-secret-access-key *** > ./out.tar`

# gpg-encrypt-asym
encrypt data with public key
- ``` docker run --rm -u `id -u`:`id -g` -v `pwd`:/work -w /work -t -i s1s5/gpg-encrypt-asym -i a.txt -o a.txt.gpg -k pub.key ```

## export public key
how to export public key
- `gpg --armor --export KEY_ID > pub.key`

how to export private key
- `gpg --armor --export-secret-keys KEY_ID > pub.key`

## decrypt file
- `gpg --output DRCRYPTED_FILE --decrypt ENCRYPTED_FILE`

# gpg-encrypt-sym
encrypt data with AES256
- ``` docker run --rm -u `id -u`:`id -g` -v `pwd`:/work -w /work -t -i s1s5/gpg-encrypt-sym -i a.txt -o a.txt.gpg -k complex-password ```

## decrypt
decrypt data
- ``` docker run --rm -u `id -u`:`id -g` -v `pwd`:/work -w /work -t -i --entrypoint /opt/decrypt.sh s1s5/gpg-encrypt-sym -i a.txt.gpg -o a.txt.gpg.decrypted -k complex-password ```

# gql-schema-dumper
export graphql schema from url.
- ```docker run --rm -t -i s1s5/gql-schema-dumper --url http://graphql.example.com/graphql --output /tmp/schema.graphql --watch /work```

# rover-supergraph-compose
create supergraph schema for apollo router
- ```docker run --rm -t -i -e APOLLO_ELV2_LICENSE=accept -e APOLLO_TELEMETRY_DISABLED=1 -v `pwd`/federation/:/work s1s5/rover-supergraph-compose --config /work/supergraph-config.yml  --output /work/supergraph.graphql``

# dind-atlas
run https://atlasgo.io/ in Docker

# http-debugger
cache all accessed file, and make it editable locally.

## usage
- create self signed key and certificate
- register root-ca.crt to your browser
- run docker or run with cargo
- access with browser
- [detail](htt-debugger/readme.md)


# http-echo-logger
log incomming message

# musl
create musl alias

# node-watchfiles
node + python watchfiles

# rust-dev
docker for rust development

```yaml
# docker-compose.yml example
services:
  coplan:
    image: s1s5/rust-dev:1.72
    working_dir: /usr/src
    command:
      - cargo
      - watch
      - -s
      - 'mold -run cargo run'
    stop_grace_period: 5s
    environment:
      RUST_BACKTRACE: "1"
      RUST_LOG: debug
      USER_ID: ${USER_ID}  # set your user id
      GROUP_ID: ${GROUP_ID}  # set your group id
    volumes:
      - ./:/usr/src
```

# s3-download-latest
download last modified files

- docker run --rm s1s5/s3-download-latest s3://<bucket_name>/<prefix>

# ssh-client

# ssh-forward
create port forward local -> remote


# ssh-rev
create port forward remote -> local

```
docker run -d --restart=always -e USERNAME=`id -n -u` -e GROUPNAME=`id -n -g` -e UID=`id -u` -e GID=`id -g` -e LOCAL_PORT=22 -e REMOTE_PORT=22022 -e REMOTE_HOST=gateway -v ~/.ssh:/home/`id -n -u`/.ssh -v /tmp:/tmp --name gateway-rev --network host s1s5/ssh-rev
```

# tcp-logger
log all inbound/outbound message.

```
docker run --rm --network host s1s5/tcp-logger --bind 0.0.0.0:8081 --server 127.0.0.1:8080
```

# other images memo
## postgres backup
- DATABASE_URL=psql://user:password@host:port/db
- ```docker run --rm -i postgres /bin/sh -c "pg_dump -d `echo '$(DATABASE_URL)' | sed -e 's/^psql:/postgres:/' -` > filename"```

## gzip
gzip
- ```docker run --rm -i busybox /bin/sh -c "gzip -c < input > gzipped" ```

## aws upload file
upload to aws
- ```docker run --rm -i --entrypoint=/bin/sh amazon/aws-cli -c "cat file | aws s3 cp - s3://bucket/path"```

## aws download file
- `docker run --rm -i amazon/aws-cli s3 cp s3://bucket/path - > ./out.txt`
