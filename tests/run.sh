#!/usr/bin/env bash
set -e

export LEMMY_DATABASE_URL=postgres://lemmy:password@localhost:5432
killall -s1 lemmy_server || true

pushd ..
./tests/prepare.sh
popd

pnpm i
pnpm test || true

killall -s1 lemmy_server || true
for INSTANCE in lemmy_alpha lemmy_beta; do
  psql "$LEMMY_DATABASE_URL" -c "DROP DATABASE $INSTANCE"
done
