
```
docker build . -t s1s5/ssh-client

docker run --rm -e USERNAME=`id -n -u` -e GROUPNAME=`id -n -g` -e UID=`id -u` -e GID=`id -g` -v ~/.ssh:/home/`id -n -u`/.ssh --name port-foward-test --network host s1s5/ssh-client ssh -N -L 8888:localhost:8888 some-remote-host
```
