#!/bin/zsh

get_geth_container_id() {
  docker ps -q -f name=geth
}

mode=$1
case ${mode} in
start-geth)
  id=$(get_geth_container_id)
  if [ -n "${id}" ]; then
      echo "geth is already running[${id}]"
      exit 0
  fi

  docker run -d --name=geth -p 8545:8545 -p 8546:8546 -v ~/Ethereum/Local:/Data ethereum/client-go:stable --datadir /Data \
   --dev --dev.period 14 --rpc --rpcport 8545 --rpcaddr 0.0.0.0 --rpcapi db,eth,net,web3,personal,admin,txpool \
  --ws --ws.port 8546 --ws.addr 0.0.0.0 --ws.origins "*" --ws.api db,eth,net,web3,personal,admin,txpool &> /dev/null && \
  id=$(get_geth_container_id) && \
  echo "Started geth ${id}"
  ;;

stop-geth)
  id=$(get_geth_container_id)
  if [ -z "${id}" ]; then
      echo "geth node not running"
      exit 0
  fi

  docker stop geth &> /dev/null && \
  docker rm geth &> /dev/null && \
  echo "geth node stopped"
  ;;

deploy-contracts)
  dir=$(dirname "$0") && \
  source ~/.zshrc && \
  cd "${dir}"/snowbridge/ethereum/ && \
  nvm install 14.16.1 && \
  nvm use 14.16.1 && \
  yarn install && \
  cp env.template .env && \
  npx hardhat deploy --network localhost

  ;;
esac
