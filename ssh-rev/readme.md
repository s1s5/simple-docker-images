
```
docker build . -t s1s5/ssh-rev

docker run --rm -e USERNAME=`id -n -u` -e GROUPNAME=`id -n -g` -e UID=`id -u` -e GID=`id -g` -v ~/.ssh:/home/`id -n -u`/.ssh -e SSH_PORT=22022 -e SSH_HOST=some-host --name rev-test --network host s1s5/ssh-rev
```
