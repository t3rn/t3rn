# ETH2 Tests

Testnet consist of:
- execution layer ([ganache](https://github.com/trufflesuite/ganache))
- consensus layer ([lighthouse](https://github.com/sigp/lighthouse)) consisting of 4 beacon nodes and 80 validators
- optional presentation layer with [blockscout](https://www.blockscout.com/) 
- tests with [hardhat](https://hardhat.org/)


## Running Local Testnet

### Requirements

The scripts require `ganache`, `lcli` and `lighthouse` to be installed on `PATH`. 

Compile lcli  with:
```bash
git clone https://github.com/sigp/lighthouse && cd lighthouse
make
make install-lcli
```

Get ganache (v7.8.0 at time of writing)
```
npm i -g ganache
```

Get recent lighthouse (v4.0.2 at time of writing) release https://github.com/sigp/lighthouse/releases

### Starting the testnet

Modify `vars.env` as desired.

```bash
./start_local_testnet.sh
```

This will start a local eth1 ganache server plus boot node along with `BN_COUNT`
number of beacon nodes and `VC_COUNT` validator clients.

The `start_local_testnet.sh` script takes three options `-v VC_COUNT`, `-d DEBUG_LEVEL` and `-h` for help.
The options may be in any order or absent in which case they take the default value specified.
- VC_COUNT: the number of validator clients to create, default: `BN_COUNT`
- DEBUG_LEVEL: one of { error, warn, info, debug, trace }, default: `info`


### Stopping the testnet

This is not necessary before `start_local_testnet.sh` as it invokes `stop_local_testnet.sh` automatically.
```bash
./stop_local_testnet.sh
```

### Manual creation of local testnet

These scripts are used by ./start_local_testnet.sh and may be used to manually

Start a local eth1 ganache server
```bash
./ganache_test_node.sh
```

Assuming you are happy with the configuration in `vars.env`, deploy the deposit contract, make deposits,
create the testnet directory, genesis state and validator keys with:

```bash
./setup.sh
```

Generate bootnode enr and start a discv5 bootnode so that multiple beacon nodes can find each other
```bash
./bootnode.sh
```

Start a beacon node:

```bash
./beacon_node.sh <DATADIR> <NETWORK-PORT> <HTTP-PORT> <OPTIONAL-DEBUG-LEVEL>
```
e.g.
```bash
./beacon_node.sh $HOME/.lighthouse/local-testnet/node_1 9000 8000
```

In a new terminal, start the validator client which will attach to the first
beacon node:

```bash
./validator_client.sh <DATADIR> <BEACON-NODE-HTTP> <OPTIONAL-DEBUG-LEVEL>
```
e.g. to attach to the above created beacon node
```bash
./validator_client.sh $HOME/.lighthouse/local-testnet/node_1 http://localhost:8000
```

You can create additional beacon node and validator client instances with appropriate parameters.

### Additional Info

#### Adjusting number and distribution of validators
The `VALIDATOR_COUNT` parameter is used to specify the number of insecure validator keystores to generate and make deposits for.
The `BN_COUNT` parameter is used to adjust the division of these generated keys among separate validator client instances.
For e.g. for `VALIDATOR_COUNT=80` and `BN_COUNT=4`, the validator keys are distributed over 4 datadirs with 20 keystores per datadir. The datadirs are located in `$DATADIR/node_{i}` which can be passed to separate validator client
instances using the `--datadir` parameter.

#### Starting fresh

Delete the current testnet and all related files using. Generally not necessary as `start_local_test.sh` does this each time it starts.

```bash
./stop_local_testnet.sh
./clean.sh
```

#### Updating the genesis time of the beacon state

If it's been a while since you ran `./setup` then the genesis time of the
genesis state will be far in the future, causing lots of skip slots.

Update the genesis time to now using:

```bash
./reset_genesis_time.sh
```

> Note: you probably want to just rerun `./start_local_testnet.sh` to start over
> but this is another option.

### Blockscout

It's web representation of blockchain like ethscan.
For macosx with Apple Sillicone you need to run

```
DOCKER_TAG=5.1.0 docker compose -f docker-compose-blockscout.yml up
```