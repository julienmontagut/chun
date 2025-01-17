#!/bin/bash

set -e

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     MACHINE=linux;;
    Darwin*)    MACHINE=macos;;
    *)          echo "Unsupported operating system: ${OS}"; exit 1;;
esac

# Get latest version
VERSION=$(curl -s https://api.github.com/repos/julienmontagut/chun/releases/latest | grep "tag_name" | cut -d '"' -f 4)
VERSION=${VERSION#v}  # Remove 'v' prefix

# Download and install
echo "Installing chun version ${VERSION} for ${MACHINE}..."
TEMP_DIR=$(mktemp -d)
cd "${TEMP_DIR}"

curl -LO "https://github.com/julienmontagut/chun/releases/download/v${VERSION}/chun-${MACHINE}-x86_64.tar.gz"
tar xzf "chun-${MACHINE}-x86_64.tar.gz"

# Install to /usr/local/bin if running as root, otherwise to ~/.local/bin
if [ "$(id -u)" = "0" ]; then
    INSTALL_DIR="/usr/local/bin"
else
    INSTALL_DIR="${HOME}/.local/bin"
    mkdir -p "${INSTALL_DIR}"
fi

mv chun "${INSTALL_DIR}/"
chmod +x "${INSTALL_DIR}/chun"

# Cleanup
cd - > /dev/null
rm -rf "${TEMP_DIR}"

echo "Successfully installed chun to ${INSTALL_DIR}/chun"
if [ "${INSTALL_DIR}" = "${HOME}/.local/bin" ]; then
    echo "Make sure ${INSTALL_DIR} is in your PATH"
fi 