#!/bin/bash
set -e

source .env.production

echo "Deploying ${DOCKER_USERNAME}/${DOCKER_REPOSITORY}:${IMAGE_TAG} to VPS..."

ssh root@rust-ui.com << EOF
  cd ~/dioxus-ui
  docker pull ${DOCKER_USERNAME}/${DOCKER_REPOSITORY}:${IMAGE_TAG}
  docker compose -f docker-compose.shared-server.yml up -d
  echo "Done."
EOF

echo "Deploy complete — https://dioxus.rust-ui.com"
