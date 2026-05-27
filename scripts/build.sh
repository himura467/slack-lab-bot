#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR=$(cd "$(dirname "$0")"/..; pwd)

cd "$ROOT_DIR"
cargo lambda build --release --x86-64
rm -f slack-lab-bot.zip
zip -j slack-lab-bot.zip target/lambda/slack-lab-bot/bootstrap
