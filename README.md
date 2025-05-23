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
