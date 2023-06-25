#!/usr/bin/env bash

set -euo pipefail

cd barohead

trunk build --dist dist-release --release --public-url /barohead
rsync -r dist-release/ feyhin.cons.org.nz:/var/www/n.gen.nz/html/barohead
