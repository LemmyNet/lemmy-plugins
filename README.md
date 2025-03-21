# Lemmy Plugins

This repository contains various example plugins for Lemmy. See the [Extism documentation](https://extism.org/docs/quickstart/plugin-quickstart) for details on how to write plugins. Lemmy-specific details and available hooks are [described in the RFC](https://github.com/LemmyNet/rfcs/pull/8/files).

## Go: Replace Words

Uses the `create_local_post` hook to listen for newly created posts, replacing some words and rejecting posts containing specific terms. See [go-pdk readme](https://github.com/extism/go-pdk#readme) for detailed documentation.

Use the following steps to compile it:

```bash
apt install go
# install tinygo: https://tinygo.org/getting-started/install/linux/
cd plugins/go_replace_words
tinygo build -o ../go_replace_words.wasm -target wasip1 -buildmode=c-shared main.go
```

## Typescript: Push Webhook

Listens to `new_post` hook which is called after any local or remote post is created. Then calls the url configured in the [manifest](https://extism.org/docs/concepts/manifest/) (`plugins/typescript_push_webhook.json`) with the post's `ap_id`. See [js-pdk readme](https://github.com/extism/js-pdk#readme) for setup and detailed documentation.

Use the following steps to compile it:

```bash
apt install npm typescript
# use steps in js-pdk readme to install extism-js
cd plugins/typescript_push_webhook
npm install
npm run build
```

## Rust: Allowed Voters

Listens to `new_vote` hook

```bash
apt install cargo
cd plugins/rust_allowed_voters
cargo build
cp target/wasm32-unknown-unknown/debug/rust_allowed_voters.wasm ..
```

## Tests

This repository contains test cases for the plugins. To run them install `pnpm` and `postgresql`, with a database `postgres://lemmy:password@localhost:5432/lemmy`. Then compile all the plugins as described above, go into `tests` folder and execute `./run.sh`.
