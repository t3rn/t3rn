# ⚡*CM* devnet WIP ⚠️

##  `./run.sh devnet`

Spins up a rococo local devnet consisting of 5 relay chain validators and 2 collators for each parachain.

The `setkeys` and `onboard` commands need to be run for a full fledged relay-parachain topology.

##  `./run.sh setkeys`

Inserts static collator keys into the nodes' keystores.

Must be run after `./run.sh devnet`.

## `./run.sh onboard`

Initializes parachain registration with the relay chain.

After the onboarding is complete the parachain will start to collate.

## `./run.sh cleanup`

Stops all nodes and swipes their base path data directories.

## `./run.sh build`

Builds docker images and regenerates chain specs, and collator keys.

Only necessary if any of the runtimes have changed.

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
    <td>Rococo</td>
    <td>Alice</td>
    <td>10001</td>
    <td>8844</td>
    <td>9944</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>Rococo</td>
    <td>Bob</td>
    <td>10002</td>
    <td>8845</td>
    <td>9945</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>Rococo</td>
    <td>Charlie</td>
    <td>10003</td>
    <td>8846</td>
    <td>9946</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>Rococo</td>
    <td>Dave</td>
    <td>10004</td>
    <td>8847</td>
    <td>9947</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
    <td>-</td>
  </tr>
  <tr>
    <td>Rococo</td>
    <td>Eve</td>
    <td>10005</td>
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
    <td>33332</td>
    <td>8832</td>
    <td>9932</td>
    <td>33333</td>
    <td>8833</td>
    <td>9933</td>
    <td>3333</td>
  </tr>
  <tr>
    <td>t3rn</td>
    <td>t3rn2</td>
    <td>33322</td>
    <td>8822</td>
    <td>9922</td>
    <td>33323</td>
    <td>8823</td>
    <td>9923</td>
    <td>3333</td>
  </tr>
  <tr>
    <td>pchain</td>
    <td>pchain1</td>
    <td>44444</td>
    <td>4488</td>
    <td>4499</td>
    <td>44443</td>
    <td>4487</td>
    <td>4498</td>
    <td>3334</td>
  </tr>
  <tr>
    <td>pchain</td>
    <td>pchain2</td>
    <td>44404</td>
    <td>4408</td>
    <td>4409</td>
    <td>44403</td>
    <td>4407</td>
    <td>4408</td>
    <td>3334</td>
  </tr>
</table>

*The "pchain" is a plain [Substrate parachain instance](https://github.com/substrate-developer-hub/substrate-parachain-template)*. All code uses `v0.9.17` Substrate.
