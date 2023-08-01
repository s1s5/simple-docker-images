# custom images
## dropbox-upload
upload file to dropbox
- `docker run --rm -i s1s5/dropbox-upload -t dropbox_token -d /dropbox/path < filename`
- `tar czvf - -C directory ./ | docker run --rm -i s1s5/dropbox-upload -t dropbox_token -d /dropbox-path.tar.gz`

## dropbox-download
download file from dropbox
- `docker run --rm -i s1s5/dropbox-download -t dropbox_token -s /dropbox/path > filename`

## s3-download-to-tar
Download from s3 and tar all files whose path starts with prefix
- `docker run --rm -i s1s5/s3-download-to-tar --base-url s3://bucket/prefix/ --aws-access-key-id AKIA*** --aws-secret-access-key *** > ./out.tar`

## gpg-encrypt-asym
encrypt data with public key
- ``` docker run --rm -u `id -u`:`id -g` -v `pwd`:/work -w /work -t -i s1s5/gpg-encrypt-asym -i a.txt -o a.txt.gpg -k pub.key ```

### export public key
how to export public key
- `gpg --armor --export-secret-keys KEY_ID > pub.key`

## gpg-encrypt-sym
encrypt data with AES256
- ``` docker run --rm -u `id -u`:`id -g` -v `pwd`:/work -w /work -t -i s1s5/gpg-encrypt-sym -i a.txt -o a.txt.gpg -k complex-password ```

### decrypt
decrypt data
- ``` docker run --rm -u `id -u`:`id -g` -v `pwd`:/work -w /work -t -i --entrypoint /opt/decrypt.sh s1s5/gpg-encrypt-sym -i a.txt.gpg -o a.txt.gpg.decrypted -k complex-password ```

## gql-schema-dumper
export graphql schema from url.
- ```docker run --rm -t -i s1s5/gql-schema-dumper --url http://graphql.example.com/graphql --output /tmp/schema.graphql --watch /work```

## rover-supergraph-compose
create supergraph schema for apollo router
- ```docker run --rm -t -i -e APOLLO_ELV2_LICENSE=accept -e APOLLO_TELEMETRY_DISABLED=1 -v `pwd`/federation/:/work s1s5/rover-supergraph-compose --config /work/supergraph-config.yml  --output /work/supergraph.graphql``

# other images
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
