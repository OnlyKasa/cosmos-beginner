docker build -t cosmwasm/wasmd:latest .

### add new key 
docker run --rm -it --mount type=bind,source=$(pwd)/template,target=/root cosmwasm/wasmd:latest wasmd keys list --keyring-backend test


docker run --rm -it --mount type=bind,source=$(pwd)/template,target=/root cosmwasm/wasmd:latest wasmd keys add admin1 --keyring-backend test

```
- address: wasm1lhfl2dzxt4tpl0y0wlw68dryq3a0jqwlku56c0
  name: admin1
  pubkey: '{"@type":"/cosmos.crypto.secp256k1.PubKey","key":"A8E1KenOGDrmEKGtC656aBHAGr3iD+3AyoYGU7/AqZdV"}'
  type: local
```

#### run node


```

 docker run --rm -it -p 26657:26657 -p 26656:26656 -p 1317:1317 -v "C:\Users\quang\OneDrive\Documents\cosmos-beginner\sdk\wasmd\template:/root" cosmwasm/wasmd:latest  wasmd keys add validator


  docker run --rm -it -p 26657:26657 -p 26656:26656 -p 1317:1317 -v "C:\Users\quang\OneDrive\Documents\cosmos-beginner\sdk\wasmd\template:/root" cosmwasm/wasmd:latest wasmd init --chain-id wasmd-testnet node01

docker run --rm -it -p 26657:26657 -p 26656:26656 -p 1317:1317 -v "C:\Users\quang\OneDrive\Documents\cosmos-beginner\sdk\wasmd\template:/root" cosmwasm/wasmd:latest  wasmd genesis add-genesis-account validator "90000000000stake,90000000000ucosm"



 docker run --rm -it -p 26657:26657 -p 26656:26656 -p 1317:1317 -v "C:\Users\quang\OneDrive\Documents\cosmos-beginner\sdk\wasmd\template:/root" cosmwasm/wasmd:latest wasmd genesis gentx validator "9000000000stake" --chain-id="wasmd-testnet" --amount="9000000000stake"


 docker run --rm -it -p 26657:26657 -p 26656:26656 -p 1317:1317 -v "C:\Users\quang\OneDrive\Documents\cosmos-beginner\sdk\wasmd\template:/root" cosmwasm/wasmd:latest wasmd genesis collect-gentxs

```

# update gas limit before

```
docker run --rm -it -p 26657:26657 -p 26656:26656 -p 1317:1317 -v "C:\Users\quang\OneDrive\Documents\cosmos-beginner\sdk\wasmd\template:/root" cosmwasm/wasmd:latest wasmd start --rpc.laddr tcp://0.0.0.0:26657 --trace
```


docker run --rm -it -p 26657:26657 -p 26656:26656 -p 1317:1317 -v "C:\Users\quang\OneDrive\Documents\cosmos-beginner\sdk\wasmd\template:/root" cosmwasm/wasmd:latest cat /root/.wasmd/config/genesis.json



