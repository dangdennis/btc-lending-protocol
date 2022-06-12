# only run this script cmd by cmd. it's just instructional steps

# if you have issues regarding brew with apple silicon, alias brew with arch -x86_64. 
# see https://www.notion.so/Bitcoin-Liquidity-Pool-93f5baa018ae4056926f13caa3e96375

#1
docker compose up
#2
dfx start --clean
#3
dfx deploy btc
#4
cargo run --features="tokio candid ic-agent garcon tonic tonic-build" --bin adapter-shim $(dfx canister id btc)
#5
docker-compose exec bitcoind bitcoin-cli -conf=/conf/bitcoin.conf createwallet mywallet
#6
export BTC_ADDRESS=$(docker-compose exec bitcoind bitcoin-cli -conf=/conf/bitcoin.conf getnewaddress | tr -d '\r')
#7
docker-compose exec bitcoind bitcoin-cli -conf=/conf/bitcoin.conf generatetoaddress 101 $BTC_ADDRESS
#8
dfx deploy protocol --argument "(record { bitcoin_canister_id = principal \"$(dfx canister id btc)\" })" --mode=reinstall
# dfx deploy protocol-rs --argument "(record { bitcoin_canister_id = principal \"$(dfx canister id btc)\" })" --mode=reinstall
#9
export CANISTER_BTC_ADDRESS=mmdoAzumgjbvAJjVGg7fkQmtvDNFd2wjjH
#10
docker-compose exec bitcoind bitcoin-cli -conf=/conf/bitcoin.conf sendtoaddress $CANISTER_BTC_ADDRESS 10 "" "" true true null "unset" null 1.1
#11
docker-compose exec bitcoind bitcoin-cli -conf=/conf/bitcoin.conf generatetoaddress 1 $BTC_ADDRESS

# helpers
docker-compose exec bitcoind bitcoin-cli -conf=/conf/bitcoin.conf getbalance
docker-compose exec bitcoind bitcoin-cli -conf=/conf/bitcoin.conf generatetoaddress 1 $BTC_ADDRESS
dfx canister call protocol btc_address
dfx canister call protocol balance  
dfx canister call protocol get_utxos
dfx canister call protocol send "(1_0000_0000, \"$CANISTER_BTC_ADDRESS\")"