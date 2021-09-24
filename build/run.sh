#!/bin/zsh

get_geth_container_id() {
  docker ps -q -f name=geth
}

circuit_pid_file="/tmp/circuit.pid"
get_circuit_process_id() {
  if [[ -s "${circuit_pid_file}" ]]; then
    cat "${circuit_pid_file}"
  fi
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
  echo "Started geth[${id}]"
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

clean-contracts)
  echo "cleaning contracts cache..."
  dir=$(dirname "$0") && \
  cd "${dir}"/snowbridge/ethereum/ && \
  rm -rf .deployments
  echo "done"
  ;;

start-circuit)
  c_pid=$(get_circuit_process_id)
  if [ -n "${c_pid}" ]; then
      echo "circuit already running[${c_pid}]"
      exit 0
  fi
  dir=$(dirname "${0}")
  echo "building circuit..."
  cargo build --release --manifest-path="${dir}/../circuit/Cargo.toml" &> /tmp/circuit.build && \
  "${dir}"/../circuit/target/release/circuit --alice --log=main,info --tmp &> /tmp/circuit.log &
  c_pid=$!
  echo "${c_pid}" > "${circuit_pid_file}"
  echo "circuit started[${c_pid}]..."
  ;;

stop-circuit)
  c_pid=$(get_circuit_process_id)
  if [ -z "${c_pid}" ]; then
      echo "circuit not running"
      exit 0
  fi

  kill -9 "${c_pid}" &> /dev/null
  rm -rf "${circuit_pid_file}"
  echo "circuit stopped"
  ;;

all)
  echo "starting environment. may take sometime..."
  $0 start-geth && \
  $0 start-circuit && \
  $0 deploy-contracts && \
  echo "done"
  ;;

clean)
  echo "cleaning up..."
  $0 stop-geth && \
  $0 stop-circuit && \
  $0 clean-contracts && \
  echo "done"
  ;;
esac
