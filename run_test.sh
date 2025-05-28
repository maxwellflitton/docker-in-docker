
# export PATH=/usr/local/cargo/bin:/usr/local/rustup/bin:$PATH

podman load -q -i /opt/images/localstack.tar
podman run -d --name localstack -p 4566:4566 localhost/latest:latest



cargo test
