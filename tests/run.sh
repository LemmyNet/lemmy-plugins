#!/usr/bin/env bash
set -e

export LEMMY_DATABASE_URL=postgres://lemmy:password@localhost:5432
killall -s1 lemmy_server || true

# add test plugin
pushd ../plugins/go_replace_words
tinygo build -o ../go_replace_words.wasm -target wasip1 -buildmode=c-shared main.go
popd

pushd ..
./tests/prepare.sh
popd

pnpm i
pnpm test || true

killall -s1 lemmy_server || true
for INSTANCE in lemmy_alpha lemmy_beta; do
  psql "$LEMMY_DATABASE_URL" -c "DROP DATABASE $INSTANCE"
done
