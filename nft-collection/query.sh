contractAddr=erd1qqqqqqqqqqqqqpgqlmtflxcs94zkst8sah07xjwl0q6zvedyuugq02a8a8

echo 'getAccumulatedMintPayments'

erdpy --verbose contract query ${contractAddr} --function="getAccumulatedMintPayments" --proxy="https://devnet-gateway.elrond.com"

echo 'getAccumulatedRoyalties'

erdpy --verbose contract query ${contractAddr} --function="getAccumulatedRoyalties" --proxy="https://devnet-gateway.elrond.com"

echo 'getAllCollectionsInfo'

erdpy --verbose contract query ${contractAddr} --function="getAllCollectionsInfo" --proxy="https://devnet-gateway.elrond.com"

echo 'getCollectionInfo'

erdpy --verbose contract query ${contractAddr} --function="getCollectionInfo" --proxy="https://devnet-gateway.elrond.com"

echo 'getCollectionsCategory'

erdpy --verbose contract query ${contractAddr} --function="getCollectionsCategory" --proxy="https://devnet-gateway.elrond.com"

echo 'getMaxNftsPerTransaction'

erdpy --verbose contract query ${contractAddr} --function="getMaxNftsPerTransaction" --proxy="https://devnet-gateway.elrond.com"

echo 'getMintPaymentsClaimAddress'

erdpy --verbose contract query ${contractAddr} --function="getMintPaymentsClaimAddress" --proxy="https://devnet-gateway.elrond.com"

echo 'getMintWhitelist'

erdpy --verbose contract query ${contractAddr} --function="getMintWhitelist" --proxy="https://devnet-gateway.elrond.com"

echo 'getNftTokenIdForCollection'

erdpy --verbose contract query ${contractAddr} --function="getNftTokenIdForCollection" --proxy="https://devnet-gateway.elrond.com"

echo 'getPriceForTier'

erdpy --verbose contract query ${contractAddr} --function="getPriceForTier" --arguments "str:FirstCollection"  --proxy="https://devnet-gateway.elrond.com"

echo 'getRegisterdCollectionHashes'

erdpy --verbose contract query ${contractAddr} --function="getRegisterdCollectionHashes" --proxy="https://devnet-gateway.elrond.com"

echo 'getRegisteredCollections'

erdpy --verbose contract query ${contractAddr} --function="getRegisteredCollections" --proxy="https://devnet-gateway.elrond.com"

echo 'getRoyaltiesClaimAddress'

erdpy --verbose contract query ${contractAddr} --function="getRoyaltiesClaimAddress" --proxy="https://devnet-gateway.elrond.com"



