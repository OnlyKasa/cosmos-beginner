``` 
docker build -t cosmwasm/wasmd:latest .
```
### Add new key 
```
docker run --rm -it --mount type=bind,source=$(pwd)/template,target=/root cosmwasm/wasmd:latest wasmd keys list --keyring-backend test

docker run --rm -it --mount type=bind,source=$(pwd)/template,target=/root cosmwasm/wasmd:latest wasmd keys add admin1 --keyring-backend test
```

```
- address: wasm1lhfl2dzxt4tpl0y0wlw68dryq3a0jqwlku56c0
  name: admin1
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A8E1KenOGDrmEKGtC656aBHAGr3iD+3AyoYGU7/AqZdV"}'
  type: local
```

## Run node

Path = C:\Users\${user}\OneDrive\Documents\cosmos-beginner\sdk\wasmd
```

docker run --rm -it -p 26657:26657 -p 26656:26656 -p 1317:1317 -v "C:\\cosmos-beginner\sdk\wasmd\template:/root" cosmwasm/wasmd:latest  wasmd keys add validator


docker run --rm -it -p 26657:26657 -p 26656:26656 -p 1317:1317 -v "C:\\cosmos-beginner\sdk\wasmd\template:/root" cosmwasm/wasmd:latest wasmd init --chain-id wasmd-testnet node01

docker run --rm -it -p 26657:26657 -p 26656:26656 -p 1317:1317 -v "C:\\cosmos-beginner\sdk\wasmd\template:/root" cosmwasm/wasmd:latest  wasmd genesis add-genesis-account validator "90000000000stake,90000000000ucosm"



 docker run --rm -it -p 26657:26657 -p 26656:26656 -p 1317:1317 -v "C:\\cosmos-beginner\sdk\wasmd\template:/root" cosmwasm/wasmd:latest wasmd genesis gentx validator "9000000000stake" --chain-id="wasmd-testnet" --amount="9000000000stake"


 docker run --rm -it -p 26657:26657 -p 26656:26656 -p 1317:1317 -v "C:\\cosmos-beginner\sdk\wasmd\template:/root" cosmwasm/wasmd:latest wasmd genesis collect-gentxs

```

# update gas limit before

```
docker run --rm -it -p 26657:26657 -p 26656:26656 -p 1317:1317 -v "C:\\cosmos-beginner\sdk\wasmd\template:/root" cosmwasm/wasmd:latest wasmd start --rpc.laddr tcp://0.0.0.0:26657 --trace

docker run --rm -it -p 26657:26657 -p 26656:26656 -p 1317:1317 -v "C:\\cosmos-beginner\sdk\wasmd\template:/root" cosmwasm/wasmd:latest cat /root/.wasmd/config/genesis.json
```



### deployment 

docker cp "C:\\cosmos-beginner\smart-contract\erc20\target\wasm32-unknown-unknown\release\erc20_contract.wasm" node1:/opt/erc20_contract.wasm

docker exec -it node1 wasmd tx wasm store erc20_contract.wasm --from validator --gas auto --gas-adjustment 1.5 --chain-id wasmd-testnet --yes


docker exec -it node1 wasmd query tx 28421DF6F23C43121768979FCD60D5CDF9E2EC3533AB1A4AD9293875CCA3DE12 --chain-id wasmd-testnet


## Init token
docker exec -it node1 wasmd tx wasm instantiate 3 '{"name":"MyToken","symbol":"MTK","decimals":18,"initial_supply":"1000000000000000000000000000"}' --from validator --label "My ERC-20 Token" --gas 200000 --fees 1000ucosm  --chain-id wasmd-testnet --no-admin

## Query balance
docker exec -it node1 wasmd keys list
CONTRACT_ADDRESS = wasm1hrpna9v7vs3stzyd4z3xf00676kf78zpe2u5ksvljswn2vnjp3ys8c5wp9

docker exec -it node1 wasmd query wasm contract-state smart wasm1hrpna9v7vs3stzyd4z3xf00676kf78zpe2u5ksvljswn2vnjp3ys8c5wp9 '{"BalanceOf":{"owner":"wasm12d3unjwsd0zhge0csv959erklz0klkmj7jfdu
6"}}'

docker exec -it node1 wasmd query wasm contract-state smart wasm1hrpna9v7vs3stzyd4z3xf00676kf78zpe2u5ksvljswn2vnjp3ys8c5wp9 '{"TotalSupply":{}}
