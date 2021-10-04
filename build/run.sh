#!/bin/zsh

get_geth_container_id() {
  docker ps -q -f name=geth
}

get_beefy_pid_file() {
  name=$1
  echo "/tmp/beefy_${name}.pid"
}

get_beefy_process_id() {
  pid_file=$(get_beefy_pid_file "$1")
  if [[ -s "${pid_file}" ]]; then
    cat "${pid_file}"
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

build-beefy)
  dir=$(dirname "${0}")
  build="${BUILD_BEEFY:-false}"
  if [[ "${build}" == "true" ]]; then
    echo "Building beefy..."
    cargo build --release --manifest-path="${dir}/../beefy/Cargo.toml" &> /tmp/beefy.build
  fi
  ;;

start-beefy)
  c_pid=$(get_beefy_process_id "$2")
  if [ -n "${c_pid}" ]; then
      echo "beefy[$2] already running[${c_pid}]"
      exit 0
  fi
  dir=$(dirname "${0}")
  log_file="/tmp/beefy_$2.log"
  "${dir}"/../beefy/target/release/beefy-node --enable-offchain-indexing=true --"$2" --log=main,info --tmp &> "${log_file}" &
  c_pid=$!
  pid_file=$(get_beefy_pid_file "$2")
  echo "${c_pid}" > "${pid_file}"
  echo "beefy[$2] started[${c_pid}]..."
  ;;

stop-beefy)
  c_pid=$(get_beefy_process_id "$2")
  if [ -z "${c_pid}" ]; then
      echo "beefy[$2] not running"
      exit 0
  fi

  kill -9 "${c_pid}" &> /dev/null
  pid_file=$(get_beefy_pid_file "$2")
  rm -rf "${pid_file}"
  echo "beefy[$2] stopped"
  ;;

all)
  echo "starting environment. may take sometime..."
  $0 start-geth && \
  $0 build-beefy && \
  $0 start-beefy alice && \
  $0 start-beefy bob && \
  $0 start-beefy charlie && \
  $0 deploy-contracts && \
  echo "done"
  ;;

clean)
  echo "cleaning up..."
  $0 stop-geth && \
  $0 stop-beefy alice && \
  $0 stop-beefy bob && \
  $0 stop-beefy charlie && \
  $0 clean-contracts && \
  echo "done"
  ;;
esac
