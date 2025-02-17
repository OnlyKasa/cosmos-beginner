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
docker run --rm -it -e CHAIN_ID=wasmd-testnet -e MONIKER=node01 -e PASSWORD=123456 --mount type=bind,source=$(pwd)/template,target=/root cosmwasm/wasmd:latest sh setup_wasmd.sh wasm1lhfl2dzxt4tpl0y0wlw68dryq3a0jqwlku56c0

docker run --rm -it -e PASSWORD=123456 --mount type=bind,source=$(pwd)/template,target=/root cosmwasm/wasmd:latest sh run_wasmd.sh wasm1lhfl2dzxt4tpl0y0wlw68dryq3a0jqwlku56c0
```
docker run --rm -it -p 26657:26657 -p 26656:26656 -p 9090:9090 --mount type=bind,source=$(pwd)/template,target=/template --mount type=volume,source=wasmd_data,target=/root cosmwasm/wasmd:latest ./docker/run_wasmd.sh /template
```

```
    update gas limit
```