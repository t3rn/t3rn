# Ethereum Ranger
The ranger submits each new epoch to the Sepolia light client pallet. In case of a sync committee update, the new one is submitted automatically to the Sepolia pallet.

## Setup:
1. install all dependencies by running `make install-all` from the root dir of the project
2. in the `/app` dir create a file called `.env` (it's git-ignored by default)
3. copy the contents of `.env-example.local` to the newly created `.env` file
4. (optional) if you want to run the ranger for other environments (sepolia, eth), some env vars will differ (eg. RPC endpoints, signer, intervals, etc.). For that purpose, you have to change them in the `.env` file. Examples can be seen in the `.env-example.sepolia` file

⚠️ Before running, ensure the Sepolia gateway is registered. This can be done by running `make cli-register-eth-sepl` from the project root dir


## Running:
- run `pnpm run start:dev`, to start the ranger locally

On each submission, the circuit will emit the `EpochUpdate` event, which can be observed in polkadot js

## Helm Diff

```
helm diff upgrade ethereum-ranger -n ethereum-ranger helm/ --values helm/values.yaml --set tag=8876d4f35a05280933e53464cf79447b0a67c485,repository="ghcr.io/t3rn/grandpa-ranger"
```
