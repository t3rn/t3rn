#!/bin/bash

get_geth_container_id() {
  container_id=$(docker ps -q -f name=geth)
  echo "${container_id}"
}

mode=$1
case ${mode} in
start-geth)
  id=$(get_geth_container_id)
  if [ -n "${id}" ]; then
      echo "geth is already running[${id}]"
      exit 0
  fi

  id=$(docker run -d --name=geth -p 8545:8545 ethereum/client-go --http --dev --dev.period 14) && \
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
esac
