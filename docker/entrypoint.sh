#! /bin/bash

set -euo pipefail

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

cat $script
password_file="password.txt"
printf "password" >"$password_file"

etn-sc js --dev --datadir /opt/data --password "$password_file" "$script"

etn-sc --dev --datadir /opt/data \
    --http --http.addr 0.0.0.0 --http.port 8545 --http.api eth,web3,net \
    --http.corsdomain "${CORS_HOSTS:-}" \
    --ws --ws.addr 0.0.0.0 --ws.port 8546 \
    --password "$password_file"
