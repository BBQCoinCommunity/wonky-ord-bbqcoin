# Flames of BBQCoin

ℹ️ This is a fork/based on [apezord/ord-bbqcoin](https://github.com/apezord/ord-bbqcoin)

## Key differences

‼️ DISCLAIMER: OUR CODE MAY STILL HAVE BUGS️

We included the real wonky block rewards from block 0 until block 144,999. We invite you to critically review our code in `src/epoch.rs`. We are convinced that doginals should use actual block rewards instead of a simplified version.

## API documentation
You can find the API documentation [here](openapi.yaml).
Most convenient way to view the API documentation is to use the [Swagger Editor](https://editor.swagger.io/).
You can import the `openapi.yaml` file and view the API documentation via Import URL: `https://raw.githubusercontent.com/verydogelabs/wonky-ord-bbqcoin/main/openapi.yaml`.

## TL;DR How to run

### Preqrequisites
You will have to launch your own BBQCoin node and have it fully synced. You can use the following guide to set up your own BBQCoin node:
1. Download latest version from [BBQCoin](https://github.com/bbqcoin/bbqcoin/releases) and install it.
   1. We have tested and launched the indexer with BBQCoin Core v1.14.8.
2. Follow the [installation instructions](https://github.com/bbqcoin/bbqcoin/blob/master/INSTALL.md)
   1. We started the BBQCoin Core with the following flags:
      ```shell
      bbqcoind -txindex -rpcuser=foo -rpcpassword=bar -rpcport=22555 -rpcallowip=0.0.0.0/0 -rpcbind=127.0.0.1
      ```
   2. Make sure your BBQCoin node is fully synced before starting the indexer.
   3. ‼️ **IMPORTANT** Ensure to replace `foo` and `bar` with your own username and password. **IMPORTANT** ‼️
3. Start the indexer with rpc-url pointing to your BBQCoin node and the data-dir pointing to the directory where the indexer should store its data.

```shell

### Start the ord indexer / server
```shell
export RUST_LOG=info
// Set the path to the subsidies.json and starting_sats.json files
export SUBSIDIES_PATH=/home/bqceuser/wonky-ord-bbqcoin/subsidies.json
export STARTING_SATS_PATH=/home/bqceuser/wonky-ord-bbqcoin/starting_sats.json

# ensure the data directory exists
mkdir -p /mnt/ord-node/indexer-data-main

# replace YOUR_RPC_URL with the URL of your BBQCoin node like: http://foo:bar@127.0.0.1:22555

// Start Indexing
ord --rpc-url=YOUR_RPC_URL --data-dir=/mnt/ord-node/indexer-data-main --nr-parallel-requests=16 --first-inscription-height=4609723 --first-rune-height=5084000 --index-runes --index-transactions --index-bqc20 index

// Start Indexing + Server
ord --rpc-url=YOUR_RPC_URL --data-dir=/mnt/ord-node/indexer-data-main --nr-parallel-requests=16 --first-inscription-height=4609723 --first-rune-height=5084000 --index-runes --index-transactions --index-bqc20 server
```
`--index-transactions` will store transaction data, this is currently needed for `--index-bqc20` and furthermore helps
for a better performance for the API.
`--nr-parallel-requests` will configure how many parallel requests while indexing are sent to your RPC Server - 16 is
recommended for default node settings.

With all settings enabled, the database will currently need around 400gb when fully indexed.

### Required env vars

On the root level of this repo you'll find a `subsidies.json` and `starting_sats.json` file. When starting ord you will need to set the location of these files to env variables.

Example:
If your `wonky-ord-bbqcoin` dir is `/home/bqceuser/wonky-ord-bbqcoin` then set the vars:
`SUBSIDIES_PATH=/home/bqceuser/wonky-ord-bbqcoin/subsidies.json`
and
`STARTING_SATS_PATH=/home/bqceuser/wonky-ord-bbqcoin/starting_sats.json`.

## Start the ord indexer / server in Docker
You can use a docker image to run the ord indexer / server.

### Prerequisites Docker
1. Use ubuntu linux or a similar distribution
2. Install bbqcoind and have it fully synced
   1See [BBQCoin installation instructions](#preqrequisites)
3. Install docker and docker-compose (Ubuntu)[https://docs.docker.com/engine/install/ubuntu/]
4. Clone this repository

### Build the Docker image
```shell
docker build -t verydogelabs/wonky-ord-bbqcoin .
```
### Start the ord in a docker container
```shell
docker compose up -d
```

### Stop the ord in a docker container
When stopping the ord in a container it is important to add a timeout.
If no timeout is add, the process cannot close the database properly and the next start will take ages or fail.

```shell
docker compose stop -t 600
docker compose down
```

## Original README
Please check the original [README](READMEFROMAPEZORD.md) for more information on how to run `ord` and the required env vars.
