#!/usr/bin/env bash

set -euo pipefail

emit_json() {
  cd build-indexes
  bundle exec main.rb
}

json_to_bincode() {
  cargo run --bin pack-index
}

emit_json | jq | json_to_bincode > barohead/recipes.bincode
