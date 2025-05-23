# docker-in-docker
running docker in docker


# run in privillaged

Below is the building of the image:

```
docker build -t rust-dind .
```

```
docker run -d --privileged --name rust-dind-test rust-dind

docker run -d --privileged \
  --name rust-dind-test \
  -v dind-data:/var/lib/docker \        # inner daemon’s data
  -e DOCKER_DRIVER=overlay2 \           # tell dockerd to use overlay2
  rust-dind                             # ← the image you built

docker run -d --privileged \
  --name rust-dind-test \
  -v dind-data:/var/lib/docker \
  -e DOCKER_DRIVER=overlay2 \
  rust-dind
```

we run it below:

```
docker exec -it rust-dind-test bash
```

inside we check the docker binding with the following:

```
docker exec -it rust-dind docker info --format \
  'Driver={{.Driver}}, Data-root={{.DockerRootDir}}'
```


```
# inside your existing Dockerfile that already has Docker CE
RUN apt-get update && apt-get install -y fuse-overlayfs && rm -rf /var/lib/apt/lists/*

-e IPTABLES=/usr/sbin/iptables-legacy \
apt-get install -y fuse-overlayfs iptables-legacy
```


```
# Dockerfile only installs the CLI, not the daemon
FROM rust:1.78-bookworm
RUN apt-get update && apt-get install -y docker-ce-cli fuse-overlayfs

# run it
docker run --rm -it \
  -v /var/run/docker.sock:/var/run/docker.sock \
  myrust-dockercli bash

```