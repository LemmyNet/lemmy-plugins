#!/usr/bin/env bash
# IMPORTANT NOTE: this script does not use the normal LEMMY_DATABASE_URL format
#   it is expected that this script is called by run.sh script.
set -e

export LEMMY_TEST_FAST_FEDERATION=1 # by default, the persistent federation queue has delays in the scale of 30s-5min
export RUST_LOG=extism=info,lemmy_api_common=info,error

for INSTANCE in lemmy_alpha lemmy_beta; do
  echo "DB URL: ${LEMMY_DATABASE_URL} INSTANCE: $INSTANCE"
  psql "${LEMMY_DATABASE_URL}/lemmy" -c "DROP DATABASE IF EXISTS $INSTANCE"
  echo "create database"
  psql "${LEMMY_DATABASE_URL}/lemmy" -c "CREATE DATABASE $INSTANCE"
done

if [ -z "$DO_WRITE_HOSTS_FILE" ]; then
  if ! grep -q lemmy-alpha /etc/hosts; then
    echo "Please add the following to your /etc/hosts file, then press enter:

      127.0.0.1       lemmy-alpha
      127.0.0.1       lemmy-beta"
    read -p ""
  fi
else
  for INSTANCE in lemmy-alpha lemmy-beta; do
    echo "127.0.0.1 $INSTANCE" >>/etc/hosts
  done
fi

LOG_DIR=tests/log
mkdir -p $LOG_DIR

echo "start alpha"
LEMMY_CONFIG_LOCATION=./config/lemmy_alpha.hjson \
  LEMMY_DATABASE_URL="${LEMMY_DATABASE_URL}/lemmy_alpha" \
  ./lemmy_server >$LOG_DIR/lemmy_alpha.out 2>&1 &

echo "start beta"
LEMMY_CONFIG_LOCATION=./config/lemmy_beta.hjson \
  LEMMY_DATABASE_URL="${LEMMY_DATABASE_URL}/lemmy_beta" \
  ./lemmy_server >$LOG_DIR/lemmy_beta.out 2>&1 &

echo "wait for all instances to start"
sleep 5;
cat $LOG_DIR/lemmy_alpha.out
cat $LOG_DIR/lemmy_beta.out
while [[ "$(curl -s -o /dev/null -w '%{http_code}' 'lemmy-alpha:8541/api/v4/site')" != "200" ]]; do sleep 1; done
echo "alpha started"
while [[ "$(curl -s -o /dev/null -w '%{http_code}' 'lemmy-beta:8551/api/v4/site')" != "200" ]]; do sleep 1; done
echo "beta started"
