# custom images
## dropbox-upload
upload file to dropbox
- `docker run --rm -i s1s5/dropbox-upload python main.py -t dropbox_token -d /dropbox/path < filename`

## dropbox-download
download file from dropbox
- `docker run --rm -i s1s5/dropbox-download python main.py -t dropbox_token -s /dropbox/path > filename`

## s3-download-to-tar
Download from s3 and tar all files whose path starts with prefix
- `docker run --rm -i s1s5/s3-download-to-tar python main.py --base-url s3://bucket/prefix/ --aws-access-key-id AKIA*** --aws-secret-access-key *** > ./out.tar`

## gpg-encrypt-asym
- ``` docker run --rm -v `pwd`:/work -w /work -t -i s1s5/gpg-encrypt-asym -i a.txt -o a.txt.gpg -k pub.key ```

### export public key
- `gpg --armor --export-secret-keys KEY_ID > pub.key`

# other images
## postgres backup
- ```docker run --rm -i postgres /bin/sh -c "pg_dump -d `echo '$(DATABASE_URL)' | sed -e 's/^psql:/postgres:/' -` > filename"```

## gzip
- ```docker run --rm -i busybox /bin/sh -c "gzip -c < input > gzipped" ```

## aws upload file
- ```docker run --rm -i --entrypoint=/bin/sh amazon/aws-cli -c "cat file | aws s3 cp - s3://bucket/path"```
