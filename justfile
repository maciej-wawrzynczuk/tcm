IMAGE_NAME := "my-ssh"

[default]
help:
    @just --list

build-ssh-image:
    podman build -t {{IMAGE_NAME}} tests-fixtures/

run-ssh:
    podman run {{IMAGE_NAME}}
