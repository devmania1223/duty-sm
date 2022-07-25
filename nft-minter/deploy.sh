erdpy contract build

# deploy by project 

#erdpy --verbose contract deploy --chain="D" --project=esdt-nft-marketplace --pem="../wallet-owner.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send --arguments "1000"

# deploy by wasm 

#erdpy --verbose contract deploy --chain="D" --bytecode "output/esdt-nft-marketplace.wasm" --pem="../../wallet-owner.pem" --gas-limit=80000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send --arguments "1000"


erdpy --verbose contract deploy --chain="D" --bytecode "output/nft-minter.wasm" --pem="../../wallet-owner.pem" --gas-limit=600000000 --proxy="https://devnet-gateway.elrond.com" --recall-nonce --send --arguments "str:test" "erd1p467yn4jzn88le3m2drlsqynk6nesn5l8dd08nuxk6qy4mvduugqqkmmpz" "erd1p467yn4jzn88le3m2drlsqynk6nesn5l8dd08nuxk6qy4mvduugqqkmmpz" 1000

# erdpy contract test

# erdpy contract test --wildcard view_functions.scen.json

# cargo test