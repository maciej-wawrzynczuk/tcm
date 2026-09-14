#!/bin/sh
set -eu

readonly SSH_DIR="/home/${LOCAL_USER}/.ssh"
readonly AUTHORIZED_KEYS="${SSH_DIR}/authorized_keys"

mkdir -p "${SSH_DIR}"
chown "${LOCAL_USER}" "${SSH_DIR}"
chmod 700 "${SSH_DIR}"
echo "${PUB_KEY}" >> "${AUTHORIZED_KEYS}"
chown "${LOCAL_USER}" "${AUTHORIZED_KEYS}"
chmod 600 "${AUTHORIZED_KEYS}"

exec "$@"
