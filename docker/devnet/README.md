# ⚡*BI* devnet

##  `./run.sh [devnet|dev|net]`

Spins up a full-fledged rococo local devnet consisting of 5 relay chain validators and 2 collators for each parachain.

To actually apply and test Circuit runtime changes the Docker image must be rebuilt, to that end trash the image manually, fx `docker image rm circuit-collator:update_v0.9.19 -f`, then just run `./run.sh`.

## `./run.sh cleanup`

Stops all nodes and swipes their base path data directories.

<!-- ## `./run.sh setkeys`

Inserts static collator keys into the nodes' keystores.

Is run as part of `./run.sh devnet`, no manual execution required.

## `./run.sh onboard`

Initializes registration with the relay chain for both t3rn and acala.

After [onboarding](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/parachains) is complete the parachains should start to collate.

Also run as part of `./run.sh devnet`, no manual execution required. -->

## `./run.sh build`

Builds docker images and regenerates chain specs, and collator keys.

Only necessary if any of the runtimes have changed.

To actually have docker images rebuilt, prune them manually in advance, fx `docker image rm circuit-collator:update_v0.9.19 -f`.

## Topology

<table style="margin-bottom:0;">
  <tr>
    <td><b>Network</b></td>
    <td><b>Node</b></td>
    <td colspan="3"><b>Relaychain Ports</b></td>
    <td colspan="3"><b>Parachain Ports</b></td>
    <td><b>Parachain Id</b></td>
  </tr>
  <tr>
    <td>-</td>
    <td>-</td>
    <td>P2P</td>
    <td>RPC</td>
    <td>WS</td>
    <td>P2P</td>
    <td>RPC</td>
    <td>WS</td>
    <td>-</td>
  </tr>
  <tr>
    <td>rococo</td>
    <td>alice</td>
    <td>60001</td>
    <td>1844</td>
    <td><b>1944</b></td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>rococo</td>
    <td>bob</td>
    <td>60002</td>
    <td>8845</td>
    <td>9945</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>rococo</td>
    <td>charlie</td>
    <td>60003</td>
    <td>8846</td>
    <td>9946</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>rococo</td>
    <td>dave</td>
    <td>60004</td>
    <td>8847</td>
    <td>9947</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>rococo</td>
    <td>eve</td>
    <td>60005</td>
    <td>8848</td>
    <td>9948</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>t3rn</td>
    <td>t3rn1</td>
    <td>13332</td>
    <td>1832</td>
    <td>1932</td>
    <td>13333</td>
    <td><b>1833</b></td>
    <td><b>1933</b></td>
    <td>3333</td>
  </tr>
  <tr>
    <td>t3rn</td>
    <td>t3rn2</td>
    <td>13322</td>
    <td>1822</td>
    <td>1922</td>
    <td>13323</td>
    <td>1823</td>
    <td>1923</td>
    <td>3333</td>
  </tr>
  <tr>
    <td>pchain</td>
    <td>pchain1</td>
    <td>44443</td>
    <td>4487</td>
    <td>4498</td>
    <td>44444</td>
    <td>4488</td>
    <td>4499</td>
    <td>3334</td>
  </tr>
  <tr>
    <td>pchain</td>
    <td>pchain2</td>
    <td>44403</td>
    <td>4407</td>
    <td>4418</td>
    <td>44404</td>
    <td>4408</td>
    <td>4409</td>
    <td>3334</td>
  </tr>
</table>
</br>

The highlighted ports are the only ones mapped from the Docker network onto the host machine. Given selection allows connecting to both the relay- and parachain and should serve all your development needs. If you want to expose more ports to your machine just uncomment them within the Docker Compose file.

The HRMP channels setup between the parachains have a maximum capacity of 8 and a maximum message size of 1024 bytes.

All codebases are based on `v0.9.19` Substrate.