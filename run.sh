#!/usr/bin/env bash
set -e

if [ ! -f lemmy_server ]; then
  wget "https://github.com/LemmyNet/lemmy/releases/download/1.0.0-beta.1/lemmy_server.gz"
  gunzip lemmy_server.gz -f
  chmod +x lemmy_server
fi

DANGER_PLUGIN_SKIP_HASH_CHECK=1 ./lemmy_server
