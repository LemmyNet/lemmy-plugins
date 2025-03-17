# Lemmy Plugins

This repository contains various example plugins for Lemmy. See the [Extism documentation](https://extism.org/docs/quickstart/plugin-quickstart) for details on how to write plugins. Lemmy-specific details and available hooks are [described in the RFC](https://github.com/LemmyNet/rfcs/pull/8/files). 

## Contributing

- Install `go`, tinygo`, `pnpm`, `postgresql`
- `cd tests`
- `./run.sh` to start test cases

## Available Examples

- `go_replace_words` Written in Go, uses `create_local_post` hook to replace specific words in post title, and reject posts with forbidden words in title
