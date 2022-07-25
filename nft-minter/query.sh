contractAddr=erd1qqqqqqqqqqqqqpgqlmtflxcs94zkst8sah07xjwl0q6zvedyuugq02a8a8

echo 'getAccumulatedMintPayments'

erdpy --verbose contract query ${contractAddr} --function="getAccumulatedMintPayments" --proxy="https://devnet-gateway.elrond.com"

echo 'getAccumulatedRoyalties'

erdpy --verbose contract query ${contractAddr} --function="getAccumulatedRoyalties" --proxy="https://devnet-gateway.elrond.com"

echo 'getAllBrandsInfo'

erdpy --verbose contract query ${contractAddr} --function="getAllBrandsInfo" --proxy="https://devnet-gateway.elrond.com"

echo 'getBrandInfo'

erdpy --verbose contract query ${contractAddr} --function="getBrandInfo" --proxy="https://devnet-gateway.elrond.com"

echo 'getCollectionsCategory'

erdpy --verbose contract query ${contractAddr} --function="getCollectionsCategory" --proxy="https://devnet-gateway.elrond.com"

echo 'getMaxNftsPerTransaction'

erdpy --verbose contract query ${contractAddr} --function="getMaxNftsPerTransaction" --proxy="https://devnet-gateway.elrond.com"

echo 'getMintPaymentsClaimAddress'

erdpy --verbose contract query ${contractAddr} --function="getMintPaymentsClaimAddress" --proxy="https://devnet-gateway.elrond.com"

echo 'getMintWhitelist'

erdpy --verbose contract query ${contractAddr} --function="getMintWhitelist" --proxy="https://devnet-gateway.elrond.com"

echo 'getNftTokenIdForBrand'

erdpy --verbose contract query ${contractAddr} --function="getNftTokenIdForBrand" --proxy="https://devnet-gateway.elrond.com"

echo 'getPriceForTier'

erdpy --verbose contract query ${contractAddr} --function="getPriceForTier" --arguments "str:FirstBrand"  --proxy="https://devnet-gateway.elrond.com"

echo 'getRegisterdCollectionHashes'

erdpy --verbose contract query ${contractAddr} --function="getRegisterdCollectionHashes" --proxy="https://devnet-gateway.elrond.com"

echo 'getRegisteredBrands'

erdpy --verbose contract query ${contractAddr} --function="getRegisteredBrands" --proxy="https://devnet-gateway.elrond.com"

echo 'getRoyaltiesClaimAddress'

erdpy --verbose contract query ${contractAddr} --function="getRoyaltiesClaimAddress" --proxy="https://devnet-gateway.elrond.com"



