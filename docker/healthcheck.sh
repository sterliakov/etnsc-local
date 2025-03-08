#! /bin/bash

set -euo pipefail

if ! [ -f /ready.txt ]; then
    echo "Not ready yet"
    exit 1
fi

curl -X POST -H "Content-Type: application/json" \
    --data '{"jsonrpc":"2.0","method":"web3_clientVersion","params":[],"id":67}' \
    http://localhost:8545
