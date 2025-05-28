docker build -f Dockerfile.pod -t rust-bookworm-podman .
docker run --privileged \                               
       --name rust-tests \
       -v rust_podman_storage:/var/lib/containers \
       -it rust-bookworm-podman


docker run --privileged \                               
       --name rust-tests \
       -v rust_podman_storage:/var/lib/containers \
       -it rust-bookworm-podman


docker run --privileged --name rust-tests -v rust_podman_storage:/var/lib/containers -it rust-bookworm-podman