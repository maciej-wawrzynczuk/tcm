IMAGE_NAME := "my-ssh"

[default]
help:
    @just --list

build-ssh-image:
    podman build -t {{IMAGE_NAME}} tests-fixtures/

run-ssh:
    podman run --rm \
        -p 12322:22 \
        -e PUB_KEY="$(cat $HOME/.ssh/id_ed25519.pub)" \
        {{IMAGE_NAME}}
