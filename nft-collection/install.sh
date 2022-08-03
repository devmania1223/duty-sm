sudo apt-get update 
sudo apt install libncurses5 build-essential python3-pip  python3.8-venv
wget -O erdpy-up.py https://raw.githubusercontent.com/ElrondNetwork/elrond-sdk-erdpy/master/erdpy-up.py
python3 erdpy-up.py
erdpy --verbose wallet derive ./wallet-owner.pem --mnemonic

https://github.com/ElrondNetwork/sc-nft-marketplace.git
https://github.com/ElrondNetwork/sc-nft-collection-minter.git

erdpy deps install rust --overwrite
rustup default nightly


curl https://sh.rustup.rs -sSf | sh