
```
docker build . -t s1s5/ssh-forward

docker run --rm -e USERNAME=`id -n -u` -e GROUPNAME=`id -n -g` -e UID=`id -u` -e GID=`id -g` -v ~/.ssh:/home/`id -n -u`/.ssh -e LOCAL_PORT=22022 -e REMOTE_PORT=22022 -e REMOTE_HOST=some-host --name port-forward-test --network host s1s5/ssh-forward
```
