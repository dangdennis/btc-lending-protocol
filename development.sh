# only run this script cmd by cmd. it's just instructional steps

#1
docker compose up
#2
dfx start --clean
#3
dfx deploy btc --no-wallet
#4
docker-compose exec bitcoind bitcoin-cli -conf=/conf/bitcoin.conf createwallet mywallet
#5
export BTC_ADDRESS=$(docker-compose exec bitcoind bitcoin-cli -conf=/conf/bitcoin.conf getnewaddress | tr -d '\r')
#6
docker-compose exec bitcoind bitcoin-cli -conf=/conf/bitcoin.conf generatetoaddress 101 $BTC_ADDRESS
#7
dfx deploy btc-example-rust --no-wallet --argument "(record { bitcoin_canister_id = principal \"$(dfx canister id btc)\" })" --mode=reinstall
#8
export CANISTER_BTC_ADDRESS=mmdoAzumgjbvAJjVGg7fkQmtvDNFd2wjjH
#9
docker-compose exec bitcoind bitcoin-cli -conf=/conf/bitcoin.conf sendtoaddress $CANISTER_BTC_ADDRESS 10 "" "" true true null "unset" null 1.1

# helpers
docker-compose exec bitcoind bitcoin-cli -conf=/conf/bitcoin.conf getbalance
docker-compose exec bitcoind bitcoin-cli -conf=/conf/bitcoin.conf generatetoaddress 1 $BTC_ADDRESS
dfx canister call btc-example-rust btc_address
dfx canister call btc-example-rust balance  
dfx canister call btc-example-rust get_utxos
dfx canister call btc-example-rust send "(1_0000_0000, \"$CANISTER_BTC_ADDRESS\")"