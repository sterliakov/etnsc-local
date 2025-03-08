# /// script
# requires-python = ">=3.12"
# dependencies = ["web3", "eth_account"]
# ///
from __future__ import annotations

import time

from eth_account import Account
from web3 import Web3
from web3.exceptions import TransactionNotFound

NODE_URL = "http://localhost:8545"
CHAIN_ID = 1337

# Pre-seeded account found in default compose file.
SENDER_PRIVKEY = "0xafdfd9c3d2095ef696594f6cedcae59e72dcd697e2a7521b1578140422a4f890"
SENDER_ADDRESS = Account.from_key(SENDER_PRIVKEY).address
RECIPIENT_ADDRESS = "0xA1399ad5E35Fa5e1E7924882b2Ed53B198438c43"


def main() -> None:
    web3 = Web3(Web3.HTTPProvider(NODE_URL))
    assert web3.is_connected(), "Node is not live"

    nonce = web3.eth.get_transaction_count(SENDER_ADDRESS)
    tx = {
        "chainId": CHAIN_ID,
        "nonce": nonce,
        "to": RECIPIENT_ADDRESS,
        "value": web3.to_wei(1, "ether"),
        "gas": 2_000_000,
        "gasPrice": web3.to_wei(50, "gwei"),
    }
    signed_tx = web3.eth.account.sign_transaction(tx, SENDER_PRIVKEY)
    tx_hash = web3.eth.send_raw_transaction(signed_tx.raw_transaction)

    for _ in range(10):
        try:
            receipt = web3.eth.get_transaction_receipt(tx_hash)
        except TransactionNotFound:
            print("Transaction not yet mined, waiting...")
            time.sleep(2)
        else:
            assert receipt.status == 1, "Transaction failed"
            break
    else:
        raise RuntimeError("Transaction was not mined.")

    balance = web3.eth.get_balance(RECIPIENT_ADDRESS)
    assert balance == web3.to_wei(1, "ether"), (
        f"Unexpected recipient balance: {balance}"
    )
    print("Success!")


if __name__ == "__main__":
    main()
