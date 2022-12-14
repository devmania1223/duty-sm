# echo 'addUserToAdminList'

# erdpy --verbose contract call  --chain="D" erd1qqqqqqqqqqqqqpgqrsqdthhs7nn5h6r3xqxkp5jajnm3rcfcuugqapeu8l --pem="../../wallet-owner.pem" --gas-limit=8000000 --function="addUserToAdminList" --arguments erd1p467yn4jzn88le3m2drlsqynk6nesn5l8dd08nuxk6qy4mvduugqqkmmpz  --recall-nonce --send --proxy="https://devnet-gateway.elrond.com"


# echo 'setMaxNftsPerTransaction'

# erdpy --verbose contract call  --chain="D" erd1qqqqqqqqqqqqqpgqrsqdthhs7nn5h6r3xqxkp5jajnm3rcfcuugqapeu8l --pem="../../wallet-owner.pem" --gas-limit=8000000 --function="setMaxNftsPerTransaction" --arguments 1000  --recall-nonce --send --proxy="https://devnet-gateway.elrond.com"

contractAddr=erd1qqqqqqqqqqqqqpgqgexlna9fwu6mmd8e0k2f5hucjjj86aw3uugq98r4wc

echo 'addCollection'
# not working
# erdpy --verbose contract call  --chain="D" ${contractAddr} --pem="../../wallet-owner.pem" --gas-limit=80000000  --function="ESDTTransfer" --arguments    str:EGLD 50000000000000000 "str:addCollection" "str:FirstCollection________________________________" "str:FirstCollection21" "str:png" 10  100000000 200000000  str:EGLD "str:FirstToken" "str:FIRST" 0 "str:TAG" 5  1 --recall-nonce --send --proxy="https://devnet-gateway.elrond.com" 

# erdpy --verbose contract call  --chain="D" ${contractAddr} --pem="../../wallet-owner.pem" --gas-limit=80000000 --function="addCollection" --arguments   "str:FirstCollection_______________________________" "str:FirstCollectionId" "str:png" 10  100000000 20000000000000  str:EGLD "str:FirstToken" "str:FIRST" 0 "str:TAG" 5 50000000000000000 --recall-nonce --send --proxy="https://devnet-gateway.elrond.com"  --value=50000000000000000

echo 'mintNft'

erdpy --verbose contract call  --chain="D" ${contractAddr} --pem="../../wallet-owner.pem" --gas-limit=80000000 --function="mintNft" --arguments  "str:FirstCollectionId" 4 --recall-nonce --send --proxy="https://devnet-gateway.elrond.com"  --value=200000000000000000