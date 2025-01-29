#!/bin/bash
set -e

# Variables
REMOTE_HOST=${REMOTE_HOST:-"your.server.com"}
REMOTE_USER=${REMOTE_USER:-"deploy"}
REMOTE_DIR=${REMOTE_DIR:-"/var/www/anya-core"}
SSH_KEY=${SSH_KEY:-"$HOME/.ssh/id_rsa"}

echo "Deploying to $REMOTE_USER@$REMOTE_HOST:$REMOTE_DIR"

# Build the project before deployment
./scripts/build.sh

# Sync files to the remote server
rsync -avz -e "ssh -i ${SSH_KEY}" --delete ./target/release/ "$REMOTE_USER@$REMOTE_HOST:${REMOTE_DIR}/new_release"

# Perform zero-downtime deployment using symlinks
ssh -i "${SSH_KEY}" "$REMOTE_USER@$REMOTE_HOST" << EOF
  ln -sfn ${REMOTE_DIR}/new_release ${REMOTE_DIR}/current
  systemctl restart anya-core
EOF

echo "Deployment completed successfully." 