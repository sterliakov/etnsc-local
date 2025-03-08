#! /bin/bash

set -euo pipefail

data_mount=/opt/data

script=init.js
cat <<"EOF" >"$script"
    const accounts = [
EOF

while read -r info; do
    if [ -z "${info}" ]; then
        break
    fi
    IFS=":" read -r -a split <<<"$info"
    privkey=${split[0]}
    amount=${split[1]}
    if [[ $privkey == 0x* ]]; then
        printf 'Private key %s must not start with 0x.\n' "$privkey" >&2
        exit 1
    fi
    printf '["%s", %d],\n' "$privkey" "$amount" >>"$script"
done <<<"${ACCOUNTS:-}"

cat <<"EOF" >>"$script"
    ];
    for (const [pk, amount] of accounts) {
        try {
            const address = web3.personal.importRawKey(pk, 'password');
            eth.sendTransaction({
                from: eth.coinbase,
                to: address,
                value: web3.toWei(amount),
            });
            console.log(`Seeded address ${address} with ${amount}.`);
        } catch (err) {
            if (err.toString().includes('account already exists')) {
                console.log(`Private key 0x${pk} already seeded.`);
            } else {
                throw err;
            }
        }
    }
EOF

password_file="password.txt"
printf "password" >"$password_file"

etn-sc js --dev --datadir "$data_mount" --password "$password_file" "$script"
touch /ready.txt

etn-sc --dev --datadir "$data_mount" \
    --http --http.addr 0.0.0.0 --http.port 8545 \
    --http.api "${HTTP_APIS:-eth,web3,net}" \
    --http.corsdomain "${HTTP_CORS_HOSTS:-${CORS_HOSTS:-}}" \
    --ws --ws.addr 0.0.0.0 --ws.port 8546 \
    --ws.api "${WS_APIS:-eth,web3,net}" \
    --ws.origins "${WS_CORS_HOSTS:-${CORS_HOSTS:-}}" \
    --password "$password_file" \
    "$@"
